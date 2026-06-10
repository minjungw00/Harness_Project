# Reference Index

Use this index to choose the single canonical owner for an exact Harness planning contract. This README routes; it does not define schemas, DDL, state effects, security guarantees, template bodies, or active MVP scope.

These documents remain source material for a future Harness Server. They do not mean this repository contains a server/runtime implementation, Harness Runtime Home, runtime state, generated projections, executable fixtures, or implementation-complete behavior.

## Reading Rules

- Pick one owner document for the contract question in front of you.
- Do not load all Reference docs by default.
- Do not load paired English and Korean docs for the same owner in one prompt unless the task is translation or semantic-parity review.
- If a non-owner repeats a contract, update the owner first, then replace the duplicate with a short consequence and a route here.

## Canonical Owners

| Contract area | Canonical owner |
|---|---|
| Current MVP scope | [active-mvp-scope.md](active-mvp-scope.md) |
| Core product concepts and user-owned judgment | [core-model.md](core-model.md) |
| API method behavior | [api/mvp-api.md](api/mvp-api.md) |
| Common API envelope and response branches | [api/schema-core.md](api/schema-core.md) |
| API state schemas | [api/schema-state.md](api/schema-state.md) |
| API artifact schemas | [api/schema-artifacts.md](api/schema-artifacts.md) |
| API judgment schemas | [api/schema-judgment.md](api/schema-judgment.md) |
| API value sets and enum-like values | [api/schema-value-sets.md](api/schema-value-sets.md) |
| Public API errors | [api/errors.md](api/errors.md) |
| Storage records | [storage-records.md](storage-records.md) |
| Storage effects | [storage-effects.md](storage-effects.md) |
| Artifact storage lifecycle | [storage-artifacts.md](storage-artifacts.md) |
| State versioning, idempotency, locks, and migrations | [storage-versioning.md](storage-versioning.md) |
| Runtime, repository, and server boundaries | [runtime-boundaries.md](runtime-boundaries.md) |
| Security claims and non-claims | [security.md](security.md) |
| Agent connector reference | [agent-integration.md](agent-integration.md) |
| Surface-specific usage recipes | [../use/surface-recipes.md](../use/surface-recipes.md) |
| Projection authority | [projection-and-templates.md](projection-and-templates.md) |
| Template bodies | [template-bodies.md](template-bodies.md) |
| Later candidates | [../later/index.md](../later/index.md) |
| Terminology | [glossary.md](glossary.md), [Translation Guide](../maintain/translation-guide.md), and [docs/terminology-map.yaml](../../terminology-map.yaml) |
| Documentation authoring rules | [Authoring Guide](../maintain/authoring-guide.md) |
| Documentation checks | [Checks](../maintain/checks.md) |

## Non-Owner Routing

README, Start, Use, Build, Maintain, and route/index documents may summarize reader-visible consequences, but they must not become secondary sources of truth for technical contracts. They should route to this index or to the canonical owner selected from it.

Broad compatibility pages that remain for older links, such as [storage.md](storage.md), are routing aids only when a split owner exists.
