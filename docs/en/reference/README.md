# Reference Index

Use Reference when you need the exact owner contract for a schema, gate, state transition, DDL profile, projection rule, template body, security meaning, conformance rule, connector behavior, policy, or term.

These owner docs describe future Harness Server contracts for planning and review. They do not mean a server/runtime, Harness Runtime Home, conformance runner, generated projection system, or implementation exists in this repository today.

Do not read the whole Reference set by default. Choose the owner for the question in front of you, then follow its links only when that owner delegates a stricter detail.

## Owner-Contract Map

| Contract area | Owner |
|---|---|
| Task, scope/Change Unit, `user_judgment`, `evidence_ref`, blocker and close-readiness meaning, gates, state transitions, `prepare_write`, `record_run`, `close_task`, state invariants, and non-substitution rules | [Core Model Reference](core-model.md) |
| Active MVP-1 public methods and per-method request/response behavior | [MVP API](api/mvp-api.md) |
| MVP-1 shared schemas, envelopes, read-only resources, refs, `ArtifactRef`, `ValidatorResult`, staged active value sets, and API-owned enums | [API Schema Core](api/schema-core.md) |
| Error taxonomy, user-facing error labels, primary error precedence, close-blocker error mapping, idempotency, and state conflict behavior | [API Errors](api/errors.md) |
| Later/profile-gated API methods, schema branches, enum extensions, and future validator IDs | [API Schema Later](api/schema-later.md); use [Assurance Profile](../later/assurance-profile.md) for the later reader path. |
| Runtime home layout, persisted state model, DDL profiles, storage-owned JSON `TEXT`, artifact storage, migrations, locks, baseline capture, projection-job storage, and validator-run storage | [Storage](storage.md) |
| Derived views, status cards, agent context packets, managed blocks, human-editable projection sections, template implementation classes, artifact-ref rendering, and freshness/failure behavior | [Projection And Templates Reference](projection-and-templates.md) |
| Full rendered template bodies and display card shapes | [Template Reference](templates/README.md) |
| Guarantee levels, threat model, assets, trust boundaries, threat/control categories, and honest security wording | [Security Reference](security.md) |
| How agents interact with Core/API without overloading context: connector profiles, generated manifests, context push/pull, fallback behavior, Role Lens, and reference-surface behavior | [Agent Integration Reference](agent-integration.md); [Surface Cookbook](surface-cookbook.md) owns surface recipes. |
| Operator behavior, diagnostics, conformance run entrypoints, recovery/export/reconcile operations, and docs-maintenance reporting | [Operations And Conformance Reference](operations-and-conformance.md); use [Operations Profile](../later/operations-profile.md) for the later reader path. |
| Fixture body shape, runner behavior, assertion semantics, fixture profiles, suite metadata boundaries, current-phase fixture status, and the reduced Kernel Smoke queue | [Conformance Fixtures Reference](conformance-fixtures.md) |
| Compact future scenario-family inventory, promotion criteria, suite-family labels, and catalog-only candidates | [Future Fixtures](../later/future-fixtures.md) |
| Design-quality policies, validator IDs, severity composition, waiver semantics, evidence expectations, and close impact | [Design Quality Policies](design-quality-policies.md) |
| Public/internal terminology definitions, capitalization, record-name orientation, and owner routing | [Glossary Reference](glossary.md) |
| Runtime spaces, Core process placement, Core-only mutation authority, transaction ordering, artifacts, projection/reconcile placement, and recovery overview | [Runtime Architecture Reference](runtime-architecture.md) |

## Reader Shortcuts

- If you are implementing the future server, use [Implementation Overview](../build/implementation-overview.md), then [MVP-1 User Work Loop](../build/mvp-user-work-loop.md) -> [MVP API](api/mvp-api.md) -> [Storage](storage.md). Pull other Reference owners only for exact questions.
- If you are planning the first internal smoke, use [Engineering Checkpoint](../build/engineering-checkpoint.md), then [Core Model Reference](core-model.md), [MVP API](api/mvp-api.md), and [Storage](storage.md).
- If you are writing agent instructions, start with [Agent Guide](../use/agent-guide.md), then use [Agent Integration Reference](agent-integration.md) and [Surface Cookbook](surface-cookbook.md) only for connector-specific contracts.
- If you are checking an MVP-1 method, start with [MVP API](api/mvp-api.md). If you are checking shared refs or envelopes, use [API Schema Core](api/schema-core.md). For later methods, use [API Schema Later](api/schema-later.md) and keep them out of the MVP path unless promoted.
- If you are checking a persisted shape, start with [Storage](storage.md).
- If you are checking a `harness://` resource, start with the staged [Read-only resources](api/schema-core.md#read-only-resources) table before treating a URI as required for a delivery stage.
- If you are checking a user-facing wording claim, start with the owner of the underlying fact. Projection and template docs control display, but they do not create authority.
- If you are checking future assurance, operations, or fixture catalog material, use [Assurance Profile](../later/assurance-profile.md), [Operations Profile](../later/operations-profile.md), and [Future Fixtures](../later/future-fixtures.md). These are not the MVP implementation path.
