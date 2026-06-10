# Agent Integration Reference

This document owns agent connector behavior for the current documentation set. It does not own surface-specific usage recipes; those live in [Surface Recipes](../use/surface-recipes.md).

This is documentation source material only. It does not implement a connector, MCP server, runtime state, generated manifest, or conformance runner.

## Owns / Does not own

This document owns:

- connector behavior and fallback semantics
- `capability_profile` meaning at the connector boundary
- context push/pull guidance
- verified surface context in agent packets
- detective display gating from capability checks
- one-language-per-`doc_id` retrieval guidance for agent context

This document does not own:

- surface-specific usage recipes; see [Surface Recipes](../use/surface-recipes.md)
- API methods or schemas; see API owners through [Reference Index](README.md)
- storage layout or artifact lifecycle; see storage owners
- security guarantee meanings; see [Security](security.md)
- exact template bodies; see [Template Bodies](template-bodies.md)

## Connector Boundary

Connectors carry context between Harness and an agent surface. A connector description, generated file, chat text, Product Repository file, projection, or agent memory does not prove authority by itself. Local surface authority depends on the registered and verified surface context defined by the API and security owners.

When Core, MCP, projection data, local access, or a capability is unavailable or insufficient, connector behavior should expose that limitation and route the next safe action to the relevant owner instead of fabricating authority.

## Context Discipline

Agent context should stay small: current task summary, scope/non-goals, pending user-owned judgments, blockers, next safe action, evidence gaps, close readiness, residual risk, guarantee level, source refs, and freshness. Do not inject full schemas, DDL, template bodies, historical logs, generated artifacts, or both languages for the same `doc_id` unless translation or semantic-parity review requires it.

## Related Owners

- [Surface Recipes](../use/surface-recipes.md) for practical surface-specific usage.
- [API Schema Core](api/schema-core.md) and [API Value Sets](api/schema-value-sets.md) for common API context fields and values.
- [MVP API](api/mvp-api.md) for method request conditions.
- [Security](security.md) for guarantee wording and non-claims.
- [Runtime Boundaries](runtime-boundaries.md) for Product Repository, Harness Server, and Runtime Home separation.
