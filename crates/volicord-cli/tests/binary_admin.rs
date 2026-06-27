#![forbid(unsafe_code)]

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use serde_json::Value;
use volicord_store::agent_connections::{
    agent_connection_record, list_connection_projects, CONNECTION_MODE_READ_ONLY,
    CONNECTION_MODE_WORKFLOW,
};
use volicord_test_support::TempRuntimeHome;

#[test]
fn binary_help_uses_agent_connection_model() -> Result<(), Box<dyn Error>> {
    let help = run_without_home(["--help"])?;
    assert_success(&help);
    let text = stdout(&help);

    assert!(text.contains("volicord agent connect"));
    assert!(text.contains("--connection-id ID"));
    let removed_command = ["sur", "face"].concat();
    assert!(!text.contains(&format!("volicord {removed_command}")));
    assert!(!text.contains(&format!("--{}-id", "integration")));
    assert!(!text.contains("Agent Integration"));

    let removed = run_without_home([
        removed_command.as_str(),
        "list",
        "--project-id",
        "project_a",
    ])?;
    assert_eq!(removed.status.code(), Some(2));
    assert!(stderr(&removed).contains(&format!("unknown command: {removed_command}")));
    Ok(())
}

#[cfg(unix)]
#[test]
fn agent_connect_defaults_to_read_only_and_writes_connection_config() -> Result<(), Box<dyn Error>>
{
    let runtime_home = TempRuntimeHome::new("cli-bin-connection-read-only")?;
    let repo_root = runtime_home.create_product_repo("product-repo")?;
    let bin_dir = runtime_home.path().join("bin");
    write_fake_codex(&bin_dir)?;
    write_fake_mcp(&bin_dir)?;

    let output = run_with_home_env(
        runtime_home.path(),
        [
            "agent",
            "connect",
            "--host",
            "codex",
            "--scope",
            "project",
            "--project-id",
            "project_read_only",
            "--repo-root",
            path_text(&repo_root).as_str(),
            "--allow-repository-write",
            "--output",
            "json",
        ],
        &[("PATH", path_env(&[bin_dir.as_path()]))],
    )?;
    assert_success(&output);
    let value = json_stdout(&output)?;
    let connection = &value["connection"];
    let connection_id = connection["connection_id"]
        .as_str()
        .expect("connection_id should be present");

    assert_eq!(connection["mode"], CONNECTION_MODE_READ_ONLY);
    assert_eq!(connection["host_kind"], "codex");
    assert_eq!(connection["host_scope"], "project");
    assert_eq!(value["status"], "action_required");

    let record = agent_connection_record(runtime_home.path(), connection_id)?
        .expect("connection should be stored");
    assert_eq!(record.mode, CONNECTION_MODE_READ_ONLY);
    let projects = list_connection_projects(runtime_home.path(), connection_id)?;
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].project_id, "project_read_only");

    let config = fs::read_to_string(repo_root.join(".codex").join("config.toml"))?;
    assert!(config.contains(&format!("args = [\"--connection\", \"{connection_id}\"]")));
    assert!(!config.contains(&format!("--{}", "integration")));
    Ok(())
}

#[cfg(unix)]
#[test]
fn agent_connect_uses_explicit_workflow_mode() -> Result<(), Box<dyn Error>> {
    let runtime_home = TempRuntimeHome::new("cli-bin-connection-workflow")?;
    let repo_root = runtime_home.create_product_repo("product-repo")?;
    let bin_dir = runtime_home.path().join("bin");
    write_fake_codex(&bin_dir)?;
    write_fake_mcp(&bin_dir)?;

    let output = run_with_home_env(
        runtime_home.path(),
        [
            "agent",
            "connect",
            "--host",
            "codex",
            "--scope",
            "project",
            "--project-id",
            "project_workflow",
            "--repo-root",
            path_text(&repo_root).as_str(),
            "--mode",
            "workflow",
            "--server-name",
            "volicord-workflow",
            "--allow-repository-write",
            "--output",
            "json",
        ],
        &[
            ("PATH", path_env(&[bin_dir.as_path()])),
            ("VOLICORD_TEST_CONNECTION_MODE", "workflow".to_owned()),
        ],
    )?;
    assert_success(&output);
    let value = json_stdout(&output)?;
    let connection_id = value["connection"]["connection_id"]
        .as_str()
        .expect("connection_id should be present");

    assert_eq!(value["connection"]["mode"], CONNECTION_MODE_WORKFLOW);
    let record = agent_connection_record(runtime_home.path(), connection_id)?
        .expect("connection should be stored");
    assert_eq!(record.mode, CONNECTION_MODE_WORKFLOW);
    Ok(())
}

