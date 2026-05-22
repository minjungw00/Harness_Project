# DIRECT-RESULT Template

## Used when

Use `DIRECT-RESULT` for a compact result report after small direct work closes or escalates.

## Source records

- direct run record
- consumed Write Authorization ref, when present for direct product writes
- changed paths
- checks performed
- artifact refs
- escalation flag
- close assurance

## Rendered sections

- Request
- Scope
- Changed Files
- Checks And Validator Outcomes
- Outcome
- Assurance
- Escalation
- Evidence Refs

## Full template

````md
---
doc_type: direct_result
task_id: TASK-0001
run_id: RUN-20260506-093015-LEAD-01
result: passed
assurance_level: self_checked
surface_id: reference
source_state_version: 41
updated_at: 2026-05-06T09:40:00+09:00
---

# DIRECT-RESULT

## Request
- user request:

## Scope
- direct run scope:
- limits:
- write authorization:
- allowed paths:
- allowed tools:
- allowed commands:
- approval refs:

## Changed Files
- `path/to/file`

## Checks And Validator Outcomes
### Core Checks And Command Checks
- changed_paths:
- approval_scope:
- test:
- build:

### ValidatorResult IDs
- context_hygiene_check:
- surface_capability_check:

## Outcome
- result summary:

## Assurance
- assurance_level:
- meaning:
- detached verify needed:

## Escalation
- escalated_to_work: yes | no
- reason:

## Evidence Refs
- logs:
- diff:
- follow-up report:
````

## Notes

Direct work may close self-checked by default unless policy or the user requires detached verification or other gates. A consumed Write Authorization ref may be displayed, but the projection does not become the canonical authorization record.
