#![forbid(unsafe_code)]

use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Output, Stdio},
};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use volicord_core::{CoreService, InvocationContext};
use volicord_store::agent_connections::{
    add_connection_project, ensure_agent_connection, AgentConnectionRegistration,
    ConnectionProjectRegistration, CONNECTION_INTENT_SHARED, CONNECTION_MODE_WORKFLOW,
    HOST_KIND_CODEX, HOST_SCOPE_PROJECT, VERIFIED_STATUS_COMPLETE,
};
use volicord_store::guards::{
    guard_event, guard_installation, list_unresolved_unrecorded_changes, prompt_capture,
    upsert_guard_installation, GuardInstallationUpsert,
};
use volicord_test_support::core_fixtures::{CoreFixture, UpdateScopeFixture, UserJudgmentFixture};
use volicord_types::{
    ActorSource, ChangeUnitOperation, JudgmentKind, OperationCategory, ProjectId,
    VERIFICATION_BASIS_TEST_FIXTURE_BINDING, VERIFICATION_BASIS_USER_PROMPT_SUBMIT_HOOK,
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
fn guard_session_start_promotes_matching_installation_active() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-session-activates")?;
    let (guard_installation_id, policy_hash) = fixture.install_guard_policy()?;
    let event = json!({
        "event_id": "guard_session_activate_event",
        "session_id": "guard_session_activate",
        "connection_id": fixture.connection_id(),
        "guard_installation_id": guard_installation_id,
        "host_kind": "codex",
        "timestamp": "2026-06-30T04:00:00Z"
    });

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "session-start", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_success(&output);

    let stored = guard_installation(fixture.runtime_home(), &guard_installation_id)?
        .expect("guard installation should be stored");
    assert_eq!(stored.installation_status, "active");
    assert_eq!(
        stored.first_seen_at.as_deref(),
        Some("2026-06-30T04:00:00Z")
    );
    assert_eq!(stored.last_seen_at.as_deref(), Some("2026-06-30T04:00:00Z"));
    assert_eq!(stored.last_seen_phase.as_deref(), Some("session_start"));
    assert_eq!(stored.observed_host_kind.as_deref(), Some("codex"));
    assert_eq!(
        stored.observed_policy_hash.as_deref(),
        Some(policy_hash.as_str())
    );
    assert_eq!(
        stored.observed_binary_version.as_deref(),
        Some(env!("CARGO_PKG_VERSION"))
    );
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
fn guard_pre_tool_rejects_paths_outside_project_allowlist() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-pre-outside-project")?;
    let event = json!({
        "event_id": "guard_pre_outside_project",
        "session_id": "guard_session_pre_outside_project",
        "connection_id": fixture.connection_id(),
        "host_kind": "codex",
        "tool_name": "read",
        "paths": ["../outside-product-repo.txt"]
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
    assert_reason(&value, "target_outside_project_allowlist");

    let stored = guard_event(
        fixture.runtime_home(),
        fixture.project_id(),
        "guard_pre_outside_project",
    )?
    .expect("outside-project guard event should be stored");
    assert_eq!(stored.decision, "deny");
    assert_eq!(stored.event_kind, "pre_tool");
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
    let stored = guard_event(
        fixture.runtime_home(),
        fixture.project_id(),
        "guard_post_changed",
    )?
    .expect("post-tool guard event should be stored");
    assert_eq!(stored.decision, "warn");
    assert_eq!(stored.event_kind, "post_tool");
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
fn guard_session_start_shows_chat_judgment_instructions() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-instructions")?;
    fixture.create_pending_authority_judgment("instructions")?;
    let event = json!({
        "event_id": "guard_session_chat_instructions",
        "session_id": "guard_session_chat_instructions",
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
    let pending = &value["result"]["context"]["pending_user_judgments"][0];
    assert_eq!(pending["chat_id"], "J-1");
    assert_eq!(pending["answer_instruction"], "Volicord: answer J-1 1");
    assert_eq!(pending["note_instruction"], "Volicord: note J-1 \"text\"");
    assert_eq!(
        pending["options"][1]["instruction"],
        "Volicord: answer J-1 reject"
    );
    assert_eq!(
        pending["options"][2]["instruction"],
        "Volicord: answer J-1 defer"
    );
    assert!(
        pending["verification_nonce"]
            .as_str()
            .expect("nonce should be present")
            .len()
            >= 10
    );
    Ok(())
}

#[test]
fn guard_prompt_capture_records_answer_command() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-answer")?;
    let judgment_id = fixture.create_pending_authority_judgment("answer")?;
    let event = prompt_event(
        &fixture,
        "guard_prompt_answer",
        "guard_prompt_capture_answer",
        "Volicord: answer J-1 1",
    );

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_success(&output);
    let value = json_stdout(&output)?;
    assert_eq!(value["decision"], "inject_context");
    assert_eq!(
        value["result"]["recognized_judgment_command"]["selected_option_id"],
        "accept"
    );
    assert_eq!(
        value["result"]["recognized_judgment_command"]["resolution_outcome"],
        "accepted"
    );
    assert!(value["result"]["model_context"]
        .as_str()
        .expect("model context should be present")
        .contains("Volicord recorded the user-owned judgment"));
    fixture.assert_recorded_prompt_judgment(&judgment_id, "accepted", "accept")?;
    Ok(())
}

#[test]
fn guard_prompt_capture_records_reject_command() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-reject")?;
    let judgment_id = fixture.create_pending_authority_judgment("reject")?;
    let event = prompt_event(
        &fixture,
        "guard_prompt_reject",
        "guard_prompt_capture_reject",
        "Volicord: answer J-1 reject",
    );

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_success(&output);
    fixture.assert_recorded_prompt_judgment(&judgment_id, "rejected", "reject")?;
    Ok(())
}

#[test]
fn guard_prompt_capture_records_defer_command() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-defer")?;
    let judgment_id = fixture.create_pending_authority_judgment("defer")?;
    let event = prompt_event(
        &fixture,
        "guard_prompt_defer",
        "guard_prompt_capture_defer",
        "Volicord: answer J-1 defer",
    );

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_success(&output);
    fixture.assert_recorded_prompt_judgment(&judgment_id, "deferred", "defer")?;
    Ok(())
}

#[test]
fn guard_prompt_capture_records_note_as_deferred_judgment() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-note")?;
    let judgment_id = fixture.create_pending_authority_judgment("note")?;
    let event = prompt_event(
        &fixture,
        "guard_prompt_note",
        "guard_prompt_capture_note",
        "Volicord: note J-1 \"Need to review this later\"",
    );

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_success(&output);
    fixture.assert_recorded_prompt_judgment(&judgment_id, "deferred", "defer")?;
    let resolution = fixture.judgment_resolution(&judgment_id)?;
    assert_eq!(resolution["note"], "Need to review this later");
    Ok(())
}

