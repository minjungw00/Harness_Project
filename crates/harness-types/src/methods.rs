use serde::{Deserialize, Serialize};

use crate::ids::{
    BaselineRef, ChangeUnitId, RunId, TaskId, UserJudgmentId, UserJudgmentOptionId,
    WriteAuthorizationId,
};
use crate::schema::{
    AcceptedRiskInput, ArtifactInput, ArtifactRef, CloseReadinessBlocker, EvidenceCoverageItem,
    EvidenceSummary, GuaranteeDisplay, JsonObject, NextActionSummary, ObservedChanges,
    RecordUserJudgmentPayload, RunSummary, StagedArtifactHandle, StateRecordRef, StateSummary,
    ToolEnvelope, ToolResponse, ToolResultBase, UserJudgment, UserJudgmentCandidate,
    UserJudgmentContext, UserJudgmentOption, WriteAuthorizationSummary, WriteDecisionReason,
};
use crate::values::{
    AuthorizationEffect, ChangeUnitOperation, CloseIntent, CloseReason, CloseState, JudgmentKind,
    JudgmentPresentation, JudgmentRequiredFor, PrepareWriteDecision, RedactionState, RequestedMode,
    ResumePolicy, RunKind, StatusCloseState,
};

/// Response branch type for `harness.intake`.
pub type IntakeResponse = ToolResponse<IntakeResult>;

/// Response branch type for `harness.update_scope`.
pub type UpdateScopeResponse = ToolResponse<UpdateScopeResult>;

/// Response branch type for `harness.status`.
pub type StatusResponse = ToolResponse<StatusResult>;

/// Response branch type for `harness.prepare_write`.
pub type PrepareWriteResponse = ToolResponse<PrepareWriteResult>;

/// Response branch type for `harness.stage_artifact`.
pub type StageArtifactResponse = ToolResponse<StageArtifactResult>;

/// Response branch type for `harness.record_run`.
pub type RecordRunResponse = ToolResponse<RecordRunResult>;

/// Response branch type for `harness.request_user_judgment`.
pub type RequestUserJudgmentResponse = ToolResponse<RequestUserJudgmentResult>;

/// Response branch type for `harness.record_user_judgment`.
pub type RecordUserJudgmentResponse = ToolResponse<RecordUserJudgmentResult>;

/// Response branch type for `harness.close_task`.
pub type CloseTaskResponse = ToolResponse<CloseTaskResult>;

/// `harness.intake` request params.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntakeRequest {
    pub envelope: ToolEnvelope,
    pub plain_language_request: String,
    pub requested_mode: RequestedMode,
    pub resume_policy: ResumePolicy,
    pub initial_scope: InitialScope,
    pub initial_context_refs: Vec<StateRecordRef>,
}

/// Intake initial scope object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitialScope {
    pub boundary: String,
    pub non_goals: Vec<String>,
    pub acceptance_criteria: Vec<String>,
}

/// `harness.intake` method result branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntakeResult {
    pub base: ToolResultBase,
    pub task_ref: StateRecordRef,
    pub change_unit_ref: Option<StateRecordRef>,
    pub state: StateSummary,
    pub next_actions: Vec<NextActionSummary>,
}

/// `harness.update_scope` request params.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateScopeRequest {
    pub envelope: ToolEnvelope,
    pub task_id: TaskId,
    pub goal_summary: Option<String>,
    pub scope_update: Option<ScopeUpdate>,
    pub scope_boundary: Option<String>,
    pub non_goals: Option<Vec<String>>,
    pub acceptance_criteria: Option<Vec<String>>,
    pub autonomy_boundary: Option<String>,
    pub baseline_ref: Option<BaselineRef>,
    pub change_unit: ChangeUnitUpdate,
    pub related_scope_decision_refs: Vec<StateRecordRef>,
}

/// Include/exclude scope-update object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScopeUpdate {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

/// Change Unit update object. Additional method-owned fields remain object data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeUnitUpdate {
    pub operation: ChangeUnitOperation,
    #[serde(flatten)]
    pub fields: JsonObject,
}

/// `harness.update_scope` method result branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateScopeResult {
    pub base: ToolResultBase,
    pub task_ref: StateRecordRef,
    pub change_unit_ref: Option<StateRecordRef>,
    pub linked_scope_decision_refs: Vec<StateRecordRef>,
    pub stale_write_authorization_refs: Vec<StateRecordRef>,
    pub blocker_refs: Vec<StateRecordRef>,
    pub state: StateSummary,
    pub next_actions: Vec<NextActionSummary>,
}

/// `harness.status` request params.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusRequest {
    pub envelope: ToolEnvelope,
    pub include: StatusInclude,
}

/// Status include flags shown by the method owner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusInclude {
    pub task: bool,
    pub pending_user_judgments: bool,
    pub write_authority: bool,
    pub evidence: bool,
    pub close: bool,
    pub guarantees: bool,
}

/// `harness.status` method result branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusResult {
    pub base: ToolResultBase,
    pub active_task: Option<StateSummary>,
    pub status_summary: String,
    pub next_actions: Vec<NextActionSummary>,
    pub pending_user_judgments: Vec<StateRecordRef>,
    pub blocker_refs: Vec<StateRecordRef>,
    pub close_state: StatusCloseState,
    pub close_blockers: Vec<CloseReadinessBlocker>,
    pub guarantee_display: Option<GuaranteeDisplay>,
}

