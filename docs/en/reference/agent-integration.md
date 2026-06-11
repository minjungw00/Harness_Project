# Agent integration reference

This document owns agent connector behavior and capability-context boundaries for the current documentation set. It does not own surface-specific usage recipes; those live in [Surface Recipes](../use/surface-recipes.md).

This is documentation source material only. It does not implement a connector, MCP server, runtime state, generated manifest, or conformance runner.

## Owns / Does not own

This document owns:

- `capability_profile` meaning at the connector boundary
- `VerifiedSurfaceContext` meaning at the connector boundary
- guarantee display gating from verified capability context
- context push/pull guidance
- connector fallback semantics
- connector conformance boundary
- one-language-per-`doc_id` retrieval guidance for agent context

This document does not own:

- CLI, IDE/editor, chat, or local MCP surface recipes; see [Surface Recipes](../use/surface-recipes.md)
- API method behavior, request envelopes, or schema shapes; see API owners through [Reference Index](README.md)
- storage layout, artifact lifecycle, or staged-handle validation; see storage and artifact owners through [Reference Index](README.md)
- security guarantee meanings; see [Security](security.md)
- Product Repository, Harness Server, and Runtime Home separation; see [Runtime Boundaries](runtime-boundaries.md)
- exact template bodies; see [Template Bodies](template-bodies.md)

## Connector boundary

Connectors carry context between Harness and an agent surface.

Allowed effects:
- request owner-defined Harness state
- display owner results
- pass compact context to the agent

Non-claims:
- A connector description, generated file, chat text, Product Repository file, projection, or agent memory does not prove authority by itself.
- Prose or cached display text does not create Core state, user-owned judgment, Write Authorization, evidence sufficiency, artifact authority, close readiness, residual-risk acceptance, or security guarantees.

Local surface authority depends on the registered and verified surface context defined by the API and security owners.

## `capability_profile`

`capability_profile` is the connector-owned description of what a registered surface can support.

When the relevant owner documents make those concepts active, `capability_profile` may describe:
- supported access classes
- local reachability
- changed-path detection
- artifact staging or body-read support
- display capabilities

Conditions:
- Before a protected read, mutation, artifact operation, detective display, or guarantee claim relies on `capability_profile`, compare it with the registered local surface and the current request.
- Profile-gated behavior remains inactive until active-scope and owner documents promote it with scope, fallback behavior, and proof expectations.

Non-claims:
- `capability_profile` is not authority by itself.
- A stale, copied, generated, or user-provided capability description cannot make a later candidate active.
- The same description cannot justify a stronger guarantee level.

A connector should show missing support as unavailable or capability-limited instead of silently degrading into a stronger claim.

## `VerifiedSurfaceContext`

`VerifiedSurfaceContext` is the result a future server derives by matching a request's selected `surface_id` to registered local surface facts, transport/session/binding evidence, access class, and capability posture.

Conditions:
- The connector may pass `surface_id` as a selector.
- The connector does not get to assert `verified=true`.
- Protected reads and mutations can rely on a surface only when the API owner says the verified context is compatible with the method.

Results:
- A public API request has exactly one request-level `VerifiedSurfaceContext.access_class`.
- Nested payloads such as artifact inputs do not add a second request access class.
- In a future server, staged artifact provenance such as `created_by_surface_id` and `created_by_surface_instance_id` comes from `VerifiedSurfaceContext`.

Non-claims:
- Caller prose does not supply staged artifact provenance.
- Copied identifiers, generated Markdown, chat text, projection text, and agent memory are not substitutes for the verified context.

The exact request envelope and access-class values belong to the [MVP API router](api/mvp-api.md), method owner documents, and [API Value Sets](api/schema-value-sets.md).

## Guarantee display gating

Guarantee display starts at the current documented level: cooperative by default.

Conditions for limited `detective` display:
- the relevant capability verification passed
- the security owner allows that wording
- the displayed scope is limited to the named surface, capability, and observed scope

The connector should display these limitation conditions:
- Core, MCP, local access, changed-path detection, artifact access, or another required capability is unavailable
- a required capability is stale, mismatched, or insufficient

Non-claims:
- Do not infer `detective`, `preventive`, or `isolated` from a surface name.
- Do not infer a stronger guarantee level from a status card, chat summary, rendered projection, or user phrase.

The guarantee vocabulary and non-claims belong to [Security](security.md). Current MVP scope and profile-gated boundaries belong to [Active MVP Scope](active-mvp-scope.md).

## Context push and pull

A connector may push compact agent context when it is fresh enough for the next action. Keep the packet to:

- current task summary
- active scope and non-goals
- relevant surface status
- `state_version`
- pending user-owned judgments
- blockers
- next safe action
- evidence gaps
- artifact availability summary
- close readiness
- residual-risk status
- guarantee level
- source refs and freshness

A connector should pull exact owner sections only when the next action needs them.

Do not push:
- full schemas
- DDL
- template bodies
- historical logs
- generated artifacts
- full artifact contents
- unrelated contract material
- future catalog material
- both languages for the same `doc_id`, unless bilingual maintenance requires semantic-parity review

If a pushed context packet becomes stale, disconnected, or incompatible with the current surface, the connector should ask the owner path for a refreshed result or show the stale condition before the agent relies on it.

## Fallback semantics

When Core, MCP, projection data, local access, artifact access, or a capability is unavailable or insufficient, connector behavior should expose that limitation and route the next safe action to the relevant owner instead of fabricating authority.

Use owner-defined failure meanings. Typical routing is:

- `MCP_UNAVAILABLE`: Core, MCP, or required surface reachability is unavailable.
- `LOCAL_ACCESS_MISMATCH`: reachable local access does not match the registered surface expectation or has been revoked.
- `CAPABILITY_INSUFFICIENT`: the surface is recognized but lacks a required access class, observation, artifact capability, or guarantee support.

Fallback should be honest and small: reconnect or diagnose, move to a capable surface, narrow the operation, refresh state, request the missing user-owned judgment, or continue outside Harness only when the user explicitly chooses that mode.

## Connector conformance boundary

Connector conformance means preserving owner-defined results and not strengthening them. A conforming connector:

- derives authority from owner paths rather than generated or conversational text
- preserves the request-level `VerifiedSurfaceContext.access_class` boundary
- reports unavailable, mismatched, stale, or insufficient capability states without inventing Core records
- displays guarantee levels only when the relevant owner and capability check support them
- keeps user-owned judgment, sensitive-action approval, final acceptance, residual-risk acceptance, evidence sufficiency, and close readiness distinct
- keeps surface recipes in [Surface Recipes](../use/surface-recipes.md) instead of turning this reference into an operating manual

This boundary is a documentation contract for future connector behavior. It is not an executable conformance runner and does not create generated conformance output.

## Related owners

- [Surface Recipes](../use/surface-recipes.md) for practical surface-specific usage.
- [API Schema Core](api/schema-core.md) and [API Value Sets](api/schema-value-sets.md) for common API context fields and values.
- [MVP API router](api/mvp-api.md) and method owner documents for method request conditions.
- [Security](security.md) for guarantee wording and non-claims.
- [Runtime Boundaries](runtime-boundaries.md) for Product Repository, Harness Server, and Runtime Home separation.
- [Storage Records](storage-records.md), [Storage Effects](storage-effects.md), and [Artifact Storage](storage-artifacts.md) for storage and artifact authority boundaries.
