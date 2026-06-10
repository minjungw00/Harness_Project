# Later: Policy and Conformance

## What this document owns

This document owns inactive later candidates about policy families, validators, gates, waivers, severity-based blocking, executable fixture families, and conformance entrypoints. It keeps policy and conformance candidates grouped so [Later Candidate Index](index.md) can remain a router and short summary.

Every candidate here is future-facing. The candidate details are documentation source material only and do not create current validator, gate, waiver, fixture, or conformance behavior.

## What this document does not own

This document does not define current MVP close-readiness requirements, active validators, active blocker categories, active waiver routes, executable fixture suites, conformance reports, API behavior, storage effects, security guarantees, or implementation readiness.

It also does not collapse policy, QA, verification, acceptance, waiver, and residual-risk judgment into one approval path. Those boundaries must be preserved during any promotion.

## Category boundary

This category is for candidates whose main question is "what normative policy or check could Harness define later?" It includes design policy, manual QA gates, verification gates, validator IDs, severity meanings, fixture families, conformance entrypoints, and connector conformance policy.

It does not own product connector surfaces, artifact capture mechanics, multi-user workflow, or assurance controls unless the candidate is expressed as policy or conformance. Cross-cutting candidates may also appear in another category later, but this document owns only the policy-and-conformance framing before promotion.

## Candidate summary

| Candidate | Summary |
|---|---|
| Manual QA workflow and `qa_gate` | Future manual QA gate and related policy candidate. |
| Manual QA waiver `qa_waiver` | Future waiver route for manual QA policy, without substituting for user-owned judgment. |
| Verification gate `verification_gate` | Future verification gate policy and close-readiness relationship. |
| Design gates and policy blockers | Future `design_gate`, `design_policy`, blocker categories, and design-quality policy. |
| Broad design validators and severity-based blocking | Future validator IDs, severity meanings, and blocking policy. |
| Full design-quality policy families | Future families such as `shared_design`, `domain_language`, `vertical_slice`, `feedback_loop`, `tdd_trace`, `deep_module_interface`, `codebase_stewardship`, and related policy families. |
| `ValidatorResult` stable IDs and policy families | Future stable validator identity, policy family, severity, and waiver vocabulary. |
| Future fixture and conformance entrypoints | Future fixture families, executable suite entrypoints, reports, and conformance run behavior. |
| Advanced validators and interface checks | Future advanced design, language, and interface-check validators. |

## Candidate details

### Manual QA workflow and `qa_gate`

Summary: Holds future manual QA gate policy and its relationship to close readiness.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Manual QA waiver `qa_waiver`

Summary: Tracks a future waiver route for manual QA policy without replacing user-owned judgment.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Verification gate `verification_gate`

Summary: Holds future verification gate policy and its possible relationship to close readiness.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Design gates and policy blockers

Summary: Tracks future `design_gate`, `design_policy`, design blocker categories, and close-readiness effects.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Broad design validators and severity-based blocking

Summary: Holds future validator IDs, severity meanings, and blocking policy for design quality.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Full design-quality policy families

Summary: Groups future design-quality policy families such as `shared_design`, `domain_language`, `vertical_slice`, `feedback_loop`, `tdd_trace`, `deep_module_interface`, `codebase_stewardship`, `manual_qa`, `two_stage_review_display`, detached-verification policy, and steward policy.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### `ValidatorResult` stable IDs and policy families

Summary: Tracks future stable validator identity, policy family, severity, and waiver vocabulary.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Future fixture and conformance entrypoints

Summary: Holds future fixture families, executable conformance run entrypoints, suite formats, assertions, and reports.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Advanced validators and interface checks

Summary: Tracks future advanced validators, design-policy validators, language checks, and interface checks.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

## Promotion rule

Promotion is not a local edit to this file. A candidate becomes active only when the current active scope and the relevant current owner documents are updated in the same documentation-only batch.

If no current owner exists for the promoted behavior, the promotion batch must create or designate that owner before defining active API, storage, security, UI, or conformance requirements.

## Active-scope non-effect

This document has no effect on the current MVP. Mentioning a candidate here does not activate a gate, blocker, validator, waiver, fixture suite, conformance runner, report, severity model, or close-readiness requirement.

## Related owners

- [Later Candidate Index](index.md)
- [Active MVP Scope](../reference/active-mvp-scope.md)
- [Reference Index](../reference/README.md)
- [Core Model](../reference/core-model.md)
- [Design Quality](../reference/design-quality.md)
- [Conformance](../reference/conformance.md)
- [API State Schemas](../reference/api/schema-state.md)
- [API Value Sets](../reference/api/schema-value-sets.md)
