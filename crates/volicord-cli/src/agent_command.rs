use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fmt, fs,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use volicord_store::{
    agent_connections::{
        add_connection_project, agent_connection_record, ensure_agent_connection,
        list_agent_connections, list_connection_projects, remove_agent_connection_if_unused,
        remove_connection_project, set_connection_enabled, set_connection_mode,
        update_agent_connection_verification_report, AgentConnectionRecord,
        AgentConnectionRegistration, ConnectionProjectRecord, ConnectionProjectRegistration,
        CONNECTION_INTENT_GLOBAL, CONNECTION_INTENT_PERSONAL, CONNECTION_INTENT_SHARED,
        CONNECTION_MODE_READ_ONLY, CONNECTION_MODE_WORKFLOW, HOST_KIND_CLAUDE_CODE,
        HOST_KIND_CODEX, HOST_KIND_GENERIC, HOST_SCOPE_EXPORT, HOST_SCOPE_LOCAL,
        HOST_SCOPE_PROJECT, HOST_SCOPE_USER, VERIFIED_STATUS_ACTION_REQUIRED,
        VERIFIED_STATUS_COMPLETE, VERIFIED_STATUS_FAILED, VERIFIED_STATUS_NOT_VERIFIED,
    },
    bootstrap::{
        ensure_project_for_repo, initialize_runtime_home, installation_profile, list_projects,
        project_record, project_record_by_repo_root, register_project, validate_project_id,
        InstallationProfileRecord, ProjectRecord, ProjectRegistration, RepoProjectRegistration,
        ACTIVE_PROJECT_STATUS,
    },
    runtime_home::{resolve_runtime_home, RuntimeHomeResolutionError},
    StoreError,
};

use crate::host_integration::{
    claude_code::{ClaudeCodeAdapter, ProductionCommandRunner},
    codex::{CodexAdapter, CodexEnvironment, CodexExistingPlanRequest},
    export_file_name,
    generic::{GenericAdapter, GenericExportRequest},
    is_valid_server_name,
    verification::{Verification, VerificationStatus},
    ConnectionIntent, HostAdapter, HostConfigError, HostKind, HostPlan, HostPlanRequest,
    HostRemoveRequest, HostScope, HostTarget, InstallationProfile, ManagedServerEntry,
    PlannedChange, ProjectContext, UserAction, UserActionKind,
};

const VOLICORD_HOME: &str = "VOLICORD_HOME";
const PATH_ENV: &str = "PATH";
const AGENT_METADATA_CREATED_BY: &str = "volicord_cli_agent_connection";
const AGENT_RUNTIME_HOME_ID: &str = "runtime_home_agent";
const DEFAULT_MCP_COMMAND: &str = "volicord-mcp";
const DEFAULT_SERVER_NAME: &str = "volicord";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

const WORKFLOW_TOOL_NAMES: [&str; 9] = [
    "volicord.intake",
    "volicord.update_scope",
    "volicord.status",
    "volicord.prepare_write",
    "volicord.stage_artifact",
    "volicord.record_run",
    "volicord.request_user_judgment",
    "volicord.close_task",
    "volicord.list_projects",
];
const READ_ONLY_TOOL_NAMES: [&str; 3] = [
    "volicord.status",
    "volicord.close_task",
    "volicord.list_projects",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentCommandError {
    Usage(String),
    Runtime(String),
    FailureOutput(String),
}

impl AgentCommandError {
    fn usage(message: impl Into<String>) -> Self {
        Self::Usage(message.into())
    }

    fn runtime(message: impl Into<String>) -> Self {
        Self::Runtime(message.into())
    }
}

impl fmt::Display for AgentCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) | Self::Runtime(message) | Self::FailureOutput(message) => {
                formatter.write_str(message)
            }
        }
    }
}

impl std::error::Error for AgentCommandError {}

impl From<StoreError> for AgentCommandError {
    fn from(error: StoreError) -> Self {
        Self::runtime(error.to_string())
    }
}

impl From<RuntimeHomeResolutionError> for AgentCommandError {
    fn from(error: RuntimeHomeResolutionError) -> Self {
        Self::runtime(error.to_string())
    }
}

