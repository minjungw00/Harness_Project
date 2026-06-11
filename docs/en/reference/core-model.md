# Core model reference

This reference defines the future Harness Core authority model as documentation source material only. This repository still has no Harness runtime or server implementation, and the current documentation is not implementation-complete unless the maintainer-owned status says so in [MVP Plan](../build/mvp-plan.md).

Core is the local authority record for task scope, user-owned judgment, evidence, verification expectations, close readiness, and residual risk. It owns the product meaning of those boundaries. Security guarantee wording and non-claims belong to [Security](security.md).

## 1. Owns / Does not own

This document owns:

- Core authority invariants and non-substitution rules.
- Product meaning for Task scope, Change Unit boundaries, user-owned judgment, evidence, close readiness, close honesty, waivers, and residual risk.
- Conceptual lifecycle and gate boundaries.
- The difference between `WriteDecisionReason`, close-readiness blocking reasons, and `CloseReadinessBlocker`.
- Cross-owner routing when Core concepts touch API, Storage, Security, Projection, or Later material.

This document does not own:

- Public API payload schemas, response branch shapes, envelopes, or method result structures. Use the [MVP API router](api/mvp-api.md), method owner documents, [API Schema Core](api/schema-core.md), and the API schema owners.
- Storage DDL, persisted JSON layout, locks, migrations, runtime-home placement, or method-to-storage effects. Use [Storage Records](storage-records.md), [Storage Effects](storage-effects.md), [Artifact Storage](storage-artifacts.md), and [Storage Versioning](storage-versioning.md).
- Exact active enum-like values and wire field lists. Use [API Value Sets](api/schema-value-sets.md) and [API State Schemas](api/schema-state.md).
- Public error code definitions or error precedence. Use [API Errors](api/errors.md).
- Rendered projection bodies, template text, connector recipes, security guarantee vocabulary, or later candidate catalogs.

Exact identifiers may appear here when needed to explain meaning. Their schema shape, value set, storage effect, and public error behavior remain with the linked owner documents.

## 2. Kernel invariants

