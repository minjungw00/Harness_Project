<a id="harnessrecord_user_judgment"></a>

# `harness.record_user_judgment` reference

## What this document owns

This document owns baseline method behavior for `harness.record_user_judgment`:

- method-specific required inputs, access requirements, state-version behavior, result branches, and dry-run behavior
- recording the user's answer to one existing pending `UserJudgment`
- method-specific boundaries for resolving, rejecting, deferring, blocking, or marking that pending user-owned judgment
- the minimal request and representative response for an account data export confirmation scenario

## What this document does not own

This document does not own:

- common `ToolEnvelope`, `ToolResultBase`, `ToolRejectedResponse`, or `ToolDryRunResponse` schema bodies
- `UserJudgment`, `RecordUserJudgmentPayload`, `SensitiveActionScope`, `AcceptedRiskInput`, value-set, or status field definitions
- Core user-owned judgment meaning, final acceptance meaning, residual-risk meaning, sensitive-action approval meaning, or `Write Authorization` meaning
- storage record layouts, exact storage effects, public error code meaning, or public error precedence

## Purpose

`harness.record_user_judgment` records the user's answer to one existing pending `UserJudgment`.

The method updates the addressed pending judgment according to the user's answer. It does not broaden the answer into unrelated approval, current scope expansion, final acceptance, residual-risk acceptance, sensitive-action approval, or `Write Authorization`.

## Required inputs

- `ToolEnvelope` with non-null `idempotency_key` and current `expected_state_version` for non-dry-run commits.
- `user_judgment_id` for an existing pending judgment.
- Matching `judgment_kind`.
- `selected_option_id`, `answer`, `note`, and `accepted_risks`.
- An `answer` containing only the decision-specific payload branch for the pending `judgment_kind`.

`selected_option_id` and `note` stay at request level. `RecordUserJudgmentPayload` must not repeat them inside the decision-specific answer branch.

Shared field shapes for `UserJudgmentResolution`, `RecordUserJudgmentPayload`, `SensitiveActionScope`, and `AcceptedRiskInput` are owned by [API Judgment Schemas](schema-judgment.md).

## Access requirements

The method requires `VerifiedSurfaceContext.access_class=core_mutation` and `verified=true`.

The pending judgment must belong to the same project and compatible `Task` selected by the request. Local access failures, unreadable judgment identity, and insufficient local capability reject before commit.

## State-version behavior

Committed `dry_run=false` result:

- increments `project_state.state_version` exactly once
- updates the addressed `user_judgments` row
- may update dependent blocker or summary state only as allowed by the storage-effect owner

Non-claims:

- Dry run and rejection create no judgment resolution, blocker update, event, replay row, or state-version increment.
- A recorded `scope_decision` does not silently change current scope or current Change Unit records. Those records still require the scope owner-defined transition, such as `harness.update_scope`.

## Success result

Returns `RecordUserJudgmentResult` with:

- `base.response_kind=result`
- `base.effect_kind=core_committed`
- `user_judgment_ref`
- updated `user_judgment`
- `updated_refs`
- current `state`
- `next_actions`

## Committed judgment outcomes

The method may commit the addressed judgment as `resolved`, `rejected`, `deferred`, `blocked`, or another supported judgment status when that status is the user's answer or the compatible result of the focused judgment.

The result updates only covered blockers and judgment-dependent summaries. It does not create unrelated approvals, evidence, scope updates, `Write Authorization`, close state, or residual-risk acceptance beyond the recorded judgment itself.

## Rejected result

Returns `ToolRejectedResponse` for pre-commit failures, including:

- stale `expected_state_version`
- unknown or non-pending judgment
- `judgment_kind` mismatch
- invalid selected option
- invalid answer payload
- expired pending judgment
- answer incompatible with the pending judgment
- local access failure
- validator failure

Public error code meaning is owned by [API error codes](error-codes.md). Public error precedence is owned by [API error precedence](error-precedence.md).

## Dry-run behavior

For `dry_run=true`, a valid preview returns `ToolDryRunResponse`. Branch shape is owned by [API Schema Core](schema-core.md); no-effect persistence semantics are owned by [Storage Effects](../storage-effects.md).

The preview must not resolve the judgment, update blockers, append events, create replay rows, or increment state version.

## Storage effect

