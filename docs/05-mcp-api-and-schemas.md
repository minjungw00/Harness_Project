# MCP API And Schemas

## Document Role

This document owns the public MCP resources, public tools, common envelope, request and response schemas, error taxonomy, idempotency behavior, state conflict behavior, validator result schema, and artifact ref schema.

It does not own SQLite DDL, the full kernel transition table, projection template text, CLI command semantics, or connector cookbook details.

## API Scope

MCP resources are read-only. All state changes go through public tools and Core. A tool response may include projection paths and artifact refs, but those are references to state records or raw evidence files, not a replacement for canonical state.

Capability is not a first-class kernel gate. Surface capability appears through:

- the `surface_capability_check` validator
- `harness.prepare_write.response.blocked_reasons`
- guarantee display in status and write decisions

## MCP Resources

Resources expose current state and projection-oriented summaries without mutating state:

```text
harness://project/current
harness://project/surfaces
harness://task/active
harness://task/{task_id}
harness://task/{task_id}/summary
harness://task/{task_id}/spine
harness://task/{task_id}/reports/latest
harness://task/{task_id}/evidence-manifest
harness://task/{task_id}/bundle/current
harness://design/domain-language
harness://design/module-map
harness://design/interface-contracts
harness://policy/sensitive-categories
harness://status/card
```

Resource reads must not create Task records, decisions, projection jobs, or reconcile items. If a resource detects stale projection, it reports freshness; it does not repair it.

## Common Tool Envelope

Every public tool request carries an envelope. State-changing tools require a non-null `idempotency_key` and `expected_state_version`. Read-only tools accept the same envelope for tracing; they may set `expected_state_version` to `null`.

```yaml
ToolEnvelope:
  request_id: string
  idempotency_key: string | null
  expected_state_version: integer | null
  project_id: string
  task_id: string | null
  surface_id: string
  run_id: string | null
  actor_kind: user | lead_agent | evaluator | operator
  dry_run: boolean
```

Common response fields:

```yaml
ToolResponseBase:
  request_id: string
  idempotency_key: string | null
  project_id: string
  task_id: string | null
  state_version: integer
  dry_run: boolean
  errors: ToolError[]
  validator_results: ValidatorResult[]
  events: EventRef[]
  projection_jobs: ProjectionJobRef[]
```

`dry_run=true` validates and returns the transition plan but does not update current records, append to `state.sqlite.task_events`, register artifacts, or enqueue projection jobs.

## Shared Schemas

```yaml
EventRef:
  event_id: string
  event_type: string
  task_id: string | null
  state_version: integer

ProjectionJobRef:
  projection_job_id: string
  projection_kind: TASK | APR | RUN-SUMMARY | EVIDENCE-MANIFEST | EVAL | DIRECT-RESULT | MANUAL-QA | TDD-TRACE | DOMAIN-LANGUAGE | MODULE-MAP | INTERFACE-CONTRACT
  target_ref: string
  projection_version: integer

ToolError:
  code: ErrorCode
  message: string
  retryable: boolean
  details: object

StateSummary:
  mode: advisor | direct | work
  lifecycle_phase: intake | shaping | ready | executing | verifying | qa | waiting_user | blocked | completed | cancelled
  result: none | advice_only | passed | failed | cancelled
  close_reason: none | completed_verified | completed_self_checked | completed_with_risk_accepted | cancelled | superseded
  assurance_level: none | self_checked | detached_verified
  gates:
    scope_gate: not_required | required | pending | passed | failed | blocked
    approval_gate: not_required | required | pending | granted | denied | expired
    design_gate: not_required | required | pending | passed | partial | waived | stale | blocked
    evidence_gate: not_required | none | partial | sufficient | stale | blocked
    verification_gate: not_required | required | pending | passed | failed | waived_by_user | blocked
    qa_gate: not_required | required | pending | passed | failed | waived
    acceptance_gate: not_required | required | pending | accepted | rejected
```

Sensitive categories:

```text
auth_change
permission_model_change
schema_change
dependency_change
public_api_change
destructive_write
network_write
external_service_write
secret_access
production_config_change
ci_cd_change
infra_or_deployment_change
privacy_or_pii_change
data_export
telemetry_or_logging_change
license_or_compliance_change
billing_or_cost_change
model_or_prompt_policy_change
policy_override
```

