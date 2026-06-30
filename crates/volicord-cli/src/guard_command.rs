use std::{
    collections::BTreeSet,
    ffi::OsString,
    fmt, fs,
    io::{self, Read},
    path::{Component, Path, PathBuf},
    str::FromStr,
    time::SystemTime,
};

use chrono::{DateTime, SecondsFormat, Utc};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use volicord_core::{CorePipelineError, CoreService, InvocationContext};
use volicord_store::{
    bootstrap::{project_record_for_execution, ProjectRecord},
    core_pipeline::CoreProjectStore,
    guards::{
        agent_session, guard_event, insert_agent_session, insert_guard_event,
        insert_prompt_capture, insert_unrecorded_change, list_unresolved_unrecorded_changes,
        prompt_capture, unrecorded_change, AgentSessionInsert, GuardEventInsert,
        PromptCaptureInsert, UnrecordedChangeInsert,
    },
    runtime_home::{resolve_runtime_home, RuntimeHomeResolutionError},
    StoreError,
};
use volicord_types::{
    ActorSource, GuardDecision, HostKind, OperationCategory, ProjectId, RequestId, StatusInclude,
    StatusRequest, TaskId, ToolEnvelope, UtcTimestamp,
    VERIFICATION_BASIS_MCP_STDIO_CONNECTION_BINDING,
};

use crate::project_context::{
    registered_project_for_repo, resolve_repository_root, ProjectCommandError,
};

