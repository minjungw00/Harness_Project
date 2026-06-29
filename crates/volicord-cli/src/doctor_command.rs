use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::Serialize;
use serde_json::{json, Value};
use volicord_store::{
    agent_connections::{CONNECTION_MODE_READ_ONLY, CONNECTION_MODE_WORKFLOW},
    inspection::{
        inspect_runtime_home, DatabaseInspection, InspectionSchemaState,
        InstallationProfileInspectionRecord, RegistryInspectionSnapshot,
    },
    runtime_home::{resolve_runtime_home, RuntimeHomeResolutionError},
};

use crate::{
    setup_command::{path_text, CommandOutcome, CommandStatus},
    shell_path::{
        is_executable_file, mcp_binary_name, path_directory_is_on_path, volicord_binary_name,
        PATH_ENV,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoctorCommandError {
    Usage(String),
    Runtime(String),
}

impl std::fmt::Display for DoctorCommandError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Usage(message) | Self::Runtime(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for DoctorCommandError {}

impl From<RuntimeHomeResolutionError> for DoctorCommandError {
    fn from(error: RuntimeHomeResolutionError) -> Self {
        Self::Runtime(error.to_string())
    }
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

    fn skipped(id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: "skipped".to_owned(),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct DiagnosticAction {
    id: String,
    instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<String>,
}

pub fn doctor_usage() -> String {
    "volicord doctor [--json]\n".to_owned()
}

pub fn run_doctor_command<F>(
    args: &[String],
    env_var: F,
    current_dir: &Path,
) -> Result<CommandOutcome, DoctorCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    if is_help_request(args) {
        return Ok(CommandOutcome {
            status: CommandStatus::Complete,
            output: doctor_usage(),
        });
    }
    let output = parse_doctor_options(args)?;
    let runtime_home = resolve_runtime_home(&env_var, current_dir)?;
    let mut checks = Vec::new();
    let mut actions = Vec::new();

    inspect_runtime_home_path(&runtime_home, &mut checks, &mut actions);
    let inspection = inspect_runtime_home(&runtime_home);
    let mut profile = None;
    let mut project_count = None;
    let mut connection_count = None;

    match &inspection.registry {
        DatabaseInspection::Missing { path } => {
            checks.push(
                DiagnosticCheck::failed("registry", "Runtime Home registry is missing")
                    .with_details(json!({ "path": path_text(path) })),
            );
            actions.push(run_setup_action());
        }
        DatabaseInspection::Present(snapshot) => {
            inspect_registry_snapshot(snapshot, &mut checks);
            profile = snapshot.installation_profile.as_ref();
            project_count = Some(snapshot.projects.len());
            connection_count = Some(snapshot.agent_connections.len());
        }
        DatabaseInspection::Unsupported {
            path,
            detected_version,
            latest_supported_version,
            detail,
        } => {
            checks.push(
                DiagnosticCheck::failed(
                    "registry",
                    "Runtime Home registry uses an unsupported schema",
                )
                .with_details(json!({
                    "path": path_text(path),
                    "detected_version": detected_version,
                    "latest_supported_version": latest_supported_version,
                    "detail": detail,
                })),
            );
        }
        DatabaseInspection::Malformed { path, detail } => {
            checks.push(
                DiagnosticCheck::failed("registry", "Runtime Home registry is malformed")
                    .with_details(json!({ "path": path_text(path), "detail": detail })),
            );
        }
        DatabaseInspection::Unreadable { path, detail } => {
            checks.push(
                DiagnosticCheck::failed("registry", "Runtime Home registry is unreadable")
                    .with_details(json!({ "path": path_text(path), "detail": detail })),
            );
        }
    }

    if let Some(profile) = profile {
        inspect_installation_profile(profile, &env_var, &mut checks, &mut actions);
    } else {
        checks.push(
            DiagnosticCheck::failed("installation_profile", "installation profile is missing")
                .with_details(json!({ "runtime_home": path_text(&runtime_home) })),
        );
        if !actions.iter().any(|action| action.id == "run_setup") {
            actions.push(run_setup_action());
        }
        checks.push(DiagnosticCheck::skipped(
            "volicord_command",
            "volicord command check needs an installation profile",
        ));
        checks.push(DiagnosticCheck::skipped(
            "volicord_mcp_command",
            "volicord-mcp command check needs an installation profile",
        ));
        checks.push(DiagnosticCheck::skipped(
            "path_or_shim",
            "PATH and shim check needs an installation profile",
        ));
    }

    checks.push(
        DiagnosticCheck::skipped(
            "host_detection",
            "supported host detection is reported by connection verification after setup",
        )
        .with_details(json!({ "supported_hosts": ["codex", "claude_code"] })),
    );
    if let (Some(projects), Some(connections)) = (project_count, connection_count) {
        checks.push(
            DiagnosticCheck::passed("registry_counts", "registry records are readable")
                .with_details(json!({
                    "projects": projects,
                    "connections": connections,
                })),
        );
    } else {
        checks.push(DiagnosticCheck::skipped(
            "registry_counts",
            "project and connection counts are unavailable until the registry is readable",
        ));
    }

    let status = doctor_status(&checks);
    Ok(CommandOutcome {
        status,
        output: render_doctor_output(output, status, &runtime_home, &checks, &actions)?,
    })
}

fn parse_doctor_options(args: &[String]) -> Result<OutputFormat, DoctorCommandError> {
    let mut output = OutputFormat::Text;
    for token in args {
        match token.as_str() {
            "-h" | "--help" | "help" => return Err(DoctorCommandError::Usage(doctor_usage())),
            "--json" => output = OutputFormat::Json,
            option if option.starts_with("--json=") => {
                return Err(DoctorCommandError::Usage(
                    "--json does not accept a value".to_owned(),
                ))
            }
            option if option.starts_with('-') => {
                return Err(DoctorCommandError::Usage(format!(
                    "unknown option: {option}"
                )))
            }
            argument => {
                return Err(DoctorCommandError::Usage(format!(
                    "unexpected argument: {argument}"
                )))
            }
        }
    }
    Ok(output)
}

fn inspect_runtime_home_path(
    runtime_home: &Path,
    checks: &mut Vec<DiagnosticCheck>,
    actions: &mut Vec<DiagnosticAction>,
) {
    match fs::metadata(runtime_home) {
        Ok(metadata) if metadata.is_dir() => checks.push(
            DiagnosticCheck::passed(
                "runtime_home_access",
                "Runtime Home directory is accessible",
            )
            .with_details(json!({ "path": path_text(runtime_home) })),
        ),
        Ok(_) => {
            checks.push(
                DiagnosticCheck::failed(
                    "runtime_home_access",
                    "Runtime Home path is not a directory",
                )
                .with_details(json!({ "path": path_text(runtime_home) })),
            );
            actions.push(run_setup_action());
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            checks.push(
                DiagnosticCheck::failed("runtime_home_access", "Runtime Home directory is missing")
                    .with_details(json!({ "path": path_text(runtime_home) })),
            );
            actions.push(run_setup_action());
        }
        Err(error) => {
            checks.push(
                DiagnosticCheck::failed(
                    "runtime_home_access",
                    "Runtime Home directory is not accessible",
                )
                .with_details(
                    json!({ "path": path_text(runtime_home), "detail": error.to_string() }),
                ),
            );
        }
    }
}

fn inspect_registry_snapshot(
    snapshot: &RegistryInspectionSnapshot,
    checks: &mut Vec<DiagnosticCheck>,
) {
    match snapshot.schema {
        InspectionSchemaState::Current { version } => checks.push(
            DiagnosticCheck::passed("registry_schema", "Runtime Home registry schema is current")
                .with_details(json!({
                    "path": path_text(&snapshot.path),
                    "version": version,
                    "storage_profile": snapshot.runtime_home.storage_profile,
                })),
        ),
        InspectionSchemaState::MigrationRequired {
            detected_version,
            latest_supported_version,
        } => checks.push(
            DiagnosticCheck::failed("registry_schema", "Runtime Home registry needs migration")
                .with_details(json!({
                    "path": path_text(&snapshot.path),
                    "detected_version": detected_version,
                    "latest_supported_version": latest_supported_version,
                })),
        ),
    }
}

fn inspect_installation_profile<F>(
    profile: &InstallationProfileInspectionRecord,
    env_var: &F,
    checks: &mut Vec<DiagnosticCheck>,
    actions: &mut Vec<DiagnosticAction>,
) where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let mode_supported = matches!(
        profile.default_connection_mode.as_str(),
        CONNECTION_MODE_WORKFLOW | CONNECTION_MODE_READ_ONLY
    );
    if mode_supported {
        checks.push(
            DiagnosticCheck::passed("installation_profile", "installation profile is present")
                .with_details(json!({
                    "installation_id": profile.installation_id,
                    "default_connection_mode": profile.default_connection_mode,
                    "bin_dir": path_text(&profile.bin_dir),
                })),
        );
    } else {
        checks.push(
            DiagnosticCheck::failed(
                "installation_profile",
                "installation profile has an unsupported default connection mode",
            )
            .with_details(json!({
                "installation_id": profile.installation_id,
                "default_connection_mode": profile.default_connection_mode,
            })),
        );
        actions.push(run_setup_action());
    }
    inspect_command_path(
        "volicord_command",
        "volicord command",
        &PathBuf::from(&profile.volicord_command),
        checks,
        actions,
    );
    inspect_command_path(
        "volicord_mcp_command",
        "volicord-mcp command",
        &PathBuf::from(&profile.volicord_mcp_command),
        checks,
        actions,
    );
    inspect_path_or_shim(profile, env_var, checks, actions);
}

fn inspect_command_path(
    id: &str,
    label: &str,
    command: &Path,
    checks: &mut Vec<DiagnosticCheck>,
    actions: &mut Vec<DiagnosticAction>,
) {
    if is_executable_file(command) {
        checks.push(
            DiagnosticCheck::passed(id, format!("{label} is executable"))
                .with_details(json!({ "path": path_text(command) })),
        );
    } else {
        checks.push(
            DiagnosticCheck::failed(id, format!("{label} is missing or not executable"))
                .with_details(json!({ "path": path_text(command) })),
        );
        actions.push(DiagnosticAction {
            id: format!("repair_{id}"),
            instruction:
                "Run volicord setup --mcp-command PATH again after selecting executable Volicord commands."
                    .to_owned(),
            command: Some("volicord setup --mcp-command PATH".to_owned()),
        });
    }
}

fn inspect_path_or_shim<F>(
    profile: &InstallationProfileInspectionRecord,
    env_var: &F,
    checks: &mut Vec<DiagnosticCheck>,
    actions: &mut Vec<DiagnosticAction>,
) where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let path_env = env_var(PATH_ENV);
    let bin_dir_on_path = path_directory_is_on_path(path_env.as_deref(), &profile.bin_dir);
    let volicord_link = profile.bin_dir.join(volicord_binary_name());
    let mcp_link = profile.bin_dir.join(mcp_binary_name());
    let link_ready = is_executable_file(&volicord_link) && is_executable_file(&mcp_link);

    if bin_dir_on_path && link_ready {
        checks.push(
            DiagnosticCheck::passed(
                "path_or_shim",
                "profile command directory is on PATH and contains command links",
            )
            .with_details(json!({
                "bin_dir": path_text(&profile.bin_dir),
                "volicord": path_text(&volicord_link),
                "volicord_mcp": path_text(&mcp_link),
            })),
        );
    } else if bin_dir_on_path {
        checks.push(
            DiagnosticCheck::warning(
                "path_or_shim",
                "profile command directory is on PATH, but command links are incomplete",
            )
            .with_details(json!({
                "bin_dir": path_text(&profile.bin_dir),
                "volicord_link_ready": is_executable_file(&volicord_link),
                "volicord_mcp_link_ready": is_executable_file(&mcp_link),
            })),
        );
    } else if link_ready {
        checks.push(
            DiagnosticCheck::warning(
                "path_or_shim",
                "command links exist, but the link directory is not on PATH",
            )
            .with_details(json!({ "bin_dir": path_text(&profile.bin_dir) })),
        );
        actions.push(DiagnosticAction {
            id: "add_link_bin_to_path".to_owned(),
            instruction: format!(
                "Add {} to PATH before starting new shells or MCP hosts.",
                profile.bin_dir.display()
            ),
            command: Some(format!(
                "export PATH=\"{}:$PATH\"",
                profile.bin_dir.display()
            )),
        });
    } else {
        checks.push(
            DiagnosticCheck::warning(
                "path_or_shim",
                "no command link directory is active for this shell",
            )
            .with_details(json!({ "bin_dir": path_text(&profile.bin_dir) })),
        );
    }
}

fn doctor_status(checks: &[DiagnosticCheck]) -> CommandStatus {
    if checks.iter().any(|check| {
        check.status == "failed"
            && !matches!(
                check.id.as_str(),
                "runtime_home_access" | "registry" | "installation_profile"
            )
    }) {
        CommandStatus::Failed
    } else if checks.iter().any(|check| check.status == "failed") {
        CommandStatus::ActionRequired
    } else {
        CommandStatus::Complete
    }
}

fn render_doctor_output(
    output: OutputFormat,
    status: CommandStatus,
    runtime_home: &Path,
    checks: &[DiagnosticCheck],
    actions: &[DiagnosticAction],
) -> Result<String, DoctorCommandError> {
    match output {
        OutputFormat::Json => serde_json::to_string_pretty(&json!({
            "status": status.as_str(),
            "runtime_home": path_text(runtime_home),
            "checks": checks,
            "actions": actions,
        }))
        .map(|text| format!("{text}\n"))
        .map_err(|error| DoctorCommandError::Runtime(error.to_string())),
        OutputFormat::Text => {
            let mut text = format!(
                "Volicord doctor {}\nruntime_home: {}\n",
                status.as_str(),
                runtime_home.display()
            );
            text.push_str("checks:\n");
            for check in checks {
                text.push_str(&format!(
                    "- {}: {} ({})\n",
                    check.id, check.summary, check.status
                ));
            }
            if !actions.is_empty() {
                text.push_str("actions:\n");
                for action in actions {
                    text.push_str(&format!("- {}\n", action.instruction));
                }
            }
            Ok(text)
        }
    }
}

fn run_setup_action() -> DiagnosticAction {
    DiagnosticAction {
        id: "run_setup".to_owned(),
        instruction:
            "Run volicord setup before project, connection, export, MCP, or user workflows."
                .to_owned(),
        command: Some("volicord setup".to_owned()),
    }
}

fn is_help_request(args: &[String]) -> bool {
    matches!(
        args.first().map(String::as_str),
        Some("-h" | "--help" | "help")
    )
}
