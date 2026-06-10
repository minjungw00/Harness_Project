# Glossary Reference

This document owns official terminology for Harness documentation. It defines terms for readers and translators; it does not define exact schemas, value sets, DDL, storage effects, security mechanisms, API behavior, or implementation sequencing.

## Owns / Does not own

This document owns:

- official English terminology for product, Core, API, storage, security, agent, projection, and later-candidate concepts
- term-level meaning for documentation prose
- links from terms to canonical technical owners

This document does not own:

- exact API field shapes or enum-like values; see API schema owners and [API Value Sets](api/schema-value-sets.md)
- public error codes; see [API Errors](api/errors.md)
- storage records, effects, artifacts, versioning, locks, or migrations; see storage owners through [Reference Index](README.md)
- template bodies; see [Template Bodies](template-bodies.md)
- implementation readiness; see [MVP Plan](../build/mvp-plan.md)

## Product Terms

| Term | Meaning | Owner |
|---|---|---|
| Harness | Planned local work-authority server for AI-assisted product work. | [Active MVP Scope](active-mvp-scope.md), [Runtime Boundaries](runtime-boundaries.md) |
| Product Repository | The user's project workspace. Product files are not Harness runtime state. | [Runtime Boundaries](runtime-boundaries.md) |
| Harness Runtime Home | Future operational data space for Harness records and artifacts. This documentation repo is not one. | [Runtime Boundaries](runtime-boundaries.md), storage owners |
| current MVP | The active product scope boundary for the first planned local work loop. | [Active MVP Scope](active-mvp-scope.md) |
| later candidate | Deferred material that is not active until an owner promotes it. | [Later Candidate Index](../later/index.md) |

## Core Terms

| Term | Meaning | Owner |
|---|---|---|
| Core-owned state | Harness-owned records that carry work authority. | [Core Model](core-model.md), storage owners |
| user-owned judgment | A decision Harness must ask or preserve instead of inferring. | [Core Model](core-model.md), [API Judgment Schemas](api/schema-judgment.md) |
| sensitive-action approval | User judgment for a named sensitive action; not Write Authorization or final acceptance. | [Core Model](core-model.md), [Security](security.md) |
| final acceptance | User judgment that accepts a result when the owner path requires it. | [Core Model](core-model.md) |
| residual-risk acceptance | User judgment that accepts a visible residual risk when required. | [Core Model](core-model.md) |
| close readiness | Whether the current work can be closed honestly, including remaining blockers. | [Core Model](core-model.md), [API State Schemas](api/schema-state.md) |

## API And Schema Terms

| Term | Meaning | Owner |
|---|---|---|
| `ToolEnvelope` | Common request envelope for public methods. | [API Schema Core](api/schema-core.md) |
| response branch | One of a method result, `ToolRejectedResponse`, or `ToolDryRunResponse`. | [API Schema Core](api/schema-core.md), [MVP API](api/mvp-api.md) |
| `ErrorCode` | Public API error identity. | [API Errors](api/errors.md) |
| `StateSummary` | API state-shaped summary. | [API State Schemas](api/schema-state.md) |
| `UserJudgment` | API shape for user-owned judgment records or candidates. | [API Judgment Schemas](api/schema-judgment.md) |
| `ArtifactRef` | Public pointer to a persisted artifact. | [API Artifact Schemas](api/schema-artifacts.md), [Artifact Storage](storage-artifacts.md) |
| API value set | Canonical list of active enum-like API values. | [API Value Sets](api/schema-value-sets.md) |

## Storage Terms

| Term | Meaning | Owner |
|---|---|---|
| storage record | Future persisted Harness record shape. | [Storage Records](storage-records.md) |
| storage effect | Whether a method branch changes storage or has no effect. | [Storage Effects](storage-effects.md) |
| artifact storage lifecycle | Staging, promotion, persistent linking, body-read eligibility, retention, and integrity. | [Artifact Storage](storage-artifacts.md) |
| state versioning | Public state clock, idempotency, locks, and migration semantics. | [Storage Versioning](storage-versioning.md) |

## Security And Agent Terms

| Term | Meaning | Owner |
|---|---|---|
| cooperative guarantee | Harness can guide, record, compare, or refuse owner-defined Harness state-changing paths when the surface follows the procedure. | [Security](security.md) |
| detective guarantee | Harness can report supported observable facts only after the relevant capability check has passed. | [Security](security.md), [Agent Integration](agent-integration.md) |
| `surface_id` | Surface identifier, not proof of authority by itself. | [Agent Integration](agent-integration.md), [Security](security.md) |
| `capability_profile` | Connector-owned description of supported surface capabilities. | [Agent Integration](agent-integration.md) |
| surface recipe | Practical usage guidance for a named surface. | [Surface Recipes](../use/surface-recipes.md) |

## Projection And Template Terms

| Term | Meaning | Owner |
|---|---|---|
| projection | Read-only derived display or support context from owner records. | [Projection And Templates](projection-and-templates.md) |
| rendered label | User-visible display text, not a canonical schema value. | [Projection And Templates](projection-and-templates.md), [Template Bodies](template-bodies.md) |
| template body | Exact rendered text owned separately from projection authority. | [Template Bodies](template-bodies.md) |

## Translation Owners

Korean terminology policy is shared with [Translation Guide](../maintain/translation-guide.md) and [docs/terminology-map.yaml](../../terminology-map.yaml). Exact identifiers remain unchanged across languages.