impl From<HostConfigError> for AgentCommandError {
    fn from(error: HostConfigError) -> Self {
        Self::runtime(error.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentProcessOutput {
    pub success: bool,
    pub status_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpLaunch {
    command: PathBuf,
    args: Vec<String>,
    env: BTreeMap<String, String>,
    cwd: Option<PathBuf>,
}

pub trait AgentProcess {
    fn env_var(&self, name: &str) -> Option<OsString>;
    fn current_exe(&self) -> Result<PathBuf, String>;
    fn run_preflight(
        &mut self,
        launch: &McpLaunch,
        runtime_home: &Path,
        connection_id: &str,
        project_id: Option<&str>,
    ) -> Result<AgentProcessOutput, String>;
    fn verify_mcp_stdio(
        &mut self,
        launch: &McpLaunch,
        runtime_home: &Path,
        connection_id: &str,
        mode: &str,
    ) -> Result<McpVerification, String>;
}

pub struct ProductionAgentProcess;

impl AgentProcess for ProductionAgentProcess {
    fn env_var(&self, name: &str) -> Option<OsString> {
        std::env::var_os(name)
    }

    fn current_exe(&self) -> Result<PathBuf, String> {
        std::env::current_exe()
            .map_err(|error| format!("failed to read current executable: {error}"))
    }

    fn run_preflight(
        &mut self,
        launch: &McpLaunch,
        runtime_home: &Path,
        connection_id: &str,
        project_id: Option<&str>,
    ) -> Result<AgentProcessOutput, String> {
        let mut child = Command::new(&launch.command);
        child.arg("--check").arg("--connection").arg(connection_id);
        if let Some(project_id) = project_id {
            child.arg("--project").arg(project_id);
        }
        apply_mcp_launch_context(&mut child, launch, runtime_home);
        child.stdin(Stdio::null());
        let output = child.output().map_err(|error| {
            format!(
                "failed to run {} --check --connection {}: {error}",
                launch.command.display(),
                connection_id
            )
        })?;
        Ok(AgentProcessOutput {
            success: output.status.success(),
            status_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }

    fn verify_mcp_stdio(
        &mut self,
        launch: &McpLaunch,
        runtime_home: &Path,
        connection_id: &str,
        mode: &str,
    ) -> Result<McpVerification, String> {
        verify_mcp_stdio_process(launch, runtime_home, connection_id, mode, DEFAULT_TIMEOUT)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AgentResultStatus {
    Complete,
    ActionRequired,
    Failed,
    NotVerified,
    DryRun,
}

impl AgentResultStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::ActionRequired => "action_required",
            Self::Failed => "failed",
            Self::NotVerified => "not_verified",
            Self::DryRun => "dry_run",
        }
    }

    fn store_status(self) -> &'static str {
        match self {
            Self::Complete => VERIFIED_STATUS_COMPLETE,
            Self::ActionRequired => VERIFIED_STATUS_ACTION_REQUIRED,
            Self::Failed => VERIFIED_STATUS_FAILED,
            Self::NotVerified | Self::DryRun => VERIFIED_STATUS_NOT_VERIFIED,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StepStatus {
    Passed,
    Failed,
    Skipped,
}

impl StepStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
        }
    }
}

#[derive(Debug, Clone)]
struct VerificationStep {
    status: StepStatus,
    details: String,
}

impl VerificationStep {
    fn passed(details: impl Into<String>) -> Self {
        Self {
            status: StepStatus::Passed,
            details: details.into(),
        }
    }

    fn failed(details: impl Into<String>) -> Self {
        Self {
            status: StepStatus::Failed,
            details: details.into(),
        }
    }

    fn skipped(details: impl Into<String>) -> Self {
        Self {
            status: StepStatus::Skipped,
            details: details.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct McpVerification {
    step: VerificationStep,
    tools: Vec<String>,
}

impl McpVerification {
    fn passed(tools: Vec<String>) -> Self {
        Self {
            step: VerificationStep::passed(format!("tools/list returned {} tools", tools.len())),
            tools,
        }
    }

    fn failed(details: impl Into<String>) -> Self {
        Self {
            step: VerificationStep::failed(details),
            tools: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct VerificationReport {
    status: AgentResultStatus,
    host: Verification,
    preflight: VerificationStep,
    handshake: VerificationStep,
    tools: Vec<String>,
}

#[derive(Debug, Clone)]
struct ParsedAgentOptions {
    runtime_home: Option<PathBuf>,
    repo_root: Option<PathBuf>,
    project_id: Option<String>,
    connection_id: Option<String>,
    mode: Option<String>,
    host_kind: Option<HostKind>,
    host_scope: Option<HostScope>,
    server_name: Option<String>,
    export_path: Option<PathBuf>,
    export_dir: Option<PathBuf>,
    output: OutputFormat,
    dry_run: bool,
    allow_repository_write: bool,
    replace_managed: bool,
}

impl Default for ParsedAgentOptions {
    fn default() -> Self {
        Self {
            runtime_home: None,
            repo_root: None,
            project_id: None,
            connection_id: None,
            mode: None,
            host_kind: None,
            host_scope: None,
            server_name: None,
            export_path: None,
            export_dir: None,
            output: OutputFormat::Text,
            dry_run: false,
            allow_repository_write: false,
            replace_managed: false,
        }
    }
}

pub fn agent_usage() -> String {
    concat!(
        "volicord agent connect --host codex|claude-code|claude_code --scope user|project|local [--project-id ID] [--repo-root PATH] [--connection-id ID] [--mode read_only|workflow] [--server-name NAME] [--output text|json] [--dry-run] [--allow-repository-write] [--replace-managed]\n",
        "volicord agent list [--output text|json]\n",
        "volicord agent status --connection-id ID [--output text|json]\n",
        "volicord agent enable --connection-id ID [--output text|json]\n",
        "volicord agent disable --connection-id ID [--output text|json]\n",
        "volicord agent project add --connection-id ID --project-id ID [--repo-root PATH] [--output text|json] [--dry-run]\n",
        "volicord agent project remove --connection-id ID --project-id ID [--output text|json] [--dry-run]\n",
        "volicord agent verify --connection-id ID [--output text|json]\n",
        "volicord agent uninstall --connection-id ID [--output text|json] [--dry-run] [--allow-repository-write]\n"
    )
    .to_owned()
}

fn agent_connect_usage() -> String {
    concat!(
        "Usage:\n",
        "  volicord agent connect --host codex|claude-code|claude_code --scope user|project|local [--project-id ID] [--repo-root PATH] [--connection-id ID] [--mode read_only|workflow] [--server-name NAME] [--output text|json] [--dry-run] [--allow-repository-write] [--replace-managed]\n",
        "\n",
        "Defaults:\n",
        "  --mode defaults to the setup profile default, which is workflow after volicord setup.\n",
        "  --server-name defaults to volicord.\n",
        "  volicord-mcp command location comes from volicord setup.\n",
        "  Project and local scopes allow one selected project by default.\n",
        "  User scope may allow more projects with volicord agent project add.\n"
    )
    .to_owned()
}

pub fn connect_usage() -> String {
    "volicord connect [HOST] [--repo PATH] [--shared|--global] [--read-only] [--dry-run] [--json]\n"
        .to_owned()
}

pub fn connections_usage() -> String {
    "volicord connections [--repo PATH] [--json]\n".to_owned()
}

pub fn connection_usage() -> String {
    concat!(
        "volicord connection status [HOST] [--repo PATH] [--shared|--global] [--json]\n",
        "volicord connection verify [HOST] [--repo PATH] [--shared|--global] [--json]\n",
        "volicord connection mode [HOST] workflow|read-only [--repo PATH] [--shared|--global] [--json]\n",
        "volicord connection remove [HOST] [--repo PATH] [--shared|--global] [--dry-run] [--json]\n"
    )
    .to_owned()
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ParsedConnectionOptions {
    host_kind: Option<HostKind>,
    repo: Option<PathBuf>,
    shared: bool,
    global: bool,
    read_only: bool,
    dry_run: bool,
    json: bool,
    positionals: Vec<String>,
}

#[derive(Debug, Clone)]
struct ConnectionSelector {
    host_kind: HostKind,
    intent: ConnectionIntent,
    host_scope: HostScope,
    repo_root: PathBuf,
}

pub fn run_connect_command(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    if is_help_request(args) {
        return Ok(connect_usage());
    }
    let parsed = parse_connection_options(
        args,
        &["repo", "shared", "global", "read-only", "dry-run", "json"],
        1,
    )?;
    let host_kind = resolve_connection_host(parsed.host_kind, process)?;
    let intent = connection_intent_from_flags(&parsed)?;
    let host_scope = host_scope_for_intent(host_kind, intent)?;
    let mode = if parsed.read_only {
        CONNECTION_MODE_READ_ONLY
    } else {
        CONNECTION_MODE_WORKFLOW
    };
    let runtime_home = resolve_runtime_home(|name| process.env_var(name), current_dir)?;
    let setup_profile = required_installation_profile(&runtime_home)?;
    let repo_root = resolve_connection_repo_root(current_dir, parsed.repo.as_deref())?;
    let server_name = DEFAULT_SERVER_NAME.to_owned();
    let target_hint = connection_target_hint(
        host_kind,
        host_scope,
        Some(&repo_root),
        &ParsedAgentOptions::default(),
        process,
        &server_name,
        None,
    )?;
    let existing = connection_for_host_target(
        &runtime_home,
        host_kind,
        intent,
        host_scope,
        &target_hint,
        &server_name,
    )?;
    let connection_id = existing
        .as_ref()
        .map(|connection| connection.connection_id.clone())
        .unwrap_or_else(|| {
            deterministic_connection_id(
                host_kind,
                host_scope,
                Some(&path_text(&repo_root)),
                &target_hint,
                &server_name,
            )
        });
    let project_hint = project_record_by_repo_root(&runtime_home, &repo_root)
        .ok()
        .flatten();
    let expected_fingerprint = existing
        .as_ref()
        .map(|connection| connection.managed_fingerprint.as_str());
    let host_plan = build_host_plan(
        BuildHostPlanRequest {
            host_kind,
            connection_intent: intent,
            connection_id: &connection_id,
            repo_root: Some(&repo_root),
            project_id: project_hint
                .as_ref()
                .map(|project| project.project_id.as_str())
                .or(Some("planned_project")),
            project_name: project_hint
                .as_ref()
                .map(|project| project.project_name.as_str())
                .or(Some("planned project")),
            installation_profile: installation_profile_context(&runtime_home, &setup_profile),
            mode,
            expected_fingerprint,
            export_target: None,
            export_dir: None,
            current_dir,
        },
        process,
    )?;
    if let Some(conflict) = host_plan.conflicts.first() {
        return Err(AgentCommandError::runtime(conflict.message.clone()));
    }
    if parsed.dry_run {
        return render_simplified_plan_output(SimplifiedPlanOutput {
            format: connection_output_format(&parsed),
            action: "connect",
            status: AgentResultStatus::DryRun,
            connection_id: &connection_id,
            host_kind,
            intent,
            host_scope,
            mode,
            enabled: true,
            repo_root: Some(&repo_root),
            plan: &host_plan,
            projects_remaining: None,
            user_actions: host_plan.user_actions.clone(),
        });
    }

    initialize_runtime_home(
        &runtime_home,
        AGENT_RUNTIME_HOME_ID,
        metadata_json_base()?.as_str(),
    )?;
    let project = ensure_project_for_repo(
        &runtime_home,
        RepoProjectRegistration {
            project_name: None,
            project_alias: None,
            repo_root: repo_root.clone(),
            project_home: None,
            status: ACTIVE_PROJECT_STATUS.to_owned(),
            metadata_json: metadata_json_base()?,
        },
    )?;
    let existing = connection_for_host_target(
        &runtime_home,
        host_kind,
        intent,
        host_scope,
        &target_hint,
        &server_name,
    )?;
    let expected_fingerprint = existing
        .as_ref()
        .map(|connection| connection.managed_fingerprint.as_str());
    let host_plan = build_host_plan(
        BuildHostPlanRequest {
            host_kind,
            connection_intent: intent,
            connection_id: &connection_id,
            repo_root: Some(&project.repo_root),
            project_id: Some(&project.project_id),
            project_name: Some(&project.project_name),
            installation_profile: installation_profile_context(&runtime_home, &setup_profile),
            mode,
            expected_fingerprint,
            export_target: None,
            export_dir: None,
            current_dir,
        },
        process,
    )?;
    if let Some(conflict) = host_plan.conflicts.first() {
        return Err(AgentCommandError::runtime(conflict.message.clone()));
    }
    let mcp_command = PathBuf::from(&host_plan.entry.command);
    let metadata_json = connection_metadata_json(&host_plan, &mcp_command, &runtime_home)?;
    let mut connection = ensure_agent_connection(
        &runtime_home,
        AgentConnectionRegistration {
            connection_id: connection_id.clone(),
            host_kind: host_kind.as_str().to_owned(),
            intent: intent.as_str().to_owned(),
            host_scope: host_scope.as_str().to_owned(),
            server_name: host_plan.server_name.clone(),
            config_target: host_target_text(&host_plan.target),
            mode: mode.to_owned(),
            enabled: true,
            managed_fingerprint: host_plan.fingerprint.clone(),
            last_verified_status: existing
                .as_ref()
                .map(|record| record.last_verified_status.clone())
                .unwrap_or_else(|| VERIFIED_STATUS_NOT_VERIFIED.to_owned()),
            last_verification_report_json: existing
                .as_ref()
                .map(|record| record.last_verification_report_json.clone())
                .unwrap_or_else(|| "{}".to_owned()),
            last_user_actions_json: user_actions_json(&host_plan.user_actions)?,
            metadata_json,
        },
    )?;
    enforce_single_project_scope(&runtime_home, &connection, &project.project_id)?;
    add_connection_project(
        &runtime_home,
        ConnectionProjectRegistration {
            connection_id: connection.connection_id.clone(),
            project_id: project.project_id.clone(),
        },
    )?;
    apply_host_plan(host_kind, &host_plan, process)?;
    let launch = mcp_launch_from_host_plan(&host_plan, Some(&project.repo_root));
    let verification = verify_connection(
        &runtime_home,
        &connection,
        &host_plan,
        &launch,
        Some(&project.project_id),
        process,
    )?;
    connection = update_agent_connection_verification_report(
        &runtime_home,
        &connection.connection_id,
        verification.status.store_status(),
        &host_plan.fingerprint,
        &verification_report_json(&verification)?,
        &user_actions_json(&verification.host.user_actions)?,
    )?;
    let projects = list_connection_projects(&runtime_home, &connection.connection_id)?;
    render_simplified_connection_output(SimplifiedConnectionOutput {
        format: connection_output_format(&parsed),
        action: "connected",
        status: verification.status,
        connection: &connection,
        projects: &projects,
        verification: Some(&verification),
        plan: Some(&host_plan),
        user_actions: verification.host.user_actions.clone(),
    })
}

pub fn run_connections_command(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    if is_help_request(args) {
        return Ok(connections_usage());
    }
    let parsed = parse_connection_options(args, &["repo", "json"], 0)?;
    let runtime_home = resolve_runtime_home(|name| process.env_var(name), current_dir)?;
    let repo_root = parsed
        .repo
        .as_deref()
        .map(|repo| resolve_connection_repo_root(current_dir, Some(repo)))
        .transpose()?;
    let mut rows = Vec::new();
    for connection in list_agent_connections(&runtime_home)? {
        let projects = list_connection_projects(&runtime_home, &connection.connection_id)?;
        if repo_root.as_ref().is_none_or(|repo_root| {
            projects
                .iter()
                .any(|project| project.project.repo_root == *repo_root)
        }) {
            rows.push((connection, projects));
        }
    }
    render_simplified_connections_output(connection_output_format(&parsed), &rows)
}

pub fn run_connection_command(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    let Some(subcommand) = args.first().map(String::as_str) else {
        return Ok(connection_usage());
    };
    if matches!(subcommand, "-h" | "--help" | "help") {
        if args.len() == 1 {
            return Ok(connection_usage());
        }
        return Err(AgentCommandError::usage(format!(
            "unexpected argument: {}\n\n{}",
            args[1],
            connection_usage()
        )));
    }
    match subcommand {
        "status" => command_connection_status(&args[1..], current_dir, process),
        "verify" => command_connection_verify(&args[1..], current_dir, process),
        "mode" => command_connection_mode(&args[1..], current_dir, process),
        "remove" => command_connection_remove(&args[1..], current_dir, process),
        other => Err(AgentCommandError::usage(format!(
            "unknown connection command: {other}\n\n{}",
            connection_usage()
        ))),
    }
}

fn command_connection_status(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    if is_help_request(args) {
        return Ok(connection_usage());
    }
    let parsed = parse_connection_options(args, &["repo", "shared", "global", "json"], 1)?;
    let runtime_home = resolve_runtime_home(|name| process.env_var(name), current_dir)?;
    let selector = connection_selector(&parsed, current_dir, process)?;
    let (connection, projects) = select_connection(&runtime_home, &selector)?;
    render_simplified_connection_output(SimplifiedConnectionOutput {
        format: connection_output_format(&parsed),
        action: "status",
        status: status_from_store(&connection.last_verification_status),
        user_actions: stored_user_actions(&connection),
        connection: &connection,
        projects: &projects,
        verification: None,
        plan: None,
    })
}

fn command_connection_verify(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    if is_help_request(args) {
        return Ok(connection_usage());
    }
    let parsed = parse_connection_options(args, &["repo", "shared", "global", "json"], 1)?;
    let runtime_home = resolve_runtime_home(|name| process.env_var(name), current_dir)?;
    let selector = connection_selector(&parsed, current_dir, process)?;
    let (mut connection, _) = select_connection(&runtime_home, &selector)?;
    let host_plan = existing_host_plan(&connection, &runtime_home, process)?;
    let launch = mcp_launch_from_host_plan(&host_plan, None);
    let verification = verify_connection(
        &runtime_home,
        &connection,
        &host_plan,
        &launch,
        None,
        process,
    )?;
    connection = update_agent_connection_verification_report(
        &runtime_home,
        &connection.connection_id,
        verification.status.store_status(),
        &host_plan.fingerprint,
        &verification_report_json(&verification)?,
        &user_actions_json(&verification.host.user_actions)?,
    )?;
    let projects = list_connection_projects(&runtime_home, &connection.connection_id)?;
    render_simplified_connection_output(SimplifiedConnectionOutput {
        format: connection_output_format(&parsed),
        action: "verified",
        status: verification.status,
        user_actions: verification.host.user_actions.clone(),
        connection: &connection,
        projects: &projects,
        verification: Some(&verification),
        plan: Some(&host_plan),
    })
}

fn command_connection_mode(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    if is_help_request(args) {
        return Ok(connection_usage());
    }
    let parsed = parse_connection_options(args, &["repo", "shared", "global", "json"], 2)?;
    let (host_kind, mode) = mode_positionals(&parsed, process)?;
    let parsed = ParsedConnectionOptions {
        host_kind: Some(host_kind),
        ..parsed
    };
    let runtime_home = resolve_runtime_home(|name| process.env_var(name), current_dir)?;
    let selector = connection_selector(&parsed, current_dir, process)?;
    let (connection, _) = select_connection(&runtime_home, &selector)?;
    let mut connection = set_connection_mode(&runtime_home, &connection.connection_id, &mode)?;
    let mut actions = stored_or_default_user_actions(
        &connection,
        parse_host_kind(&connection.host_kind)?,
        parse_host_scope(&connection.host_scope)?,
    );
    actions.push(UserAction::new(
        UserActionKind::ReloadRequired,
        "Restart or reload the host so it refreshes the Volicord tool list for the selected mode",
    ));
    connection = update_agent_connection_verification_report(
        &runtime_home,
        &connection.connection_id,
        &connection.last_verification_status,
        &connection.managed_fingerprint,
        &connection.last_verification_report_json,
        &user_actions_json(&actions)?,
    )?;
    let projects = list_connection_projects(&runtime_home, &connection.connection_id)?;
    render_simplified_connection_output(SimplifiedConnectionOutput {
        format: connection_output_format(&parsed),
        action: "mode_updated",
        status: status_from_store(&connection.last_verification_status),
        user_actions: actions,
        connection: &connection,
        projects: &projects,
        verification: None,
        plan: None,
    })
}

fn command_connection_remove(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    if is_help_request(args) {
        return Ok(connection_usage());
    }
    let parsed =
        parse_connection_options(args, &["repo", "shared", "global", "dry-run", "json"], 1)?;
    let runtime_home = resolve_runtime_home(|name| process.env_var(name), current_dir)?;
    let selector = connection_selector(&parsed, current_dir, process)?;
    let (connection, projects) = select_connection(&runtime_home, &selector)?;
    let selected_project = projects
        .iter()
        .find(|project| project.project.repo_root == selector.repo_root)
        .ok_or_else(|| AgentCommandError::runtime("selected repository is not connected"))?;
    let remaining_count = projects.len().saturating_sub(1);
    let host_plan = if remaining_count == 0 {
        Some(existing_host_plan(&connection, &runtime_home, process)?)
    } else {
        None
    };
    if parsed.dry_run {
        let plan = host_plan
            .as_ref()
            .map(SimplifiedRemovePlan::Host)
            .unwrap_or(SimplifiedRemovePlan::MembershipOnly);
        return render_simplified_remove_dry_run(
            connection_output_format(&parsed),
            &connection,
            &projects,
            selected_project,
            plan,
            remaining_count,
        );
    }

    remove_connection_project(
        &runtime_home,
        &connection.connection_id,
        &selected_project.project_id,
    )?;
    let remaining_projects = list_connection_projects(&runtime_home, &connection.connection_id)?;
    if remaining_projects.is_empty() {
        if let Some(host_plan) = &host_plan {
            remove_host_configuration(host_plan, &connection, process)?;
        }
        remove_agent_connection_if_unused(&runtime_home, &connection.connection_id)?;
    }
    render_simplified_connection_output(SimplifiedConnectionOutput {
        format: connection_output_format(&parsed),
        action: "removed",
        status: AgentResultStatus::Complete,
        user_actions: Vec::new(),
        connection: &connection,
        projects: &remaining_projects,
        verification: None,
        plan: host_plan.as_ref(),
    })
}

pub fn run_agent_command(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    let Some(subcommand) = args.first().map(String::as_str) else {
        return Ok(agent_usage());
    };

    match subcommand {
        "-h" | "--help" | "help" => {
            if args.len() == 1 {
                Ok(agent_usage())
            } else {
                Err(AgentCommandError::usage(format!(
                    "unexpected argument: {}\n\n{}",
                    args[1],
                    agent_usage()
                )))
            }
        }
        "connect" => command_connect(&args[1..], current_dir, process),
        "list" => command_list(&args[1..], current_dir, process),
        "status" => command_status(&args[1..], current_dir, process),
        "enable" => command_enable_disable(&args[1..], current_dir, process, true),
        "disable" => command_enable_disable(&args[1..], current_dir, process, false),
        "project" => command_project(&args[1..], current_dir, process),
        "verify" => command_verify(&args[1..], current_dir, process),
        "uninstall" => command_uninstall(&args[1..], current_dir, process),
        other => Err(AgentCommandError::usage(format!(
            "unknown agent command: {other}\n\n{}",
            agent_usage()
        ))),
    }
}

fn command_connect(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    if is_help_request(args) {
        return Ok(agent_connect_usage());
    }
    let parsed = parse_agent_options(args, connect_allowed_options())?;
    let host_kind = required_host_kind(&parsed)?;
    let host_scope = required_host_scope(&parsed)?;
    let connection_intent = connection_intent_for_host_scope(host_kind, host_scope)?;
    validate_host_scope(host_kind, host_scope)?;
    validate_repository_write_permission(&parsed, host_scope)?;
    let server_name = parsed
        .server_name
        .clone()
        .unwrap_or_else(|| DEFAULT_SERVER_NAME.to_owned());
    validate_server_name(&server_name)?;
    let runtime_home = resolve_agent_runtime_home(&parsed, current_dir, process)?;
    let setup_profile = required_installation_profile(&runtime_home)?;
    let mode = parse_connection_mode(
        parsed
            .mode
            .as_deref()
            .unwrap_or(setup_profile.default_connection_mode.as_str()),
    )?;
    let repo_root = resolve_optional_repo_root(parsed.repo_root.as_deref(), current_dir)?;
    let export_target = resolve_export_target(&parsed, current_dir, None);

    if parsed.dry_run {
        let project = resolve_selected_project_for_dry_run(&parsed, repo_root.as_deref())?;
        let target_hint = connection_target_hint(
            host_kind,
            host_scope,
            project.repo_root.as_deref(),
            &parsed,
            process,
            &server_name,
            export_target.as_deref(),
        )?;
        let connection_id = parsed.connection_id.clone().unwrap_or_else(|| {
            deterministic_connection_id(
                host_kind,
                host_scope,
                project.project_id.as_deref(),
                &target_hint,
                &server_name,
            )
        });
        return render_dry_run_output(
            parsed.output,
            DryRunRenderData {
                action: "connect",
                connection_id: &connection_id,
                host_kind,
                host_scope,
                mode: &mode,
                server_name: &server_name,
                config_target: &target_hint,
                project_id: project.project_id.as_deref(),
            },
        );
    }

    initialize_runtime_home(
        &runtime_home,
        AGENT_RUNTIME_HOME_ID,
        metadata_json_base()?.as_str(),
    )?;
    let project = resolve_or_register_project(
        &runtime_home,
        parsed.project_id.as_deref(),
        repo_root.as_deref(),
    )?;
    let export_target =
        resolve_export_target(&parsed, current_dir, parsed.connection_id.as_deref());
    let target_hint = connection_target_hint(
        host_kind,
        host_scope,
        Some(&project.repo_root),
        &parsed,
        process,
        &server_name,
        export_target.as_deref(),
    )?;
    let connection_id = parsed.connection_id.clone().unwrap_or_else(|| {
        deterministic_connection_id(
            host_kind,
            host_scope,
            Some(&project.project_id),
            &target_hint,
            &server_name,
        )
    });
    let existing = agent_connection_record(&runtime_home, &connection_id)?;
    let expected_fingerprint = existing
        .as_ref()
        .map(|record| record.managed_fingerprint.as_str());
    let host_plan = build_host_plan(
        BuildHostPlanRequest {
            host_kind,
            connection_intent,
            connection_id: &connection_id,
            repo_root: Some(&project.repo_root),
            project_id: Some(&project.project_id),
            project_name: Some(&project.project_name),
            installation_profile: installation_profile_context(&runtime_home, &setup_profile),
            mode: &mode,
            expected_fingerprint,
            export_target: export_target.as_deref(),
            export_dir: parsed.export_dir.as_deref(),
            current_dir,
        },
        process,
    )?;
    if let Some(conflict) = host_plan.conflicts.first() {
        return Err(AgentCommandError::runtime(conflict.message.clone()));
    }
    let mcp_command = PathBuf::from(&host_plan.entry.command);
    let metadata_json = connection_metadata_json(&host_plan, &mcp_command, &runtime_home)?;
    let mut connection = ensure_agent_connection(
        &runtime_home,
        AgentConnectionRegistration {
            connection_id: connection_id.clone(),
            host_kind: host_kind.as_str().to_owned(),
            intent: connection_intent.as_str().to_owned(),
            host_scope: host_scope.as_str().to_owned(),
            server_name: host_plan.server_name.clone(),
            config_target: host_target_text(&host_plan.target),
            mode: mode.clone(),
            enabled: true,
            managed_fingerprint: host_plan.fingerprint.clone(),
            last_verified_status: existing
                .as_ref()
                .map(|record| record.last_verified_status.clone())
                .unwrap_or_else(|| VERIFIED_STATUS_NOT_VERIFIED.to_owned()),
            last_verification_report_json: existing
                .as_ref()
                .map(|record| record.last_verification_report_json.clone())
                .unwrap_or_else(|| "{}".to_owned()),
            last_user_actions_json: user_actions_json(&host_plan.user_actions)?,
            metadata_json,
        },
    )?;
    enforce_single_project_scope(&runtime_home, &connection, &project.project_id)?;
    add_connection_project(
        &runtime_home,
        ConnectionProjectRegistration {
            connection_id: connection.connection_id.clone(),
            project_id: project.project_id.clone(),
        },
    )?;
    apply_host_plan(host_kind, &host_plan, process)?;
    let launch = mcp_launch_from_host_plan(&host_plan, Some(&project.repo_root));
    let verification = verify_connection(
        &runtime_home,
        &connection,
        &host_plan,
        &launch,
        Some(&project.project_id),
        process,
    )?;
    connection = update_agent_connection_verification_report(
        &runtime_home,
        &connection.connection_id,
        verification.status.store_status(),
        &host_plan.fingerprint,
        &verification_report_json(&verification)?,
        &user_actions_json(&verification.host.user_actions)?,
    )?;
    let projects = list_connection_projects(&runtime_home, &connection.connection_id)?;
    render_connection_output(
        parsed.output,
        "connected",
        verification.status,
        &connection,
        &projects,
        Some(&verification),
    )
}

fn command_list(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    let parsed = parse_agent_options(args, list_allowed_options())?;
    let runtime_home = resolve_agent_runtime_home(&parsed, current_dir, process)?;
    let connections = list_agent_connections(&runtime_home)?;
    match parsed.output {
        OutputFormat::Text => {
            let mut output = String::from(
                "connection_id\thost_kind\thost_scope\tmode\tenabled\tconnected_projects\tverification_status\tserver_name\tconfig_target\n",
            );
            for connection in connections {
                let projects = project_ids_or_empty(&runtime_home, &connection.connection_id)?;
                output.push_str(&format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                    connection.connection_id,
                    connection.host_kind,
                    connection.host_scope,
                    connection.mode,
                    connection.enabled,
                    projects.join(","),
                    connection.last_verified_status,
                    connection.server_name,
                    connection.config_target
                ));
            }
            Ok(output)
        }
        OutputFormat::Json => {
            let mut values = Vec::new();
            for connection in connections {
                let projects = project_ids_or_empty(&runtime_home, &connection.connection_id)?;
                values.push(connection_json(&connection, &projects));
            }
            serde_json::to_string_pretty(&json!({ "connections": values }))
                .map(|text| format!("{text}\n"))
                .map_err(|error| AgentCommandError::runtime(error.to_string()))
        }
    }
}

fn command_status(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    let parsed = parse_agent_options(args, status_allowed_options())?;
    let runtime_home = resolve_agent_runtime_home(&parsed, current_dir, process)?;
    let connection_id = required_text(parsed.connection_id.as_deref(), "connection-id")?;
    let connection = required_connection(&runtime_home, connection_id)?;
    let projects = list_connection_projects(&runtime_home, connection_id)?;
    render_connection_output(
        parsed.output,
        "status",
        status_from_store(&connection.last_verified_status),
        &connection,
        &projects,
        None,
    )
}

fn command_enable_disable(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
    enabled: bool,
) -> Result<String, AgentCommandError> {
    let parsed = parse_agent_options(args, enable_allowed_options())?;
    let runtime_home = resolve_agent_runtime_home(&parsed, current_dir, process)?;
    let connection_id = required_text(parsed.connection_id.as_deref(), "connection-id")?;
    let connection = set_connection_enabled(&runtime_home, connection_id, enabled)?;
    let projects = list_connection_projects(&runtime_home, connection_id)?;
    render_connection_output(
        parsed.output,
        if enabled { "enabled" } else { "disabled" },
        status_from_store(&connection.last_verified_status),
        &connection,
        &projects,
        None,
    )
}

fn command_project(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    let Some(subcommand) = args.first().map(String::as_str) else {
        return Err(AgentCommandError::usage(agent_usage()));
    };
    match subcommand {
        "add" => command_project_add(&args[1..], current_dir, process),
        "remove" => command_project_remove(&args[1..], current_dir, process),
        "-h" | "--help" | "help" => Ok(agent_usage()),
        other => Err(AgentCommandError::usage(format!(
            "unknown agent project command: {other}\n\n{}",
            agent_usage()
        ))),
    }
}

fn command_project_add(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    let parsed = parse_agent_options(args, project_add_allowed_options())?;
    let runtime_home = resolve_agent_runtime_home(&parsed, current_dir, process)?;
    let connection_id = required_text(parsed.connection_id.as_deref(), "connection-id")?;
    let project_id = required_text(parsed.project_id.as_deref(), "project-id")?;
    let connection = required_connection(&runtime_home, connection_id)?;
    let repo_root = resolve_optional_repo_root(parsed.repo_root.as_deref(), current_dir)?;
    if parsed.dry_run {
        return render_project_output(
            parsed.output,
            "project_add_dry_run",
            AgentResultStatus::DryRun,
            &connection,
            &[project_id.to_owned()],
        );
    }
    let project =
        resolve_or_register_project(&runtime_home, Some(project_id), repo_root.as_deref())?;
    enforce_single_project_scope(&runtime_home, &connection, &project.project_id)?;
    add_connection_project(
        &runtime_home,
        ConnectionProjectRegistration {
            connection_id: connection_id.to_owned(),
            project_id: project.project_id.clone(),
        },
    )?;
    let projects = list_connection_projects(&runtime_home, connection_id)?;
    render_connection_output(
        parsed.output,
        "project_added",
        status_from_store(&connection.last_verified_status),
        &connection,
        &projects,
        None,
    )
}

fn command_project_remove(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    let parsed = parse_agent_options(args, project_remove_allowed_options())?;
    let runtime_home = resolve_agent_runtime_home(&parsed, current_dir, process)?;
    let connection_id = required_text(parsed.connection_id.as_deref(), "connection-id")?;
    let project_id = required_text(parsed.project_id.as_deref(), "project-id")?;
    let connection = required_connection(&runtime_home, connection_id)?;
    if parsed.dry_run {
        return render_project_output(
            parsed.output,
            "project_remove_dry_run",
            AgentResultStatus::DryRun,
            &connection,
            &[project_id.to_owned()],
        );
    }
    remove_connection_project(&runtime_home, connection_id, project_id)?;
    let projects = list_connection_projects(&runtime_home, connection_id)?;
    render_connection_output(
        parsed.output,
        "project_removed",
        status_from_store(&connection.last_verified_status),
        &connection,
        &projects,
        None,
    )
}

fn command_verify(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    let parsed = parse_agent_options(args, verify_allowed_options())?;
    let runtime_home = resolve_agent_runtime_home(&parsed, current_dir, process)?;
    let connection_id = required_text(parsed.connection_id.as_deref(), "connection-id")?;
    let mut connection = required_connection(&runtime_home, connection_id)?;
    let host_plan = existing_host_plan(&connection, &runtime_home, process)?;
    let launch = mcp_launch_from_host_plan(&host_plan, None);
    let verification = verify_connection(
        &runtime_home,
        &connection,
        &host_plan,
        &launch,
        None,
        process,
    )?;
    connection = update_agent_connection_verification_report(
        &runtime_home,
        &connection.connection_id,
        verification.status.store_status(),
        &host_plan.fingerprint,
        &verification_report_json(&verification)?,
        &user_actions_json(&verification.host.user_actions)?,
    )?;
    let projects = list_connection_projects(&runtime_home, connection_id)?;
    render_connection_output(
        parsed.output,
        "verified",
        verification.status,
        &connection,
        &projects,
        Some(&verification),
    )
}

fn command_uninstall(
    args: &[String],
    current_dir: &Path,
    process: &mut impl AgentProcess,
) -> Result<String, AgentCommandError> {
    let parsed = parse_agent_options(args, uninstall_allowed_options())?;
    let runtime_home = resolve_agent_runtime_home(&parsed, current_dir, process)?;
    let connection_id = required_text(parsed.connection_id.as_deref(), "connection-id")?;
    let connection = required_connection(&runtime_home, connection_id)?;
    let host_scope = parse_host_scope(&connection.host_scope)?;
    if host_scope == HostScope::Project && !parsed.dry_run && !parsed.allow_repository_write {
        return Err(AgentCommandError::usage(
            "project-scoped Agent Connection uninstall requires --allow-repository-write",
        ));
    }
    let projects = list_connection_projects(&runtime_home, connection_id)?;
    if parsed.dry_run {
        return render_connection_output(
            parsed.output,
            "uninstall_dry_run",
            AgentResultStatus::DryRun,
            &connection,
            &projects,
            None,
        );
    }
    let host_plan = existing_host_plan(&connection, &runtime_home, process)?;
    remove_host_configuration(&host_plan, &connection, process)?;
    for project in &projects {
        remove_connection_project(&runtime_home, connection_id, &project.project_id)?;
    }
    remove_agent_connection_if_unused(&runtime_home, connection_id)?;
    render_connection_output(
        parsed.output,
        "uninstalled",
        AgentResultStatus::Complete,
        &connection,
        &[],
        None,
    )
}

fn is_help_request(args: &[String]) -> bool {
    matches!(
        args.first().map(String::as_str),
        Some("-h" | "--help" | "help")
    )
}

fn parse_agent_options(
    args: &[String],
    allowed: &[&str],
) -> Result<ParsedAgentOptions, AgentCommandError> {
    let mut parsed = ParsedAgentOptions::default();
    let mut seen = BTreeSet::new();
    let mut index = 0;

    while index < args.len() {
        let token = &args[index];
        if token == "-h" || token == "--help" || token == "help" {
            return Err(AgentCommandError::usage(agent_usage()));
        }
        if !token.starts_with("--") {
            return Err(AgentCommandError::usage(format!(
                "unexpected argument: {token}"
            )));
        }
        let without_prefix = &token[2..];
        let (name, value) = if let Some((name, value)) = without_prefix.split_once('=') {
            (name.to_owned(), Some(value.to_owned()))
        } else if is_boolean_agent_option(without_prefix) {
            (without_prefix.to_owned(), None)
        } else {
            index += 1;
            let Some(value) = args.get(index) else {
                return Err(AgentCommandError::usage(format!(
                    "missing value for --{without_prefix}"
                )));
            };
            (without_prefix.to_owned(), Some(value.clone()))
        };

        if !allowed.iter().any(|allowed_name| *allowed_name == name) {
            return Err(AgentCommandError::usage(format!(
                "unknown option: --{name}"
            )));
        }
        if !seen.insert(name.clone()) {
            return Err(AgentCommandError::usage(format!(
                "duplicate option: --{name}"
            )));
        }
        set_agent_option(&mut parsed, &name, value.as_deref())?;
        index += 1;
    }
    Ok(parsed)
}

fn parse_connection_options(
    args: &[String],
    allowed: &[&str],
    max_positionals: usize,
) -> Result<ParsedConnectionOptions, AgentCommandError> {
    let mut parsed = ParsedConnectionOptions::default();
    let mut seen = BTreeSet::new();
    let mut index = 0;

    while index < args.len() {
        let token = &args[index];
        if token == "-h" || token == "--help" || token == "help" {
            return Err(AgentCommandError::usage(connection_usage()));
        }
        if !token.starts_with("--") {
            parsed.positionals.push(token.clone());
            index += 1;
            continue;
        }
        let without_prefix = &token[2..];
        let (name, value) = if let Some((name, value)) = without_prefix.split_once('=') {
            (name.to_owned(), Some(value.to_owned()))
        } else if is_boolean_connection_option(without_prefix) {
            (without_prefix.to_owned(), None)
        } else {
            index += 1;
            let Some(value) = args.get(index) else {
                return Err(AgentCommandError::usage(format!(
                    "missing value for --{without_prefix}"
                )));
            };
            (without_prefix.to_owned(), Some(value.clone()))
        };

        if !allowed.iter().any(|allowed_name| *allowed_name == name) {
            return Err(AgentCommandError::usage(format!(
                "unknown option: --{name}"
            )));
        }
        if !seen.insert(name.clone()) {
            return Err(AgentCommandError::usage(format!(
                "duplicate option: --{name}"
            )));
        }
        set_connection_option(&mut parsed, &name, value.as_deref())?;
        index += 1;
    }

    if parsed.positionals.len() > max_positionals {
        return Err(AgentCommandError::usage(format!(
            "unexpected argument: {}",
            parsed.positionals[max_positionals]
        )));
    }
    if max_positionals == 1 {
        if let Some(host) = parsed.positionals.first() {
            parsed.host_kind = Some(parse_public_host_kind(host)?);
        }
    }
    if parsed.shared && parsed.global {
        return Err(AgentCommandError::usage(
            "--shared and --global are mutually exclusive",
        ));
    }
    Ok(parsed)
}

fn is_boolean_connection_option(name: &str) -> bool {
    matches!(name, "shared" | "global" | "read-only" | "dry-run" | "json")
}

fn set_connection_option(
    parsed: &mut ParsedConnectionOptions,
    name: &str,
    value: Option<&str>,
) -> Result<(), AgentCommandError> {
    match name {
        "repo" => parsed.repo = Some(value_path(name, value)?),
        "shared" => {
            reject_boolean_value(name, value)?;
            parsed.shared = true;
        }
        "global" => {
            reject_boolean_value(name, value)?;
            parsed.global = true;
        }
        "read-only" => {
            reject_boolean_value(name, value)?;
            parsed.read_only = true;
        }
        "dry-run" => {
            reject_boolean_value(name, value)?;
            parsed.dry_run = true;
        }
        "json" => {
            reject_boolean_value(name, value)?;
            parsed.json = true;
        }
        _ => {
            return Err(AgentCommandError::usage(format!(
                "unknown option: --{name}"
            )))
        }
    }
    Ok(())
}

fn reject_boolean_value(name: &str, value: Option<&str>) -> Result<(), AgentCommandError> {
    if value.is_some() {
        Err(AgentCommandError::usage(format!(
            "--{name} does not accept a value"
        )))
    } else {
        Ok(())
    }
}

fn connection_output_format(parsed: &ParsedConnectionOptions) -> OutputFormat {
    if parsed.json {
        OutputFormat::Json
    } else {
        OutputFormat::Text
    }
}

fn connection_intent_from_flags(
    parsed: &ParsedConnectionOptions,
) -> Result<ConnectionIntent, AgentCommandError> {
    if parsed.shared && parsed.global {
        return Err(AgentCommandError::usage(
            "--shared and --global are mutually exclusive",
        ));
    }
    if parsed.shared {
        Ok(ConnectionIntent::Shared)
    } else if parsed.global {
        Ok(ConnectionIntent::Global)
    } else {
        Ok(ConnectionIntent::Personal)
    }
}

fn host_scope_for_intent(
    host_kind: HostKind,
    intent: ConnectionIntent,
) -> Result<HostScope, AgentCommandError> {
    match (host_kind, intent) {
        (HostKind::Codex, ConnectionIntent::Personal) => Ok(HostScope::User),
        (HostKind::Codex, ConnectionIntent::Shared) => Ok(HostScope::Project),
        (HostKind::Codex, ConnectionIntent::Global) => Err(AgentCommandError::usage(
            "Codex does not support --global; use codex personal/shared or claude-code --global",
        )),
        (HostKind::ClaudeCode, ConnectionIntent::Personal) => Ok(HostScope::Local),
        (HostKind::ClaudeCode, ConnectionIntent::Shared) => Ok(HostScope::Project),
        (HostKind::ClaudeCode, ConnectionIntent::Global) => Ok(HostScope::User),
        (HostKind::Generic, _) => Err(AgentCommandError::usage(
            "generic MCP export is not a host connection; use the export command",
        )),
    }
}

fn resolve_connection_host(
    explicit: Option<HostKind>,
    process: &impl AgentProcess,
) -> Result<HostKind, AgentCommandError> {
    if let Some(host_kind) = explicit {
        return Ok(host_kind);
    }
    let mut available = Vec::new();
    if let Ok(detection) = CodexAdapter::new(codex_environment(process)).detect() {
        if detection.available {
            available.push(detection.host_kind);
        }
    }
    if let Ok(detection) = ClaudeCodeAdapter::new(ProductionCommandRunner).detect() {
        if detection.available {
            available.push(detection.host_kind);
        }
    }
    available.sort_by_key(|host| host.as_str());
    available.dedup();
    match available.as_slice() {
        [host_kind] => Ok(*host_kind),
        [] => Err(AgentCommandError::usage(
            "host could not be identified; choose `codex` or `claude-code`",
        )),
        _ => Err(AgentCommandError::usage(
            "host is ambiguous; choose `codex` or `claude-code`",
        )),
    }
}

fn connection_selector(
    parsed: &ParsedConnectionOptions,
    current_dir: &Path,
    process: &impl AgentProcess,
) -> Result<ConnectionSelector, AgentCommandError> {
    let host_kind = resolve_connection_host(parsed.host_kind, process)?;
    let intent = connection_intent_from_flags(parsed)?;
    let host_scope = host_scope_for_intent(host_kind, intent)?;
    let repo_root = resolve_connection_repo_root(current_dir, parsed.repo.as_deref())?;
    Ok(ConnectionSelector {
        host_kind,
        intent,
        host_scope,
        repo_root,
    })
}

fn mode_positionals(
    parsed: &ParsedConnectionOptions,
    process: &impl AgentProcess,
) -> Result<(HostKind, String), AgentCommandError> {
    match parsed.positionals.as_slice() {
        [mode] => {
            if let Ok(mode) = parse_user_connection_mode(mode) {
                Ok((resolve_connection_host(None, process)?, mode))
            } else {
                Err(AgentCommandError::usage(
                    "missing mode; use `workflow` or `read-only`",
                ))
            }
        }
        [host, mode] => Ok((
            parse_public_host_kind(host)?,
            parse_user_connection_mode(mode)?,
        )),
        [] => Err(AgentCommandError::usage(
            "missing mode; use `workflow` or `read-only`",
        )),
        _ => Err(AgentCommandError::usage("unexpected mode arguments")),
    }
}

fn parse_public_host_kind(value: &str) -> Result<HostKind, AgentCommandError> {
    match value {
        HOST_KIND_CODEX => Ok(HostKind::Codex),
        "claude-code" | HOST_KIND_CLAUDE_CODE => Ok(HostKind::ClaudeCode),
        other => Err(AgentCommandError::usage(format!(
            "unknown host: {other}; choose `codex` or `claude-code`"
        ))),
    }
}

fn parse_user_connection_mode(value: &str) -> Result<String, AgentCommandError> {
    match value {
        "workflow" => Ok(CONNECTION_MODE_WORKFLOW.to_owned()),
        "read-only" => Ok(CONNECTION_MODE_READ_ONLY.to_owned()),
        other => Err(AgentCommandError::usage(format!(
            "unknown connection mode: {other}; use `workflow` or `read-only`"
        ))),
    }
}

fn resolve_connection_repo_root(
    current_dir: &Path,
    selected_path: Option<&Path>,
) -> Result<PathBuf, AgentCommandError> {
    let selected = selected_path.unwrap_or(current_dir);
    let absolute = absolute_path(current_dir, selected.to_path_buf());
    let canonical = fs::canonicalize(&absolute).map_err(|error| {
        AgentCommandError::runtime(format!(
            "repository path is not accessible: {} ({error})",
            absolute.display()
        ))
    })?;
    let metadata = fs::metadata(&canonical).map_err(|error| {
        AgentCommandError::runtime(format!(
            "repository path is not accessible: {} ({error})",
            canonical.display()
        ))
    })?;
    let mut cursor = if metadata.is_file() {
        canonical
            .parent()
            .ok_or_else(|| {
                AgentCommandError::runtime(format!(
                    "repository path has no parent directory: {}",
                    canonical.display()
                ))
            })?
            .to_path_buf()
    } else {
        canonical
    };

    loop {
        let git_path = cursor.join(".git");
        match git_path.try_exists() {
            Ok(true) => return Ok(cursor),
            Ok(false) => {}
            Err(error) => {
                return Err(AgentCommandError::runtime(format!(
                    "failed to inspect Git repository marker {}: {error}",
                    git_path.display()
                )));
            }
        }
        if !cursor.pop() {
            break;
        }
    }

    Err(AgentCommandError::runtime(format!(
        "no Git repository root found from {}; run `volicord project use PATH` from inside a Git repository or pass --repo PATH",
        absolute.display()
    )))
}

fn connection_for_host_target(
    runtime_home: &Path,
    host_kind: HostKind,
    intent: ConnectionIntent,
    host_scope: HostScope,
    config_target: &str,
    server_name: &str,
) -> Result<Option<AgentConnectionRecord>, AgentCommandError> {
    let matches = list_agent_connections(runtime_home)?
        .into_iter()
        .filter(|connection| {
            connection.host_kind == host_kind.as_str()
                && connection.intent == intent.as_str()
                && connection.host_scope == host_scope.as_str()
                && connection.config_target == config_target
                && connection.server_name == server_name
        })
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Ok(None),
        [connection] => Ok(Some(connection.clone())),
        connections => Err(AgentCommandError::runtime(ambiguous_target_message(
            connections,
        ))),
    }
}

fn select_connection(
    runtime_home: &Path,
    selector: &ConnectionSelector,
) -> Result<(AgentConnectionRecord, Vec<ConnectionProjectRecord>), AgentCommandError> {
    let mut matches = Vec::new();
    for connection in list_agent_connections(runtime_home)? {
        if connection.host_kind != selector.host_kind.as_str()
            || connection.intent != selector.intent.as_str()
            || connection.host_scope != selector.host_scope.as_str()
        {
            continue;
        }
        let projects = list_connection_projects(runtime_home, &connection.connection_id)?;
        if projects
            .iter()
            .any(|project| project.project.repo_root == selector.repo_root)
        {
            matches.push((connection, projects));
        }
    }
    match matches.len() {
        0 => Err(AgentCommandError::runtime(format!(
            "no Agent Connection matches host {}, intent {}, and repository {}; run `volicord connect {}{} --repo {}`",
            public_host_label(selector.host_kind),
            selector.intent.as_str(),
            selector.repo_root.display(),
            public_host_label(selector.host_kind),
            intent_flag_suffix(selector.intent),
            selector.repo_root.display()
        ))),
        1 => Ok(matches.remove(0)),
        _ => Err(AgentCommandError::runtime(ambiguous_selector_message(
            selector, &matches,
        ))),
    }
}

fn public_host_label(host_kind: HostKind) -> &'static str {
    match host_kind {
        HostKind::Codex => "codex",
        HostKind::ClaudeCode => "claude-code",
        HostKind::Generic => "generic",
    }
}

fn intent_flag_suffix(intent: ConnectionIntent) -> &'static str {
    match intent {
        ConnectionIntent::Personal => "",
        ConnectionIntent::Shared => " --shared",
        ConnectionIntent::Global => " --global",
    }
}

fn ambiguous_target_message(connections: &[AgentConnectionRecord]) -> String {
    let mut message = String::from("host target matches multiple Agent Connections; choices:\n");
    for connection in connections {
        message.push_str(&format!(
            "- host: {}; intent: {}; target: {}; mode: {}\n",
            public_host_name_text(&connection.host_kind),
            connection.intent,
            connection.config_target,
            public_mode_text(&connection.mode)
        ));
    }
    message
}

fn ambiguous_selector_message(
    selector: &ConnectionSelector,
    matches: &[(AgentConnectionRecord, Vec<ConnectionProjectRecord>)],
) -> String {
    let mut message = format!(
        "connection selector is ambiguous for host {}, intent {}, repository {}; choices:\n",
        public_host_label(selector.host_kind),
        selector.intent.as_str(),
        selector.repo_root.display()
    );
    for (connection, projects) in matches {
        message.push_str(&format!(
            "- target: {}; mode: {}; connected_repositories: {}\n",
            connection.config_target,
            public_mode_text(&connection.mode),
            display_project_roots(projects)
        ));
    }
    message.push_str("Use a more specific repository path or remove the duplicate connection.\n");
    message
}

fn public_host_name_text(host_kind: &str) -> &str {
    match host_kind {
        HOST_KIND_CODEX => "codex",
        HOST_KIND_CLAUDE_CODE => "claude-code",
        other => other,
    }
}

fn public_mode_text(mode: &str) -> &str {
    match mode {
        CONNECTION_MODE_READ_ONLY => "read-only",
        CONNECTION_MODE_WORKFLOW => "workflow",
        other => other,
    }
}

fn connect_allowed_options() -> &'static [&'static str] {
    &[
        "host",
        "scope",
        "project-id",
        "repo-root",
        "connection-id",
        "mode",
        "server-name",
        "output",
        "dry-run",
        "allow-repository-write",
        "replace-managed",
    ]
}

fn list_allowed_options() -> &'static [&'static str] {
    &["output"]
}

fn status_allowed_options() -> &'static [&'static str] {
    &["connection-id", "output"]
}

fn enable_allowed_options() -> &'static [&'static str] {
    &["connection-id", "output"]
}

fn project_add_allowed_options() -> &'static [&'static str] {
    &[
        "connection-id",
        "project-id",
        "repo-root",
        "output",
        "dry-run",
    ]
}

fn project_remove_allowed_options() -> &'static [&'static str] {
    &["connection-id", "project-id", "output", "dry-run"]
}

fn verify_allowed_options() -> &'static [&'static str] {
    &["connection-id", "output"]
}

