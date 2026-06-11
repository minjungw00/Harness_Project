# API errors

This document describes future Harness Server behavior for planning and review. It does not mean this documentation repository implements an MCP server or any runtime behavior.

## Owns / Does not own

This document owns:

| This document owns | Boundary |
|---|---|
| Public `ErrorCode` identifiers | The public code set, public meanings, and which public path may carry each code. |
| Error precedence | How to choose `errors[0]` when a response branch carries more than one public error. |
| Error vs blocker routing | Whether a condition belongs in `ToolRejectedResponse.errors[]`, a method-specific blocked result, or dry-run preview data. |
| `STATE_VERSION_CONFLICT` | Public stale-state and idempotency-conflict behavior. It is a public error code, not a blocker code. |
| User-facing labels | Display guidance for public errors. Labels do not replace public identifiers. |

This document does not own:

| Not owned here | Owner |
|---|---|
| Method payload schemas, response field shapes, and common envelopes | [API Schema Core](schema-core.md), method owner documents routed from [MVP API](mvp-api.md), and the API schema owners. |
| Core gates, user judgments, and close-readiness order | [Core Model](../core-model.md), [User-judgment methods](method-user-judgment.md), and [Close-task method](method-close-task.md). |
| `CloseReadinessBlocker`, `WriteDecisionReason`, `PlannedBlocker`, and value-set field definitions | [API State Schemas](schema-state.md), [API Schema Core](schema-core.md), and [API Value Sets](schema-value-sets.md). |
| Storage rows, replay rows, DDL, locks, migrations, and storage effects | [Storage Records](../storage-records.md), [Storage Effects](../storage-effects.md), and [Storage Versioning](../storage-versioning.md). |
| Security guarantee wording and access-boundary claims | [Security](../security.md). |

## Error vs blocker

