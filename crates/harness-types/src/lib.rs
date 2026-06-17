#![forbid(unsafe_code)]

//! Shared Rust type boundary for Harness public API and domain-shaped values.
//!
//! This crate contains serde models, controlled API value sets, opaque string
//! identifier wrappers, and deterministic canonical JSON hashing helpers. It
//! does not implement Core behavior, storage effects, CLI behavior, or MCP
//! adapter behavior.

pub mod canonical;
pub mod ids;
pub mod methods;
pub mod schema;
pub mod values;

pub use canonical::*;
pub use ids::*;
pub use methods::*;
pub use schema::*;
pub use values::*;

/// High-level placement marker for shared type groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeBoundary {
    /// API-facing Rust types live behind this boundary.
    Api,
    /// Core/domain Rust types live behind this boundary.
    Domain,
}

impl TypeBoundary {
    /// Returns a stable implementation-facing label for the boundary marker.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Api => "api",
            Self::Domain => "domain",
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::*;

    #[test]
    fn boundary_labels_are_stable() {
        assert_eq!(TypeBoundary::Api.label(), "api");
        assert_eq!(TypeBoundary::Domain.label(), "domain");
    }

    #[test]
    fn tool_envelope_round_trips_documented_field_names() {
        let envelope: ToolEnvelope = serde_json::from_value(json!({
            "project_id": "proj_onboard_001",
            "task_id": null,
            "actor_kind": "agent",
            "surface_id": "surface_onboard",
            "request_id": "req_intake_onboard_001",
            "idempotency_key": "idem_intake_onboard_001",
            "expected_state_version": 17,
            "dry_run": false,
            "locale": "en-US"
        }))
        .expect("documented envelope example should deserialize");

        assert_eq!(envelope.project_id.as_str(), "proj_onboard_001");
        assert_eq!(envelope.actor_kind, ActorKind::Agent);
        assert_eq!(
            envelope
                .idempotency_key
                .as_ref()
                .map(IdempotencyKey::as_str),
            Some("idem_intake_onboard_001")
        );

        let encoded = serde_json::to_value(&envelope).expect("envelope should serialize");
        assert_eq!(encoded["project_id"], "proj_onboard_001");
        assert_eq!(encoded["actor_kind"], "agent");
        assert_eq!(encoded["task_id"], Value::Null);
    }

    #[test]
    fn stage_artifact_result_serializes_documented_shape() {
        let result = StageArtifactResult {
            base: ToolResultBase {
                response_kind: ResponseKind::Result,
                effect_kind: EffectKind::StagingCreated,
                dry_run: false,
                state_version: Some(42),
                events: vec![],
            },
            staged_artifact_handle: StagedArtifactHandle {
                handle_id: StagedArtifactHandleId::new("staged_trace_log_001"),
                project_id: ProjectId::new("proj_trace_001"),
                task_id: TaskId::new("task_trace_001"),
                created_by_surface_id: SurfaceId::new("surface_artifact"),
                created_by_surface_instance_id: SurfaceInstanceId::new("surface_instance_trace_01"),
                content_type: "text/plain".to_owned(),
                sha256: "sha256:example-trace".to_owned(),
                size_bytes: 42,
                redaction_state: RedactionState::None,
                expires_at: "<future-expiration-timestamp>".to_owned(),
                consumed: false,
            },
            expires_at: "<future-expiration-timestamp>".to_owned(),
        };

        let encoded = serde_json::to_value(&result).expect("result should serialize");

        assert_eq!(encoded["base"]["response_kind"], "result");
        assert_eq!(encoded["base"]["effect_kind"], "staging_created");
        assert_eq!(encoded["staged_artifact_handle"]["redaction_state"], "none");
        assert_eq!(
            encoded["staged_artifact_handle"]["created_by_surface_instance_id"],
            "surface_instance_trace_01"
        );

        let decoded: StageArtifactResult =
            serde_json::from_value(encoded).expect("result should deserialize");
        assert!(!decoded.staged_artifact_handle.consumed);
        assert_eq!(decoded.staged_artifact_handle.size_bytes, 42);
    }

    #[test]
    fn record_user_judgment_request_keeps_payload_branches_as_objects() {
        let request: RecordUserJudgmentRequest = serde_json::from_value(json!({
            "envelope": envelope_json("user"),
            "user_judgment_id": "uj_empty_001",
            "judgment_kind": "product_decision",
            "selected_option_id": "keep",
            "answer": {
                "product_decision": {
                    "judgment": {
                        "decision": "accepted",
                        "rationale": "The illustration is suitable."
                    }
                },
                "technical_decision": null,
                "scope_decision": null,
                "sensitive_action_scope": null,
                "final_acceptance": null,
                "residual_risk_acceptance": null,
                "cancellation": null
            },
            "note": null,
            "accepted_risks": []
        }))
        .expect("judgment request should deserialize");

        assert_eq!(request.judgment_kind, JudgmentKind::ProductDecision);
        assert_eq!(request.selected_option_id.as_str(), "keep");
        assert!(request.answer.product_decision.is_some());
        assert!(request.answer.sensitive_action_scope.is_none());

        let encoded = serde_json::to_value(&request).expect("judgment request should serialize");
        assert_eq!(
            encoded["answer"]["product_decision"]["judgment"]["decision"],
            "accepted"
        );
        assert_eq!(encoded["answer"]["technical_decision"], Value::Null);
    }

    #[test]
    fn method_local_reason_codes_remain_strings() {
        let reason: WriteDecisionReason = serde_json::from_value(json!({
            "category": "sensitive_approval",
            "code": "sensitive_approval_missing",
            "message": "Approval is required.",
            "related_refs": []
        }))
        .expect("write decision reason should deserialize");

        assert_eq!(reason.category, WriteDecisionCategory::SensitiveApproval);
        assert_eq!(reason.code, "sensitive_approval_missing");

        let blocker: CloseReadinessBlocker = serde_json::from_value(json!({
            "category": "final_acceptance",
            "code": "missing_final_acceptance",
            "message": "Final acceptance is required.",
            "related_refs": [],
            "next_actions": []
        }))
        .expect("close blocker should deserialize");

        assert_eq!(
            blocker.category,
            CloseReadinessBlockerCategory::FinalAcceptance
        );
        assert_eq!(blocker.code, "missing_final_acceptance");
    }

    #[test]
    fn canonical_json_hash_is_order_stable() {
        let first = json!({
            "z": 3,
            "a": {
                "b": true,
                "a": [2, 1]
            }
        });
        let second = json!({
            "a": {
                "a": [2, 1],
                "b": true
            },
            "z": 3
        });

        let canonical = canonical_json_string(&first).expect("canonical JSON should serialize");
        assert_eq!(canonical, r#"{"a":{"a":[2,1],"b":true},"z":3}"#);

        let first_hash = canonical_request_hash(&first).expect("hash should compute");
        let second_hash = canonical_request_hash(&second).expect("hash should compute");

        assert_eq!(first_hash, second_hash);
        assert_eq!(
            first_hash.as_str(),
            "sha256:22b1cca5763ebd5996581c6551cea0c733f4267c2fb26da60176f1bcac3ca5de"
        );
    }

    fn envelope_json(actor_kind: &str) -> Value {
        json!({
            "project_id": "proj_empty_001",
            "task_id": "task_empty_001",
            "actor_kind": actor_kind,
            "surface_id": "surface_empty",
            "request_id": "req_empty_answer_001",
            "idempotency_key": "idem_empty_answer_001",
            "expected_state_version": 62,
            "dry_run": false,
            "locale": "en-US"
        })
    }
}