## Artifact Ref Schema

An artifact ref points to a durable evidence file registered in the artifact store. Report projections and record projections use artifact refs when they need evidence-file references; the projection itself is not the evidence file.

```yaml
ArtifactRef:
  artifact_id: string
  kind: diff | log | screenshot | checkpoint | bundle | manifest | qa_capture | export_component | other
  uri: string
  sha256: string
  size_bytes: integer
  content_type: string
  redaction_state: none | redacted | secret_omitted | blocked
  task_id: string
  run_id: string | null
  created_at: string
  produced_by: lead_agent | evaluator | operator | harness
  retention_class: task | project | export | temporary
```

For the reference MVP, `uri` uses `harness-artifact://{project_id}/{artifact_id}`. The local file path is resolved through the per-project `artifacts` registry row in `state.sqlite`, not by trusting an absolute path in the API payload.

TODO_IMPLEMENT: Define the exact request-side staged artifact input shape for `record_run`, `launch_verify`, `record_eval`, and `record_manual_qa`. The committed response shape is `ArtifactRef`; the missing implementation detail is how a connector or operator supplies a staged file before Core registers it and returns the final `harness-artifact://` URI.

Record or projection references use `StateRecordRef`, not `ArtifactRef`:

```yaml
StateRecordRef:
  record_kind: task | change_unit | run | approval | decision_request | evidence_manifest | eval | manual_qa_record | tdd_trace | reconcile_item | projection
  record_id: string
  projection_path: string | null
```

## Validator Result Schema

```yaml
ValidatorResult:
  validator_id: string
  validator_kind: state | scope | approval | evidence | verification | qa | acceptance | design | artifact | projection | connector | capability
  status: passed | warning | failed | blocked | skipped
  guarantee_level: cooperative | detective | preventive | isolated
  checked_at: string
  target:
    task_id: string | null
    change_unit_id: string | null
    run_id: string | null
    artifact_id: string | null
  summary: string
  findings:
    - code: string
      severity: info | warning | error | blocker
      message: string
      path: string | null
      artifact_ref: ArtifactRef | null
  blocked_reasons: string[]
  suggested_next_action: string | null
```

The `surface_capability_check` validator uses this schema with `validator_kind=capability`.

## Error Taxonomy

| Code | Meaning |
|---|---|
| `STATE_CONFLICT` | `expected_state_version` is stale, lock ownership changed, or the same idempotency key was reused with a different payload |
| `NO_ACTIVE_TASK` | a Task is required but none is active or addressed |
| `NO_ACTIVE_CHANGE_UNIT` | a write-capable operation has no active scoped Change Unit |
| `SCOPE_REQUIRED` | scope confirmation is required before the requested write can proceed |
| `SCOPE_VIOLATION` | intended paths, tools, commands, network, secrets, or categories exceed scope |
| `APPROVAL_REQUIRED` | sensitive change requires approval before proceeding |
| `APPROVAL_DENIED` | the relevant approval was denied |
| `APPROVAL_EXPIRED` | approval expired or drifted from baseline/scope |
| `CAPABILITY_INSUFFICIENT` | the connected surface cannot satisfy a required validator or enforcement condition |
| `MCP_UNAVAILABLE` | required MCP access is unavailable or stale |
| `EVIDENCE_INSUFFICIENT` | required evidence coverage is absent, partial, stale, or blocked |
| `VERIFY_NOT_DETACHED` | verification cannot count as detached verification |
| `QA_REQUIRED` | required Manual QA is pending, failed, or missing |
| `ACCEPTANCE_REQUIRED` | required user acceptance is pending or rejected |
| `PROJECTION_STALE` | projection freshness is stale or failed for the requested action |
| `RECONCILE_REQUIRED` | human-editable or managed-block drift requires reconcile |
| `ARTIFACT_MISSING` | a referenced artifact file is missing or integrity check failed |
| `BASELINE_STALE` | baseline no longer matches the repository state required by the operation |
| `VALIDATOR_FAILED` | one or more required validators failed |

## Idempotency And State Conflict Behavior