fn uninstall_allowed_options() -> &'static [&'static str] {
    &[
        "connection-id",
        "output",
        "dry-run",
        "allow-repository-write",
    ]
}

fn is_boolean_agent_option(name: &str) -> bool {
    matches!(
        name,
        "dry-run" | "allow-repository-write" | "replace-managed"
    )
}

fn set_agent_option(
    parsed: &mut ParsedAgentOptions,
    name: &str,
    value: Option<&str>,
) -> Result<(), AgentCommandError> {
    match name {
        "runtime-home" => parsed.runtime_home = Some(value_path(name, value)?),
        "repo-root" => parsed.repo_root = Some(value_path(name, value)?),
        "project-id" => parsed.project_id = Some(value_text(name, value)?),
        "connection-id" => parsed.connection_id = Some(value_text(name, value)?),
        "mode" => parsed.mode = Some(value_text(name, value)?),
        "host" => parsed.host_kind = Some(parse_host_kind(&value_text(name, value)?)?),
        "scope" => parsed.host_scope = Some(parse_host_scope(&value_text(name, value)?)?),
        "server-name" => parsed.server_name = Some(value_text(name, value)?),
        "output" => {
            parsed.output = match value_text(name, value)?.as_str() {
                "text" => OutputFormat::Text,
                "json" => OutputFormat::Json,
                other => {
                    return Err(AgentCommandError::usage(format!(
                        "unknown output format: {other}"
                    )))
                }
            }
        }
        "dry-run" => parsed.dry_run = true,
        "allow-repository-write" => parsed.allow_repository_write = true,
        "replace-managed" => parsed.replace_managed = true,
        _ => {
            return Err(AgentCommandError::usage(format!(
                "unknown option: --{name}"
            )))
        }
    }
    Ok(())
}