| Concept | Public shape | Details |
|---|---|---|
| Rejected response | `ToolRejectedResponse.errors[]` | [Rejected response](#error-vs-blocker-rejected-response) |
| Blocked result | method-specific result fields | [Blocked result](#error-vs-blocker-blocked-result) |
| Dry-run preview | `ToolDryRunResponse` | [Dry-run preview](#error-vs-blocker-dry-run-preview) |

<a id="error-vs-blocker-rejected-response"></a>
Rejected response:
- Public shape: `ToolRejectedResponse.errors[]` with `ToolError.code: ErrorCode`.
- Meaning: The method did not proceed to the committed operation.
- Condition: The failure is public transport, request, freshness, local-access, capability, or precondition rejection.
- State effect: No committed operation and no state change.

<a id="error-vs-blocker-blocked-result"></a>
Blocked result:
- Public shape: Method-specific result fields such as `write_decision_reasons` or `blockers`.
- Meaning: The method may have returned an operation-specific blocked outcome.
- Non-claim: This is not a public transport or schema error.
- State effect: Only the method owner may allow a committed blocked result or read-only blocker data.

<a id="error-vs-blocker-dry-run-preview"></a>
Dry-run preview:
- Public shape: `ToolDryRunResponse` with `DryRunSummary.would_errors[]` or `DryRunSummary.would_blockers[]`.
- Meaning: Previewable diagnostics for a valid dry-run request.
- State effect: Not a committed write and not stored blocker state.

`ErrorCode` values are public API identifiers. Blocker codes are operation-specific result values. A public `ErrorCode` must not be reused as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="error-taxonomy"></a>

## Public `ErrorCode` summary

| ErrorCode | Details |
|---|---|
| `VALIDATION_FAILED` | See [`VALIDATION_FAILED`](#errorcode-validation-failed) |
| `STATE_VERSION_CONFLICT` | See [`STATE_VERSION_CONFLICT`](#errorcode-state-version-conflict) |
| `MCP_UNAVAILABLE` | See [`MCP_UNAVAILABLE`](#errorcode-mcp-unavailable) |
| `LOCAL_ACCESS_MISMATCH` | See [`LOCAL_ACCESS_MISMATCH`](#errorcode-local-access-mismatch) |
| `NO_ACTIVE_TASK` | See [`NO_ACTIVE_TASK`](#errorcode-no-active-task) |
| `NO_ACTIVE_CHANGE_UNIT` | See [`NO_ACTIVE_CHANGE_UNIT`](#errorcode-no-active-change-unit) |
| `BASELINE_STALE` | See [`BASELINE_STALE`](#errorcode-baseline-stale) |
| `SCOPE_REQUIRED` | See [`SCOPE_REQUIRED`](#errorcode-scope-required) |
| `SCOPE_VIOLATION` | See [`SCOPE_VIOLATION`](#errorcode-scope-violation) |
| `WRITE_AUTHORIZATION_REQUIRED` | See [`WRITE_AUTHORIZATION_REQUIRED`](#errorcode-write-authorization-required) |
| `WRITE_AUTHORIZATION_INVALID` | See [`WRITE_AUTHORIZATION_INVALID`](#errorcode-write-authorization-invalid) |
| `APPROVAL_DENIED` | See [`APPROVAL_DENIED`](#errorcode-approval-denied) |
| `APPROVAL_EXPIRED` | See [`APPROVAL_EXPIRED`](#errorcode-approval-expired) |
| `APPROVAL_REQUIRED` | See [`APPROVAL_REQUIRED`](#errorcode-approval-required) |
| `DECISION_UNRESOLVED` | See [`DECISION_UNRESOLVED`](#errorcode-decision-unresolved) |
| `AUTONOMY_BOUNDARY_EXCEEDED` | See [`AUTONOMY_BOUNDARY_EXCEEDED`](#errorcode-autonomy-boundary-exceeded) |
| `DECISION_REQUIRED` | See [`DECISION_REQUIRED`](#errorcode-decision-required) |
| `CAPABILITY_INSUFFICIENT` | See [`CAPABILITY_INSUFFICIENT`](#errorcode-capability-insufficient) |
| `EVIDENCE_INSUFFICIENT` | See [`EVIDENCE_INSUFFICIENT`](#errorcode-evidence-insufficient) |
| `RESIDUAL_RISK_NOT_VISIBLE` | See [`RESIDUAL_RISK_NOT_VISIBLE`](#errorcode-residual-risk-not-visible) |
| `ACCEPTANCE_REQUIRED` | See [`ACCEPTANCE_REQUIRED`](#errorcode-acceptance-required) |
| `PROJECTION_STALE` | See [`PROJECTION_STALE`](#errorcode-projection-stale) |
| `ARTIFACT_MISSING` | See [`ARTIFACT_MISSING`](#errorcode-artifact-missing) |
| `VALIDATOR_FAILED` | See [`VALIDATOR_FAILED`](#errorcode-validator-failed) |

<a id="errorcode-validation-failed"></a>
### `VALIDATION_FAILED`

Used in:
- `ToolRejectedResponse.errors[]`

Condition:
- Invalid payload shape, enum value, activation rule, profile validation, or artifact input shape.

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Not allowed:
- Do not use this as a blocker code for request rejection.

<a id="errorcode-state-version-conflict"></a>
### `STATE_VERSION_CONFLICT`

Used in:
- `ToolRejectedResponse.errors[]`

Condition:
- `expected_state_version` is stale.
- `WriteAuthorization.basis_state_version` is stale.
- The idempotency request hash conflicts.

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Not allowed:
- Do not use this as a close-readiness blocker code.
- Do not use this as `WriteDecisionReason.code`, `CloseReadinessBlocker.code`, `PlannedBlocker.code`, `MethodResult.decision`, or a committed blocked-result primary code.

<a id="errorcode-mcp-unavailable"></a>
### `MCP_UNAVAILABLE`

Used in:
- `ToolRejectedResponse.errors[]`

Condition:
- Required Core, MCP, or surface reachability is unavailable.

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Not allowed:
- Do not use this as a blocker code for request rejection.

<a id="errorcode-local-access-mismatch"></a>
### `LOCAL_ACCESS_MISMATCH`

Used in:
- `ToolRejectedResponse.errors[]`

Condition:
- Reachable local access does not match the registered transport, session, binding, project, or surface instance, or access was revoked.

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Not allowed:
- Do not use this as a blocker code for request rejection.

<a id="errorcode-no-active-task"></a>
### `NO_ACTIVE_TASK`

Used in:
- `ToolRejectedResponse.errors[]`

Condition:
- A Task is required but none is active or addressed.

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-no-active-change-unit"></a>
### `NO_ACTIVE_CHANGE_UNIT`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- A write-capable or close-relevant operation lacks an active scoped Change Unit.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-baseline-stale"></a>
### `BASELINE_STALE`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- The baseline no longer matches the repository state required by the operation.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-scope-required"></a>
### `SCOPE_REQUIRED`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- Scope confirmation is required before the requested write or action can proceed.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-scope-violation"></a>
### `SCOPE_VIOLATION`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- Intended or observed paths or sensitive categories exceed active scope or stored authorized scope.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-write-authorization-required"></a>
### `WRITE_AUTHORIZATION_REQUIRED`

Used in:
- `ToolRejectedResponse.errors[]`

Condition:
- A write-capable Run lacks a required Write Authorization.

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-write-authorization-invalid"></a>
### `WRITE_AUTHORIZATION_INVALID`

Used in:
- `ToolRejectedResponse.errors[]`

Condition:
- Supplied Write Authorization is expired, revoked, consumed, or incompatible for a non-version reason.

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-approval-denied"></a>
### `APPROVAL_DENIED`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- Required sensitive-action approval was denied.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-approval-expired"></a>
### `APPROVAL_EXPIRED`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- Required sensitive-action approval expired or drifted from scope or baseline.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-approval-required"></a>
### `APPROVAL_REQUIRED`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- Sensitive-action approval is required before proceeding.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-decision-unresolved"></a>
### `DECISION_UNRESOLVED`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- A relevant user judgment is pending, deferred without coverage, rejected, blocked, stale, superseded, or incompatible.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-autonomy-boundary-exceeded"></a>
### `AUTONOMY_BOUNDARY_EXCEEDED`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- The intended operation exceeds the active Change Unit Autonomy Boundary.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-decision-required"></a>
### `DECISION_REQUIRED`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- A blocking user-owned judgment must be requested before proceeding.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-capability-insufficient"></a>
### `CAPABILITY_INSUFFICIENT`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- The surface is recognized but lacks a required access class, observation, capture, guarantee support, or active behavior.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the canonical method or schema owner explicitly allows that use.

<a id="errorcode-evidence-insufficient"></a>
### `EVIDENCE_INSUFFICIENT`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- Required evidence coverage is absent, partial, stale, or blocked.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the close-readiness owner explicitly allows that mapping.

<a id="errorcode-residual-risk-not-visible"></a>
### `RESIDUAL_RISK_NOT_VISIBLE`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- Known close-relevant residual risk has not been made visible before final acceptance or close.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the close-readiness owner explicitly allows that mapping.

<a id="errorcode-acceptance-required"></a>
### `ACCEPTANCE_REQUIRED`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- Required final acceptance is pending, rejected, or incompatible with the visible result basis.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the close-readiness owner explicitly allows that mapping.

<a id="errorcode-projection-stale"></a>
### `PROJECTION_STALE`

Used in:
- `ToolRejectedResponse.errors[]`

Condition:
- A requested readable status or view is stale or failed.

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Not allowed:
- Do not use this by itself as a close-readiness blocker code.

<a id="errorcode-artifact-missing"></a>
### `ARTIFACT_MISSING`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- A referenced persistent artifact is missing, unavailable, unusable for the close basis, or failed integrity/metadata checks.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this as a blocker code unless the close-readiness owner explicitly allows that mapping.

<a id="errorcode-validator-failed"></a>
### `VALIDATOR_FAILED`

Used in:
- `ToolRejectedResponse.errors[]`
- Owner-defined result paths

Condition:
- Fallback when a required active validator or blocker check failed and no more specific typed code applies.

State effect:
- Rejection path: no committed operation proceeds and no owner state mutation occurs.
- Owner-defined result paths: only the owning method or schema may define committed result effects.

Not allowed:
- Do not use this fallback when a more specific active code applies.
- Do not use this as a blocker code outside the owning method or schema fallback.

`ToolError.details.authorization_reason` uses `missing`, `expired`, `stale`, `revoked`, `consumed`, or `incompatible`. A stale `WriteAuthorization.basis_state_version` uses `STATE_VERSION_CONFLICT`, not `WRITE_AUTHORIZATION_INVALID`.

`ToolError.details.artifact_input_error.reason` uses these detail helper values. They are not top-level public `ErrorCode` values; staged-handle validation failures keep the public code `VALIDATION_FAILED` unless the actual failure is request-level local access or capability verification.

| `artifact_input_error.reason` | Meaning |
|---|---|
| `staged_handle_expired` | The staged handle is past its usable lifetime. |
| `staged_handle_consumed` | The staged handle was already consumed. |
| `staged_handle_project_mismatch` | The staged handle belongs to a different project. |
| `staged_handle_task_mismatch` | The staged handle belongs to a different Task. |
| `staged_handle_surface_mismatch` | The staged handle provenance does not match the verified surface. |
| `staged_handle_checksum_mismatch` | The staged bytes do not match the expected checksum. |
| `staged_handle_size_mismatch` | The staged bytes do not match the expected size. |
| `staged_handle_not_found` | The staged handle cannot be found. |

<a id="primary-error-code-precedence"></a>

## Error precedence

When an error-bearing branch has non-empty `errors`, `errors[0]` is the primary public code selected by this order unless a method owner defines a stricter method-specific order.

| Precedence | Primary `ErrorCode` | Applies to |
|---:|---|---|
| 1 | `VALIDATION_FAILED` | Rejected request shape or validation failure. |
| 2 | `STATE_VERSION_CONFLICT` | Rejected response only; see [precedence exclusion](#state-version-conflict-precedence-exclusion). |
| 3 | `MCP_UNAVAILABLE` | Rejected Core, MCP, or surface reachability failure. |
| 4 | `LOCAL_ACCESS_MISMATCH` | Rejected local-access binding mismatch or revocation. |
| 5 | `NO_ACTIVE_TASK` | Rejected missing Task identity. |
| 6 | `NO_ACTIVE_CHANGE_UNIT` | Missing active Change Unit. |
| 7 | `BASELINE_STALE` | Stale baseline. |
| 8 | `SCOPE_REQUIRED` | Missing required scope confirmation. |
| 9 | `SCOPE_VIOLATION` | Scope or authorized-attempt violation. |
| 10 | `WRITE_AUTHORIZATION_REQUIRED` | Missing required Write Authorization. |
| 11 | `WRITE_AUTHORIZATION_INVALID` | Non-version invalid Write Authorization. |
| 12 | `APPROVAL_DENIED` | Denied sensitive-action approval. |
| 13 | `APPROVAL_EXPIRED` | Expired or drifted sensitive-action approval. |
| 14 | `APPROVAL_REQUIRED` | Missing sensitive-action approval. |
| 15 | `DECISION_UNRESOLVED` | Existing user judgment is not usable. |
| 16 | `AUTONOMY_BOUNDARY_EXCEEDED` | Autonomy boundary exceeded. |
| 17 | `DECISION_REQUIRED` | New user-owned judgment required. |
| 18 | `CAPABILITY_INSUFFICIENT` | Missing surface capability. |
| 19 | `EVIDENCE_INSUFFICIENT` | Evidence coverage insufficient. |
| 20 | `RESIDUAL_RISK_NOT_VISIBLE` | Close-relevant risk not visible. |
| 21 | `ACCEPTANCE_REQUIRED` | Final acceptance required or incompatible. |
| 22 | `PROJECTION_STALE` | Readable view stale or failed. |
| 23 | `ARTIFACT_MISSING` | Persistent artifact missing, unavailable, unusable, or failed. |
| 24 | `VALIDATOR_FAILED` | Typed fallback when no more specific active code applies. |

<a id="state-version-conflict-precedence-exclusion"></a>
### `STATE_VERSION_CONFLICT` precedence exclusion

Used in:
- `ToolRejectedResponse.errors[]`

Condition:
- A rejected response is selected because stale state or idempotency conflict prevents the method from proceeding.

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Not allowed:
- Do not select `STATE_VERSION_CONFLICT` as `MethodResult.base.errors[0]`, `CloseTaskResult(close_state=blocked).errors[0]`, `WriteDecisionReason.code`, `CloseReadinessBlocker.code`, or `PlannedBlocker.code`.

<a id="blocked-and-dry-run-behavior"></a>

## Rejected response behavior

| Condition | Details |
|---|---|
| request validation fails before proceed | See [Request validation failure](#rejected-request-validation-failure) |
| precondition fails before commit | See [Precondition failure](#rejected-precondition-failure) |
| state or idempotency conflict | See [State or idempotency conflict](#rejected-state-or-idempotency-conflict) |
| `dry_run=true` pre-preview failure | See [`dry_run=true` pre-preview failure](#rejected-dry-run-pre-preview-failure) |

<a id="rejected-request-validation-failure"></a>
### Request validation failure

Condition:
- Request shape, schema, profile, or staged-handle validation fails before the method can proceed.

Route:
- `ToolRejectedResponse.errors[]`.

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Not allowed:
- Do not include method-specific result-only fields.

<a id="rejected-precondition-failure"></a>
### Precondition failure

Condition:
- Core, MCP, local access, surface capability, state lookup, Task identity, or a required precondition fails before commit.

Route:
- `ToolRejectedResponse.errors[]`.

State effect:
- No records, replay rows, artifacts, events, Write Authorization consumption, close-state mutation, or state-version increment.

<a id="rejected-state-or-idempotency-conflict"></a>
### State or idempotency conflict

Condition:
- `expected_state_version`, `WriteAuthorization.basis_state_version`, or idempotency request hash is stale or conflicting.

Route:
- `ToolRejectedResponse.errors[]` with `STATE_VERSION_CONFLICT`.

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Not allowed:
- The conflict is not a blocker.

<a id="rejected-dry-run-pre-preview-failure"></a>
### `dry_run=true` pre-preview failure

Condition:
- A `dry_run=true` request fails before a read result or dry-run preview can be produced.

Route:
- `ToolRejectedResponse` with `dry_run=true`.

State effect:
- No committed operation or dry-run preview is produced.

Not allowed:
- Do not represent the rejection as `DryRunSummary.would_errors[]` or `PlannedBlocker`.

Rejected response means the method did not proceed to the committed operation. It is not a blocked result and does not create the authority, evidence, acceptance, or close state that the request lacked.

## Blocked result behavior

| Blocked path | Details |
|---|---|
| `PrepareWriteResult` blocked decision | See [`PrepareWriteResult` blocked decision](#blocked-prepare-write-result) |
| `CloseTaskResult(close_state=blocked)` | See [`CloseTaskResult(close_state=blocked)`](#blocked-close-task-result) |
| read-only close-blocker observation | See [Read-only close-blocker observation](#blocked-read-only-observation) |

<a id="blocked-prepare-write-result"></a>
### `PrepareWriteResult` blocked decision

Condition:
- `PrepareWriteResult` has `decision=blocked`, `decision=approval_required`, or `decision=decision_required`.

Route:
- `write_decision_reasons: WriteDecisionReason[]`.

State effect:
- Only the method owner may define any committed blocked-result effect.

Result data:
- Uses method-owned decision reasons.

Not allowed:
- Does not return `CloseReadinessBlocker`.

<a id="blocked-close-task-result"></a>
### `CloseTaskResult(close_state=blocked)`

Condition:
- A valid close-readiness evaluation returns close blockers.

Route:
- `blockers: CloseReadinessBlocker[]`.

State effect:
- Only the close-task method owner may define any committed blocked-result effect.

Result data:
- Uses close-readiness blocker mapping.

Not allowed:
- Must not use `STATE_VERSION_CONFLICT`.

<a id="blocked-read-only-observation"></a>
### Read-only close-blocker observation

Condition:
- `StatusResult.close_blockers` or `harness.close_task intent=check` returns blocker observation data.

Route:
- Read-only `CloseReadinessBlocker` observation data.

Not allowed:
- No stored blocker and no state-version increment for the read.

Blocked result means the method may have returned an operation-specific blocked outcome. It is not a public transport/schema error. Any committed blocked result and any state effect must be allowed by the relevant method owner routed from [MVP API](mvp-api.md) and [Storage Effects](../storage-effects.md).

## Dry-run behavior

| Request | Response | Rule |
|---|---|---|
| Valid read-only call with `dry_run=true` | Method-specific result with `base.dry_run=true` and `base.effect_kind=read_only` | `dry_run=true` is not a synonym for `ToolDryRunResponse`. |
| Valid state-effecting or storage-owned staging operation with `dry_run=true` | `ToolDryRunResponse` with `DryRunSummary` | Dry-run preview is not a committed write. |
| Valid dry-run preview with expected blockers | `DryRunSummary.would_blockers: PlannedBlocker[]` | Preview blockers are not stored `CloseReadinessBlocker` objects. |
| Pre-commit failure with `dry_run=true` | `ToolRejectedResponse` | The failure is rejected, not previewed. |

`PlannedBlocker.code` must not be `STATE_VERSION_CONFLICT`. Stale state is rejected before preview.

<a id="idempotency"></a>
<a id="state-conflict-behavior"></a>

## State version conflict

| Conflict case | Details |
|---|---|
| stale `expected_state_version` | See [Stale `expected_state_version`](#state-conflict-expected-state-version) |
| stale `WriteAuthorization.basis_state_version` | See [Stale Write Authorization basis](#state-conflict-write-authorization-basis) |
| idempotency request-hash conflict | See [Idempotency request-hash conflict](#state-conflict-idempotency-hash) |

`STATE_VERSION_CONFLICT` has one active current MVP meaning: a project-wide pre-commit freshness or idempotency conflict.

<a id="state-conflict-expected-state-version"></a>
### Stale `expected_state_version`

Condition:
- `ToolEnvelope.expected_state_version` is older than `project_state.state_version`.

Public code:
- `STATE_VERSION_CONFLICT`

Response path:
- `ToolRejectedResponse.errors[]`

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Detail guidance:
- Include `state_clock: project_state.state_version`, `current_state_version`, `expected_state_version`, `project_id`, and `task_id` when available.

Not allowed:
- Do not use this as a blocker code.

<a id="state-conflict-write-authorization-basis"></a>
### Stale Write Authorization basis

Condition:
- `WriteAuthorization.basis_state_version` is stale before consumption.

Public code:
- `STATE_VERSION_CONFLICT`

Response path:
- `ToolRejectedResponse.errors[]`

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.
- The Write Authorization is not consumed.

Detail guidance:
- Identify the stale authorization basis and current `project_state.state_version`.

Not allowed:
- Do not use this as a blocker code.

<a id="state-conflict-idempotency-hash"></a>
### Idempotency request-hash conflict

Condition:
- The same `idempotency_key` is reused with a different request hash.

Public code:
- `STATE_VERSION_CONFLICT`

Response path:
- `ToolRejectedResponse.errors[]`

State effect:
- No committed operation proceeds.
- No owner state mutation occurs.

Detail guidance:
- Identify the `idempotency_key` and request-hash mismatch without exposing sensitive request bodies.

Not allowed:
- Do not use this as a blocker code.
- Do not represent this as dry-run preview data, `MethodResult.decision`, `WriteDecisionReason.code`, `CloseReadinessBlocker.code`, or `PlannedBlocker.code`.

## Forbidden blocker-code rules

| Forbidden use | Details |
|---|---|
| stale-state public error used as a blocker code | See [Stale-state blocker code](#forbidden-stale-state-blocker-code) |
| pre-commit public error copied into blocker arrays | See [Pre-commit public error copy](#forbidden-pre-commit-public-error-copy) |
| public `ErrorCode` reused without owner permission | See [Public code reuse](#forbidden-public-code-reuse) |
| user-facing label used as API identifier | See [User-facing label identifier](#forbidden-user-facing-label-identifier) |
| dry-run stale-state conflict previewed | See [Dry-run stale-state preview](#forbidden-dry-run-stale-state-preview) |

<a id="forbidden-stale-state-blocker-code"></a>
### Stale-state blocker code

Not allowed:
- Do not use `STATE_VERSION_CONFLICT` as `WriteDecisionReason.code`, `CloseReadinessBlocker.code`, `PlannedBlocker.code`, `MethodResult.decision`, or a committed blocked-result primary code.

Use instead:
- Return `ToolRejectedResponse.errors[]` with `effect_kind=no_effect`.

<a id="forbidden-pre-commit-public-error-copy"></a>
### Pre-commit public error copy

Not allowed:
- Do not copy pre-commit public errors into blocker arrays.

Use instead:
- Return `ToolRejectedResponse.errors[]`.

<a id="forbidden-public-code-reuse"></a>
### Public code reuse

Not allowed:
- Do not reuse a public `ErrorCode` as a blocker code without explicit canonical owner permission.

Use instead:
- Use the method/schema owner's blocker code or result reason.

<a id="forbidden-user-facing-label-identifier"></a>
### User-facing label identifier

Not allowed:
- Do not use a user-facing label as an API identifier.

Use instead:
- Keep the public `ErrorCode` unchanged and localize only display text.

<a id="forbidden-dry-run-stale-state-preview"></a>
### Dry-run stale-state preview

Not allowed:
- Do not represent a dry-run stale-state conflict in `DryRunSummary.would_errors[]` or `DryRunSummary.would_blockers[]`.

Use instead:
- Reject the request with `STATE_VERSION_CONFLICT`.

<a id="harnessclose_task-close-blockers"></a>

## `close_task` blocker mapping

| `close_task` situation | Details |
|---|---|
| Preflight failure before close-readiness evaluation | [Preflight failure](#close-task-preflight-failure) |
| `intent=check` with a valid read | [`intent=check`](#close-task-intent-check) |
| `intent=complete` with close-readiness blockers | [`intent=complete` blocked](#close-task-intent-complete-blocked) |
| `intent=complete` with no close blockers | [`intent=complete` closed](#close-task-intent-complete-closed) |
| Invalid `intent=cancel` or `intent=supersede` terminal transition | [Invalid terminal transition](#close-task-invalid-terminal-transition) |

<a id="close-task-preflight-failure"></a>
### Preflight failure

Condition:
- Stale state, stale Write Authorization basis, idempotency conflict, validation failure, local-access failure, capability failure, unreadable Core state, or unresolved project/Task identity occurs before close-readiness evaluation.

Response path:
- `ToolRejectedResponse.errors[]`

Public-code rule:
- `STATE_VERSION_CONFLICT` and other pre-commit errors stay in the rejected response.

Not allowed:
- Do not return `CloseReadinessBlocker` entries.

<a id="close-task-intent-check"></a>
### `intent=check`

Condition:
- The request is a valid read.

Response path:
- `CloseTaskResult` read-only result

Allowed:
- May return `CloseReadinessBlocker` observation data.

State effect:
- No stored blocker and no state-version increment.

<a id="close-task-intent-complete-blocked"></a>
### `intent=complete` blocked

Condition:
- A valid evaluation finds close-readiness blockers.

Response path:
- `CloseTaskResult(close_state=blocked)`

Allowed:
- May return `CloseReadinessBlocker[]`.

Not allowed:
- Do not use `STATE_VERSION_CONFLICT`.

<a id="close-task-intent-complete-closed"></a>
### `intent=complete` closed

Condition:
- No remaining owner-defined close blockers exist.

Response path:
- `CloseTaskResult(close_state=closed)`

Public-code rule:
- No close blockers.

<a id="close-task-invalid-terminal-transition"></a>
### Invalid terminal transition

Condition:
- `intent=cancel` or `intent=supersede` has an invalid terminal transition.

Response path:
- Method-owned result or rejection path

Public-code rule:
- Blockers are limited to transition validity.

Not allowed:
- Do not require evidence sufficiency, final acceptance, or residual-risk acceptance for cancellation or supersession.

### Close-readiness finding code summary

| Close-readiness finding | Details |
|---|---|
| Evidence gap | See [Evidence gap](#close-mapping-evidence-gap) |
| Persistent artifact issue | See [Persistent artifact issue](#close-mapping-artifact-issue) |
| Final acceptance issue | See [Final acceptance issue](#close-mapping-final-acceptance) |
| Residual risk not visible | See [Residual risk not visible](#close-mapping-residual-risk-not-visible) |
| Visible but unaccepted residual risk | See [Unaccepted residual risk](#close-mapping-unaccepted-residual-risk) |
| Unresolved user-owned judgment | See [Unresolved user-owned judgment](#close-mapping-unresolved-user-judgment) |
| Sensitive-action approval issue | See [Sensitive-action approval issue](#close-mapping-sensitive-approval) |
| Scope, boundary, or baseline blocker | See [Scope, boundary, or baseline blocker](#close-mapping-scope-boundary-baseline) |
| Readable view freshness issue | See [Readable view freshness issue](#close-mapping-readable-view-freshness) |
| Stale project-wide state or Write Authorization basis | See [Stale state is rejected](#close-mapping-stale-state-rejected) |

<a id="close-mapping-evidence-gap"></a>
### Evidence gap

Condition:
- Close-readiness evaluation finds an evidence gap.

Public code mapping:
- `EVIDENCE_INSUFFICIENT`

<a id="close-mapping-artifact-issue"></a>
### Persistent artifact issue

Condition:
- A close-relevant persistent artifact is missing, unavailable, unusable for the close basis, or failed.

Public code mapping:
- `ARTIFACT_MISSING`

<a id="close-mapping-final-acceptance"></a>
### Final acceptance issue

Condition:
- Required final acceptance is missing or incompatible.

Public code mapping:
- `ACCEPTANCE_REQUIRED`

<a id="close-mapping-residual-risk-not-visible"></a>
### Residual risk not visible

Condition:
- Known close-relevant residual risk is not visible.

Public code mapping:
- `RESIDUAL_RISK_NOT_VISIBLE`

<a id="close-mapping-unaccepted-residual-risk"></a>
### Unaccepted residual risk

Condition:
- Residual risk is visible but not accepted.

Public code mapping:
- `DECISION_REQUIRED` or `DECISION_UNRESOLVED` with `category=residual_risk_acceptance`

<a id="close-mapping-unresolved-user-judgment"></a>
### Unresolved user-owned judgment

Condition:
- A user-owned judgment is unresolved.

Public code mapping:
- `DECISION_REQUIRED` or `DECISION_UNRESOLVED`

<a id="close-mapping-sensitive-approval"></a>
### Sensitive-action approval issue

Condition:
- Sensitive-action approval is missing, denied, expired, or drifted.

Public code mapping:
- `APPROVAL_REQUIRED`, `APPROVAL_DENIED`, or `APPROVAL_EXPIRED`

<a id="close-mapping-scope-boundary-baseline"></a>
### Scope, boundary, or baseline blocker

Condition:
- A valid evaluation finds a scope, autonomy boundary, or baseline blocker.

Public code mapping:
- `SCOPE_REQUIRED`, `SCOPE_VIOLATION`, `AUTONOMY_BOUNDARY_EXCEEDED`, or `BASELINE_STALE`

Not allowed:
- Do not use this mapping unless the owner permits it.

<a id="close-mapping-readable-view-freshness"></a>
### Readable view freshness issue

Condition:
- A readable view freshness issue is present.

Public code mapping:
- `PROJECTION_STALE`

Not allowed:
- Do not use `PROJECTION_STALE` by itself as a close blocker.

<a id="close-mapping-stale-state-rejected"></a>
### Stale state is rejected

Condition:
- Project-wide state or `WriteAuthorization.basis_state_version` is stale.

Response path:
- `ToolRejectedResponse.errors[]` with `STATE_VERSION_CONFLICT`

Not allowed:
- Do not use this as a close blocker.

Full close-readiness evaluation order is owned by [Core Model close readiness](../core-model.md#close_task). Method behavior is owned by [`harness.close_task`](method-close-task.md). `CloseReadinessBlocker` shape and categories are owned by [API State Schemas](schema-state.md) and [API Value Sets](schema-value-sets.md).

## User-facing labels

User-facing labels may differ from public error identifiers. Labels are display text, not new public codes.

| Public condition | Suggested label | Smallest unblocker |
|---|---|---|
| `VALIDATION_FAILED` | invalid request | Fix the payload, enum value, activation rule, profile value, or field set before retrying. |
| `STATE_VERSION_CONFLICT` | state version conflict | Refresh current state and retry with the current `project_state.state_version`, or replay the original idempotent request. |
| `MCP_UNAVAILABLE` | Core or surface unavailable | Reconnect or diagnose Core, MCP, and surface reachability. |
| `LOCAL_ACCESS_MISMATCH` | local access mismatch | Use the registered local transport/session/binding or repair local access registration. |
| `CAPABILITY_INSUFFICIENT` | insufficient surface capability | Use a capable surface, reduce the operation, or avoid the missing capability. |
| `NO_ACTIVE_TASK` | no active Task | Select or create a Task before a Task-scoped action. |
| scope, boundary, or baseline codes | scope, boundary, or baseline issue | See [Scope, boundary, or baseline label](#label-scope-boundary-baseline) |
| `WRITE_AUTHORIZATION_REQUIRED`, `WRITE_AUTHORIZATION_INVALID` | missing or unusable pre-write check | Call or retry `harness.prepare_write` for the exact operation, current scope, and current state. |
| `DECISION_REQUIRED`, `DECISION_UNRESOLVED` | judgment needed | Request or resolve the focused `UserJudgment`. |
| `APPROVAL_REQUIRED`, `APPROVAL_DENIED`, `APPROVAL_EXPIRED` | sensitive-action approval needed or not usable | Request, resolve, or renew `judgment_kind=sensitive_approval`. |
| `EVIDENCE_INSUFFICIENT` | evidence needed | Record, rerun, or show the missing evidence and smallest unblocker. |
| `ACCEPTANCE_REQUIRED` | final acceptance needed | Request or resolve `judgment_kind=final_acceptance` for the visible result basis. |
| `RESIDUAL_RISK_NOT_VISIBLE` | residual risk not visible | Show the close-relevant residual risk before final acceptance or close. |
| `PROJECTION_STALE` | stale readable view | Refresh the view before relying on it. |
| `ARTIFACT_MISSING` | artifact issue | Restore, regenerate, replace, or reconnect the missing or unusable artifact. |
| `VALIDATOR_FAILED` | check failed | Show the specific validator or blocker when available; use this fallback only when no typed code applies. |

<a id="label-scope-boundary-baseline"></a>
### Scope, boundary, or baseline label

Public condition:
- `NO_ACTIVE_CHANGE_UNIT`, `SCOPE_REQUIRED`, `SCOPE_VIOLATION`, `AUTONOMY_BOUNDARY_EXCEEDED`, or `BASELINE_STALE`.

Suggested label:
- scope, boundary, or baseline issue.

Smallest unblocker:
- Confirm or narrow scope.
- Update valid scope or baseline through the owner path.
- Request the needed user judgment.

<a id="documentation-smoke-error-coverage"></a>

## Owner links

| Question | Owner |
|---|---|
| Public `ErrorCode` values, meanings, and precedence | This document. |
| `ToolRejectedResponse`, `ToolDryRunResponse`, `ToolError`, `ToolResultBase`, `DryRunSummary`, and response branch shape | [API Schema Core](schema-core.md). |
| Method behavior, branch selection, and method-specific payloads | Method owner documents routed from [MVP API](mvp-api.md). |
| `WriteDecisionReason`, `CloseReadinessBlocker`, state summaries, and close-readiness data shapes | [API State Schemas](schema-state.md). |
| `response_kind`, `effect_kind`, `PlannedBlocker.source_kind`, blocker categories, and enum-like API values | [API Value Sets](schema-value-sets.md). |
| `ArtifactInput`, `ArtifactRef`, `StagedArtifactHandle`, and artifact input shape | [API Artifact Schemas](schema-artifacts.md). |
| Staged-handle storage validation and artifact promotion lifecycle | [Artifact Storage](../storage-artifacts.md). |
| User judgments, sensitive-action approval, final acceptance, and residual-risk acceptance shapes | [API Judgment Schemas](schema-judgment.md) and [Core Model](../core-model.md). |
| Full close-readiness evaluation order and non-substitution rules | [Core Model close readiness](../core-model.md#close_task). |
| Storage effects, replay rows, state clocks, and DDL | [Storage Effects](../storage-effects.md), [Storage Versioning](../storage-versioning.md), and [Storage Records](../storage-records.md). |
| Security guarantee wording and access-boundary claims | [Security](../security.md). |
