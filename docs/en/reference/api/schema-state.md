# API state schemas

This document owns API state-shaped schemas for the current MVP. It is documentation source material only and does not create runtime state, generated projections, storage rows, or state effects.

## Owns / Does not own

This document owns:

- `StateSummary`
- `StateRecordRef`
- task lifecycle state as API data shape
- state-related snapshot and reference structures
- `ShapingReadiness`
- current-position display schemas such as `NextActionSummary`, `WriteAuthoritySummary`, `EvidenceSummary`, `CloseReadinessBlocker`, `ValidatorResult`, and `GuaranteeDisplay`
- the boundary between state-shaped data and response effects

This document does not own:

- common envelopes or response branches; see [API Schema Core](schema-core.md)
- active enum-like values; see [API Value Sets](schema-value-sets.md)
- method behavior; see [MVP API](mvp-api.md)
- public error semantics; see [API Errors](errors.md)
- Core lifecycle and close-readiness product meaning; see [Core Model](../core-model.md)
- storage records or persistence effects; see [Storage Records](../storage-records.md) and [Storage Effects](../storage-effects.md)

## Boundary

State schemas describe API data shapes. A state-shaped field does not by itself create persistence, a Core transition, replay rows, `task_events`, artifact effects, Write Authorization effects, or a `state_version` increment. The selected response branch and method behavior define those effects.

## State references

`StateRecordRef` is the common public reference shape for Core-owned records that appear in API responses. It is a pointer, not an embedded storage row.

```yaml
StateRecordRef:
  record_kind: string
  record_id: string
  project_id: string
  task_id: string | null
  state_version: integer | null
```