Idempotency keys are scoped to `(project_id, tool_name, idempotency_key)`. Repeating the same payload with the same key returns the original committed response. Reusing a key with a different payload returns `STATE_CONFLICT`.

For state-changing tools, Core compares `expected_state_version` with current project/task state. A mismatch returns `STATE_CONFLICT` and includes the current state version and a status summary in `details`. The caller must refresh state and either retry with a new idempotency key or replay the exact previous request.

## Public Tools

### `harness.status`

Purpose: return project, surface, active Task, gate, guarantee, projection, and pending-decision status.

Allowed actor: `user`, `lead_agent`, `evaluator`, `operator`.

Request schema:

```yaml
StatusRequest:
  envelope: ToolEnvelope
  include:
    task: boolean
    gates: boolean
    projections: boolean
    pending_decisions: boolean
    guarantees: boolean
```

Response schema:

```yaml
StatusResponse:
  base: ToolResponseBase
  active_task: StateSummary | null
  status_card: string
  pending_decisions: StateRecordRef[]
  projection_freshness:
    status: current | stale | failed | unknown
    stale_refs: StateRecordRef[]
  guarantee_display:
    level: cooperative | detective | preventive | isolated
    notes: string[]
```

State transition summary: no state transition.

Events emitted: none.

Projection jobs enqueued: none.

Validators run: optional `surface_capability_check`, optional projection freshness read.

Possible errors: `MCP_UNAVAILABLE`, `PROJECTION_STALE`.

Idempotency behavior: read-only; repeated requests do not mutate state.

### `harness.intake`

Purpose: create or resume a Task from user intent and classify it as advisor, direct, or work.

Allowed actor: `user`, `lead_agent`, `operator`.

Request schema:

```yaml
IntakeRequest:
  envelope: ToolEnvelope
  user_request: string
  requested_mode: advisor | direct | work | auto
  resume_policy: resume_active | create_new | supersede_active | reject_if_active
  acceptance_criteria: string[]
  constraints:
    allowed_paths: string[]
    non_goals: string[]
    sensitive_categories: string[]
  initial_context_refs: StateRecordRef[]
```

Response schema:

```yaml
IntakeResponse:
  base: ToolResponseBase
  task_id: string
  created: boolean
  resumed: boolean
  state: StateSummary
  next_action: string
  change_unit_id: string | null
```

State transition summary: creates or resumes a Task; sets `mode` and initial `lifecycle_phase`; may create an initial Change Unit for write-capable direct/work.

Events emitted: `task_intake_recorded`, `task_created`, `task_resumed`, `task_superseded`, `change_unit_created`.

Projection jobs enqueued: `TASK`; optionally `DOMAIN-LANGUAGE`, `MODULE-MAP`, or `INTERFACE-CONTRACT` if intake accepted design support records.

Validators run: `state_envelope`, `active_task_policy`, `surface_capability_check`.

Possible errors: `STATE_CONFLICT`, `MCP_UNAVAILABLE`, `VALIDATOR_FAILED`, `CAPABILITY_INSUFFICIENT`.

Idempotency behavior: same key returns the same Task/resume decision; different payload with same key returns `STATE_CONFLICT`.

### `harness.next`

Purpose: return the next safe action, instruction bundle, and pending decisions for the current Task.

Allowed actor: `user`, `lead_agent`, `evaluator`, `operator`.

Request schema:

```yaml
NextRequest:
  envelope: ToolEnvelope
  task_id: string | null
  focus: status | shaping | implementation | verification | qa | acceptance | reconcile
  include_instruction_bundle: boolean
```

Response schema:

```yaml
NextResponse:
  base: ToolResponseBase
  state: StateSummary | null
  next_action:
    action_kind: ask_user | prepare_write | implement | launch_verify | record_eval | record_manual_qa | request_acceptance | close_task | reconcile | idle
    summary: string
    required_tool: string | null
  instruction_bundle:
    summary: string
    constraints: string[]
    relevant_refs: StateRecordRef[]
    artifact_refs: ArtifactRef[]
  pending_decisions: StateRecordRef[]
```

State transition summary: no state transition.

Events emitted: none.

Projection jobs enqueued: none.

Validators run: optional `surface_capability_check`, optional `docs_consistency`.