const GUARD_SCHEMA_VERSION: u64 = 1;
const DEFAULT_GUARD_MODE: &str = "guarded";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardCommandOutcome {
    pub output: String,
    pub exits_failure: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardCommandError {
    Usage(String),
    Runtime(String),
}

impl fmt::Display for GuardCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) | Self::Runtime(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for GuardCommandError {}

impl From<StoreError> for GuardCommandError {
    fn from(error: StoreError) -> Self {
        Self::Runtime(error.to_string())
    }
}

impl From<RuntimeHomeResolutionError> for GuardCommandError {
    fn from(error: RuntimeHomeResolutionError) -> Self {
        Self::Runtime(error.to_string())
    }
}

impl From<ProjectCommandError> for GuardCommandError {
    fn from(error: ProjectCommandError) -> Self {
        match error {
            ProjectCommandError::Usage(message) => Self::Usage(message),
            ProjectCommandError::Runtime(message) => Self::Runtime(message),
        }
    }
}

impl From<CorePipelineError> for GuardCommandError {
    fn from(error: CorePipelineError) -> Self {
        Self::Runtime(error.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GuardPhase {
    SessionStart,
    PreTool,
    PostTool,
    PromptCapture,
    Stop,
}

impl GuardPhase {
    fn event_kind(self) -> &'static str {
        match self {
            Self::SessionStart => "session_start",
            Self::PreTool => "pre_tool",
            Self::PostTool => "post_tool",
            Self::PromptCapture => "prompt_capture",
            Self::Stop => "stop",
        }
    }

    fn command_name(self) -> &'static str {
        match self {
            Self::SessionStart => "session-start",
            Self::PreTool => "pre-tool",
            Self::PostTool => "post-tool",
            Self::PromptCapture => "prompt-capture",
            Self::Stop => "stop",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Json,
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GuardOptions {
    event_file: Option<PathBuf>,
    repo: Option<PathBuf>,
    connection_id: Option<String>,
    session_id: Option<String>,
    guard_installation_id: Option<String>,
    host_kind: Option<String>,
    guard_mode: Option<String>,
    output: OutputFormat,
}

impl Default for GuardOptions {
    fn default() -> Self {
        Self {
            event_file: None,
            repo: None,
            connection_id: None,
            session_id: None,
            guard_installation_id: None,
            host_kind: None,
            guard_mode: None,
            output: OutputFormat::Json,
        }
    }
}

#[derive(Debug, Clone)]
struct GuardInput {
    raw_text: String,
    raw_value: Value,
    raw_sha256: String,
    redacted_value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GuardEnvelope {
    event_id: String,
    session_id: Option<String>,
    connection_id: String,
    guard_installation_id: Option<String>,
    host_kind: String,
    guard_mode: String,
    occurred_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GuardStateSummary {
    project_id: String,
    project_name: String,
    repo_root: String,
    state_version: u64,
    active_task_id: Option<String>,
    current_write_check_ids: Vec<String>,
    stale_write_check_ids: Vec<String>,
    pending_user_judgment_count: usize,
    active_blocker_count: usize,
    unresolved_unrecorded_change_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ToolObservation {
    tool_name: Option<String>,
    command: Option<String>,
    classification: ToolClassification,
    paths: Vec<PathAssessment>,
    changed_paths: Vec<PathAssessment>,
    explicit_write_attempt: bool,
    exit_code: Option<i64>,
    success: Option<bool>,
    status: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ToolClassification {
    ReadOnly,
    Mutating,
    UnknownMutationRisk,
    Unknown,
}

impl ToolClassification {
    fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::Mutating => "mutating",
            Self::UnknownMutationRisk => "unknown_mutation_risk",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PathAssessment {
    raw: String,
    normalized: Option<String>,
    inside_repo: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GuardReason {
    code: &'static str,
    message: String,
    severity: &'static str,
}

pub fn guard_usage() -> String {
    concat!(
        "volicord guard session-start [--file PATH] [--repo PATH] [--connection ID] [--session ID] [--guard-installation ID] [--host HOST] [--guard-mode MODE] [--text]\n",
        "volicord guard pre-tool [--file PATH] [--repo PATH] [--connection ID] [--session ID] [--guard-installation ID] [--host HOST] [--guard-mode MODE] [--text]\n",
        "volicord guard post-tool [--file PATH] [--repo PATH] [--connection ID] [--session ID] [--guard-installation ID] [--host HOST] [--guard-mode MODE] [--text]\n",
        "volicord guard prompt-capture [--file PATH] [--repo PATH] [--connection ID] [--session ID] [--guard-installation ID] [--host HOST] [--guard-mode MODE] [--text]\n",
        "volicord guard stop [--file PATH] [--repo PATH] [--connection ID] [--session ID] [--guard-installation ID] [--host HOST] [--guard-mode MODE] [--text]\n",
    )
    .to_owned()
}

pub fn run_guard_command<F>(
    args: &[String],
    env_var: F,
    current_dir: &Path,
) -> Result<GuardCommandOutcome, GuardCommandError>
where
    F: Fn(&str) -> Option<OsString>,
{
    let Some(subcommand) = args.first().map(String::as_str) else {
        return Ok(GuardCommandOutcome {
            output: guard_usage(),
            exits_failure: false,
        });
    };
    if matches!(subcommand, "-h" | "--help" | "help") {
        if args.len() == 1 {
            return Ok(GuardCommandOutcome {
                output: guard_usage(),
                exits_failure: false,
            });
        }
        return Err(GuardCommandError::Usage(format!(
            "unexpected argument: {}\n\n{}",
            args[1],
            guard_usage()
        )));
    }

    let phase = match subcommand {
        "session-start" => GuardPhase::SessionStart,
        "pre-tool" => GuardPhase::PreTool,
        "post-tool" => GuardPhase::PostTool,
        "prompt-capture" => GuardPhase::PromptCapture,
        "stop" => GuardPhase::Stop,
        other => {
            return Err(GuardCommandError::Usage(format!(
                "unknown guard command: {other}\n\n{}",
                guard_usage()
            )))
        }
    };
    let options = parse_guard_options(&args[1..])?;
    let runtime_home = resolve_runtime_home(env_var, current_dir)?;
    let input = read_guard_input(options.event_file.as_deref())?;
    let project = resolve_guard_project(&runtime_home, current_dir, &options, &input.raw_value)?;
    let envelope = guard_envelope(phase, &options, &input, &project)?;
    ensure_required_session(&runtime_home, &project, &envelope, phase)?;

    let (decision, result, exits_failure) = match phase {
        GuardPhase::SessionStart => {
            let summary = guard_state_summary(&runtime_home, &project, &envelope, &input)?;
            (
                GuardDecision::InjectContext,
                json!({
                    "decision": GuardDecision::InjectContext.as_str(),
                    "message": "Volicord context is available for this host session.",
                    "context": context_json(&summary),
                    "enforcement_level": "cooperative_detective"
                }),
                false,
            )
        }
        GuardPhase::PreTool => {
            let summary = guard_state_summary(&runtime_home, &project, &envelope, &input)?;
            let observation = tool_observation(&input.raw_value, &project.repo_root);
            let (decision, reasons) = pre_tool_decision(&summary, &observation, &input.raw_value);
            let exits_failure = decision == GuardDecision::Deny;
            (
                decision,
                json!({
                    "decision": decision.as_str(),
                    "allowed": decision != GuardDecision::Deny,
                    "reasons": reasons_json(&reasons),
                    "tool": tool_observation_json(&observation),
                    "context": context_json(&summary),
                    "enforcement_level": "cooperative_detective"
                }),
                exits_failure,
            )
        }
        GuardPhase::PostTool => {
            let summary = guard_state_summary(&runtime_home, &project, &envelope, &input)?;
            let observation = tool_observation(&input.raw_value, &project.repo_root);
            let inserted_changes = record_unrecorded_changes(
                &runtime_home,
                &project,
                &envelope,
                &summary,
                &observation,
            )?;
            let decision = if inserted_changes.is_empty() {
                GuardDecision::Allow
            } else {
                GuardDecision::Warn
            };
            (
                decision,
                json!({
                    "decision": decision.as_str(),
                    "allowed": true,
                    "tool": tool_observation_json(&observation),
                    "unrecorded_changes": inserted_changes,
                    "context": context_json(&summary),
                    "enforcement_level": "cooperative_detective"
                }),
                false,
            )
        }
        GuardPhase::PromptCapture => {
            let capture = record_prompt_capture(&runtime_home, &project, &envelope, &input)?;
            (
                GuardDecision::Allow,
                json!({
                    "decision": GuardDecision::Allow.as_str(),
                    "allowed": true,
                    "prompt_capture": capture,
                    "recognized_judgment_command": null,
                    "enforcement_level": "cooperative_detective"
                }),
                false,
            )
        }
        GuardPhase::Stop => {
            let summary = guard_state_summary(&runtime_home, &project, &envelope, &input)?;
            let (decision, reasons, close_status) =
                stop_decision(&runtime_home, &project, &envelope, &summary)?;
            let exits_failure = decision == GuardDecision::Deny;
            (
                decision,
                json!({
                    "decision": decision.as_str(),
                    "allowed": decision != GuardDecision::Deny,
                    "reasons": reasons_json(&reasons),
                    "close_status": close_status,
                    "context": context_json(&summary),
                    "enforcement_level": "cooperative_detective"
                }),
                exits_failure,
            )
        }
    };

    let subject = guard_subject(phase, &input, &envelope, &project);
    persist_guard_event(
        &runtime_home,
        &project,
        &envelope,
        phase,
        decision,
        subject,
        result.clone(),
    )?;
    Ok(GuardCommandOutcome {
        output: render_guard_output(phase, decision, &envelope, result, options.output)?,
        exits_failure,
    })
}

fn parse_guard_options(args: &[String]) -> Result<GuardOptions, GuardCommandError> {
    let mut options = GuardOptions::default();
    let mut index = 0;
    while index < args.len() {
        let token = &args[index];
        if matches!(token.as_str(), "-h" | "--help" | "help") {
            return Err(GuardCommandError::Usage(guard_usage()));
        }
        if let Some(value) = token.strip_prefix("--file=") {
            set_path_option(&mut options.event_file, "--file", value)?;
        } else if token == "--file" {
            index += 1;
            let value = args
                .get(index)
                .ok_or_else(|| GuardCommandError::Usage("--file requires a value".to_owned()))?;
            set_path_option(&mut options.event_file, "--file", value)?;
        } else if let Some(value) = token.strip_prefix("--repo=") {
            set_path_option(&mut options.repo, "--repo", value)?;
        } else if token == "--repo" {
            index += 1;
            let value = args
                .get(index)
                .ok_or_else(|| GuardCommandError::Usage("--repo requires a value".to_owned()))?;
            set_path_option(&mut options.repo, "--repo", value)?;
        } else if let Some(value) = token.strip_prefix("--connection=") {
            set_string_option(&mut options.connection_id, "--connection", value)?;
        } else if token == "--connection" {
            index += 1;
            let value = args.get(index).ok_or_else(|| {
                GuardCommandError::Usage("--connection requires a value".to_owned())
            })?;
            set_string_option(&mut options.connection_id, "--connection", value)?;
        } else if let Some(value) = token.strip_prefix("--session=") {
            set_string_option(&mut options.session_id, "--session", value)?;
        } else if token == "--session" {
            index += 1;
            let value = args
                .get(index)
                .ok_or_else(|| GuardCommandError::Usage("--session requires a value".to_owned()))?;
            set_string_option(&mut options.session_id, "--session", value)?;
        } else if let Some(value) = token.strip_prefix("--guard-installation=") {
            set_string_option(
                &mut options.guard_installation_id,
                "--guard-installation",
                value,
            )?;
        } else if token == "--guard-installation" {
            index += 1;
            let value = args.get(index).ok_or_else(|| {
                GuardCommandError::Usage("--guard-installation requires a value".to_owned())
            })?;
            set_string_option(
                &mut options.guard_installation_id,
                "--guard-installation",
                value,
            )?;
        } else if let Some(value) = token.strip_prefix("--host=") {
            set_string_option(&mut options.host_kind, "--host", value)?;
        } else if token == "--host" {
            index += 1;
            let value = args
                .get(index)
                .ok_or_else(|| GuardCommandError::Usage("--host requires a value".to_owned()))?;
            set_string_option(&mut options.host_kind, "--host", value)?;
        } else if let Some(value) = token.strip_prefix("--guard-mode=") {
            set_string_option(&mut options.guard_mode, "--guard-mode", value)?;
        } else if token == "--guard-mode" {
            index += 1;
            let value = args.get(index).ok_or_else(|| {
                GuardCommandError::Usage("--guard-mode requires a value".to_owned())
            })?;
            set_string_option(&mut options.guard_mode, "--guard-mode", value)?;
        } else if token == "--text" {
            options.output = OutputFormat::Text;
        } else if token == "--json" {
            options.output = OutputFormat::Json;
        } else if token.starts_with('-') {
            return Err(GuardCommandError::Usage(format!("unknown option: {token}")));
        } else {
            return Err(GuardCommandError::Usage(format!(
                "unexpected argument: {token}"
            )));
        }
        index += 1;
    }
    Ok(options)
}

fn set_path_option(
    slot: &mut Option<PathBuf>,
    option: &'static str,
    value: &str,
) -> Result<(), GuardCommandError> {
    if slot.is_some() {
        return Err(GuardCommandError::Usage(format!(
            "{option} was supplied more than once"
        )));
    }
    if value.trim().is_empty() {
        return Err(GuardCommandError::Usage(format!(
            "{option} requires a non-empty value"
        )));
    }
    *slot = Some(PathBuf::from(value));
    Ok(())
}

fn set_string_option(
    slot: &mut Option<String>,
    option: &'static str,
    value: &str,
) -> Result<(), GuardCommandError> {
    if slot.is_some() {
        return Err(GuardCommandError::Usage(format!(
            "{option} was supplied more than once"
        )));
    }
    if value.trim().is_empty() {
        return Err(GuardCommandError::Usage(format!(
            "{option} requires a non-empty value"
        )));
    }
    *slot = Some(value.to_owned());
    Ok(())
}

fn read_guard_input(path: Option<&Path>) -> Result<GuardInput, GuardCommandError> {
    let raw_text = match path {
        Some(path) => fs::read_to_string(path).map_err(|error| {
            GuardCommandError::Runtime(format!(
                "failed to read guard event file {}: {error}",
                path.display()
            ))
        })?,
        None => {
            let mut text = String::new();
            io::stdin().read_to_string(&mut text).map_err(|error| {
                GuardCommandError::Runtime(format!("failed to read guard event stdin: {error}"))
            })?;
            text
        }
    };
    if raw_text.trim().is_empty() {
        return Err(GuardCommandError::Usage(
            "guard event JSON must not be empty".to_owned(),
        ));
    }
    let raw_value = serde_json::from_str::<Value>(&raw_text)
        .map_err(|error| GuardCommandError::Usage(format!("guard event must be JSON: {error}")))?;
    let raw_sha256 = sha256_text(&raw_text);
    let redacted_value = redact_event_value(&raw_value);
    Ok(GuardInput {
        raw_text,
        raw_value,
        raw_sha256,
        redacted_value,
    })
}

fn resolve_guard_project(
    runtime_home: &Path,
    current_dir: &Path,
    options: &GuardOptions,
    event: &Value,
) -> Result<ProjectRecord, GuardCommandError> {
    if let Some(repo) = options
        .repo
        .as_deref()
        .or_else(|| event_path_field(event, &[&["repo_root"], &["repository_root"], &["cwd"]]))
    {
        let repo_root = resolve_repository_root(current_dir, Some(repo))?;
        return registered_project_for_repo(runtime_home, &repo_root).map_err(Into::into);
    }
    if let Some(project_id) = event_string(event, &[&["project_id"], &["project", "id"]]) {
        return project_record_for_execution(runtime_home, &project_id)?
            .ok_or_else(|| GuardCommandError::Runtime(format!("project not found: {project_id}")));
    }
    let repo_root = resolve_repository_root(current_dir, None)?;
    registered_project_for_repo(runtime_home, &repo_root).map_err(Into::into)
}

fn event_path_field<'a>(event: &'a Value, paths: &[&[&str]]) -> Option<&'a Path> {
    for path in paths {
        if let Some(value) = value_at(event, path).and_then(Value::as_str) {
            if !value.trim().is_empty() {
                return Some(Path::new(value));
            }
        }
    }
    None
}

fn guard_envelope(
    phase: GuardPhase,
    options: &GuardOptions,
    input: &GuardInput,
    project: &ProjectRecord,
) -> Result<GuardEnvelope, GuardCommandError> {
    let connection_id = options
        .connection_id
        .clone()
        .or_else(|| {
            event_string(
                &input.raw_value,
                &[
                    &["connection_id"],
                    &["connection_internal_id"],
                    &["connection", "id"],
                    &["volicord", "connection_id"],
                ],
            )
        })
        .ok_or_else(|| {
            GuardCommandError::Usage(
                "guard command requires --connection or connection_id in the event".to_owned(),
            )
        })?;
    let host_kind = normalize_host_kind(
        options
            .host_kind
            .clone()
            .or_else(|| {
                event_string(
                    &input.raw_value,
                    &[
                        &["host_kind"],
                        &["host", "kind"],
                        &["source", "host_kind"],
                        &["source", "host"],
                    ],
                )
            })
            .unwrap_or_else(|| "generic".to_owned()),
    )?;
    let guard_mode = normalize_guard_mode(
        options
            .guard_mode
            .clone()
            .or_else(|| event_string(&input.raw_value, &[&["guard_mode"], &["guard", "mode"]]))
            .unwrap_or_else(|| DEFAULT_GUARD_MODE.to_owned()),
    )?;
    let session_id = options.session_id.clone().or_else(|| {
        event_string(
            &input.raw_value,
            &[
                &["session_id"],
                &["session", "id"],
                &["conversation_id"],
                &["transcript_id"],
            ],
        )
    });
    let session_id = match (phase, session_id) {
        (GuardPhase::SessionStart | GuardPhase::PromptCapture, None) => Some(stable_id(
            "agent_session",
            &[
                phase.command_name(),
                &connection_id,
                &project.project_id,
                &input.raw_sha256,
            ],
        )),
        (_, value) => value,
    };
    let event_id = event_string(
        &input.raw_value,
        &[
            &["guard_event_id"],
            &["event_id"],
            &["hook_event_id"],
            &["tool_call_id"],
            &["id"],
        ],
    )
    .unwrap_or_else(|| {
        stable_id(
            "guard_event",
            &[
                phase.command_name(),
                &connection_id,
                session_id.as_deref().unwrap_or(""),
                &project.project_id,
                &input.raw_sha256,
            ],
        )
    });
    let occurred_at = event_string(
        &input.raw_value,
        &[&["occurred_at"], &["timestamp"], &["time"]],
    )
    .unwrap_or_else(current_timestamp);
    Ok(GuardEnvelope {
        event_id,
        session_id,
        connection_id,
        guard_installation_id: options.guard_installation_id.clone().or_else(|| {
            event_string(
                &input.raw_value,
                &[
                    &["guard_installation_id"],
                    &["guard", "installation_id"],
                    &["volicord", "guard_installation_id"],
                ],
            )
        }),
        host_kind,
        guard_mode,
        occurred_at,
    })
}

fn normalize_host_kind(value: String) -> Result<String, GuardCommandError> {
    let normalized = match value.as_str() {
        "claude-code" => "claude_code".to_owned(),
        other => other.to_owned(),
    };
    HostKind::from_str(&normalized).map_err(|error| GuardCommandError::Usage(error.to_string()))?;
    Ok(normalized)
}

fn normalize_guard_mode(value: String) -> Result<String, GuardCommandError> {
    let normalized = match value.as_str() {
        "mcp-only" => "mcp_only".to_owned(),
        other => other.to_owned(),
    };
    if matches!(normalized.as_str(), "mcp_only" | "guarded" | "managed") {
        Ok(normalized)
    } else {
        Err(GuardCommandError::Usage(
            "guard mode must be mcp_only, guarded, or managed".to_owned(),
        ))
    }
}

fn ensure_required_session(
    runtime_home: &Path,
    project: &ProjectRecord,
    envelope: &GuardEnvelope,
    phase: GuardPhase,
) -> Result<(), GuardCommandError> {
    let Some(session_id) = envelope.session_id.as_deref() else {
        return Ok(());
    };
    if agent_session(runtime_home, &project.project_id, session_id)?.is_some() {
        return Ok(());
    }
    if matches!(phase, GuardPhase::SessionStart | GuardPhase::PromptCapture)
        || envelope.session_id.is_some()
    {
        insert_agent_session(
            runtime_home,
            &project.project_id,
            AgentSessionInsert {
                session_id: session_id.to_owned(),
                connection_internal_id: envelope.connection_id.clone(),
                guard_installation_id: envelope.guard_installation_id.clone(),
                host_kind: envelope.host_kind.clone(),
                guard_mode: envelope.guard_mode.clone(),
                started_at: envelope.occurred_at.clone(),
                metadata_json: json!({
                    "source": "volicord_guard_cli",
                    "schema_version": GUARD_SCHEMA_VERSION
                })
                .to_string(),
            },
        )?;
    }
    Ok(())
}

fn guard_state_summary(
    runtime_home: &Path,
    project: &ProjectRecord,
    envelope: &GuardEnvelope,
    input: &GuardInput,
) -> Result<GuardStateSummary, GuardCommandError> {
    let store = CoreProjectStore::open(runtime_home, &ProjectId::new(&project.project_id))?;
    let project_state = store.project_state()?;
    let now = event_time_or_now(&envelope.occurred_at);
    let now_timestamp = UtcTimestamp::from_datetime(now);
    let mut current_write_check_ids = Vec::new();
    let mut stale_write_check_ids = Vec::new();
    let mut pending_user_judgment_count = 0;
    let mut active_blocker_count = 0;
    if let Some(active_task_id) = project_state.active_task_id.as_deref() {
        let task_id = TaskId::new(active_task_id);
        for record in store.active_write_checks(&task_id)? {
            let current_basis = record.basis_state_version == project_state.state_version;
            let not_expired = UtcTimestamp::parse(&record.expires_at)
                .map(|expires_at| now_timestamp < expires_at)
                .unwrap_or(false);
            if current_basis && not_expired {
                current_write_check_ids.push(record.write_check_id);
            } else {
                stale_write_check_ids.push(record.write_check_id);
            }
        }
        pending_user_judgment_count = store.pending_user_judgment_records(&task_id)?.len();
        active_blocker_count = store
            .active_blocker_refs(&task_id, project_state.state_version)?
            .len();
    }
    let unresolved_unrecorded_change_count = list_unresolved_unrecorded_changes(
        runtime_home,
        &project.project_id,
        Some(&envelope.connection_id),
    )?
    .len();
    let _ = input.raw_text.len();
    Ok(GuardStateSummary {
        project_id: project.project_id.clone(),
        project_name: project.project_name.clone(),
        repo_root: project.repo_root.display().to_string(),
        state_version: project_state.state_version,
        active_task_id: project_state.active_task_id,
        current_write_check_ids,
        stale_write_check_ids,
        pending_user_judgment_count,
        active_blocker_count,
        unresolved_unrecorded_change_count,
    })
}

fn tool_observation(event: &Value, repo_root: &Path) -> ToolObservation {
    let tool_name = event_string(
        event,
        &[
            &["tool_name"],
            &["tool", "name"],
            &["tool_use", "name"],
            &["tool"],
        ],
    );
    let command = event_string(
        event,
        &[
            &["command"],
            &["tool_input", "command"],
            &["input", "command"],
            &["tool", "input", "command"],
            &["tool", "arguments", "command"],
            &["tool_use", "input", "command"],
        ],
    );
    let classification = classify_tool(tool_name.as_deref(), command.as_deref());
    let paths = collect_path_assessments(event, repo_root, false);
    let changed_paths = collect_path_assessments(event, repo_root, true);
    let explicit_write_attempt = event_bool(
        event,
        &[
            &["product_file_write_intended"],
            &["write_attempt"],
            &["mutates_files"],
            &["tool_input", "product_file_write_intended"],
            &["tool_input", "write_attempt"],
            &["input", "product_file_write_intended"],
            &["input", "write_attempt"],
        ],
    )
    .unwrap_or(false);
    ToolObservation {
        tool_name,
        command,
        classification,
        paths,
        changed_paths,
        explicit_write_attempt,
        exit_code: event_i64(
            event,
            &[
                &["exit_code"],
                &["tool_result", "exit_code"],
                &["result", "exit_code"],
                &["output", "exit_code"],
            ],
        ),
        success: event_bool(
            event,
            &[
                &["success"],
                &["tool_result", "success"],
                &["result", "success"],
                &["output", "success"],
            ],
        ),
        status: event_string(
            event,
            &[
                &["status"],
                &["tool_result", "status"],
                &["result", "status"],
                &["output", "status"],
            ],
        ),
    }
}

fn classify_tool(tool_name: Option<&str>, command: Option<&str>) -> ToolClassification {
    let normalized_tool = tool_name.unwrap_or("").trim().to_ascii_lowercase();
    if matches!(
        normalized_tool.as_str(),
        "read" | "view" | "grep" | "search" | "list" | "glob"
    ) {
        return ToolClassification::ReadOnly;
    }
    if matches!(
        normalized_tool.as_str(),
        "edit" | "write" | "write_file" | "apply_patch" | "patch" | "notebook_edit"
    ) {
        return ToolClassification::Mutating;
    }
    let Some(command) = command.map(str::trim).filter(|value| !value.is_empty()) else {
        return if normalized_tool.is_empty() {
            ToolClassification::Unknown
        } else {
            ToolClassification::UnknownMutationRisk
        };
    };
    if shell_command_is_clearly_mutating(command) {
        return ToolClassification::Mutating;
    }
    if shell_command_is_read_only(command) {
        return ToolClassification::ReadOnly;
    }
    ToolClassification::UnknownMutationRisk
}

fn shell_command_is_clearly_mutating(command: &str) -> bool {
    let compact = format!(" {command} ");
    if compact.contains(" > ") || compact.contains(" >> ") || compact.contains(" tee ") {
        return true;
    }
    if compact.contains(" sed -i ")
        || compact.contains(" perl -pi ")
        || compact.contains(" git add ")
        || compact.contains(" git commit ")
        || compact.contains(" git reset ")
        || compact.contains(" git clean ")
        || compact.contains(" git checkout ")
        || compact.contains(" git switch ")
    {
        return true;
    }
    command_segments(command).iter().any(|segment| {
        let first = first_command_word(segment);
        matches!(
            first.as_deref(),
            Some(
                "rm" | "mv"
                    | "cp"
                    | "touch"
                    | "mkdir"
                    | "rmdir"
                    | "ln"
                    | "chmod"
                    | "chown"
                    | "truncate"
                    | "install"
                    | "cargo-fmt"
            )
        ) || segment.trim_start().starts_with("cargo fmt")
            || segment.trim_start().starts_with("npm install")
            || segment.trim_start().starts_with("pnpm install")
            || segment.trim_start().starts_with("yarn install")
    })
}

fn shell_command_is_read_only(command: &str) -> bool {
    command_segments(command).iter().all(|segment| {
        let trimmed = segment.trim();
        if trimmed.is_empty() {
            return true;
        }
        if trimmed.contains(" -delete") {
            return false;
        }
        let first = first_command_word(trimmed);
        matches!(
            first.as_deref(),
            Some(
                "pwd"
                    | "ls"
                    | "cat"
                    | "rg"
                    | "grep"
                    | "find"
                    | "wc"
                    | "head"
                    | "tail"
                    | "sed"
                    | "awk"
                    | "git"
                    | "cargo"
                    | "npm"
                    | "pnpm"
                    | "yarn"
                    | "node"
                    | "rustc"
            )
        ) && !trimmed.starts_with("cargo fmt")
            && !trimmed.starts_with("npm install")
            && !trimmed.starts_with("pnpm install")
            && !trimmed.starts_with("yarn install")
            && !trimmed.starts_with("git add")
            && !trimmed.starts_with("git commit")
            && !trimmed.starts_with("git reset")
            && !trimmed.starts_with("git clean")
            && !trimmed.starts_with("git checkout")
            && !trimmed.starts_with("git switch")
    })
}

fn command_segments(command: &str) -> Vec<&str> {
    command
        .split([';', '\n'])
        .flat_map(|part| part.split("&&"))
        .flat_map(|part| part.split("||"))
        .collect()
}

fn first_command_word(segment: &str) -> Option<String> {
    let mut words = segment.split_whitespace();
    let first = words.next()?;
    if first == "sudo" || first == "command" {
        words.next().map(str::to_owned)
    } else {
        Some(first.to_owned())
    }
}

fn collect_path_assessments(
    event: &Value,
    repo_root: &Path,
    changed_only: bool,
) -> Vec<PathAssessment> {
    let mut raw_paths = BTreeSet::new();
    collect_paths_recursive(event, changed_only, &mut raw_paths);
    if !changed_only {
        if let Some(command) = event_string(
            event,
            &[
                &["command"],
                &["tool_input", "command"],
                &["input", "command"],
                &["tool", "input", "command"],
            ],
        ) {
            raw_paths.extend(paths_from_redirection(&command));
        }
    }
    raw_paths
        .into_iter()
        .map(|raw| assess_path(repo_root, &raw))
        .collect()
}

fn collect_paths_recursive(value: &Value, changed_only: bool, paths: &mut BTreeSet<String>) {
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                let path_key = if changed_only {
                    matches!(
                        key.as_str(),
                        "changed_paths" | "observed_paths" | "modified_paths"
                    )
                } else {
                    matches!(
                        key.as_str(),
                        "paths"
                            | "path"
                            | "file_path"
                            | "target_path"
                            | "changed_paths"
                            | "observed_paths"
                            | "modified_paths"
                    )
                };
                if path_key {
                    collect_string_values(value, paths);
                }
                collect_paths_recursive(value, changed_only, paths);
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_paths_recursive(value, changed_only, paths);
            }
        }
        _ => {}
    }
}

fn collect_string_values(value: &Value, values: &mut BTreeSet<String>) {
    match value {
        Value::String(text) if !text.trim().is_empty() => {
            values.insert(text.to_owned());
        }
        Value::Array(items) => {
            for item in items {
                collect_string_values(item, values);
            }
        }
        _ => {}
    }
}

fn paths_from_redirection(command: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let words = command.split_whitespace().collect::<Vec<_>>();
    for (index, word) in words.iter().enumerate() {
        if matches!(*word, ">" | ">>") {
            if let Some(path) = words.get(index + 1) {
                paths.push(path.trim_matches('"').trim_matches('\'').to_owned());
            }
        }
    }
    paths
}

fn assess_path(repo_root: &Path, raw: &str) -> PathAssessment {
    let path = Path::new(raw);
    let (inside_repo, normalized) = if path.is_absolute() {
        match path.strip_prefix(repo_root) {
            Ok(relative) => normalized_relative_path(relative)
                .map(|path| (true, Some(path)))
                .unwrap_or((false, None)),
            Err(_) => (false, None),
        }
    } else {
        normalized_relative_path(path)
            .map(|path| (true, Some(path)))
            .unwrap_or((false, None))
    };
    PathAssessment {
        raw: raw.to_owned(),
        normalized,
        inside_repo,
    }
}

fn normalized_relative_path(path: &Path) -> Option<String> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => parts.push(value.to_string_lossy().into_owned()),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("/"))
    }
}

fn pre_tool_decision(
    summary: &GuardStateSummary,
    observation: &ToolObservation,
    event: &Value,
) -> (GuardDecision, Vec<GuardReason>) {
    let mut reasons = Vec::new();
    let product_file_write_attempt = observation.explicit_write_attempt
        || observation.classification == ToolClassification::Mutating
        || tool_name_implies_write(observation.tool_name.as_deref());
    if observation
        .paths
        .iter()
        .chain(observation.changed_paths.iter())
        .any(|path| !path.inside_repo)
    {
        reasons.push(GuardReason {
            code: "target_outside_project_allowlist",
            message: "One or more target paths are outside the selected Product Repository."
                .to_owned(),
            severity: "deny",
        });
    }
    if product_file_write_attempt {
        if summary.active_task_id.is_none() {
            reasons.push(GuardReason {
                code: "no_active_task",
                message: "Product-file writes require an active Volicord task.".to_owned(),
                severity: "deny",
            });
        } else if summary.current_write_check_ids.is_empty() {
            reasons.push(GuardReason {
                code: "write_readiness_missing",
                message: "The current task does not have a current active Write Check.".to_owned(),
                severity: "deny",
            });
        }
    }
    if observation.classification == ToolClassification::UnknownMutationRisk {
        let severity = event_string(
            event,
            &[
                &["policy", "unknown_mutation_decision"],
                &["guard_policy", "unknown_mutation_decision"],
            ],
        )
        .unwrap_or_else(|| "warn".to_owned());
        reasons.push(GuardReason {
            code: "unknown_mutation_risk",
            message: "Volicord could not confidently classify this tool invocation as read-only."
                .to_owned(),
            severity: if severity == "deny" { "deny" } else { "warn" },
        });
    }
    if observation.classification == ToolClassification::Mutating
        && event_bool(
            event,
            &[
                &["policy", "block_mutating_shell"],
                &["guard_policy", "block_mutating_shell"],
            ],
        )
        .unwrap_or(false)
    {
        reasons.push(GuardReason {
            code: "mutating_shell_blocked_by_policy",
            message: "Guard policy blocks clearly mutating shell commands.".to_owned(),
            severity: "deny",
        });
    }
    let decision = if reasons.iter().any(|reason| reason.severity == "deny") {
        GuardDecision::Deny
    } else if reasons.iter().any(|reason| reason.severity == "warn") {
        GuardDecision::Warn
    } else {
        GuardDecision::Allow
    };
    (decision, reasons)
}

fn tool_name_implies_write(tool_name: Option<&str>) -> bool {
    tool_name
        .map(|name| {
            matches!(
                name.to_ascii_lowercase().as_str(),
                "edit" | "write" | "write_file" | "apply_patch" | "patch" | "notebook_edit"
            )
        })
        .unwrap_or(false)
}

fn record_unrecorded_changes(
    runtime_home: &Path,
    project: &ProjectRecord,
    envelope: &GuardEnvelope,
    summary: &GuardStateSummary,
    observation: &ToolObservation,
) -> Result<Vec<Value>, GuardCommandError> {
    if observation.tool_name.as_deref() == Some("volicord.record_run") {
        return Ok(Vec::new());
    }
    let changed = observation
        .changed_paths
        .iter()
        .filter(|path| path.inside_repo)
        .filter_map(|path| path.normalized.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if changed.is_empty() {
        return Ok(Vec::new());
    }
    let change_id = stable_id(
        "unrecorded_change",
        &[&envelope.event_id, &project.project_id, &changed.join("|")],
    );
    if unrecorded_change(runtime_home, &project.project_id, &change_id)?.is_some() {
        return Ok(vec![json!({
            "unrecorded_change_id": change_id,
            "status": "already_recorded",
            "observed_paths": changed
        })]);
    }
    insert_unrecorded_change(
        runtime_home,
        &project.project_id,
        UnrecordedChangeInsert {
            unrecorded_change_id: change_id.clone(),
            session_id: envelope.session_id.clone(),
            connection_internal_id: envelope.connection_id.clone(),
            task_id: summary.active_task_id.clone(),
            summary: "Product file changes were observed after a host tool without a matching Volicord run record".to_owned(),
            observed_paths_json: serde_json::to_string(&changed).map_err(json_error)?,
            detection_json: json!({
                "source": "volicord_guard_post_tool",
                "tool_name": observation.tool_name,
                "exit_code": observation.exit_code,
                "success": observation.success,
                "status": observation.status
            })
            .to_string(),
            detected_at: envelope.occurred_at.clone(),
            metadata_json: json!({
                "guard_event_id": envelope.event_id,
                "schema_version": GUARD_SCHEMA_VERSION
            })
            .to_string(),
        },
    )?;
    Ok(vec![json!({
        "unrecorded_change_id": change_id,
        "status": "unresolved",
        "observed_paths": changed
    })])
}

fn record_prompt_capture(
    runtime_home: &Path,
    project: &ProjectRecord,
    envelope: &GuardEnvelope,
    input: &GuardInput,
) -> Result<Value, GuardCommandError> {
    let Some(prompt) = extract_prompt_text(&input.raw_value) else {
        return Ok(json!({
            "captured": false,
            "reason": "no_prompt_text"
        }));
    };
    let session_id = envelope.session_id.as_ref().ok_or_else(|| {
        GuardCommandError::Runtime("prompt capture requires a session id".to_owned())
    })?;
    let prompt_sha256 = sha256_text(&prompt);
    let capture_id = event_string(
        &input.raw_value,
        &[&["prompt_capture_id"], &["capture_id"], &["id"]],
    )
    .unwrap_or_else(|| stable_id("prompt_capture", &[session_id, &prompt_sha256]));
    if prompt_capture(runtime_home, &project.project_id, &capture_id)?.is_none() {
        insert_prompt_capture(
            runtime_home,
            &project.project_id,
            PromptCaptureInsert {
                prompt_capture_id: capture_id.clone(),
                session_id: session_id.clone(),
                connection_internal_id: envelope.connection_id.clone(),
                capture_kind: event_string(&input.raw_value, &[&["capture_kind"]])
                    .unwrap_or_else(|| "user_prompt".to_owned()),
                prompt_sha256: prompt_sha256.clone(),
                prompt_text: None,
                captured_at: envelope.occurred_at.clone(),
                metadata_json: json!({
                    "source": "volicord_guard_prompt_capture",
                    "raw_event_sha256": input.raw_sha256,
                    "prompt_size_bytes": prompt.len(),
                    "prompt_text_omitted": true,
                    "schema_version": GUARD_SCHEMA_VERSION
                })
                .to_string(),
            },
        )?;
    }
    Ok(json!({
        "captured": true,
        "prompt_capture_id": capture_id,
        "prompt_sha256": prompt_sha256,
        "prompt_text_omitted": true
    }))
}

fn stop_decision(
    runtime_home: &Path,
    project: &ProjectRecord,
    envelope: &GuardEnvelope,
    summary: &GuardStateSummary,
) -> Result<(GuardDecision, Vec<GuardReason>, Value), GuardCommandError> {
    let Some(task_id) = summary.active_task_id.as_deref() else {
        return Ok((
            GuardDecision::Allow,
            Vec::new(),
            json!({"active_task": null, "close_blockers": []}),
        ));
    };
    let response = CoreService::new(runtime_home).status(
        StatusRequest {
            envelope: ToolEnvelope {
                project_id: ProjectId::new(&project.project_id),
                task_id: Some(TaskId::new(task_id)).into(),
                request_id: RequestId::new(stable_id(
                    "req_guard_stop_status",
                    &[&envelope.event_id, task_id],
                )),
                idempotency_key: None.into(),
                expected_state_version: None.into(),
                dry_run: false,
                locale: None.into(),
            },
            include: StatusInclude {
                task: true,
                pending_user_judgments: true,
                write_check: true,
                evidence: true,
                close: true,
                guarantees: true,
                continuity: false,
            },
        },
        InvocationContext::new(
            ProjectId::new(&project.project_id),
            ActorSource::agent_connection(envelope.connection_id.clone()),
            OperationCategory::Read,
            VERIFICATION_BASIS_MCP_STDIO_CONNECTION_BINDING,
        ),
    )?;
    let close_blockers = response
        .response_value
        .get("close_blockers")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut reasons = Vec::new();
    if !close_blockers.is_empty() {
        reasons.push(GuardReason {
            code: "close_readiness_blocked",
            message: "Close readiness has blockers for the active task.".to_owned(),
            severity: "deny",
        });
    }
    if summary.pending_user_judgment_count > 0 {
        reasons.push(GuardReason {
            code: "pending_user_judgments",
            message: "User-owned judgments are still pending for the active task.".to_owned(),
            severity: "deny",
        });
    }
    if summary.unresolved_unrecorded_change_count > 0 {
        reasons.push(GuardReason {
            code: "unresolved_unrecorded_changes",
            message: "Observed Product Repository changes still need reconciliation.".to_owned(),
            severity: "deny",
        });
    }
    let decision = if reasons.iter().any(|reason| reason.severity == "deny") {
        GuardDecision::Deny
    } else {
        GuardDecision::Allow
    };
    Ok((
        decision,
        reasons,
        json!({
            "active_task": task_id,
            "status_summary": response.response_value.get("status_summary").cloned().unwrap_or(Value::Null),
            "close_state": response.response_value.get("close_state").cloned().unwrap_or(Value::Null),
            "close_blockers": close_blockers
        }),
    ))
}

fn persist_guard_event(
    runtime_home: &Path,
    project: &ProjectRecord,
    envelope: &GuardEnvelope,
    phase: GuardPhase,
    decision: GuardDecision,
    subject: Value,
    result: Value,
) -> Result<(), GuardCommandError> {
    if guard_event(runtime_home, &project.project_id, &envelope.event_id)?.is_some() {
        return Ok(());
    }
    insert_guard_event(
        runtime_home,
        &project.project_id,
        GuardEventInsert {
            guard_event_id: envelope.event_id.clone(),
            session_id: envelope.session_id.clone(),
            connection_internal_id: envelope.connection_id.clone(),
            guard_installation_id: envelope.guard_installation_id.clone(),
            event_kind: phase.event_kind().to_owned(),
            decision: decision.as_str().to_owned(),
            subject_json: object_text(subject)?,
            result_json: object_text(result)?,
            occurred_at: envelope.occurred_at.clone(),
            metadata_json: json!({
                "source": "volicord_guard_cli",
                "schema_version": GUARD_SCHEMA_VERSION,
                "cooperative_detective": true
            })
            .to_string(),
        },
    )?;
    Ok(())
}

fn guard_subject(
    phase: GuardPhase,
    input: &GuardInput,
    envelope: &GuardEnvelope,
    project: &ProjectRecord,
) -> Value {
    json!({
        "schema_version": GUARD_SCHEMA_VERSION,
        "lifecycle_phase": phase.event_kind(),
        "host_kind": envelope.host_kind,
        "connection_id": envelope.connection_id,
        "project_id": project.project_id,
        "repo_root": project.repo_root.display().to_string(),
        "raw_event_sha256": input.raw_sha256,
        "raw_event": input.redacted_value
    })
}

fn render_guard_output(
    phase: GuardPhase,
    decision: GuardDecision,
    envelope: &GuardEnvelope,
    result: Value,
    output: OutputFormat,
) -> Result<String, GuardCommandError> {
    match output {
        OutputFormat::Json => Ok(format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({
                "schema_version": GUARD_SCHEMA_VERSION,
                "phase": phase.event_kind(),
                "decision": decision.as_str(),
                "allowed": decision != GuardDecision::Deny,
                "guard_event_id": envelope.event_id,
                "session_id": envelope.session_id,
                "result": result
            }))
            .map_err(json_error)?
        )),
        OutputFormat::Text => {
            let allowed = if decision == GuardDecision::Deny {
                "blocked"
            } else {
                "allowed"
            };
            Ok(format!(
                "Volicord guard {}: {} ({})\n",
                phase.command_name(),
                decision.as_str(),
                allowed
            ))
        }
    }
}

fn context_json(summary: &GuardStateSummary) -> Value {
    json!({
        "project_id": summary.project_id,
        "project_name": summary.project_name,
        "repo_root": summary.repo_root,
        "state_version": summary.state_version,
        "active_task_id": summary.active_task_id,
        "current_write_check_ids": summary.current_write_check_ids,
        "stale_write_check_ids": summary.stale_write_check_ids,
        "pending_user_judgment_count": summary.pending_user_judgment_count,
        "active_blocker_count": summary.active_blocker_count,
        "unresolved_unrecorded_change_count": summary.unresolved_unrecorded_change_count
    })
}

fn tool_observation_json(observation: &ToolObservation) -> Value {
    json!({
        "tool_name": observation.tool_name,
        "command": observation.command,
        "classification": observation.classification.as_str(),
        "paths": path_assessments_json(&observation.paths),
        "changed_paths": path_assessments_json(&observation.changed_paths),
        "explicit_write_attempt": observation.explicit_write_attempt,
        "exit_code": observation.exit_code,
        "success": observation.success,
        "status": observation.status
    })
}

fn path_assessments_json(paths: &[PathAssessment]) -> Vec<Value> {
    paths
        .iter()
        .map(|path| {
            json!({
                "raw": path.raw,
                "normalized": path.normalized,
                "inside_repo": path.inside_repo
            })
        })
        .collect()
}

fn reasons_json(reasons: &[GuardReason]) -> Vec<Value> {
    reasons
        .iter()
        .map(|reason| {
            json!({
                "code": reason.code,
                "message": reason.message,
                "severity": reason.severity
            })
        })
        .collect()
}

fn object_text(value: Value) -> Result<String, GuardCommandError> {
    match value {
        Value::Object(_) => serde_json::to_string(&value).map_err(json_error),
        other => serde_json::to_string(&json!({ "value": other })).map_err(json_error),
    }
}

fn value_at<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut cursor = value;
    for key in path {
        cursor = cursor.get(*key)?;
    }
    Some(cursor)
}

fn event_string(value: &Value, paths: &[&[&str]]) -> Option<String> {
    for path in paths {
        if let Some(text) = value_at(value, path).and_then(Value::as_str) {
            if !text.trim().is_empty() {
                return Some(text.to_owned());
            }
        }
    }
    None
}

fn event_bool(value: &Value, paths: &[&[&str]]) -> Option<bool> {
    paths
        .iter()
        .find_map(|path| value_at(value, path).and_then(Value::as_bool))
}

fn event_i64(value: &Value, paths: &[&[&str]]) -> Option<i64> {
    paths
        .iter()
        .find_map(|path| value_at(value, path).and_then(Value::as_i64))
}

fn extract_prompt_text(value: &Value) -> Option<String> {
    event_string(
        value,
        &[
            &["prompt"],
            &["user_prompt"],
            &["message"],
            &["input", "prompt"],
            &["input", "message"],
            &["event", "prompt"],
        ],
    )
}

fn redact_event_value(value: &Value) -> Value {
    match value {
        Value::Object(object) => Value::Object(
            object
                .iter()
                .map(|(key, value)| {
                    if prompt_like_key(key) {
                        (key.clone(), redacted_prompt_value(value))
                    } else {
                        (key.clone(), redact_event_value(value))
                    }
                })
                .collect::<Map<_, _>>(),
        ),
        Value::Array(values) => Value::Array(values.iter().map(redact_event_value).collect()),
        other => other.clone(),
    }
}

fn prompt_like_key(key: &str) -> bool {
    matches!(
        key,
        "prompt" | "user_prompt" | "message" | "messages" | "content" | "transcript"
    )
}

fn redacted_prompt_value(value: &Value) -> Value {
    match value {
        Value::String(text) => json!({
            "omitted": true,
            "sha256": sha256_text(text),
            "size_bytes": text.len()
        }),
        Value::Array(values) => json!({
            "omitted": true,
            "sha256": sha256_text(&value.to_string()),
            "item_count": values.len()
        }),
        _ => json!({
            "omitted": true,
            "sha256": sha256_text(&value.to_string())
        }),
    }
}

fn current_timestamp() -> String {
    DateTime::<Utc>::from(SystemTime::now()).to_rfc3339_opts(SecondsFormat::Secs, true)
}

fn event_time_or_now(raw: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(raw)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .unwrap_or_else(|_| DateTime::<Utc>::from(SystemTime::now()))
}

fn sha256_text(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("sha256:{}", hex_bytes(&hasher.finalize()))
}

fn stable_id(prefix: &str, parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0]);
    }
    let digest = hex_bytes(&hasher.finalize());
    format!("{prefix}_{}", &digest[..16])
}

fn hex_bytes(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn json_error(error: serde_json::Error) -> GuardCommandError {
    GuardCommandError::Runtime(format!("failed to serialize guard output: {error}"))
}
