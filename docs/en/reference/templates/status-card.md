# Status Card Template

## Used when

Use `status-card` when MVP-1 needs a short user-visible current-state view. It shows what is happening now, what is in scope, what the user must decide, what evidence exists or is missing, what blocks close, and the next safe action.

Implementation tier: MVP-1 User Work Loop view. Engineering Checkpoint may return plain structured status/blocker output instead of this card.

Boundary: this template is rendered display only. It is not Core state, not evidence, not approval, not final acceptance, not residual-risk acceptance, not Write Authorization, and not close readiness authority. It must be rendered from current Core-owned state and refs, not stale chat.

## Source records

- current Task summary, work shape, and next safe action
- current scope, non-goals, and active Change Unit summary when useful to the user
- pending user judgments, rendered with user-readable labels
- active blockers and the plain reason progress or close is held
- current evidence summary, supporting refs, redaction or availability notes, and evidence gaps
- close blockers, final-acceptance need, residual-risk visibility, and residual-risk acceptance status when relevant
- design-quality routed action only when it changes the visible next step
- guarantee level or unavailable capability status
- short source refs, render time, and freshness state

## Rendered sections

- work
- scope
- judgment
- blocked reason
- evidence
- checks
- close
- next safe action
- sources and freshness

## Full template

````text
{task_id} {title}
Display only: derived from Core state and refs; not Core state or a Write Authorization.

Work: {work_shape}. {current_task_summary}
Scope: {scope_summary}
Out of scope: {non_goals|none}
Blocked because: {active_blocked_reason|none}
User must decide: {pending_user_judgments_with_localized_labels|none}
Evidence: {evidence_status}. {known_evidence_summary|none}
Evidence gaps: {evidence_gaps|none}
Checks: {check_summary|none}
Close: {close_readiness_summary}; blockers={close_blockers|none}
Design quality action: {design_quality_routed_action|none}
Remaining risk: {residual_risk_visibility|none}
Next safe action: {next_safe_action}
Guarantee: {guarantee_level_or_unavailable}; {guarantee_note}
Sources/freshness: {source_freshness_summary}
````

## Notes

Keep this card readable for a user who does not know Harness internals. Do not dump schemas, DDL, event logs, full artifacts, full report bodies, full templates, future catalogs, detailed evidence catalogs, detailed evaluation bodies, or later assurance records.

When a field has no source record, render `none`, `unknown`, `not_required`, or an explicit blocker instead of inventing state.

Always render the guarantee line. For MVP-1 default behavior, the note should say cooperative hold or detective reporting when that is the actual limit. If Core/MCP is unavailable, render the unavailable condition instead of a stale or guessed guarantee.

Design-quality content should fit one line: the current routed action and, when blocking, the single next action. Do not list full domain-language, module/interface, TDD, stewardship, feedback-loop, QA, or assurance catalogs in the MVP-1 status card.

Agent-only refs and action-boundary details belong in [agent-context-packet](agent-context-packet.md). Put a ref in the status card only when it helps the user decide, understand a blocker, or inspect source freshness.
