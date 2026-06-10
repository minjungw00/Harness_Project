# Projection And Templates Reference

This document owns projection authority and derived-display boundaries. It no longer owns exact template bodies; those live in [Template Bodies](template-bodies.md).

This is documentation source material only. It is not a runtime projection, runtime state, generated artifact, evidence record, QA record, final-acceptance record, residual-risk record, close record, or implementation-ready server plan.

## Owns / Does not own

This document owns:

- projection authority boundaries
- read-only derived-display rules
- freshness and unavailable-state wording for projected output
- the rule that rendered labels are display text, not canonical schema values
- the later-only boundary for projection reconcile, persistent projection jobs, and managed block drift repair

This document does not own:

- exact template bodies; see [Template Bodies](template-bodies.md)
- source-of-truth Core state; see [Core Model](core-model.md)
- storage records or projection storage candidates; see storage owners and [Later Index](../later/index.md)
- public API schemas; see API schema owners
- connector behavior; see [Agent Integration](agent-integration.md)

## Authority Boundary

Core-owned state and persisted artifact references are the authority. A projection, status card, Markdown report, rendered template, chat message, connector output, or agent context packet is display or support context only.

Editing a rendered projection, Markdown status card, generated document, managed block, front matter, displayed state, artifact ref, close status, acceptance status, residual-risk status, or template text does not directly mutate Core state.

## Derived Display

Projection output is computed from current owner records at read time unless a future owner promotes a persisted projection job. It may help a person read scope, evidence gaps, blockers, freshness, next safe action, residual risk, and current guarantee wording. It is not a second state store.

Generated display must preserve omission, redaction, blocked-artifact, and unavailable notes without reconstructing hidden source values.

## Later Boundary

The current MVP has no active reconcile queue, editable projection input path, projection-to-Core repair path, persistent projection job, or managed block drift repair. Those remain later candidates until promoted with scope, fallback behavior, non-substitution rules, and proof expectations.

## Related Owners

- [Template Bodies](template-bodies.md) for exact rendered template text.
- [API State Schemas](api/schema-state.md) for state-shaped data used by displays.
- [API Artifact Schemas](api/schema-artifacts.md) for `ArtifactRef` display inputs.
- [Security](security.md) for guarantee wording.
- [Later Candidate Index](../later/index.md) for projection reconcile and persistent projection job candidates.