| Invariant | Consequence |
|---|---|
| Core-owned state is the authority for Harness operations. | Chat, reports, generated Markdown, projections, and template output are displays or context, not authority. |
| Harness governs Harness records and state transitions. | It is not a general security-control surface; see [Security](security.md) for canonical non-claims. |
| Product writes require compatible active scope. | A write path outside the current Task and Change Unit must be reshaped before it can be compatible. |
| User-owned judgment stays user-owned. | Agent inference, broad consent, evidence, projection text, and generated summaries cannot replace a required user judgment. |
| Write Authorization creation is narrow. | See [Write Authorization creation](#core-invariant-write-authorization-creation). |
| Runs record what happened. | `record_run` cannot retroactively authorize work that lacked scope, required judgment, sensitive-action approval, or `Write Authorization`. |
| Evidence records support only the claims they actually record. | Evidence does not become acceptance, QA, verification, residual-risk acceptance, or proof of unrecorded facts. |
| Close must stay honest. | If close-relevant blockers remain, Core reports blockers instead of treating the Task as successfully completed. |
| Current MVP and later candidates stay separate. | Later verification, Manual QA, rich waiver, and assurance material is inactive until an owner promotes it. |

<a id="core-invariant-write-authorization-creation"></a>
### Write Authorization creation

Condition:
- Only a non-dry-run allowed `prepare_write` path creates a consumable `Write Authorization`.

Effect:
- `Write Authorization` is single-use for one compatible product-file attempt.

Not allowed:
- It is not reusable scope and not general permission.

## 3. Core entities

These entities describe authority relationships, not storage tables or API bodies.

| Entity | Core meaning | Boundary |
|---|---|---|
| Task | The user-value unit being shaped, executed, blocked, or closed. | See [Task entity boundary](#core-entity-task-boundary). |
| Change Unit | The active scoped work boundary for write-capable work. | It is not final acceptance, evidence, or permission to widen scope silently. |
| Autonomy Boundary | The agent latitude inside a Change Unit. | It is not scope expansion, sensitive-action approval, or permission to make user-owned judgments. |
| `user_judgment` | The record family for decisions the user owns. | It feeds compatibility but does not by itself mutate active scope, create evidence, authorize writes, accept risk, or close a Task. |
| `Write Authorization` | A durable, single-use Core authorization for one compatible product-file write attempt. | See [`Write Authorization` boundary](#core-entity-write-authorization-boundary). |
| Run | A record of execution or observation. | Read-only and shaping-only Runs do not make later product writes compatible. |
| Evidence summary | The compact Core path for close-relevant support, gaps, refs, and coverage expectations. | Full `Evidence Manifest` behavior is not active unless promoted by an owner. |
| `ArtifactRef` | A durable reference to an evidence-eligible artifact when the artifact owners allow it. | See [`ArtifactRef` boundary](#core-entity-artifactref-boundary). |
| Blocker | A structured reason progress, write, Run recording, or close cannot proceed honestly. | Schema shape and active value sets belong to the API schema/value owners. |
| Residual-risk summary | The compact visibility path for known remaining uncertainty, limits, or trade-offs. | Rich residual-risk records and assurance displays are later candidate material until promoted. |
| Projection output | Derived display from Core state and refs. | Authority and freshness boundaries belong to [Projection Authority Reference](projection-and-templates.md). |
| Template output | Rendered body text for cards, requests, summaries, results, and packets. | See [Template output boundary](#core-entity-template-output-boundary). |

<a id="core-entity-task-boundary"></a>
### Task entity boundary

Owner links:
- Exact lifecycle values and public state fields are owned by [API Value Sets](api/schema-value-sets.md) and [API State Schemas](api/schema-state.md).

<a id="core-entity-write-authorization-boundary"></a>
### `Write Authorization` boundary

Not allowed:
- It is not OS permission, command approval, sensitive-action approval, final acceptance, or reusable scope.

<a id="core-entity-artifactref-boundary"></a>
### `ArtifactRef` boundary

Owner links:
- Artifact shape, staging, promotion, integrity, and body-read rules are owned by [API Artifact Schemas](api/schema-artifacts.md) and [Artifact Storage](storage-artifacts.md).

<a id="core-entity-template-output-boundary"></a>
### Template output boundary

Owner links:
- Body expectations belong to [Template Bodies](template-bodies.md).

Not allowed:
- Readability or manual editing does not turn output into authority.

`ShapingReadiness` is a compact derived view over Task, Change Unit, pending judgments, evidence summary, blockers, and next-action state. Core owns the readiness meaning: whether the current owner state is concrete enough for the next safe action. The wire fields are owned by [API State Schemas](api/schema-state.md).

## 4. User-owned judgment

Concept:
- User-owned judgment is the boundary where Harness must ask the user or preserve the user's recorded choice instead of inferring it.
- This page owns the product meaning. Exact schema fields and input shapes belong to the judgment schema owner.

Inputs:
- A product, technical, scope, sensitive-action, final-acceptance, residual-risk, or cancellation question that belongs to the user.
- The affected object, scope, consequence, and close or write impact when one user reply is meant to satisfy more than one judgment kind.

Not the same as:
- Agent inference, broad consent, evidence, projection text, or generated summaries.
- Active scope mutation, `Write Authorization`, sensitive-action approval, final acceptance, or residual-risk acceptance unless that exact judgment kind was asked and recorded.

Owner links:
- [API Judgment Schemas](api/schema-judgment.md)

Judgment kinds:

| Judgment kind | User owns the decision when the question concerns |
|---|---|
| `product_decision` | User-visible behavior, user flow, copy, UX, accessibility, release promise, product trade-off, or user value. |
| `technical_decision` | See [`technical_decision`](#core-judgment-technical-decision). |
| `scope_decision` | Scope expansion, non-goal removal, Change Unit boundary changes, or Autonomy Boundary changes. |
| `sensitive_approval` | Permission for a named sensitive step inside a bounded `SensitiveActionScope`. |
| `final_acceptance` | The user's result judgment when the close path requires acceptance. |
| `residual_risk_acceptance` | The user's acceptance of a named visible residual risk for the requested close. |
| `cancellation` | Stopping the Task without a successful completed result. |

<a id="core-judgment-technical-decision"></a>
### `technical_decision`

Condition:
- The question concerns architecture, dependency or external service introduction, authentication direction, or migration.
- The question concerns public interface, compatibility break, data retention, privacy, or security.
- The question concerns another material and costly-to-reverse technical direction.

Agent latitude:
- Inside accepted scope and acceptance criteria, the agent may choose ordinary implementation details that do not change product behavior, technical direction, scope, security/privacy posture, compatibility, or costly-to-reverse architecture.

Not the same as:
- A new permission system.
- Broad consent that silently satisfies another judgment kind.

Multiple judgments:
- "Go ahead", "looks good", or similar wording cannot silently satisfy another judgment kind.
- A single reply may satisfy multiple judgments only when the prompt asked those distinct questions and Core records each compatible judgment with its affected object, scope, consequence, and close or write impact.

## 5. Non-substitution rules

| One thing | Does not substitute for |
|---|---|
| Chat, reports, generated Markdown, projection prose, or status cards | Core-owned state. |
| Evidence, logs, screenshots, artifacts, test output, or Run records | Final acceptance, future Manual QA, future verification, or residual-risk acceptance. |
| `final_acceptance` | Evidence, QA, verification, sensitive-action approval, scope change, residual-risk acceptance, or blocker override. |
| `residual_risk_acceptance` | Verification, evidence sufficiency, QA, final acceptance, or a no-risk result. |
| `sensitive_approval` | Product direction, technical direction, scope, correctness, evidence, QA, final acceptance, residual-risk acceptance, or `Write Authorization`. |
| `Write Authorization` and `AuthorizedAttemptScope` | Command approval, dependency approval, host/network/secret access, deployment approval, destructive-action approval, system access, or final acceptance. |
| `WriteDecisionReason` | A close-readiness blocker or `CloseReadinessBlocker`. |
| `CloseReadinessBlocker` | A prepare-write decision reason, the entire close-readiness concept, evidence, acceptance, or storage effect by itself. |
| Waiver or accepted risk | Automatic success, verification, evidence, final acceptance, or close without the remaining required owner paths. |

Compact user-facing displays may summarize these boundaries, but they must not collapse them.

## 6. Task lifecycle

| Lifecycle area | Core meaning | Required honesty |
|---|---|---|
| Intake and shaping | See [Intake and shaping](#core-lifecycle-intake-and-shaping). | See [Intake and shaping](#core-lifecycle-intake-and-shaping). |
| Scope update | Move accepted scope or Change Unit changes through `harness.update_scope`. | `scope_decision` records may support the change, but they do not mutate active scope by themselves. |
| Execution and observation | Run records describe actions or observations. | Product-file writes must be compatible with active scope and `Write Authorization`; read-only work does not authorize later writes. |
| Waiting or blocked | Progress pauses because an owner path is missing, stale, incompatible, or unsafe to bypass. | The blocker should point to the next safe owner path rather than hide the gap. |
| Close attempt | Core evaluates whether the Task can close honestly. | Close readiness is evaluated from current Core state, not from a final chat summary alone. |
| Terminal outcome | See [Terminal outcome](#core-lifecycle-terminal-outcome). | See [Terminal outcome](#core-lifecycle-terminal-outcome). |

<a id="core-lifecycle-intake-and-shaping"></a>
### Intake and shaping

Effect:
- Turns ordinary user intent into a concrete goal, active scope, non-goals, acceptance criteria, Autonomy Boundary, and next safe action.

Required honesty:
- If a user-owned issue blocks the next safe action, expose the judgment need instead of guessing.

<a id="core-lifecycle-terminal-outcome"></a>
### Terminal outcome

Effect:
- Completion, cancellation, or supersession ends the Task path.

Not allowed:
- Cancellation and supersession are terminal, but they are not successful completion.
- They do not satisfy evidence, acceptance, or risk requirements for completion.

## 7. Active gates

Gates are compatibility summaries for progress, write, Run recording, and close. This page owns their product meaning. Public fields, exact values, and wire shapes are owned by [API State Schemas](api/schema-state.md) and [API Value Sets](api/schema-value-sets.md).

| Gate area | Meaning | Common confusion to avoid |
|---|---|---|
| Scope gate | Whether active scope and Change Unit cover the requested work. | It does not decide product or technical questions for the user. |
| Decision gate | Whether unresolved user-owned judgment blocks progress, write, or close. | It does not replace evidence, sensitive-action approval, final acceptance, or residual-risk acceptance. |
| Sensitive-action approval gate | Whether a named sensitive step inside `SensitiveActionScope` is approved. | It is not `Write Authorization` and not broad permission. |
| Write-compatibility gate | See [Write-compatibility gate](#core-gate-write-compatibility). | See [Write-compatibility gate](#core-gate-write-compatibility). |
| Evidence gate | Whether close-relevant required support is present and usable enough for the close path. | Evidence does not prove more than recorded and does not replace user acceptance. |
| Acceptance gate | Whether required final acceptance is present for the visible close basis. | It cannot fill evidence gaps or accept residual risk. |
| Residual-risk gate | Whether close-relevant residual risk is visible and, when required, accepted. | Accepted risk is not verification and does not make the result risk-free. |
| Close-readiness gate | Whether all close-relevant checks support an honest close. | A close blocker means the Task remains open until the owner path addresses it. |

<a id="core-gate-write-compatibility"></a>
### Write-compatibility gate

Meaning:
- Whether a product-file write attempt is compatible with active scope and a consumable `Write Authorization`.

Not allowed:
- It does not approve commands, hosts, network, secrets, deployments, or destructive operations.

Verification and Manual QA are conceptual boundaries in the current MVP, not active gates. They must not be described as active close requirements unless a future owner promotes them.

## 8. Write authorization boundary

Concept:
- `Write Authorization` is the Core record that makes one product-file write attempt compatible with current Harness state.

Creation:
- It is created only through the compatible non-dry-run `prepare_write` path defined by the API owner.

Inputs:
- Current Harness state.
- Active Task and Change Unit scope.
- The intended product-file write attempt.
- A compatible non-dry-run `prepare_write` result.

Properties:
- Scope-limited: it covers the intended product-file write attempt, not future work or a broader project area.
- Single-use: a compatible product-write Run consumes it once. Reuse, replay, and stale-state behavior are API/storage-owned details.
- Cooperative: it tells a connected agent or surface what is compatible with Harness state; it does not enforce OS-level prevention.

Not the same as:
- `sensitive_approval`, command approval, dependency approval, host/network/secret access, deployment approval, destructive-action approval, system access, or final acceptance.
- Proof that the write happened, evidence creation, acceptance, residual-risk acceptance, or Task close.

Owner links:
- [Prepare Write Method](api/method-prepare-write.md)
- [Record Run Method](api/method-record-run.md)
- [API State Schemas](api/schema-state.md)
- [Storage Effects](storage-effects.md)

Decision reason boundary:
- `WriteDecisionReason` belongs to prepare-write decision output.
- `CloseReadinessBlocker` belongs to close-readiness blocking data.
- They answer different questions and must not be interchanged.

## 9. Evidence and run authority

| Record | What it can establish | What it cannot establish |
|---|---|---|
| Run | That an execution or observation was recorded with the available context and refs. | That missing authorization, missing judgment, or missing approval existed retroactively. |
| Evidence summary | That specific close-relevant claims have recorded support, gaps, refs, or coverage expectations. | That unrecorded behavior happened, that the result is accepted, or that risk is accepted. |
| `ArtifactRef` | See [`ArtifactRef` evidence use](#core-evidence-artifactref-use). | See [`ArtifactRef` evidence use](#core-evidence-artifactref-use). |
| Projection or report | That a display was generated from available state and refs. | That the display itself is authority, evidence, or acceptance. |

<a id="core-evidence-artifactref-use"></a>
### `ArtifactRef` evidence use

Can establish:
- An artifact reference is available for evidence use when artifact owners allow it.

Cannot establish:
- The artifact content is safe, sufficient, or readable beyond the recorded integrity/redaction/availability facts.

### Evidence authority

Concept:
- Evidence records support only the claims they record at their recorded scope.

Inputs:
- Run records.
- Evidence summaries.
- Evidence-eligible artifacts and `ArtifactRef` values when artifact owners allow them.
- Related refs and coverage expectations.

Can establish:
- A passing test log supports the test it names.
- A screenshot supports the visible state it captures.
- An artifact supports only the content and integrity facts represented by the artifact owners.

Not the same as:
- Proof of broader correctness.
- Final acceptance, future Manual QA, future verification, or residual-risk acceptance.
- Proof of unrecorded behavior.

Owner links:
- [API Artifact Schemas](api/schema-artifacts.md)
- [Artifact Storage](storage-artifacts.md)
- [API Judgment Schemas](api/schema-judgment.md)

<a id="close_task"></a>
## 10. Close readiness

Concept:
- Close readiness is the Core evaluation concept for whether the current Task can close honestly.

Inputs:
- Current Core state.
- Active Task scope and Change Unit scope.
- Required user-owned judgments.
- Required sensitive-action approval.
- Write and Run compatibility.
- Evidence and artifacts.
- Final acceptance.
- Residual risk and required residual-risk acceptance.
- Recovery constraints.

Not the same as:
- `CloseReadinessBlocker`.
- `intent=complete`.
- User acceptance alone.
- Preflight rejection.

Owner links:
- [Close Task Method](api/method-close-task.md)
- [API State Schemas](api/schema-state.md)
- [API Value Sets](api/schema-value-sets.md)
- [Storage Effects](storage-effects.md)
- [API Errors](api/errors.md)

For an `intent=complete` close attempt, Core evaluates blockers in this conceptual order. Later rows do not satisfy earlier rows.

| Order | Check area | Close-readiness meaning |
|---:|---|---|
| 1 | Task lifecycle | The selected Task must be eligible for the requested terminal path. |
| 2 | Open or unrepaired Runs | Close cannot rely on open, unsafe, interrupted, incompatible, or unrepaired Run state. |
| 3 | Scope and Change Unit | Active scope, acceptance criteria, and the applicable completion policy must support the close claim. |
| 4 | User-owned judgment | Required product, technical, scope, and other non-sensitive user judgments must be resolved and compatible. |
| 5 | Sensitive-action approval | Required sensitive-action approval must be present and compatible with the bounded step. |
| 6 | Write and Run compatibility | Product-write claims must be backed by compatible authorization and recorded Run relationships. |
| 7 | Baseline and surface capability | The baseline and connected surface must honestly support the close claim and any guarantee display. |
| 8 | Evidence sufficiency | Required evidence coverage must be present, current, and usable for the close basis. |
| 9 | Artifact availability | Close-relevant artifacts must be available and usable under artifact-owner rules. |
| 10 | Final acceptance | Required final acceptance must be tied to the visible close basis. |
| 11 | Residual-risk visibility | Known close-relevant risk must be visible enough for the user to judge. |
| 12 | Residual-risk acceptance | Required acceptance of visible residual risk must be compatible with the requested close. |
| 13 | Recovery constraints | Remaining repair, corruption, reconciliation, or recovery work must be handled before close. |
| 14 | Close transition | If no blocker remains, the terminal transition may proceed through the API-owned method behavior; otherwise the Task stays open. |

Preflight failures:
- Stale state, invalid request identity, local access failure before evaluation, and similar API-owned failures are not semantic close-readiness findings.
- They are routed through the API and error owners.

## 11. Blockers and waivers

### Blocker

Concept:
- A blocker is a structured reason progress, write, Run recording, or close cannot proceed honestly.

Not the same as:
- Projection prose.
- Broad approval.
- A successful-looking close result.

Owner links:
- [API State Schemas](api/schema-state.md)
- [API Value Sets](api/schema-value-sets.md)

### Close blocker

Concept:
- A close blocker is a close-relevant reason that prevents honest close readiness.

Not the same as:
- `WriteDecisionReason`.
- Proof of storage effects by itself.

Owner links:
- [Close Task Method](api/method-close-task.md)
- [API State Schemas](api/schema-state.md)
- [API Value Sets](api/schema-value-sets.md)

### `CloseReadinessBlocker`

Concept:
- `CloseReadinessBlocker` is the API data representation of close blocking reasons.

Not the same as:
- The whole close-readiness concept.
- A prepare-write reason.
- Proof of persistence by itself.

Owner links:
- [API State Schemas](api/schema-state.md)
- [API Value Sets](api/schema-value-sets.md)
- [API Errors](api/errors.md)

### Waiver

Concept:
- A waiver is a scoped exception to a named requirement where the responsible owner allows it.

Allowed effect:
- It can unblock only the named requirement and only through the owner path that permits it.

Not the same as:
- Decision deferral.
- Scope creation, sensitive-action approval, required evidence, final acceptance, or residual-risk visibility.
- QA evidence, a QA pass, verification, or an assurance upgrade.

Owner links:
- [Later Candidate Index](../later/index.md)

## 12. Residual risk

Concept:
- Residual risk is known remaining uncertainty, an unchecked condition, limitation, or trade-off that matters to close.

Inputs:
- The visible named risk.
- The requested close and visible close basis.
- Related evidence, artifact, blocker, or Run refs.
- Compatible `residual_risk_acceptance` when close depends on accepting the risk.

Required order:
- Known close-relevant residual risk must be visible before successful close.
- The user cannot accept a risk that has not been made visible enough to judge.

Scope:
- Acceptance applies to the named visible risk for the requested close, not to all unknowns.

Not the same as:
- Verification, evidence sufficiency, QA, sensitive-action approval, final acceptance, or a no-risk result.
- A waiver or automatic success.

Current MVP path:
- The current path is compact residual-risk summary, blockers, evidence refs, and `user_judgment` refs unless an owner promotes more.
- Rich risk workflows are later material.

Owner links:
- [API Judgment Schemas](api/schema-judgment.md)
- [API State Schemas](api/schema-state.md)
- [Later Candidate Index](../later/index.md)

## 13. Cross-owner links

| Topic | Owner |
|---|---|
| API method behavior, request/response shapes, envelopes, dry-run/rejection branches, and method effects | [MVP API router](api/mvp-api.md), method owner documents, [API Schema Core](api/schema-core.md) |
| State-shaped API data, `ShapingReadiness`, `CloseReadinessBlocker`, `ValidatorResult`, and public state fields | [API State Schemas](api/schema-state.md), [API Value Sets](api/schema-value-sets.md) |
| User judgment schema, `SensitiveActionScope`, and accepted-risk input shapes | [API Judgment Schemas](api/schema-judgment.md) |
| Artifact schemas and lifecycle | [API Artifact Schemas](api/schema-artifacts.md), [Artifact Storage](storage-artifacts.md) |
| Public error codes, error routing, and error precedence | [API Errors](api/errors.md) |
| Storage records and effects | [Storage Records](storage-records.md), [Storage Effects](storage-effects.md), [Storage Versioning](storage-versioning.md) |
| Projection authority and read-only display boundaries | [Projection Authority Reference](projection-and-templates.md) |
| Status card, judgment request, run/evidence summary, close result, and agent context packet bodies | [Template Bodies](template-bodies.md) |
| Security guarantee wording, cooperative/detective/preventive claims, and local access posture | [Security Reference](security.md) |
| Product Repository, Harness Server, and Harness Runtime Home separation | [Runtime Boundaries Reference](runtime-boundaries.md) |
| Design-quality boundaries and non-gate routing | [Design Quality](design-quality.md) |
| Connector behavior and surface capability posture | [Agent Integration Reference](agent-integration.md) |
| Later candidates and future assurance, waiver, QA, verification, and fixture material | [Later Candidate Index](../later/index.md) |

If another document needs exact schema, DDL, rendered template text, public error codes, or later candidate catalogs, it must link to the owner instead of redefining them here.
