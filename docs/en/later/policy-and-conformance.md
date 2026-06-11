# Later: policy and conformance

<a id="what-this-document-owns"></a>
## Owns / Does not own

This document owns inactive later candidates about policy families, validators, gates, waivers, severity-based blocking, executable fixture families, and conformance entrypoints. It keeps policy and conformance candidates grouped so [Later Candidate Index](index.md) can remain a router and short summary.

Every candidate here is future-facing. The candidate details are documentation source material only and do not create current validator, gate, waiver, fixture, or conformance behavior.

<a id="what-this-document-does-not-own"></a>
This document does not define current MVP close-readiness requirements, active validators, active blocker categories, active waiver routes, executable fixture suites, conformance reports, API behavior, storage effects, security guarantees, or implementation readiness.

It also does not collapse policy, QA, verification, acceptance, waiver, and residual-risk judgment into one approval path. Those boundaries must be preserved during any promotion.

## Category boundary

This category is for candidates whose main question is "what normative policy or check could Harness define later?" It includes design policy, Manual QA gates, verification gates, validator IDs, severity meanings, fixture families, conformance entrypoints, and connector conformance policy.

It does not own product connector surfaces, artifact capture mechanics, multi-user workflow, or assurance controls unless the candidate is expressed as policy or conformance. Cross-cutting candidates may also appear in another category later, but this document owns only the policy-and-conformance framing before promotion.

## Candidate summary

| Candidate | Summary |
|---|---|
| Manual QA workflow and `qa_gate` | Future Manual QA gate policy and close-readiness relationship. |
| Manual QA waiver `qa_waiver` | Future waiver route for Manual QA policy without replacing user-owned judgment. |
| Verification gate `verification_gate` | Future verification gate policy and close-readiness relationship. |
| Design gates and policy blockers | Future `design_gate`, `design_policy`, blocker categories, and design-quality policy. |
| Design-policy waiver | Future waiver route for design-policy blockers. |
| Broad design validators and severity-based blocking | Future validator IDs, severity meanings, and blocking policy. |
| Full design-quality policy families | Future design-quality policy families such as `shared_design`, `domain_language`, and `codebase_stewardship`. |
| Future conformance run entrypoint | Future executable conformance runner, suite, and reporting contract. |
| Later schema extensions | Future cross-cutting fields, enum values, and validators. |
| `ValidatorResult` stable IDs and policy families | Future stable validator identity, policy family, severity, and waiver vocabulary. |
| Future fixture families | Future executable fixture families, conformance suites, assertions, and report formats. |
| Advanced validators and interface checks | Future advanced validators, design-policy validators, language checks, and interface checks. |

## Candidate details

The promotion rule below applies to every candidate in this section.

<a id="manual-qa-workflow-and-qa-gate"></a>
### Manual QA workflow and `qa_gate`

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active QA workflow, gate, waiver, or close-readiness requirements.

<a id="manual-qa-waiver-qa-waiver"></a>
### Manual QA waiver `qa_waiver`

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create an active waiver route or substitute for user-owned judgment.

<a id="verification-gate-verification-gate"></a>
### Verification gate `verification_gate`

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active verification gate fields or close-readiness requirements.

<a id="design-gates-and-policy-blockers"></a>
### Design gates and policy blockers

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active design gate fields, blocker categories, or close-readiness requirements.

<a id="design-policy-waiver"></a>
### Design-policy waiver

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create an active design-policy waiver route.

<a id="broad-design-validators-and-severity-based-blocking"></a>
### Broad design validators and severity-based blocking

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active validator IDs, severity meanings, or blocking policy.

<a id="full-design-quality-policy-families"></a>
### Full design-quality policy families

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not activate `shared_design`, `domain_language`, `vertical_slice`, `feedback_loop`, `tdd_trace`, `deep_module_interface`, `codebase_stewardship`, `manual_qa`, `two_stage_review_display`, detached-verification policy, or steward policy families.

<a id="future-conformance-run-entrypoint"></a>
### Future conformance run entrypoint

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create an active conformance runner, executable fixture suite, or reporting contract.

<a id="later-schema-extensions"></a>
### Later schema extensions

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active fields, enum values, or validators.

<a id="validatorresult-stable-ids-and-policy-families"></a>
### `ValidatorResult` stable IDs and policy families

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active validator IDs, policy families, severity meanings, or waiver behavior.

<a id="future-fixture-families"></a>
### Future fixture families

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create executable fixtures, conformance suites, assertions, or report formats.

<a id="advanced-validators-and-interface-checks"></a>
### Advanced validators and interface checks

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active advanced validators, design-policy validators, language checks, or interface checks.

## Promotion rule

Promotion is not a local edit to this file. A candidate becomes active only when current active scope and the relevant current owner document, or a new owner document created during promotion, are updated in the same documentation-only batch.

Promotion-time owner update means that the owner work happens at promotion time. It does not create active requirements before promotion, and the candidate entry is not itself an active owner document.

## Active-scope non-effect

This document has no effect on the current MVP. Mentioning a candidate here does not activate a gate, blocker, validator, waiver, fixture suite, conformance runner, report, severity model, or close-readiness requirement.

## Related owners

- [Later Candidate Index](index.md)
- [Active MVP Scope](../reference/active-mvp-scope.md)
- [Reference Index](../reference/README.md)
- [Glossary](../reference/glossary.md)
- [Core Model](../reference/core-model.md)
- [Design Quality](../reference/design-quality.md)
- [Conformance](../reference/conformance.md)
- [API State Schemas](../reference/api/schema-state.md)
- [API Value Sets](../reference/api/schema-value-sets.md)
