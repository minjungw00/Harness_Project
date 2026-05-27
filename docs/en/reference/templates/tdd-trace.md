# TDD-TRACE Template

## Used when

Use `TDD-TRACE` when TDD is required, selected, or recorded for a Change Unit and the RED, GREEN, refactor/check, waiver, and evidence refs need a readable projection.

Boundary: projection template only; it does not authorize runtime/server implementation or generated operational outputs. Shared phase and projection rules live in [Template Reference](README.md#used-when).

## Source records

- `tdd_traces`
- selected `feedback_loops`
- Task and Change Unit refs
- RED, GREEN, and refactor/check artifact refs
- Evidence Manifest coverage refs
- waiver or non-TDD justification refs
- finding routes through Evidence Manifest, Decision Packet, Change Unit, Residual Risk, Manual QA, Eval, close-blocker, or follow-up refs when applicable
- design-quality validator results related to `tdd_trace`
- projection freshness inputs

## Rendered sections

- Identity
- Red
- Green
- Refactor
- Non-TDD Justification
- Evidence Refs
- Finding Routing

## Full template

````md
---
doc_type: tdd_trace
tdd_trace_id: TDD-0001
task_id: TASK-0001
change_unit_id: CU-01
status: recorded
source_state_version: 43
updated_at: 2026-05-06T09:40:00+09:00
---

# TDD-0001 Trace Title

> Projection view: rendered from `source_state_version` at `updated_at`; displays TDD records and refs. Plan text is not RED evidence until recorded artifact or result refs support it.

## Identity
- task_id:
- change_unit_id:
- trace status: required | recorded | waived | not_required
- requirement/source:
- feedback loop ref:
- evidence manifest coverage ref:

## Red
- target / plan:
- failing test ref:
- command:
- result: failed_as_expected | failed_unexpectedly | missing
- log ref:
- recorded before non-test implementation: yes | no | waived
- target / plan counts as Evidence Manifest coverage: no

## Green
- command:
- result: passed | failed | missing
- log ref:

## Refactor
- performed: yes | no
- notes:
- verification command:
- log ref:

## Non-TDD Justification
- reason:
- feedback loop ref:
- alternate feedback loop:
- waiver recorded before non-test implementation: yes | no

## Evidence Refs
- test:
- red log:
- green log:
- refactor/check log:
- Evidence Manifest:
- diff:

## Finding Routing
- evidence gaps or support:
- Decision Packet candidates or refs:
- Change Unit update or follow-up:
- residual-risk candidates or refs:
- Manual QA or Eval refs:
- close blockers:
````

## Notes

This template is a rendered shape, not canonical state. RED target or plan text is planning context; actual RED evidence must still come from recorded artifact or result refs.

If TDD is advisory but not required or selected, no TDD waiver is needed. Render required, selected, recorded, or waived TDD from owner records only; route any findings through existing owner refs rather than adding template-only state.
