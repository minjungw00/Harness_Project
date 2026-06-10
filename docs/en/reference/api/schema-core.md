# API Schema Core

This document owns the common API envelope and response-branch schemas for the current MVP. It is documentation source material only and does not define method behavior, storage effects, artifact lifecycle, user-judgment meaning, or active value sets.

## Owns / Does not own

This document owns:

- `ToolEnvelope`
- common response-branch structure
- `ToolResultBase`
- `ToolRejectedResponse`
- `ToolDryRunResponse`
- common response metadata fields and branch separation
- shared `ToolError` and `EventRef` shapes as response-branch support structures

This document does not own:

- method behavior; see [MVP API](mvp-api.md)
- API state schemas; see [API State Schemas](schema-state.md)
- API artifact schemas; see [API Artifact Schemas](schema-artifacts.md)
- API judgment schemas; see [API Judgment Schemas](schema-judgment.md)
- API value sets and enum-like values; see [API Value Sets](schema-value-sets.md)
- public error codes or precedence; see [API Errors](errors.md)
- storage records or effects; see [Storage Records](../storage-records.md) and [Storage Effects](../storage-effects.md)

## Schema Notation

Schema blocks in this page are planning notation, not generated code. They describe future API contract shape only.

<a id="tool-envelope"></a>
## Tool Envelope

`ToolEnvelope` is the common request envelope used by public methods unless a method owner explicitly says otherwise.

```yaml
ToolEnvelope:
  project_id: string
  actor_kind: string
  surface_id: string
  request_id: string
  idempotency_key: string | null
  expected_state_version: integer | null
  dry_run: boolean
  locale: string | null
```

The exact active values for envelope fields live in [API Value Sets](schema-value-sets.md). Method-specific request bodies and required/optional behavior live in [MVP API](mvp-api.md).

<a id="common-response"></a>
## Common Response Branches

Every public method response uses exactly one branch:

- a method-specific `MethodResult`
- `ToolRejectedResponse`
- `ToolDryRunResponse` when the selected state-effecting operation has a valid preview branch

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
  preview: object
```

Method-specific result fields belong only to the method result branch. Rejected responses and dry-run preview responses do not require result-only fields from successful methods.

## Shared Support Shapes

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

Public `ErrorCode` values and precedence live in [API Errors](errors.md). Active `response_kind`, `effect_kind`, and other enum-like values live in [API Value Sets](schema-value-sets.md).

<a id="local-surface-access-values"></a>
## Local Surface Context

Common local surface context fields are part of the request/response contract, but their active values and connector behavior are split:

- [API Value Sets](schema-value-sets.md) owns active `access_class` and related values.
- [MVP API](mvp-api.md) owns method request conditions.
- [Agent Integration](../agent-integration.md) owns connector behavior.
- [Security](../security.md) owns guarantee claims and non-claims.

<a id="state-summary"></a>
## State Schema Route

`StateSummary`, `StateRecordRef`, and `ShapingReadiness` are owned by [API State Schemas](schema-state.md).

<a id="artifactref"></a>
## Artifact Schema Route

`ArtifactRef`, `ArtifactInput`, and `StagedArtifactHandle` are owned by [API Artifact Schemas](schema-artifacts.md).

<a id="current-position-display-schemas"></a>
## Current-Position Display Schema Route

Current-position display schemas, including close-readiness and next-action data shapes, are owned by [API State Schemas](schema-state.md).

<a id="validatorresult"></a>
## ValidatorResult Route

`ValidatorResult` shape is owned by [API State Schemas](schema-state.md). Active validator IDs and severity-like values are owned by [API Value Sets](schema-value-sets.md).

<a id="current-mvp-value-sets"></a>
## Current MVP Value Sets Route

Active method names, API enum-like values, and profile-gated value boundaries are owned by [API Value Sets](schema-value-sets.md). This anchor remains only for older links.