fn value_text(name: &str, value: Option<&str>) -> Result<String, AgentCommandError> {
    let value =
        value.ok_or_else(|| AgentCommandError::usage(format!("missing value for --{name}")))?;
    if value.trim().is_empty() {
        Err(AgentCommandError::usage(format!(
            "--{name} must not be empty"
        )))
    } else {
        Ok(value.to_owned())
    }
}

fn value_path(name: &str, value: Option<&str>) -> Result<PathBuf, AgentCommandError> {
    Ok(PathBuf::from(value_text(name, value)?))
}

fn required_host_kind(parsed: &ParsedAgentOptions) -> Result<HostKind, AgentCommandError> {
    parsed
        .host_kind
        .ok_or_else(|| AgentCommandError::usage("missing required option: --host"))
}

fn required_host_scope(parsed: &ParsedAgentOptions) -> Result<HostScope, AgentCommandError> {
    parsed
        .host_scope
        .ok_or_else(|| AgentCommandError::usage("missing required option: --scope"))
}

fn required_text<'a>(
    value: Option<&'a str>,
    field: &'static str,
) -> Result<&'a str, AgentCommandError> {
    value
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| AgentCommandError::usage(format!("missing required option: --{field}")))
}

fn parse_host_kind(value: &str) -> Result<HostKind, AgentCommandError> {
    match value {
        HOST_KIND_CODEX => Ok(HostKind::Codex),
        "claude-code" | HOST_KIND_CLAUDE_CODE => Ok(HostKind::ClaudeCode),
        HOST_KIND_GENERIC => Ok(HostKind::Generic),
        other => Err(AgentCommandError::usage(format!("unknown host: {other}"))),
    }
}

