use std::{
    collections::BTreeSet,
    error::Error,
    io::{BufReader, Cursor},
};

use harness_core::{CoreService, InvocationContext};
use harness_mcp::{
    public_method_tools, run_stdio, McpAdapter, McpSessionContext, PUBLIC_METHOD_TOOL_NAMES,
};
use harness_store::bootstrap::{register_surface, SurfaceRegistration};
use harness_test_support::core_fixtures::{CoreFixture, UpdateScopeFixture};
use harness_types::{AccessClass, ChangeUnitOperation, SurfaceId, SurfaceInstanceId};
use serde_json::{json, Value};

#[test]
fn mcp_exposes_exactly_the_documented_public_methods() {
    let tools = public_method_tools();
    let names = tools.iter().map(|tool| tool.name).collect::<Vec<_>>();
    let unique_names = names.iter().copied().collect::<BTreeSet<_>>();

    assert_eq!(names, PUBLIC_METHOD_TOOL_NAMES);
    assert_eq!(tools.len(), 9);
    assert_eq!(unique_names.len(), 9);
}

#[test]
fn stdio_tools_list_exposes_exactly_the_public_method_set() -> Result<(), Box<dyn Error>> {
    let fixture = CoreFixture::new("mcp_tools")?;
    let adapter = adapter(&fixture, AccessClass::ReadStatus);
    let input = Cursor::new(
        br#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}
"#
        .to_vec(),
    );
    let mut output = Vec::new();

    run_stdio(adapter, BufReader::new(input), &mut output)?;

    let response: Value = serde_json::from_slice(&output)?;
    let names = response["result"]["tools"]
        .as_array()
        .expect("tools should be an array")
        .iter()
        .map(|tool| tool["name"].as_str().expect("tool name"))
        .collect::<Vec<_>>();
    assert_eq!(names, PUBLIC_METHOD_TOOL_NAMES);
    Ok(())
}

#[test]
fn adapter_uses_session_surface_context_for_artifact_provenance() -> Result<(), Box<dyn Error>> {
    let fixture = CoreFixture::new("mcp_surface")?;
    let core = CoreService::new(fixture.runtime_home_path());
    let intake = core.intake(
        fixture.intake_request("req_mcp_task", "idem_mcp_task", false, Some(0)),
        invocation(&fixture, AccessClass::CoreMutation),
    )?;
    let task_id = intake.response_value["task_ref"]["record_id"]
        .as_str()
        .expect("task ref should be present")
        .to_owned();

    let adapter = adapter(&fixture, AccessClass::ArtifactRegistration);
    let params = serde_json::to_value(fixture.stage_artifact_request(
        "req_mcp_stage",
        None,
        false,
        Some(1),
        &task_id,
    ))?;

    let response = adapter.call_tool("harness.stage_artifact", params)?;

    assert_eq!(response.response_value["base"]["response_kind"], "result");
    assert_eq!(
        response.response_value["staged_artifact_handle"]["created_by_surface_id"],
        fixture.surface_id()
    );
    assert_eq!(
        response.response_value["staged_artifact_handle"]["created_by_surface_instance_id"],
        fixture.surface_instance_id()
    );
    assert_eq!(fixture.counts()?.state_version, 1);
    assert_eq!(fixture.counts()?.artifact_staging, 1);
    Ok(())
}

