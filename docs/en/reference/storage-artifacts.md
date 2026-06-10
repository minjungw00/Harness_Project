# Artifact Storage

This document owns the artifact storage lifecycle for the current MVP. It is documentation source material only and does not create artifact bytes, artifact directories, or runtime storage.

## Owns / Does not own

This document owns:

- staged artifact storage lifecycle
- `StagedArtifactHandle` validation against stored staging records
- promotion from a compatible staged handle to a persistent `ArtifactRef`
- persistent `existing_artifact` linking eligibility
- artifact body-read storage eligibility, availability, redaction, retention, and integrity boundaries

This document does not own:

- API artifact wire schemas; see [API Artifact Schemas](api/schema-artifacts.md)
- method behavior; see [MVP API](api/mvp-api.md)
- general record DDL; see [Storage Records](storage-records.md)
- generic storage effects; see [Storage Effects](storage-effects.md)
- local-access security claims; see [Security](security.md) and [Runtime Boundaries](runtime-boundaries.md)

## Boundary

Artifact storage distinguishes staging, promotion, persistent linking, and body reads. `existing_artifact` links an existing persistent artifact; it does not register new artifact body bytes. Staged handle shape is not authority unless it resolves to a compatible stored staging record.

## Related Owners

- [API Artifact Schemas](api/schema-artifacts.md) for `ArtifactRef`, `ArtifactInput`, and `StagedArtifactHandle` shapes.
- [MVP API](api/mvp-api.md) for `harness.stage_artifact`, `harness.record_run`, and artifact read behavior.
- [Storage Effects](storage-effects.md) for whether a response branch creates storage effects.
- [Security](security.md) for access and guarantee non-claims.
