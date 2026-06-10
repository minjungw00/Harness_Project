# Later: Security and Assurance

## What this document owns

This document owns inactive later candidates about stronger security posture, assurance claims, capability labels, observation, blocking, and verification confidence. It keeps those candidates grouped so [Later Candidate Index](index.md) can remain a router and short summary.

Every candidate here is future-facing. The candidate details are documentation source material only and do not activate runtime behavior.

## What this document does not own

This document does not define current MVP security guarantees, active access classes, active API methods, storage effects, UI behavior, connector behavior, executable conformance, or implementation readiness.

It also does not decide that a stronger guarantee is possible. If any preventive, detective, isolation, redaction, observation, or blocking claim is promoted, update the relevant current owner, or create a new owner document during promotion.

## Category boundary

This category is for candidates whose main question is "what assurance can Harness honestly claim?" It includes preventive-control candidates, isolation labels, capability-profile hardening, command/network/secret observation, pre-tool blocking, and stronger verification-confidence claims.

It does not own native artifact capture as a storage mechanism, connector surface design, team workflow, or validator catalog detail unless the candidate is specifically about an assurance claim. Cross-cutting candidates may also appear in another category later, but this document owns only the security-and-assurance framing before promotion.

## Candidate summary

| Candidate | Summary |
|---|---|
| Assurance hardening | Stronger evidence, verification, and close-readiness assurance claims beyond the current MVP. |
| Operations hardening | Future operator diagnostics and stronger security posture for local operation. |
| Stronger local capability profiles | Future profile labels for observation, capture, isolation, or blocking capabilities. |
| Command, network, and secret-access observation | Future observation of selected command, network, or secret-access intent. |
| Command, network, and secret pre-tool blocking | Future preventive blocking claims before tool execution. |
| Capability-gated `prepare_write` and `record_run` observation | Future command, network, or secret-access observation around write preparation and run recording. |
| Capability-profile support fields | Future support fields for observation, capture, pre-tool blocking, and isolation capabilities. |
| Capability-gated authorization observation fields | Future fields such as `intended_commands`, `intended_network`, `network_write`, and `secret_access`. |
| Later close and assurance fields | Future close, gate, verification, QA, design, and assurance fields. |
| Native hooks and advanced sidecar watcher | Future native hook or sidecar watcher claims for broader tool visibility. |

## Candidate details

<a id="assurance-hardening"></a>
### Assurance hardening

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active gates, validators, or close-readiness requirements.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="operations-hardening"></a>
### Operations hardening

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active operator commands, diagnostics, or security guarantees.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="stronger-local-capability-profiles"></a>
### Stronger local capability profiles

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active preventive, isolated, observation, capture, pre-tool blocking, or isolation guarantees.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="command-network-and-secret-access-observation"></a>
### Command, network, and secret-access observation

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active command observation, network observation, or secret-access observation authority.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="command-network-and-secret-pre-tool-blocking"></a>
### Command, network, and secret pre-tool blocking

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active preventive blocking, isolation, or arbitrary-tool-control guarantees.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="capability-gated-prepare-write-and-record-run-observation"></a>
### Capability-gated `prepare_write` and `record_run` observation

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active command, network, or secret-access observation for `prepare_write` or `record_run`.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="capability-profile-support-fields"></a>
### Capability-profile support fields

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not activate `command_observation_supported`, `network_observation_supported`, `secret_access_observation_supported`, `artifact_capture_supported`, `pre_tool_blocking_supported`, or `isolation_supported`.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="capability-gated-authorization-observation-fields"></a>
### Capability-gated authorization observation fields

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not activate `intended_commands`, `intended_network`, `intended_secret_scope`, `network_write`, `external_service_write`, or `secret_access`.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="later-close-and-assurance-fields"></a>
### Later close and assurance fields

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not activate `verifying`, `qa`, `completed_verified`, `detached_verified`, `design_gate`, `verification_gate`, `qa_gate`, Manual QA gate fields, design-policy blockers, or assurance blockers.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="native-hooks-and-advanced-sidecar-watcher"></a>
### Native hooks and advanced sidecar watcher

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active preventive guard expansion, native hook, sidecar watcher, or arbitrary-tool-control guarantees.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

## Promotion rule

Promotion is not a local edit to this file.

Update the relevant current owner, or create a new owner document during promotion.

This candidate entry is not itself an active owner document.

Promotion-time owner updates do not create active requirements before promotion.

A candidate becomes active only when current active scope and the relevant current owner document, or the new owner document created during promotion, are updated in the same documentation-only batch.

## Active-scope non-effect

This document has no effect on the current MVP. Mentioning a candidate here does not activate a guarantee, profile, field, method, access class, validator, fixture, UI, or runtime control.

## Related owners

- [Later Candidate Index](index.md)
- [Active MVP Scope](../reference/active-mvp-scope.md)
- [Reference Index](../reference/README.md)
- [Security](../reference/security.md)
- [Agent Integration](../reference/agent-integration.md)
- [Runtime Boundaries](../reference/runtime-boundaries.md)
- [API Value Sets](../reference/api/schema-value-sets.md)
