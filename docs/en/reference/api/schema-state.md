# API State Schemas

This document owns API state-shaped schemas for the current MVP. It is documentation source material only and does not create runtime state or generated projections.

## Owns / Does not own

This document owns:

- `StateSummary` and state-shaped public response fields
- `StateRecordRef`
- `ShapingReadiness` wire fields
- current-position display schemas such as `NextActionSummary`, `CloseReadinessBlocker`, and `ValidatorResult`
- the boundary between state-shaped data and response effects

This document does not own:

- common envelopes or response branches; see [API Schema Core](schema-core.md)
- active enum-like values; see [API Value Sets](schema-value-sets.md)
- method behavior; see [MVP API](mvp-api.md)
- Core lifecycle meaning; see [Core Model](../core-model.md)
- storage records or persistence effects; see [Storage Records](../storage-records.md) and [Storage Effects](../storage-effects.md)

## Boundary

State schemas describe API data shapes. A state-shaped field does not create persistence, a Core transition, replay rows, `task_events`, artifact effects, Write Authorization effects, or a `state_version` increment by itself. The response branch and method behavior owners define those effects.

## Related Owners

- [API Schema Core](schema-core.md) for `ToolEnvelope`, `ToolResultBase`, `ToolRejectedResponse`, and `ToolDryRunResponse`.
- [API Value Sets](schema-value-sets.md) for exact values used by state fields.
- [MVP API](mvp-api.md) for the methods that return these schemas.
- [Storage Effects](../storage-effects.md) for persistence and state-effect consequences.
