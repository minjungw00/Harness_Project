# API artifact schemas

This document owns API artifact-shaped schemas for the current MVP. It is documentation source material only and does not grant local file access, create artifact bytes, create storage rows, or prove evidence sufficiency.

## Owns / Does not own

This document owns:

- `ArtifactRef`
- `ArtifactInput`
- `StagedArtifactHandle`
- staged versus existing artifact input distinctions
- artifact-shaped request and response fields for staging, linking, and body-read references
- artifact-related access constraints by reference
- redaction, availability, checksum, and size fields that appear on artifact-shaped API responses

This document does not own:

- artifact storage layout, staging records, promotion persistence, retention, or body-read storage eligibility; see [Artifact Storage](../storage-artifacts.md)
- method behavior for `harness.stage_artifact`, `harness.record_run`, or artifact reads; see [MVP API](mvp-api.md)
- active artifact value sets; see [API Value Sets](schema-value-sets.md)
- evidence sufficiency; see [Core Model](../core-model.md) and [API State Schemas](schema-state.md)
- security claims about access, blocking, or isolation; see [Security](../security.md)

## Boundary

Artifact schemas never make a caller-supplied path authoritative. New artifact bytes enter the active MVP only through `harness.stage_artifact`, which returns a temporary `StagedArtifactHandle`. Existing artifacts are linked only through compatible persistent `ArtifactRef` records. The storage and API method owners define validation, promotion, linking, and read eligibility.

## ArtifactRef

`ArtifactRef` is the public pointer to a persistent artifact that has already been registered by an owner path.

```yaml
ArtifactRef:
  artifact_id: string
  project_id: string
  task_id: string
  display_name: string
  content_type: string
  sha256: string
  size_bytes: integer
  redaction_state: string
  availability: string
  created_by_run_ref: StateRecordRef | null
  created_by_surface_id: string | null
  created_by_surface_instance_id: string | null
  storage_ref: string | null
```

`ArtifactRef` is a reference and metadata shape. It does not make artifact body content readable by default and does not prove that the content is sufficient evidence for close. Artifact body reads require the owner path with `access_class=artifact_read`.

## StagedArtifactHandle

`StagedArtifactHandle` is a temporary handle returned by successful `harness.stage_artifact`. It represents storage-owned temporary staging, not a persistent artifact.

```yaml
StagedArtifactHandle:
  handle_id: string
  project_id: string
  task_id: string
  created_by_surface_id: string
  created_by_surface_instance_id: string
  content_type: string
  sha256: string
  size_bytes: integer
  redaction_state: string
  expires_at: string
  consumed: boolean
```

The caller does not submit `created_by_surface_id` or `created_by_surface_instance_id` as authority claims. A future server records those fields from the verified local surface context of the staging request. A staged handle is scoped, expiring, and single-consumption. It is not a bearer token for any local caller and is not evidence authority until a compatible `harness.record_run` promotion creates a persistent `ArtifactRef`.

## ArtifactInput

`ArtifactInput` is used by methods that link artifacts into run or evidence output.

```yaml
ArtifactInput:
  artifact_input_id: string
  source_kind: string
  staged_artifact_handle: StagedArtifactHandle | null
  existing_artifact_ref: ArtifactRef | null
  relation_hint: string | null
  claim: string | null
  expected_sha256: string | null
  expected_size_bytes: integer | null
  redaction_state: string | null
```

Exactly one source field is active for each input:

| `source_kind` | Required source field | Meaning |
|---|---|---|
| `staged_artifact` | `staged_artifact_handle` | Promote a compatible temporary staged handle during `harness.record_run`. |
| `existing_artifact` | `existing_artifact_ref` | Link an already persistent same-project artifact without registering new bytes. |

`captured_artifact`, native capture handles, raw capture-adapter output, raw filesystem paths, arbitrary local path strings, and raw logs as authority claims are not active MVP `ArtifactInput` sources.

## Access constraints by reference

Artifact-shaped references are checked through owner paths:

- `harness.stage_artifact` uses `access_class=artifact_registration` and creates only a temporary `StagedArtifactHandle`.
- `harness.record_run` uses `access_class=run_recording`, even when `ArtifactInput[]` contains `source_kind=staged_artifact`.
- Staged promotion requires the current verified `surface_id` and `surface_instance_id` to match the staged handle's recorded `created_by_surface_id` and `created_by_surface_instance_id`.
- `existing_artifact` requires a persistent `ArtifactRef` that is valid for the same project and allowed Task scope.
- Artifact body reads are separate from staging and promotion; they require `access_class=artifact_read` and the artifact-body owner path.
- `ArtifactInput[]` does not add a second request-level access class to a public API request.

Invalid source-field shape and staged-handle validation failures return through `ToolRejectedResponse` with public error semantics owned by [API Errors](errors.md). Staged-handle storage validation and promotion lifecycle are owned by [Artifact Storage](../storage-artifacts.md).

## Related owners

- [MVP API](mvp-api.md) for artifact-related method behavior.
- [Artifact Storage](../storage-artifacts.md) for staging, promotion, persistent linking, and body-read lifecycle.
- [API Value Sets](schema-value-sets.md) for `ArtifactInput.source_kind`, `redaction_state`, availability, and related values.
- [API State Schemas](schema-state.md) for evidence summaries that mention `ArtifactRef`.
- [Runtime Boundaries](../runtime-boundaries.md) and [Security](../security.md) for local access and non-claim boundaries.