#[test]
fn guard_prompt_capture_rejects_malformed_command() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-malformed")?;
    let judgment_id = fixture.create_pending_authority_judgment("malformed")?;
    let event = prompt_event(
        &fixture,
        "guard_prompt_malformed",
        "guard_prompt_capture_malformed",
        "Volicord: answer J-1",
    );

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_eq!(output.status.code(), Some(1));
    let value = json_stdout(&output)?;
    assert_reason(&value, "malformed_judgment_command");
    assert_eq!(fixture.judgment_status(&judgment_id)?, "pending");
    Ok(())
}

#[test]
fn guard_prompt_capture_ignores_non_command_prompt() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-non-command")?;
    let event = prompt_event(
        &fixture,
        "guard_prompt_non_command",
        "guard_prompt_capture_non_command",
        "Please explain what Volicord should do next.",
    );

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_success(&output);
    let value = json_stdout(&output)?;
    assert_eq!(value["decision"], "allow");
    assert!(value["result"]["recognized_judgment_command"].is_null());
    Ok(())
}

#[test]
fn guard_prompt_capture_rejects_invalid_chat_id() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-invalid-id")?;
    let judgment_id = fixture.create_pending_authority_judgment("invalid_id")?;
    let event = prompt_event(
        &fixture,
        "guard_prompt_invalid_id",
        "guard_prompt_capture_invalid_id",
        "Volicord: answer J-99 1",
    );

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_eq!(output.status.code(), Some(1));
    assert_reason(&json_stdout(&output)?, "unknown_judgment_id");
    assert_eq!(fixture.judgment_status(&judgment_id)?, "pending");
    Ok(())
}

#[test]
fn guard_prompt_capture_rejects_mismatched_project() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-project-mismatch")?;
    let judgment_id = fixture.create_pending_authority_judgment("project_mismatch")?;
    let mut event = prompt_event(
        &fixture,
        "guard_prompt_project_mismatch",
        "guard_prompt_capture_project_mismatch",
        "Volicord: answer J-1 1",
    );
    event["project_id"] = json!("other_project");

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_eq!(output.status.code(), Some(1));
    assert_reason(&json_stdout(&output)?, "project_mismatch");
    assert_eq!(fixture.judgment_status(&judgment_id)?, "pending");
    Ok(())
}

