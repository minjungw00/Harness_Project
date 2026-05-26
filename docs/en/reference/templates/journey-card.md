# JOURNEY-CARD Template

## Used when

Use `JOURNEY-CARD` when a current-position card needs to show where the work is, what is in and out of scope, what is blocking the next move, what judgment is pending, the Autonomy Boundary, Write Authority Summary, evidence and checks, residual risk, close context, gates, and projection freshness.

## Source records

- current Task state and gates
- scope and out-of-bounds summaries
- active Change Unit
- Autonomy Boundary summary
- Write Authorization, approval, baseline, and guarantee refs
- active Decision Packet refs
- primary blocker, secondary blocker, and smallest unblocker display summaries
- blocker owner display summary
- evidence coverage, verification, and Manual QA summaries
- residual-risk summary and refs
- acceptance and close-reason summaries
- latest evidence, Eval, Manual QA, and report refs
- projection freshness inputs
- state, baseline, evidence, MCP, and capability freshness/blocker display summaries

Judgment, write-authority, close-impact, residual-risk, and freshness placeholders are display bindings derived from the records above. If a user decision is actually needed, render a Decision Packet or decision prompt rather than treating this card as the judgment-context source.

## Rendered sections

- current position and next action
- scope and out of bounds
- blocking now
- Judgment context
- Autonomy boundary
- Write Authority Summary
- Evidence and checks
- Residual risk
- Close context
- Gates
- Projection freshness
- State/input freshness and capability availability

## Full template

````text
TASK-{id} {title}
Display only: current-position view, not canonical state or write authority.
Where we are: {mode} / {lifecycle_phase} / {current_position}
Scope: {scope_summary|none}
Out of bounds: {out_of_bounds_summary|none}
Next action: {next_action}

Blocking now:
- primary: {primary_blocker_label|none}
- owner: {primary_blocker_owner_label|none}
- smallest unblocker: {smallest_unblocker|none}
- also blocking: {secondary_blockers_summary|none}

Judgment context:
- pending decision: {decision_packet_ref|none}
- user deciding: {what_user_is_deciding|none}
- agent may decide: {what_agent_may_decide_without_user}

Autonomy boundary:
- profile: {autonomy_profile}
- agent may do: {agent_may_do}
- user judgment required: {user_judgment_required}
- AFK stop conditions: {afk_stop_conditions}

Write Authority Summary:
- active Change Unit: {active_change_unit_ref|none}
- write authorization: {write_authorization_ref|none}
- allowed paths: {allowed_paths}
- allowed tools: {allowed_tools}
- allowed commands: {allowed_commands}
- allowed network targets: {allowed_network_targets}
- secret scope: {secret_scope}
- sensitive categories: {sensitive_categories}
- approval status: {approval_status}
- baseline: {baseline_ref|none}
- guarantee: {guarantee_display}
- note: Autonomy Boundary is judgment latitude, not write authority.

Evidence and checks:
- action: {next_evidence_action}
- needed for: {evidence_needed_for}
- latest evidence: {latest_evidence_ref|none}
- verification: {verification_summary|none}
- self-check vs detached boundary: {self_check_or_detached_boundary|none}
- Manual QA: {manual_qa_summary|not_required}
- omitted or blocked impact: {redaction_availability_summary|none}

Residual risk:
- status: {residual_risk_status}
- close impact: {residual_risk_close_impact}
- accepted residual-risk record refs: {accepted_residual_risk_record_refs|none}

Close context:
- blockers: {close_blockers|none}
- acceptance: {acceptance_summary|not_required}
- close reason: {close_reason|none}

Gates:
- scope: {scope_gate}
- decision: {decision_gate}
- approval: {approval_gate}
- evidence: {evidence_gate}
- verification: {verification_gate}
- Manual QA: {qa_gate display: pending|passed|failed|waived|not_required}
- acceptance: {acceptance_gate}

Projection freshness: {projection_freshness}; source_state_version={source_state_version|unknown} (view freshness, not task result)
State/input freshness: {state_baseline_evidence_freshness_summary|current or none}
````

## Notes

This template is a rendered shape, not canonical state. Persisted `JOURNEY-CARD` Markdown is optional; current-position Journey Card output in status, next, and significant resume flows remains a read/display surface.

Close context in a Journey Card is compact status/resume display. `TASK` owns the continuity Close Summary for active or recently closed `work` tasks, while `DIRECT-RESULT` owns the low-ceremony close impact summary for direct work. These displays follow the [projection/report boundary](../document-projection.md#projection-principles); close and gate effects still come from owner records.

The blocker lines translate API and state records into user-facing status. The primary blocker should be the first blocker the next action must resolve, and the owner label should make clear whether the next move is user-owned, agent-resolvable, or surface/system-owned. The owner may render as `none` or be omitted when there is no primary blocker. Secondary blockers stay visible only when they affect the follow-on path. Do not expose raw `ErrorCode` values as the only explanation.

When latest or next evidence includes `secret_omitted` or `blocked` artifact refs, this card should show only the availability impact. It must not include omitted values or blocked raw payload content.
