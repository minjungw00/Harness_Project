# Active MVP Scope Reference

This document owns the current MVP scope boundary for Harness planning documentation. It is documentation source material only. It does not mean a Harness Server, runtime state, generated projection system, conformance runner, or implementation-complete behavior exists in this repository today.

## Owns / Does not own

This document owns:

- the closed current MVP capability list
- the active/later boundary at the product-scope level
- scope-level non-claims that keep later candidates out of the active MVP
- routing from each active scope item to its exact contract owner

This document does not own:

- implementation sequencing, maintainer readiness, or server-coding handoff; see [MVP Plan](../build/mvp-plan.md)
- Core transition meaning or user-owned judgment boundaries; see [Core Model](core-model.md)
- API method behavior; see [MVP API](api/mvp-api.md)
- API schemas and value sets; see [API Schema Core](api/schema-core.md), [API State Schemas](api/schema-state.md), [API Artifact Schemas](api/schema-artifacts.md), [API Judgment Schemas](api/schema-judgment.md), and [API Value Sets](api/schema-value-sets.md)
- storage records, effects, artifacts, versioning, locks, or migrations; see [Storage Records](storage-records.md), [Storage Effects](storage-effects.md), [Artifact Storage](storage-artifacts.md), and [Storage Versioning](storage-versioning.md)
- security guarantee meanings; see [Security](security.md)
- later candidate ownership; see [Later Candidate Index](../later/index.md)

## Current MVP Scope

The current MVP is limited to the smallest local work-authority loop that preserves scope, user-owned judgment, write compatibility, evidence references, artifacts, close readiness, final acceptance, and residual risk without claiming tool sandboxing or runtime isolation.

The active MVP includes only:

- plain-language intake and Task creation through `harness.intake`
- scope and Change Unit updates through `harness.update_scope`
- compact active-state display through `ShapingReadiness`
- user-owned judgment requests and recorded answers through `harness.request_user_judgment` and `harness.record_user_judgment`
- sensitive-action approval as the active `sensitive_approval` judgment path
- path-level `harness.prepare_write` and single-use Write Authorization for product-file writes
- `harness.stage_artifact` for active staged artifact intake
- `harness.record_run` for recording shaping, direct, and implementation Runs, including compatible persistent artifact promotion or linking
- compact `EvidenceSummary`
- `harness.status` read-time state output
- `harness.close_task` close-readiness checks and close/cancel/supersede outcomes
- read-time projection/status output as derived display
- registered local surface access for the reference local MCP surface
- cooperative guarantee display
- detective guarantee display only after the relevant capability check has passed for the covered observable scope

## Out Of Scope Until Promoted

The active MVP does not include native artifact capture, `captured_artifact`, projection reconcile, persistent projection jobs, managed block drift repair, full Evidence Manifest, Manual QA workflow, `qa_gate`, `verification_gate`, command/network/secret observation, command/network/secret pre-tool blocking, preventive guarantees, isolated guarantees, hosted dashboards, connector marketplaces, export/handoff formats, executable fixture runners, generated conformance artifacts, or operations profiles.

Mentioning a later candidate in examples or route text does not promote it. Promotion requires a named owner to add scope, fallback behavior, proof expectations, and paired English/Korean documentation.

## Related Owners

| Need | Owner |
|---|---|
| Implementation readiness and maintainer handoff status | [MVP Plan](../build/mvp-plan.md) |
| Core meaning and user-owned judgment | [Core Model](core-model.md) |
| API methods | [MVP API](api/mvp-api.md) |
| API schemas and value sets | [API Schema Core](api/schema-core.md), [API State Schemas](api/schema-state.md), [API Artifact Schemas](api/schema-artifacts.md), [API Judgment Schemas](api/schema-judgment.md), [API Value Sets](api/schema-value-sets.md) |
| Storage | [Storage Records](storage-records.md), [Storage Effects](storage-effects.md), [Artifact Storage](storage-artifacts.md), [Storage Versioning](storage-versioning.md) |
| Projection authority and template bodies | [Projection And Templates](projection-and-templates.md), [Template Bodies](template-bodies.md) |
| Surface usage and connector behavior | [Surface Recipes](../use/surface-recipes.md), [Agent Integration](agent-integration.md) |
| Security claims and non-claims | [Security](security.md) |
| Later material | [Later Candidate Index](../later/index.md) |
