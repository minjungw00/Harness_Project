# Storage Records

This document owns persistent storage record shapes for the current MVP. It is documentation source material only and does not create a runtime database, generated records, or implementation-ready DDL in this repository.

## Owns / Does not own

This document owns:

- persistent record families such as project state, tasks, change units, user judgments, runs, evidence summaries, write authorizations, surfaces, replay rows, and task events
- DDL ownership and record-column meaning for active current MVP storage
- storage-owned JSON field placement
- record-level active/later exclusions

This document does not own:

- method-to-storage effects; see [Storage Effects](storage-effects.md)
- artifact staging, promotion, linking, body reads, retention, or integrity lifecycle; see [Artifact Storage](storage-artifacts.md)
- `state_version`, idempotency, locks, and migrations; see [Storage Versioning](storage-versioning.md)
- API wire schemas; see the API schema owners under [API](api/schema-core.md)
- runtime/repository/server boundaries; see [Runtime Boundaries](runtime-boundaries.md)

## Boundary

Storage records are future Harness Runtime Home records, not files in this documentation repository. Documentation edits must not create runtime state, generated records, operational artifacts, conformance output, or acceptance records.

## Related Owners

- [Storage Effects](storage-effects.md) for which methods create, update, observe, or leave records untouched.
- [Artifact Storage](storage-artifacts.md) for artifact-specific storage lifecycle.
- [Storage Versioning](storage-versioning.md) for clocks, idempotency, locks, and migration semantics.
- [MVP API](api/mvp-api.md) for public method behavior that uses records.
