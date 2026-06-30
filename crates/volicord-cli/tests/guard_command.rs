#![forbid(unsafe_code)]

use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Output, Stdio},
};

use serde_json::{json, Value};
use volicord_core::{CoreService, InvocationContext};
use volicord_store::guards::{guard_event, list_unresolved_unrecorded_changes, prompt_capture};
use volicord_test_support::core_fixtures::{CoreFixture, UpdateScopeFixture};
use volicord_types::{
    ActorSource, ChangeUnitOperation, OperationCategory, ProjectId,
    VERIFICATION_BASIS_TEST_FIXTURE_BINDING,
};

#[test]
fn guard_session_start_injects_context_and_records_event() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-session-start")?;
    let event = json!({
        "event_id": "guard_session_start_event",
        "session_id": "guard_session_a",
        "connection_id": fixture.connection_id(),
        "host_kind": "codex"
    });

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "session-start", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_success(&output);
    let value = json_stdout(&output)?;
    assert_eq!(value["decision"], "inject_context");
    assert_eq!(value["allowed"], true);
    assert_eq!(value["session_id"], "guard_session_a");
    assert_eq!(
        value["result"]["context"]["project_id"],
        fixture.project_id()
    );

    let stored = guard_event(
        fixture.runtime_home(),
        fixture.project_id(),
        "guard_session_start_event",
    )?
    .expect("guard event should be stored");
    assert_eq!(stored.decision, "inject_context");
    assert_eq!(stored.event_kind, "session_start");
    Ok(())
}

#[test]
fn guard_pre_tool_denies_product_write_without_active_task() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-pre-no-task")?;
    let event = json!({
        "event_id": "guard_pre_no_task",
        "session_id": "guard_session_pre_no_task",
        "connection_id": fixture.connection_id(),
        "host": {"kind": "claude_code"},
        "tool_name": "shell",
        "command": "touch src/lib.rs",
        "paths": ["src/lib.rs"]
    });

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "pre-tool", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_eq!(output.status.code(), Some(1));
    let value = json_stdout(&output)?;
    assert_eq!(value["decision"], "deny");
    assert_reason(&value, "no_active_task");

    let stored = guard_event(
        fixture.runtime_home(),
        fixture.project_id(),
        "guard_pre_no_task",
    )?
    .expect("deny event should be stored");
    assert_eq!(stored.decision, "deny");
    Ok(())
}

#[test]
fn guard_pre_tool_allows_read_status_without_active_task() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-pre-read")?;
    let event = json!({
        "event_id": "guard_pre_read",
        "connection_id": fixture.connection_id(),
        "host_kind": "codex",
        "tool_name": "shell",
        "command": "git status --short"
    });

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "pre-tool", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_success(&output);
    let value = json_stdout(&output)?;
    assert_eq!(value["decision"], "allow");
    assert_eq!(value["allowed"], true);
    assert!(value["result"]["reasons"]
        .as_array()
        .expect("reasons should be an array")
        .is_empty());
    Ok(())
}

#[test]
fn guard_pre_tool_requires_current_write_readiness() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-pre-write-ready")?;
    let task_id = fixture.create_active_task()?;
    let denied_event = json!({
        "event_id": "guard_pre_missing_write_check",
        "session_id": "guard_session_write_ready",
        "connection_id": fixture.connection_id(),
        "host_kind": "codex",
        "tool_name": "shell",
        "command": "touch src/export.rs",
        "paths": ["src/export.rs"]
    });

    let denied = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "pre-tool", "--repo", fixture.repo_arg()],
        &denied_event,
    )?;
    assert_eq!(denied.status.code(), Some(1));
    assert_reason(&json_stdout(&denied)?, "write_readiness_missing");

    fixture.prepare_write(&task_id)?;
    let allowed_event = json!({
        "event_id": "guard_pre_with_write_check",
        "session_id": "guard_session_write_ready",
        "connection_id": fixture.connection_id(),
        "host_kind": "codex",
        "tool_name": "shell",
        "command": "touch src/export.rs",
        "paths": ["src/export.rs"]
    });
    let allowed = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "pre-tool", "--repo", fixture.repo_arg()],
        &allowed_event,
    )?;
    assert_success(&allowed);
    let value = json_stdout(&allowed)?;
    assert_eq!(value["decision"], "allow");
    assert_eq!(value["allowed"], true);
    Ok(())
}

