# Storage Reference Routing

This page remains only as a compatibility route for older links. It is not a canonical owner for storage contracts under the split ownership model.

## Owns / Does not own

This page owns:

- routing from older `storage.md` links to the current storage owner documents
- a reminder that documentation edits must not create runtime storage

This page does not own:

- storage records or DDL; see [Storage Records](storage-records.md)
- storage effects; see [Storage Effects](storage-effects.md)
- artifact storage lifecycle; see [Artifact Storage](storage-artifacts.md)
- state versioning, idempotency, locks, or migrations; see [Storage Versioning](storage-versioning.md)
- API schemas, method behavior, security claims, or runtime deployment

## Storage Owner Routes

| Need | Owner |
|---|---|
| Persistent records, DDL, record-column meaning, storage-owned JSON | [Storage Records](storage-records.md) |
| Method-to-storage effects and no-effect branches | [Storage Effects](storage-effects.md) |
| Staged artifacts, promotion, persistent linking, body-read eligibility, retention, integrity | [Artifact Storage](storage-artifacts.md) |
| Project-wide `state_version`, idempotency, locks, migrations | [Storage Versioning](storage-versioning.md) |
| Runtime Home separation | [Runtime Boundaries](runtime-boundaries.md) |

Storage owners describe future Harness Runtime Home records only. This documentation repository is not a Runtime Home and must not contain generated runtime state.
