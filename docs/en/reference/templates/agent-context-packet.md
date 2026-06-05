# Agent Context Packet Template

## Used when

Use `agent-context-packet` when an agent needs compact, current context for the next safe action. It is optimized for accuracy, freshness, Core-derived refs, active blockers, unresolved user judgments, evidence gaps, close blockers, guarantee level, and one next action, not for user-facing prose or full report detail.

Implementation tier: MVP-1 support view. It can be returned as a structured payload or prompt-sized text. It is not a required persisted Markdown projection.

Boundary: this packet is support context only. It cannot authorize writes, satisfy gates, create evidence, grant approval, record final acceptance, accept residual risk, create close readiness, or close a Task.

## Source records

- task and active Change Unit refs
- current state version and source refs
- active scope and non-goals
- unresolved user judgments
- active blockers
- evidence gaps
- close blockers
- residual-risk summary if active
- guarantee level or unavailable capability status
- exactly one next safe action

## Rendered sections

- task and change unit refs
- state version and source refs
- active scope
- unresolved user judgments
- blockers
- next safe action
- evidence gaps
- close blockers
- residual-risk summary
- guarantee level

## Full template

````text
agent_context_packet:
  display_only: true
  authority: none; use current Core state for authority
  task_ref: {task_ref}
  change_unit_ref: {change_unit_ref|none}
  state_version: {source_state_version}
  source_refs: {source_refs}
  freshness: {freshness_state}
  active_scope: {scope_summary}
  unresolved_user_judgments: {pending_user_judgment_refs_with_kind_labels|none}
  blockers: {active_blockers|none}
  next_safe_action: {next_safe_action}
  evidence_gaps: {evidence_gaps|none}
  close_blockers: {close_blockers|none}
  residual_risk_summary: {residual_risk_summary_if_active|none}
  guarantee_level: {guarantee_level_or_unavailable}
````

## Notes

Keep the packet one screen or less. It carries only current, next-action-relevant state. Do not include full schemas, full reference docs, full historical event logs, registered artifact file bodies, full report bodies, full templates, unrelated templates, full design-quality catalogs, or future catalog material by default.

If the next action needs a fuller owner section, the agent should pull that owner section on demand instead of embedding it in the packet.

The `guarantee_level` field is required context. If Core/MCP is unavailable, set it to the unavailable/capability condition and treat Harness-dependent state, write, evidence, acceptance, residual-risk, and close claims as unavailable until refreshed.
