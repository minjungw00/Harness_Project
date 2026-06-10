# API Judgment Schemas

This document owns API schemas for user-owned judgment in the current MVP. It is documentation source material only and does not record user decisions by itself.

## Owns / Does not own

This document owns:

- `UserJudgment`
- `UserJudgmentCandidate`
- `UserJudgmentOption`
- `UserJudgmentContext`
- `UserJudgmentResolution`
- `RecordUserJudgmentPayload`
- `SensitiveActionScope`
- `AcceptedRiskInput`

This document does not own:

- the product meaning and non-substitution rules for user-owned judgment; see [Core Model](../core-model.md)
- method behavior for requesting or recording judgment; see [MVP API](mvp-api.md)
- active judgment-kind values; see [API Value Sets](schema-value-sets.md)
- final acceptance or residual-risk close effects; see [Core Model](../core-model.md) and [MVP API](mvp-api.md)

## Boundary

Judgment schemas preserve the structure of a user-owned choice. They do not let broad approval replace product decisions, technical decisions, scope decisions, sensitive-action approval, final acceptance, residual-risk acceptance, cancellation, later QA waiver, or later verification-risk acceptance.

## Related Owners

- [Core Model](../core-model.md) for user-owned judgment meaning.
- [MVP API](mvp-api.md) for `harness.request_user_judgment` and `harness.record_user_judgment`.
- [API Value Sets](schema-value-sets.md) for `judgment_kind`, `presentation`, `required_for`, and option values.
- [Later Candidate Index](../../later/index.md) for later judgment presentations and later/reserved judgment routes.
