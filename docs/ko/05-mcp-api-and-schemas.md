# MCP API와 스키마

## 문서 역할

이 문서는 public MCP resource, public tool, common envelope, request/response schema, error taxonomy, idempotency behavior, state conflict behavior, validator result schema, artifact ref schema를 담당한다.

SQLite DDL, full kernel transition table, projection template text, CLI command semantic, connector cookbook detail은 담당하지 않는다.

## API 범위

MCP resource는 read-only다. 모든 state change는 public tool과 Core를 통해 이뤄진다. Tool response는 projection path와 artifact ref를 포함할 수 있지만, 이는 state record 또는 raw evidence file에 대한 reference일 뿐 canonical state의 대체물이 아니다.

Capability는 first-class kernel gate가 아니다. Surface capability는 다음을 통해 나타난다.

- `surface_capability_check` validator
- `harness.prepare_write.response.blocked_reasons`
- status와 write decision의 guarantee display

## MCP Resource

Resource는 state를 mutate하지 않고 current state와 projection-oriented summary를 expose한다.

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

Resource read는 Task record, decision, projection job, reconcile item을 만들면 안 된다. Resource가 stale projection을 detect하면 freshness를 report할 뿐 repair하지 않는다.

## Common Tool Envelope

모든 public tool request는 envelope을 가진다. State-changing tool에는 non-null `idempotency_key`와 `expected_state_version`이 필요하다. Read-only tool도 tracing을 위해 같은 envelope을 받을 수 있으며, `expected_state_version`을 `null`로 설정할 수 있다.

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

공통 response field:

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

`dry_run=true`는 validate하고 transition plan을 반환하지만 current record update, `state.sqlite.task_events` append, artifact register, projection job enqueue는 수행하지 않는다.

## Shared Schema

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

Sensitive category:

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

Artifact ref는 artifact store에 registered된 durable evidence file을 가리킨다. Report projection과 record projection은 evidence-file reference가 필요할 때 artifact ref를 사용한다. Projection 자체는 evidence file이 아니다.

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

Reference MVP에서 `uri`는 `harness-artifact://{project_id}/{artifact_id}`를 사용한다. Local file path는 API payload의 absolute path를 신뢰하는 것이 아니라 `state.sqlite`의 per-project `artifacts` registry row를 통해 resolve한다.

Evidence를 create하거나 attach하는 request는 `ArtifactInput`을 사용한다. Request는 existing committed artifact를 reference하거나, Core가 validate, register, `ArtifactRef`로 반환할 staged file을 제공할 수 있다.

```yaml
ArtifactInput:
  input_id: string
  source_kind: staged_file | existing_artifact
  existing_artifact_ref: ArtifactRef | null
  staged: StagedArtifactSource | null
  kind: diff | log | screenshot | checkpoint | bundle | manifest | qa_capture | export_component | other
  redaction_state: none | redacted | secret_omitted | blocked
  produced_by: lead_agent | evaluator | operator | harness
  retention_class: task | project | export | temporary
  relation:
    task_id: string
    run_id: string | null
    record_kind: run | eval | manual_qa_record | verification_bundle | export | other
    record_id_hint: string | null
  description: string | null

StagedArtifactSource:
    staged_uri: string
    display_name: string | null
    content_type: string
    expected_sha256: string | null
    expected_size_bytes: integer | null
```

규칙:

- `source_kind=existing_artifact`는 `existing_artifact_ref`가 필요하고 `staged`를 `null`로 설정해야 한다.
- `source_kind=staged_file`은 `staged`가 필요하고 `existing_artifact_ref`를 `null`로 설정해야 한다.
- Existing artifact를 new record에 attach할 때 Core는 artifact의 task relation을 verify하고 incompatible reuse를 reject한다.
- `staged_uri`는 arbitrary absolute path가 아니라 harness staging location 또는 approved capture adapter를 가리켜야 한다.
- `expected_sha256` 또는 `expected_size_bytes`가 있으면 Core는 commit 전에 stored bytes를 verify한다.
- Core는 final storage 전에 redaction rule을 적용하고 committed artifact를 `ArtifactRef`로 기록한다.
- Tool response는 committed `ArtifactRef` 값을 `registered_artifacts`, `bundle_ref`, 기타 response field에 반환한다.

Record 또는 projection reference는 `ArtifactRef`가 아니라 `StateRecordRef`를 사용한다.

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

