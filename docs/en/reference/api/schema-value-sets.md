# API Value Sets

This document owns active API value sets and enum-like values for the current MVP. It is documentation source material only and does not widen active scope by naming later candidates.

## Owns / Does not own

This document owns:

- active public method-name values
- API `response_kind` and `effect_kind` values
- active `access_class` values
- active lifecycle, close-state, source-kind, judgment-kind, presentation, required-for, option, artifact, redaction, validator, guarantee-display, and similar API value sets
- the rule that rendered labels are not canonical schema values

This document does not own:

- public `ErrorCode` values or precedence; see [API Errors](errors.md)
- field shapes that use these values; see [API Schema Core](schema-core.md), [API State Schemas](schema-state.md), [API Artifact Schemas](schema-artifacts.md), and [API Judgment Schemas](schema-judgment.md)
- method behavior; see [MVP API](mvp-api.md)
- later candidate value names until promoted; see [Later Candidate Index](../../later/index.md)

## Boundary

Only values listed here by the active owner are active API values. Profile-gated values must name the profile or capability gate at the point of use. Later names remain catalog-only until a promoted owner adds exact active fields, fallback behavior, and proof expectations.

## Related Owners

- [Active MVP Scope](../active-mvp-scope.md) for whether a value belongs in the current MVP.
- [API Errors](errors.md) for public error codes.
- [API Schema Core](schema-core.md), [API State Schemas](schema-state.md), [API Artifact Schemas](schema-artifacts.md), and [API Judgment Schemas](schema-judgment.md) for fields that use these values.
- [Later Candidate Index](../../later/index.md) for inactive value names.