Possible errors: `NO_ACTIVE_TASK`, `MCP_UNAVAILABLE`, `PROJECTION_STALE`, `RECONCILE_REQUIRED`.

Idempotency behavior: read-only; repeated requests do not mutate state.

### `harness.prepare_write`

Purpose: decide whether an intended product write is allowed before the agent writes.

Allowed actor: `lead_agent`, `operator`.

Request schema:

```yaml
PrepareWriteRequest:
  envelope: ToolEnvelope
  task_id: string
  change_unit_id: string | null
  intended_operation: string
  intended_paths: string[]
  intended_tools: string[]
  intended_commands:
    - command: string
      command_class: string
      writes_product_files: boolean
  intended_network:
    - target: string
      direction: read | write
  intended_secrets:
    - secret_handle: string
      access_kind: read | write
  sensitive_categories: string[]
  baseline_ref: string | null
```

Response schema:

```yaml
PrepareWriteResponse:
  base: ToolResponseBase
  decision: allowed | blocked | approval_required | state_conflict
  state: StateSummary | null
  change_unit_id: string | null
  baseline_ref: string | null
  blocked_reasons:
    - code: string
      message: string
      related_error: ErrorCode
  approval_request_candidate: ApprovalRequestCandidate | null
  guarantee_display:
    level: cooperative | detective | preventive | isolated
    notes: string[]

ApprovalRequestCandidate:
  sensitive_categories: string[]
  allowed_paths: string[]
  allowed_tools: string[]
  allowed_network_targets: string[]
  secret_scope: string[]
  baseline_ref: string | null
```

`approval_request_candidate` is present only when `decision=approval_required` or when Core can suggest a new approval request. Otherwise it is `null`.

State transition summary: may move Task to `executing`, `waiting_user`, or `blocked`; may set `scope_gate=pending/blocked`, `approval_gate=pending/expired`, or stale evidence/approval markers.

Events emitted: `prepare_write_allowed`, `prepare_write_blocked`, `scope_required`, `approval_required`, `baseline_stale_detected`, `capability_insufficient_detected`.

Projection jobs enqueued: `TASK`; `APR` when approval is required.

Validators run: `state_envelope`, `active_task`, `active_change_unit`, `scope_coverage`, `changed_paths_intent`, `baseline_freshness`, `approval_scope`, `surface_capability_check`, design precondition validators that apply before write.

Possible errors: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `NO_ACTIVE_CHANGE_UNIT`, `SCOPE_REQUIRED`, `SCOPE_VIOLATION`, `APPROVAL_REQUIRED`, `APPROVAL_DENIED`, `APPROVAL_EXPIRED`, `BASELINE_STALE`, `CAPABILITY_INSUFFICIENT`, `MCP_UNAVAILABLE`, `VALIDATOR_FAILED`.

Idempotency behavior: repeated allowed/blocked decision with same payload returns the original decision and event refs; changed payload with same key returns `STATE_CONFLICT`.

### `harness.record_run`

Purpose: record shaping, implementation, direct-result, or verification-input run data, including artifacts and evidence updates.

Allowed actor: `lead_agent`, `evaluator`, `operator`.

Request schema:

