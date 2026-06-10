# Security Reference

Use this page when security wording, local-access posture, trust boundaries, or guarantee labels need to stay honest. This is documentation source material only and does not implement security controls.

## Owns / Does not own

This document owns:

- security claims and non-claims
- trust-boundary wording
- cooperative and detective guarantee meanings
- profile-gated preventive or isolated wording boundaries
- sensitive-action approval versus product-file write-scope separation

This document does not own:

- API method behavior or schemas; see API owners through [Reference Index](README.md)
- storage layout, artifact lifecycle, locks, hashes, or migrations; see storage owners through [Reference Index](README.md)
- connector behavior or surface recipes; see [Agent Integration](agent-integration.md) and [Surface Recipes](../use/surface-recipes.md)
- OS-level enforcement, arbitrary-tool sandboxing, or deployment hardening

<a id="honest-guarantee-display"></a>
## Current MVP Guarantee Level

Current MVP wording is cooperative by default. Harness can guide, record, compare, or refuse owner-defined Harness state-changing paths when the connected surface follows the procedure. This is not hard blocking, OS permission, arbitrary-tool sandboxing, tamper-proof enforcement, or isolation.

Detective wording is allowed only when the relevant capability check has passed for the covered observable scope. A copied `surface_id`, generated file, projection, chat text, Product Repository file, or agent memory is not proof of capability.

Preventive and isolated claims are later/profile-gated unless a future owner documents the exact mechanism, scope, fallback behavior, and proof path.

## Explicit Non-Claims

The current MVP does not claim:

- OS-level permission control
- arbitrary-tool sandboxing
- tamper-proof local files or storage
- universal pre-tool blocking
- command, network, or secret observation by default
- command, network, or secret pre-tool blocking
- security isolation or permission isolation
- native artifact capture as an active guarantee

## Sensitive Action Boundary

Sensitive-action approval is a user-owned judgment for a named sensitive action. It is not product-file write scope, Write Authorization, final acceptance, residual-risk acceptance, evidence, artifact authority, OS permission, or tool sandboxing.

Product-file write scope is handled by the Core/API/storage owners for the write path. Broad approval does not substitute for either sensitive-action approval or product-file write compatibility.

## Related Owners

- [Core Model](core-model.md) for user-owned judgment and non-substitution meaning.
- [Runtime Boundaries](runtime-boundaries.md) for Product Repository, Harness Server, Runtime Home, and non-isolation boundaries.
- [Agent Integration](agent-integration.md) for connector capability profile behavior.
- [API Errors](api/errors.md) for public security-related error mapping.
- [Later Candidate Index](../later/index.md) for future operations and stronger guarantee candidates.
