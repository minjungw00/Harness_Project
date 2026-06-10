# Later: Connectors and Surfaces

## What this document owns

This document owns inactive later candidates about future IDE, CLI, chat, MCP, hosted, remote, dashboard, and connector-facing surfaces. It keeps connector and surface candidates grouped so [Later Candidate Index](index.md) can remain a router and short summary.

Every candidate here is future-facing. The candidate details are documentation source material only and do not create current surface support, connector authority, hosted runtime behavior, or UI requirements.

## What this document does not own

This document does not define current MVP API methods, security guarantees, artifact body policies, validator catalogs, conformance fixtures, hosted services, remote runtime behavior, or implementation readiness.

It also does not make a `surface_id`, connector name, dashboard, hosted workflow, or read-only resource into authority. Any promoted connector or surface must be re-owned by active scope and the relevant current owner documents.

## Category boundary

This category is for candidates whose main question is "where and how might a user or agent interact with Harness later?" It includes local operator commands, `doctor` surfaces, read-only resources, dashboard and hosted surfaces, broader connector ecosystems, and cross-surface presentation or verification surfaces.

It does not own runtime security claims, artifact capture storage, policy catalogs, or team lifecycle. If a future surface depends on those areas, this document records only the surface-facing candidate before promotion.

## Candidate summary

| Candidate | Summary |
|---|---|
| Future local operator command family | Future CLI commands such as `harness connect`, `harness serve mcp`, `harness doctor`, `harness projection refresh`, `harness reconcile`, `harness recover`, `harness export`, `harness artifacts check`, and `harness conformance run`. |
| Operator readiness and `doctor` surfaces | Future readiness and diagnostic surfaces for local operation. |
| Projection refresh and freshness diagnostics | Future surface for refresh and freshness visibility, without activating projection state changes. |
| Later read-only resources | Future read-only resources such as `policy`, `evidence-manifest`, `surface`, `report`, `bundle`, `journey`, and `design`. |
| Dashboard and hosted workflows | Future dashboard, hosted workflow, visualization, and card surfaces. |
| Broader connectors and hosted runtime | Future connector marketplace, hosted UI, hosted runtime, and remote runtime candidates. |
| Cross-surface verification | Future verification visibility across IDE, CLI, chat, MCP, or hosted surfaces. |
| Connector conformance ecosystem | Future connector-facing conformance claims and marketplace signals, without defining the conformance suite itself. |

## Candidate details

### Future local operator command family

Summary: Holds future local command surfaces and operator entrypoints.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Operator readiness and `doctor` surfaces

Summary: Tracks future diagnostic surfaces that could explain local readiness or configuration health.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Projection refresh and freshness diagnostics

Summary: Holds future user-visible refresh and freshness surfaces for read-only projection material.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Later read-only resources

Summary: Tracks future resource surfaces for policy, evidence manifests, surface metadata, reports, bundles, journeys, or design material.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Dashboard and hosted workflows

Summary: Holds future dashboard, hosted UI, visualization, artifact dashboard, card, and hosted workflow surfaces.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Broader connectors and hosted runtime

Summary: Tracks future connector marketplace, hosted runtime, hosted UI, and remote runtime concepts.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Cross-surface verification

Summary: Holds future verification presentation across local and hosted surfaces without granting verification authority by itself.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Connector conformance ecosystem

Summary: Tracks future connector-facing compatibility claims, marketplace signals, and report surfaces while leaving conformance policy to its owner.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

## Promotion rule

Promotion is not a local edit to this file. A candidate becomes active only when the current active scope and the relevant current owner documents are updated in the same documentation-only batch.

If no current owner exists for the promoted behavior, the promotion batch must create or designate that owner before defining active API, storage, security, UI, or conformance requirements.

## Active-scope non-effect

This document has no effect on the current MVP. Mentioning a candidate here does not activate a connector, hosted runtime, remote service, dashboard, UI, local command, read-only resource, or cross-surface authority.

## Related owners

- [Later Candidate Index](index.md)
- [Active MVP Scope](../reference/active-mvp-scope.md)
- [Reference Index](../reference/README.md)
- [Agent Integration](../reference/agent-integration.md)
- [MVP API](../reference/api/mvp-api.md)
- [Projection Authority Reference](../reference/projection-and-templates.md)
- [Conformance](../reference/conformance.md)