`surface_capability_check` validator는 이 schema를 `validator_kind=capability`로 사용한다.

## Error Taxonomy

| Code | Meaning |
|---|---|
| `STATE_CONFLICT` | `expected_state_version`이 stale이거나, lock ownership이 changed되었거나, 같은 idempotency key가 다른 payload로 reuse됨 |
| `NO_ACTIVE_TASK` | Task가 required인데 active 또는 addressed Task가 없음 |
| `NO_ACTIVE_CHANGE_UNIT` | write-capable operation에 active scoped Change Unit이 없음 |
| `SCOPE_REQUIRED` | requested write 진행 전에 scope confirmation이 필요함 |
| `SCOPE_VIOLATION` | intended path, tool, command, network, secret, category가 scope를 초과함 |
| `APPROVAL_REQUIRED` | sensitive change 진행 전에 approval 필요 |
| `APPROVAL_DENIED` | relevant approval이 denied됨 |
| `APPROVAL_EXPIRED` | approval이 expired되었거나 baseline/scope에서 drift됨 |
| `CAPABILITY_INSUFFICIENT` | connected surface가 required validator 또는 enforcement condition을 satisfy할 수 없음 |
| `MCP_UNAVAILABLE` | required MCP access가 unavailable 또는 stale |
| `EVIDENCE_INSUFFICIENT` | required evidence coverage가 absent, partial, stale, blocked |
| `VERIFY_NOT_DETACHED` | verification이 detached verification으로 count될 수 없음 |
| `QA_REQUIRED` | required Manual QA가 pending, failed, missing |
| `ACCEPTANCE_REQUIRED` | required user acceptance가 pending 또는 rejected |
| `PROJECTION_STALE` | requested action에 필요한 projection freshness가 stale 또는 failed |
| `RECONCILE_REQUIRED` | human-editable 또는 managed-block drift에 reconcile 필요 |
| `ARTIFACT_MISSING` | referenced artifact file이 missing이거나 integrity check failed |
| `BASELINE_STALE` | baseline이 operation에 필요한 repository state와 더 이상 match하지 않음 |
| `VALIDATOR_FAILED` | 하나 이상의 required validator failed |

## Idempotency And State Conflict Behavior

Idempotency key는 `(project_id, tool_name, idempotency_key)` 범위에 속한다. 같은 key와 같은 payload를 반복하면 original committed response를 반환한다. 같은 key를 다른 payload로 reuse하면 `STATE_CONFLICT`를 반환한다.

State-changing tool에서 Core는 `expected_state_version`을 current project/task state와 비교한다. Mismatch는 `STATE_CONFLICT`를 반환하고 `details`에 current state version과 status summary를 포함한다. Caller는 state를 refresh한 뒤 새 idempotency key로 retry하거나 정확히 같은 previous request를 replay해야 한다.

## Public Tool

### `harness.status`

목적: project, surface, active Task, gate, guarantee, projection, pending-decision status를 반환한다.

허용 actor: `user`, `lead_agent`, `evaluator`, `operator`.

요청 schema:

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

응답 schema:

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

State transition 요약: state transition 없음.

발행 Event: 없음.

Projection job enqueue: 없음.

실행 Validator: optional `surface_capability_check`, optional projection freshness read.

가능한 오류: `MCP_UNAVAILABLE`, `PROJECTION_STALE`.

Idempotency 동작: read-only; repeated request는 state를 mutate하지 않는다.

### `harness.intake`

목적: user intent에서 Task를 create 또는 resume하고 advisor, direct, work로 classify한다.

허용 actor: `user`, `lead_agent`, `operator`.

요청 schema:

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

응답 schema:

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

State transition 요약: Task를 create 또는 resume한다. `mode`와 initial `lifecycle_phase`를 set한다. Write-capable direct/work에는 initial Change Unit을 만들 수 있다.

발행 Event: `task_intake_recorded`, `task_created`, `task_resumed`, `task_superseded`, `change_unit_created`.

Projection job enqueue: `TASK`; intake가 design support record를 accepted한 경우 optional `DOMAIN-LANGUAGE`, `MODULE-MAP`, `INTERFACE-CONTRACT`.

실행 Validator: `state_envelope`, `active_task_policy`, `surface_capability_check`.

