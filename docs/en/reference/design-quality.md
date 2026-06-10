# Design quality

## 1. Owns / Does not own

This Reference page owns the active current MVP design-quality routing boundary as a judgment-routing and evidence/scope reference: how design-quality observations identify product decisions, technical decisions, scope decisions, evidence gaps, residual-risk visibility issues, or close blockers that are already owned by active Core/API categories.

It does not define an independent active gate, active design-quality `CloseReadinessBlocker.category`, active validator family, design-policy waiver route, severity-based blocking policy, evidence record, QA record, acceptance record, residual-risk record, or close authority.

It owns:

- the active design-quality role as judgment-routing and evidence/scope reference
- how design-quality observations route to `judgment_kind=product_decision`, `judgment_kind=technical_decision`, and `judgment_kind=scope_decision`
- how design-quality observations point to existing active blocker categories such as `scope`, `user_judgment`, `evidence`, `artifact_availability`, `residual_risk_visibility`, or `surface_capability`
- the active design-quality severity boundary: severity-like wording is advisory triage unless an active owner path separately requires action
- the boundary between design-quality observations, active `ValidatorResult.validator_id` values, and later design-policy catalogs

It does not own:

- Core lifecycle, gates, blockers, `prepare_write`, `close_task`, Write Authorization, final acceptance, residual-risk acceptance, or non-substitution rules; see [Core Model Reference](core-model.md)
- MCP request/response schemas, `ValidatorResult`, `UserJudgment`, `AcceptedRiskInput`, public errors, or active/later schema values; see [MVP API](api/mvp-api.md), [API Schema Core](api/schema-core.md), [API Judgment Schemas](api/schema-judgment.md), and [API Errors](api/errors.md)
- SQLite DDL and persisted tables; see [Storage Records](storage-records.md)
- validator-run storage effects; see [Storage Effects](storage-effects.md)
- artifact storage; see [Artifact Storage](storage-artifacts.md)
- projection authority; see [Projection Authority Reference](projection-and-templates.md)
- template bodies, status cards, or rendered reports; see [Template Bodies](template-bodies.md)
- broad design-policy validators, design-policy waiver, severity-based active blocking policy, steward policies, full review procedure, operations/reporting candidates, or future conformance catalogs

Use these owner links when a design-quality finding crosses another contract:

| Question | Owner |
|---|---|
| Core non-substitution, close readiness, waiver, accepted-risk, and residual-risk meaning | [Core Model Reference](core-model.md) |
| `UserJudgment`, `RecordUserJudgmentPayload`, `SensitiveActionScope`, and `AcceptedRiskInput` shapes | [API Judgment Schemas](api/schema-judgment.md) |
| Active method behavior for requesting or recording judgment, reporting status, and closing a Task | [MVP API](api/mvp-api.md) |
| Method-to-storage effects for active API method branches | [Storage Effects](storage-effects.md) |
| Deferred design gates, policy blockers, broad validators, waiver candidates, and policy catalogs | [Later Candidate Index](../later/index.md) |

Documentation in this repository remains planning source material. It does not mean a Harness Server, runtime state, generated evidence, QA record, Acceptance record, residual-risk record, or close record exists here today.

## 2. Active current MVP design-quality role

Active current MVP design quality is a narrow judgment-routing and evidence/scope reference layer. It makes a quality concern legible, then sends the concern to an existing active owner path.

A design-quality finding can do only these things in the active MVP:

| What the finding can do | Active route or result | Close-readiness boundary |
|---|---|---|
| Identify a product behavior, UX, wording, release promise, or user-value choice that needs the user. | Route as `judgment_kind=product_decision`. | Blocks close only when the active close path already requires `CloseReadinessBlocker.category=user_judgment`. |
| Identify an architecture, dependency, migration, public-interface, compatibility, security/privacy, or material technical direction choice that needs the user. | Route as `judgment_kind=technical_decision`. | Blocks close only when the active close path already requires `CloseReadinessBlocker.category=user_judgment`. |
| Identify scope expansion, non-goal removal, Change Unit boundary, or Autonomy Boundary change. | Route as `judgment_kind=scope_decision` or `CloseReadinessBlocker.category=scope`, depending on the owner path. | Blocks close only through the active scope or judgment owner path. |
| Point out missing support for a close-relevant claim. | Request evidence or use `CloseReadinessBlocker.category=evidence` or `CloseReadinessBlocker.category=artifact_availability` through the Core evidence owner path. | Required evidence can block close only through the Core evidence owner path. |
| Make a known limitation, unchecked condition, or trade-off visible. | Use residual-risk visibility, and `CloseReadinessBlocker.category=residual_risk_acceptance` only when the active close path requires acceptance. | Accepted risk records judgment about a visible risk; it does not prove success or erase the risk. |
| Report that the connected surface cannot honestly support the claimed operation or guarantee. | Use `CloseReadinessBlocker.category=surface_capability`, `CAPABILITY_INSUFFICIENT`, or a lower guarantee display through the capability owner path. | The design-quality label does not strengthen the guarantee. |
| Describe relative urgency or attention for the concern. | Advisory triage only, unless an active owner path separately requires action. | Severity alone never creates a blocker, validator mapping, waiver, evidence expectation, or close result. |
| Choose one focused next action. | Ask one focused user judgment, request evidence, mark residual risk visible, show an advisory next action, or no action. | The action must stay narrow enough to unblock or clarify the named owner path. |
| Stay advisory when no active owner path applies. | Advisory text or no action. | No new gate, blocker, validator mapping, waiver route, evidence rule, or close authority is created. |

Active design quality does not create new Core state, `StateSummary.gates.design_gate`, `CloseReadinessBlocker.category=design_policy`, new schemas, new validator result fields, active design-policy validators, design-policy waiver, or a separate design-review authority.

Design quality must not turn ordinary work into an open-ended planning loop. Full domain-language audits, full module/interface review, full TDD trace, full feedback-loop audit, full codebase-stewardship review, detailed Manual QA policy, detached verification, two-stage review displays, and steward policies are not active current MVP blockers unless another active owner path explicitly requires a narrow piece of that work.

## 3. Routing rules

A design-quality observation affects current MVP state only through an active owner path. The observation must name the active route it depends on:

| Concern | Active current MVP route |
|---|---|
| Product behavior, UX, wording, release promise, or user value is undecided. | `judgment_kind=product_decision`; use `CloseReadinessBlocker.category=user_judgment` only when the active close path requires that judgment. |
| Architecture, dependency, migration, public interface, compatibility, security/privacy, or material technical direction is undecided. | `judgment_kind=technical_decision`; use `CloseReadinessBlocker.category=user_judgment` only when the active close path requires that judgment. |
| Scope expansion, non-goal removal, Change Unit boundary, or Autonomy Boundary change is needed. | `judgment_kind=scope_decision` or `CloseReadinessBlocker.category=scope`, depending on the owner path. |
| A close-relevant claim lacks support. | `CloseReadinessBlocker.category=evidence`, `CloseReadinessBlocker.category=artifact_availability`, or an evidence request through the Core evidence owner path. |
| A known limitation or unchecked condition matters to close. | Residual-risk visibility through `CloseReadinessBlocker.category=residual_risk_visibility`, and `CloseReadinessBlocker.category=residual_risk_acceptance` only when the active close path requires acceptance. |
| The connected surface cannot honestly support the claimed operation or guarantee. | `CloseReadinessBlocker.category=surface_capability`, `CAPABILITY_INSUFFICIENT`, or a lower guarantee display through the capability owner path. |

A design-quality label, policy name, severity value, validator ID, or review phrase does not create the route. If no active owner path applies, the current MVP result is advisory text or no action.

<a id="when-a-finding-blocks-close"></a>
## 4. When a finding blocks close

A design-quality observation blocks close only when all of these are true:

- it is tied to the active Task or Change Unit and the attempted close
- it names an existing active `CloseReadinessBlocker.category`, `judgment_kind`, API error, or owner path from the active close-blocking set
- the named owner path would block close even if no design-quality label existed
- it gives exactly one next action that can unblock, defer through the owning path, request the required evidence, or mark residual risk visible
- it does not rely on `design_gate`, `CloseReadinessBlocker.category=design_policy`, a design-policy waiver, a broad policy catalog, or severity alone

A finding does not block close merely because it mentions domain language, vertical slice shape, TDD, module/interface review, stewardship, Manual QA, detached verification, review stages, or a future policy family. Those may produce an advisory next action, an evidence request, a focused user judgment, or a residual-risk marker only when an active owner path needs that narrow action.

When a design-quality observation affects close, the close-readiness finding must use an active `CloseReadinessBlocker.category` value owned by [API Value Sets](api/schema-value-sets.md).

## 5. No current design-policy waiver

The current MVP has no active design-quality waiver or design-policy waiver route. If an owner path allows a requirement to be deferred, accepted as risk, or resolved by user judgment, use that active owner path and its exact `judgment_kind`, blocker category, or evidence behavior.