fn parse_host_scope(value: &str) -> Result<HostScope, AgentCommandError> {
    match value {
        HOST_SCOPE_USER => Ok(HostScope::User),
        HOST_SCOPE_PROJECT => Ok(HostScope::Project),
        HOST_SCOPE_LOCAL => Ok(HostScope::Local),
        HOST_SCOPE_EXPORT => Ok(HostScope::Export),
        other => Err(AgentCommandError::usage(format!("unknown scope: {other}"))),
    }
}

fn parse_connection_mode(value: &str) -> Result<String, AgentCommandError> {
    match value {
        CONNECTION_MODE_READ_ONLY | CONNECTION_MODE_WORKFLOW => Ok(value.to_owned()),
        other => Err(AgentCommandError::usage(format!(
            "unknown connection mode: {other}"
        ))),
    }
}

fn parse_connection_intent(value: &str) -> Result<ConnectionIntent, AgentCommandError> {
    match value {
        CONNECTION_INTENT_PERSONAL => Ok(ConnectionIntent::Personal),
        CONNECTION_INTENT_SHARED => Ok(ConnectionIntent::Shared),
        CONNECTION_INTENT_GLOBAL => Ok(ConnectionIntent::Global),
        other => Err(AgentCommandError::runtime(format!(
            "unknown connection intent in registry: {other}"
        ))),
    }
}

fn connection_intent_for_host_scope(
    host_kind: HostKind,
    scope: HostScope,
) -> Result<ConnectionIntent, AgentCommandError> {
    match (host_kind, scope) {
        (HostKind::Codex, HostScope::User) => Ok(ConnectionIntent::Personal),
        (HostKind::Codex, HostScope::Project) => Ok(ConnectionIntent::Shared),
        (HostKind::ClaudeCode, HostScope::Local) => Ok(ConnectionIntent::Personal),
        (HostKind::ClaudeCode, HostScope::Project) => Ok(ConnectionIntent::Shared),
        (HostKind::ClaudeCode, HostScope::User) => Ok(ConnectionIntent::Global),
        _ => Err(AgentCommandError::usage(
            "host and scope must match the supported Agent Connection matrix",
        )),
    }
}

fn validate_host_scope(host_kind: HostKind, scope: HostScope) -> Result<(), AgentCommandError> {
    let valid = matches!(
        (host_kind, scope),
        (HostKind::Codex, HostScope::User)
            | (HostKind::Codex, HostScope::Project)
            | (HostKind::ClaudeCode, HostScope::Local)
            | (HostKind::ClaudeCode, HostScope::Project)
            | (HostKind::ClaudeCode, HostScope::User)
    );
    if valid {
        Ok(())
    } else {
        Err(AgentCommandError::usage(
            "host and scope must match the supported Agent Connection matrix",
        ))
    }
}

fn validate_server_name(value: &str) -> Result<(), AgentCommandError> {
    if is_valid_server_name(value) {
        Ok(())
    } else {
        Err(AgentCommandError::usage(format!(
            "server name must use ASCII letters, numbers, hyphen, or underscore and start with a letter or number: {value}"
        )))
    }
}

fn validate_repository_write_permission(
    parsed: &ParsedAgentOptions,
    scope: HostScope,
) -> Result<(), AgentCommandError> {
    if scope == HostScope::Project && !parsed.dry_run && !parsed.allow_repository_write {
        return Err(AgentCommandError::usage(
            "project-scoped Agent Connection host configuration writes require --allow-repository-write",
        ));
    }
    Ok(())
}

fn resolve_agent_runtime_home(
    parsed: &ParsedAgentOptions,
    current_dir: &Path,
    process: &impl AgentProcess,
) -> Result<PathBuf, AgentCommandError> {
    if let Some(path) = &parsed.runtime_home {
        if path.is_absolute() {
            Ok(path.clone())
        } else {
            Err(AgentCommandError::usage(
                "--runtime-home must be an absolute path",
            ))
        }
    } else {
        resolve_runtime_home(|name| process.env_var(name), current_dir).map_err(Into::into)
    }
}

fn required_installation_profile(
    runtime_home: &Path,
) -> Result<InstallationProfileRecord, AgentCommandError> {
    installation_profile(runtime_home)?.ok_or_else(|| {
        AgentCommandError::runtime(format!(
            "setup has not been completed for Runtime Home {}; run `volicord setup` before connection workflows",
            runtime_home.display()
        ))
    })
}

fn installation_profile_context<'a>(
    runtime_home: &'a Path,
    profile: &'a InstallationProfileRecord,
) -> InstallationProfile<'a> {
    InstallationProfile {
        runtime_home,
        volicord_command: Path::new(&profile.volicord_command),
        volicord_mcp_command: Path::new(&profile.volicord_mcp_command),
        default_connection_mode: &profile.default_connection_mode,
    }
}

fn resolve_optional_repo_root(
    value: Option<&Path>,
    current_dir: &Path,
) -> Result<Option<PathBuf>, AgentCommandError> {
    value
        .map(|path| {
            canonical_existing_dir(&absolute_path(current_dir, path.to_path_buf()), "repo-root")
        })
        .transpose()
}

fn canonical_existing_dir(path: &Path, field: &'static str) -> Result<PathBuf, AgentCommandError> {
    let path = fs::canonicalize(path).map_err(|error| {
        AgentCommandError::runtime(format!("{field} is not accessible: {error}"))
    })?;
    if path.is_dir() {
        Ok(path)
    } else {
        Err(AgentCommandError::runtime(format!(
            "{field} must be a directory"
        )))
    }
}

fn resolve_or_register_project(
    runtime_home: &Path,
    project_id: Option<&str>,
    repo_root: Option<&Path>,
) -> Result<ProjectRecord, AgentCommandError> {
    match (project_id, repo_root) {
        (Some(project_id), Some(repo_root)) => {
            validate_project_id(project_id)?;
            if let Some(existing) = project_record(runtime_home, project_id)? {
                if existing.repo_root != repo_root {
                    return Err(AgentCommandError::runtime(
                        "--repo-root must match the existing project registration",
                    ));
                }
                Ok(existing)
            } else {
                register_project(
                    runtime_home,
                    ProjectRegistration {
                        project_id: project_id.to_owned(),
                        repo_root: repo_root.to_path_buf(),
                        project_home: None,
                        status: ACTIVE_PROJECT_STATUS.to_owned(),
                        metadata_json: metadata_json_base()?,
                    },
                )
                .map_err(Into::into)
            }
        }
        (Some(project_id), None) => project_record(runtime_home, project_id)?.ok_or_else(|| {
            AgentCommandError::runtime("project is not registered; provide --repo-root")
        }),
        (None, Some(repo_root)) => {
            let matches = list_projects(runtime_home)?
                .into_iter()
                .filter(|project| project.repo_root == repo_root)
                .collect::<Vec<_>>();
            match matches.as_slice() {
                [project] => Ok(project.clone()),
                [] => Err(AgentCommandError::usage(
                    "missing required option: --project-id",
                )),
                _ => Err(AgentCommandError::usage(
                    "--repo-root matches multiple projects; provide --project-id",
                )),
            }
        }
        (None, None) => Err(AgentCommandError::usage(
            "missing required option: --project-id",
        )),
    }
}

#[derive(Debug, Clone)]
struct DryRunProject {
    project_id: Option<String>,
    repo_root: Option<PathBuf>,
}

fn resolve_selected_project_for_dry_run(
    parsed: &ParsedAgentOptions,
    repo_root: Option<&Path>,
) -> Result<DryRunProject, AgentCommandError> {
    if parsed.project_id.is_none() && repo_root.is_none() {
        return Err(AgentCommandError::usage(
            "dry-run connect requires --project-id or --repo-root",
        ));
    }
    Ok(DryRunProject {
        project_id: parsed.project_id.clone(),
        repo_root: repo_root.map(Path::to_path_buf),
    })
}

fn enforce_single_project_scope(
    runtime_home: &Path,
    connection: &AgentConnectionRecord,
    project_id: &str,
) -> Result<(), AgentCommandError> {
    let scope = parse_host_scope(&connection.host_scope)?;
    if !matches!(scope, HostScope::Project | HostScope::Local) {
        return Ok(());
    }
    let projects = list_connection_projects(runtime_home, &connection.connection_id)?;
    if projects
        .iter()
        .any(|project| project.project_id != project_id)
    {
        return Err(AgentCommandError::runtime(
            "project and local Agent Connections may allow only one project",
        ));
    }
    Ok(())
}

fn connection_target_hint(
    host_kind: HostKind,
    scope: HostScope,
    repo_root: Option<&Path>,
    parsed: &ParsedAgentOptions,
    process: &impl AgentProcess,
    server_name: &str,
    export_target: Option<&Path>,
) -> Result<String, AgentCommandError> {
    match (host_kind, scope) {
        (HostKind::Codex, HostScope::User) => {
            let path = codex_home(process)?.join("config.toml");
            Ok(path_text(&path))
        }
        (HostKind::Codex, HostScope::Project) => {
            let repo_root = repo_root.ok_or_else(|| {
                AgentCommandError::usage("Codex project scope requires --repo-root")
            })?;
            Ok(path_text(&repo_root.join(".codex").join("config.toml")))
        }
        (HostKind::ClaudeCode, HostScope::Project) => {
            let repo_root = repo_root.ok_or_else(|| {
                AgentCommandError::usage("Claude Code project scope requires --repo-root")
            })?;
            Ok(path_text(&repo_root.join(".mcp.json")))
        }
        (HostKind::ClaudeCode, HostScope::Local) => {
            let repo_root = repo_root.ok_or_else(|| {
                AgentCommandError::usage("Claude Code local scope requires --repo-root")
            })?;
            Ok(format!("claude local {}", path_text(repo_root)))
        }
        (HostKind::ClaudeCode, HostScope::User) => Ok("claude user".to_owned()),
        (HostKind::Generic, HostScope::Export) => {
            let target = export_target
                .map(Path::to_path_buf)
                .unwrap_or_else(|| generic_default_export_target(parsed, server_name));
            Ok(path_text(&target))
        }
        _ => Err(AgentCommandError::usage(
            "host and scope must match the supported Agent Connection matrix",
        )),
    }
}

struct BuildHostPlanRequest<'a> {
    host_kind: HostKind,
    connection_intent: ConnectionIntent,
    connection_id: &'a str,
    repo_root: Option<&'a Path>,
    project_id: Option<&'a str>,
    project_name: Option<&'a str>,
    installation_profile: InstallationProfile<'a>,
    mode: &'a str,
    expected_fingerprint: Option<&'a str>,
    export_target: Option<&'a Path>,
    export_dir: Option<&'a Path>,
    current_dir: &'a Path,
}

fn build_host_plan(
    request: BuildHostPlanRequest<'_>,
    process: &impl AgentProcess,
) -> Result<HostPlan, AgentCommandError> {
    let project = request.repo_root.map(|repo_root| ProjectContext {
        project_id: request.project_id.unwrap_or(""),
        project_name: request.project_name.unwrap_or(""),
        repo_root,
    });
    let plan_request = HostPlanRequest {
        host_kind: request.host_kind,
        connection_intent: request.connection_intent,
        project,
        installation_profile: request.installation_profile,
        connection_id: request.connection_id,
        mode: request.mode,
        expected_fingerprint: request.expected_fingerprint,
    };
    match request.host_kind {
        HostKind::Codex => {
            let adapter = CodexAdapter::new(codex_environment(process));
            adapter.plan(plan_request).map_err(Into::into)
        }
        HostKind::ClaudeCode => {
            let mut adapter = ClaudeCodeAdapter::new(ProductionCommandRunner);
            adapter.plan(plan_request).map_err(Into::into)
        }
        HostKind::Generic => {
            let adapter = GenericAdapter;
            let output_dir = request.export_dir.unwrap_or(request.current_dir);
            let target_path = request
                .export_target
                .map(Path::to_path_buf)
                .unwrap_or_else(|| output_dir.join(export_file_name(request.connection_id)));
            adapter
                .plan_export(GenericExportRequest {
                    connection_id: request.connection_id,
                    installation_profile: request.installation_profile,
                    mode: request.mode,
                    target_path: &target_path,
                    expected_fingerprint: request.expected_fingerprint,
                })
                .map_err(Into::into)
        }
    }
}

fn apply_host_plan(
    host_kind: HostKind,
    plan: &HostPlan,
    process: &impl AgentProcess,
) -> Result<(), AgentCommandError> {
    match host_kind {
        HostKind::Codex => {
            let mut adapter = CodexAdapter::new(codex_environment(process));
            adapter.apply(plan)?;
        }
        HostKind::ClaudeCode => {
            let mut adapter = ClaudeCodeAdapter::new(ProductionCommandRunner);
            adapter.apply(plan)?;
        }
        HostKind::Generic => {
            let mut adapter = GenericAdapter;
            adapter.apply(plan)?;
        }
    }
    Ok(())
}

