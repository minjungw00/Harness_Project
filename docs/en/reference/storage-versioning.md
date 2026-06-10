# Storage Versioning

This document owns state versioning, idempotency, locks, and migration semantics for current MVP storage. It is documentation source material only and does not run migrations or create runtime locks.

## Owns / Does not own

This document owns:

- the public project-wide `project_state.state_version` conflict basis
- state-version increment rules at the storage-semantics level
- idempotency and request-hash replay semantics
- lock policy
- migration semantics and active/later migration boundaries

This document does not own:

- record shapes or DDL; see [Storage Records](storage-records.md)
- which method branch produces an effect; see [Storage Effects](storage-effects.md) and [MVP API](api/mvp-api.md)
- public error codes and precedence; see [API Errors](api/errors.md)
- runtime deployment or operational commands

## Boundary

The current MVP public conflict clock is project-wide unless a future owner promotes a different clock with scope and proof expectations. A task-scoped clock may exist only as a non-public or later boundary until promoted by the owner.

## Related Owners

- [API Errors](api/errors.md) for public conflict errors such as `STATE_VERSION_CONFLICT`.
- [Storage Effects](storage-effects.md) for branches that increment or do not increment state.
- [Storage Records](storage-records.md) for columns that store versioning or replay data.
- [Runtime Boundaries](runtime-boundaries.md) for Runtime Home separation.
