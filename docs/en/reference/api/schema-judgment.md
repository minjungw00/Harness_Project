# API judgment schemas

This document owns API schemas for user-owned judgment in the baseline scope. The schemas define judgment-shaped API data; they do not record user decisions by themselves.

## Owns / Does not own

This document owns:

- `UserJudgment`
- `UserJudgmentCandidate`
- `UserJudgmentOption`
- `UserJudgmentContext`
- `JudgmentBasis`
- `UserJudgmentResolution`
- `RecordUserJudgmentPayload`
- `SensitiveActionScope`
- `AcceptedRiskInput`
- user-owned judgment schema fields and nesting

This document does not own:

- the product meaning and non-substitution rules for user-owned judgment; see [Core Model](../core-model.md)
- method behavior for requesting judgment; see [Request-user-judgment method](method-request-user-judgment.md)
- method behavior for recording judgment; see [Record-user-judgment method](method-record-user-judgment.md)
- supported judgment-kind values, status values, presentation values, and required-for values; see [API Value Sets](schema-value-sets.md)
- final acceptance or residual-risk close effects; see [Core Model](../core-model.md) and [Close-task method](method-close-task.md)
- public error semantics for missing, unresolved, denied, or expired judgment; see [API error codes](error-codes.md)

## Boundary

Judgment schemas preserve the field structure of a user-owned choice. They are not behavior contracts for product decisions, technical decisions, scope decisions, sensitive-action approval, final acceptance, residual-risk acceptance, cancellation, or unsupported judgment categories; those meanings stay with the Core and method owners.

`UserJudgmentCandidate` is not a pending judgment.

`UserJudgment` and `UserJudgmentCandidate` are distinct shapes. Method owners define when each shape appears in a response.

A `RecordUserJudgmentPayload` is not the schema for current scope, evidence, `Write Authorization`, a close result, or a broad approval.

<a id="userjudgment"></a>
## `UserJudgment`

```yaml
UserJudgment:
  judgment_id: string
  project_id: string
  task_id: string
  change_unit_id: string | null
  judgment_kind: string
  status: string
  presentation: string
  question: string
  options: UserJudgmentOption[]
  context: UserJudgmentContext
  affected_refs: StateRecordRef[]
  basis: JudgmentBasis | null
  required_for: string
  resolution: UserJudgmentResolution | null
  expires_at: string | null
  created_at: string
  resolved_at: string | null
```