fn verify_host_plan(
    host_kind: HostKind,
    plan: &HostPlan,
    process: &impl AgentProcess,
) -> Result<Verification, AgentCommandError> {
    match host_kind {
        HostKind::Codex => {
            let mut adapter = CodexAdapter::new(codex_environment(process));
            adapter.verify(plan).map_err(Into::into)
        }
        HostKind::ClaudeCode => {
            let mut adapter = ClaudeCodeAdapter::new(ProductionCommandRunner);
            adapter.verify(plan).map_err(Into::into)
        }
        HostKind::Generic => {
            let mut adapter = GenericAdapter;
            adapter.verify(plan).map_err(Into::into)
        }
    }
}

fn remove_host_configuration(
    plan: &HostPlan,
    connection: &AgentConnectionRecord,
    process: &impl AgentProcess,
) -> Result<(), AgentCommandError> {
    let host_kind = parse_host_kind(&connection.host_kind)?;
    let request = HostRemoveRequest {
        host_kind,
        connection_intent: parse_connection_intent(&connection.intent)?,
        host_scope: parse_host_scope(&connection.host_scope)?,
        mode: connection.mode.clone(),
        server_name: connection.server_name.clone(),
        target: plan.target.clone(),
        expected_fingerprint: connection.managed_fingerprint.clone(),
    };
    match host_kind {
        HostKind::Codex => {
            let mut adapter = CodexAdapter::new(codex_environment(process));
            adapter.remove(request)?;
        }
        HostKind::ClaudeCode => {
            let mut adapter = ClaudeCodeAdapter::new(ProductionCommandRunner);
            adapter.remove(request)?;
        }
        HostKind::Generic => {
            let mut adapter = GenericAdapter;
            adapter.remove(request)?;
        }
    }
    Ok(())
}

fn existing_host_plan(
    connection: &AgentConnectionRecord,
    runtime_home: &Path,
    process: &impl AgentProcess,
) -> Result<HostPlan, AgentCommandError> {
    let host_kind = parse_host_kind(&connection.host_kind)?;
    let host_scope = parse_host_scope(&connection.host_scope)?;
    let connection_intent = parse_connection_intent(&connection.intent)?;
    let metadata = parse_metadata(&connection.metadata_json);
    let mcp_command = metadata
        .get("mcp_command")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_MCP_COMMAND));
    let runtime_home_for_entry = metadata
        .get("host_runtime_home")
        .map(PathBuf::from)
        .or_else(|| runtime_home_for_host_config(host_scope, runtime_home).map(Path::to_path_buf));
    match host_kind {
        HostKind::Codex => {
            let adapter = CodexAdapter::new(codex_environment(process));
            adapter
                .plan_existing(CodexExistingPlanRequest {
                    connection_intent,
                    scope: host_scope,
                    connection_id: &connection.connection_id,
                    server_name: &connection.server_name,
                    config_target: Path::new(&connection.config_target),
                    mcp_command: &mcp_command,
                    runtime_home: runtime_home_for_entry.as_deref(),
                    managed_fingerprint: &connection.managed_fingerprint,
                    mode: &connection.mode,
                })
                .map_err(Into::into)
        }
        _ => Ok(manual_existing_host_plan(
            connection,
            host_kind,
            connection_intent,
            host_scope,
            &mcp_command,
            runtime_home_for_entry.as_deref(),
            &metadata,
        )),
    }
}

fn manual_existing_host_plan(
    connection: &AgentConnectionRecord,
    host_kind: HostKind,
    connection_intent: ConnectionIntent,
    host_scope: HostScope,
    mcp_command: &Path,
    runtime_home: Option<&Path>,
    metadata: &BTreeMap<String, String>,
) -> HostPlan {
    let target = match metadata.get("target_kind").map(String::as_str) {
        Some("file") => HostTarget::File(PathBuf::from(
            metadata
                .get("target_path")
                .cloned()
                .unwrap_or_else(|| connection.config_target.clone()),
        )),
        Some("export") => HostTarget::Export(PathBuf::from(
            metadata
                .get("target_path")
                .cloned()
                .unwrap_or_else(|| connection.config_target.clone()),
        )),
        Some("external_cli") => HostTarget::ExternalCli {
            program: metadata
                .get("external_program")
                .cloned()
                .unwrap_or_else(|| "claude".to_owned()),
            cwd: metadata.get("external_cwd").map(PathBuf::from),
        },
        _ if host_kind == HostKind::Generic => {
            HostTarget::Export(PathBuf::from(&connection.config_target))
        }
        _ => HostTarget::File(PathBuf::from(&connection.config_target)),
    };
    HostPlan {
        host_kind,
        connection_intent,
        host_scope,
        mode: connection.mode.clone(),
        server_name: connection.server_name.clone(),
        target,
        entry: ManagedServerEntry::new(&connection.connection_id, mcp_command, runtime_home),
        change: PlannedChange::Noop,
        fingerprint: connection.managed_fingerprint.clone(),
        conflicts: Vec::new(),
        user_actions: stored_or_default_user_actions(connection, host_kind, host_scope),
        file_snapshot: None,
    }
}

fn stored_or_default_user_actions(
    connection: &AgentConnectionRecord,
    host_kind: HostKind,
    host_scope: HostScope,
) -> Vec<UserAction> {
    let parsed = serde_json::from_str::<Vec<UserAction>>(&connection.last_user_actions_json)
        .unwrap_or_default();
    if !parsed.is_empty() {
        return parsed;
    }
    match (host_kind, host_scope) {
        (HostKind::ClaudeCode, HostScope::Project) => vec![UserAction::new(
            UserActionKind::ProjectApprovalRequired,
            "Claude Code requires user approval before project-scoped .mcp.json servers load",
        )],
        (HostKind::Generic, HostScope::Export) => vec![UserAction::new(
            UserActionKind::HostTrustRequired,
            "generic export must be loaded, trusted, or approved in the target host by the user",
        )],
        _ => Vec::new(),
    }
}

fn verify_connection(
    runtime_home: &Path,
    connection: &AgentConnectionRecord,
    host_plan: &HostPlan,
    launch: &McpLaunch,
    project_id: Option<&str>,
    process: &mut impl AgentProcess,
) -> Result<VerificationReport, AgentCommandError> {
    let host_kind = parse_host_kind(&connection.host_kind)?;
    let host = verify_host_plan(host_kind, host_plan, process)?;
    let preflight = run_connection_preflight(
        process,
        launch,
        runtime_home,
        &connection.connection_id,
        project_id,
        &connection.mode,
    );
    let handshake = if host.mcp_handshake_allowed && preflight.status == StepStatus::Passed {
        match process.verify_mcp_stdio(
            launch,
            runtime_home,
            &connection.connection_id,
            &connection.mode,
        ) {
            Ok(verification) => verification,
            Err(error) => McpVerification::failed(error),
        }
    } else if !host.mcp_handshake_allowed {
        McpVerification {
            step: VerificationStep::skipped("host state does not allow direct MCP handshake"),
            tools: Vec::new(),
        }
    } else {
        McpVerification {
            step: VerificationStep::skipped("MCP preflight failed"),
            tools: Vec::new(),
        }
    };
    let status = aggregate_status(&host, &preflight, &handshake.step);
    Ok(VerificationReport {
        status,
        host,
        preflight,
        handshake: handshake.step,
        tools: handshake.tools,
    })
}

fn aggregate_status(
    host: &Verification,
    preflight: &VerificationStep,
    handshake: &VerificationStep,
) -> AgentResultStatus {
    if preflight.status == StepStatus::Failed || handshake.status == StepStatus::Failed {
        return AgentResultStatus::Failed;
    }
    match host.status {
        VerificationStatus::Complete if handshake.status == StepStatus::Passed => {
            AgentResultStatus::Complete
        }
        VerificationStatus::ActionRequired if handshake.status == StepStatus::Passed => {
            AgentResultStatus::ActionRequired
        }
        VerificationStatus::NotVerified => AgentResultStatus::NotVerified,
        _ => AgentResultStatus::Failed,
    }
}

fn run_connection_preflight(
    process: &mut impl AgentProcess,
    launch: &McpLaunch,
    runtime_home: &Path,
    connection_id: &str,
    project_id: Option<&str>,
    mode: &str,
) -> VerificationStep {
    match process.run_preflight(launch, runtime_home, connection_id, project_id) {
        Ok(output) if output.success => {
            match validate_connection_preflight_report(&output.stdout, connection_id, mode) {
                Ok(()) => VerificationStep::passed("volicord-mcp preflight passed"),
                Err(message) => VerificationStep::failed(message),
            }
        }
        Ok(output) => VerificationStep::failed(format!(
            "volicord-mcp preflight failed with status {}; stderr: {}",
            status_text(output.status_code),
            compact_stream(&output.stderr)
        )),
        Err(message) => VerificationStep::failed(message),
    }
}

fn validate_connection_preflight_report(
    stdout: &str,
    connection_id: &str,
    mode: &str,
) -> Result<(), String> {
    let report = parse_colon_report(stdout)?;
    expect_report_field(&report, "configuration", "valid")?;
    expect_report_field(&report, "transport", "stdio")?;
    expect_report_field(&report, "connection_id", connection_id)?;
    expect_report_field(&report, "mode", mode)?;
    expect_report_field(&report, "enabled", "true")?;
    Ok(())
}

fn parse_colon_report(stdout: &str) -> Result<BTreeMap<String, String>, String> {
    let mut report = BTreeMap::new();
    for line in stdout.lines() {
        if let Some((key, value)) = line.split_once(':') {
            report.insert(key.trim().to_owned(), value.trim().to_owned());
        }
    }
    if report.is_empty() {
        Err("preflight did not return a key-value report".to_owned())
    } else {
        Ok(report)
    }
}

fn expect_report_field(
    report: &BTreeMap<String, String>,
    key: &str,
    expected: &str,
) -> Result<(), String> {
    match report.get(key) {
        Some(actual) if actual == expected => Ok(()),
        Some(actual) => Err(format!(
            "preflight field {key} was {actual}, expected {expected}"
        )),
        None => Err(format!("preflight field {key} was missing")),
    }
}

fn mcp_launch_from_host_plan(plan: &HostPlan, repo_root: Option<&Path>) -> McpLaunch {
    let cwd = match plan.host_scope {
        HostScope::Project | HostScope::Local => repo_root.map(Path::to_path_buf),
        HostScope::User | HostScope::Export => None,
    };
    McpLaunch {
        command: PathBuf::from(&plan.entry.command),
        args: plan.entry.args.clone(),
        env: plan.entry.env.clone(),
        cwd,
    }
}

fn apply_mcp_launch_context(command: &mut Command, launch: &McpLaunch, runtime_home: &Path) {
    command.env(VOLICORD_HOME, runtime_home);
    for (key, value) in &launch.env {
        command.env(key, value);
    }
    if let Some(cwd) = &launch.cwd {
        command.current_dir(cwd);
    }
}

fn verify_mcp_stdio_process(
    launch: &McpLaunch,
    runtime_home: &Path,
    connection_id: &str,
    mode: &str,
    timeout: Duration,
) -> Result<McpVerification, String> {
    let mut child = Command::new(&launch.command);
    child.args(&launch.args);
    apply_mcp_launch_context(&mut child, launch, runtime_home);
    child
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = child.spawn().map_err(|error| {
        format!(
            "failed to launch {} for MCP handshake with connection {}: {error}",
            launch.command.display(),
            connection_id
        )
    })?;
    let deadline = Instant::now() + timeout;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "failed to capture MCP stdout".to_owned())?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| "failed to open MCP stdin".to_owned())?;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    let _ = tx.send(Ok(None));
                    break;
                }
                Ok(_) => {
                    let _ = tx.send(Ok(Some(line)));
                }
                Err(error) => {
                    let _ = tx.send(Err(error.to_string()));
                    break;
                }
            }
        }
    });

    write_json_line(
        &mut stdin,
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-11-25",
                "capabilities": {},
                "clientInfo": {"name": "volicord-cli", "version": env!("CARGO_PKG_VERSION")}
            }
        }),
    )?;
    let initialize = read_json_response(&rx, deadline)?;
    validate_initialize_response(&initialize)?;
    write_json_line(
        &mut stdin,
        json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized",
            "params": {}
        }),
    )?;
    write_json_line(
        &mut stdin,
        json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        }),
    )?;
    let tools = validate_tools_response(&read_json_response(&rx, deadline)?)?;
    validate_tools_for_mode(mode, &tools)?;
    drop(stdin);
    terminate_child(&mut child, deadline)?;
    Ok(McpVerification::passed(tools))
}

fn write_json_line(writer: &mut impl Write, value: Value) -> Result<(), String> {
    serde_json::to_writer(&mut *writer, &value).map_err(|error| error.to_string())?;
    writer.write_all(b"\n").map_err(|error| error.to_string())?;
    writer.flush().map_err(|error| error.to_string())
}

fn read_json_response(
    rx: &mpsc::Receiver<Result<Option<String>, String>>,
    deadline: Instant,
) -> Result<Value, String> {
    let now = Instant::now();
    if now >= deadline {
        return Err("MCP handshake timed out".to_owned());
    }
    match rx.recv_timeout(deadline.saturating_duration_since(now)) {
        Ok(Ok(Some(line))) => serde_json::from_str::<Value>(&line)
            .map_err(|error| format!("invalid MCP JSON response: {error}; line: {line}")),
        Ok(Ok(None)) => Err("MCP process exited before response".to_owned()),
        Ok(Err(error)) => Err(format!("failed reading MCP response: {error}")),
        Err(mpsc::RecvTimeoutError::Timeout) => Err("MCP handshake timed out".to_owned()),
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            Err("MCP response reader disconnected".to_owned())
        }
    }
}

fn validate_initialize_response(value: &Value) -> Result<(), String> {
    if value.get("error").is_some() {
        return Err(format!("MCP initialize returned error: {value}"));
    }
    let result = value
        .get("result")
        .ok_or_else(|| "MCP initialize response missing result".to_owned())?;
    if result
        .get("instructions")
        .and_then(Value::as_str)
        .is_none_or(str::is_empty)
    {
        return Err("MCP initialize response missing instructions".to_owned());
    }
    Ok(())
}

