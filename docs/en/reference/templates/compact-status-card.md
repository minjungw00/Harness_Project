# Compact Status Card Template

## Used when

Use the compact status card when a short current-state display needs to show task state, next action, pending user decision, risk, gates, Manual QA, and latest report.

## Source records

- current Task state and lifecycle phase
- pending Decision Packet summary
- risk summary
- evidence, design, and QA gates
- latest report ref

## Rendered sections

- task identity
- state
- next action
- user decision
- risk
- evidence gate
- design gate
- Manual QA
- latest report

## Full template

````text
TASK-{id} {title}
State: {mode} / {lifecycle_phase}
Next action: {next_action}
User decision: {pending_decision_summary|none}
Risk: {risk_summary}
Evidence gate: {evidence_gate}
Design gate: {design_gate}
Manual QA: {qa_gate display: pending|passed|failed|waived|not_required}
Latest report: {latest_report|none}
````

## Notes

This template is a rendered card shape, not canonical state. Gate values remain owned by canonical state.
