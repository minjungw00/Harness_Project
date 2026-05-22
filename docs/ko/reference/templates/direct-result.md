# DIRECT-RESULT Template

## 사용 시점

작은 direct work가 close되었거나 `work`로 escalation된 뒤 결과를 간단히 보여줘야 할 때 `DIRECT-RESULT`를 사용합니다.

## 기준 기록

- direct run 기록
- direct product write에 있는 경우 consumed Write Authorization 참조
- changed path
- performed check
- artifact 참조
- escalation flag
- close assurance

## 렌더링 섹션

- Request
- Scope
- Changed Files
- Checks And Validator Outcomes
- Outcome
- Assurance
- Escalation
- Evidence Refs

## 전체 템플릿

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

## 메모

Policy 또는 user가 detached verification 또는 다른 gate를 요구하지 않으면 direct work는 기본적으로 self-checked 상태로 close될 수 있습니다. Consumed Write Authorization 참조를 표시할 수 있지만, projection이 기준 authorization 기록이 되는 것은 아닙니다.