#[test]
fn guard_prompt_capture_rejects_mismatched_connection() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-connection-mismatch")?;
    let judgment_id = fixture.create_pending_authority_judgment("connection_mismatch")?;
    fixture.register_extra_connection("other_connection")?;
    let mut event = prompt_event(
        &fixture,
        "guard_prompt_connection_mismatch",
        "guard_prompt_capture_connection_mismatch",
        "Volicord: answer J-1 1",
    );
    event["connection_id"] = json!("other_connection");

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_eq!(output.status.code(), Some(1));
    assert_reason(&json_stdout(&output)?, "connection_mismatch");
    assert_eq!(fixture.judgment_status(&judgment_id)?, "pending");
    Ok(())
}

#[test]
fn guard_prompt_capture_rejects_stale_judgment() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-stale")?;
    let judgment_id = fixture.create_pending_authority_judgment("stale")?;
    fixture.set_judgment_basis_status(&judgment_id, "stale")?;
    let event = prompt_event(
        &fixture,
        "guard_prompt_stale",
        "guard_prompt_capture_stale",
        "Volicord: answer J-1 1",
    );

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_eq!(output.status.code(), Some(1));
    assert_reason(&json_stdout(&output)?, "stale_judgment");
    assert_eq!(fixture.judgment_status(&judgment_id)?, "pending");
    Ok(())
}

#[test]
fn guard_prompt_capture_rejects_duplicate_answer() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-duplicate")?;
    let judgment_id = fixture.create_pending_authority_judgment("duplicate")?;
    let first = prompt_event(
        &fixture,
        "guard_prompt_duplicate_first",
        "guard_prompt_capture_duplicate_first",
        "Volicord: answer J-1 1",
    );
    let second = prompt_event(
        &fixture,
        "guard_prompt_duplicate_second",
        "guard_prompt_capture_duplicate_second",
        "Volicord: answer J-1 1",
    );

    assert_success(&run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &first,
    )?);
    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &second,
    )?;
    assert_eq!(output.status.code(), Some(1));
    assert_reason(&json_stdout(&output)?, "judgment_not_pending");
    assert_eq!(fixture.judgment_status(&judgment_id)?, "resolved");
    Ok(())
}