#[test]
fn invalid_mcp_authority_fields_are_rejected_before_core() -> Result<(), Box<dyn Error>> {
    let fixture = CoreFixture::new("mcp_invalid_fields")?;
    let core = CoreService::new(fixture.runtime_home_path());
    let intake = core.intake(
        fixture.intake_request("req_invalid_task", "idem_invalid_task", false, Some(0)),
        invocation(&fixture, AccessClass::CoreMutation),
    )?;
    let task_id = intake.response_value["task_ref"]["record_id"]
        .as_str()
        .expect("task ref should be present")
        .to_owned();
    let adapter = adapter(&fixture, AccessClass::ArtifactRegistration);

    for (field_path, forged_value) in [
        ("envelope.verified", json!(true)),
        (
            "envelope.surface_instance_id",
            json!("surface_instance_forged"),
        ),
        ("verified_surface_context", json!({ "verified": true })),
        ("access_class", json!("core_mutation")),
        (
            "capability_profile",
            json!({ "artifact_registration": true }),
        ),
    ] {
        let mut params = serde_json::to_value(fixture.stage_artifact_request(
            &format!("req_invalid_{}", field_path.replace('.', "_")),
            None,
            false,
            Some(1),
            &task_id,
        ))?;
        if let Some(field) = field_path.strip_prefix("envelope.") {
            params["envelope"][field] = forged_value;
        } else {
            params[field_path] = forged_value;
        }
        let before = fixture.counts()?;

        let error = adapter
            .call_tool("harness.stage_artifact", params)
            .expect_err("invalid request params should fail before Core");

        assert!(matches!(
            error,
            harness_mcp::McpAdapterError::InvalidParams { .. }
        ));
        assert_eq!(
            fixture.counts()?,
            before,
            "{field_path} should create no storage effect"
        );
    }

    Ok(())
}

#[test]
fn stdio_invalid_params_returns_protocol_error_without_storage_effect() -> Result<(), Box<dyn Error>>
{
    let fixture = CoreFixture::new("mcp_stdio_invalid")?;
    let adapter = adapter(&fixture, AccessClass::ReadStatus);
    let before = fixture.counts()?;
    let input = Cursor::new(
        br#"{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"harness.status","arguments":{"envelope":{"project_id":"project_fixture","task_id":null,"actor_kind":"agent","surface_id":"surface_fixture","request_id":"req_stdio_invalid","idempotency_key":null,"expected_state_version":null,"dry_run":false,"locale":"en-US"},"include":{"task":true,"pending_user_judgments":true,"write_authority":true,"evidence":true,"close":true,"guarantees":true},"access_class":"core_mutation"}}}
"#
        .to_vec(),
    );
    let mut output = Vec::new();

    run_stdio(adapter, BufReader::new(input), &mut output)?;

    let response: Value = serde_json::from_slice(&output)?;
    assert_eq!(response["error"]["code"], -32602);
    assert_eq!(response["error"]["message"], "Invalid params");
    assert_eq!(fixture.counts()?, before);
    Ok(())
}

#[test]
fn adapter_does_not_expand_access_class_for_method_calls() -> Result<(), Box<dyn Error>> {
    let fixture = CoreFixture::new("mcp_access")?;
    let adapter = adapter(&fixture, AccessClass::CoreMutation);
    let response = adapter.call_tool(
        "harness.status",
        serde_json::to_value(fixture.status_request("req_status_wrong_access", None))?,
    )?;

    assert_eq!(response.response_value["base"]["response_kind"], "rejected");
    assert_eq!(
        response.response_value["errors"][0]["code"],
        "CAPABILITY_INSUFFICIENT"
    );
    Ok(())
}

#[test]
fn mcp_replay_rejects_different_session_access_class_without_stored_response(
) -> Result<(), Box<dyn Error>> {
    let fixture = CoreFixture::new("mcp_replay_context")?;
    let core = CoreService::new(fixture.runtime_home_path());
    let intake = core.intake(
        fixture.intake_request(
            "req_mcp_replay_task",
            "idem_mcp_replay_task",
            false,
            Some(0),
        ),
        invocation(&fixture, AccessClass::CoreMutation),
    )?;
    let task_id = intake.response_value["task_ref"]["record_id"]
        .as_str()
        .expect("task ref should be present")
        .to_owned();
    core.update_scope(
        fixture.update_scope_request(UpdateScopeFixture {
            request_id: "req_mcp_replay_scope",
            idempotency_key: "idem_mcp_replay_scope",
            dry_run: false,
            expected_state_version: Some(1),
            task_id: &task_id,
            operation: ChangeUnitOperation::CreateCurrent,
            scope_summary: "MCP replay context scope.",
        }),
        invocation(&fixture, AccessClass::CoreMutation),
    )?;
    let change_unit_id = fixture
        .current_change_unit_id(&task_id)?
        .expect("Change Unit should be current");
    let request = fixture.prepare_write_request(
        "req_mcp_prepare_replay",
        "idem_mcp_prepare_replay",
        Some(2),
        Some(&task_id),
        Some(&change_unit_id),
    );

    let first = adapter(&fixture, AccessClass::WriteAuthorization).call_tool(
        "harness.prepare_write",
        serde_json::to_value(request.clone())?,
    )?;
    let after_first = fixture.counts()?;
    let write_authorization_id = first.response_value["write_authorization_ref"]["record_id"]
        .as_str()
        .expect("prepare_write should return an authorization id")
        .to_owned();

    let mismatch = adapter(&fixture, AccessClass::CoreMutation)
        .call_tool("harness.prepare_write", serde_json::to_value(request)?)?;

    assert_rejected_code(&mismatch.response_value, "LOCAL_ACCESS_MISMATCH");
    assert!(!mismatch.response_json.contains(&write_authorization_id));
    assert_eq!(fixture.counts()?, after_first);
    Ok(())
}