가능한 오류: `STATE_CONFLICT`, `MCP_UNAVAILABLE`, `VALIDATOR_FAILED`, `CAPABILITY_INSUFFICIENT`.

Idempotency 동작: same key는 same Task/resume decision을 반환한다. Same key with different payload는 `STATE_CONFLICT`를 반환한다.

### `harness.next`

목적: current Task의 next safe action, instruction bundle, pending decision을 반환한다.

허용 actor: `user`, `lead_agent`, `evaluator`, `operator`.

요청 schema:

```yaml
NextRequest:
  envelope: ToolEnvelope
  task_id: string | null
  focus: status | shaping | implementation | verification | qa | acceptance | reconcile
  include_instruction_bundle: boolean
```

응답 schema:

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

State transition 요약: state transition 없음.

발행 Event: 없음.

Projection job enqueue: 없음.

실행 Validator: optional `surface_capability_check`, optional `docs_consistency`.

가능한 오류: `NO_ACTIVE_TASK`, `MCP_UNAVAILABLE`, `PROJECTION_STALE`, `RECONCILE_REQUIRED`.

Idempotency 동작: read-only; repeated request는 state를 mutate하지 않는다.

### `harness.prepare_write`

목적: agent가 write하기 전에 intended product write가 allowed인지 결정한다.

허용 actor: `lead_agent`, `operator`.

요청 schema:

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

응답 schema:

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

`approval_request_candidate`는 `decision=approval_required`이거나 Core가 new approval request를 suggest할 수 있을 때만 present하다. 그 외에는 `null`이다.

State transition 요약: Task를 `executing`, `waiting_user`, `blocked`로 이동시킬 수 있다. `scope_gate=pending/blocked`, `approval_gate=pending/expired`, stale evidence/approval marker를 set할 수 있다.

발행 Event: `prepare_write_allowed`, `prepare_write_blocked`, `scope_required`, `approval_required`, `baseline_stale_detected`, `capability_insufficient_detected`.

Projection job enqueue: `TASK`; approval required 시 `APR`.

실행 Validator: `state_envelope`, `active_task`, `active_change_unit`, `scope_coverage`, `changed_paths_intent`, `baseline_freshness`, `approval_scope`, `surface_capability_check`, write 전에 적용되는 design precondition validator.

가능한 오류: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `NO_ACTIVE_CHANGE_UNIT`, `SCOPE_REQUIRED`, `SCOPE_VIOLATION`, `APPROVAL_REQUIRED`, `APPROVAL_DENIED`, `APPROVAL_EXPIRED`, `BASELINE_STALE`, `CAPABILITY_INSUFFICIENT`, `MCP_UNAVAILABLE`, `VALIDATOR_FAILED`.

Idempotency 동작: 같은 payload의 repeated allowed/blocked decision은 original decision과 event ref를 반환한다. Same key with changed payload는 `STATE_CONFLICT`를 반환한다.

### `harness.record_run`

목적: artifact와 evidence update를 포함해 shaping, implementation, direct-result, verification-input run data를 기록한다.

허용 actor: `lead_agent`, `evaluator`, `operator`.

요청 schema:

```yaml
RecordRunRequest:
  envelope: ToolEnvelope
  kind: shaping_update | implementation | direct | verification_input
  task_id: string
  change_unit_id: string | null
  run_id: string | null
  baseline_ref: string | null
  summary: string
  artifact_inputs: ArtifactInput[]
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
  evaluator_bundle_input: ArtifactInput | null
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
  artifact_inputs: ArtifactInput[]
  summary: string

EvidenceUpdates:
  acceptance_criteria:
    - criteria_id: string
      status: supported | unsupported | not_applicable
      supporting_refs: StateRecordRef[]
      artifact_inputs: ArtifactInput[]

TddTraceUpdate:
  tdd_trace_id: string | null
  status: required | recorded | waived | not_required
  red_inputs: ArtifactInput[]
  green_inputs: ArtifactInput[]
  refactor_inputs: ArtifactInput[]
  non_tdd_justification: string | null
```

`payload` branch는 `kind`와 match해야 하며, 다른 branch는 `null` 또는 absent여야 한다. `ArtifactInput` 값은 같은 Core transaction 중 resolve된다. Response field에는 committed `ArtifactRef` 값이 들어간다. MVP에서 Change Unit creation/update는 `kind=shaping_update`의 `change_unit_updates`를 통해 일어난다. `operation=create`는 `change_units` record를 만들고, `operation=select_active`는 Task의 `active_change_unit_id`를 update한다.