/// `harness.prepare_write` request params.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrepareWriteRequest {
    pub envelope: ToolEnvelope,
    pub task_id: Option<TaskId>,
    pub change_unit_id: Option<ChangeUnitId>,
    pub intended_operation: String,
    pub intended_paths: Vec<String>,
    pub product_file_write_intended: bool,
    pub sensitive_categories: Vec<String>,
    pub baseline_ref: BaselineRef,
}

/// `harness.prepare_write` method result branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrepareWriteResult {
    pub base: ToolResultBase,
    pub decision: PrepareWriteDecision,
    pub state: Option<StateSummary>,
    pub write_authorization_ref: Option<StateRecordRef>,
    pub write_authorization: Option<WriteAuthorizationSummary>,
    pub authorization_effect: AuthorizationEffect,
    pub active_user_judgment_refs: Vec<StateRecordRef>,
    pub write_decision_reasons: Vec<WriteDecisionReason>,
    pub user_judgment_candidate: Option<UserJudgmentCandidate>,
    pub guarantee_display: Option<GuaranteeDisplay>,
}

/// `harness.stage_artifact` request params.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageArtifactRequest {
    pub envelope: ToolEnvelope,
    pub task_id: TaskId,
    pub display_name: String,
    pub content_type: String,
    pub redaction_state: RedactionState,
    pub safe_bytes_or_notice: String,
    pub expected_sha256: Option<String>,
    pub expected_size_bytes: Option<u64>,
    pub relation_hint: Option<String>,
}

/// `harness.stage_artifact` method result branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageArtifactResult {
    pub base: ToolResultBase,
    pub staged_artifact_handle: StagedArtifactHandle,
    pub expires_at: String,
}

/// `harness.record_run` request params.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordRunRequest {
    pub envelope: ToolEnvelope,
    pub task_id: TaskId,
    pub change_unit_id: ChangeUnitId,
    pub kind: RunKind,
    pub run_id: Option<RunId>,
    pub baseline_ref: BaselineRef,
    pub write_authorization_id: Option<WriteAuthorizationId>,
    pub summary: String,
    pub observed_changes: ObservedChanges,
    pub artifact_inputs: Vec<ArtifactInput>,
    pub evidence_updates: Vec<EvidenceCoverageItem>,
}

/// `harness.record_run` method result branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordRunResult {
    pub base: ToolResultBase,
    pub run_summary: RunSummary,
    pub registered_artifacts: Vec<ArtifactRef>,
    pub evidence_summary: Option<EvidenceSummary>,
    pub blocker_refs: Vec<StateRecordRef>,
    pub state: StateSummary,
}

/// `harness.request_user_judgment` request params.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestUserJudgmentRequest {
    pub envelope: ToolEnvelope,
    pub task_id: TaskId,
    pub change_unit_id: Option<ChangeUnitId>,
    pub judgment_kind: JudgmentKind,
    pub presentation: JudgmentPresentation,
    pub question: String,
    pub options: Vec<UserJudgmentOption>,
    pub context: UserJudgmentContext,
    pub affected_refs: Vec<StateRecordRef>,
    pub required_for: JudgmentRequiredFor,
    pub expires_at: Option<String>,
}

/// `harness.request_user_judgment` method result branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestUserJudgmentResult {
    pub base: ToolResultBase,
    pub user_judgment_ref: StateRecordRef,
    pub user_judgment: UserJudgment,
    pub blocker_refs: Vec<StateRecordRef>,
    pub state: StateSummary,
}

/// `harness.record_user_judgment` request params.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordUserJudgmentRequest {
    pub envelope: ToolEnvelope,
    pub user_judgment_id: UserJudgmentId,
    pub judgment_kind: JudgmentKind,
    pub selected_option_id: UserJudgmentOptionId,
    pub answer: RecordUserJudgmentPayload,
    pub note: Option<String>,
    pub accepted_risks: Vec<AcceptedRiskInput>,
}

/// `harness.record_user_judgment` method result branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordUserJudgmentResult {
    pub base: ToolResultBase,
    pub user_judgment_ref: StateRecordRef,
    pub user_judgment: UserJudgment,
    pub updated_refs: Vec<StateRecordRef>,
    pub state: StateSummary,
    pub next_actions: Vec<NextActionSummary>,
}

/// `harness.close_task` request params.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseTaskRequest {
    pub envelope: ToolEnvelope,
    pub task_id: TaskId,
    pub intent: CloseIntent,
    pub close_reason: Option<CloseReason>,
    pub superseding_task_id: Option<TaskId>,
    pub user_note: Option<String>,
}

/// `harness.close_task` method result branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseTaskResult {
    pub base: ToolResultBase,
    pub close_state: CloseState,
    pub state: StateSummary,
    pub blockers: Vec<CloseReadinessBlocker>,
    pub evidence_summary: Option<EvidenceSummary>,
    pub artifact_refs: Vec<ArtifactRef>,
}
