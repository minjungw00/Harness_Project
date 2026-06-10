# API schema core

This document owns the common API envelope and response-branch schemas for the current MVP. It is documentation source material only and does not define method behavior, storage effects, state snapshots, artifact lifecycle, user-judgment meaning, public error semantics, or active value sets.

## Owns / Does not own

This document owns:

- schema notation conventions for API schema owner documents
- `ToolEnvelope`
- the common method result branch model
- `ToolResultBase`
- `ToolRejectedResponse`
- `ToolDryRunResponse`
- `ToolError`
- `EventRef`
- the common `response_kind` and `effect_kind` fields

This document does not own:

- method behavior; see [MVP API](mvp-api.md)
- state and current-position schemas; see [API State Schemas](schema-state.md)
- artifact schemas; see [API Artifact Schemas](schema-artifacts.md)
- user-owned judgment schemas; see [API Judgment Schemas](schema-judgment.md)
- active method names, `response_kind` values, `effect_kind` values, access classes, or other enum-like values; see [API Value Sets](schema-value-sets.md)
- public error codes, precedence, or error semantics; see [API Errors](errors.md)
- storage records or effects; see [Storage Records](../storage-records.md) and [Storage Effects](../storage-effects.md)

## Schema notation

Schema blocks in this page are planning notation, not generated code. They describe future API contract shape only.

`string | null` means the field is present and may be null. `Type[]` means an array of that type. Field value sets are listed in [API Value Sets](schema-value-sets.md) unless this page says the field is free-form text or an opaque identifier.

<a id="tool-envelope"></a>
## ToolEnvelope

`ToolEnvelope` is the common request envelope used by public methods unless [MVP API](mvp-api.md) gives a narrower method-specific request rule.

```yaml
ToolEnvelope:
  project_id: string
  task_id: string | null
  actor_kind: string
  surface_id: string
  request_id: string
  idempotency_key: string | null
  expected_state_version: integer | null
  dry_run: boolean
  locale: string | null
```

`task_id` is an optional request-level Task selector. Method-specific `task_id` fields, when present, take precedence as described by [MVP API](mvp-api.md#shared-request-rules). `expected_state_version` names the project-wide state clock used by state-changing methods; conflict behavior is owned by [API Errors](errors.md#state-conflict-behavior) and [Storage Versioning](../storage-versioning.md).

<a id="common-response"></a>
## Common response branches

Every public method response uses exactly one branch:

- a method-specific `MethodResult`
- `ToolRejectedResponse`
- `ToolDryRunResponse` when the selected state-effecting or storage-staging operation has a valid preview branch

`MethodResult` is not a single concrete schema. It is the method-specific successful or committed result branch defined by [MVP API](mvp-api.md). Every concrete method result carries `base: ToolResultBase` and then only that method's result fields.

```yaml
ToolResultBase:
  response_kind: string
  effect_kind: string
  dry_run: boolean
  state_version: integer | null
  events: EventRef[]

ToolRejectedResponse:
  base: ToolResultBase
  errors: ToolError[]

ToolDryRunResponse:
  base: ToolResultBase
  dry_run_summary: DryRunSummary
```

Method-specific result fields belong only to the method result branch. `ToolRejectedResponse` and `ToolDryRunResponse` do not carry result-only fields such as `task_ref`, `run_summary`, `staged_artifact_handle`, `write_authorization_ref`, `user_judgment_ref`, `decision`, or `close_state`.

The active `response_kind` and `effect_kind` values are owned by [API Value Sets](schema-value-sets.md#response-and-effect-values). Branch selection and state effects are owned by [MVP API](mvp-api.md#shared-request-rules). Public error precedence is owned by [API Errors](errors.md).

## Dry-run summary shapes

`DryRunSummary`, `PlannedEffect`, and `PlannedBlocker` are common dry-run branch support shapes. They are descriptive preview data only. They do not create records, reserve refs, consume handles, create replay rows, or increment `state_version`.

```yaml
DryRunSummary:
  planned_effects: PlannedEffect[]
  would_blockers: PlannedBlocker[]
  would_errors: ToolError[]
  next_actions: NextActionSummary[]
  diagnostics: string[]

PlannedEffect:
  target_kind: string
  action: string
  description: string

PlannedBlocker:
  source_kind: string
  category: string
  code: string
  message: string
  related_refs: StateRecordRef[]
```

`NextActionSummary` and `StateRecordRef` are owned by [API State Schemas](schema-state.md). `PlannedBlocker.source_kind` values are owned by [API Value Sets](schema-value-sets.md#state-and-blocker-values). Public `ErrorCode` values used in `ToolError.code` are owned by [API Errors](errors.md).

## Shared support shapes

```yaml
ToolError:
  code: string
  message: string
  retryable: boolean
  details: object | null

EventRef:
  event_id: string
  event_kind: string
```

`ToolError` is the shape used by `ToolRejectedResponse.errors` and previewable `DryRunSummary.would_errors`. The public error code set, error details semantics, and primary-error precedence stay in [API Errors](errors.md).
