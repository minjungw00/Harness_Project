# Storage effects

This document owns method-to-storage effect semantics for the current MVP source design. It is documentation source material only and does not execute or simulate Harness runtime procedures.

## Owns / Does not own

This document owns:

- read-only, dry-run, rejected, staging-created, Core-committed, and committed-blocked storage-effect distinctions
- whether a method branch creates replay rows, `task_events`, record changes, state-version increments, staged-handle consumption, artifact promotion, or Write Authorization changes
- the persistence boundary for blocker-like response data
- no-effect guarantees for rejected branches and valid dry-run preview branches

This document does not own:

- record layout or DDL; see [Storage Records](storage-records.md)
- artifact lifecycle details; see [Artifact Storage](storage-artifacts.md)
- idempotency, locks, state-version clocks, event ordering, or migrations; see [Storage Versioning](storage-versioning.md)
- public response branches or schemas; see [API Schema Core](api/schema-core.md)
- API method behavior; see [MVP API](api/mvp-api.md)
- public error code precedence; see [API Errors](api/errors.md)

## Shape versus effect

Response data shape and storage effect are separate. `CloseReadinessBlocker`, `WriteDecisionReason`, `PlannedBlocker`, `ArtifactRef`, and `StagedArtifactHandle` are API data shapes. Their presence in a response does not by itself prove persistence, artifact promotion, staged-handle consumption, replay storage, close-state mutation, or `project_state.state_version` increment.

Effects come from the selected method behavior and response branch:

| Branch | Storage effect |
|---|---|
| Read-only `MethodResult` | Response only. No replay row, event, current-row mutation, artifact effect, Write Authorization effect, or state-version increment. |
| `ToolRejectedResponse` | No effect. No current row, no replay row, no event, no artifact effect, no Write Authorization creation/consumption, no state-version increment. |
| Valid `ToolDryRunResponse` | Preview only. No current row, no generated persistent ref, no replay row, no event, no staged handle, no artifact promotion/link, no state-version increment. |
| `StageArtifactResult` with `effect_kind=staging_created` | Temporary storage-owned staging only. No Core current row, replay row, event, persistent `ArtifactRef`, or state-version increment. |
| Core committed `MethodResult` | May mutate current rows, append `task_events`, create replay rows, and increment `project_state.state_version` exactly once as allowed by the method owner. |
| Committed blocked `MethodResult` | May persist only the blocker-state, event, replay-row, and state-version effects explicitly allowed by the method owner. It must not create the missing authority it reports. |

## No-effect branches

These failures return no-effect branches:

- malformed requests
- validation failures before commit
- local access failures before a protected operation can proceed
- capability failures
- stale `expected_state_version`
- stale `WriteAuthorization.basis_state_version`
- idempotency request-hash conflicts
- rejected artifact inputs

No-effect branches must not:

- create current rows
- append `task_events`
- write `tool_invocations.response_json`
- create replay rows
- update evidence summaries
- mutate close state
- create or consume Write Authorizations
- change `artifact_staging.status`
- set `consumed_by_run_id` or `promoted_artifact_id`
- promote or link artifacts
- increment `project_state.state_version`

Valid dry-run previews may include `DryRunSummary.would_blockers: PlannedBlocker[]` or planned effects. Those preview entries do not create:

- `task_event` or `task_events` append
- replay row or `tool_invocations.response_json`
- `close_state` mutation
- Write Authorization change
- staged-handle creation or consumption
- artifact effect
- evidence update
- `CloseReadinessBlocker` storage
- `project_state.state_version` increment

## Read-only effects

Read-only results are response-only and not replay rows. `harness.status` and `harness.close_task intent=check` may compute blockers, `CloseReadinessBlocker[]`, evidence summaries, artifact refs, diagnostics, and next actions for the response.

Storage must not persist those computed values merely because the read occurred.

`harness.status` with `close_blockers: CloseReadinessBlocker[]` is a read-only observation. It does not create:

- `task_event` or `task_events` append
- replay row or `tool_invocations.response_json`
- `close_state` mutation
- Write Authorization change
- staged-handle consumption
- artifact effect
- evidence update
- `project_state.state_version` increment

