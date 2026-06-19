use std::{
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

use harness_store::bootstrap::validate_project_id;

use crate::{
    host_config::{binding_name, render_configs},
    local_mcp_command::{
        absolute_path, action_kind_text, action_target_name, discover_selected_mcp_command,
        execute_local_mcp_setup, resolve_setup_runtime_home, validate_config_destinations,
        ConfigDestinationStatus, LocalMcpCommandError, LocalMcpProcess, OutputFormat,
        ParsedLocalMcpOptions,
    },
    setup::{
        plan_local_mcp_setup, LocalMcpSetupOptions, LocalMcpSetupPlan, SetupActionKind,
        SetupActionTarget, SetupConflict, SetupPlanError, SetupResource, SetupSurfaceBinding,
    },
};

pub trait WizardIo {
    fn input_is_terminal(&self) -> bool;
    fn read_line(&mut self, buffer: &mut String) -> io::Result<usize>;
    fn write_prompt(&mut self, text: &str) -> io::Result<()>;
}

pub struct TerminalWizardIo<R, W> {
    input: R,
    prompts: W,
    input_is_terminal: bool,
}

impl<R, W> TerminalWizardIo<R, W> {
    pub const fn new(input: R, prompts: W, input_is_terminal: bool) -> Self {
        Self {
            input,
            prompts,
            input_is_terminal,
        }
    }
}

impl<R, W> WizardIo for TerminalWizardIo<R, W>
where
    R: BufRead,
    W: Write,
{
    fn input_is_terminal(&self) -> bool {
        self.input_is_terminal
    }

    fn read_line(&mut self, buffer: &mut String) -> io::Result<usize> {
        self.input.read_line(buffer)
    }

    fn write_prompt(&mut self, text: &str) -> io::Result<()> {
        self.prompts.write_all(text.as_bytes())?;
        self.prompts.flush()
    }
}

pub struct NoWizardIo;

impl WizardIo for NoWizardIo {
    fn input_is_terminal(&self) -> bool {
        false
    }

    fn read_line(&mut self, _buffer: &mut String) -> io::Result<usize> {
        Ok(0)
    }

    fn write_prompt(&mut self, _text: &str) -> io::Result<()> {
        Ok(())
    }
}

enum WizardError {
    Cancelled,
    Command(LocalMcpCommandError),
}

impl From<LocalMcpCommandError> for WizardError {
    fn from(error: LocalMcpCommandError) -> Self {
        Self::Command(error)
    }
}

type WizardResult<T> = Result<T, WizardError>;

pub(crate) fn run_local_mcp_wizard(
    parsed: ParsedLocalMcpOptions,
    current_dir: &Path,
    process: &mut impl LocalMcpProcess,
    io: &mut dyn WizardIo,
) -> Result<String, LocalMcpCommandError> {
    match run_local_mcp_wizard_inner(parsed, current_dir, process, io) {
        Ok(output) => Ok(output),
        Err(WizardError::Cancelled) => Ok("setup: cancelled\n".to_owned()),
        Err(WizardError::Command(error)) => Err(error),
    }
}

fn run_local_mcp_wizard_inner(
    mut parsed: ParsedLocalMcpOptions,
    current_dir: &Path,
    process: &mut impl LocalMcpProcess,
    io: &mut dyn WizardIo,
) -> WizardResult<String> {
    if parsed.output == OutputFormat::Json {
        return Err(LocalMcpCommandError::usage(
            "--interactive uses text output only and cannot be combined with --output json",
        )
        .into());
    }
    if !io.input_is_terminal() {
        return Err(LocalMcpCommandError::usage(
            "interactive local MCP setup requires terminal input; run without --interactive and pass explicit setup options such as --runtime-home, --repo-root, --project-id, --with-user-interaction, --config-dir, --replace-conflicting-surfaces, and --overwrite-config as needed",
        )
        .into());
    }

    parsed.output = OutputFormat::Text;
    write(io, "Interactive local MCP setup\n")?;
    write(io, "Type cancel at any prompt to exit without writing.\n\n")?;

    let runtime_home = prompt_runtime_home(&mut parsed, current_dir, process, io)?;
    parsed.runtime_home = Some(runtime_home.clone());

    let repo_root = prompt_repo_root(&mut parsed, current_dir, io)?;
    parsed.repo_root = Some(repo_root.clone());

    let project_id = prompt_project_id(&mut parsed, &runtime_home, &repo_root, io)?;
    parsed.project_id = Some(project_id);

    write(io, "\nAgent binding and access review:\n")?;
    write_binding(io, SetupSurfaceBinding::Agent)?;

    parsed.include_user_interaction = prompt_user_interaction(parsed.include_user_interaction, io)?;

    let config_dir = prompt_config_dir(&mut parsed, current_dir, io)?;
    parsed.config_dir = config_dir;

    let mcp_command = discover_selected_mcp_command(&parsed, current_dir, process)?;

    let mut plan = prompt_surface_conflicts(&mut parsed, &runtime_home, &repo_root, io)?;

    let project_id = plan
        .selected_project_id
        .as_deref()
        .ok_or_else(|| LocalMcpCommandError::runtime("setup plan has no selected project_id"))?;
    let config_dir = parsed
        .config_dir
        .as_ref()
        .map(|path| absolute_path(current_dir, path.clone()));
    let configs = render_configs(
        parsed.include_user_interaction,
        &runtime_home,
        project_id,
        &mcp_command,
        config_dir.as_deref(),
    );
    prompt_config_conflicts(&mut parsed, io)?;

    plan = plan_setup(&parsed, &runtime_home, &repo_root)?;
    render_final_plan(&plan, &mcp_command, &configs, parsed.dry_run, io)?;
    if !prompt_yes_no(io, "Apply this plan?", false)? {
        return Err(WizardError::Cancelled);
    }

    parsed.interactive = false;
    execute_local_mcp_setup(parsed, current_dir, process).map_err(Into::into)
}

fn prompt_runtime_home(
    parsed: &mut ParsedLocalMcpOptions,
    current_dir: &Path,
    process: &impl LocalMcpProcess,
    io: &mut dyn WizardIo,
) -> WizardResult<PathBuf> {
    let default = resolve_setup_runtime_home(parsed, current_dir, process)?;
    loop {
        let input = prompt_value(io, &format!("Runtime Home [{}]: ", default.display()))?;
        match input {
            PromptValue::Cancel => return Err(WizardError::Cancelled),
            PromptValue::Empty => return Ok(default),
            PromptValue::Text(value) => {
                let mut probe = parsed.clone();
                probe.runtime_home = Some(PathBuf::from(value));
                match resolve_setup_runtime_home(&probe, current_dir, process) {
                    Ok(path) => return Ok(path),
                    Err(error) => write(io, &format!("{error}\n"))?,
                }
            }
        }
    }
}

fn prompt_repo_root(
    parsed: &mut ParsedLocalMcpOptions,
    current_dir: &Path,
    io: &mut dyn WizardIo,
) -> WizardResult<PathBuf> {
    let default = parsed
        .repo_root
        .clone()
        .map(|path| absolute_path(current_dir, path))
        .unwrap_or_else(|| current_dir.to_path_buf());
    loop {
        let input = prompt_value(io, &format!("Product Repository [{}]: ", default.display()))?;
        let candidate = match input {
            PromptValue::Cancel => return Err(WizardError::Cancelled),
            PromptValue::Empty => default.clone(),
            PromptValue::Text(value) => absolute_path(current_dir, PathBuf::from(value)),
        };
        match std::fs::canonicalize(&candidate) {
            Ok(path) if path.is_dir() => return Ok(path),
            Ok(path) => write(
                io,
                &format!(
                    "Product Repository must be a directory: {}\n",
                    path.display()
                ),
            )?,
            Err(error) => write(
                io,
                &format!(
                    "Product Repository is not accessible: {}: {error}\n",
                    candidate.display()
                ),
            )?,
        }
    }
}

fn prompt_project_id(
    parsed: &mut ParsedLocalMcpOptions,
    runtime_home: &Path,
    repo_root: &Path,
    io: &mut dyn WizardIo,
) -> WizardResult<String> {
    let mut initial_default = parsed.project_id.clone();
    if initial_default.is_none() {
        initial_default = suggested_project_id(parsed, runtime_home, repo_root, io)?;
    }

    loop {
        let prompt = match &initial_default {
            Some(default) => format!("Project ID [{default}]: "),
            None => "Project ID: ".to_owned(),
        };
        let input = prompt_value(io, &prompt)?;
        let candidate = match input {
            PromptValue::Cancel => return Err(WizardError::Cancelled),
            PromptValue::Empty => {
                if let Some(default) = &initial_default {
                    default.clone()
                } else {
                    write(io, "Project ID is required for this repository.\n")?;
                    continue;
                }
            }
            PromptValue::Text(value) => value,
        };
        if let Err(error) = validate_project_id(&candidate) {
            write(io, &format!("{error}\n"))?;
            initial_default = None;
            continue;
        }

        let mut probe = parsed.clone();
        probe.project_id = Some(candidate.clone());
        let plan = plan_setup(&probe, runtime_home, repo_root)?;
        let project_conflicts = plan
            .conflicts
            .iter()
            .filter(|conflict| conflict.target == SetupActionTarget::Project)
            .collect::<Vec<_>>();
        if project_conflicts.is_empty() {
            return Ok(candidate);
        }
        for conflict in project_conflicts {
            write(io, &format!("Project ID conflict: {}\n", conflict.message))?;
        }
        initial_default = None;
    }
}

fn suggested_project_id(
    parsed: &ParsedLocalMcpOptions,
    runtime_home: &Path,
    repo_root: &Path,
    io: &mut dyn WizardIo,
) -> WizardResult<Option<String>> {
    let mut probe = parsed.clone();
    probe.project_id = None;
    let plan = plan_setup(&probe, runtime_home, repo_root)?;
    let project_conflicts = plan
        .conflicts
        .iter()
        .filter(|conflict| conflict.target == SetupActionTarget::Project)
        .collect::<Vec<_>>();
    if project_conflicts.is_empty() {
        Ok(plan.selected_project_id)
    } else {
        for conflict in project_conflicts {
            write(
                io,
                &format!("Project selection needs input: {}\n", conflict.message),
            )?;
        }
        Ok(None)
    }
}

fn prompt_user_interaction(default: bool, io: &mut dyn WizardIo) -> WizardResult<bool> {
    write(
        io,
        "\nUser-interaction connector review:\nThis is a separate connector binding, not an extension of the agent role. It is needed only when a real user-facing UI or connector will submit the user action. actor_kind=user alone does not establish user authority. Its configuration remains separate from the agent configuration.\n",
    )?;
    let include = prompt_yes_no(io, "Configure the user-interaction connector?", default)?;
    if include {
        write(io, "\nUser-interaction binding and access review:\n")?;
        write_binding(io, SetupSurfaceBinding::UserInteraction)?;
    }
    Ok(include)
}

fn prompt_config_dir(
    parsed: &mut ParsedLocalMcpOptions,
    current_dir: &Path,
    io: &mut dyn WizardIo,
) -> WizardResult<Option<PathBuf>> {
    let default = parsed
        .config_dir
        .clone()
        .map(|path| absolute_path(current_dir, path));
    let prompt = match &default {
        Some(path) => format!("Configuration output directory [{}]: ", path.display()),
        None => "Configuration output directory [stdout-only]: ".to_owned(),
    };
    let input = prompt_value(io, &prompt)?;
    match input {
        PromptValue::Cancel => Err(WizardError::Cancelled),
        PromptValue::Empty => Ok(default),
        PromptValue::Text(value) if value == "-" || value.eq_ignore_ascii_case("stdout") => {
            Ok(None)
        }
        PromptValue::Text(value) => Ok(Some(absolute_path(current_dir, PathBuf::from(value)))),
    }
}

fn prompt_surface_conflicts(
    parsed: &mut ParsedLocalMcpOptions,
    runtime_home: &Path,
    repo_root: &Path,
    io: &mut dyn WizardIo,
) -> WizardResult<LocalMcpSetupPlan> {
    let mut conflict_probe = parsed.clone();
    conflict_probe.replace_conflicting_surfaces = false;
    conflict_probe.authorized_surface_replacements.clear();
    let plan = plan_setup(&conflict_probe, runtime_home, repo_root)?;
    let project_conflicts = plan
        .conflicts
        .iter()
        .filter(|conflict| conflict.target == SetupActionTarget::Project)
        .collect::<Vec<_>>();
    if !project_conflicts.is_empty() {
        let messages = project_conflicts
            .iter()
            .map(|conflict| conflict.message.as_str())
            .collect::<Vec<_>>()
            .join("; ");
        return Err(LocalMcpCommandError::runtime(format!(
            "project conflict remains after project selection: {messages}"
        ))
        .into());
    }

    let surface_conflicts = plan
        .conflicts
        .iter()
        .filter(|conflict| {
            matches!(
                conflict.target,
                SetupActionTarget::AgentSurface | SetupActionTarget::UserInteractionSurface
            )
        })
        .collect::<Vec<_>>();
    if surface_conflicts.is_empty() {
        return Ok(plan_setup(parsed, runtime_home, repo_root)?);
    }

    for conflict in surface_conflicts {
        render_surface_conflict(conflict, io)?;
        let default_replace = parsed.replace_conflicting_surfaces;
        if !prompt_yes_no(io, "Replace this exact target surface?", default_replace)? {
            return Err(WizardError::Cancelled);
        }
        if !parsed
            .authorized_surface_replacements
            .contains(&conflict.target)
        {
            parsed.authorized_surface_replacements.push(conflict.target);
        }
    }
    parsed.replace_conflicting_surfaces = false;
    Ok(plan_setup(parsed, runtime_home, repo_root)?)
}

fn prompt_config_conflicts(
    parsed: &mut ParsedLocalMcpOptions,
    io: &mut dyn WizardIo,
) -> WizardResult<()> {
    let Some(config_dir) = parsed.config_dir.clone() else {
        return Ok(());
    };
    let destination_plan =
        validate_config_destinations(Some(&config_dir), parsed.include_user_interaction, true)?
            .ok_or_else(|| {
                LocalMcpCommandError::runtime("configuration destination plan was not produced")
            })?;

    let mut overwrite_any = parsed.overwrite_config;
    for target in destination_plan.targets {
        if target.status != ConfigDestinationStatus::ExistingRegularFile {
            continue;
        }
        write(
            io,
            &format!("Configuration file exists: {}\n", target.path.display()),
        )?;
        if !prompt_yes_no(
            io,
            "Overwrite this generated configuration file?",
            parsed.overwrite_config,
        )? {
            return Err(WizardError::Cancelled);
        }
        overwrite_any = true;
    }
    parsed.overwrite_config = overwrite_any;
    validate_config_destinations(
        Some(&config_dir),
        parsed.include_user_interaction,
        parsed.overwrite_config,
    )?;
    Ok(())
}

fn render_final_plan(
    plan: &LocalMcpSetupPlan,
    mcp_command: &Path,
    configs: &[crate::host_config::GeneratedConfig],
    dry_run: bool,
    io: &mut dyn WizardIo,
) -> WizardResult<()> {
    let project_id = plan
        .selected_project_id
        .as_deref()
        .ok_or_else(|| LocalMcpCommandError::runtime("setup plan has no selected project_id"))?;

    write(io, "\nComplete setup plan:\n")?;
    write(
        io,
        &format!("Runtime Home: {}\n", plan.runtime_home.display()),
    )?;
    write(io, &format!("repository: {}\n", plan.repo_root.display()))?;
    write(
        io,
        &format!(
            "project: {} ({})\n",
            project_id,
            action_kind_text(plan.project_action.kind)
        ),
    )?;
    for action in &plan.surface_actions {
        let SetupResource::Surface {
            binding,
            surface_id,
            surface_instance_id,
            ..
        } = &action.resource
        else {
            continue;
        };
        write(
            io,
            &format!(
                "{}: {} / {} ({})\n",
                binding_name(*binding),
                surface_id,
                surface_instance_id,
                action_kind_text(action.kind)
            ),
        )?;
    }
    write(io, &format!("MCP executable: {}\n", mcp_command.display()))?;
    write(io, "preflight bindings:\n")?;
    for binding in setup_bindings(plan.include_user_interaction) {
        write(
            io,
            &format!(
                "  {}: {} / {}\n",
                binding_name(binding),
                binding.surface_id(),
                binding.surface_instance_id()
            ),
        )?;
    }
    write(io, "configuration destinations:\n")?;
    for config in configs {
        match &config.output_path {
            Some(path) => write(
                io,
                &format!("  {}: {}\n", binding_name(config.binding), path.display()),
            )?,
            None => write(io, &format!("  {}: stdout\n", binding_name(config.binding)))?,
        }
    }
    write(io, &format!("dry_run: {}\n", yes_no(dry_run)))?;
    write(io, "destructive updates:\n")?;
    let mut destructive = false;
    for action in plan
        .ordered_actions()
        .into_iter()
        .filter(|action| action.kind == SetupActionKind::Update)
    {
        destructive = true;
        write(
            io,
            &format!("  {}: updated\n", action_target_name(action.target)),
        )?;
    }
    for config in configs {
        if let Some(path) = &config.output_path {
            if path.exists() {
                destructive = true;
                write(io, &format!("  overwrite_config: {}\n", path.display()))?;
            }
        }
    }
    if !destructive {
        write(io, "  none\n")?;
    }
    Ok(())
}

fn render_surface_conflict(conflict: &SetupConflict, io: &mut dyn WizardIo) -> WizardResult<()> {
    write(
        io,
        &format!(
            "\nSurface conflict for {}",
            action_target_name(conflict.target)
        ),
    )?;
    if let Some(project_id) = &conflict.project_id {
        write(io, &format!(" in project {project_id}"))?;
    }
    write(io, ":\n")?;
    if let Some(surface_id) = &conflict.surface_id {
        write(io, &format!("surface_id: {surface_id}\n"))?;
    }
    if let Some(surface_instance_id) = &conflict.surface_instance_id {
        write(io, &format!("surface_instance_id: {surface_instance_id}\n"))?;
    }
    write(io, &format!("reason: {}\n", conflict.message))?;
    if let Some(details) = &conflict.surface_details {
        write(io, "current:\n")?;
        write(io, &format!("  kind: {}\n", details.current_kind))?;
        write(
            io,
            &format!("  interaction_role: {}\n", details.current_role),
        )?;
        write(
            io,
            &format!(
                "  access_classes: {}\n",
                details
                    .current_access_classes
                    .as_deref()
                    .map(format_access_classes)
                    .unwrap_or_else(|| "unavailable".to_owned())
            ),
        )?;
        write(io, "desired:\n")?;
        write(io, &format!("  kind: {}\n", details.desired_kind))?;
        write(
            io,
            &format!("  interaction_role: {}\n", details.desired_role),
        )?;
        write(
            io,
            &format!(
                "  access_classes: {}\n",
                format_access_classes(&details.desired_access_classes)
            ),
        )?;
    }
    Ok(())
}

fn write_binding(io: &mut dyn WizardIo, binding: SetupSurfaceBinding) -> WizardResult<()> {
    write(io, &format!("surface_id: {}\n", binding.surface_id()))?;
    write(
        io,
        &format!("surface_instance_id: {}\n", binding.surface_instance_id()),
    )?;
    write(
        io,
        &format!(
            "interaction_role: {}\n",
            binding.interaction_role().as_str()
        ),
    )?;
    write(io, "access_classes:\n")?;
    for access_class in binding.expected_access_classes() {
        write(io, &format!("  {}\n", access_class.as_str()))?;
    }
    Ok(())
}

fn plan_setup(
    parsed: &ParsedLocalMcpOptions,
    runtime_home: &Path,
    repo_root: &Path,
) -> Result<LocalMcpSetupPlan, LocalMcpCommandError> {
    let mut setup_options = LocalMcpSetupOptions::new(runtime_home, repo_root);
    setup_options.project_id = parsed.project_id.clone();
    setup_options.include_user_interaction = parsed.include_user_interaction;
    setup_options.replace_conflicting_surfaces = parsed.replace_conflicting_surfaces;
    setup_options.authorized_surface_replacements = parsed.authorized_surface_replacements.clone();
    plan_local_mcp_setup(setup_options).map_err(plan_error)
}

fn plan_error(error: SetupPlanError) -> LocalMcpCommandError {
    match error {
        SetupPlanError::InvalidOptions { detail } => LocalMcpCommandError::usage(detail),
        other => LocalMcpCommandError::runtime(other.to_string()),
    }
}

enum PromptValue {
    Empty,
    Text(String),
    Cancel,
}

fn prompt_value(io: &mut dyn WizardIo, prompt: &str) -> WizardResult<PromptValue> {
    write(io, prompt)?;
    let mut line = String::new();
    let bytes = io.read_line(&mut line).map_err(prompt_io_error)?;
    if bytes == 0 {
        return Ok(PromptValue::Cancel);
    }
    let value = line.trim();
    if value.eq_ignore_ascii_case("cancel")
        || value.eq_ignore_ascii_case("quit")
        || value.eq_ignore_ascii_case("q")
    {
        Ok(PromptValue::Cancel)
    } else if value.is_empty() {
        Ok(PromptValue::Empty)
    } else {
        Ok(PromptValue::Text(value.to_owned()))
    }
}

fn prompt_yes_no(io: &mut dyn WizardIo, question: &str, default: bool) -> WizardResult<bool> {
    let suffix = if default { " [Y/n]: " } else { " [y/N]: " };
    loop {
        match prompt_value(io, &format!("{question}{suffix}"))? {
            PromptValue::Cancel => return Err(WizardError::Cancelled),
            PromptValue::Empty => return Ok(default),
            PromptValue::Text(value)
                if value.eq_ignore_ascii_case("y") || value.eq_ignore_ascii_case("yes") =>
            {
                return Ok(true);
            }
            PromptValue::Text(value)
                if value.eq_ignore_ascii_case("n") || value.eq_ignore_ascii_case("no") =>
            {
                return Ok(false);
            }
            PromptValue::Text(_) => write(io, "Enter yes or no.\n")?,
        }
    }
}

fn setup_bindings(include_user_interaction: bool) -> Vec<SetupSurfaceBinding> {
    let mut bindings = vec![SetupSurfaceBinding::Agent];
    if include_user_interaction {
        bindings.push(SetupSurfaceBinding::UserInteraction);
    }
    bindings
}

fn format_access_classes(values: &[String]) -> String {
    values.join(",")
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn write(io: &mut dyn WizardIo, text: &str) -> WizardResult<()> {
    io.write_prompt(text).map_err(prompt_io_error)
}

fn prompt_io_error(error: io::Error) -> WizardError {
    LocalMcpCommandError::runtime(format!("interactive prompt I/O failed: {error}")).into()
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeMap,
        error::Error,
        ffi::OsString,
        fs,
        io::Cursor,
        path::{Path, PathBuf},
    };

    use harness_store::{
        bootstrap::{
            initialize_runtime_home, list_projects, list_surfaces, register_project,
            register_surface, ProjectRegistration, SurfaceRegistration, ACTIVE_PROJECT_STATUS,
        },
        migrations::{
            test_support::create_project_state_fixture_version, PROJECT_STATE_DATABASE_KIND,
            PROJECT_STATE_SCHEMA_VERSION,
        },
        sqlite::{open_read_only_database, project_state_db_path, registry_db_path},
    };
    use harness_test_support::TempRuntimeHome;
    use harness_types::{AccessClass, SurfaceInteractionRole};
    use rusqlite::{params, Connection};
    use serde_json::Value;

    use super::*;
    use crate::{
        registration::{capability_profile_json, local_access_json},
        setup::{
            AGENT_SURFACE_ID, AGENT_SURFACE_INSTANCE_ID, LOCAL_MCP_SURFACE_KIND,
            USER_INTERACTION_SURFACE_ID,
        },
    };

    #[test]
    fn non_terminal_interactive_is_rejected() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-non-terminal")?;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("", false);

        let error =
            run_local_mcp_wizard(fixture.parsed(), fixture.repo_root(), &mut process, &mut io)
                .expect_err("non-terminal input should be rejected");

        assert!(matches!(error, LocalMcpCommandError::Usage(_)));
        assert!(error.to_string().contains("requires terminal input"));
        assert!(!registry_db_path(fixture.runtime_home()).exists());
        Ok(())
    }

    #[test]
    fn interactive_json_output_is_rejected() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-json")?;
        let mut parsed = fixture.parsed();
        parsed.output = OutputFormat::Json;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("", true);

        let error = run_local_mcp_wizard(parsed, fixture.repo_root(), &mut process, &mut io)
            .expect_err("json output should be rejected");

        assert!(matches!(error, LocalMcpCommandError::Usage(_)));
        assert!(error.to_string().contains("--output json"));
        Ok(())
    }

    #[test]
    fn default_runtime_repo_project_and_stdout_config_are_accepted() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-defaults")?;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\n\nn\n", true);

        let output =
            run_local_mcp_wizard(fixture.parsed(), fixture.repo_root(), &mut process, &mut io)?;

        assert_eq!(output, "setup: cancelled\n");
        assert!(io.prompts().contains("Runtime Home"));
        assert!(io.prompts().contains("Product Repository"));
        assert!(io.prompts().contains("Project ID [repo]"));
        assert!(io
            .prompts()
            .contains("Configuration output directory [stdout-only]"));
        assert!(!registry_db_path(fixture.runtime_home()).exists());
        Ok(())
    }

    #[test]
    fn explicit_runtime_repo_project_user_and_config_dir_flow_applies() -> Result<(), Box<dyn Error>>
    {
        let fixture = WizardFixture::new("wizard-explicit")?;
        let config_dir = fixture.temp.path().join("configs");
        let other_runtime = fixture.temp.path().join("entered-runtime");
        let other_repo = fixture.temp.path().join("entered-repo");
        fs::create_dir_all(&other_repo)?;
        let mut parsed = fixture.parsed();
        parsed.runtime_home = None;
        parsed.repo_root = None;
        parsed.project_id = None;
        let input = format!(
            "{}\n{}\nproject_entered\ny\n{}\ny\n",
            other_runtime.display(),
            other_repo.display(),
            config_dir.display()
        );
        let mut process = FakeProcess::new(fixture.repo_root());
        process.env.insert(
            "HOME".to_owned(),
            fixture.temp.path().join("home").into_os_string(),
        );
        let mut io = TestWizardIo::new(&input, true);

        let output = run_local_mcp_wizard(parsed, fixture.repo_root(), &mut process, &mut io)?;

        assert!(output.contains("setup: complete\n"));
        assert!(output.contains("user_interaction_surface_id"));
        assert!(config_dir.join("harness-agent.mcp.json").exists());
        assert!(config_dir
            .join("harness-user-interaction.mcp.json")
            .exists());
        assert_eq!(process.calls.len(), 2);
        let projects = list_projects(&other_runtime)?;
        assert_eq!(projects[0].project_id, "project_entered");
        Ok(())
    }

    #[test]
    fn invalid_repository_retries() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-invalid-repo")?;
        let missing = fixture.temp.path().join("missing-repo");
        let input = format!("\n{}\n\n\n\nn\n", missing.display());
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new(&input, true);

        let output =
            run_local_mcp_wizard(fixture.parsed(), fixture.repo_root(), &mut process, &mut io)?;

        assert_eq!(output, "setup: cancelled\n");
        assert!(io.prompts().contains("not accessible"));
        Ok(())
    }

    #[test]
    fn invalid_project_id_retries_before_valid_input() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-invalid-project-retry")?;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\na/b\nproject_valid\nn\n\ny\n", true);

        let output =
            run_local_mcp_wizard(fixture.parsed(), fixture.repo_root(), &mut process, &mut io)?;

        assert!(output.contains("setup: complete\n"));
        assert!(output.contains("project_id: project_valid\n"));
        assert!(io
            .prompts()
            .contains("project_id must be a single path component"));
        assert_eq!(process.calls.len(), 1);
        let projects = list_projects(fixture.runtime_home())?;
        assert_eq!(projects[0].project_id, "project_valid");
        Ok(())
    }

    #[test]
    fn existing_project_suggestion_is_reused() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-existing-project")?;
        initialize_runtime_home(fixture.runtime_home(), "runtime", "{}")?;
        register_project(
            fixture.runtime_home(),
            ProjectRegistration {
                project_id: "project_existing".to_owned(),
                repo_root: fs::canonicalize(fixture.repo_root())?,
                project_home: None,
                status: ACTIVE_PROJECT_STATUS.to_owned(),
                metadata_json: "{}".to_owned(),
            },
        )?;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\n\ny\n", true);

        let output =
            run_local_mcp_wizard(fixture.parsed(), fixture.repo_root(), &mut process, &mut io)?;

        assert!(output.contains("project_id: project_existing\n"));
        assert!(io.prompts().contains("Project ID [project_existing]"));
        Ok(())
    }

    #[test]
    fn access_classes_and_separate_roles_are_displayed() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-access-display")?;
        let mut parsed = fixture.parsed();
        parsed.include_user_interaction = true;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\n\nn\n", true);

        let output = run_local_mcp_wizard(parsed, fixture.repo_root(), &mut process, &mut io)?;

        assert_eq!(output, "setup: cancelled\n");
        let prompts = io.prompts();
        assert!(prompts.contains("interaction_role: agent"));
        assert!(prompts.contains("interaction_role: user_interaction"));
        assert!(prompts.contains("write_authorization"));
        assert!(prompts.contains("artifact_registration"));
        assert!(prompts.contains("run_recording"));
        assert!(prompts.contains("This is a separate connector binding"));
        Ok(())
    }

    #[test]
    fn surface_replacement_decline_cancels_without_writes() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-surface-decline")?;
        register_read_only_agent_surface(&fixture)?;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\nn\n", true);

        let output =
            run_local_mcp_wizard(fixture.parsed(), fixture.repo_root(), &mut process, &mut io)?;

        assert_eq!(output, "setup: cancelled\n");
        assert!(io.prompts().contains("current:"));
        assert!(io.prompts().contains("desired:"));
        assert!(io.prompts().contains("Replace this exact target surface?"));
        assert!(process.calls.is_empty());
        let surfaces = list_surfaces(fixture.runtime_home(), "repo")?;
        assert_eq!(
            surfaces[0].local_access_json,
            local_access_json(&[AccessClass::ReadStatus])?
        );
        Ok(())
    }

    #[test]
    fn surface_replacement_accepts_and_applies() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-surface-accept")?;
        register_read_only_agent_surface(&fixture)?;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\n\ny\ny\n", true);

        let output =
            run_local_mcp_wizard(fixture.parsed(), fixture.repo_root(), &mut process, &mut io)?;

        assert!(output.contains("agent_surface: updated\n"));
        let surfaces = list_surfaces(fixture.runtime_home(), "repo")?;
        assert!(surfaces[0]
            .local_access_json
            .contains("write_authorization"));
        Ok(())
    }

    #[test]
    fn config_overwrite_decline_cancels_without_writes() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-config-decline")?;
        let config_dir = fixture.temp.path().join("configs");
        fs::create_dir_all(&config_dir)?;
        let target = config_dir.join("harness-agent.mcp.json");
        fs::write(&target, "old")?;
        let mut parsed = fixture.parsed();
        parsed.config_dir = Some(config_dir.clone());
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\nn\n", true);

        let output = run_local_mcp_wizard(parsed, fixture.repo_root(), &mut process, &mut io)?;

        assert_eq!(output, "setup: cancelled\n");
        assert_eq!(fs::read_to_string(target)?, "old");
        assert!(process.calls.is_empty());
        Ok(())
    }

    #[test]
    fn config_overwrite_accepts_and_writes() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-config-accept")?;
        let config_dir = fixture.temp.path().join("configs");
        fs::create_dir_all(&config_dir)?;
        let target = config_dir.join("harness-agent.mcp.json");
        fs::write(&target, "old")?;
        let mut parsed = fixture.parsed();
        parsed.config_dir = Some(config_dir);
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\n\ny\ny\n", true);

        let output = run_local_mcp_wizard(parsed, fixture.repo_root(), &mut process, &mut io)?;

        assert!(output.contains("setup: complete\n"));
        let value: Value = serde_json::from_str(&fs::read_to_string(target)?)?;
        assert_eq!(
            value["mcpServers"]["harness-agent"]["env"]["HARNESS_SURFACE_ID"],
            AGENT_SURFACE_ID
        );
        Ok(())
    }

    #[test]
    fn interactive_dry_run_writes_nothing_and_runs_no_preflight() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-dry-run")?;
        let mut parsed = fixture.parsed();
        parsed.dry_run = true;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\n\ny\n", true);

        let output = run_local_mcp_wizard(parsed, fixture.repo_root(), &mut process, &mut io)?;

        assert!(output.contains("setup: dry_run\n"));
        assert!(process.calls.is_empty());
        assert!(!registry_db_path(fixture.runtime_home()).exists());
        Ok(())
    }

    #[test]
    fn historical_final_confirmation_decline_does_not_migrate() -> Result<(), Box<dyn Error>> {
        let fixture = HistoricalWizardFixture::new(
            "wizard-historical-final-decline",
            &crate::registration::baseline_workflow_access_classes(),
        )?;
        let before = migration_count(fixture.state_path())?;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\n\nn\n", true);

        let output =
            run_local_mcp_wizard(fixture.parsed(), fixture.repo_root(), &mut process, &mut io)?;

        assert_eq!(output, "setup: cancelled\n");
        assert_eq!(migration_count(fixture.state_path())?, before);
        assert!(process.calls.is_empty());
        Ok(())
    }

    #[test]
    fn historical_surface_replacement_decline_does_not_migrate() -> Result<(), Box<dyn Error>> {
        let fixture = HistoricalWizardFixture::new(
            "wizard-historical-surface-decline",
            &[AccessClass::ReadStatus],
        )?;
        let before = migration_count(fixture.state_path())?;
        let original_access = surface_local_access(fixture.state_path())?;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\n\nn\n", true);

        let output =
            run_local_mcp_wizard(fixture.parsed(), fixture.repo_root(), &mut process, &mut io)?;

        assert_eq!(output, "setup: cancelled\n");
        assert!(io.prompts().contains("Replace this exact target surface?"));
        assert_eq!(migration_count(fixture.state_path())?, before);
        assert_eq!(surface_local_access(fixture.state_path())?, original_access);
        assert!(process.calls.is_empty());
        Ok(())
    }

    #[test]
    fn historical_config_overwrite_decline_does_not_migrate() -> Result<(), Box<dyn Error>> {
        let fixture = HistoricalWizardFixture::new(
            "wizard-historical-config-decline",
            &crate::registration::baseline_workflow_access_classes(),
        )?;
        let config_dir = fixture.temp.path().join("configs");
        fs::create_dir_all(&config_dir)?;
        let target = config_dir.join("harness-agent.mcp.json");
        fs::write(&target, "old")?;
        let mut parsed = fixture.parsed();
        parsed.config_dir = Some(config_dir);
        let before = migration_count(fixture.state_path())?;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\n\nn\n", true);

        let output = run_local_mcp_wizard(parsed, fixture.repo_root(), &mut process, &mut io)?;

        assert_eq!(output, "setup: cancelled\n");
        assert_eq!(fs::read_to_string(target)?, "old");
        assert_eq!(migration_count(fixture.state_path())?, before);
        assert!(process.calls.is_empty());
        Ok(())
    }

    #[test]
    fn historical_interactive_dry_run_does_not_migrate() -> Result<(), Box<dyn Error>> {
        let fixture = HistoricalWizardFixture::new(
            "wizard-historical-dry-run",
            &crate::registration::baseline_workflow_access_classes(),
        )?;
        let mut parsed = fixture.parsed();
        parsed.dry_run = true;
        let before = migration_count(fixture.state_path())?;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\n\ny\n", true);

        let output = run_local_mcp_wizard(parsed, fixture.repo_root(), &mut process, &mut io)?;

        assert!(output.contains("setup: dry_run\n"));
        assert_eq!(migration_count(fixture.state_path())?, before);
        assert!(process.calls.is_empty());
        Ok(())
    }

    #[test]
    fn prompt_output_is_separate_from_final_output() -> Result<(), Box<dyn Error>> {
        let fixture = WizardFixture::new("wizard-streams")?;
        let mut process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\n\ny\n", true);

        let output =
            run_local_mcp_wizard(fixture.parsed(), fixture.repo_root(), &mut process, &mut io)?;

        assert!(output.starts_with("setup: complete\n"));
        assert!(!output.contains("Runtime Home ["));
        assert!(io.prompts().contains("Runtime Home ["));
        Ok(())
    }

    #[test]
    fn interactive_dry_run_matches_equivalent_non_interactive_options() -> Result<(), Box<dyn Error>>
    {
        let fixture = WizardFixture::new("wizard-equivalent-options")?;
        let mut interactive = fixture.parsed();
        interactive.dry_run = true;
        let mut interactive_process = FakeProcess::new(fixture.repo_root());
        let mut io = TestWizardIo::new("\n\n\n\n\ny\n", true);

        let interactive_output = run_local_mcp_wizard(
            interactive,
            fixture.repo_root(),
            &mut interactive_process,
            &mut io,
        )?;

        let mut non_interactive = fixture.parsed();
        non_interactive.interactive = false;
        non_interactive.dry_run = true;
        non_interactive.project_id = Some("repo".to_owned());
        let mut non_interactive_process = FakeProcess::new(fixture.repo_root());
        let non_interactive_output = execute_local_mcp_setup(
            non_interactive,
            fixture.repo_root(),
            &mut non_interactive_process,
        )?;

        assert_eq!(interactive_output, non_interactive_output);
        Ok(())
    }

    fn register_read_only_agent_surface(fixture: &WizardFixture) -> Result<(), Box<dyn Error>> {
        initialize_runtime_home(fixture.runtime_home(), "runtime", "{}")?;
        register_project(
            fixture.runtime_home(),
            ProjectRegistration {
                project_id: "repo".to_owned(),
                repo_root: fs::canonicalize(fixture.repo_root())?,
                project_home: None,
                status: ACTIVE_PROJECT_STATUS.to_owned(),
                metadata_json: "{}".to_owned(),
            },
        )?;
        register_surface(
            fixture.runtime_home(),
            SurfaceRegistration {
                project_id: "repo".to_owned(),
                surface_id: AGENT_SURFACE_ID.to_owned(),
                surface_instance_id: AGENT_SURFACE_INSTANCE_ID.to_owned(),
                surface_kind: LOCAL_MCP_SURFACE_KIND.to_owned(),
                interaction_role: SurfaceInteractionRole::Agent,
                display_name: None,
                capability_profile_json: capability_profile_json(&[AccessClass::ReadStatus], None)?,
                local_access_json: local_access_json(&[AccessClass::ReadStatus])?,
                metadata_json: "{}".to_owned(),
            },
        )?;
        Ok(())
    }

    struct TestWizardIo {
        input: Cursor<Vec<u8>>,
        output: Vec<u8>,
        terminal: bool,
    }

    impl TestWizardIo {
        fn new(input: &str, terminal: bool) -> Self {
            Self {
                input: Cursor::new(input.as_bytes().to_vec()),
                output: Vec::new(),
                terminal,
            }
        }

        fn prompts(&self) -> String {
            String::from_utf8_lossy(&self.output).into_owned()
        }
    }

    impl WizardIo for TestWizardIo {
        fn input_is_terminal(&self) -> bool {
            self.terminal
        }

        fn read_line(&mut self, buffer: &mut String) -> io::Result<usize> {
            self.input.read_line(buffer)
        }

        fn write_prompt(&mut self, text: &str) -> io::Result<()> {
            self.output.write_all(text.as_bytes())
        }
    }

    struct WizardFixture {
        temp: TempRuntimeHome,
        runtime_home: PathBuf,
        repo_root: PathBuf,
        mcp_command: PathBuf,
    }

    impl WizardFixture {
        fn new(prefix: &str) -> Result<Self, Box<dyn Error>> {
            let temp = TempRuntimeHome::new(prefix)?;
            let runtime_home = temp.path().join("runtime-home");
            let repo_root = temp.path().join("repo");
            fs::create_dir_all(&repo_root)?;
            let mcp_command = temp.path().join("harness-mcp");
            fs::write(&mcp_command, "test")?;
            Ok(Self {
                temp,
                runtime_home,
                repo_root,
                mcp_command,
            })
        }

        fn runtime_home(&self) -> &Path {
            &self.runtime_home
        }

        fn repo_root(&self) -> &Path {
            &self.repo_root
        }

        fn parsed(&self) -> ParsedLocalMcpOptions {
            ParsedLocalMcpOptions {
                interactive: true,
                runtime_home: Some(self.runtime_home.clone()),
                repo_root: Some(self.repo_root.clone()),
                mcp_command: Some(self.mcp_command.clone()),
                ..Default::default()
            }
        }
    }

    struct HistoricalWizardFixture {
        temp: TempRuntimeHome,
        runtime_home: PathBuf,
        repo_root: PathBuf,
        mcp_command: PathBuf,
        state_path: PathBuf,
    }

    impl HistoricalWizardFixture {
        fn new(prefix: &str, access_classes: &[AccessClass]) -> Result<Self, Box<dyn Error>> {
            let temp = TempRuntimeHome::new(prefix)?;
            let runtime_home = temp.path().join("runtime-home");
            let repo_root = temp.path().join("repo");
            fs::create_dir_all(&repo_root)?;
            let mcp_command = temp.path().join("harness-mcp");
            fs::write(&mcp_command, "test")?;
            initialize_runtime_home(&runtime_home, "runtime_home_wizard_historical", "{}")?;
            register_project(
                &runtime_home,
                ProjectRegistration {
                    project_id: "repo".to_owned(),
                    repo_root: fs::canonicalize(&repo_root)?,
                    project_home: None,
                    status: ACTIVE_PROJECT_STATUS.to_owned(),
                    metadata_json: "{}".to_owned(),
                },
            )?;
            let state_path = project_state_db_path(&runtime_home, "repo");
            fs::remove_file(&state_path)?;
            let mut conn = Connection::open(&state_path)?;
            create_project_state_fixture_version(
                &mut conn,
                "repo",
                PROJECT_STATE_SCHEMA_VERSION - 1,
            )?;
            insert_historical_agent_surface(&conn, access_classes)?;
            drop(conn);
            Ok(Self {
                temp,
                runtime_home,
                repo_root,
                mcp_command,
                state_path,
            })
        }

        fn repo_root(&self) -> &Path {
            &self.repo_root
        }

        fn state_path(&self) -> &Path {
            &self.state_path
        }

        fn parsed(&self) -> ParsedLocalMcpOptions {
            ParsedLocalMcpOptions {
                interactive: true,
                runtime_home: Some(self.runtime_home.clone()),
                repo_root: Some(self.repo_root.clone()),
                mcp_command: Some(self.mcp_command.clone()),
                ..Default::default()
            }
        }
    }

    fn insert_historical_agent_surface(
        conn: &Connection,
        access_classes: &[AccessClass],
    ) -> Result<(), Box<dyn Error>> {
        conn.execute(
            "INSERT INTO surfaces (
                project_id,
                surface_id,
                surface_instance_id,
                surface_kind,
                display_name,
                capability_profile_json,
                local_access_json,
                registered_at,
                metadata_json
            )
            VALUES ('repo', ?1, ?2, ?3, 'Agent MCP', ?4, ?5, 't0', '{}')",
            params![
                AGENT_SURFACE_ID,
                AGENT_SURFACE_INSTANCE_ID,
                LOCAL_MCP_SURFACE_KIND,
                capability_profile_json(access_classes, None)?,
                local_access_json(access_classes)?
            ],
        )?;
        Ok(())
    }

    fn migration_count(path: &Path) -> Result<i64, Box<dyn Error>> {
        let conn = open_read_only_database(path)?;
        Ok(conn.query_row(
            "SELECT COUNT(*)
               FROM schema_migrations
              WHERE database_kind = ?1",
            params![PROJECT_STATE_DATABASE_KIND],
            |row| row.get(0),
        )?)
    }

    fn surface_local_access(path: &Path) -> Result<String, Box<dyn Error>> {
        Ok(open_read_only_database(path)?.query_row(
            "SELECT local_access_json
               FROM surfaces
              WHERE project_id = 'repo'
                AND surface_id = ?1
                AND surface_instance_id = ?2",
            params![AGENT_SURFACE_ID, AGENT_SURFACE_INSTANCE_ID],
            |row| row.get(0),
        )?)
    }

    #[derive(Debug)]
    struct FakeProcess {
        env: BTreeMap<String, OsString>,
        current_exe: PathBuf,
        calls: Vec<crate::local_mcp_command::PreflightEnvironment>,
    }

    impl FakeProcess {
        fn new(current_dir: &Path) -> Self {
            Self {
                env: BTreeMap::new(),
                current_exe: current_dir.join("harness"),
                calls: Vec::new(),
            }
        }
    }

    impl LocalMcpProcess for FakeProcess {
        fn env_var(&self, name: &str) -> Option<OsString> {
            self.env.get(name).cloned()
        }

        fn current_exe(&self) -> Result<PathBuf, String> {
            Ok(self.current_exe.clone())
        }

        fn run_preflight(
            &mut self,
            _command: &Path,
            environment: &crate::local_mcp_command::PreflightEnvironment,
        ) -> Result<crate::local_mcp_command::PreflightProcessOutput, String> {
            self.calls.push(environment.clone());
            let (role, baseline, access) = if environment.surface_id == USER_INTERACTION_SURFACE_ID
            {
                (
                    "user_interaction",
                    "not_applicable",
                    "read_status,core_mutation",
                )
            } else {
                (
                    "agent",
                    "full",
                    "read_status,core_mutation,write_authorization,artifact_registration,run_recording",
                )
            };
            Ok(crate::local_mcp_command::PreflightProcessOutput {
                success: true,
                status_code: Some(0),
                stdout: format!(
                    "configuration: valid\ntransport: stdio\nruntime_home: {}\nproject_id: {}\nsurface_id: {}\nsurface_instance_id: {}\ninteraction_role: {}\naccess_classes: {}\nbaseline_workflow_access: {}\nmissing_access_classes: \n",
                    environment.runtime_home.display(),
                    environment.project_id,
                    environment.surface_id,
                    environment.surface_instance_id,
                    role,
                    access,
                    baseline
                ),
                stderr: String::new(),
            })
        }
    }
}