`record_kind` values are owned by [API Value Sets](schema-value-sets.md#record-and-reference-values). Storage table names and DDL stay in [Storage Records](../storage-records.md).

## StateSummary

`StateSummary` is the compact current-position state returned by active methods that need to show the current Task path.

```yaml
StateSummary:
  project_id: string
  state_version: integer
  task_ref: StateRecordRef | null
  mode: string | null
  lifecycle: TaskLifecycleState | null
  goal_summary: string | null
  scope_summary: string | null
  non_goals: string[]
  acceptance_criteria: string[]
  autonomy_boundary: string | null
  active_change_unit_ref: StateRecordRef | null
  baseline_ref: string | null
  shaping_readiness: ShapingReadiness | null
  pending_user_judgment_refs: StateRecordRef[]
  blocker_refs: StateRecordRef[]
  write_authority_summary: WriteAuthoritySummary | null
  evidence_summary: EvidenceSummary | null
  close_state: string | null
  close_blockers: CloseReadinessBlocker[]
  guarantee_display: GuaranteeDisplay | null
```

`StateSummary` may summarize stored Core state, computed read-only status, and close-readiness observations. It does not decide whether a method committed. That decision comes from the response branch and [MVP API](mvp-api.md).

## Task lifecycle state

`TaskLifecycleState` is the API shape for Task lifecycle fields that may appear inside `StateSummary` or close results.

```yaml
TaskLifecycleState:
  lifecycle_phase: string
  close_reason: string
  result: string
  closed_at: string | null
```

Active values for `lifecycle_phase`, `close_reason`, and `result` are owned by [API Value Sets](schema-value-sets.md#task-lifecycle-values). The product meaning of lifecycle areas is owned by [Core Model](../core-model.md#6-task-lifecycle).

## ShapingReadiness

`ShapingReadiness` is a derived API view over Task, Change Unit, pending judgments, evidence summary, blockers, and next-action state. It shows whether the current owner state is concrete enough for the next safe action.

```yaml
ShapingReadiness:
  goal_summary_known: boolean
  scope_boundary_known: boolean
  non_goals_known: boolean
  affected_area_or_paths_known: boolean
  acceptance_criteria_known: boolean
  autonomy_boundary_known: boolean
  first_change_unit_known: boolean
  user_owned_blocker_kind: string | null
  next_safe_action: NextActionSummary | null
  gaps: ShapingGap[]

ShapingGap:
  gap_kind: string
  message: string
  blocker_ref: StateRecordRef | null
  user_judgment_candidate_ref: StateRecordRef | null
```

Missing readiness can route to a blocker, a pending or candidate user judgment, or an update-scope next action. It does not create separate active Discovery Brief, Question Queue, Assumption Register, or generated planning artifact.

## Current-position display shapes

```yaml
NextActionSummary:
  action_kind: string
  owner_method: string | null
  label: string
  blocking_question: string | null
  required_refs: StateRecordRef[]

WriteAuthoritySummary:
  status: string
  write_authorization_ref: StateRecordRef | null
  basis_state_version: integer | null
  intended_paths: string[]
  guarantee_display: GuaranteeDisplay | null

WriteAuthorizationSummary:
  write_authorization_ref: StateRecordRef
  status: string
  authorized_attempt_scope: object
  basis_state_version: integer
  expires_at: string | null

WriteDecisionReason:
  category: string
  code: string
  message: string
  related_refs: StateRecordRef[]
```

`WriteDecisionReason` is used by `PrepareWriteResult.write_decision_reasons`; it is not a close-readiness blocker. The active categories and reason values are owned by [API Value Sets](schema-value-sets.md#state-and-blocker-values). Public error code meaning stays in [API Errors](errors.md).

## Evidence and run snapshot shapes

```yaml
EvidenceSummary:
  status: string
  completion_policy: CompletionPolicy
  coverage_items: EvidenceCoverageItem[]
  artifact_refs: ArtifactRef[]
  updated_by_run_ref: StateRecordRef | null

CompletionPolicy:
  evidence_required: boolean
  required_claims: string[]

EvidenceCoverageItem:
  claim: string
  required_for_close: boolean
  coverage_state: string
  supporting_refs: StateRecordRef[]
  supporting_artifact_refs: ArtifactRef[]
  gap_refs: StateRecordRef[]

RunSummary:
  run_ref: StateRecordRef
  kind: string
  summary: string
  observed_changes: ObservedChanges
  artifact_refs: ArtifactRef[]

ObservedChanges:
  changed_paths: string[]
  product_file_write_observed: boolean
  sensitive_categories: string[]
  baseline_ref: string | null
```

`ArtifactRef` is owned by [API Artifact Schemas](schema-artifacts.md). Evidence sufficiency meaning is owned by [Core Model](../core-model.md#9-evidence-and-run-authority) and method behavior is owned by [MVP API](mvp-api.md).

## Close readiness and validation shapes

```yaml
CloseReadinessBlocker:
  category: string
  code: string
  message: string
  related_refs: StateRecordRef[]
  next_actions: NextActionSummary[]

ValidatorResult:
  validator_id: string
  status: string
  severity: string | null
  message: string
  related_refs: StateRecordRef[]

GuaranteeDisplay:
  level: string
  basis: string
  capability_refs: StateRecordRef[]
```

`CloseReadinessBlocker` is a data shape for close-readiness findings. It is not the whole close-readiness concept and it does not itself imply persistence. Complete close-readiness order is owned by [Core Model](../core-model.md#close_task). Response branch behavior and committed blocked outcomes are owned by [MVP API](mvp-api.md#harnessclose_task). Public error routing is owned by [API Errors](errors.md#harnessclose_task-close-blockers).

Active `CloseReadinessBlocker.category`, `ValidatorResult.status`, `ValidatorResult.severity`, and `GuaranteeDisplay.level` values are owned by [API Value Sets](schema-value-sets.md). Security guarantee meaning is owned by [Security](../security.md).

## Related owners

- [API Schema Core](schema-core.md) for `ToolEnvelope`, `ToolResultBase`, `ToolRejectedResponse`, and `ToolDryRunResponse`.
- [API Value Sets](schema-value-sets.md) for exact values used by state fields.
- [MVP API](mvp-api.md) for the methods that return these schemas.
- [API Artifact Schemas](schema-artifacts.md) for `ArtifactRef`.
- [API Judgment Schemas](schema-judgment.md) for `UserJudgmentCandidate`.
- [Storage Effects](../storage-effects.md) for persistence and state-effect consequences.