For `harness.close_task intent=check`, the response branch is owned by [`harness.close_task`](api/mvp-api.md#harnessclose_task). This storage page only asserts that the check remains read-only, including with `dry_run=true` and with `blockers: CloseReadinessBlocker[]`.

## Committed blocked effects

Committed blocked outcomes are distinct from rejected responses. A committed blocked `harness.prepare_write` or `harness.close_task` outcome is a `MethodResult` only when [MVP API](api/mvp-api.md) allows the blocked commit.

A committed non-dry-run `PrepareWriteResult` with `decision=blocked`, `decision=approval_required`, or `decision=decision_required` may include `write_decision_reasons: WriteDecisionReason[]` in the response and replay payload when the method state-effect contract permits committing that decision.

Those reasons are prepare-write decision reasons. They are not:

- close-readiness blockers
- `CloseReadinessBlocker[]`
- close-readiness blocker records

This branch must not:

- create a consumable Write Authorization
- mutate `close_state`
- run close-readiness evaluation
- create `CloseReadinessBlocker` storage
- update evidence
- touch artifacts
- consume staged handles
- perform `close_task` effects

`CloseTaskResult(close_state=blocked)` is storage-effective only when close readiness evaluation has run and the `harness.close_task` method contract permits committing the blocked result. It may include `blockers: CloseReadinessBlocker[]` and may create only the effects explicitly allowed by the API/storage contract:

- blocker state
- `task_events`
- replay row
- `project_state.state_version`

The Task remains open. This branch must not be used for `STATE_VERSION_CONFLICT`; that code belongs to the preflight `ToolRejectedResponse` branch and is not stored as replay.

## Method effects

This table summarizes persistence effects. Method behavior and response unions remain owned by [MVP API](api/mvp-api.md).

| Method or selected intent | Non-dry-run committed effect | Read-only, dry-run, and rejected boundaries |
|---|---|---|
| `harness.intake` | May create the Task, optional Change Unit, shaping records, events, replay row, and one `project_state.state_version` increment. | Valid `dry_run=true` returns `ToolDryRunResponse` and creates no Task, refs, event, replay row, or state-version increment. Rejections have no effect. |
| `harness.update_scope` | May update active Task scope fields, create/replace active `change_units`, update blockers or stale Write Authorization refs as the method owner allows, append events, create replay row, and increment state once. | Valid dry-run previews only describe scope, Change Unit, blocker, and stale authorization effects. Rejections have no effect. |
| `harness.status` | None; read-only response. | `dry_run=true` remains `StatusResult` with `effect_kind=read_only`, not `ToolDryRunResponse`; no replay row or mutation. |
| `harness.prepare_write` | `decision=allowed` may create or return a compatible active Write Authorization, append events, create replay row, and increment state once. Committed non-allowed decisions may persist only allowed decision-state/replay effects. | Rejected and valid dry-run branches create no replay row, no Write Authorization, no event, no close-state mutation, no artifact/evidence effect, and no state-version increment. |
| `harness.stage_artifact` | Successful staging creates only `artifact_staging` or an equivalent storage-owned staging manifest plus temporary safe bytes or notices under `artifacts/tmp/`. | Valid `dry_run=true` creates no bytes, no staging manifest, no `StagedArtifactHandle`, no replay row, and no state-version increment. Invalid staging requests have no Core/storage mutation beyond no-effect rejection. |
| `harness.record_run` | May create `runs`, consume compatible `write_authorizations`, consume eligible `artifact_staging`, promote/link `artifacts`, update `evidence_summaries` or allowed blockers, append events, create replay row, and increment state once. | Valid dry-run previews create no `run_summary`, persistent artifact, artifact link, evidence update, blocker update, event, replay row, staged-handle consumption, Write Authorization consumption, or state-version increment. Rejected attempts do not change staging rows or artifacts. |
| `harness.request_user_judgment` | May create a pending `user_judgments` row, update affected blockers, append events, create replay row, and increment state once. | Valid dry-run previews create no real `user_judgment_ref`, pending judgment, blocker update, event, replay row, or state-version increment. |
| `harness.record_user_judgment` | May resolve a `user_judgments` row, update dependent blockers or next actions, append events, create replay row, and increment state once. | Valid dry-run previews create no judgment resolution, blocker update, event, replay row, or state-version increment. |
| `harness.close_task intent=check` | None; read-only response with computed close readiness. | `dry_run=true` remains `CloseTaskResult` with `effect_kind=read_only`; no replay row, event, blocker row, close-state mutation, artifact/evidence effect, or state-version increment. |
| `harness.close_task intent=complete` | May close the Task when blockers allow it, or commit allowed blocked complete effects while the Task remains open; appends events, creates replay row, and increments state once on commit. | Valid `dry_run=true` returns `ToolDryRunResponse`; preflight failures are no-effect `ToolRejectedResponse`. |
| `harness.close_task intent=cancel` | May cancel the Task, or commit blockers that invalidate cancellation itself while the Task remains open; appends events, creates replay row, and increments state once on commit. | Valid `dry_run=true` returns `ToolDryRunResponse`; preflight failures have no effect. Cancellation is not evidence sufficiency. |
| `harness.close_task intent=supersede` | May supersede the Task and update `project_state.active_task_id` in the same mutation, or commit blockers that invalidate supersession itself; appends events, creates replay row, and increments state once on commit. | Valid `dry_run=true` returns `ToolDryRunResponse`; preflight failures have no effect. Supersession is not evidence sufficiency. |

## Related owners

- [MVP API](api/mvp-api.md) for selected method behavior and response unions.
- [API Errors](api/errors.md) for rejected-response public errors.
- [Storage Records](storage-records.md) for records that effects may touch.
- [Artifact Storage](storage-artifacts.md) for staged-handle and artifact lifecycle details.
- [Storage Versioning](storage-versioning.md) for state clocks and replay/idempotency semantics.