#[test]
fn registered_core_mutation_grant_rejects_requested_write_authorization(
) -> Result<(), Box<dyn Error>> {
    let fixture = CoreFixture::new("grant_reject")?;
    fixture.set_surface_local_access(json!({
        "access_class": "core_mutation",
        "authorized_access_classes": ["core_mutation"],
        "verification_basis": "integration_registration"
    }))?;
    let core = CoreService::new(fixture.runtime_home_path());

    let response = core.prepare_write(
        fixture.prepare_write_request("req_grant_reject", "idem_grant_reject", Some(0), None, None),
        invocation(&fixture, AccessClass::WriteAuthorization),
    )?;

    assert_rejected_code(&response.response_value, "LOCAL_ACCESS_MISMATCH");
    assert!(response.verified_surface.is_none());
    Ok(())
}

#[test]
fn capability_profile_cannot_override_registered_local_grant() -> Result<(), Box<dyn Error>> {
    let fixture = CoreFixture::new("capability_no_grant")?;
    fixture.set_surface_local_access(json!({
        "access_class": "core_mutation",
        "authorized_access_classes": ["core_mutation"],
        "verification_basis": "integration_registration"
    }))?;
    fixture.set_surface_capability(json!({
        "access_class": "write_authorization",
        "supported_access_classes": ["write_authorization"],
        "write_authorization": true
    }))?;
    let core = CoreService::new(fixture.runtime_home_path());

    let response = core.prepare_write(
        fixture.prepare_write_request("req_cap_no_grant", "idem_cap_no_grant", Some(0), None, None),
        invocation(&fixture, AccessClass::WriteAuthorization),
    )?;

    assert_rejected_code(&response.response_value, "LOCAL_ACCESS_MISMATCH");
    assert!(response.verified_surface.is_none());
    Ok(())
}

#[test]
fn matching_registered_grant_and_requested_access_succeeds() -> Result<(), Box<dyn Error>> {
    let fixture = CoreFixture::new("grant_match")?;
    fixture.set_surface_local_access(json!({
        "access_class": "read_status",
        "authorized_access_classes": ["read_status"],
        "verification_basis": "integration_registration"
    }))?;
    let core = CoreService::new(fixture.runtime_home_path());

    let response = core.status(
        fixture.status_request("req_grant_match", None),
        invocation(&fixture, AccessClass::ReadStatus),
    )?;

    assert_eq!(response.response_value["base"]["response_kind"], "result");
    let verified = response
        .verified_surface
        .as_ref()
        .expect("matching grant should create verified surface context");
    assert_eq!(verified.access_class, AccessClass::ReadStatus);
    assert_eq!(
        verified.verification_basis,
        "integration_registration; invocation_binding_basis=integration_fixture"
    );
    Ok(())
}

#[test]
fn unknown_surface_instance_is_rejected() -> Result<(), Box<dyn Error>> {
    let fixture = CoreFixture::new("unknown_instance")?;
    let core = CoreService::new(fixture.runtime_home_path());

    let response = core.status(
        fixture.status_request("req_unknown_instance", None),
        InvocationContext {
            surface_instance_id: Some(SurfaceInstanceId::new("missing_surface_instance")),
            requested_access_class: AccessClass::ReadStatus,
            invocation_binding_basis: "integration_fixture".to_owned(),
        },
    )?;

    assert_rejected_code(&response.response_value, "LOCAL_ACCESS_MISMATCH");
    assert!(response.verified_surface.is_none());
    Ok(())
}

