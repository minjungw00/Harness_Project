# Later: Connectors and surfaces

<a id="what-this-document-owns"></a>
## Owns / Does not own

This document owns inactive later candidates about future IDE, CLI, chat, MCP, hosted, remote, dashboard, and connector-facing surfaces. It keeps connector and surface candidates grouped so [Later Candidate Index](index.md) can remain a router and short summary.

Every candidate here is future-facing. The candidate details are documentation source material only and do not create current surface support, connector authority, hosted runtime behavior, or UI requirements.

<a id="what-this-document-does-not-own"></a>
This document does not define current MVP API methods, security guarantees, artifact body policies, validator catalogs, conformance fixtures, hosted services, remote runtime behavior, or implementation readiness.

It also does not make a `surface_id`, connector name, dashboard, hosted workflow, or read-only resource into authority. If one of these candidates is promoted, update the relevant current owner, or create a new owner document during promotion.

## Category boundary

This category is for candidates whose main question is "where and how might a user or agent interact with Harness later?" It includes local operator commands, `doctor` surfaces, read-only resources, dashboard and hosted surfaces, broader connector ecosystems, and cross-surface presentation or verification surfaces.

It does not own runtime security claims, artifact capture storage, policy catalogs, or team lifecycle. If a future surface depends on those areas, this document records only the surface-facing candidate before promotion.

## Candidate summary

| Candidate | Summary |
|---|---|
| Future local operator command family | Future local command surfaces such as `harness doctor`, `harness export`, and `harness conformance run`. |
| Operator readiness and `doctor` surfaces | Future local readiness and diagnostic surfaces. |
| Projection refresh and freshness diagnostics | Future refresh and freshness visibility for projection material. |
| Later read-only resources | Future read-only resources such as `policy`, `evidence-manifest`, `surface`, `report`, `bundle`, `journey`, and `design`. |
| Dashboard and hosted workflows | Future dashboard, hosted workflow, visualization, card, and artifact dashboard surfaces. |
| Cross-surface verification | Future verification visibility across IDE, CLI, chat, MCP, or hosted surfaces. |
| Broader connectors and hosted runtime | Future connector marketplace, hosted UI, hosted runtime, and remote runtime candidates. |
| Connector conformance ecosystem | Future connector-facing compatibility claims, marketplace signals, and report surfaces. |

## Candidate details

<a id="future-local-operator-command-family"></a>
### Future local operator command family

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not activate `harness connect`, `harness serve mcp`, `harness doctor`, `harness projection refresh`, `harness reconcile`, `harness recover`, `harness export`, `harness artifacts check`, or `harness conformance run`.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="operator-readiness-and-doctor-surfaces"></a>
### Operator readiness and `doctor` surfaces

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active readiness checks or `doctor` diagnostics.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="projection-refresh-and-freshness-diagnostics"></a>
### Projection refresh and freshness diagnostics

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active projection refresh commands, freshness diagnostics, or state-changing projection behavior.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="later-read-only-resources"></a>
### Later read-only resources

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not activate read-only resources such as `policy`, `evidence-manifest`, `surface`, `report`, `bundle`, `journey`, or `design`.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="dashboard-and-hosted-workflows"></a>
### Dashboard and hosted workflows

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active dashboard, hosted workflow, artifact dashboard, card, or visualization requirements.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="cross-surface-verification"></a>
### Cross-surface verification

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active cross-surface verification authority.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="broader-connectors-and-hosted-runtime"></a>
### Broader connectors and hosted runtime

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active connector marketplace, hosted UI, hosted runtime, or remote runtime requirements.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

<a id="connector-conformance-ecosystem"></a>
### Connector conformance ecosystem

- Status: Later candidate; currently inactive.
- Current MVP non-effect: Not part of the current MVP. Does not create active connector assertions, suite formats, reports, or marketplace claims.
- Promotion requirement: Update the relevant current owner, or create a new owner document during promotion. This candidate entry is not itself an active owner document. Promotion-time owner updates do not create active requirements before promotion.

## Promotion rule

Promotion is not a local edit to this file.

Update the relevant current owner, or create a new owner document during promotion.

This candidate entry is not itself an active owner document.

Promotion-time owner updates do not create active requirements before promotion.

A candidate becomes active only when current active scope and the relevant current owner document, or the new owner document created during promotion, are updated in the same documentation-only batch.

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
