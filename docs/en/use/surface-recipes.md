# Surface Recipes

This document owns surface-specific usage recipes for the current documentation set. It is Use documentation, not a connector contract, API schema, storage contract, security proof, or implementation guide.

## Owns / Does not own

This document owns:

- user-facing and agent-facing recipes for working through a named surface
- surface-specific phrasing for unavailable, insufficient, or capability-limited behavior
- recipe-level reminders about what a surface can ask Harness to check
- links from practical usage situations to exact Reference owners

This document does not own:

- connector behavior, `capability_profile`, context packet rules, or fallback semantics; see [Agent Integration](../reference/agent-integration.md)
- public API methods or schemas; see [Reference Index](../reference/README.md)
- security guarantees; see [Security](../reference/security.md)
- storage, artifacts, or projections; see the relevant Reference owners

## Recipe Boundary

Recipes must be practical and non-normative. If a recipe needs exact method behavior, schema fields, storage effects, security guarantees, or template bodies, add that contract to the canonical owner first and link from the recipe.

## Related Owners

- [Agent Integration](../reference/agent-integration.md) for connector behavior.
- [Active MVP Scope](../reference/active-mvp-scope.md) for what a recipe may treat as active.
- [MVP API](../reference/api/mvp-api.md) for method behavior.
- [Security](../reference/security.md) for guarantee wording.