#[test]
fn ambiguous_surface_id_without_usable_default_is_rejected() -> Result<(), Box<dyn Error>> {
    let fixture = CoreFixture::new("ambiguous_surface")?;
    for surface_instance_id in [
        "surface_instance_ambiguous_a",
        "surface_instance_ambiguous_b",
    ] {
        register_surface(
            fixture.runtime_home_path(),
            SurfaceRegistration {
                project_id: fixture.project_id().to_owned(),
                surface_id: "surface_ambiguous".to_owned(),
                surface_instance_id: surface_instance_id.to_owned(),
                surface_kind: "local_test".to_owned(),
                display_name: None,
                capability_profile_json: json!({}).to_string(),
                local_access_json: json!({
                    "access_class": "read_status",
                    "authorized_access_classes": ["read_status"],
                    "verification_basis": "integration_registration"
                })
                .to_string(),
                metadata_json: "{}".to_owned(),
            },
        )?;
    }
    let core = CoreService::new(fixture.runtime_home_path());
    let mut request = fixture.status_request("req_ambiguous_surface", None);
    request.envelope.surface_id = SurfaceId::new("surface_ambiguous");

    let response = core.status(
        request,
        InvocationContext {
            surface_instance_id: None,
            requested_access_class: AccessClass::ReadStatus,
            invocation_binding_basis: "integration_fixture".to_owned(),
        },
    )?;

    assert_rejected_code(&response.response_value, "LOCAL_ACCESS_MISMATCH");
    assert!(response.verified_surface.is_none());
    Ok(())
}

#[test]
fn malformed_local_access_document_fails_closed() -> Result<(), Box<dyn Error>> {
    let fixture = CoreFixture::new("malformed_grant")?;
    fixture.set_surface_local_access(json!({
        "authorized_access_classes": [],
        "verification_basis": "integration_registration"
    }))?;
    let core = CoreService::new(fixture.runtime_home_path());

    let response = core.status(
        fixture.status_request("req_malformed_grant", None),
        invocation(&fixture, AccessClass::ReadStatus),
    )?;

    assert_rejected_code(&response.response_value, "LOCAL_ACCESS_MISMATCH");
    assert!(response.verified_surface.is_none());
    Ok(())
}

#[test]
fn legacy_single_access_class_grant_remains_readable() -> Result<(), Box<dyn Error>> {
    let fixture = CoreFixture::new("legacy_grant")?;
    fixture.set_surface_local_access(json!({
        "access_class": "read_status"
    }))?;
    let core = CoreService::new(fixture.runtime_home_path());

    let response = core.status(
        fixture.status_request("req_legacy_grant", None),
        invocation(&fixture, AccessClass::ReadStatus),
    )?;

    assert_eq!(response.response_value["base"]["response_kind"], "result");
    let verified = response
        .verified_surface
        .as_ref()
        .expect("legacy grant should create verified surface context");
    assert_eq!(verified.access_class, AccessClass::ReadStatus);
    assert_eq!(
        verified.verification_basis,
        "registered_local_access; invocation_binding_basis=integration_fixture"
    );
    Ok(())
}

fn adapter(fixture: &CoreFixture, access_class: AccessClass) -> McpAdapter {
    McpAdapter::new(
        fixture.runtime_home_path(),
        McpSessionContext::new(access_class)
            .with_surface_instance_id(SurfaceInstanceId::new(fixture.surface_instance_id()))
            .with_verification_basis("integration_fixture"),
    )
}

fn invocation(fixture: &CoreFixture, access_class: AccessClass) -> InvocationContext {
    InvocationContext {
        surface_instance_id: Some(SurfaceInstanceId::new(fixture.surface_instance_id())),
        requested_access_class: access_class,
        invocation_binding_basis: "integration_fixture".to_owned(),
    }
}

fn assert_rejected_code(response: &Value, code: &str) {
    assert_eq!(response["base"]["response_kind"], "rejected");
    assert_eq!(response["errors"][0]["code"], code);
}