fn validate_tools_response(value: &Value) -> Result<Vec<String>, String> {
    if value.get("error").is_some() {
        return Err(format!("MCP tools/list returned error: {value}"));
    }
    let tools = value
        .get("result")
        .and_then(|result| result.get("tools"))
        .and_then(Value::as_array)
        .ok_or_else(|| "MCP tools/list response missing result.tools".to_owned())?;
    let mut names = Vec::new();
    for tool in tools {
        let name = tool
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| "MCP tool entry missing name".to_owned())?;
        names.push(name.to_owned());
    }
    Ok(names)
}

fn validate_tools_for_mode(mode: &str, tools: &[String]) -> Result<(), String> {
    let expected = match mode {
        CONNECTION_MODE_READ_ONLY => READ_ONLY_TOOL_NAMES.as_slice(),
        CONNECTION_MODE_WORKFLOW => WORKFLOW_TOOL_NAMES.as_slice(),
        other => {
            return Err(format!(
                "unsupported connection mode for tool validation: {other}"
            ))
        }
    };
    for name in expected {
        if !tools.iter().any(|tool| tool == name) {
            return Err(format!("MCP tools/list missing required tool: {name}"));
        }
    }
    Ok(())
}

struct SimplifiedConnectionOutput<'a> {
    format: OutputFormat,
    action: &'a str,
    status: AgentResultStatus,
    connection: &'a AgentConnectionRecord,
    projects: &'a [ConnectionProjectRecord],
    verification: Option<&'a VerificationReport>,
    plan: Option<&'a HostPlan>,
    user_actions: Vec<UserAction>,
}

struct SimplifiedPlanOutput<'a> {
    format: OutputFormat,
    action: &'a str,
    status: AgentResultStatus,
    connection_id: &'a str,
    host_kind: HostKind,
    intent: ConnectionIntent,
    host_scope: HostScope,
    mode: &'a str,
    enabled: bool,
    repo_root: Option<&'a Path>,
    plan: &'a HostPlan,
    projects_remaining: Option<usize>,
    user_actions: Vec<UserAction>,
}

enum SimplifiedRemovePlan<'a> {
    Host(&'a HostPlan),
    MembershipOnly,
}

fn render_simplified_connection_output(
    data: SimplifiedConnectionOutput<'_>,
) -> Result<String, AgentCommandError> {
    let project_ids = data
        .projects
        .iter()
        .map(|project| project.project_id.clone())
        .collect::<Vec<_>>();
    let target = data
        .plan
        .map(|plan| host_target_text(&plan.target))
        .unwrap_or_else(|| data.connection.config_target.clone());
    let planned_change = data.plan.map(|plan| planned_change_text(plan.change));
    match data.format {
        OutputFormat::Text => {
            let mut output = format!(
                "Agent Connection {}\nhost: {}\nintent: {}\nmode: {}\nenabled: {}\nconnected_repositories: {}\nverification_status: {}\ntarget: {}\n",
                data.action,
                public_host_name_text(&data.connection.host_kind),
                data.connection.intent,
                public_mode_text(&data.connection.mode),
                data.connection.enabled,
                display_project_roots(data.projects),
                data.status.as_str(),
                target
            );
            if let Some(planned_change) = planned_change {
                output.push_str(&format!("planned_change: {planned_change}\n"));
            }
            if let Some(verification) = data.verification {
                output.push_str(&format!(
                    "host_verification: {}\npreflight: {}\nmcp_handshake: {}\n",
                    verification.host.status.as_str(),
                    verification.preflight.status.as_str(),
                    verification.handshake.status.as_str()
                ));
            }
            append_user_actions_text(&mut output, &data.user_actions);
            Ok(output)
        }
        OutputFormat::Json => {
            let value = json!({
                "action": data.action,
                "status": data.status.as_str(),
                "connection": connection_json(data.connection, &project_ids),
                "target": target,
                "planned_change": planned_change,
                "checks": checks_json(data.connection, data.verification),
                "actions": actions_json_values(&data.user_actions),
                "verification": data.verification.map(verification_json),
            });
            serde_json::to_string_pretty(&value)
                .map(|text| format!("{text}\n"))
                .map_err(|error| AgentCommandError::runtime(error.to_string()))
        }
    }
}

fn render_simplified_plan_output(
    data: SimplifiedPlanOutput<'_>,
) -> Result<String, AgentCommandError> {
    let target = host_target_text(&data.plan.target);
    let planned_change = planned_change_text(data.plan.change);
    match data.format {
        OutputFormat::Text => {
            let mut output = format!(
                "Agent Connection {} {}\nhost: {}\nintent: {}\nmode: {}\nenabled: {}\nconnected_repositories: {}\nverification_status: {}\ntarget: {}\nplanned_change: {}\n",
                data.action,
                data.status.as_str(),
                public_host_label(data.host_kind),
                data.intent.as_str(),
                public_mode_text(data.mode),
                data.enabled,
                data.repo_root
                    .map(|path| path.display().to_string())
                    .unwrap_or_default(),
                data.status.as_str(),
                target,
                planned_change
            );
            if let Some(remaining) = data.projects_remaining {
                output.push_str(&format!("remaining_connected_projects: {remaining}\n"));
            }
            append_user_actions_text(&mut output, &data.user_actions);
            Ok(output)
        }
        OutputFormat::Json => {
            let connected_repositories = data
                .repo_root
                .into_iter()
                .map(path_text)
                .collect::<Vec<_>>();
            let value = json!({
                "action": data.action,
                "status": data.status.as_str(),
                "connection": {
                    "connection_id": data.connection_id,
                    "host_kind": data.host_kind.as_str(),
                    "connection_intent": data.intent.as_str(),
                    "host_scope": data.host_scope.as_str(),
                    "mode": data.mode,
                    "enabled": data.enabled,
                    "connected_repositories": connected_repositories,
                    "verification_status": data.status.as_str(),
                    "server_name": data.plan.server_name,
                    "config_target": target,
                },
                "target": target,
                "planned_change": planned_change,
                "remaining_connected_projects": data.projects_remaining,
                "checks": [{
                    "id": "host_plan",
                    "status": "passed",
                    "summary": "host plan was built"
                }],
                "actions": actions_json_values(&data.user_actions),
            });
            serde_json::to_string_pretty(&value)
                .map(|text| format!("{text}\n"))
                .map_err(|error| AgentCommandError::runtime(error.to_string()))
        }
    }
}

fn render_simplified_connections_output(
    format: OutputFormat,
    rows: &[(AgentConnectionRecord, Vec<ConnectionProjectRecord>)],
) -> Result<String, AgentCommandError> {
    match format {
        OutputFormat::Text => {
            let mut output = String::from(
                "host\tintent\tmode\tenabled\tconnected_repositories\tverification_status\ttarget\n",
            );
            for (connection, projects) in rows {
                output.push_str(&format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                    public_host_name_text(&connection.host_kind),
                    connection.intent,
                    public_mode_text(&connection.mode),
                    connection.enabled,
                    display_project_roots(projects),
                    connection.last_verification_status,
                    connection.config_target
                ));
            }
            Ok(output)
        }
        OutputFormat::Json => {
            let values = rows
                .iter()
                .map(|(connection, projects)| {
                    let project_ids = projects
                        .iter()
                        .map(|project| project.project_id.clone())
                        .collect::<Vec<_>>();
                    let mut value = connection_json(connection, &project_ids);
                    if let Some(object) = value.as_object_mut() {
                        object.insert(
                            "connected_repositories".to_owned(),
                            Value::Array(
                                projects
                                    .iter()
                                    .map(|project| {
                                        Value::String(path_text(&project.project.repo_root))
                                    })
                                    .collect(),
                            ),
                        );
                    }
                    value
                })
                .collect::<Vec<_>>();
            serde_json::to_string_pretty(&json!({
                "status": "complete",
                "connections": values,
                "checks": [],
                "actions": [],
            }))
            .map(|text| format!("{text}\n"))
            .map_err(|error| AgentCommandError::runtime(error.to_string()))
        }
    }
}

fn render_simplified_remove_dry_run(
    format: OutputFormat,
    connection: &AgentConnectionRecord,
    projects: &[ConnectionProjectRecord],
    selected_project: &ConnectionProjectRecord,
    plan: SimplifiedRemovePlan<'_>,
    remaining_count: usize,
) -> Result<String, AgentCommandError> {
    match plan {
        SimplifiedRemovePlan::Host(host_plan) => {
            render_simplified_plan_output(SimplifiedPlanOutput {
                format,
                action: "remove",
                status: AgentResultStatus::DryRun,
                connection_id: &connection.connection_id,
                host_kind: parse_host_kind(&connection.host_kind)?,
                intent: parse_connection_intent(&connection.intent)?,
                host_scope: parse_host_scope(&connection.host_scope)?,
                mode: &connection.mode,
                enabled: connection.enabled,
                repo_root: Some(&selected_project.project.repo_root),
                plan: host_plan,
                projects_remaining: Some(remaining_count),
                user_actions: Vec::new(),
            })
        }
        SimplifiedRemovePlan::MembershipOnly => match format {
            OutputFormat::Text => Ok(format!(
                "Agent Connection remove dry_run\nhost: {}\nintent: {}\nmode: {}\nconnected_repositories: {}\nverification_status: dry_run\ntarget: {}\nplanned_change: membership\nremaining_connected_projects: {}\n",
                public_host_name_text(&connection.host_kind),
                connection.intent,
                public_mode_text(&connection.mode),
                display_project_roots(projects),
                connection.config_target,
                remaining_count
            )),
            OutputFormat::Json => {
                let project_ids = projects
                    .iter()
                    .map(|project| project.project_id.clone())
                    .collect::<Vec<_>>();
                serde_json::to_string_pretty(&json!({
                    "action": "remove",
                    "status": AgentResultStatus::DryRun.as_str(),
                    "connection": connection_json(connection, &project_ids),
                    "target": connection.config_target,
                    "planned_change": "membership",
                    "remaining_connected_projects": remaining_count,
                    "checks": [{
                        "id": "connection_membership",
                        "status": "passed",
                        "summary": "selected repository membership can be removed"
                    }],
                    "actions": [],
                }))
                .map(|text| format!("{text}\n"))
                .map_err(|error| AgentCommandError::runtime(error.to_string()))
            }
        },
    }
}

fn planned_change_text(change: PlannedChange) -> &'static str {
    match change {
        PlannedChange::Create => "create",
        PlannedChange::Update => "update",
        PlannedChange::Remove => "remove",
        PlannedChange::Noop => "noop",
        PlannedChange::ExternalCommand => "external_command",
    }
}

fn display_project_roots(projects: &[ConnectionProjectRecord]) -> String {
    projects
        .iter()
        .map(|project| path_text(&project.project.repo_root))
        .collect::<Vec<_>>()
        .join(",")
}

fn append_user_actions_text(output: &mut String, actions: &[UserAction]) {
    if actions.is_empty() {
        return;
    }
    output.push_str("actions:\n");
    for action in actions {
        output.push_str(&format!(
            "- {}: {}\n",
            user_action_id(action.kind),
            action.message
        ));
    }
}

fn actions_json_values(actions: &[UserAction]) -> Value {
    Value::Array(
        actions
            .iter()
            .map(|action| {
                json!({
                    "id": user_action_id(action.kind),
                    "instruction": action.message,
                })
            })
            .collect(),
    )
}

fn user_action_id(kind: UserActionKind) -> &'static str {
    match kind {
        UserActionKind::HostTrustRequired => "host_trust_required",
        UserActionKind::ProjectApprovalRequired => "project_approval_required",
        UserActionKind::ReloadRequired => "reload_required",
    }
}

fn checks_json(
    connection: &AgentConnectionRecord,
    verification: Option<&VerificationReport>,
) -> Value {
    if let Some(verification) = verification {
        return Value::Array(vec![
            json!({
                "id": "host",
                "status": verification.host.status.as_str(),
                "summary": verification.host.details,
                "details": {
                    "host_state": verification.host.host_state.as_str(),
                    "managed_config": verification.host.managed_config.as_str(),
                    "host_executable": verification.host.host_executable.as_str(),
                    "host_gate": verification.host.host_gate.as_str(),
                    "host_configuration": verification.host.host_configuration.as_str(),
                }
            }),
            json!({
                "id": "mcp_preflight",
                "status": verification.preflight.status.as_str(),
                "summary": verification.preflight.details,
            }),
            json!({
                "id": "mcp_handshake",
                "status": verification.handshake.status.as_str(),
                "summary": verification.handshake.details,
            }),
        ]);
    }
    stored_checks_json(connection)
}

fn stored_checks_json(connection: &AgentConnectionRecord) -> Value {
    let report = json_object_text(&connection.last_verification_report_json);
    let Some(object) = report.as_object() else {
        return json!([]);
    };
    let mut checks = Vec::new();
    if let Some(host) = object.get("host").and_then(Value::as_object) {
        checks.push(json!({
            "id": "host",
            "status": host.get("status").and_then(Value::as_str).unwrap_or("not_verified"),
            "summary": host
                .get("details")
                .and_then(Value::as_str)
                .unwrap_or("stored host verification state"),
            "details": host,
        }));
    }
    if let Some(preflight) = object.get("preflight").and_then(Value::as_object) {
        checks.push(json!({
            "id": "mcp_preflight",
            "status": preflight.get("status").and_then(Value::as_str).unwrap_or("skipped"),
            "summary": preflight
                .get("details")
                .and_then(Value::as_str)
                .unwrap_or("stored MCP preflight state"),
        }));
    }
    if let Some(handshake) = object.get("mcp_handshake").and_then(Value::as_object) {
        checks.push(json!({
            "id": "mcp_handshake",
            "status": handshake.get("status").and_then(Value::as_str).unwrap_or("skipped"),
            "summary": handshake
                .get("details")
                .and_then(Value::as_str)
                .unwrap_or("stored MCP handshake state"),
        }));
    }
    Value::Array(checks)
}

fn stored_user_actions(connection: &AgentConnectionRecord) -> Vec<UserAction> {
    serde_json::from_str::<Vec<UserAction>>(&connection.last_user_actions_json).unwrap_or_default()
}

fn terminate_child(child: &mut Child, deadline: Instant) -> Result<(), String> {
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return Ok(()),
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                return Ok(());
            }
            Err(error) => return Err(format!("failed to wait for MCP process: {error}")),
        }
    }
}