#[cfg(unix)]
#[test]
fn connection_project_enable_disable_and_uninstall_flow() -> Result<(), Box<dyn Error>> {
    let runtime_home = TempRuntimeHome::new("cli-bin-connection-lifecycle")?;
    let repo_a = runtime_home.create_product_repo("product-a")?;
    let repo_b = runtime_home.create_product_repo("product-b")?;
    let bin_dir = runtime_home.path().join("bin");
    let codex_home = runtime_home.path().join("codex-home");
    let mcp = write_fake_mcp(&bin_dir)?;
    write_fake_codex(&bin_dir)?;

    let connect = run_with_home_env(
        runtime_home.path(),
        [
            "agent",
            "connect",
            "--host",
            "codex",
            "--scope",
            "user",
            "--project-id",
            "project_a",
            "--repo-root",
            path_text(&repo_a).as_str(),
            "--mcp-command",
            path_text(&mcp).as_str(),
            "--output",
            "json",
        ],
        &[
            ("PATH", path_env(&[bin_dir.as_path()])),
            ("CODEX_HOME", path_text(&codex_home)),
        ],
    )?;
    assert_success(&connect);
    let connect_json = json_stdout(&connect)?;
    let connection_id = connect_json["connection"]["connection_id"]
        .as_str()
        .expect("connection_id should be present")
        .to_owned();
    assert_eq!(connect_json["status"], "complete");

    let add = run_with_home_env(
        runtime_home.path(),
        [
            "agent",
            "project",
            "add",
            "--connection-id",
            connection_id.as_str(),
            "--project-id",
            "project_b",
            "--repo-root",
            path_text(&repo_b).as_str(),
            "--output",
            "json",
        ],
        &[("CODEX_HOME", path_text(&codex_home))],
    )?;
    assert_success(&add);
    assert_eq!(
        list_connection_projects(runtime_home.path(), &connection_id)?.len(),
        2
    );

    let disable = run_with_home_env(
        runtime_home.path(),
        [
            "agent",
            "disable",
            "--connection-id",
            connection_id.as_str(),
            "--output",
            "json",
        ],
        &[("CODEX_HOME", path_text(&codex_home))],
    )?;
    assert_success(&disable);
    assert_eq!(json_stdout(&disable)?["connection"]["enabled"], false);

    let enable = run_with_home_env(
        runtime_home.path(),
        [
            "agent",
            "enable",
            "--connection-id",
            connection_id.as_str(),
            "--output",
            "json",
        ],
        &[("CODEX_HOME", path_text(&codex_home))],
    )?;
    assert_success(&enable);
    assert_eq!(json_stdout(&enable)?["connection"]["enabled"], true);

    let remove = run_with_home_env(
        runtime_home.path(),
        [
            "agent",
            "project",
            "remove",
            "--connection-id",
            connection_id.as_str(),
            "--project-id",
            "project_b",
            "--output",
            "json",
        ],
        &[("CODEX_HOME", path_text(&codex_home))],
    )?;
    assert_success(&remove);
    assert_eq!(
        list_connection_projects(runtime_home.path(), &connection_id)?.len(),
        1
    );

    let uninstall = run_with_home_env(
        runtime_home.path(),
        [
            "agent",
            "uninstall",
            "--connection-id",
            connection_id.as_str(),
            "--output",
            "json",
        ],
        &[
            ("PATH", path_env(&[bin_dir.as_path()])),
            ("CODEX_HOME", path_text(&codex_home)),
        ],
    )?;
    assert_success(&uninstall);
    assert!(agent_connection_record(runtime_home.path(), &connection_id)?.is_none());
    let config = fs::read_to_string(codex_home.join("config.toml"))?;
    assert!(!config.contains(&connection_id));
    Ok(())
}

fn run_without_home<const N: usize>(args: [&str; N]) -> Result<Output, Box<dyn Error>> {
    Ok(Command::new(volicord_bin()).args(args).output()?)
}

