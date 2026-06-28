use std::{
    collections::BTreeMap,
    env, fs, io,
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

const INSTALLATION_ID: &str = "default";
const PATH_ENV: &str = "PATH";
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct DiagnosticAction {
    id: String,
    instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<String>,
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

    let volicord_command = discover_volicord_command(process)?;
    let volicord_mcp = discover_mcp_command(&parsed, process)?;
    let bin_dir = parsed
        .link_bin
        .clone()
        .unwrap_or_else(|| command_parent(&volicord_command.path));

    let mut checks = vec![
        DiagnosticCheck::passed("runtime_home", "Runtime Home registry is ready").with_details(
            json!({
                "runtime_home": path_text(&runtime_home_record.runtime_home),
                "registry_db": path_text(&runtime_home_record.registry_db_path),
                "runtime_home_id": runtime_home_record.runtime_home_id,
            }),
        ),
        DiagnosticCheck::passed("volicord_command", "volicord command was discovered")
            .with_details(json!({
                "path": path_text(&volicord_command.path),
                "source": volicord_command.source,
            })),
        DiagnosticCheck::passed(
            "volicord_mcp_command",
            "volicord-mcp command was discovered",
        )
        .with_details(json!({
            "path": path_text(&volicord_mcp.path),
            "source": volicord_mcp.source,
        })),
    ];
    let mut actions = Vec::new();
    let mut link_results = BTreeMap::new();

    if let Some(link_bin) = &parsed.link_bin {
        let link_bin = absolute_path(current_dir, link_bin.clone());
        fs::create_dir_all(&link_bin)?;
        let volicord_link =
            install_command_link(&link_bin, &volicord_binary_name(), &volicord_command.path);
        let mcp_link = install_command_link(&link_bin, &mcp_binary_name(), &volicord_mcp.path);
        push_link_check(
            "link_volicord",
            "volicord command link",
            &link_bin,
            &volicord_binary_name(),
            &volicord_link,
            &mut checks,
            &mut actions,
        );
        push_link_check(
            "link_volicord_mcp",
            "volicord-mcp command link",
            &link_bin,
            &mcp_binary_name(),
            &mcp_link,
            &mut checks,
            &mut actions,
        );
        link_results.insert("volicord".to_owned(), link_volicord_status(&volicord_link));
        link_results.insert("volicord_mcp".to_owned(), link_volicord_status(&mcp_link));
        if !path_contains_dir(process.env_var(PATH_ENV).as_deref(), &link_bin) {
            actions.push(DiagnosticAction {
                id: "add_link_bin_to_path".to_owned(),
                instruction: format!(
                    "Add {} to PATH before starting new shells or MCP hosts.",
                    link_bin.display()
                ),
                command: Some(format!("export PATH=\"{}:$PATH\"", link_bin.display())),
            });
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

    let status = if checks.iter().any(|check| check.status == "failed") {
        CommandStatus::ActionRequired
    } else {
        CommandStatus::Complete
    };
    Ok(CommandOutcome {
        status,
        output: render_setup_output(
            parsed.output,
            status,
            &runtime_home_record,
            &profile,
            &checks,
            &actions,
        )?,
    })
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

    if let Some(path) = process.env_var(PATH_ENV) {
        for dir in env::split_paths(&path) {
            let candidate = dir.join(mcp_binary_name());
            if is_executable_file(&candidate) {
                return Ok(DiscoveredCommand {
                    path: canonical_existing_executable(&candidate, "volicord-mcp from PATH")?,
                    source: "path",
                });
            }
        }
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

pub(crate) fn is_executable_file(path: &Path) -> bool {
    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }
    is_executable_metadata(&metadata)
}

#[cfg(unix)]
fn is_executable_metadata(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;

    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable_metadata(_metadata: &fs::Metadata) -> bool {
    true
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

fn push_link_check(
    check_id: &str,
    label: &str,
    link_bin: &Path,
    name: &str,
    result: &LinkInstallResult,
    checks: &mut Vec<DiagnosticCheck>,
    actions: &mut Vec<DiagnosticAction>,
) {
    match result {
        LinkInstallResult::Created(path) => checks.push(
            DiagnosticCheck::passed(check_id, format!("{label} was created"))
                .with_details(json!({ "path": path_text(path) })),
        ),
        LinkInstallResult::Existing(path) => checks.push(
            DiagnosticCheck::passed(
                check_id,
                format!("{label} already points to the selected executable"),
            )
            .with_details(json!({ "path": path_text(path) })),
        ),
        LinkInstallResult::Unsupported(path) => {
            checks.push(
                DiagnosticCheck::warning(
                    check_id,
                    format!("{label} was not created on this platform"),
                )
                .with_details(json!({ "path": path_text(path) })),
            );
            actions.push(DiagnosticAction {
                id: format!("create_{name}_shim"),
                instruction: format!(
                    "Create a command shim for {name} under {} if your shell cannot find it.",
                    link_bin.display()
                ),
                command: None,
            });
        }
        LinkInstallResult::UnsafeExisting(path) => {
            checks.push(
                DiagnosticCheck::failed(
                    check_id,
                    format!(
                        "{label} was not replaced because an existing path is not Volicord-managed"
                    ),
                )
                .with_details(json!({ "path": path_text(path) })),
            );
            actions.push(DiagnosticAction {
                id: format!("repair_{name}_link"),
                instruction: format!(
                    "Move or remove the existing {} path, then rerun volicord setup --link-bin {}.",
                    path.display(),
                    link_bin.display()
                ),
                command: None,
            });
        }
        LinkInstallResult::Failed { path, detail } => {
            checks.push(
                DiagnosticCheck::failed(check_id, format!("{label} could not be created"))
                    .with_details(json!({ "path": path_text(path), "detail": detail })),
            );
            actions.push(DiagnosticAction {
                id: format!("repair_{name}_link"),
                instruction: format!(
                    "Fix write access for {}, then rerun volicord setup --link-bin {}.",
                    path.display(),
                    link_bin.display()
                ),
                command: None,
            });
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
    status: CommandStatus,
    runtime_home: &RuntimeHomeRecord,
    profile: &InstallationProfileRecord,
    checks: &[DiagnosticCheck],
    actions: &[DiagnosticAction],
) -> Result<String, SetupCommandError> {
    match output {
        OutputFormat::Json => serde_json::to_string_pretty(&json!({
            "status": status.as_str(),
            "runtime_home": path_text(&runtime_home.runtime_home),
            "registry_db": path_text(&runtime_home.registry_db_path),
            "installation_profile": profile_json(profile),
            "checks": checks,
            "actions": actions,
        }))
        .map(|text| format!("{text}\n"))
        .map_err(|error| SetupCommandError::Runtime(error.to_string())),
        OutputFormat::Text => {
            let mut text = format!(
                "Volicord setup {}\nruntime_home: {}\nregistry_db: {}\nvolicord_command: {}\nvolicord_mcp_command: {}\ndefault_connection_mode: {}\n",
                status.as_str(),
                runtime_home.runtime_home.display(),
                runtime_home.registry_db_path.display(),
                profile.volicord_command,
                profile.volicord_mcp_command,
                profile.default_connection_mode
            );
            let failed = checks
                .iter()
                .filter(|check| check.status == "failed")
                .collect::<Vec<_>>();
            if !failed.is_empty() {
                text.push_str("checks:\n");
                for check in failed {
                    text.push_str(&format!("- {}: {}\n", check.id, check.summary));
                }
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

pub(crate) fn path_contains_dir(path_env: Option<&std::ffi::OsStr>, dir: &Path) -> bool {
    path_env
        .map(env::split_paths)
        .into_iter()
        .flatten()
        .any(|candidate| paths_equivalent(&candidate, dir))
}

fn paths_equivalent(left: &Path, right: &Path) -> bool {
    if left == right {
        return true;
    }
    match (fs::canonicalize(left), fs::canonicalize(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

pub(crate) fn volicord_binary_name() -> String {
    format!("volicord{}", env::consts::EXE_SUFFIX)
}

pub(crate) fn mcp_binary_name() -> String {
    format!("volicord-mcp{}", env::consts::EXE_SUFFIX)
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsString, io::Write};

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

        assert_eq!(outcome.status, CommandStatus::Complete);
        let value: Value = serde_json::from_str(&outcome.output)?;
        assert_eq!(value["status"], "complete");
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
            ],
            fixture.path(),
            &process,
        )?;

        assert_eq!(outcome.status, CommandStatus::Complete);
        assert_eq!(
            fs::canonicalize(link_bin.join(volicord_binary_name()))?,
            volicord
        );
        assert_eq!(fs::canonicalize(link_bin.join(mcp_binary_name()))?, mcp);
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