```yaml
RecordRunRequest:
  envelope: ToolEnvelope
  kind: shaping_update | implementation | direct | verification_input
  task_id: string
  change_unit_id: string | null
  run_id: string | null
  baseline_ref: string | null
  summary: string
  artifact_refs: ArtifactRef[]
  payload: RecordRunPayload

RecordRunPayload:
  shaping_update: ShapingUpdatePayload | null
  implementation: ImplementationPayload | null
  direct: DirectPayload | null
  verification_input: VerificationInputPayload | null

ShapingUpdatePayload:
  task_summary_update: string | null
  acceptance_criteria_updates:
    - criteria_id: string | null
      operation: add | update | remove
      statement: string
  change_unit_updates:
    - operation: create | update | select_active | complete | defer | supersede
      change_unit_id: string | null
      title: string | null
      purpose: string | null
      non_goals: string[]
      slice_type: vertical | enabling | cleanup | horizontal-exception | null
      horizontal_exception_reason: string | null
      follow_up_vertical_change_unit_id: string | null
      allowed_paths: string[]
      allowed_tools: string[]
      allowed_commands: string[]
      allowed_network_targets: string[]
      secret_scope: string[]
      sensitive_categories: string[]
      validator_profile: string[]
      completion_conditions: string[]
      evaluator_focus: string[]
  design_record_refs: StateRecordRef[]
  pending_decision_refs: StateRecordRef[]

ImplementationPayload:
  observed_changes: ObservedChanges
  command_results: CommandResult[]
  evidence_updates: EvidenceUpdates
  tdd_trace_update: TddTraceUpdate | null

DirectPayload:
  observed_changes: ObservedChanges
  command_results: CommandResult[]
  evidence_updates: EvidenceUpdates
  self_check_summary: string
  escalation:
    value: none | escalate_to_work
    reason: string | null

VerificationInputPayload:
  evaluator_bundle_ref: ArtifactRef | null
  evaluator_focus: string[]
  observed_changes: ObservedChanges
  command_results: CommandResult[]

ObservedChanges:
  changed_paths: string[]
  created_paths: string[]
  deleted_paths: string[]

CommandResult:
  command: string
  exit_code: integer
  artifact_refs: ArtifactRef[]
  summary: string

EvidenceUpdates:
  acceptance_criteria:
    - criteria_id: string
      status: supported | unsupported | not_applicable
      supporting_refs: StateRecordRef[]
      artifact_refs: ArtifactRef[]

TddTraceUpdate:
  tdd_trace_id: string | null
  status: required | recorded | waived | not_required
  red_refs: ArtifactRef[]
  green_refs: ArtifactRef[]
  refactor_refs: ArtifactRef[]
  non_tdd_justification: string | null
```

The `payload` branch must match `kind`; all other branches must be `null` or absent. Change Unit creation and update for MVP happens through `kind=shaping_update` with `change_unit_updates`; `operation=create` creates a `change_units` record, and `operation=select_active` updates the Task's `active_change_unit_id`.

Response schema:

```yaml
RecordRunResponse:
  base: ToolResponseBase
  run_id: string
  state: StateSummary
  evidence_manifest_ref: StateRecordRef | null
  run_summary_ref: StateRecordRef | null
  direct_result_ref: StateRecordRef | null
  registered_artifacts: ArtifactRef[]
  next_action: string
```

State transition summary: shaping updates can keep `shaping`, move to `ready`, or move to `waiting_user`; implementation moves toward `verifying`; direct can become close-eligible or escalate to work; verification input records evaluator bundle context without proving detached verification.

Events emitted: `run_recorded`, `shaping_updated`, `implementation_recorded`, `direct_result_recorded`, `verification_input_recorded`, `evidence_manifest_updated`, `artifact_registered`, `tdd_trace_updated`.

Projection jobs enqueued: `TASK`, `RUN-SUMMARY`, `EVIDENCE-MANIFEST`; `DIRECT-RESULT` for `kind=direct`; `TDD-TRACE` when updated.

Validators run: `state_envelope`, `changed_paths`, `scope_coverage`, `approval_scope`, `baseline_freshness`, `artifact_integrity`, `evidence_sufficiency`, applicable design-quality validators, `surface_capability_check`.

Possible errors: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `NO_ACTIVE_CHANGE_UNIT`, `SCOPE_VIOLATION`, `APPROVAL_REQUIRED`, `APPROVAL_EXPIRED`, `ARTIFACT_MISSING`, `BASELINE_STALE`, `EVIDENCE_INSUFFICIENT`, `VALIDATOR_FAILED`, `CAPABILITY_INSUFFICIENT`, `MCP_UNAVAILABLE`.

Idempotency behavior: repeated request returns the same run, artifact records, evidence updates, events, and projection jobs; artifact refs must match the original payload.

### `harness.request_user_decision`

Purpose: create a structured user decision request.

Allowed actor: `lead_agent`, `evaluator`, `operator`.

Request schema:

```yaml
RequestUserDecisionRequest:
  envelope: ToolEnvelope
  decision_kind: approval | scope_confirmation | design_choice | qa_waiver | acceptance | reconcile
  task_id: string
  change_unit_id: string | null
  prompt: string
  options:
    - option_id: string
      label: string
      consequence: string
  recommendation: string | null
  expires_at: string | null
  approval_scope: ApprovalScope | null
  reconcile_item_id: string | null

ApprovalScope:
  sensitive_categories: string[]
  allowed_paths: string[]
  allowed_tools: string[]
  allowed_commands: string[]
  allowed_network_targets: string[]
  secret_scope: string[]
  baseline_ref: string | null
```

`approval_scope` is required when `decision_kind=approval`. For all other `decision_kind` values it must be `null` or omitted.

Response schema:

```yaml
RequestUserDecisionResponse:
  base: ToolResponseBase
  decision_request_id: string
  approval_id: string | null
  reconcile_item_id: string | null
  state: StateSummary
  user_visible_summary: string
```

`pending_decisions` returned by status and next-action responses contain `StateRecordRef` entries with `record_kind=decision_request`.

State transition summary: records a pending decision and usually moves Task to `waiting_user`; approval requests set `approval_gate=pending`; scope confirmation sets `scope_gate=pending`; acceptance sets `acceptance_gate=pending`.

Events emitted: `user_decision_requested`, `approval_requested`, `scope_confirmation_requested`, `design_choice_requested`, `qa_waiver_requested`, `acceptance_requested`, `reconcile_decision_requested`.

Projection jobs enqueued: `TASK`; `APR` for approval; affected projection for reconcile.

Validators run: `state_envelope`, `decision_request_validity`, `approval_scope` for approval decisions, `reconcile_required` for reconcile decisions.

Possible errors: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `NO_ACTIVE_CHANGE_UNIT`, `SCOPE_REQUIRED`, `APPROVAL_REQUIRED`, `RECONCILE_REQUIRED`, `PROJECTION_STALE`, `VALIDATOR_FAILED`, `MCP_UNAVAILABLE`.

Idempotency behavior: repeated request returns the same pending decision record; a different prompt/scope/options with the same key returns `STATE_CONFLICT`.

### `harness.record_user_decision`

Purpose: record the user's answer to a pending approval, scope, design, QA waiver, acceptance, or reconcile decision.

Allowed actor: `user`, `operator`.

Request schema:

```yaml
RecordUserDecisionRequest:
  envelope: ToolEnvelope
  decision_request_id: string
  decision_kind: approval | scope_confirmation | design_choice | qa_waiver | acceptance | reconcile
  selected_option_id: string
  decision: RecordUserDecisionPayload
  note: string
  waiver_reason: string | null

RecordUserDecisionPayload:
  approval:
    value: granted | denied | expired
  scope_confirmation:
    value: confirmed | rejected | revise_scope
  design_choice:
    value: selected | rejected | defer
  qa_waiver:
    value: waived | rejected
  acceptance:
    value: accepted | rejected
  reconcile:
    value: merge | reject | convert_to_note | create_decision | defer
```

The payload branch must match `decision_kind`; other branches must be absent.

Response schema:

```yaml
RecordUserDecisionResponse:
  base: ToolResponseBase
  decision_request_id: string
  state: StateSummary
  updated_records: StateRecordRef[]
  next_action: string
```

State transition summary: updates the targeted gate or reconcile item; approval grant/deny updates `approval_gate`; accepted scope updates `scope_gate`; QA waiver updates `qa_gate`; acceptance updates `acceptance_gate`; reconcile may create accepted state records.

Events emitted: `user_decision_recorded`, `approval_granted`, `approval_denied`, `scope_confirmed`, `scope_rejected`, `design_choice_recorded`, `qa_waiver_recorded`, `acceptance_recorded`, `reconcile_resolved`.

Projection jobs enqueued: `TASK`; `APR` for approval; `MANUAL-QA` for QA waiver when represented as a QA record; affected design/task projections for reconcile.

Validators run: `state_envelope`, `pending_decision_exists`, `approval_scope`, `qa_waiver_reason`, `reconcile_target_validity`.

Possible errors: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `APPROVAL_DENIED`, `APPROVAL_EXPIRED`, `SCOPE_VIOLATION`, `QA_REQUIRED`, `ACCEPTANCE_REQUIRED`, `RECONCILE_REQUIRED`, `PROJECTION_STALE`, `VALIDATOR_FAILED`, `MCP_UNAVAILABLE`.

