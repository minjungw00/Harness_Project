# Later: Workflow and Collaboration

## What this document owns

This document owns inactive later candidates about richer task shaping, next-action flow, verification and evaluation workflow, user-judgment branches, risk review, release handoff, reconciliation, team orchestration, and collaboration lifecycle. It keeps workflow and collaboration candidates grouped so [Later Candidate Index](index.md) can remain a router and short summary.

Every candidate here is future-facing. The candidate details are documentation source material only and do not create current workflow, API, storage, UI, or collaboration requirements.

## What this document does not own

This document does not define current MVP API behavior, task lifecycle, close-readiness behavior, acceptance behavior, residual-risk acceptance, team permissions, release automation, storage effects, security guarantees, conformance behavior, or implementation readiness.

It also does not make any later judgment branch a substitute for user-owned judgment. Sensitive-action approval, final acceptance, residual-risk acceptance, waiver, reconciliation, and verification-risk acceptance remain distinct unless a promotion batch explicitly changes the relevant current owners.

## Category boundary

This category is for candidates whose main question is "how might Harness coordinate people, agents, judgments, and lifecycle steps later?" It includes shaping records, next actions, richer verification and evaluation flows, `Decision Packet` presentation, risk review, reconcile, release handoff, team workflows, and derived workflow context.

It does not own connector mechanics, artifact body storage, security guarantees, or validator catalogs. If a future workflow depends on those areas, this document records only the workflow-and-collaboration framing before promotion.

## Candidate summary

| Candidate | Summary |
|---|---|
| Discovery brief, question queue, and assumption register | Future shaping records before or during task work. |
| `harness.next` and later next-action values | Future next-action orchestration and action suggestions. |
| Verification, Eval, and Manual QA workflow APIs | Future `harness.launch_verify`, `harness.record_eval`, `harness.record_manual_qa`, and related flow branches. |
| Full `Decision Packet` and `presentation=full` | Future full-format decision presentation. |
| Rich risk review and residual-risk lifecycle | Future richer risk review material, residual-risk lifecycle, and expiry behavior. |
| Later user-judgment branches | Future `qa_waiver`, `verification_risk_acceptance`, waiver, reconcile, residual-risk, and richer acceptance branches. |
| Recovery and reconcile | Future workflow for recovery, reconcile, and state-repair coordination. |
| Release handoff and deployment automation | Future handoff, release, deployment, canary, rollback, merge, and monitoring workflow. |
| Team workflows and orchestration | Future team permissions, shared capability sets, orchestration, and parallel-lane behavior. |
| Context index and derived metrics | Future context and derived metrics that support workflow review without becoming authority by themselves. |

## Candidate details

### Discovery brief, question queue, and assumption register

Summary: Holds future shaping records that capture open questions, assumptions, and task context.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### `harness.next` and later next-action values

Summary: Tracks future next-action orchestration, including later values such as `launch_verify`, `record_eval`, `record_manual_qa`, and `reconcile`.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Verification, Eval, and Manual QA workflow APIs

Summary: Holds future workflow branches and API candidates such as `harness.launch_verify`, `harness.record_eval`, `harness.record_manual_qa`, later `harness.record_run` branches, and detached verification flow.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Full `Decision Packet` and `presentation=full`

Summary: Tracks future full-format decision presentation and richer judgment context.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Rich risk review and residual-risk lifecycle

Summary: Holds future risk review records, residual-risk lifecycle, and expiry behavior.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Later user-judgment branches

Summary: Tracks future `qa_waiver`, `verification_risk_acceptance`, waiver, reconcile, residual-risk, and richer acceptance branches.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Recovery and reconcile

Summary: Holds future workflow for recovery, reconcile, and state-repair coordination.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Release handoff and deployment automation

Summary: Tracks future handoff, release, deployment, canary, rollback, merge, and production-monitoring workflow.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Team workflows and orchestration

Summary: Holds future multi-user permissions, shared capability sets, orchestration, and parallel-lane workflow.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

### Context index and derived metrics

Summary: Tracks future context indexing and derived metrics that support workflow review without becoming authority or close effect by themselves.

Status: Later candidate.

Not active: Not part of the current MVP and does not create active requirements.

Promotion requires: current active scope update, relevant current owner update or new owner creation during promotion, conformance/check updates if the behavior becomes normative.

Current owner impact: none before promotion.

## Promotion rule

Promotion is not a local edit to this file. A candidate becomes active only when the current active scope and the relevant current owner documents are updated in the same documentation-only batch.

If no current owner exists for the promoted behavior, the promotion batch must create or designate that owner before defining active API, storage, security, UI, or conformance requirements.

## Active-scope non-effect

This document has no effect on the current MVP. Mentioning a candidate here does not activate a workflow branch, API method, state transition, judgment route, permission model, release automation, team behavior, UI, storage effect, or close-readiness requirement.

## Related owners

- [Later Candidate Index](index.md)
- [Active MVP Scope](../reference/active-mvp-scope.md)
- [Reference Index](../reference/README.md)
- [Core Model](../reference/core-model.md)
- [MVP API](../reference/api/mvp-api.md)
- [API Judgment Schemas](../reference/api/schema-judgment.md)
- [Projection Authority Reference](../reference/projection-and-templates.md)
- [Template Bodies](../reference/template-bodies.md)
