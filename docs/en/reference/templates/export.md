# EXPORT Template

## Used when

Use `EXPORT` when an optional export or report projection is generated for review, archival, migration, or Release Handoff use.

## Source records

- included Task and gate records
- Change Units
- Runs
- approvals
- Evidence Manifests
- Eval records
- Manual QA records
- reconcile items
- projection snapshots and projection freshness
- artifact refs, redaction state, retention, and integrity metadata
- redaction, omission, and blocked-artifact summaries
- export profile boundary and non-deployment/non-merge reminder display

## Rendered sections

- Scope
- State Snapshots
- Projection Snapshots
- Artifact Refs
- Redaction Summary
- Omitted Or Blocked Content
- Integrity
- Release Handoff

## Full template

````md
---
doc_type: export_manifest
export_id: EXPORT-0001
project_id: PRJ-0001
profile: standard | release_handoff
export_bundle_status: current
source_state_version: 50
updated_at: 2026-05-06T10:30:00+09:00
---

# EXPORT-0001 Harness Export

> Projection view: rendered from `source_state_version` at `updated_at`; this export is a report snapshot. The Release Handoff/export authority boundary is owned by [Operations And Conformance](../operations-and-conformance.md#release-handoff-export-profile).

## Scope
- project_id:
- task_ids:
- included state version range:
- omitted by policy or profile:
- created by:
- created at:

## State Snapshots
- tasks:
- task gates:
- change units:
- runs:
- approvals:
- evidence manifests:
- Eval records:
- Manual QA records:
- reconcile items:

## Projection Snapshots
- TASK:
- APR:
- RUN-SUMMARY:
- EVIDENCE-MANIFEST:
- EVAL:
- DIRECT-RESULT:
- optional design projections:

## Artifact Refs
| Artifact ID | Kind | Owner Record | URI | SHA256 | Redaction State | Retention | Omission/Block Note |
|---|---|---|---|---|---|---|---|

## Redaction Summary
- secrets omitted:
- PII omitted:
- redacted artifacts:
- blocked artifacts:
- omission notes preserved:
- raw files excluded by policy:

## Omitted Or Blocked Content
| Artifact ID | Affected Owner Or Display | Redaction State | Downstream Effect | Note |
|---|---|---|---|---|

## Integrity
- export hash:
- manifest hash:
- generated at:

## Release Handoff
- close readiness:
- close blockers:
- evidence refs:
- verification refs:
- Manual QA refs:
- residual-risk refs:
- changed files:
- projection freshness:
- redaction/omission/block notes:
- suggested PR checklist:
- suggested deploy checklist:
- suggested rollback or monitoring notes:
- external authority reminder: Deployment, merge, approval, production monitoring, QA or verification waiver, gate satisfaction, final acceptance, residual-risk acceptance, assurance upgrade, and Task close remain outside this report.
````

## Notes

This template is a rendered shape, not canonical state. `EXPORT` is a `ProjectionKind` only; export snapshots and components remain artifacts linked to owner records or projection refs.

`EXPORT` must not embed raw secrets, PII, sensitive logs, network traces, screenshots, or other sensitive artifact bodies by default. Large or sensitive artifacts are listed by `ArtifactRef`; raw files are included only when policy and retention allow them, and `secret_omitted` or `blocked` entries stay represented by refs and notes.

If the export profile omits a projection snapshot, raw artifact, or state snapshot, show the omission and its review or Release Handoff impact rather than implying the bundle is complete.

For `secret_omitted`, export may include safe omission notes or handles and hashes over safe stored bytes, but not omitted values. For `blocked`, export may include the committed metadata-only notice artifact and its hash, size, and content type; those fields describe the notice bytes, never the forbidden raw payload. Release Handoff sections must show the same omission or block impact as unavailable, insufficient, or unresolved input unless a documented replacement, waiver, Decision Packet outcome, accepted risk, or fallback resolved it before export.