#[test]
fn guard_post_tool_records_unrecorded_product_file_changes() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-post-unrecorded")?;
    let task_id = fixture.create_active_task()?;
    let event = json!({
        "event_id": "guard_post_changed",
        "session_id": "guard_session_post",
        "connection_id": fixture.connection_id(),
        "host_kind": "codex",
        "tool_name": "shell",
        "command": "touch src/export.rs",
        "success": true,
        "changed_paths": ["src/export.rs"]
    });

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "post-tool", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_success(&output);
    let value = json_stdout(&output)?;
    assert_eq!(value["decision"], "warn");
    assert_eq!(
        value["result"]["unrecorded_changes"][0]["observed_paths"][0],
        "src/export.rs"
    );
    let unresolved = list_unresolved_unrecorded_changes(
        fixture.runtime_home(),
        fixture.project_id(),
        Some(fixture.connection_id()),
    )?;
    assert_eq!(unresolved.len(), 1);
    assert_eq!(unresolved[0].task_id.as_deref(), Some(task_id.as_str()));
    Ok(())
}

#[test]
fn guard_prompt_capture_hashes_prompt_and_omits_text() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-prompt-capture")?;
    let event_file = fixture.repo_root().join("prompt-event.json");
    fs::write(
        &event_file,
        json!({
            "event_id": "guard_prompt_event",
            "prompt_capture_id": "guard_prompt_capture_a",
            "session_id": "guard_session_prompt",
            "connection_id": fixture.connection_id(),
            "host": {"kind": "claude_code"},
            "message": "Please prepare the write carefully."
        })
        .to_string(),
    )?;

    let output = run_guard_file(
        fixture.runtime_home(),
        fixture.repo_root(),
        [
            "guard",
            "prompt-capture",
            "--repo",
            fixture.repo_arg(),
            "--file",
            event_file.to_str().expect("test path should be UTF-8"),
        ],
    )?;
    assert_success(&output);
    let value = json_stdout(&output)?;
    assert_eq!(value["decision"], "allow");
    assert_eq!(
        value["result"]["prompt_capture"]["prompt_capture_id"],
        "guard_prompt_capture_a"
    );
    assert_eq!(
        value["result"]["prompt_capture"]["prompt_text_omitted"],
        true
    );

    let stored = prompt_capture(
        fixture.runtime_home(),
        fixture.project_id(),
        "guard_prompt_capture_a",
    )?
    .expect("prompt capture should be stored");
    assert!(stored.prompt_text.is_none());
    assert!(stored.prompt_sha256.starts_with("sha256:"));
    Ok(())
}

#[test]
fn guard_stop_denies_false_completion_when_close_readiness_blocks() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-stop-blocked")?;
    fixture.create_active_task()?;
    let event = json!({
        "event_id": "guard_stop_blocked",
        "session_id": "guard_session_stop",
        "connection_id": fixture.connection_id(),
        "host_kind": "codex",
        "message": "All done."
    });

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "stop", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_eq!(output.status.code(), Some(1));
    let value = json_stdout(&output)?;
    assert_eq!(value["decision"], "deny");
    assert_reason(&value, "close_readiness_blocked");
    assert!(value["result"]["close_status"]["close_blockers"]
        .as_array()
        .expect("close blockers should be an array")
        .iter()
        .any(|blocker| blocker["code"] == "missing_current_close_basis"));
    Ok(())
}

struct GuardCliFixture {
    inner: CoreFixture,
    repo_root: std::path::PathBuf,
    repo_arg: String,
}

