# Build: First Runnable Slice

## What this document helps you do

This document turns the Build overview into the v0.1 Core Authority Slice an implementer should plan first.

This is planning documentation. It does not authorize runtime/server implementation, generated operational files, executable fixtures, fixture files, or runtime data before documentation acceptance and a separate implementation-planning readiness decision. Conformance fixture documentation is a future verification plan; the current documentation-only repository does not contain runnable Harness Server conformance tests. The first runnable target is v0.1 Core Authority Slice, with Kernel Smoke as its narrow conformance authoring profile. It is an internal implementation milestone, not the user-facing MVP. The first product MVP target is v0.2 User-Facing Harness MVP.

## Read this when

- You are planning v0.1 Core Authority Slice.
- You need a checklist for the first end-to-end authority path.
- You want to review whether a proposed first slice is small enough to run without becoming the product MVP.

## Before you read

Read [Implementation Overview](implementation-overview.md) first, including its [Documentation Acceptance Status](implementation-overview.md#documentation-acceptance-status). That handoff table is the Build entry gate; until maintainers accept implementation-planning readiness for the first runtime batch, this slice remains planning-only. For storage and DDL details, use [Storage And DDL](../reference/storage-and-ddl.md). For staged delivery after this slice, use [MVP Plan](mvp-plan.md). For v1+ Expansion candidates, use the [Roadmap](../roadmap.md).

## Main idea

Prove one Task can move through the smallest Core authority record: project registration, one basic scope represented by the Change Unit owner shape where the reference contract requires it, one write authorization decision, one authorized Run, one evidence link, and one structured blocker/status response.

The first slice should show that Harness state is local, durable, and authoritative without trying to prove the whole user-facing product. It keeps `prepare_write` as the product-write authorization decision point, Write Authorization as durable and single-use, `record_run` as the place where one compatible Run consumes authority, and `close_task` or status as the place where missing evidence or required judgment can be reported as structured blockers.

Use [Kernel Reference](../reference/kernel.md#prepare_write) and [MCP API And Schemas](../reference/mcp-api-and-schemas.md#public-tools) for the exact contracts.

## Goal

Plan v0.1 Core Authority Slice: the smallest Harness path that can prove local authority over one Task.

The slice should create or seed:

- one project and one reference surface
- one Task
- one basic scope for the intended change
- one allowed `prepare_write` decision and at least one blocked decision
- one durable single-use Write Authorization
- one compatible recorded Run that consumes the authorization
- one artifact or evidence ref linked to the Run or evidence relation
- one minimal evidence state that can support or fail the selected claim
- one read-only status/next response
- one structured blocker/status response when scope, evidence, or required seeded user judgment is missing

This is a command-independent implementation guide. It describes capabilities and observable behavior, not CLI syntax. Do not include or duplicate full DDL here. Storage details and DDL are owned by [Storage And DDL](../reference/storage-and-ddl.md).

The first slice is deliberately not the User-Facing Harness MVP, the hardened local reference target as a whole, a projection-template-polish milestone, dashboard or hosted-workflow-UI milestone, broad connector ecosystem or marketplace milestone, multi-surface connector expansion, Context Index, Browser QA Capture system, Cross-Surface Verification path, hook expansion, preventive guard expansion, Advanced Sidecar Watcher, Local Derived Metrics surface, team workflow, export/recover path, release handoff path, or parallel automation path.

## Success story

An implementer can run a local Harness process against a temporary product repository and observe this story:

1. Harness registers one project and one reference surface.
2. A Task exists in Core state and changes append `task_events`.
3. A basic scope names the intended product change.
4. `prepare_write` blocks a missing or incompatible scope.
5. `prepare_write` allows one compatible scoped write and creates a durable single-use Write Authorization.
6. `record_run` records one compatible Run and consumes that authorization once.
7. One artifact or evidence ref is registered and linked to the Run or evidence relation.
8. Status and next reads show current Task, scope, write authority, evidence state, and blockers without mutating state.
9. Close or status output returns a structured blocker when required evidence, scope, or required seeded user judgment is missing.

Passing this story means v0.1 Core Authority Slice works. It does not mean users have experienced the Harness MVP yet. The user-facing MVP begins when ordinary requests are clarified into scope, user-owned judgment, evidence, close-readiness, final-acceptance, and residual-risk language.

## Doc-level acceptance checks

Use these checks to review the planned v0.1 Core Authority Slice before executable fixtures exist, and again when mapping the slice to the [Kernel Smoke Authoring Queue](../reference/conformance-fixtures.md#kernel-smoke-authoring-queue). They are planning checks, not fixture body fields, schema additions, DDL, or runtime authorization.

A proposed first runnable slice is acceptable when:

- It remains local, single-project, single-reference-surface, and focused on one Task authority loop.
- It stays planning-only until the [Documentation Acceptance Status](implementation-overview.md#documentation-acceptance-status) explicitly marks implementation-planning readiness as accepted for the first runtime batch.
- It proves exactly one scoped write path: active Task, one basic scope, `prepare_write` allow/block, durable single-use Write Authorization, `record_run` consumption, artifact/evidence link, read-only status/next, and structured blocker/status response.
- It blocks or refuses missing authority: missing scope, out-of-scope intended path, missing Write Authorization for product-write Runs, reuse of a consumed Write Authorization, missing required evidence, or missing required seeded user judgment.
- It keeps status reads, generated prose, and any projection output downstream from Core records; none of them authorize writes, satisfy evidence, close work, repair state, or become conformance truth by being read.
- It links strict fixture body shape, assertion modes, primary errors, artifact refs, projection assertions, and seed validation to [Conformance Fixtures Reference](../reference/conformance-fixtures.md#conformance-fixture-format) instead of copying those contracts here.
- It names excluded capabilities as not yet proven by v0.1 Core Authority Slice, not as failed first-slice requirements.

The build order below is a post-acceptance, post-readiness planning sequence. The headings use implementation verbs so the future runtime batch is easy to execute, but this document still does not authorize runtime/server implementation, generated operational files, executable fixtures, or runtime data before documentation acceptance and a separate implementation-planning readiness decision.

## Build order

### 1. Runtime Home And Project Registration

Plan enough runtime home support to create local Harness authority outside chat history and generated Markdown, then register exactly one local product repository.

Planning focus:

- Make one local project and one reference surface resolvable for later Task-scoped actions.
- Keep the runtime home, registry, project state, artifact store, and static project configuration in the storage owner path.
- Provide a read-only status that can report an unregistered, registered-idle, or active-work state.

Done when:

- A fresh environment can be initialized repeatedly without creating duplicate authority records.
- Core can resolve the current project for all later Task-scoped actions.
- Status can distinguish an unregistered or idle project from an active Task.

Owner contracts: runtime home layout and DDL are owned by [Storage And DDL](../reference/storage-and-ddl.md); local spaces and surface guarantees are owned by [Runtime Architecture Reference](../reference/runtime-architecture.md) and [Agent Integration Reference](../reference/agent-integration.md).

### 2. One Task Record

Create the first Task through Core or a fixture seed path that uses the same validation rules.

Planning focus:

- Create or seed exactly one active Task through an owner-valid path.
- Keep enough current state and event history for status and later Core actions to refer to the Task.
- Keep mode policy depth, intake quality, and procedural budget routing for v0.2 User-Facing Harness MVP.

Done when:

- The system can show one active Task and its state version.
- A state-changing request with a stale expected state version is rejected or returns a state conflict where the owner contract requires it.

Owner contracts: Task lifecycle and state conflict behavior are owned by [Kernel Reference](../reference/kernel.md#task), [Lifecycle and transitions](../reference/kernel.md#lifecycle-and-transitions), and [MCP API And Schemas](../reference/mcp-api-and-schemas.md#state-conflict-behavior).

### 3. One Basic Scope

Add the smallest scope record that can constrain one intended product write. A Change Unit may be the owner shape, but the first slice should not expand into dependency graphs, full Autonomy Boundary policy, or multi-lane orchestration.

Planning focus:

- Attach one owner-valid scope to the active Task.
- Make the selected intended write checkable against that scope.
- Keep only the evidence expectation needed for the first selected claim.
- Keep full Discovery and user-facing procedural budget routing for v0.2.

Done when:

- Status can explain what may change.
- Product writes without an active compatible scope cannot receive write authority.

Owner contracts: Change Unit and Autonomy Boundary semantics are owned by [Kernel Reference](../reference/kernel.md#change-unit) and [Autonomy Boundary](../reference/kernel.md#autonomy-boundary).

### 4. `prepare_write` Allow/Block

Implement the first meaningful write gate.

Planning focus:

- Route the selected product-write attempt through the owner `prepare_write` path.
- Allow exactly one compatible scoped write or return an owner-shaped blocker.
- Keep candidate Approval or Decision Packet material as candidate context until the owning path commits it.

Done when:

- Missing scope blocks.
- Out-of-scope intended paths block.
- A compatible scoped write returns a Write Authorization ref.
- No product write can be recorded by a product-write Run without that ref.

Owner contracts: write-gate semantics are owned by [Kernel Reference: prepare_write](../reference/kernel.md#prepare_write); public request/response shape and error precedence are owned by [`harness.prepare_write`](../reference/mcp-api-and-schemas.md#harnessprepare_write) and [Primary Error Code Precedence](../reference/mcp-api-and-schemas.md#primary-error-code-precedence).

### 5. `record_run`

Record one direct Run or implementation Run and consume the Write Authorization.

Planning focus:

- Record one owner-valid Run for the selected direct or implementation write.
- Consume the compatible Write Authorization once.
- Keep observed changes, artifacts, events, and state updates in the Core transaction model.

Done when:

- `record_run` without write authority is blocked.
- `record_run` with compatible authority succeeds once.
- A second distinct Run cannot reuse the consumed authorization.

Owner contracts: Run semantics are owned by [Kernel Reference: record_run](../reference/kernel.md#record_run); public schema is owned by [`harness.record_run`](../reference/mcp-api-and-schemas.md#harnessrecord_run); transaction ordering is owned by [State transaction flow](../reference/runtime-architecture.md#state-transaction-flow).

### 6. Artifact Or Evidence Link

Register one durable evidence file or equivalent evidence ref through the owner path.

Planning focus:

- Register one artifact or evidence ref through an owner path.
- Link it to the Run, evidence relation, or other owner record that uses it.
- Preserve redaction, omission, integrity, and retention boundaries through the storage/API owners.

Done when:

- A Run can cite a registered artifact or evidence ref.
- Raw secrets are omitted or blocked rather than stored as evidence.

Owner contracts: artifact refs are owned by [ArtifactRef](../reference/mcp-api-and-schemas.md#artifactref); storage layout and registration details are owned by [Artifact directory layout](../reference/storage-and-ddl.md#artifact-directory-layout) and [Artifact Registration Contract](../reference/storage-and-ddl.md#artifact-registration-contract).

### 7. Minimal Evidence State

Create the smallest evidence relation needed to explain whether the selected claim is supported.

Planning focus:

- Map one selected claim to the Run and artifact/evidence ref.
- Preserve the distinction between supported, partial, and insufficient evidence through the owner gate semantics.
- Avoid treating chat text, status prose, or projection prose as evidence.

Done when:

- A completed Run can produce a supported or partial evidence state.
- Missing required evidence causes close/status output to block.

Owner contracts: evidence records and close-support categories are owned by [Evidence Manifest](../reference/kernel.md#evidence-manifest), [Evidence Gate](../reference/kernel.md#evidence-gate), and [`harness.close_task`](../reference/mcp-api-and-schemas.md#harnessclose_task).

### 8. Status, Next, And Structured Blockers

Expose current work state and safe next action without mutation, and return structured blockers when the first slice cannot close or proceed.

Planning focus:

- Return current Task, scope, write-authority summary, evidence status, blockers, and next safe action from canonical records.
- Keep blocker identity structured enough for fixtures without making prose authoritative.
- Do not append events, enqueue projections, create artifacts, satisfy gates, authorize writes, or close the Task from a read.

Done when:

- Repeated status/next reads return the same state version unless another action changed state.
- The structured blocker can be compared by fixtures without matching prose.
- Close/status results are based on canonical records, not rendered reports.

Owner contracts: status/next schemas are owned by [`harness.status`](../reference/mcp-api-and-schemas.md#harnessstatus) and [`harness.next`](../reference/mcp-api-and-schemas.md#harnessnext); close behavior is owned by [Kernel Reference: close_task](../reference/kernel.md#close_task).

## What this proves

The first runnable slice proves:

- Core can own state transitions.
- The state store and `task_events` are usable.
- A scoped record is required for product writes.
- `prepare_write` is the product-write authorization decision point.
- Write Authorization is durable and single-use.
- `record_run` consumes write authority once and records observed work.
- One artifact/evidence link can support the recorded Run.
- Evidence can be insufficient without relying on chat.
- Status and next are read-only.
- Structured blockers can report missing scope, evidence, authorization, or seeded required user judgment.

## What this does not prove yet

This slice does not prove the items below. They are stage boundaries, not failed v0.1 requirements.

| Later stage | Not yet proven by v0.1 Core Authority Slice |
|---|---|
| v0.2 User-Facing Harness MVP | Natural-language intake quality, Discovery, product/UX versus architecture judgment presentation, small-change versus tracked-work budgets, residual-risk display, final acceptance separation, user-facing projection/card sufficiency. |
| v0.3 Agency Assurance Pack | Full Decision Packet quality, full Approval lifecycle and drift handling, detached verification independence, Manual QA policy matrix, residual-risk accepted close, final acceptance separation, feedback-loop policy, TDD trace, codebase stewardship, stewardship validators, context hygiene. |
| v0.4 Operations & Handoff Pack | Release handoff, recover, export, artifact integrity operations, broad operator smoke, broader fixture suite coverage. |
| v1+ Expansion | Dashboard, hosted workflow UI, Context Index, connector marketplace, Browser QA Capture, Cross-Surface Verification automation, native hook expansion, Advanced Sidecar Watcher, Local Derived Metrics, preventive guard expansion, parallel orchestration, team workflow. |

## Fixtures to write

After documentation acceptance and implementation-planning readiness handoff, write Core Authority Slice fixtures that drive Core behavior and assert state, events, artifacts, projection freshness when applicable, and errors. Do not assert success by matching rendered prose or polished projection output. These rows are future authoring candidates; they do not imply executable fixture files exist now.

Build owns the coverage intent for v0.1: project/status, one active Task, one basic scope, `prepare_write` allow/block, `record_run` consume/block, one artifact/evidence link, minimal evidence sufficiency, read-only status/next, and close/status blockers. Projection polish and detailed templates are not v0.1 fixture requirements. The exact fixture queue, body fields, seed rules, assertion modes, stable events, artifact/projection assertions, and primary-error expectations are owned by the [Kernel Smoke Authoring Queue](../reference/conformance-fixtures.md#kernel-smoke-authoring-queue) and [Conformance Fixture Format](../reference/conformance-fixtures.md#conformance-fixture-format).

Do not add fields to the fixture body to express suite stage, authoring order, or docs-maintenance results.

## Reference docs to consult

- [Kernel Reference](../reference/kernel.md): Task, Change Unit, Decision Packet, gates, `prepare_write`, Write Authorization, `record_run` semantics, and `close_task`.
- [Runtime Architecture Reference](../reference/runtime-architecture.md): three spaces, Core process model, transaction flow, artifact store, projection/reconcile, guarantee levels, and failure handling.
- [MCP API And Schemas](../reference/mcp-api-and-schemas.md): public resources, tool envelopes, request/response schemas, error taxonomy, artifact refs, and `ProjectionKind`.
- [Storage And DDL](../reference/storage-and-ddl.md): runtime layout, DDL, migrations, locks, artifacts, baselines, projection jobs, and validator-run storage.
- [Operations And Conformance Reference](../reference/operations-and-conformance.md): operator semantics and conformance staging.
- [Conformance Fixtures Reference](../reference/conformance-fixtures.md): fixture format, execution, assertion rules, suite catalogs, and examples.
