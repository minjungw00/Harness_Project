# Agent Context Packet Template

## Used when

Use `agent-context-packet` when an agent needs compact, current context for the next safe action. It is optimized for accuracy, freshness, refs, and blockers, not for user-facing prose or full report detail.

Implementation tier: MVP-1 support view. It can be returned as a structured payload or prompt-sized text. It is not a required persisted Markdown projection.

Boundary: this packet is support context only. It cannot authorize writes, satisfy gates, create evidence, grant approval, record final acceptance, accept residual risk, create close readiness, or close a Task.

## Source records

- current Task, work shape, lifecycle, and next safe action
- active scope, non-goals, Change Unit refs, and write-preparation blockers
- pending user judgments and judgment request refs
- evidence refs, Run refs, ArtifactRefs, `redaction_state`, and evidence gaps
- close blockers, residual-risk summary, final-acceptance need/status, and relevant owner refs
- design-quality routed action and owner refs when the next safe action depends on them
- source clocks, freshness state, MCP/Core availability, and guarantee level
- owner document or owner-section pointers needed for the next action

## Rendered sections

- current task
- active scope
- pending judgments
- blockers
- evidence state
- close and residual-risk state
- next safe action
- freshness and source refs
- pull-on-demand pointers

## Full template

````text
agent_context_packet:
  display_only: true
  authority: none; use current Core state for authority
  task: {task_id} {task_summary}
  work_shape: {work_shape}
  scope: {scope_summary}
  non_goals: {non_goals|none}
  pending_judgments: {pending_user_judgment_refs|none}
  active_blockers: {active_blockers|none}
  evidence: {evidence_refs_and_gaps}
  design_quality: {design_quality_routed_action|none}
  close: {close_blockers_and_acceptance_state}
  residual_risk: {residual_risk_summary}
  next_safe_action: {next_safe_action}
  guarantee: {guarantee_level_or_unavailable}
  sources:
    state_version: {source_state_version}
    refs: {source_refs}
    freshness: {freshness_state}
    rendered_at: {updated_at}
  pull_if_needed: {owner_section_refs_for_next_action|none}
````

## Notes

Keep the packet one screen or less. Do not include full schemas, full reference docs, full historical event logs, registered artifact file bodies, full report bodies, full templates, unrelated templates, full design-quality catalogs, or future catalog material by default.

The guarantee field is required context. If Core/MCP is unavailable, set it to the unavailable/capability condition and treat Harness-dependent state, write, evidence, acceptance, residual-risk, and close claims as unavailable until refreshed.