Idempotency behavior: repeated decision returns the same updated records and events; attempting to change an already-recorded decision with the same key returns `STATE_CONFLICT`.

### `harness.launch_verify`

Purpose: create a detached verification run or manual evaluator bundle.

Allowed actor: `lead_agent`, `operator`.

Request schema:

```yaml
LaunchVerifyRequest:
  envelope: ToolEnvelope
  task_id: string
  change_unit_id: string | null
  verification_mode: fresh_session | fresh_worktree | sandbox | manual_bundle
  evaluator_surface_id: string | null
  baseline_ref: string
  include_artifacts: ArtifactRef[]
  evaluator_focus: string[]
```

Response schema:

```yaml
LaunchVerifyResponse:
  base: ToolResponseBase
  evaluator_run_id: string | null
  bundle_ref: ArtifactRef
  state: StateSummary
  evaluator_instructions: string
  independence_expected:
    context: fresh_session | fresh_worktree | sandbox | manual_bundle
    write_capable: boolean
```

State transition summary: records verification launch, sets or keeps `verification_gate=pending`, and creates evaluator run/bundle references.

Events emitted: `verification_launched`, `verification_bundle_created`, `evaluator_run_created`.

Projection jobs enqueued: `TASK`; optionally `EVIDENCE-MANIFEST`.

Validators run: `state_envelope`, `evidence_sufficiency`, `baseline_freshness`, `artifact_integrity`, `surface_capability_check`, `same_session_verify_guard`.

Possible errors: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `EVIDENCE_INSUFFICIENT`, `BASELINE_STALE`, `ARTIFACT_MISSING`, `CAPABILITY_INSUFFICIENT`, `MCP_UNAVAILABLE`, `VALIDATOR_FAILED`.

Idempotency behavior: repeated request returns the same evaluator run and bundle ref; bundle contents must be byte-identical for the same key.

### `harness.record_eval`

Purpose: record a verification result and update verification gate/assurance when independence is valid.

Allowed actor: `evaluator`, `operator`.

Request schema:

```yaml
RecordEvalRequest:
  envelope: ToolEnvelope
  task_id: string
  evaluator_run_id: string | null
  target_run_id: string | null
  verdict: passed | failed | blocked | inconclusive
  checks_performed:
    - check_id: string
      result: passed | failed | skipped | blocked
      summary: string
  evidence_reviewed:
    state_refs: StateRecordRef[]
    artifact_refs: ArtifactRef[]
  independence:
    context: same_session | subagent_context | fresh_session | fresh_worktree | sandbox | manual_bundle
    write_capable: boolean
    baseline_reverified: boolean
    evaluator_surface_id: string
    parent_run_id: string | null
  blockers: string[]
  artifact_refs: ArtifactRef[]
```

Response schema:

```yaml
RecordEvalResponse:
  base: ToolResponseBase
  eval_id: string
  state: StateSummary
  assurance_updated: boolean
  eval_ref: StateRecordRef
  registered_artifacts: ArtifactRef[]
  next_action: string
```

State transition summary: records Eval; passed detached verification can set `verification_gate=passed` and `assurance_level=detached_verified`; failed or blocked Eval moves gate to failed/blocked; same-session or invalid independence cannot upgrade assurance.

Events emitted: `eval_recorded`, `verification_passed`, `verification_failed`, `verification_blocked`, `assurance_updated`, `verify_not_detached_detected`.

Projection jobs enqueued: `TASK`, `EVAL`; optionally `EVIDENCE-MANIFEST`.

Validators run: `state_envelope`, `same_session_verify_guard`, `baseline_freshness`, `artifact_integrity`, `evidence_sufficiency`, `approval_scope`, `surface_capability_check`.

Possible errors: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `VERIFY_NOT_DETACHED`, `EVIDENCE_INSUFFICIENT`, `BASELINE_STALE`, `ARTIFACT_MISSING`, `VALIDATOR_FAILED`, `CAPABILITY_INSUFFICIENT`, `MCP_UNAVAILABLE`.