A waiver-like decision or accepted-risk answer records the responsible user judgment about a named requirement or a named visible risk. It does not erase the facts, remove the underlying limitation from the close basis, create evidence, prove verification, pass QA, replace final acceptance, or automatically make close successful.

Keep the judgment routes separate:

| Route | What it records | What it must not be treated as |
|---|---|---|
| `final_acceptance` | The user's result judgment after the close basis is visible. | Evidence creation, residual-risk acceptance, QA, verification, or blocker override. |
| `residual_risk_acceptance` | The user's acceptance of a named visible residual risk for the requested close. | Correctness proof, evidence sufficiency, final acceptance, no-risk result, or automatic success. |
| Active current MVP `UserJudgment.judgment_kind` values | Focused user-owned decisions whose values are owned by [API Value Sets](api/schema-value-sets.md). | Design-policy waiver, broad approval, later QA waiver, later verification-risk acceptance, or any future candidate that has not been promoted. |
| Future design-quality waiver candidates | Later-only material in [Later](../later/index.md). | Active requirements, close blockers, validator behavior, or evidence rules. |

Broad approval, a friendly "looks good", or a general go-ahead must not be treated as any of these judgments unless the active owner path asked for that specific judgment.

## 6. Evidence expectation

Design-quality observations may identify evidence gaps, but required evidence belongs to the Core evidence owner path. A finding should ask for evidence only when that active owner path needs support for a claim that affects write safety, close readiness, user judgment, residual risk, or guarantee honesty.

Useful evidence references can include:

- persisted `ArtifactRef` values, Run refs, command/check summaries, or source refs
- current state/version/freshness refs when stale context affects the close basis
- user-judgment refs for product, technical, scope, final-acceptance, or residual-risk decisions
- residual-risk refs when a known limitation remains visible at close
- future Manual QA or verification refs only after those later owner paths are promoted

Chat assertions, generic summaries, rendered projection prose, unregistered files, screenshots without an owner path, passing tests alone, future waiver candidates, final acceptance, or residual-risk acceptance do not automatically satisfy required evidence. Required evidence can block close only through the Core evidence owner path. Non-required evidence gaps should be routed as `request evidence`, `show advisory next action`, or residual-risk visibility as appropriate.

## 7. Validator ID boundary

Validator IDs are reporting labels. They do not create Core invariants, gates, close blockers, waivers, evidence records, or user judgments.

`ValidatorResult` shape is owned by [API State Schemas](api/schema-state.md). Severity-like values and the active stable `ValidatorResult.validator_id` set are owned by [API Value Sets](api/schema-value-sets.md).

This document does not publish active design-policy validator IDs or a policy-to-validator mapping. Later stable validator ID sets remain candidates in [Later policy and conformance: `ValidatorResult` stable IDs and policy families](../later/policy-and-conformance.md#validatorresult-stable-ids-and-policy-families) unless an owner promotes a narrow active contract.

## 8. Later policy catalog boundary

The full design-quality policy catalog is not active current MVP scope. These ideas are later-only until a named owner promotes a narrow behavior with scope, fallback behavior, exact contracts, and proof expectations.

| Later-only idea | What it does not do in the active MVP | What promotion would need |
|---|---|---|
| `design_gate` and `CloseReadinessBlocker.category=design_policy` | No active gate, active close blocker, or close-readiness category. | Core/API owner changes plus value-set, schema, close-readiness, and storage-effect ownership. |
| Design-policy waiver | No active waiver route and no automatic success path. | A named owner path, non-substitution rules, judgment behavior, and close-readiness effects. |
| Broad design validators and severity-based blocking | No active validator IDs, severity meanings, policy-to-validator mapping, or severity-only blocker. | Stable validator set ownership, severity semantics, API/schema boundaries, and fallback behavior. |
| Full design-quality policy families and steward policies | No active policy catalog, stewardship gate, or full review procedure. | A scoped policy owner, reader-facing behavior, proof expectations, and active/later migration path. |
| Detailed review displays, operations/reporting candidates, full validator mappings, and future conformance fixtures | No active operations report, fixture requirement, implementation task, or conformance obligation. | Promotion through [Later Candidate Index](../later/index.md), active owner updates, and documentation-only acceptance before implementation work starts. |

Later candidates may keep names only. They must not be presented as active current MVP requirements, blockers, waiver rules, evidence expectations, validator mappings, fixture requirements, operations reports, or implementation tasks.
