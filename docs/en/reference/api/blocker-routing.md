# API blocker routing

This document owns close-readiness blocker routing and the boundary between public API errors and close-readiness blockers.

It does not define Core close-readiness authority, `CloseReadinessBlocker` shape, `harness.close_task` method behavior, display wording, storage effects, or API response branch routing.

## Owner boundaries

This document owns:

- Close-readiness blocker routing categories and their relationship to public error-code families.
- The boundary between `ToolRejectedResponse.errors[]` public API errors and `CloseReadinessBlocker[]` close-readiness blocker data.
- Conditions where a public error-code family is represented as a close-readiness blocker without copying the public `ErrorCode` into `CloseReadinessBlocker.code`.
- `harness.close_task` blocker mapping for preflight rejection, read-only close checks, blocked close attempts, closed results, and invalid terminal transitions.
- How close-readiness blocker routing relates to rejected responses, blocked results, and `dry_run` previews.

This document does not own:

- Core authority, close-readiness meaning, final acceptance, residual-risk acceptance, or non-substitution rules; see [Core Model close readiness](../core-model.md#close_task).
- `CloseReadinessBlocker` shape, fields, and exact `CloseReadinessBlocker.category` values; see [API State Schemas](schema-state.md) and [API Value Sets](schema-value-sets.md#state-and-blocker-values).
- `harness.close_task` request behavior, close-readiness evaluation order, and committed blocked outcomes; see [`harness.close_task`](method-close-task.md).
- Rejected-response, blocked-result, and `dry_run` response branch routing; see [API error routing](error-routing.md).
- Public `ErrorCode` meanings and precedence; see [API error codes](error-codes.md) and [API error precedence](error-precedence.md).
- Display labels and rendered wording as display text only; see [Template Bodies](../template-bodies.md).

## Close-readiness blocker categories

Exact `CloseReadinessBlocker.category` value names belong to [API Value Sets](schema-value-sets.md#state-and-blocker-values). This page uses those values only to route close-readiness blocker findings to the applicable owner.

| Routing group | `CloseReadinessBlocker.category` values | Owner boundary |
|---|---|---|
| Core state and transition | `task`, `open_run`, `write_compatibility`, `baseline`, `recovery` | Routes blockers about Core state, open runs, write compatibility, baseline state, or recovery. Core meaning stays with [Core Model](../core-model.md). |
| Scope and authority boundary | `scope`, `user_judgment`, `sensitive_approval`, `surface_capability` | Routes blockers that require a scope path, user-owned judgment, sensitive-action approval, or surface capability resolution. |
| Evidence and artifact basis | `evidence`, `artifact_availability` | Routes blockers about evidence sufficiency or persistent artifact availability. Evidence and artifact semantics stay with their owners. |
| Acceptance and residual risk | `final_acceptance`, `residual_risk_visibility`, `residual_risk_acceptance` | Routes blockers about final acceptance, visible residual risk, or residual-risk acceptance without creating acceptance itself. |

## API error and blocker boundary

| Situation | Route | Boundary |
|---|---|---|
| Failure before a valid close-readiness evaluation | `ToolRejectedResponse.errors[]` with `ToolError.code: ErrorCode` | The request did not reach a valid close-readiness result. It does not return `CloseReadinessBlocker[]`. |
| Valid close-readiness evaluation finds a close blocker | `CloseReadinessBlocker[]` in the method result or read-only state result | The data explains why close is blocked. It is not a public transport or schema rejection. |
| Valid dry-run preview predicts blocker-like outcomes | `DryRunSummary.would_blockers: PlannedBlocker[]` | Preview blockers are not stored `CloseReadinessBlocker` objects and do not create close-readiness state. |
| Response branch selection is the question | [API error routing](error-routing.md) | This page routes blocker meaning after the response branch is known. |

## Forbidden public error representation

Public `ErrorCode` values are public API identifiers, not blocker codes. A close-readiness blocker may correspond to a public error-code family only when the condition is found during a valid close-readiness evaluation and the applicable owner defines a supported blocker category or blocker code for that condition.

The public `ErrorCode` family may be cited as a mapping, but the value is not copied into `CloseReadinessBlocker.code` unless the schema or method owner explicitly allows that exact use.

| Public error family | Close-readiness blocker representation | Boundary |
|---|---|---|
| `EVIDENCE_INSUFFICIENT` | A valid evaluation finds an evidence gap and routes it through `category=evidence`. | A preflight failure still uses `ToolRejectedResponse.errors[]`. |
| `ARTIFACT_MISSING` | A persistent close-relevant artifact issue routes through `category=artifact_availability`. | Artifact shape and storage meaning stay with artifact owners. |
| `ACCEPTANCE_REQUIRED` | Missing or incompatible final acceptance routes through `category=final_acceptance`. | The blocker does not create final acceptance. |
| `RESIDUAL_RISK_NOT_VISIBLE` | Known close-relevant residual risk that is not visible routes through `category=residual_risk_visibility`. | Visibility is distinct from residual-risk acceptance. |
| `DECISION_REQUIRED`, `DECISION_UNRESOLVED` | Unresolved user-owned judgment or residual-risk acceptance routes through `category=user_judgment` or `category=residual_risk_acceptance`. | The blocker does not record the user's decision. |
| `APPROVAL_REQUIRED`, `APPROVAL_DENIED`, `APPROVAL_EXPIRED` | Sensitive-action approval issues route through `category=sensitive_approval`. | The blocker does not create approval or `Write Authorization`. |
| `SCOPE_REQUIRED`, `SCOPE_VIOLATION`, `AUTONOMY_BOUNDARY_EXCEEDED`, `BASELINE_STALE`, `CAPABILITY_INSUFFICIENT` | Scope, autonomy-boundary, baseline, or surface-capability findings route through `category=scope`, `category=baseline`, or `category=surface_capability` when the owner permits the mapping. | Do not infer support from the public code name alone. |
| `PROJECTION_STALE` | A readable-view freshness issue may be named as a related public-code family. | `PROJECTION_STALE` by itself is not a close blocker. |
| `STATE_VERSION_CONFLICT` | No close-readiness blocker representation. | Stale state is rejected before close-readiness evaluation. |

## Relationship to API responses

| API response path | How blocker routing applies | Owner boundary |
|---|---|---|
| Rejected response | Preflight, validation, freshness, local-access, capability, and request failures stay in `ToolRejectedResponse.errors[]`. | Rejected-response branch routing belongs to [API error routing](error-routing.md). |
| Blocked result | A valid `CloseTaskResult(close_state=blocked)` may include `blockers: CloseReadinessBlocker[]`. | Method behavior and committed blocked effects belong to [`harness.close_task`](method-close-task.md) and [Storage Effects](../storage-effects.md). |
| Read-only observation | `StatusResult.close_blockers` or `harness.close_task intent=check` may return read-only blocker observation data. | Read-only observation does not store blocker state or increment `state_version`. |
| `dry_run` preview | `DryRunSummary.would_blockers: PlannedBlocker[]` may preview blocker-like outcomes. | `PlannedBlocker` is not `CloseReadinessBlocker`. |

<a id="harnessclose_task-close-blockers"></a>

## `harness.close_task` blocker mapping

- Preflight failure before close-readiness evaluation:
  - [Preflight failure](#close-task-preflight-failure)
- `intent=check` with a valid read:
  - [`intent=check`](#close-task-intent-check)
- `intent=complete` with close-readiness blockers:
  - [`intent=complete` blocked](#close-task-intent-complete-blocked)
- `intent=complete` with no close blockers:
  - [`intent=complete` closed](#close-task-intent-complete-closed)
- Invalid `intent=cancel` or `intent=supersede` terminal transition:
  - [Invalid terminal transition](#close-task-invalid-terminal-transition)

<a id="close-task-preflight-failure"></a>
### Preflight failure

Condition:
- Stale state, stale `Write Authorization` basis, idempotency conflict, validation failure, local-access failure, capability failure, unreadable Core state, or unresolved project/`Task` identity occurs before close-readiness evaluation.

Response path:
- `ToolRejectedResponse.errors[]`

Public-code rule:
- `STATE_VERSION_CONFLICT` and other pre-commit errors stay in the rejected response.

Response boundary:
- Preflight failures do not return `CloseReadinessBlocker` entries.

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

Public-code boundary:
- `intent=complete` blocked does not use `STATE_VERSION_CONFLICT`.

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

Transition boundary:
- Cancellation and supersession do not require evidence sufficiency, final acceptance, or residual-risk acceptance.

## Close-readiness finding code summary

These rows summarize public error-code families for close-readiness findings. They do not turn public `ErrorCode` values into blocker codes.

| Close-readiness finding | Detail section |
|---|---|
| Evidence gap | [Evidence gap](#close-mapping-evidence-gap) |
| Persistent artifact issue | [Persistent artifact issue](#close-mapping-artifact-issue) |
| Final acceptance issue | [Final acceptance issue](#close-mapping-final-acceptance) |
| Residual risk not visible | [Residual risk not visible](#close-mapping-residual-risk-not-visible) |
| Residual risk missing acceptance | [Residual risk missing acceptance](#close-mapping-unaccepted-residual-risk) |
| Unresolved judgment | [Unresolved user-owned judgment](#close-mapping-unresolved-user-judgment) |
| Sensitive approval issue | [Sensitive-action approval issue](#close-mapping-sensitive-approval) |
| Scope, boundary, baseline, or capability | [Scope, boundary, baseline, or capability blocker](#close-mapping-scope-boundary-baseline) |
| Readable view freshness | [Readable view freshness issue](#close-mapping-readable-view-freshness) |
| Stale state rejection | [Stale state is rejected](#close-mapping-stale-state-rejected) |

<a id="close-mapping-evidence-gap"></a>
### Evidence gap

Condition:
- Close-readiness evaluation finds an evidence gap.

Blocker route:
- `category=evidence`

Public code mapping:
- `EVIDENCE_INSUFFICIENT`

<a id="close-mapping-artifact-issue"></a>
### Persistent artifact issue

Condition:
- A close-relevant persistent artifact is missing, unavailable, unusable for the close basis, or failed.

Blocker route:
- `category=artifact_availability`

Public code mapping:
- `ARTIFACT_MISSING`

<a id="close-mapping-final-acceptance"></a>
### Final acceptance issue

Condition:
- Required final acceptance is missing or incompatible.

Blocker route:
- `category=final_acceptance`

Public code mapping:
- `ACCEPTANCE_REQUIRED`

<a id="close-mapping-residual-risk-not-visible"></a>
### Residual risk not visible

Condition:
- Known close-relevant residual risk is not visible.

Blocker route:
- `category=residual_risk_visibility`

Public code mapping:
- `RESIDUAL_RISK_NOT_VISIBLE`

<a id="close-mapping-unaccepted-residual-risk"></a>
### Residual risk missing acceptance

Condition:
- Residual risk is visible and lacks a recorded acceptance.

Blocker route:
- `category=residual_risk_acceptance`

Public code mapping:
- `DECISION_REQUIRED` or `DECISION_UNRESOLVED` with `category=residual_risk_acceptance`

<a id="close-mapping-unresolved-user-judgment"></a>
### Unresolved user-owned judgment

Condition:
- A user-owned judgment is unresolved.

Blocker route:
- `category=user_judgment`

Public code mapping:
- `DECISION_REQUIRED` or `DECISION_UNRESOLVED`

<a id="close-mapping-sensitive-approval"></a>
### Sensitive-action approval issue

Condition:
- Sensitive-action approval is missing, denied, expired, or drifted.

Blocker route:
- `category=sensitive_approval`

Public code mapping:
- `APPROVAL_REQUIRED`, `APPROVAL_DENIED`, or `APPROVAL_EXPIRED`

<a id="close-mapping-scope-boundary-baseline"></a>
### Scope, boundary, baseline, or capability blocker

Condition:
- A valid evaluation finds a scope, autonomy-boundary, baseline, or surface-capability blocker.

Blocker route:
- `category=scope`, `category=baseline`, or `category=surface_capability`

Public code mapping:
- `SCOPE_REQUIRED`, `SCOPE_VIOLATION`, `AUTONOMY_BOUNDARY_EXCEEDED`, `BASELINE_STALE`, or `CAPABILITY_INSUFFICIENT`

Owner boundary:
- Use the public-code mapping only when the owner permits the mapping.

<a id="close-mapping-readable-view-freshness"></a>
### Readable view freshness issue

Condition:
- A readable view freshness issue is present.

Public code mapping:
- `PROJECTION_STALE`

Owner boundary:
- `PROJECTION_STALE` is not a close blocker by itself.

<a id="close-mapping-stale-state-rejected"></a>
### Stale state is rejected

Condition:
- Project-wide state or `WriteAuthorization.basis_state_version` is stale.

Response path:
- `ToolRejectedResponse.errors[]` with `STATE_VERSION_CONFLICT`

Response boundary:
- Stale state is not a close blocker.

## Non-claims

Blocker routing does not imply:

- final acceptance
- residual-risk acceptance
- user approval, sensitive-action approval, or `Write Authorization`
- evidence sufficiency or artifact availability
- close completion or terminal `Task` state
- blocker persistence or state-version increment
- rendered display wording
- permission to bypass the Core authority, method, schema, storage, or template owner