Idempotency behavior: repeated request returns the same Eval and assurance decision; a changed verdict or independence payload with the same key returns `STATE_CONFLICT`.

### `harness.record_manual_qa`

Purpose: record human QA result and update `qa_gate` when required QA is satisfied, failed, or waived.

Allowed actor: `user`, `operator`, `evaluator`.

Request schema:

```yaml
RecordManualQaRequest:
  envelope: ToolEnvelope
  task_id: string
  qa_profile: ui_quality | workflow | copy | accessibility | browser_smoke | performance_smoke | other
  performed_by: string
  result: passed | failed | waived
  findings:
    - severity: info | warning | error | blocker
      summary: string
      path: string | null
  artifact_refs: ArtifactRef[]
  waiver_reason: string | null
  next_action: rework | accept | waive | block | none
```

Response schema:

```yaml
RecordManualQaResponse:
  base: ToolResponseBase
  manual_qa_record_id: string
  state: StateSummary
  manual_qa_ref: StateRecordRef
  registered_artifacts: ArtifactRef[]
  next_action: string
```

State transition summary: records Manual QA; `passed` can set `qa_gate=passed`; `failed` sets `qa_gate=failed` and routes to rework/blocked; `waived` requires waiver reason and sets `qa_gate=waived`.

Events emitted: `manual_qa_recorded`, `qa_passed`, `qa_failed`, `qa_waived`, `artifact_registered`.

Projection jobs enqueued: `TASK`, `MANUAL-QA`; optionally `EVIDENCE-MANIFEST`.

Validators run: `state_envelope`, `manual_qa_required`, `qa_waiver_reason`, `artifact_integrity`, `evidence_sufficiency`.

Possible errors: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `QA_REQUIRED`, `ARTIFACT_MISSING`, `EVIDENCE_INSUFFICIENT`, `VALIDATOR_FAILED`, `MCP_UNAVAILABLE`.

Idempotency behavior: repeated request returns the same Manual QA record and gate update; waiver reason and artifacts must match.

### `harness.close_task`

Purpose: close, cancel, or supersede a Task after Core checks all close-relevant gates.

Allowed actor: `user`, `lead_agent`, `operator`.

Request schema:

```yaml
CloseTaskRequest:
  envelope: ToolEnvelope
  task_id: string
  intent: complete | cancel | supersede
  requested_close_reason: completed_verified | completed_self_checked | completed_with_risk_accepted | cancelled | superseded
  user_note: string | null
  superseded_by_task_id: string | null
```

Response schema:

```yaml
CloseTaskResponse:
  base: ToolResponseBase
  closed: boolean
  state: StateSummary
  blockers:
    - code: ErrorCode
      message: string
      required_next_action: string
  final_report_refs: StateRecordRef[]
  artifact_refs: ArtifactRef[]
```

State transition summary: successful completion moves Task to `completed` with result and close reason; cancellation/supersession moves Task to `cancelled`; failed close leaves Task non-terminal and reports blockers.

Events emitted: `close_requested`, `task_closed`, `task_cancelled`, `task_superseded`, `close_blocked`.

Projection jobs enqueued: `TASK`; latest required reports as needed for final freshness.

Validators run: `state_envelope`, `active_run_absent`, `active_change_unit_complete`, `scope_coverage`, `approval_scope`, `design_gate_close`, `evidence_sufficiency`, `same_session_verify_guard`, `manual_qa_required`, `acceptance_required`, `projection_freshness`.

Possible errors: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `NO_ACTIVE_CHANGE_UNIT`, `SCOPE_REQUIRED`, `SCOPE_VIOLATION`, `APPROVAL_REQUIRED`, `APPROVAL_DENIED`, `APPROVAL_EXPIRED`, `EVIDENCE_INSUFFICIENT`, `VERIFY_NOT_DETACHED`, `QA_REQUIRED`, `ACCEPTANCE_REQUIRED`, `PROJECTION_STALE`, `RECONCILE_REQUIRED`, `ARTIFACT_MISSING`, `BASELINE_STALE`, `VALIDATOR_FAILED`, `MCP_UNAVAILABLE`.

Idempotency behavior: repeated successful close returns the same terminal state and report refs; a second close with a different intent or close reason returns `STATE_CONFLICT`.