fn run_with_home_env<const N: usize>(
    runtime_home: &Path,
    args: [&str; N],
    envs: &[(&str, String)],
) -> Result<Output, Box<dyn Error>> {
    let mut command = Command::new(volicord_bin());
    command.args(args).env("VOLICORD_HOME", runtime_home);
    for (name, value) in envs {
        command.env(name, value);
    }
    Ok(command.output()?)
}

fn volicord_bin() -> &'static str {
    env!("CARGO_BIN_EXE_volicord")
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "command failed\nstdout:\n{}\nstderr:\n{}",
        stdout(output),
        stderr(output)
    );
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn json_stdout(output: &Output) -> Result<Value, Box<dyn Error>> {
    Ok(serde_json::from_str(&stdout(output))?)
}

fn path_text(path: &Path) -> String {
    path.display().to_string()
}

#[cfg(unix)]
fn path_env(path_dirs: &[&Path]) -> String {
    std::env::join_paths(path_dirs)
        .expect("test PATH should be valid")
        .to_string_lossy()
        .into_owned()
}

#[cfg(unix)]
fn write_fake_codex(dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    fs::create_dir_all(dir)?;
    let path = dir.join("codex");
    fs::write(
        &path,
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then printf 'codex 1.2.3-test\\n'; exit 0; fi\nprintf 'unexpected codex invocation\\n' >&2\nexit 2\n",
    )?;
    make_executable(&path)?;
    Ok(path)
}

#[cfg(unix)]
fn write_fake_mcp(dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    fs::create_dir_all(dir)?;
    let path = dir.join("volicord-mcp");
    fs::write(
        &path,
        "#!/bin/sh\n\
         mode=\"${VOLICORD_TEST_CONNECTION_MODE:-read_only}\"\n\
         if [ \"$1\" = \"--check\" ]; then\n\
         shift\n\
         if [ \"$1\" != \"--connection\" ]; then printf 'missing connection\\n' >&2; exit 2; fi\n\
         connection=\"$2\"\n\
         printf 'configuration: valid\\n'\n\
         printf 'transport: stdio\\n'\n\
         printf 'runtime_home: %s\\n' \"$VOLICORD_HOME\"\n\
         printf 'connection_id: %s\\n' \"$connection\"\n\
         printf 'mode: %s\\n' \"$mode\"\n\
         printf 'enabled: true\\n'\n\
         printf 'allowed_projects: 1\\n'\n\
         printf 'available_projects: 1\\n'\n\
         printf 'verification_scope: startup_check_only\\n'\n\
         exit 0\n\
         fi\n\
         if [ \"$1\" = \"--connection\" ]; then\n\
         while IFS= read -r line; do\n\
         case \"$line\" in\n\
         *'\"method\":\"initialize\"'*) printf '%s\\n' '{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"protocolVersion\":\"2025-11-25\",\"capabilities\":{\"tools\":{}},\"serverInfo\":{\"name\":\"volicord-mcp\",\"version\":\"test\"},\"instructions\":\"Use Volicord.\"}}' ;;\n\
         *'\"method\":\"tools/list\"'*)\n\
         if [ \"$mode\" = \"workflow\" ]; then\n\
         printf '%s\\n' '{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{\"tools\":[{\"name\":\"volicord.intake\"},{\"name\":\"volicord.update_scope\"},{\"name\":\"volicord.status\"},{\"name\":\"volicord.prepare_write\"},{\"name\":\"volicord.stage_artifact\"},{\"name\":\"volicord.record_run\"},{\"name\":\"volicord.request_user_judgment\"},{\"name\":\"volicord.close_task\"},{\"name\":\"volicord.list_projects\"}]}}'\n\
         else\n\
         printf '%s\\n' '{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{\"tools\":[{\"name\":\"volicord.status\"},{\"name\":\"volicord.close_task\"},{\"name\":\"volicord.list_projects\"}]}}'\n\
         fi\n\
         exit 0 ;;\n\
         esac\n\
         done\n\
         exit 0\n\
         fi\n\
         printf 'unexpected invocation\\n' >&2\n\
         exit 2\n",
    )?;
    make_executable(&path)?;
    Ok(path)
}

#[cfg(unix)]
fn make_executable(path: &Path) -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)?;
    Ok(())
}