fn render_connection_output(
    format: OutputFormat,
    action: &str,
    status: AgentResultStatus,
    connection: &AgentConnectionRecord,
    projects: &[ConnectionProjectRecord],
    verification: Option<&VerificationReport>,
) -> Result<String, AgentCommandError> {
    let project_ids = projects
        .iter()
        .map(|project| project.project_id.clone())
        .collect::<Vec<_>>();
    match format {
        OutputFormat::Text => {
            let mut output = format!(
                "Agent Connection {action}\nconnection_id: {}\nhost_kind: {}\nhost_scope: {}\nmode: {}\nenabled: {}\nconnected_projects: {}\nverification_status: {}\nserver_name: {}\nconfig_target: {}\n",
                connection.connection_id,
                connection.host_kind,
                connection.host_scope,
                connection.mode,
                connection.enabled,
                display_projects(&project_ids),
                status.as_str(),
                connection.server_name,
                connection.config_target
            );
            if let Some(verification) = verification {
                output.push_str(&format!(
                    "host_verification: {}\npreflight: {}\nmcp_handshake: {}\n",
                    verification.host.status.as_str(),
                    verification.preflight.status.as_str(),
                    verification.handshake.status.as_str()
                ));
            }
            Ok(output)
        }
        OutputFormat::Json => {
            let value = json!({
                "action": action,
                "status": status.as_str(),
                "connection": connection_json(connection, &project_ids),
                "verification": verification.map(verification_json)
            });
            serde_json::to_string_pretty(&value)
                .map(|text| format!("{text}\n"))
                .map_err(|error| AgentCommandError::runtime(error.to_string()))
        }
    }
}

fn render_project_output(
    format: OutputFormat,
    action: &str,
    status: AgentResultStatus,
    connection: &AgentConnectionRecord,
    project_ids: &[String],
) -> Result<String, AgentCommandError> {
    match format {
        OutputFormat::Text => Ok(format!(
            "Agent Connection {action}\nconnection_id: {}\nconnected_projects: {}\nverification_status: {}\n",
            connection.connection_id,
            display_projects(project_ids),
            status.as_str()
        )),
        OutputFormat::Json => {
            let value = json!({
                "action": action,
                "status": status.as_str(),
                "connection_id": connection.connection_id,
                "connected_projects": project_ids,
            });
            serde_json::to_string_pretty(&value)
                .map(|text| format!("{text}\n"))
                .map_err(|error| AgentCommandError::runtime(error.to_string()))
        }
    }
}

struct DryRunRenderData<'a> {
    action: &'a str,
    connection_id: &'a str,
    host_kind: HostKind,
    host_scope: HostScope,
    mode: &'a str,
    server_name: &'a str,
    config_target: &'a str,
    project_id: Option<&'a str>,
}

fn render_dry_run_output(
    format: OutputFormat,
    data: DryRunRenderData<'_>,
) -> Result<String, AgentCommandError> {
    match format {
        OutputFormat::Text => Ok(format!(
            "Agent Connection {} dry_run\nconnection_id: {}\nhost_kind: {}\nhost_scope: {}\nmode: {}\nenabled: true\nconnected_projects: {}\nverification_status: dry_run\nserver_name: {}\nconfig_target: {}\n",
            data.action,
            data.connection_id,
            data.host_kind.as_str(),
            data.host_scope.as_str(),
            data.mode,
            data.project_id.unwrap_or(""),
            data.server_name,
            data.config_target
        )),
        OutputFormat::Json => {
            let value = json!({
                "action": data.action,
                "status": AgentResultStatus::DryRun.as_str(),
                "connection": {
                    "connection_id": data.connection_id,
                    "host_kind": data.host_kind.as_str(),
                    "host_scope": data.host_scope.as_str(),
                    "mode": data.mode,
                    "enabled": true,
                    "connected_projects": data.project_id.into_iter().collect::<Vec<_>>(),
                    "verification_status": AgentResultStatus::DryRun.as_str(),
                    "server_name": data.server_name,
                    "config_target": data.config_target
                }
            });
            serde_json::to_string_pretty(&value)
                .map(|text| format!("{text}\n"))
                .map_err(|error| AgentCommandError::runtime(error.to_string()))
        }
    }
}

fn connection_json(connection: &AgentConnectionRecord, project_ids: &[String]) -> Value {
    json!({
        "connection_id": connection.connection_id,
        "host_kind": connection.host_kind,
        "connection_intent": connection.intent,
        "host_scope": connection.host_scope,
        "mode": connection.mode,
        "enabled": connection.enabled,
        "connected_projects": project_ids,
        "verification_status": connection.last_verified_status,
        "verification_report": json_object_text(&connection.last_verification_report_json),
        "user_actions": json_array_text(&connection.last_user_actions_json),
        "server_name": connection.server_name,
        "config_target": connection.config_target,
    })
}

fn json_object_text(text: &str) -> Value {
    serde_json::from_str::<Value>(text)
        .ok()
        .filter(Value::is_object)
        .unwrap_or_else(|| json!({}))
}

fn json_array_text(text: &str) -> Value {
    serde_json::from_str::<Value>(text)
        .ok()
        .filter(Value::is_array)
        .unwrap_or_else(|| json!([]))
}

fn verification_json(report: &VerificationReport) -> Value {
    json!({
        "status": report.status.as_str(),
        "host": {
            "status": report.host.status.as_str(),
            "host_state": report.host.host_state.as_str(),
            "managed_config": report.host.managed_config.as_str(),
            "host_executable": report.host.host_executable.as_str(),
            "host_gate": report.host.host_gate.as_str(),
            "host_configuration": report.host.host_configuration.as_str(),
            "mcp_handshake_allowed": report.host.mcp_handshake_allowed,
            "details": report.host.details,
            "diagnostic": report.host.diagnostic,
            "user_actions": report.host.user_actions,
        },
        "preflight": step_json(&report.preflight),
        "mcp_handshake": step_json(&report.handshake),
        "tools": report.tools,
    })
}

fn verification_report_json(report: &VerificationReport) -> Result<String, AgentCommandError> {
    serde_json::to_string(&verification_json(report))
        .map_err(|error| AgentCommandError::runtime(error.to_string()))
}

fn user_actions_json(
    actions: &[crate::host_integration::UserAction],
) -> Result<String, AgentCommandError> {
    serde_json::to_string(actions).map_err(|error| AgentCommandError::runtime(error.to_string()))
}

fn step_json(step: &VerificationStep) -> Value {
    json!({
        "status": step.status.as_str(),
        "details": step.details,
    })
}

fn display_projects(projects: &[String]) -> String {
    if projects.is_empty() {
        String::new()
    } else {
        projects.join(",")
    }
}

fn project_ids_or_empty(
    runtime_home: &Path,
    connection_id: &str,
) -> Result<Vec<String>, AgentCommandError> {
    Ok(list_connection_projects(runtime_home, connection_id)?
        .into_iter()
        .map(|project| project.project_id)
        .collect())
}

fn status_from_store(value: &str) -> AgentResultStatus {
    match value {
        VERIFIED_STATUS_COMPLETE => AgentResultStatus::Complete,
        VERIFIED_STATUS_ACTION_REQUIRED => AgentResultStatus::ActionRequired,
        VERIFIED_STATUS_FAILED => AgentResultStatus::Failed,
        _ => AgentResultStatus::NotVerified,
    }
}

fn required_connection(
    runtime_home: &Path,
    connection_id: &str,
) -> Result<AgentConnectionRecord, AgentCommandError> {
    agent_connection_record(runtime_home, connection_id)?.ok_or_else(|| {
        AgentCommandError::runtime(format!("Agent Connection not found: {connection_id}"))
    })
}

fn connection_metadata_json(
    plan: &HostPlan,
    mcp_command: &Path,
    runtime_home: &Path,
) -> Result<String, AgentCommandError> {
    let mut value = json!({
        "created_by": AGENT_METADATA_CREATED_BY,
        "mcp_command": path_text(mcp_command),
        "connection_intent": plan.connection_intent.as_str(),
        "mode": plan.mode.as_str(),
    });
    let object = value
        .as_object_mut()
        .expect("metadata should be object immediately after construction");
    if let Some(host_runtime_home) = runtime_home_for_host_config(plan.host_scope, runtime_home) {
        object.insert(
            "host_runtime_home".to_owned(),
            Value::String(path_text(host_runtime_home)),
        );
    }
    match &plan.target {
        HostTarget::File(path) => {
            object.insert("target_kind".to_owned(), Value::String("file".to_owned()));
            object.insert("target_path".to_owned(), Value::String(path_text(path)));
        }
        HostTarget::Export(path) => {
            object.insert("target_kind".to_owned(), Value::String("export".to_owned()));
            object.insert("target_path".to_owned(), Value::String(path_text(path)));
        }
        HostTarget::ExternalCli { program, cwd } => {
            object.insert(
                "target_kind".to_owned(),
                Value::String("external_cli".to_owned()),
            );
            object.insert(
                "external_program".to_owned(),
                Value::String(program.clone()),
            );
            if let Some(cwd) = cwd {
                object.insert("external_cwd".to_owned(), Value::String(path_text(cwd)));
            }
        }
    }
    serde_json::to_string(&value).map_err(|error| AgentCommandError::runtime(error.to_string()))
}

fn metadata_json_base() -> Result<String, AgentCommandError> {
    serde_json::to_string(&json!({ "created_by": AGENT_METADATA_CREATED_BY }))
        .map_err(|error| AgentCommandError::runtime(error.to_string()))
}

fn parse_metadata(text: &str) -> BTreeMap<String, String> {
    serde_json::from_str::<Value>(text)
        .ok()
        .and_then(|value| {
            value.as_object().map(|object| {
                object
                    .iter()
                    .filter_map(|(key, value)| {
                        value.as_str().map(|value| (key.clone(), value.to_owned()))
                    })
                    .collect()
            })
        })
        .unwrap_or_default()
}

fn host_target_text(target: &HostTarget) -> String {
    match target {
        HostTarget::File(path) | HostTarget::Export(path) => path_text(path),
        HostTarget::ExternalCli { program, cwd } => cwd
            .as_ref()
            .map(|cwd| format!("{program} cwd={}", path_text(cwd)))
            .unwrap_or_else(|| program.clone()),
    }
}

fn runtime_home_for_host_config(scope: HostScope, runtime_home: &Path) -> Option<&Path> {
    match scope {
        HostScope::User | HostScope::Local | HostScope::Export => Some(runtime_home),
        HostScope::Project => None,
    }
}

fn deterministic_connection_id(
    host_kind: HostKind,
    scope: HostScope,
    project_id: Option<&str>,
    config_target: &str,
    server_name: &str,
) -> String {
    let key = json!({
        "host_kind": host_kind.as_str(),
        "host_scope": scope.as_str(),
        "project_id": project_id,
        "config_target": config_target,
        "server_name": server_name,
    })
    .to_string();
    let label = match (scope, project_id) {
        (HostScope::Project | HostScope::Local, Some(project_id)) => {
            format!(
                "{}_{}_{}_{}",
                host_kind.as_str(),
                scope.as_str(),
                project_id,
                server_name
            )
        }
        _ => format!("{}_{}_{}", host_kind.as_str(), scope.as_str(), server_name),
    };
    let mut sanitized = sanitize_identifier(&label);
    let suffix = short_hash(&key);
    let max_label = 48usize.saturating_sub(suffix.len() + 6);
    if sanitized.len() > max_label {
        sanitized.truncate(max_label);
        sanitized = sanitized.trim_end_matches('_').to_owned();
    }
    if sanitized.is_empty() {
        format!("conn_{suffix}")
    } else {
        format!("conn_{sanitized}_{suffix}")
    }
}

fn sanitize_identifier(input: &str) -> String {
    let mut out = String::new();
    let mut last_underscore = false;
    for ch in input.chars().flat_map(char::to_lowercase) {
        let next = if ch.is_ascii_alphanumeric() {
            Some(ch)
        } else if ch == '_' || ch == '-' || ch == '.' || ch == '/' || ch == ':' {
            Some('_')
        } else {
            None
        };
        if let Some(ch) = next {
            if ch == '_' {
                if last_underscore {
                    continue;
                }
                last_underscore = true;
            } else {
                last_underscore = false;
            }
            out.push(ch);
        }
    }
    out.trim_matches('_').to_owned()
}

fn short_hash(input: &str) -> String {
    let digest = Sha256::digest(input.as_bytes());
    let mut text = String::new();
    for byte in digest.iter().take(6) {
        text.push_str(&format!("{byte:02x}"));
    }
    text
}

fn resolve_export_target(
    parsed: &ParsedAgentOptions,
    current_dir: &Path,
    connection_id: Option<&str>,
) -> Option<PathBuf> {
    parsed
        .export_path
        .as_ref()
        .map(|path| absolute_path(current_dir, path.clone()))
        .or_else(|| {
            parsed.export_dir.as_ref().map(|dir| {
                let dir = absolute_path(current_dir, dir.clone());
                let stem = connection_id.unwrap_or(DEFAULT_SERVER_NAME);
                dir.join(format!("volicord-{}.mcp.json", sanitize_identifier(stem)))
            })
        })
}

fn generic_default_export_target(parsed: &ParsedAgentOptions, server_name: &str) -> PathBuf {
    parsed
        .export_dir
        .clone()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(format!(
            "volicord-{}.mcp.json",
            sanitize_identifier(server_name)
        ))
}

fn codex_environment(process: &impl AgentProcess) -> CodexEnvironment {
    CodexEnvironment {
        home: process.env_var("HOME").map(PathBuf::from),
        codex_home: process.env_var("CODEX_HOME").map(PathBuf::from),
        path: process.env_var(PATH_ENV),
    }
}

fn codex_home(process: &impl AgentProcess) -> Result<PathBuf, AgentCommandError> {
    if let Some(path) = process.env_var("CODEX_HOME") {
        return Ok(PathBuf::from(path));
    }
    let home = process.env_var("HOME").ok_or_else(|| {
        AgentCommandError::runtime("Codex user configuration requires CODEX_HOME or HOME")
    })?;
    Ok(PathBuf::from(home).join(".codex"))
}

fn absolute_path(current_dir: &Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        current_dir.join(path)
    }
}

fn path_text(path: &Path) -> String {
    path.display().to_string()
}

fn status_text(status_code: Option<i32>) -> String {
    status_code
        .map(|code| code.to_string())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn compact_stream(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_connection_id_includes_connection_unit_inputs() {
        let first = deterministic_connection_id(
            HostKind::Codex,
            HostScope::Project,
            Some("project_a"),
            "/repo/.codex/config.toml",
            "volicord",
        );
        let second = deterministic_connection_id(
            HostKind::Codex,
            HostScope::Project,
            Some("project_b"),
            "/repo/.codex/config.toml",
            "volicord",
        );

        assert!(first.starts_with("conn_codex_project_project_a_"));
        assert_ne!(first, second);
    }

    #[test]
    fn connection_mode_defaults_and_validates() {
        assert_eq!(
            parse_connection_mode(CONNECTION_MODE_READ_ONLY).unwrap(),
            CONNECTION_MODE_READ_ONLY
        );
        assert_eq!(
            parse_connection_mode(CONNECTION_MODE_WORKFLOW).unwrap(),
            CONNECTION_MODE_WORKFLOW
        );
        assert!(parse_connection_mode("full").is_err());
    }
}