#[test]
fn guard_prompt_capture_rejects_ambiguous_commands() -> Result<(), Box<dyn Error>> {
    let fixture = GuardCliFixture::new("guard-chat-ambiguous")?;
    let judgment_id = fixture.create_pending_authority_judgment("ambiguous")?;
    let event = prompt_event(
        &fixture,
        "guard_prompt_ambiguous",
        "guard_prompt_capture_ambiguous",
        "Volicord: answer J-1 1\nVolicord: answer J-1 reject",
    );

    let output = run_guard(
        fixture.runtime_home(),
        fixture.repo_root(),
        ["guard", "prompt-capture", "--repo", fixture.repo_arg()],
        &event,
    )?;
    assert_eq!(output.status.code(), Some(1));
    assert_reason(&json_stdout(&output)?, "ambiguous_judgment_command");
    assert_eq!(fixture.judgment_status(&judgment_id)?, "pending");
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

    fn create_pending_authority_judgment(&self, suffix: &str) -> Result<String, Box<dyn Error>> {
        let task_id = self.create_active_task()?;
        let state_version = self.inner.store()?.project_state()?.state_version;
        let service = CoreService::new(self.runtime_home());
        let request_id = format!("req_guard_chat_judgment_{suffix}");
        let idempotency_key = format!("idem_guard_chat_judgment_{suffix}");
        let response = service.request_user_judgment(
            self.inner.user_judgment_request(UserJudgmentFixture {
                request_id: &request_id,
                idempotency_key: &idempotency_key,
                dry_run: false,
                expected_state_version: Some(state_version),
                task_id: &task_id,
                change_unit_id: None,
                judgment_kind: JudgmentKind::Cancellation,
            }),
            self.invocation(OperationCategory::AgentWorkflow),
        )?;
        record_id(&response.response_value["user_judgment_ref"])
    }

    fn assert_recorded_prompt_judgment(
        &self,
        judgment_id: &str,
        expected_outcome: &str,
        expected_action: &str,
    ) -> Result<(), Box<dyn Error>> {
        let record = self
            .inner
            .store()?
            .user_judgment_record(judgment_id)?
            .expect("judgment should be stored");
        assert_eq!(record.status, "resolved");
        assert_eq!(record.resolution_outcome.as_deref(), Some(expected_outcome));
        assert_eq!(
            record.resolution_machine_action.as_deref(),
            Some(expected_action)
        );
        assert_eq!(
            record.resolved_by_actor_source.as_deref(),
            Some("local_user")
        );
        assert_eq!(
            record.resolved_verification_basis.as_deref(),
            Some(VERIFICATION_BASIS_USER_PROMPT_SUBMIT_HOOK)
        );
        assert_eq!(
            record.resolved_assurance_level.as_deref(),
            Some("local_user_channel")
        );
        Ok(())
    }

    fn judgment_status(&self, judgment_id: &str) -> Result<String, Box<dyn Error>> {
        Ok(self.inner.user_judgment_status(judgment_id)?)
    }

    fn judgment_resolution(&self, judgment_id: &str) -> Result<Value, Box<dyn Error>> {
        self.inner.user_judgment_resolution(judgment_id)
    }

    fn set_judgment_basis_status(
        &self,
        judgment_id: &str,
        basis_status: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.inner.conn()?.execute(
            "UPDATE user_judgments
                SET basis_status = ?3
              WHERE project_id = ?1
                AND judgment_id = ?2",
            rusqlite::params![self.project_id(), judgment_id, basis_status],
        )?;
        Ok(())
    }

    fn register_extra_connection(&self, connection_id: &str) -> Result<(), Box<dyn Error>> {
        ensure_agent_connection(
            self.runtime_home(),
            AgentConnectionRegistration {
                connection_internal_id: connection_id.to_owned(),
                host_kind: HOST_KIND_CODEX.to_owned(),
                intent: CONNECTION_INTENT_SHARED.to_owned(),
                host_scope: HOST_SCOPE_PROJECT.to_owned(),
                server_name: format!("volicord-test-{connection_id}"),
                config_target: self
                    .runtime_home()
                    .join("agent-connections")
                    .join(connection_id)
                    .to_string_lossy()
                    .into_owned(),
                mode: CONNECTION_MODE_WORKFLOW.to_owned(),
                enabled: true,
                managed_fingerprint: format!("fixture:{connection_id}"),
                last_verification_status: VERIFIED_STATUS_COMPLETE.to_owned(),
                last_verification_report_json: "{}".to_owned(),
                last_user_actions_json: "[]".to_owned(),
                metadata_json: "{}".to_owned(),
            },
        )?;
        add_connection_project(
            self.runtime_home(),
            ConnectionProjectRegistration {
                connection_internal_id: connection_id.to_owned(),
                project_id: self.project_id().to_owned(),
            },
        )?;
        Ok(())
    }

    fn install_guard_policy(&self) -> Result<(String, String), Box<dyn Error>> {
        let guard_installation_id = "guard_installation_cli_activation".to_owned();
        let policy = json!({
            "schema": "volicord-policy-v1",
            "managed_by": "volicord",
            "host": "codex",
            "mode": "guarded",
            "guard_mode": "guarded",
            "connection_id": self.connection_id(),
            "guard_installation_id": guard_installation_id
        });
        let policy_hash = sha256_text(&serde_json::to_string(&policy)?);
        let policy_dir = self.repo_root.join(".volicord");
        fs::create_dir_all(&policy_dir)?;
        fs::write(
            policy_dir.join("policy.json"),
            serde_json::to_string_pretty(&policy)?,
        )?;
        upsert_guard_installation(
            self.runtime_home(),
            GuardInstallationUpsert {
                guard_installation_id: guard_installation_id.clone(),
                connection_internal_id: self.connection_id().to_owned(),
                project_id: Some(self.project_id().to_owned()),
                host_kind: "codex".to_owned(),
                guard_mode: "guarded".to_owned(),
                host_capability_json: json!({
                    "schema": "volicord-guard-capability-v1",
                    "policy_hash": policy_hash.clone(),
                    "prompt_capture": true
                })
                .to_string(),
                installation_status: "reload_required".to_owned(),
                installed_at: Some("2026-06-30T03:59:00Z".to_owned()),
                last_checked_at: "2026-06-30T03:59:00Z".to_owned(),
                first_seen_at: None,
                last_seen_at: None,
                last_seen_phase: None,
                observed_host_kind: None,
                observed_policy_hash: None,
                observed_binary_version: None,
                metadata_json: "{}".to_owned(),
            },
        )?;
        Ok((guard_installation_id, policy_hash))
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

fn prompt_event(
    fixture: &GuardCliFixture,
    event_id: &str,
    capture_id: &str,
    message: &str,
) -> Value {
    json!({
        "event_id": event_id,
        "prompt_capture_id": capture_id,
        "session_id": "guard_session_chat",
        "connection_id": fixture.connection_id(),
        "host_kind": "codex",
        "message": message
    })
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

fn sha256_text(text: &str) -> String {
    let digest = Sha256::digest(text.as_bytes());
    format!("sha256:{digest:x}")
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