On commit, the method may persist judgment resolution and dependent blocker or summary state. Exact storage effects are owned by [Storage Effects](../storage-effects.md#harnessrecord_user_judgment).

## Example

Example preconditions:

- `uj_001` is a pending `product_decision` judgment for `task_456`.
- The pending judgment was created at project `state_version` `22`.
- The user selects the `accept` option because the account data export confirmation copy is sufficient.

### Minimal valid request

```yaml
method: harness.record_user_judgment
params:
  envelope:
    project_id: proj_123
    task_id: task_456
    actor_kind: user
    surface_id: surface_local
    request_id: req_judgment_answer_001
    idempotency_key: idem_judgment_answer_001
    expected_state_version: 22
    dry_run: false
    locale: en-US
  user_judgment_id: uj_001
  judgment_kind: product_decision
  selected_option_id: accept
  answer:
    product_decision:
      judgment:
        decision: accepted
        rationale: "The account data export confirmation copy clearly warns that the account data export file may include personal data."
    technical_decision: null
    scope_decision: null
    sensitive_action_scope: null
    final_acceptance: null
    residual_risk_acceptance: null
    cancellation: null
  note: null
  accepted_risks: []
```

### Representative response

Result branch (`RecordUserJudgmentResult`, committed):

```yaml
base:
  response_kind: result
  effect_kind: core_committed
  dry_run: false
  state_version: 23
  events:
    - event_id: evt_1006
      event_kind: user_judgment_recorded
user_judgment_ref:
  record_kind: user_judgment
  record_id: uj_001
  project_id: proj_123
  task_id: task_456
  state_version: 23
user_judgment:
  judgment_id: uj_001
  project_id: proj_123
  task_id: task_456
  change_unit_id: cu_001
  judgment_kind: product_decision
  status: resolved
  presentation: short
  question: "Should the account data export confirmation copy that warns users the account data export file may include personal data be accepted as sufficient?"
  options:
    - option_id: accept
      label: "Sufficient"
      description: "Record the user-owned judgment that the account data export confirmation copy is sufficient."
      consequence: "Close readiness can evaluate the product decision as resolved."
      is_default: true
    - option_id: revise
      label: "Revise"
      description: "Keep the Task open for revised account data export confirmation copy."
      consequence: "Close remains blocked on the product decision."
      is_default: false
  context:
    summary: "The account data export confirmation copy shown before download warns that the account data export file may include personal data."
    related_refs: []
    artifact_refs: []
    visible_risks: []
    constraints:
      - "Account data export flow and account data export confirmation tests remain in scope; account deletion behavior remains out of scope."
  affected_refs:
    - record_kind: task
      record_id: task_456
      project_id: proj_123
      task_id: task_456
      state_version: 21
  required_for: close
  resolution:
    selected_option_id: accept
    answer:
      product_decision:
        judgment:
          decision: accepted
          rationale: "The account data export confirmation copy clearly warns that the account data export file may include personal data."
    note: null
    accepted_risks: []
    resolved_by_actor_kind: user
  expires_at: null
  created_at: "<example-created-at>"
  resolved_at: "<example-resolved-at>"
updated_refs:
  - record_kind: user_judgment
    record_id: uj_001
    project_id: proj_123
    task_id: task_456
    state_version: 23
state:
  project_id: proj_123
  state_version: 23
next_actions:
  - action: harness.close_task
    reason: "Evaluate close readiness after recording the user's product decision."
```

## Owner links

- Request envelope, response branches, and dry-run summaries: [API Schema Core](schema-core.md).
- `UserJudgment`, `RecordUserJudgmentPayload`, `SensitiveActionScope`, and `AcceptedRiskInput`: [API Judgment Schemas](schema-judgment.md).
- State refs and summaries: [API State Schemas](schema-state.md).
- Judgment values and supported method-local values: [API Value Sets](schema-value-sets.md).
- User-owned judgment, final acceptance, residual-risk acceptance, and non-substitution rules: [Core Model](../core-model.md).
- Exact storage effects: [Storage Effects](../storage-effects.md#harnessrecord_user_judgment).
- Public errors: [API error codes](error-codes.md) and [API error precedence](error-precedence.md).
- Creating the pending judgment request: [`harness.request_user_judgment`](method-request-user-judgment.md).
