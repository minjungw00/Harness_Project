# Later: Security and Assurance

## What this document owns

This document owns inactive later candidates about stronger security posture, assurance claims, capability labels, observation, blocking, and verification confidence. It keeps those candidates grouped so [Later Candidate Index](index.md) can remain a router and short summary.

Every candidate here is future-facing. The candidate details are documentation source material only and do not activate runtime behavior.

## What this document does not own

This document does not define current MVP security guarantees, active access classes, active API methods, storage effects, UI behavior, connector behavior, executable conformance, or implementation readiness.

It also does not decide that a stronger guarantee is possible. Any preventive, detective, isolation, redaction, observation, or blocking claim must be re-owned by the current security and active-scope owners during promotion.

## Category boundary

This category is for candidates whose main question is "what assurance can Harness honestly claim?" It includes preventive-control candidates, isolation labels, capability-profile hardening, command/network/secret observation, pre-tool blocking, capture assurance, and stronger verification-confidence claims.

It does not own native artifact capture as a storage mechanism, connector surface design, team workflow, or validator catalog detail unless the candidate is specifically about an assurance claim. Cross-cutting candidates may also appear in another category later, but this document owns only the security-and-assurance framing before promotion.

## Candidate summary

| Candidate | Summary |
|---|---|
| Assurance hardening | Stronger evidence, verification, and close-readiness assurance claims beyond the current MVP. |
| Stronger local capability profiles | Future profile labels for observation, capture, isolation, or blocking capabilities. |
| Command, network, and secret-access observation | Future ability to observe selected tool intentions or access patterns. |
| Command, network, and secret pre-tool blocking | Future preventive control claims before tool execution. |
| Capability-gated authorization observation fields | Future fields such as `intended_commands`, `intended_network`, `intended_secret_scope`, `network_write`, `external_service_write`, and `secret_access`. |
| Native hooks and advanced sidecar watcher | Future native hook or sidecar watcher claims for broader tool visibility. |
| Capture redaction and retention assurance | Future security claims around browser capture, artifact body capture, redaction, and retention. |
| Verification assurance escalation | Future stronger verification confidence claims, including detached or cross-surface verification framing. |

## Candidate details

### Assurance hardening

Summary: Tracks future claims that Harness can make stronger assurance statements than the current MVP permits.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Stronger local capability profiles

Summary: Groups future capability profile labels for observation, capture, pre-tool blocking, or isolation support.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Command, network, and secret-access observation

Summary: Holds future observation candidates for commands, network activity, and secret-access intent.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Command, network, and secret pre-tool blocking

Summary: Holds future preventive blocking candidates for tool actions before execution.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Capability-gated authorization observation fields

Summary: Tracks future schema or API vocabulary for observed command, network, external-service, and secret-access intent.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Native hooks and advanced sidecar watcher

Summary: Holds future sidecar or native hook concepts that could support stronger observation or blocking claims.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Capture redaction and retention assurance

Summary: Tracks security claims around future browser capture, artifact capture, redaction, retention, or access boundaries.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Verification assurance escalation

Summary: Holds future assurance framing for detached verification, cross-surface verification, and stronger verification-confidence displays.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

## Promotion rule

Promotion is not a local edit to this file. A candidate becomes active only when the current active scope and the relevant current owner documents are updated in the same documentation-only batch.

If no current owner exists for the promoted behavior, the promotion batch must create or designate that owner before defining active API, storage, security, UI, or conformance requirements.

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
