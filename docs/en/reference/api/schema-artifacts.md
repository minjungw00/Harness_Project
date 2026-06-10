# API Artifact Schemas

This document owns API artifact-shaped schemas for the current MVP. It is documentation source material only and does not grant local file access or create artifact storage.

## Owns / Does not own

This document owns:

- `ArtifactRef`
- `ArtifactInput`
- `StagedArtifactHandle`
- artifact-shaped request and response fields for staging, linking, and body-read references
- API redaction and availability fields that appear on artifact-shaped responses

This document does not own:

- artifact storage layout, staging records, promotion persistence, retention, or body-read storage eligibility; see [Artifact Storage](../storage-artifacts.md)
- method behavior for `harness.stage_artifact`, `harness.record_run`, or artifact reads; see [MVP API](mvp-api.md)
- active artifact value sets; see [API Value Sets](schema-value-sets.md)
- security claims about access or isolation; see [Security](../security.md)

## Boundary

Artifact schemas never make a caller-supplied path authoritative. New artifact bytes enter the active MVP only through the active staging path, and existing artifacts are linked only through compatible persistent `ArtifactRef` records. The storage and API method owners define validation, promotion, linking, and read eligibility.

## Related Owners

- [MVP API](mvp-api.md) for artifact-related method behavior.
- [Artifact Storage](../storage-artifacts.md) for staging, promotion, persistent linking, and body-read lifecycle.
- [API Value Sets](schema-value-sets.md) for `ArtifactInput.source_kind`, `ArtifactRef.kind`, and related values.
- [Runtime Boundaries](../runtime-boundaries.md) and [Security](../security.md) for local access and non-claim boundaries.
