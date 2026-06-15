# API judgment schemas

This document owns API schemas for user-owned judgment in the baseline scope. The schemas define judgment-shaped API data; they do not record user decisions by themselves.

## Owns / Does not own

This document owns:

- `UserJudgment`
- `UserJudgmentCandidate`
- `UserJudgmentOption`
- `UserJudgmentContext`
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
  required_for: string
  resolution: UserJudgmentResolution | null
  expires_at: string | null
  created_at: string
  resolved_at: string | null
```

`judgment_kind`, `status`, `presentation`, and `required_for` values are owned by [judgment values](schema-value-sets.md#judgment-values). Product meaning is owned by [Core Model user-owned judgment](../core-model.md#4-user-owned-judgment).

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

`option_id` is scoped to the judgment. Rendered labels are display text, not canonical schema values.

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

`selected_option_id` and `note` are request-level and resolution-level fields.

Shape rule:
- Exactly one decision-specific payload branch is populated for the selected `judgment_kind`.

Owner exception:
- A method owner may explicitly define a narrower payload structure.

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

## `AcceptedRiskInput`

`AcceptedRiskInput` is the shape for naming a visible residual risk inside a judgment payload.

```yaml
AcceptedRiskInput:
  risk_id: string | null
  summary: string
  consequence: string
  related_refs: StateRecordRef[]
  accepted_for_close: boolean
```

This shape is not verification, evidence sufficiency, QA, final acceptance, or proof that the result has no risk. Residual-risk meaning is owned by [Core Model](../core-model.md).

## Related owners

- [Core Model](../core-model.md) for user-owned judgment meaning and non-substitution rules.
- [Request-user-judgment method](method-request-user-judgment.md) for `harness.request_user_judgment`.
- [Record-user-judgment method](method-record-user-judgment.md) for `harness.record_user_judgment`.
- [API Value Sets](schema-value-sets.md) for `judgment_kind`, `presentation`, `required_for`, status, and option display boundaries.
- [API State Schemas](schema-state.md) for `StateRecordRef`.
- [API Artifact Schemas](schema-artifacts.md) for `ArtifactRef`.
- [Scope Reference](../scope.md) for reserved judgment routes and baseline-boundary checks.
