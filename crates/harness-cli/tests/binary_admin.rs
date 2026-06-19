#![forbid(unsafe_code)]

use std::{
    error::Error,
    fs,
    path::Path,
    process::{Command, Output},
};

use harness_store::{
    bootstrap::{list_projects, list_surfaces},
    sqlite::{project_state_db_path, registry_db_path},
};
use harness_test_support::TempRuntimeHome;
use serde_json::{json, Value};

const PROJECT_ID: &str = "project_binary_admin";
const AGENT_SURFACE_ID: &str = "surface_binary_agent";
const AGENT_INSTANCE_ID: &str = "surface_instance_binary_agent";
const USER_SURFACE_ID: &str = "surface_binary_user";
const USER_INSTANCE_ID: &str = "surface_instance_binary_user";

#[test]
fn harness_binary_runs_administrative_initialization_and_registration() -> Result<(), Box<dyn Error>>
{
    let runtime_home = TempRuntimeHome::new("cli-bin-admin")?;
    let repo_root = runtime_home.path().join("product-repo");
    fs::create_dir_all(&repo_root)?;
    let repo_root_text = path_text(&repo_root);

    let help = run_without_home(["--help"])?;
    assert_success(&help);
    assert!(stdout(&help).contains("harness init"));

    let version = run_without_home(["--version"])?;
    assert_success(&version);
    assert!(stdout(&version).starts_with("harness "));

    let init = run_with_home(
        runtime_home.path(),
        ["init", "--runtime-home-id", "runtime_home_binary_admin"],
    )?;
    assert_success(&init);
    assert!(stdout(&init).contains("runtime_home initialized"));

    let project = run_with_home(
        runtime_home.path(),
        [
            "project",
            "register",
            "--project-id",
            PROJECT_ID,
            "--repo-root",
            repo_root_text.as_str(),
        ],
    )?;
    assert_success(&project);
    assert!(stdout(&project).contains("project registered"));

    let projects = run_with_home(runtime_home.path(), ["project", "list"])?;
    assert_success(&projects);
    assert!(stdout(&projects).contains(PROJECT_ID));

    let agent_surface = run_with_home(
        runtime_home.path(),
        [
            "surface",
            "register",
            "--project-id",
            PROJECT_ID,
            "--surface-id",
            AGENT_SURFACE_ID,
            "--surface-instance-id",
            AGENT_INSTANCE_ID,
            "--kind",
            "mcp",
            "--interaction-role",
            "agent",
            "--profile",
            "baseline-workflow",
        ],
    )?;
    assert_success(&agent_surface);
    assert!(stdout(&agent_surface).contains(AGENT_INSTANCE_ID));

    let user_surface = run_with_home(
        runtime_home.path(),
        [
            "surface",
            "register",
            "--project-id",
            PROJECT_ID,
            "--surface-id",
            USER_SURFACE_ID,
            "--surface-instance-id",
            USER_INSTANCE_ID,
            "--kind",
            "mcp",
            "--interaction-role",
            "user_interaction",
            "--access-class",
            "read_status",
            "--access-class",
            "core_mutation",
        ],
    )?;
    assert_success(&user_surface);
    assert!(stdout(&user_surface).contains(USER_INSTANCE_ID));

    let surfaces = run_with_home(
        runtime_home.path(),
        ["surface", "list", "--project-id", PROJECT_ID],
    )?;
    assert_success(&surfaces);
    assert!(stdout(&surfaces).contains(AGENT_SURFACE_ID));
    assert!(stdout(&surfaces).contains(USER_SURFACE_ID));

    assert!(registry_db_path(runtime_home.path()).exists());
    assert!(project_state_db_path(runtime_home.path(), PROJECT_ID).exists());

    let project_records = list_projects(runtime_home.path())?;
    assert_eq!(project_records.len(), 1);
    assert_eq!(project_records[0].project_id, PROJECT_ID);
    assert_eq!(project_records[0].repo_root, fs::canonicalize(&repo_root)?);
    assert_eq!(project_records[0].status, "active");

    let surface_records = list_surfaces(runtime_home.path(), PROJECT_ID)?;
    assert_eq!(surface_records.len(), 2);

    let agent = surface_records
        .iter()
        .find(|surface| surface.surface_instance_id == AGENT_INSTANCE_ID)
        .expect("agent surface should be registered");
    assert_eq!(agent.surface_id, AGENT_SURFACE_ID);
    assert_eq!(agent.surface_kind, "mcp");
    assert_eq!(agent.interaction_role, "agent");
    assert_eq!(
        access_classes(&agent.local_access_json)?,
        json!([
            "read_status",
            "core_mutation",
            "write_authorization",
            "artifact_registration",
            "run_recording"
        ])
    );

    let user = surface_records
        .iter()
        .find(|surface| surface.surface_instance_id == USER_INSTANCE_ID)
        .expect("user-interaction surface should be registered");
    assert_eq!(user.surface_id, USER_SURFACE_ID);
    assert_eq!(user.surface_kind, "mcp");
    assert_eq!(user.interaction_role, "user_interaction");
    assert_eq!(
        access_classes(&user.local_access_json)?,
        json!(["read_status", "core_mutation"])
    );

    let invalid = run_without_home(["init", "--not-a-real-option", "value"])?;
    assert_eq!(invalid.status.code(), Some(2));
    assert!(stderr(&invalid).contains("unknown option"));

    let blocked_runtime_home = runtime_home.path().join("runtime-home-file");
    fs::write(&blocked_runtime_home, "not a directory")?;
    let runtime_failure = run_with_home(&blocked_runtime_home, ["init"])?;
    assert_eq!(runtime_failure.status.code(), Some(1));
    assert!(stderr(&runtime_failure).starts_with("error:"));

    Ok(())
}

fn run_without_home<const N: usize>(args: [&str; N]) -> Result<Output, Box<dyn Error>> {
    let mut command = base_command();
    command.args(args);
    Ok(command.output()?)
}

fn run_with_home<const N: usize>(
    runtime_home: &Path,
    args: [&str; N],
) -> Result<Output, Box<dyn Error>> {
    let mut command = base_command();
    command.env("HARNESS_HOME", runtime_home);
    command.args(args);
    Ok(command.output()?)
}

fn base_command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_harness"));
    command.env_clear();
    command.current_dir(env!("CARGO_MANIFEST_DIR"));
    command
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "expected success, got status {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        stdout(output),
        stderr(output)
    );
}

fn access_classes(local_access_json: &str) -> Result<Value, Box<dyn Error>> {
    let value: Value = serde_json::from_str(local_access_json)?;
    Ok(value["authorized_access_classes"].clone())
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn path_text(path: &Path) -> String {
    path.display().to_string()
}
