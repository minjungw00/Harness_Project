# Later: Operations Profile

Use this page to route later operations, recovery, export, and handoff material without making it part of the MVP implementation path.

This is planning and navigation documentation for future Harness behavior. It does not authorize runtime/server implementation, generated operational files, executable fixtures, runtime data, or product code in this repository.

## Read This When

- You are checking what belongs after the Assurance Profile.
- You need operator, diagnostics, recovery, export, artifact-integrity, projection-refresh, or handoff owner links.
- You need to keep future operations work separate from Engineering Checkpoint and MVP-1 User Work Loop.

## Main Path

Start with the stage boundary in [MVP-1 User Work Loop](../build/mvp-user-work-loop.md), then use only the owner needed for the operations question:

| Need | Owner |
|---|---|
| Operator commands, diagnostics, recover, reconcile, export, artifact checks, and conformance run entrypoints | [Operations And Conformance Reference](../reference/operations-and-conformance.md) |
| Runtime layout, artifact storage, locks, migrations, projection jobs, and validator storage | [Storage Reference](../reference/storage.md) |
| Security posture, trust boundaries, threat categories, controls, and guarantee wording | [Security Reference](../reference/security.md) |
| Runtime spaces, Core placement, transaction order, projection/reconcile placement, and recovery overview | [Runtime Architecture Reference](../reference/runtime-architecture.md) |
| Projection freshness and rendered output boundaries | [Projection And Templates Reference](../reference/projection-and-templates.md) and [Template Reference](../reference/templates/README.md) |
| Operations fixture mechanics and future operations scenarios | [Conformance Fixtures Reference](../reference/conformance-fixtures.md) and [Future Fixtures](future-fixtures.md) |

## Boundary

Operations Profile is later than MVP-1 and Assurance Profile. It covers local operator readiness, diagnostics, recover/export, artifact integrity, projection freshness, conformance run entrypoints after runtime suites are materialized, and release handoff where owner docs define it.

It does not make Runtime Home tamper-proof, make projections authoritative, create a hosted dashboard, or provide OS-level sandboxing, arbitrary-tool permission control, preventive blocking, or isolation unless a promoted owner path proves that exact mechanism.
