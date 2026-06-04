# Judgment Request Template

## Used when

Use `judgment-request` when the user owns a choice that affects progress, scope, sensitive-action permission, work acceptance, or residual-risk acceptance. This is the MVP-1 prompt shape for ordinary user-owned judgments.

Implementation tier: MVP-1 User Work Loop view. The full Decision Packet presentation is later/full-profile scope and lives in [later-profile/decision-packet.md](later-profile/decision-packet.md).

Boundary: this template displays a pending or recorded `user_judgment`; it does not create the judgment record by itself, grant Write Authorization, perform QA or verification, record work acceptance, accept residual risk, or close a Task.

## Source records

- pending or recorded `user_judgment`
- `judgment_type`, `presentation`, and `display_label`
- affected Task, scope, Change Unit, criteria, paths, gates, or sensitive-action scope
- options or selected outcome
- consequences, uncertainty, and what the agent is not deciding
- minimal source refs needed to identify the affected work
- evidence, risk, approval, QA, verification, or close refs only when they affect the judgment

## Rendered sections

- judgment request
- judgment type
- decision
- options or selected outcome
- consequence
- uncertainty
- agent is not deciding
- next safe action or deferral effect
- refs

## Full template

````text
Judgment request: {short_title}
Judgment type: {display_label} (`{judgment_type}`)
Decision: {decision}
Options: {options_or_selected_outcome}
Consequence: {consequence}
Uncertainty: {uncertainty}
Agent is not deciding: {not_deciding}
If deferred: {deferral_effect|not_applicable}
Next safe action after answer: {next_safe_action}
Refs: judgment={user_judgment_ref}; task={task_ref}; scope={scope_ref|none}; evidence={evidence_refs|none}; risk={risk_refs|none}
````

## Notes

Small judgments should fit on one screen. Use `presentation=full` only when the active profile or complexity requires fuller trade-offs, recommendation, affected gates, evidence/risk refs, and deferral analysis.

Do not merge sensitive-action approval, work acceptance, and residual-risk acceptance into one broad approval prompt.
