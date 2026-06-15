<a id="harnessrequest_user_judgment"></a>

# `harness.request_user_judgment` reference

## What this document owns

This document owns baseline method behavior for `harness.request_user_judgment`:

- method-specific required inputs, access requirements, state-version behavior, result branches, and dry-run behavior
- creation of one pending `UserJudgment` for a focused user-owned judgment
- the minimal request and representative response for an account data export confirmation scenario

## What this document does not own

This document does not own:

- common `ToolEnvelope`, `ToolResultBase`, `ToolRejectedResponse`, or `ToolDryRunResponse` schema bodies
- `UserJudgment`, option, context, answer payload, value-set, or status field definitions
- Core user-owned judgment meaning, final acceptance meaning, residual-risk meaning, sensitive-action approval meaning, or `Write Authorization` meaning
- storage record layouts, exact storage effects, public error code meaning, or public error precedence

## Purpose

`harness.request_user_judgment` creates one pending `UserJudgment` for a focused user-owned judgment. It asks the user; the agent must not answer, infer, broaden, or decide the judgment for the user.

The pending judgment is a request for a decision. It is not the decision itself, does not create evidence, does not change current scope, does not create `Write Authorization`, and does not close a `Task`.

## Required inputs

- `ToolEnvelope` with non-null `idempotency_key` and current `expected_state_version` for non-dry-run commits.
- `task_id`, `change_unit_id`, `judgment_kind`, `presentation`, `question`, `options`, `context`, `affected_refs`, `required_for`, and `expires_at`.
- A focused `question` with mutually understandable `options`.
- Enough `context` for the user to judge the exact issue without relying on hidden chat state.

Shared field shapes for `UserJudgment`, `UserJudgmentOption`, `UserJudgmentContext`, and refs are owned by [API Judgment Schemas](schema-judgment.md) and [API State Schemas](schema-state.md).

## Access requirements

The method requires `VerifiedSurfaceContext.access_class=core_mutation` and `verified=true`.

The request must target a compatible same-project `Task` and optional Change Unit. Local access failures, unreadable project or Task identity, and insufficient local capability reject before commit.

## State-version behavior

Committed `dry_run=false` result:

- increments `project_state.state_version` exactly once
- creates one pending `UserJudgment`
- may update affected blocker state only as allowed by the storage-effect owner

Non-claims:

- A `UserJudgmentCandidate` returned by another method is not durable until `harness.request_user_judgment` commits.
- Dry run and rejection create no pending judgment, blocker update, event, replay row, or state-version increment.

## Success result

Returns `RequestUserJudgmentResult` with:

- `base.response_kind=result`
- `base.effect_kind=core_committed`
- `user_judgment_ref`
- pending `user_judgment`
- affected `blocker_refs`
- current `state`

## Blocked result

There is no separate committed blocked response branch for this method.

When a pending judgment cannot be created, the method rejects before commit. Common causes include invalid request shape, an unsupported or incompatible `judgment_kind`, missing or incompatible Task identity, unresolved prerequisite judgment, local access failure, insufficient capability, stale `expected_state_version`, and validator failure.

## Rejected result

Returns `ToolRejectedResponse` for pre-commit failures. Public error code meaning is owned by [API error codes](error-codes.md). Public error precedence is owned by [API error precedence](error-precedence.md).

Rejected attempts do not create a pending judgment and do not persist request-like blocker data as a side effect.

## Dry-run behavior

For `dry_run=true`, a valid preview returns `ToolDryRunResponse`. Branch shape is owned by [API Schema Core](schema-core.md); no-effect persistence semantics are owned by [Storage Effects](../storage-effects.md).

The preview must not return a durable `user_judgment_ref` or create a pending `UserJudgment`.

## Storage effect

On commit, the method may persist a pending `user_judgments` row and related blocker state. Exact storage effects are owned by [Storage Effects](../storage-effects.md#harnessrequest_user_judgment).

## Example

Example preconditions:

- `proj_123`, `task_456`, and `cu_001` already exist in the same project.
- The current project `state_version` is `21`.
- The account data export confirmation copy is ready for a user-owned product decision.

### Minimal valid request

```yaml
method: harness.request_user_judgment
params:
  envelope:
    project_id: proj_123
    task_id: task_456
    actor_kind: agent
    surface_id: surface_local
    request_id: req_judgment_001
    idempotency_key: idem_judgment_001
    expected_state_version: 21
    dry_run: false
    locale: en-US
  task_id: task_456
  change_unit_id: cu_001
  judgment_kind: product_decision
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
  expires_at: null
```

### Representative response

Result branch (`RequestUserJudgmentResult`, committed):

```yaml
base:
  response_kind: result
  effect_kind: core_committed
  dry_run: false
  state_version: 22
  events:
    - event_id: evt_1005
      event_kind: user_judgment_requested
user_judgment_ref:
  record_kind: user_judgment
  record_id: uj_001
  project_id: proj_123
  task_id: task_456
  state_version: 22
user_judgment:
  judgment_id: uj_001
  project_id: proj_123
  task_id: task_456
  change_unit_id: cu_001
  judgment_kind: product_decision
  status: pending
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
  resolution: null
  expires_at: null
  created_at: "<example-created-at>"
  resolved_at: null
blocker_refs: []
state:
  project_id: proj_123
  state_version: 22
```

## Owner links

- Request envelope, response branches, and dry-run summaries: [API Schema Core](schema-core.md).
- `UserJudgment`, options, context, and answer payloads: [API Judgment Schemas](schema-judgment.md).
- State refs and summaries: [API State Schemas](schema-state.md).
- Judgment kinds and supported values: [API Value Sets](schema-value-sets.md).
- User-owned judgment and non-substitution rules: [Core Model](../core-model.md).
- Exact storage effects: [Storage Effects](../storage-effects.md#harnessrequest_user_judgment).
- Public errors: [API error codes](error-codes.md) and [API error precedence](error-precedence.md).
- Recording the user's answer to a pending judgment: [`harness.record_user_judgment`](method-record-user-judgment.md).