`judgment_kind`, `status`, `presentation`, and `required_for` values are owned by [judgment values](schema-value-sets.md#judgment-values). Product meaning is owned by [Core Model user-owned judgment](../core-model.md#4-user-owned-judgment).

`judgment_id`, `project_id`, `task_id`, and `change_unit_id` are opaque identifiers. `question` is a free-form display string.

`basis` is populated for newly created judgments. `basis=null` is only for preserved legacy or imported rows that lack a state basis; those rows are audit records and cannot satisfy current close, write, or sensitive-approval requirements.

## `JudgmentBasis`

`JudgmentBasis` is the Core-derived state snapshot used to decide whether a judgment can satisfy a current requirement.

```yaml
JudgmentBasis:
  task_id: string
  change_unit_id: string | null
  scope_revision: integer
  close_basis_revision: integer | null
  baseline_ref: string | null
  result_refs: StateRecordRef[]
  residual_risk_ids: string[]
  sensitive_action_scope: SensitiveActionScope | null
  created_at_state_version: integer
  compatibility_status: string
```

Core creates `JudgmentBasis` from current state when it creates the judgment. Callers do not submit `scope_revision`, `close_basis_revision`, current close-basis data, or session-binding data.

`compatibility_status` values are owned by [judgment values](schema-value-sets.md#judgment-values). `legacy_unbound` marks a preserved judgment without a basis. `stale` and `superseded` judgments remain stored when needed for audit but are not eligible to satisfy current close, write, or sensitive-approval requirements.

<a id="userjudgmentcandidate"></a>
## `UserJudgmentCandidate`

`UserJudgmentCandidate` is the candidate shape for a proposed focused question. It has no `judgment_id`, `status`, `resolution`, `created_at`, or `resolved_at` field.

```yaml
UserJudgmentCandidate:
  judgment_kind: string
  presentation: string
  question: string
  options: UserJudgmentOption[]
  context: UserJudgmentContext
  affected_refs: StateRecordRef[]
  required_for: string
  expires_at: string | null
```

## Option and context shapes

```yaml
UserJudgmentOption:
  option_id: string
  label: string
  description: string
  consequence: string
  is_default: boolean

UserJudgmentContext:
  summary: string
  related_refs: StateRecordRef[]
  artifact_refs: ArtifactRef[]
  visible_risks: AcceptedRiskInput[]
  constraints: string[]
```

`option_id` is scoped to the judgment. `label`, `description`, `consequence`, `summary`, and `constraints` entries are free-form display strings. Rendered labels are display text, not canonical schema values.

## Resolution and answer payload

```yaml
UserJudgmentResolution:
  selected_option_id: string
  answer: RecordUserJudgmentPayload
  note: string | null
  accepted_risks: AcceptedRiskInput[]
  resolved_by_actor_kind: string

RecordUserJudgmentPayload:
  product_decision: object | null
  technical_decision: object | null
  scope_decision: object | null
  sensitive_action_scope: SensitiveActionScope | null
  final_acceptance: object | null
  residual_risk_acceptance: object | null
  cancellation: object | null
```

`selected_option_id` and `note` are request-level and resolution-level fields. `selected_option_id` is scoped to the judgment option set. `note` is a free-form display string.

`resolved_by_actor_kind` uses the same controlled value set as `ToolEnvelope.actor_kind`; see [actor values](schema-value-sets.md#actor-values).

Shape rule:
- Exactly one decision-specific payload branch is populated for the selected `judgment_kind`.

Owner exception:
- A method owner may explicitly define a narrower payload structure.

String fields inside a decision-specific payload object are local to that payload structure unless the method owner explicitly defines a narrower local code list or value list. They are not global API value sets.

Not allowed:
- `RecordUserJudgmentPayload` does not contain `selected_option_id` or `note`.

## `SensitiveActionScope`

`SensitiveActionScope` is the schema shape for a named sensitive-action approval context. It is not `AuthorizedAttemptScope`, not `Write Authorization`, and not security authority; see [Security](../security.md).

```yaml
SensitiveActionScope:
  action_kind: string
  description: string
  intended_paths: string[]
  sensitive_categories: string[]
  command_or_tool_summary: string | null
  network_or_host_summary: string | null
  secret_or_credential_summary: string | null
  capability_claim: string
  expires_at: string | null
```

The presence of `SensitiveActionScope` does not define where sensitive-action approval is required. Method owners define where this shape appears, and it does not replace the `harness.prepare_write` path for product-file writes.

`SensitiveActionScope.action_kind` and `sensitive_categories[]` are opaque sensitive-action classification strings unless an affected method or profile owner publishes a narrower local list. `description`, `command_or_tool_summary`, `network_or_host_summary`, `secret_or_credential_summary`, and `capability_claim` are display or claim strings; they are not canonical value sets or security authority.

<a id="acceptedriskinput"></a>
## `AcceptedRiskInput`

`AcceptedRiskInput` is the shape for naming a visible residual risk inside a judgment payload.

```yaml
AcceptedRiskInput:
  risk_id: string
  summary: string
  consequence: string
  related_refs: StateRecordRef[]
  accepted_for_close: boolean
```

This shape is not verification, evidence sufficiency, QA, final acceptance, or proof that the result has no risk. Residual-risk meaning is owned by [Core Model](../core-model.md).

`risk_id` is the exact opaque risk identifier from the current close basis. It is required when accepting residual risk for close. `summary`, `consequence`, and `related_refs` are context for the user and audit trail; they do not authorize text matching.

## Related owners

- [Core Model](../core-model.md) for user-owned judgment meaning and non-substitution rules.
- [Request-user-judgment method](method-request-user-judgment.md) for `harness.request_user_judgment`.
- [Record-user-judgment method](method-record-user-judgment.md) for `harness.record_user_judgment`.
- [API Value Sets](schema-value-sets.md) for `judgment_kind`, `presentation`, `required_for`, status, actor values, and option display boundaries.
- [API State Schemas](schema-state.md) for `StateRecordRef`.
- [API Artifact Schemas](schema-artifacts.md) for `ArtifactRef`.
- [Scope Reference](../scope.md) for reserved judgment routes and baseline-boundary checks.