응답 schema:

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

State transition 요약: shaping update는 `shaping`을 유지하거나 `ready`, `waiting_user`로 이동할 수 있다. Implementation은 `verifying`으로 향한다. Direct는 close-eligible이 되거나 work로 escalate될 수 있다. Verification input은 detached verification을 증명하지 않고 evaluator bundle context를 기록한다.

발행 Event: `run_recorded`, `shaping_updated`, `implementation_recorded`, `direct_result_recorded`, `verification_input_recorded`, `evidence_manifest_updated`, `artifact_registered`, `tdd_trace_updated`.

Projection job enqueue: `TASK`, `RUN-SUMMARY`, `EVIDENCE-MANIFEST`; `kind=direct`에는 `DIRECT-RESULT`; updated 시 `TDD-TRACE`.

실행 Validator: `state_envelope`, `changed_paths`, `scope_coverage`, `approval_scope`, `baseline_freshness`, `artifact_integrity`, `evidence_sufficiency`, applicable design-quality validators, `surface_capability_check`.

가능한 오류: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `NO_ACTIVE_CHANGE_UNIT`, `SCOPE_VIOLATION`, `APPROVAL_REQUIRED`, `APPROVAL_EXPIRED`, `ARTIFACT_MISSING`, `BASELINE_STALE`, `EVIDENCE_INSUFFICIENT`, `VALIDATOR_FAILED`, `CAPABILITY_INSUFFICIENT`, `MCP_UNAVAILABLE`.

Idempotency 동작: repeated request는 same run, artifact record, evidence update, event, projection job을 반환한다. Artifact input과 resolved artifact ref는 original payload와 match해야 한다.

### `harness.request_user_decision`

목적: structured user decision request를 생성한다.

허용 actor: `lead_agent`, `evaluator`, `operator`.

요청 schema:

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

`decision_kind=approval`이면 `approval_scope`가 required다. 다른 모든 `decision_kind` 값에서는 `null` 또는 omitted여야 한다.

응답 schema:

```yaml
RequestUserDecisionResponse:
  base: ToolResponseBase
  decision_request_id: string
  approval_id: string | null
  reconcile_item_id: string | null
  state: StateSummary
  user_visible_summary: string
```

Status와 next-action response가 반환하는 `pending_decisions`에는 `record_kind=decision_request`인 `StateRecordRef` entry가 들어간다.

State transition 요약: pending decision을 기록하고 보통 Task를 `waiting_user`로 이동시킨다. Approval request는 `approval_gate=pending`을 set한다. Scope confirmation은 `scope_gate=pending`을 set한다. Acceptance는 `acceptance_gate=pending`을 set한다.

발행 Event: `user_decision_requested`, `approval_requested`, `scope_confirmation_requested`, `design_choice_requested`, `qa_waiver_requested`, `acceptance_requested`, `reconcile_decision_requested`.

Projection job enqueue: `TASK`; approval에는 `APR`; reconcile에는 affected projection.

실행 Validator: `state_envelope`, `decision_request_validity`, approval decision에는 `approval_scope`, reconcile decision에는 `reconcile_required`.

가능한 오류: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `NO_ACTIVE_CHANGE_UNIT`, `SCOPE_REQUIRED`, `APPROVAL_REQUIRED`, `RECONCILE_REQUIRED`, `PROJECTION_STALE`, `VALIDATOR_FAILED`, `MCP_UNAVAILABLE`.

Idempotency 동작: repeated request는 same pending decision record를 반환한다. Same key에서 prompt/scope/options가 다르면 `STATE_CONFLICT`를 반환한다.

### `harness.record_user_decision`

목적: pending approval, scope, design, QA waiver, acceptance, reconcile decision에 대한 user answer를 기록한다.

허용 actor: `user`, `operator`.

요청 schema:

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

Payload branch는 `decision_kind`와 match해야 하며, 다른 branch는 absent여야 한다.

응답 schema:

```yaml
RecordUserDecisionResponse:
  base: ToolResponseBase
  decision_request_id: string
  state: StateSummary
  updated_records: StateRecordRef[]
  next_action: string
```

