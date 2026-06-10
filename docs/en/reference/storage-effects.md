# Storage Effects

This document owns method-to-storage effect semantics for the current MVP. It is documentation source material only and does not execute or simulate Harness runtime procedures.

## Owns / Does not own

This document owns:

- read-only, dry-run, rejected, staging-created, Core-committed, and committed-blocked storage-effect distinctions
- whether a method creates replay rows, task events, record changes, state-version increments, staged-handle consumption, artifact promotion, or Write Authorization changes
- the persistence boundary for blocker-like response data
- no-effect guarantees for rejected and valid dry-run preview branches

This document does not own:

- record shapes or DDL; see [Storage Records](storage-records.md)
- artifact lifecycle details; see [Artifact Storage](storage-artifacts.md)
- idempotency, locks, state-version clocks, or migrations; see [Storage Versioning](storage-versioning.md)
- public response branches or schemas; see [API Schema Core](api/schema-core.md)
- method behavior; see [MVP API](api/mvp-api.md)

## Boundary

Response data shape and storage effect are separate. A field such as `CloseReadinessBlocker` does not by itself prove persistence or mutation. Effects come from the selected method behavior and response branch.

## Related Owners

- [MVP API](api/mvp-api.md) for selected method behavior and response unions.
- [API Errors](api/errors.md) for rejected-response public errors.
- [Storage Records](storage-records.md) for records that effects may touch.
- [Storage Versioning](storage-versioning.md) for state clocks and replay/idempotency semantics.