impl GuardCliFixture {
    fn new(prefix: &str) -> Result<Self, Box<dyn Error>> {
        let inner = CoreFixture::new(prefix)?;
        let repo_root = inner.product_repo_path();
        fs::create_dir_all(repo_root.join(".git"))?;
        let repo_arg = repo_root.display().to_string();
        Ok(Self {
            inner,
            repo_root,
            repo_arg,
        })
    }

    fn runtime_home(&self) -> &Path {
        self.inner.runtime_home_path()
    }

    fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    fn repo_arg(&self) -> &str {
        &self.repo_arg
    }

    fn project_id(&self) -> &str {
        self.inner.project_id()
    }

    fn connection_id(&self) -> &str {
        self.inner.connection_id()
    }

    fn create_active_task(&self) -> Result<String, Box<dyn Error>> {
        let service = CoreService::new(self.runtime_home());
        let response = service.intake(
            self.inner
                .intake_request("req_guard_intake", "idem_guard_intake", false, Some(0)),
            self.invocation(OperationCategory::AgentWorkflow),
        )?;
        let task_id = record_id(&response.response_value["task_ref"])?;
        service.update_scope(
            self.inner.update_scope_request(UpdateScopeFixture {
                request_id: "req_guard_scope",
                idempotency_key: "idem_guard_scope",
                dry_run: false,
                expected_state_version: Some(1),
                task_id: &task_id,
                operation: ChangeUnitOperation::CreateCurrent,
                scope_summary: "Guard fixture scope for src/export.rs.",
            }),
            self.invocation(OperationCategory::AgentWorkflow),
        )?;
        Ok(task_id)
    }

    fn prepare_write(&self, task_id: &str) -> Result<(), Box<dyn Error>> {
        let service = CoreService::new(self.runtime_home());
        let state_version = self.inner.store()?.project_state()?.state_version;
        let response = service.prepare_write(
            self.inner.prepare_write_request(
                "req_guard_prepare_write",
                "idem_guard_prepare_write",
                Some(state_version),
                Some(task_id),
                None,
            ),
            self.invocation(OperationCategory::AgentWorkflow),
        )?;
        assert_eq!(response.response_value["decision"], "allowed");
        Ok(())
    }

    fn invocation(&self, operation_category: OperationCategory) -> InvocationContext {
        InvocationContext::new(
            ProjectId::new(self.project_id()),
            ActorSource::agent_connection(self.connection_id().to_owned()),
            operation_category,
            VERIFICATION_BASIS_TEST_FIXTURE_BINDING,
        )
    }
}

fn run_guard<const N: usize>(
    runtime_home: &Path,
    current_dir: &Path,
    args: [&str; N],
    event: &Value,
) -> Result<Output, Box<dyn Error>> {
    let mut child = Command::new(volicord_bin())
        .args(args)
        .env("VOLICORD_HOME", runtime_home)
        .current_dir(current_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .as_mut()
        .expect("stdin should be piped")
        .write_all(event.to_string().as_bytes())?;
    Ok(child.wait_with_output()?)
}

fn run_guard_file<const N: usize>(
    runtime_home: &Path,
    current_dir: &Path,
    args: [&str; N],
) -> Result<Output, Box<dyn Error>> {
    Ok(Command::new(volicord_bin())
        .args(args)
        .env("VOLICORD_HOME", runtime_home)
        .current_dir(current_dir)
        .output()?)
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

fn json_stdout(output: &Output) -> Result<Value, Box<dyn Error>> {
    Ok(serde_json::from_str(&stdout(output))?)
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn assert_reason(value: &Value, code: &str) {
    assert!(
        value["result"]["reasons"]
            .as_array()
            .expect("reasons should be an array")
            .iter()
            .any(|reason| reason["code"] == code),
        "expected reason {code}, got {}",
        value["result"]["reasons"]
    );
}

fn record_id(value: &Value) -> Result<String, Box<dyn Error>> {
    value["record_id"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| "record_id should be present".into())
}