State transition 요약: targeted gate 또는 reconcile item을 update한다. Approval grant/deny는 `approval_gate`를 update한다. Accepted scope는 `scope_gate`를 update한다. QA waiver는 `qa_gate`를 update한다. Acceptance는 `acceptance_gate`를 update한다. Reconcile은 accepted state record를 만들 수 있다.

발행 Event: `user_decision_recorded`, `approval_granted`, `approval_denied`, `scope_confirmed`, `scope_rejected`, `design_choice_recorded`, `qa_waiver_recorded`, `acceptance_recorded`, `reconcile_resolved`.

Projection job enqueue: `TASK`; approval에는 `APR`; QA waiver가 QA record로 represented되면 `MANUAL-QA`; reconcile에는 affected design/task projection.

실행 Validator: `state_envelope`, `pending_decision_exists`, `approval_scope`, `qa_waiver_reason`, `reconcile_target_validity`.

가능한 오류: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `APPROVAL_DENIED`, `APPROVAL_EXPIRED`, `SCOPE_VIOLATION`, `QA_REQUIRED`, `ACCEPTANCE_REQUIRED`, `RECONCILE_REQUIRED`, `PROJECTION_STALE`, `VALIDATOR_FAILED`, `MCP_UNAVAILABLE`.

Idempotency 동작: repeated decision은 same updated record와 event를 반환한다. Same key로 already-recorded decision을 바꾸려 하면 `STATE_CONFLICT`를 반환한다.

### `harness.launch_verify`

목적: detached verification run 또는 manual evaluator bundle을 생성한다.

허용 actor: `lead_agent`, `operator`.

요청 schema:

```yaml
LaunchVerifyRequest:
  envelope: ToolEnvelope
  task_id: string
  change_unit_id: string | null
  verification_mode: fresh_session | fresh_worktree | sandbox | manual_bundle
  evaluator_surface_id: string | null
  baseline_ref: string
  include_artifacts: ArtifactRef[]
  bundle_artifact_input: ArtifactInput | null
  evaluator_focus: string[]
```

`include_artifacts`는 bundle 안에 include하거나 bundle에서 link할 already registered evidence를 reference한다. `bundle_artifact_input`은 optional이다. `null`이면 Core가 verification bundle을 assemble/register한다. Present이면 Core가 supplied staged bundle을 validate/register한다.

응답 schema:

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

State transition 요약: verification launch를 기록하고, `verification_gate=pending`을 set/keep하며 evaluator run/bundle reference를 만든다.

발행 Event: `verification_launched`, `verification_bundle_created`, `evaluator_run_created`.

Projection job enqueue: `TASK`; optional `EVIDENCE-MANIFEST`.

실행 Validator: `state_envelope`, `evidence_sufficiency`, `baseline_freshness`, `artifact_integrity`, `surface_capability_check`, `same_session_verify_guard`.

가능한 오류: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `EVIDENCE_INSUFFICIENT`, `BASELINE_STALE`, `ARTIFACT_MISSING`, `CAPABILITY_INSUFFICIENT`, `MCP_UNAVAILABLE`, `VALIDATOR_FAILED`.

Idempotency 동작: repeated request는 same evaluator run과 bundle ref를 반환한다. Included artifact ref와 bundle artifact input은 original payload와 match해야 하고, staged bundle content는 같은 key에 대해 byte-identical이어야 한다.

### `harness.record_eval`

목적: verification result를 기록하고 independence가 valid할 때 verification gate/assurance를 update한다.

허용 actor: `evaluator`, `operator`.

요청 schema:

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
  artifact_inputs: ArtifactInput[]
```

응답 schema:

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

State transition 요약: Eval을 기록한다. Passed detached verification은 `verification_gate=passed`와 `assurance_level=detached_verified`를 set할 수 있다. Failed 또는 blocked Eval은 gate를 failed/blocked로 이동시킨다. Same-session 또는 invalid independence는 assurance를 upgrade할 수 없다.

발행 Event: `eval_recorded`, `verification_passed`, `verification_failed`, `verification_blocked`, `assurance_updated`, `verify_not_detached_detected`.

Projection job enqueue: `TASK`, `EVAL`; optional `EVIDENCE-MANIFEST`.

실행 Validator: `state_envelope`, `same_session_verify_guard`, `baseline_freshness`, `artifact_integrity`, `evidence_sufficiency`, `approval_scope`, `surface_capability_check`.

가능한 오류: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `VERIFY_NOT_DETACHED`, `EVIDENCE_INSUFFICIENT`, `BASELINE_STALE`, `ARTIFACT_MISSING`, `VALIDATOR_FAILED`, `CAPABILITY_INSUFFICIENT`, `MCP_UNAVAILABLE`.

Idempotency 동작: repeated request는 same Eval과 assurance decision을 반환한다. Same key에 changed verdict, independence payload, artifact input이 있으면 `STATE_CONFLICT`를 반환한다.

### `harness.record_manual_qa`

목적: human QA result를 기록하고 required QA가 satisfied, failed, waived일 때 `qa_gate`를 update한다.

허용 actor: `user`, `operator`, `evaluator`.

요청 schema:

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
  artifact_inputs: ArtifactInput[]
  waiver_reason: string | null
  next_action: rework | accept | waive | block | none
```

