# Build: MVP Plan

Use this as the Build entry point for pre-server documentation planning. It records repository status, implementation-readiness decisions, first smoke-target intent, and documentation-planning exit criteria.

Build docs are planning guidance only. They do not define exact MVP scope, schemas, enum value sets, DDL, API request/response shapes, storage tables, projection template bodies, fixture formats, or security guarantee claims. Those contracts stay with the owners linked from [Reference Index](../reference/README.md).

<a id="documentation-acceptance-status"></a>
## Repository Status

This repository is documentation-only and remains in documentation review. It is source material for a future Harness Server; it is not a Harness Server implementation, Harness Runtime Home, Product Repository, runtime record store, or implementation-complete behavior.

Server coding must not begin until maintainers resolve, accept, or explicitly defer the implementation-blocking decisions in [Implementation Decisions Before Server Coding](#implementation-decisions-before-server-coding) with named scope impact.

## What Exists Now

- Paired English and Korean planning docs in the Start, Use, Build, Reference, Later, and Maintain structure.
- Canonical Reference owner documents for current MVP scope, Core, API, storage, runtime boundaries, security, agent integration, projection authority, template bodies, conformance, design quality, and terminology.
- One Later candidate index for material outside the active MVP.
- `docs/doc-index.yaml` for bilingual retrieval routing and one-language-per-`doc_id` context discipline.
- This Build plan as the implementation-readiness entry point.

## What Does Not Exist Now

- Server/runtime implementation.
- Executable conformance runner.
- Generated runtime reports.
- Runtime state or generated projections.
- OS-level permission control.
- Arbitrary-tool sandboxing.
- Tamper-proof storage.
- Default pre-tool blocking.
- Active operations profile.

## Current MVP Scope

The canonical current MVP scope is [Active MVP Scope](../reference/active-mvp-scope.md). Build does not restate the scope list. If scope wording changes, update the scope owner first, then keep this plan as the readiness and sequencing route.

## Excluded Later Material

Later candidates and promotion boundaries are owned by [Later Candidate Index](../later/index.md). Build may name later exclusions only to explain readiness impact; it must not define later candidate contracts or promote them into the active MVP.

<a id="first-internal-smoke-target"></a>
## First Internal Smoke Target

The first internal smoke target is a documentation smoke target. It is not the product MVP, not a complete conformance suite, and not an implementation plan.

The target should exercise the owner boundaries that are riskiest for the first future runtime batch: current MVP scope, Core transitions, API response branches, storage effects, artifact staging and promotion, user-owned judgment, close readiness, security guarantee wording, connector fallback, and derived display authority.

Exact smoke examples, fixture shapes, public errors, schemas, state effects, and storage consequences must come from the canonical owners. This plan records why a smoke target is needed; it does not create executable fixtures, generated reports, runtime state, or implementation permission.

## User Work Loop

The user work loop should start or resume ordinary work without requiring the user to know Harness internal labels. It should clarify what the user wants, what the repository or future Harness state can support, what remains uncertain, and what judgment the user still owns.

The next safe action must remain visible. If Core, MCP, or a reference surface cannot support a claim, status must say so instead of fabricating authority.

## Request-To-Close Planning Route

Use [Reference Index](../reference/README.md) to choose the exact owner for each request-to-close contract. Build keeps only the planning sequence:

1. Shape or resume a Task from ordinary language.
2. Record scope and user-owned judgment through owner-defined paths.
3. Check product-write compatibility before product writes.
4. Record runs and durable evidence references through owner-defined paths.
5. Show status and compact output as derived reads.
6. Check close readiness while keeping evidence, final acceptance, residual-risk acceptance, and later QA/verification candidates distinct.

`compatible`, `blocked`, and `allowed` are future Harness record-compatibility results. They do not mean physical OS blocking, arbitrary-tool prevention, sandbox isolation, or permission isolation unless a future promoted mechanism proves that exact behavior.

<a id="implementation-decisions-before-server-coding"></a>
## Implementation Decisions Before Server Coding

Server coding must not begin until maintainers mark each row accepted, decided, or deferred with explicit scope impact.

| Decision item | Current status | What must be decided before coding |
|---|---|---|
| Implementation-planning readiness | Not maintainer-accepted. | Maintainers must accept that the compact documentation set is ready for the first runtime-batch plan, or name the blocker and affected scope. |
| Current MVP scope acceptance | Not maintainer-accepted for coding. | Maintainers must accept the [Active MVP Scope](../reference/active-mvp-scope.md) boundary or name unresolved scope impact. |
| Core and user-judgment acceptance | Not maintainer-accepted for coding. | Core transition meaning and user-owned judgment boundaries must be maintainer-accepted for active MVP paths. |
| Public API and schema acceptance | Not maintainer-accepted for coding. | Method behavior, common envelope/response branches, state/artifact/judgment schemas, value sets, public errors, idempotency/replay behavior, and unavailable Core/MCP behavior must be maintainer-accepted before affected tools or resources are coded. |
| Storage and runtime-home acceptance | Not maintainer-accepted for coding. | Storage records, storage effects, artifact lifecycle, versioning, locks, migrations, and Runtime Home boundaries must be maintainer-accepted before DDL, runtime data files, or artifact storage are created. |
| Security and local-access acceptance | Not maintainer-accepted for coding. | Local-only posture, cooperative/limited-detective guarantee wording, and non-claims must be maintainer-accepted before API/MCP exposure. |
| Surface and compact-output acceptance | Not maintainer-accepted for coding. | Connector behavior, surface recipes, compact display boundaries, freshness/unavailable behavior, projection authority, and template bodies must be maintainer-accepted before display or connector code is implemented. |

## Reference Owners

Use [Reference Index](../reference/README.md) for the canonical owner map. Build does not duplicate the owner table because the index is the routing source of truth.

## Exit Criteria For Documentation Planning

Documentation planning can exit only when maintainers explicitly confirm:

- this Build plan is the active Build entry point
- implementation-readiness decisions above have a maintainer decision, acceptance, or deferral with named scope impact
- the current MVP scope owner is accepted, or remaining boundary issues have named scope impact
- Reference owners agree on the active Core, API, storage, runtime-boundary, security, agent-integration, projection, template, conformance, design-quality, terminology, and later-candidate boundaries needed for the active MVP
- English and Korean Build pages preserve the same implementation decisions
- no later-candidate material is presented as required for the active MVP
- documentation remains source material only, with no server/runtime code, generated runtime state, executable fixture, conformance result, generated runtime report, or product implementation output created here

Passing these documentation-planning criteria does not implement Harness, prove runtime conformance, or close any future product work.
