use std::{
    collections::BTreeMap,
    fs, io,
    path::{Path, PathBuf},
};

use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use volicord_store::{
    agent_connections::CONNECTION_MODE_WORKFLOW,
    bootstrap::{
        initialize_runtime_home, write_installation_profile, InstallationProfileRecord,
        InstallationProfileRegistration, RuntimeHomeRecord,
    },
    runtime_home::{resolve_runtime_home, RuntimeHomeResolutionError},
    StoreError,
};

use crate::registration::ADMIN_METADATA_JSON;
pub(crate) use crate::shell_path::{is_executable_file, mcp_binary_name, volicord_binary_name};
use crate::{
    setup_report::{
        CommandAvailability, SetupAction, SetupActionKind, SetupReport, SetupSectionStatus,
        SetupStatus,
    },
    shell_path::{
        candidate_user_bin_dirs, detect_command_on_path, path_directory_is_on_path,
        path_directory_is_writable, paths_equivalent, PATH_ENV,
    },
};

const INSTALLATION_ID: &str = "default";
const SETUP_CREATED_BY: &str = "volicord_cli_setup";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandStatus {
    Complete,
    ActionRequired,
    Failed,
}

impl CommandStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::ActionRequired => "action_required",
            Self::Failed => "failed",
        }
    }

    pub const fn exits_failure(self) -> bool {
        matches!(self, Self::Failed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutcome {
    pub status: CommandStatus,
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetupCommandError {
    Usage(String),
    Runtime(String),
}

impl std::fmt::Display for SetupCommandError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Usage(message) | Self::Runtime(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for SetupCommandError {}

impl From<StoreError> for SetupCommandError {
    fn from(error: StoreError) -> Self {
        Self::Runtime(error.to_string())
    }
}

impl From<RuntimeHomeResolutionError> for SetupCommandError {
    fn from(error: RuntimeHomeResolutionError) -> Self {
        Self::Runtime(error.to_string())
    }
}

impl From<io::Error> for SetupCommandError {
    fn from(error: io::Error) -> Self {
        Self::Runtime(error.to_string())
    }
}

pub trait SetupProcess {
    fn env_var(&self, name: &str) -> Option<std::ffi::OsString>;
    fn current_exe(&self) -> Result<PathBuf, String>;
}

pub struct ProductionSetupProcess;

impl SetupProcess for ProductionSetupProcess {
    fn env_var(&self, name: &str) -> Option<std::ffi::OsString> {
        std::env::var_os(name)
    }

    fn current_exe(&self) -> Result<PathBuf, String> {
        std::env::current_exe()
            .map_err(|error| format!("failed to read current executable: {error}"))
    }
}

pub struct ClosureSetupProcess<'a, F>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    env_var: &'a F,
}

impl<'a, F> ClosureSetupProcess<'a, F>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    pub fn new(env_var: &'a F) -> Self {
        Self { env_var }
    }
}

impl<F> SetupProcess for ClosureSetupProcess<'_, F>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    fn env_var(&self, name: &str) -> Option<std::ffi::OsString> {
        (self.env_var)(name)
    }

    fn current_exe(&self) -> Result<PathBuf, String> {
        std::env::current_exe()
            .map_err(|error| format!("failed to read current executable: {error}"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedSetupOptions {
    runtime_home: Option<PathBuf>,
    link_bin: Option<PathBuf>,
    mcp_command: Option<PathBuf>,
    output: OutputFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct DiagnosticCheck {
    id: String,
    status: String,
    summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Value>,
}

impl DiagnosticCheck {
    fn passed(id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: "passed".to_owned(),
            summary: summary.into(),
            details: None,
        }
    }

    fn warning(id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: "warning".to_owned(),
            summary: summary.into(),
            details: None,
        }
    }

    fn failed(id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: "failed".to_owned(),
            summary: summary.into(),
            details: None,
        }
    }

    fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiscoveredCommand {
    path: PathBuf,
    source: &'static str,
}

pub fn setup_usage() -> String {
    "volicord setup [--home PATH] [--link-bin PATH] [--mcp-command PATH] [--json]\n".to_owned()
}

pub fn run_setup_command(
    args: &[String],
    current_dir: &Path,
    process: &impl SetupProcess,
) -> Result<CommandOutcome, SetupCommandError> {
    if is_help_request(args) {
        return Ok(CommandOutcome {
            status: CommandStatus::Complete,
            output: setup_usage(),
        });
    }
    let parsed = parse_setup_options(args, current_dir)?;
    let runtime_home = resolve_setup_runtime_home(&parsed, current_dir, process)?;
    let runtime_home_id = runtime_home_id_for_path(&runtime_home)?;
    let runtime_home_record =
        initialize_runtime_home(&runtime_home, &runtime_home_id, ADMIN_METADATA_JSON)?;
    let runtime_home_section = runtime_home_report_section(&runtime_home_record);
    let mut checks =
        vec![
            DiagnosticCheck::passed("runtime_home", "Runtime Home registry is ready").with_details(
                json!({
                    "runtime_home": path_text(&runtime_home_record.runtime_home),
                    "registry_db": path_text(&runtime_home_record.registry_db_path),
                    "runtime_home_id": runtime_home_record.runtime_home_id,
                }),
            ),
        ];
    let mut actions_required = Vec::new();
    let mut actions_optional = Vec::new();
    let mut actions_performed = vec![SetupAction::performed(
        "runtime_home_ready",
        SetupActionKind::RuntimeHomeReady,
        "Runtime Home registry is ready.",
    )
    .with_path(&runtime_home_record.runtime_home)];
    let mut link_results = BTreeMap::new();

    let volicord_command = match discover_volicord_command(process) {
        Ok(command) => {
            checks.push(
                DiagnosticCheck::passed("volicord_command", "volicord command was discovered")
                    .with_details(json!({
                        "path": path_text(&command.path),
                        "source": command.source,
                    })),
            );
            command
        }
        Err(error) => {
            checks.push(
                DiagnosticCheck::failed("volicord_command", "volicord command was not discovered")
                    .with_details(json!({ "detail": error.to_string() })),
            );
            actions_required.push(SetupAction::required(
                "run_setup_from_volicord",
                SetupActionKind::CommandAvailability,
                "Run volicord setup from an accessible volicord executable.",
            ));
            let report = SetupReport::new(
                runtime_home_section,
                installation_profile_failed("installation profile was not saved", &error),
                vec![
                    missing_command_availability("volicord_command", &volicord_binary_name()),
                    missing_command_availability("volicord_mcp_command", &mcp_binary_name()),
                ],
                actions_required,
                actions_optional,
                actions_performed,
            );
            let status = command_status(report.status);
            return Ok(CommandOutcome {
                status,
                output: render_setup_output(
                    parsed.output,
                    &report,
                    &runtime_home_record,
                    None,
                    &checks,
                )?,
            });
        }
    };

    let volicord_mcp = match discover_mcp_command(&parsed, process) {
        Ok(command) => {
            checks.push(
                DiagnosticCheck::passed(
                    "volicord_mcp_command",
                    "volicord-mcp command was discovered",
                )
                .with_details(json!({
                    "path": path_text(&command.path),
                    "source": command.source,
                })),
            );
            command
        }
        Err(error) => {
            checks.push(
                DiagnosticCheck::failed(
                    "volicord_mcp_command",
                    "volicord-mcp command was not discovered",
                )
                .with_details(json!({ "detail": error.to_string() })),
            );
            actions_required.push(
                SetupAction::required(
                    "select_mcp_command",
                    SetupActionKind::SelectMcpCommand,
                    "Run volicord setup --mcp-command PATH with an executable volicord-mcp path.",
                )
                .with_command("volicord setup --mcp-command PATH"),
            );
            let path_env = process.env_var(PATH_ENV);
            let commands = vec![
                command_availability(
                    "volicord_command",
                    &volicord_binary_name(),
                    &volicord_command,
                    path_env.as_deref(),
                ),
                missing_command_availability("volicord_mcp_command", &mcp_binary_name()),
            ];
            push_command_availability_checks(&commands, &mut checks);
            plan_setup_actions(
                &commands,
                &parsed,
                process,
                None,
                &mut actions_required,
                &mut actions_optional,
            );
            let report = SetupReport::new(
                runtime_home_section,
                installation_profile_failed("installation profile was not saved", &error),
                commands,
                actions_required,
                actions_optional,
                actions_performed,
            );
            let status = command_status(report.status);
            return Ok(CommandOutcome {
                status,
                output: render_setup_output(
                    parsed.output,
                    &report,
                    &runtime_home_record,
                    None,
                    &checks,
                )?,
            });
        }
    };
    let bin_dir = parsed
        .link_bin
        .clone()
        .unwrap_or_else(|| command_parent(&volicord_command.path));
    let mut link_bin_on_path = None;

    if let Some(link_bin) = &parsed.link_bin {
        let link_bin = absolute_path(current_dir, link_bin.clone());
        let mut link_bin_usable = false;
        match fs::create_dir_all(&link_bin) {
            Ok(()) => {
                link_bin_usable = true;
                let volicord_link = install_command_link(
                    &link_bin,
                    &volicord_binary_name(),
                    &volicord_command.path,
                );
                let mcp_link =
                    install_command_link(&link_bin, &mcp_binary_name(), &volicord_mcp.path);
                push_link_check(
                    "link_volicord",
                    "volicord command link",
                    &link_bin,
                    &volicord_binary_name(),
                    &volicord_link,
                    LinkCheckOutputs {
                        checks: &mut checks,
                        actions_required: &mut actions_required,
                        actions_performed: &mut actions_performed,
                    },
                );
                push_link_check(
                    "link_volicord_mcp",
                    "volicord-mcp command link",
                    &link_bin,
                    &mcp_binary_name(),
                    &mcp_link,
                    LinkCheckOutputs {
                        checks: &mut checks,
                        actions_required: &mut actions_required,
                        actions_performed: &mut actions_performed,
                    },
                );
                link_results.insert("volicord".to_owned(), link_volicord_status(&volicord_link));
                link_results.insert("volicord_mcp".to_owned(), link_volicord_status(&mcp_link));
            }
            Err(error) => {
                checks.push(
                    DiagnosticCheck::failed("link_bin", "link directory could not be created")
                        .with_details(
                            json!({ "path": path_text(&link_bin), "detail": error.to_string() }),
                        ),
                );
                actions_required.push(
                    SetupAction::required(
                        "repair_link_bin",
                        SetupActionKind::CommandLinks,
                        format!(
                            "Fix write access for {}, then rerun volicord setup --link-bin {}.",
                            link_bin.display(),
                            link_bin.display()
                        ),
                    )
                    .with_path(&link_bin),
                );
                link_results.insert("volicord".to_owned(), "failed".to_owned());
                link_results.insert("volicord_mcp".to_owned(), "failed".to_owned());
            }
        }
        let on_path = path_directory_is_on_path(process.env_var(PATH_ENV).as_deref(), &link_bin);
        link_bin_on_path = Some(on_path);
        if !on_path {
            if link_bin_usable {
                actions_required.push(
                    SetupAction::required(
                        "add_link_bin_to_path",
                        SetupActionKind::PathUpdate,
                        format!(
                            "Add {} to PATH before starting new shells or MCP hosts.",
                            link_bin.display()
                        ),
                    )
                    .with_command(format!("export PATH=\"{}:$PATH\"", link_bin.display()))
                    .with_path(&link_bin),
                )
            }
            checks.push(
                DiagnosticCheck::warning(
                    "link_bin_path",
                    "link directory is not currently on PATH",
                )
                .with_details(json!({ "link_bin": path_text(&link_bin) })),
            );
        } else {
            checks.push(
                DiagnosticCheck::passed("link_bin_path", "link directory is on PATH")
                    .with_details(json!({ "link_bin": path_text(&link_bin) })),
            );
        }
    }

    let path_env = process.env_var(PATH_ENV);
    let commands = vec![
        command_availability(
            "volicord_command",
            &volicord_binary_name(),
            &volicord_command,
            path_env.as_deref(),
        ),
        command_availability(
            "volicord_mcp_command",
            &mcp_binary_name(),
            &volicord_mcp,
            path_env.as_deref(),
        ),
    ];
    push_command_availability_checks(&commands, &mut checks);
    plan_setup_actions(
        &commands,
        &parsed,
        process,
        link_bin_on_path,
        &mut actions_required,
        &mut actions_optional,
    );

    let metadata_json = setup_metadata_json(
        volicord_command.source,
        volicord_mcp.source,
        parsed.link_bin.as_deref(),
        &link_results,
    )?;
    let profile = write_installation_profile(
        &runtime_home,
        InstallationProfileRegistration {
            installation_id: INSTALLATION_ID.to_owned(),
            volicord_command: path_text(&volicord_command.path),
            volicord_mcp_command: path_text(&volicord_mcp.path),
            bin_dir,
            default_connection_mode: CONNECTION_MODE_WORKFLOW.to_owned(),
            metadata_json,
        },
    )?;
    checks.push(
        DiagnosticCheck::passed("installation_profile", "installation profile was saved")
            .with_details(profile_json(&profile)),
    );
    actions_performed.push(
        SetupAction::performed(
            "installation_profile_saved",
            SetupActionKind::InstallationProfileSaved,
            "Installation profile was saved.",
        )
        .with_path(&runtime_home_record.registry_db_path),
    );

    let report = SetupReport::new(
        runtime_home_section,
        SetupSectionStatus::complete("installation profile was saved", profile_json(&profile)),
        commands,
        actions_required,
        actions_optional,
        actions_performed,
    );
    let status = command_status(report.status);
    Ok(CommandOutcome {
        status,
        output: render_setup_output(
            parsed.output,
            &report,
            &runtime_home_record,
            Some(&profile),
            &checks,
        )?,
    })
}

fn runtime_home_report_section(record: &RuntimeHomeRecord) -> SetupSectionStatus {
    SetupSectionStatus::complete(
        "Runtime Home registry is ready",
        json!({
            "runtime_home": path_text(&record.runtime_home),
            "registry_db": path_text(&record.registry_db_path),
            "runtime_home_id": record.runtime_home_id,
        }),
    )
}

fn installation_profile_failed(
    summary: impl Into<String>,
    error: &SetupCommandError,
) -> SetupSectionStatus {
    SetupSectionStatus::failed(summary, json!({ "detail": error.to_string() }))
}

fn command_availability(
    id: impl Into<String>,
    command_name: &str,
    discovered: &DiscoveredCommand,
    path_env: Option<&std::ffi::OsStr>,
) -> CommandAvailability {
    let path_match = detect_command_on_path(command_name, path_env);
    let discovered_dir = command_parent(&discovered.path);
    let discovered_directory_on_path = path_directory_is_on_path(path_env, &discovered_dir);
    let path_matches_discovered = path_match
        .as_deref()
        .map(|path| paths_equivalent(path, &discovered.path))
        .unwrap_or(false);
    CommandAvailability {
        id: id.into(),
        command_name: command_name.to_owned(),
        discovered: true,
        discovered_path: Some(path_text(&discovered.path)),
        discovery_source: Some(discovered.source.to_owned()),
        available_on_path: path_match.is_some(),
        path_matches_discovered,
        discovered_directory_on_path,
        path_match: path_match.as_deref().map(path_text),
    }
}

fn missing_command_availability(id: impl Into<String>, command_name: &str) -> CommandAvailability {
    CommandAvailability {
        id: id.into(),
        command_name: command_name.to_owned(),
        discovered: false,
        discovered_path: None,
        discovery_source: None,
        available_on_path: false,
        path_matches_discovered: false,
        discovered_directory_on_path: false,
        path_match: None,
    }
}

fn push_command_availability_checks(
    commands: &[CommandAvailability],
    checks: &mut Vec<DiagnosticCheck>,
) {
    for command in commands {
        if !command.discovered {
            checks.push(DiagnosticCheck::failed(
                format!("{}_availability", command.id),
                format!("{} command was not discovered", command.command_name),
            ));
        } else if command.selected_path_ready() {
            checks.push(
                DiagnosticCheck::passed(
                    format!("{}_availability", command.id),
                    format!(
                        "{} resolves to the selected executable on PATH",
                        command.command_name
                    ),
                )
                .with_details(command_availability_details(command)),
            );
        } else if command.available_on_path {
            checks.push(
                DiagnosticCheck::warning(
                    format!("{}_availability", command.id),
                    format!(
                        "{} resolves to a different executable on PATH",
                        command.command_name
                    ),
                )
                .with_details(command_availability_details(command)),
            );
        } else {
            checks.push(
                DiagnosticCheck::warning(
                    format!("{}_availability", command.id),
                    format!("{} is not available on PATH", command.command_name),
                )
                .with_details(command_availability_details(command)),
            );
        }
    }
}

fn command_availability_details(command: &CommandAvailability) -> Value {
    json!({
        "command_name": &command.command_name,
        "discovered_path": &command.discovered_path,
        "discovery_source": &command.discovery_source,
        "available_on_path": command.available_on_path,
        "path_matches_discovered": command.path_matches_discovered,
        "discovered_directory_on_path": command.discovered_directory_on_path,
        "path_match": &command.path_match,
    })
}

fn plan_setup_actions(
    commands: &[CommandAvailability],
    parsed: &ParsedSetupOptions,
    process: &impl SetupProcess,
    link_bin_on_path: Option<bool>,
    actions_required: &mut Vec<SetupAction>,
    actions_optional: &mut Vec<SetupAction>,
) {
    let link_bin_requested_but_not_on_path = link_bin_on_path == Some(false);
    for command in commands {
        if command.selected_path_ready() || link_bin_requested_but_not_on_path {
            continue;
        }
        if command.available_on_path {
            push_unique_action(
                actions_required,
                SetupAction::required(
                    format!("resolve_{}_path_mismatch", command.id),
                    SetupActionKind::CommandAvailability,
                    format!(
                        "Update PATH so {} resolves to the selected executable before starting new shells or MCP hosts.",
                        command.command_name
                    ),
                ),
            );
        } else if command.discovered {
            let mut action = SetupAction::required(
                format!("make_{}_available", command.id),
                SetupActionKind::CommandAvailability,
                format!(
                    "Make {} available on PATH before starting new shells or MCP hosts.",
                    command.command_name
                ),
            );
            if let Some(discovered_path) = command.discovered_path.as_deref() {
                let discovered_path = Path::new(discovered_path);
                if discovered_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name == command.command_name)
                {
                    let parent = command_parent(discovered_path);
                    action =
                        action.with_command(format!("export PATH=\"{}:$PATH\"", parent.display()));
                }
            }
            push_unique_action(actions_required, action);
        }
    }

    if parsed.link_bin.is_none()
        && commands
            .iter()
            .any(|command| !command.selected_path_ready())
    {
        let mut action = SetupAction::optional(
            "create_command_links",
            SetupActionKind::CommandLinks,
            "Create command links with --link-bin; setup will not modify shell startup files.",
        );
        if let Some(link_bin) = suggested_link_bin(process) {
            action = action
                .with_command(format!("volicord setup --link-bin {}", link_bin.display()))
                .with_path(&link_bin);
        }
        push_unique_action(actions_optional, action);
    }
}

fn suggested_link_bin(process: &impl SetupProcess) -> Option<PathBuf> {
    let dirs = candidate_user_bin_dirs(&|name| process.env_var(name));
    dirs.iter()
        .find(|path| path_directory_is_writable(path))
        .cloned()
        .or_else(|| dirs.into_iter().next())
}

fn push_unique_action(actions: &mut Vec<SetupAction>, action: SetupAction) {
    if !actions.iter().any(|existing| existing.id == action.id) {
        actions.push(action);
    }
}

fn command_status(status: SetupStatus) -> CommandStatus {
    match status {
        SetupStatus::Complete => CommandStatus::Complete,
        SetupStatus::ActionRequired => CommandStatus::ActionRequired,
        SetupStatus::Failed => CommandStatus::Failed,
    }
}

fn parse_setup_options(
    args: &[String],
    current_dir: &Path,
) -> Result<ParsedSetupOptions, SetupCommandError> {
    let mut parsed = ParsedSetupOptions {
        runtime_home: None,
        link_bin: None,
        mcp_command: None,
        output: OutputFormat::Text,
    };
    let mut seen = BTreeMap::<String, ()>::new();
    let mut index = 0;
    while index < args.len() {
        let token = &args[index];
        if token == "-h" || token == "--help" || token == "help" {
            return Err(SetupCommandError::Usage(setup_usage()));
        }
        if !token.starts_with("--") {
            return Err(SetupCommandError::Usage(format!(
                "unexpected argument: {token}"
            )));
        }
        let without_prefix = &token[2..];
        let (name, value) = if let Some((name, value)) = without_prefix.split_once('=') {
            (name.to_owned(), Some(value.to_owned()))
        } else if without_prefix == "json" {
            (without_prefix.to_owned(), None)
        } else {
            index += 1;
            let Some(value) = args.get(index) else {
                return Err(SetupCommandError::Usage(format!(
                    "missing value for --{without_prefix}"
                )));
            };
            (without_prefix.to_owned(), Some(value.clone()))
        };
        if seen.insert(name.clone(), ()).is_some() {
            return Err(SetupCommandError::Usage(format!(
                "duplicate option: --{name}"
            )));
        }
        match name.as_str() {
            "home" => parsed.runtime_home = Some(value_path(&name, value.as_deref(), current_dir)?),
            "link-bin" => parsed.link_bin = Some(value_path(&name, value.as_deref(), current_dir)?),
            "mcp-command" => {
                parsed.mcp_command = Some(value_path(&name, value.as_deref(), current_dir)?)
            }
            "json" => {
                if value.is_some() {
                    return Err(SetupCommandError::Usage(
                        "--json does not accept a value".to_owned(),
                    ));
                }
                parsed.output = OutputFormat::Json;
            }
            _ => {
                return Err(SetupCommandError::Usage(format!(
                    "unknown option: --{name}"
                )))
            }
        }
        index += 1;
    }
    Ok(parsed)
}

fn value_path(
    name: &str,
    value: Option<&str>,
    current_dir: &Path,
) -> Result<PathBuf, SetupCommandError> {
    let value =
        value.ok_or_else(|| SetupCommandError::Usage(format!("missing value for --{name}")))?;
    if value.trim().is_empty() {
        return Err(SetupCommandError::Usage(format!(
            "--{name} must not be empty"
        )));
    }
    Ok(absolute_path(current_dir, PathBuf::from(value)))
}

fn resolve_setup_runtime_home(
    parsed: &ParsedSetupOptions,
    current_dir: &Path,
    process: &impl SetupProcess,
) -> Result<PathBuf, SetupCommandError> {
    if let Some(path) = &parsed.runtime_home {
        Ok(path.clone())
    } else {
        resolve_runtime_home(|name| process.env_var(name), current_dir).map_err(Into::into)
    }
}

fn discover_volicord_command(
    process: &impl SetupProcess,
) -> Result<DiscoveredCommand, SetupCommandError> {
    let current_exe = process.current_exe().map_err(SetupCommandError::Runtime)?;
    let path = canonical_existing_file(&current_exe, "volicord command")?;
    Ok(DiscoveredCommand {
        path,
        source: "current_exe",
    })
}

fn discover_mcp_command(
    parsed: &ParsedSetupOptions,
    process: &impl SetupProcess,
) -> Result<DiscoveredCommand, SetupCommandError> {
    if let Some(command) = &parsed.mcp_command {
        let path = canonical_existing_executable(command, "volicord-mcp command")?;
        return Ok(DiscoveredCommand {
            path,
            source: "explicit",
        });
    }

    let current_exe = process.current_exe().map_err(SetupCommandError::Runtime)?;
    if let Some(parent) = current_exe.parent() {
        let sibling = parent.join(mcp_binary_name());
        if is_executable_file(&sibling) {
            return Ok(DiscoveredCommand {
                path: canonical_existing_executable(&sibling, "volicord-mcp sibling")?,
                source: "sibling",
            });
        }
    }

    if let Some(candidate) =
        detect_command_on_path(&mcp_binary_name(), process.env_var(PATH_ENV).as_deref())
    {
        return Ok(DiscoveredCommand {
            path: canonical_existing_executable(&candidate, "volicord-mcp from PATH")?,
            source: "path",
        });
    }

    Err(SetupCommandError::Runtime(
        "volicord-mcp was not found; run `volicord setup --mcp-command PATH` with an executable volicord-mcp path".to_owned(),
    ))
}

fn canonical_existing_file(path: &Path, label: &'static str) -> Result<PathBuf, SetupCommandError> {
    let metadata = fs::metadata(path).map_err(|error| {
        SetupCommandError::Runtime(format!("{label} is not accessible: {error}"))
    })?;
    if !metadata.is_file() {
        return Err(SetupCommandError::Runtime(format!(
            "{label} must be a file: {}",
            path.display()
        )));
    }
    Ok(fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf()))
}

fn canonical_existing_executable(
    path: &Path,
    label: &'static str,
) -> Result<PathBuf, SetupCommandError> {
    let path = canonical_existing_file(path, label)?;
    if is_executable_file(&path) {
        Ok(path)
    } else {
        Err(SetupCommandError::Runtime(format!(
            "{label} must be executable: {}",
            path.display()
        )))
    }
}

fn is_help_request(args: &[String]) -> bool {
    matches!(
        args.first().map(String::as_str),
        Some("-h" | "--help" | "help")
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LinkInstallResult {
    Created(PathBuf),
    Existing(PathBuf),
    UnsafeExisting(PathBuf),
    #[cfg_attr(unix, allow(dead_code))]
    Unsupported(PathBuf),
    Failed {
        path: PathBuf,
        detail: String,
    },
}

fn install_command_link(link_bin: &Path, name: &str, target: &Path) -> LinkInstallResult {
    let link_path = link_bin.join(name);
    install_command_link_inner(&link_path, target)
}

#[cfg(unix)]
fn install_command_link_inner(link_path: &Path, target: &Path) -> LinkInstallResult {
    use std::os::unix::fs::symlink;

    match fs::symlink_metadata(link_path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                match fs::read_link(link_path) {
                    Ok(existing_target) if existing_target == target => {
                        LinkInstallResult::Existing(link_path.to_path_buf())
                    }
                    Ok(existing_target) => {
                        match (fs::canonicalize(existing_target), fs::canonicalize(target)) {
                            (Ok(existing), Ok(expected)) if existing == expected => {
                                LinkInstallResult::Existing(link_path.to_path_buf())
                            }
                            _ => LinkInstallResult::UnsafeExisting(link_path.to_path_buf()),
                        }
                    }
                    Err(error) => LinkInstallResult::Failed {
                        path: link_path.to_path_buf(),
                        detail: error.to_string(),
                    },
                }
            } else {
                LinkInstallResult::UnsafeExisting(link_path.to_path_buf())
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => match symlink(target, link_path) {
            Ok(()) => LinkInstallResult::Created(link_path.to_path_buf()),
            Err(error) => LinkInstallResult::Failed {
                path: link_path.to_path_buf(),
                detail: error.to_string(),
            },
        },
        Err(error) => LinkInstallResult::Failed {
            path: link_path.to_path_buf(),
            detail: error.to_string(),
        },
    }
}

#[cfg(not(unix))]
fn install_command_link_inner(link_path: &Path, _target: &Path) -> LinkInstallResult {
    LinkInstallResult::Unsupported(link_path.to_path_buf())
}

struct LinkCheckOutputs<'a> {
    checks: &'a mut Vec<DiagnosticCheck>,
    actions_required: &'a mut Vec<SetupAction>,
    actions_performed: &'a mut Vec<SetupAction>,
}

fn push_link_check(
    check_id: &str,
    label: &str,
    link_bin: &Path,
    name: &str,
    result: &LinkInstallResult,
    outputs: LinkCheckOutputs<'_>,
) {
    match result {
        LinkInstallResult::Created(path) => {
            outputs.checks.push(
                DiagnosticCheck::passed(check_id, format!("{label} was created"))
                    .with_details(json!({ "path": path_text(path) })),
            );
            outputs.actions_performed.push(
                SetupAction::performed(
                    format!("create_{name}_link"),
                    SetupActionKind::CommandLinks,
                    format!("{label} was created."),
                )
                .with_path(path),
            );
        }
        LinkInstallResult::Existing(path) => {
            outputs.checks.push(
                DiagnosticCheck::passed(
                    check_id,
                    format!("{label} already points to the selected executable"),
                )
                .with_details(json!({ "path": path_text(path) })),
            );
            outputs.actions_performed.push(
                SetupAction::performed(
                    format!("reuse_{name}_link"),
                    SetupActionKind::CommandLinks,
                    format!("{label} already points to the selected executable."),
                )
                .with_path(path),
            );
        }
        LinkInstallResult::Unsupported(path) => {
            outputs.checks.push(
                DiagnosticCheck::warning(
                    check_id,
                    format!("{label} was not created on this platform"),
                )
                .with_details(json!({ "path": path_text(path) })),
            );
            outputs.actions_required.push(
                SetupAction::required(
                    format!("create_{name}_shim"),
                    SetupActionKind::CommandLinks,
                    format!(
                        "Create a command shim for {name} under {} if your shell cannot find it.",
                        link_bin.display()
                    ),
                )
                .with_path(path),
            );
        }
        LinkInstallResult::UnsafeExisting(path) => {
            outputs.checks.push(
                DiagnosticCheck::failed(
                    check_id,
                    format!(
                        "{label} was not replaced because an existing path is not Volicord-managed"
                    ),
                )
                .with_details(json!({ "path": path_text(path) })),
            );
            outputs.actions_required.push(
                SetupAction::required(
                    format!("repair_{name}_link"),
                    SetupActionKind::CommandLinks,
                    format!(
                        "Move or remove the existing {} path, then rerun volicord setup --link-bin {}.",
                        path.display(),
                        link_bin.display()
                    ),
                )
                .with_path(path),
            );
        }
        LinkInstallResult::Failed { path, detail } => {
            outputs.checks.push(
                DiagnosticCheck::failed(check_id, format!("{label} could not be created"))
                    .with_details(json!({ "path": path_text(path), "detail": detail })),
            );
            outputs.actions_required.push(
                SetupAction::required(
                    format!("repair_{name}_link"),
                    SetupActionKind::CommandLinks,
                    format!(
                        "Fix write access for {}, then rerun volicord setup --link-bin {}.",
                        path.display(),
                        link_bin.display()
                    ),
                )
                .with_path(path),
            );
        }
    }
}

fn link_volicord_status(result: &LinkInstallResult) -> String {
    match result {
        LinkInstallResult::Created(_) => "created",
        LinkInstallResult::Existing(_) => "existing",
        LinkInstallResult::UnsafeExisting(_) => "unsafe_existing",
        LinkInstallResult::Unsupported(_) => "unsupported",
        LinkInstallResult::Failed { .. } => "failed",
    }
    .to_owned()
}

fn render_setup_output(
    output: OutputFormat,
    report: &SetupReport,
    runtime_home: &RuntimeHomeRecord,
    profile: Option<&InstallationProfileRecord>,
    checks: &[DiagnosticCheck],
) -> Result<String, SetupCommandError> {
    match output {
        OutputFormat::Json => serde_json::to_string_pretty(&json!({
            "status": report.status.as_str(),
            "runtime_home": path_text(&runtime_home.runtime_home),
            "registry_db": path_text(&runtime_home.registry_db_path),
            "installation_profile": profile.map(profile_json),
            "setup_report": report,
            "commands": &report.commands,
            "checks": checks,
            "actions": &report.actions_required,
            "actions_required": &report.actions_required,
            "actions_optional": &report.actions_optional,
            "actions_performed": &report.actions_performed,
        }))
        .map(|text| format!("{text}\n"))
        .map_err(|error| SetupCommandError::Runtime(error.to_string())),
        OutputFormat::Text => {
            let mut text = format!(
                "Volicord setup {}\nruntime_home: {}\nregistry_db: {}\n",
                report.status.as_str(),
                runtime_home.runtime_home.display(),
                runtime_home.registry_db_path.display(),
            );
            if let Some(profile) = profile {
                text.push_str(&format!(
                    "volicord_command: {}\nvolicord_mcp_command: {}\ndefault_connection_mode: {}\n",
                    profile.volicord_command,
                    profile.volicord_mcp_command,
                    profile.default_connection_mode
                ));
            }
            if !report.commands.is_empty() {
                text.push_str("command_availability:\n");
                for command in &report.commands {
                    text.push_str(&format!(
                        "- {}: {}\n",
                        command.command_name,
                        command_availability_summary(command)
                    ));
                }
            }
            let not_passed = checks
                .iter()
                .filter(|check| check.status != "passed")
                .collect::<Vec<_>>();
            if !not_passed.is_empty() {
                text.push_str("checks:\n");
                for check in not_passed {
                    text.push_str(&format!(
                        "- {}: {} ({})\n",
                        check.id, check.summary, check.status
                    ));
                }
            }
            if !report.actions_required.is_empty() {
                text.push_str("actions:\n");
                for action in &report.actions_required {
                    text.push_str(&format!("- {}\n", action.instruction));
                }
            }
            if !report.actions_optional.is_empty() {
                text.push_str("optional_actions:\n");
                for action in &report.actions_optional {
                    text.push_str(&format!("- {}\n", action.instruction));
                }
            }
            Ok(text)
        }
    }
}

fn command_availability_summary(command: &CommandAvailability) -> String {
    if !command.discovered {
        "not discovered".to_owned()
    } else if command.selected_path_ready() {
        match &command.discovered_path {
            Some(path) => format!("ready on PATH ({path})"),
            None => "ready on PATH".to_owned(),
        }
    } else if let Some(path_match) = &command.path_match {
        format!("PATH resolves {path_match}, not the selected executable")
    } else {
        match &command.discovered_path {
            Some(path) => format!("selected executable is {path}; not on PATH"),
            None => "not on PATH".to_owned(),
        }
    }
}

pub(crate) fn profile_json(profile: &InstallationProfileRecord) -> Value {
    json!({
        "installation_id": profile.installation_id,
        "runtime_home_id": profile.runtime_home_id,
        "volicord_command": profile.volicord_command,
        "volicord_mcp_command": profile.volicord_mcp_command,
        "bin_dir": path_text(&profile.bin_dir),
        "default_connection_mode": profile.default_connection_mode,
        "created_at": profile.created_at,
        "updated_at": profile.updated_at,
    })
}

fn setup_metadata_json(
    volicord_source: &str,
    mcp_source: &str,
    link_bin: Option<&Path>,
    link_results: &BTreeMap<String, String>,
) -> Result<String, SetupCommandError> {
    serde_json::to_string(&json!({
        "created_by": SETUP_CREATED_BY,
        "volicord_command_source": volicord_source,
        "volicord_mcp_command_source": mcp_source,
        "link_bin": link_bin.map(path_text),
        "link_bin_requested": link_bin.is_some(),
        "link_results": link_results,
    }))
    .map_err(|error| SetupCommandError::Runtime(error.to_string()))
}

fn runtime_home_id_for_path(path: &Path) -> Result<String, SetupCommandError> {
    let path_text = path.to_str().ok_or_else(|| {
        SetupCommandError::Runtime("Runtime Home path must be valid UTF-8".to_owned())
    })?;
    let digest = Sha256::digest(path_text.as_bytes());
    Ok(format!(
        "runtime_home_{:016x}",
        u64::from_be_bytes([
            digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7],
        ])
    ))
}

fn command_parent(path: &Path) -> PathBuf {
    path.parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

pub(crate) fn path_text(path: &Path) -> String {
    path.display().to_string()
}

pub(crate) fn absolute_path(current_dir: &Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        current_dir.join(path)
    }
}

#[cfg(test)]
mod tests {
    use std::{env, ffi::OsString, io::Write};

    use rusqlite::Connection;
    use volicord_store::{bootstrap::installation_profile, sqlite::registry_db_path};
    use volicord_test_support::TempRuntimeHome;

    use super::*;

    #[derive(Debug)]
    struct FakeProcess {
        exe: PathBuf,
        env: BTreeMap<String, OsString>,
    }

    impl SetupProcess for FakeProcess {
        fn env_var(&self, name: &str) -> Option<OsString> {
            self.env.get(name).cloned()
        }

        fn current_exe(&self) -> Result<PathBuf, String> {
            Ok(self.exe.clone())
        }
    }

    #[test]
    fn setup_records_explicit_mcp_command() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("setup-explicit")?;
        let bin_dir = fixture.path().join("bin");
        let volicord = write_executable(&bin_dir, &volicord_binary_name())?;
        let mcp = write_executable(&bin_dir, &mcp_binary_name())?;
        let process = FakeProcess {
            exe: volicord,
            env: BTreeMap::new(),
        };

        let outcome = run_setup_command(
            &[
                "--home".to_owned(),
                path_text(fixture.path()),
                "--mcp-command".to_owned(),
                path_text(&mcp),
                "--json".to_owned(),
            ],
            fixture.path(),
            &process,
        )?;

        assert_eq!(outcome.status, CommandStatus::ActionRequired);
        let value: Value = serde_json::from_str(&outcome.output)?;
        assert_eq!(value["status"], "action_required");
        assert_eq!(
            value["setup_report"]["installation_profile"]["status"],
            "complete"
        );
        assert!(value["commands"]
            .as_array()
            .expect("commands should be an array")
            .iter()
            .any(|command| {
                command["id"] == "volicord_mcp_command"
                    && command["discovered_path"] == path_text(&mcp)
                    && command["available_on_path"] == false
            }));
        assert!(value["actions_required"]
            .as_array()
            .expect("actions_required should be an array")
            .iter()
            .any(|action| action["id"] == "make_volicord_mcp_command_available"));
        let profile = installation_profile(fixture.path())?.expect("profile should be stored");
        assert_eq!(profile.volicord_mcp_command, path_text(&mcp));
        assert_eq!(profile.default_connection_mode, CONNECTION_MODE_WORKFLOW);
        assert!(registry_db_path(fixture.path()).exists());
        Ok(())
    }

    #[test]
    fn setup_discovers_mcp_from_sibling() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("setup-sibling")?;
        let bin_dir = fixture.path().join("bin");
        let volicord = write_executable(&bin_dir, &volicord_binary_name())?;
        let mcp = write_executable(&bin_dir, &mcp_binary_name())?;
        let process = FakeProcess {
            exe: volicord,
            env: BTreeMap::new(),
        };

        run_setup_command(
            &["--home".to_owned(), path_text(fixture.path())],
            fixture.path(),
            &process,
        )?;

        let profile = installation_profile(fixture.path())?.expect("profile should be stored");
        assert_eq!(profile.volicord_mcp_command, path_text(&mcp));
        Ok(())
    }

    #[test]
    fn setup_discovers_mcp_from_path() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("setup-path")?;
        let exe_dir = fixture.path().join("exe");
        let path_dir = fixture.path().join("path-bin");
        let volicord = write_executable(&exe_dir, &volicord_binary_name())?;
        let mcp = write_executable(&path_dir, &mcp_binary_name())?;
        let process = FakeProcess {
            exe: volicord,
            env: BTreeMap::from([(PATH_ENV.to_owned(), env::join_paths([path_dir.as_path()])?)]),
        };

        run_setup_command(
            &["--home".to_owned(), path_text(fixture.path())],
            fixture.path(),
            &process,
        )?;

        let profile = installation_profile(fixture.path())?.expect("profile should be stored");
        assert_eq!(profile.volicord_mcp_command, path_text(&mcp));
        Ok(())
    }

    #[test]
    fn setup_json_reports_missing_mcp_as_failed() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("setup-missing-mcp")?;
        let bin_dir = fixture.path().join("bin");
        let volicord = write_executable(&bin_dir, &volicord_binary_name())?;
        let process = FakeProcess {
            exe: volicord,
            env: BTreeMap::new(),
        };

        let outcome = run_setup_command(
            &[
                "--home".to_owned(),
                path_text(fixture.path()),
                "--json".to_owned(),
            ],
            fixture.path(),
            &process,
        )?;

        assert_eq!(outcome.status, CommandStatus::Failed);
        let value: Value = serde_json::from_str(&outcome.output)?;
        assert_eq!(value["status"], "failed");
        assert!(value["installation_profile"].is_null());
        assert_eq!(
            value["setup_report"]["installation_profile"]["status"],
            "failed"
        );
        assert!(value["actions_required"]
            .as_array()
            .expect("actions_required should be an array")
            .iter()
            .any(|action| action["id"] == "select_mcp_command"));
        assert!(installation_profile(fixture.path())?.is_none());
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn setup_creates_requested_links() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("setup-links")?;
        let bin_dir = fixture.path().join("bin");
        let link_bin = fixture.path().join("links");
        let volicord = write_executable(&bin_dir, &volicord_binary_name())?;
        let mcp = write_executable(&bin_dir, &mcp_binary_name())?;
        let process = FakeProcess {
            exe: volicord.clone(),
            env: BTreeMap::from([(PATH_ENV.to_owned(), env::join_paths([link_bin.as_path()])?)]),
        };

        let outcome = run_setup_command(
            &[
                "--home".to_owned(),
                path_text(fixture.path()),
                "--mcp-command".to_owned(),
                path_text(&mcp),
                "--link-bin".to_owned(),
                path_text(&link_bin),
                "--json".to_owned(),
            ],
            fixture.path(),
            &process,
        )?;

        assert_eq!(outcome.status, CommandStatus::Complete);
        let value: Value = serde_json::from_str(&outcome.output)?;
        assert_eq!(value["status"], "complete");
        assert!(value["actions_performed"]
            .as_array()
            .expect("actions_performed should be an array")
            .iter()
            .any(|action| action["id"] == "create_volicord_link"));
        assert_eq!(
            fs::canonicalize(link_bin.join(volicord_binary_name()))?,
            volicord
        );
        assert_eq!(fs::canonicalize(link_bin.join(mcp_binary_name()))?, mcp);
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn setup_link_bin_reports_path_action_without_prompting(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("setup-links-path-action")?;
        let bin_dir = fixture.path().join("bin");
        let link_bin = fixture.path().join("links");
        let volicord = write_executable(&bin_dir, &volicord_binary_name())?;
        let mcp = write_executable(&bin_dir, &mcp_binary_name())?;
        let process = FakeProcess {
            exe: volicord.clone(),
            env: BTreeMap::new(),
        };

        let outcome = run_setup_command(
            &[
                "--home".to_owned(),
                path_text(fixture.path()),
                "--mcp-command".to_owned(),
                path_text(&mcp),
                "--link-bin".to_owned(),
                path_text(&link_bin),
                "--json".to_owned(),
            ],
            fixture.path(),
            &process,
        )?;

        assert_eq!(outcome.status, CommandStatus::ActionRequired);
        let value: Value = serde_json::from_str(&outcome.output)?;
        assert_eq!(value["status"], "action_required");
        assert!(value["actions_required"]
            .as_array()
            .expect("actions_required should be an array")
            .iter()
            .any(|action| action["id"] == "add_link_bin_to_path"));
        assert_eq!(
            fs::canonicalize(link_bin.join(volicord_binary_name()))?,
            volicord
        );
        assert_eq!(fs::canonicalize(link_bin.join(mcp_binary_name()))?, mcp);
        Ok(())
    }

    #[test]
    fn setup_link_bin_error_still_saves_profile_when_possible(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("setup-link-bin-file")?;
        let bin_dir = fixture.path().join("bin");
        let link_bin = fixture.path().join("not-a-directory");
        fs::write(&link_bin, "not a directory")?;
        let volicord = write_executable(&bin_dir, &volicord_binary_name())?;
        let mcp = write_executable(&bin_dir, &mcp_binary_name())?;
        let process = FakeProcess {
            exe: volicord,
            env: BTreeMap::new(),
        };

        let outcome = run_setup_command(
            &[
                "--home".to_owned(),
                path_text(fixture.path()),
                "--mcp-command".to_owned(),
                path_text(&mcp),
                "--link-bin".to_owned(),
                path_text(&link_bin),
                "--json".to_owned(),
            ],
            fixture.path(),
            &process,
        )?;

        assert_eq!(outcome.status, CommandStatus::ActionRequired);
        let value: Value = serde_json::from_str(&outcome.output)?;
        assert_eq!(
            value["setup_report"]["installation_profile"]["status"],
            "complete"
        );
        assert!(value["actions_required"]
            .as_array()
            .expect("actions_required should be an array")
            .iter()
            .any(|action| action["id"] == "repair_link_bin"));
        assert!(!value["actions_required"]
            .as_array()
            .expect("actions_required should be an array")
            .iter()
            .any(|action| action["id"] == "add_link_bin_to_path"));
        assert!(installation_profile(fixture.path())?.is_some());
        Ok(())
    }

    fn write_executable(dir: &Path, name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        fs::create_dir_all(dir)?;
        let path = dir.join(name);
        let mut file = fs::File::create(&path)?;
        writeln!(file, "#!/bin/sh")?;
        make_executable(&path)?;
        Ok(path)
    }

    #[cfg(unix)]
    fn make_executable(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
        Ok(())
    }

    #[cfg(not(unix))]
    fn make_executable(_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    #[test]
    fn runtime_home_id_is_stable_for_same_path() {
        let path = Path::new("/tmp/volicord-id-test");

        assert_eq!(
            runtime_home_id_for_path(path).unwrap(),
            runtime_home_id_for_path(path).unwrap()
        );
    }

    #[test]
    fn installation_profile_table_can_be_read_after_setup() -> Result<(), Box<dyn std::error::Error>>
    {
        let fixture = TempRuntimeHome::new("setup-sql")?;
        let bin_dir = fixture.path().join("bin");
        let volicord = write_executable(&bin_dir, &volicord_binary_name())?;
        let mcp = write_executable(&bin_dir, &mcp_binary_name())?;
        let process = FakeProcess {
            exe: volicord,
            env: BTreeMap::new(),
        };

        run_setup_command(
            &[
                "--home".to_owned(),
                path_text(fixture.path()),
                "--mcp-command".to_owned(),
                path_text(&mcp),
            ],
            fixture.path(),
            &process,
        )?;

        let conn = Connection::open(registry_db_path(fixture.path()))?;
        let count: i64 =
            conn.query_row("SELECT COUNT(*) FROM installation_profile", [], |row| {
                row.get(0)
            })?;
        assert_eq!(count, 1);
        Ok(())
    }
}