응답 schema:

```yaml
RecordManualQaResponse:
  base: ToolResponseBase
  manual_qa_record_id: string
  state: StateSummary
  manual_qa_ref: StateRecordRef
  registered_artifacts: ArtifactRef[]
  next_action: string
```

State transition 요약: Manual QA를 기록한다. `passed`는 `qa_gate=passed`를 set할 수 있다. `failed`는 `qa_gate=failed`를 set하고 rework/blocked로 route한다. `waived`는 waiver reason이 필요하며 `qa_gate=waived`를 set한다.

발행 Event: `manual_qa_recorded`, `qa_passed`, `qa_failed`, `qa_waived`, `artifact_registered`.

Projection job enqueue: `TASK`, `MANUAL-QA`; optional `EVIDENCE-MANIFEST`.

실행 Validator: `state_envelope`, `manual_qa_required`, `qa_waiver_reason`, `artifact_integrity`, `evidence_sufficiency`.

가능한 오류: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `QA_REQUIRED`, `ARTIFACT_MISSING`, `EVIDENCE_INSUFFICIENT`, `VALIDATOR_FAILED`, `MCP_UNAVAILABLE`.

Idempotency 동작: repeated request는 same Manual QA record와 gate update를 반환한다. Waiver reason과 artifact input은 match해야 한다.

### `harness.close_task`

목적: Core가 close-relevant gate를 모두 확인한 뒤 Task를 close, cancel, supersede한다.

허용 actor: `user`, `lead_agent`, `operator`.

요청 schema:

```yaml
CloseTaskRequest:
  envelope: ToolEnvelope
  task_id: string
  intent: complete | cancel | supersede
  requested_close_reason: completed_verified | completed_self_checked | completed_with_risk_accepted | cancelled | superseded
  user_note: string | null
  superseded_by_task_id: string | null
```

응답 schema:

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

State transition 요약: successful completion은 Task를 `completed`로 이동시키고 result와 close reason을 설정한다. Cancellation/supersession은 Task를 `cancelled`로 이동시킨다. Failed close는 Task를 non-terminal 상태로 남기고 blocker를 report한다.

발행 Event: `close_requested`, `task_closed`, `task_cancelled`, `task_superseded`, `close_blocked`.

Projection job enqueue: `TASK`; final freshness에 필요한 latest required report.

실행 Validator: `state_envelope`, `active_run_absent`, `active_change_unit_complete`, `scope_coverage`, `approval_scope`, `design_gate_close`, `evidence_sufficiency`, `same_session_verify_guard`, `manual_qa_required`, `acceptance_required`, `projection_freshness`.

가능한 오류: `STATE_CONFLICT`, `NO_ACTIVE_TASK`, `NO_ACTIVE_CHANGE_UNIT`, `SCOPE_REQUIRED`, `SCOPE_VIOLATION`, `APPROVAL_REQUIRED`, `APPROVAL_DENIED`, `APPROVAL_EXPIRED`, `EVIDENCE_INSUFFICIENT`, `VERIFY_NOT_DETACHED`, `QA_REQUIRED`, `ACCEPTANCE_REQUIRED`, `PROJECTION_STALE`, `RECONCILE_REQUIRED`, `ARTIFACT_MISSING`, `BASELINE_STALE`, `VALIDATOR_FAILED`, `MCP_UNAVAILABLE`.

Idempotency 동작: repeated successful close는 same terminal state와 report ref를 반환한다. Different intent 또는 close reason으로 두 번째 close를 요청하면 `STATE_CONFLICT`를 반환한다.
