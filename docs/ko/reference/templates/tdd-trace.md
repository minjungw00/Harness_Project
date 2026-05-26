# TDD-TRACE Template

## 사용 시점

Change Unit에서 TDD가 필요하거나 선택 또는 기록된 상태이고 RED, GREEN, refactor/check, waiver, evidence ref를 읽기 쉬운 projection으로 볼 때 `TDD-TRACE`를 사용합니다.

## 기준 기록

- `tdd_traces`
- selected `feedback_loops`
- Task와 Change Unit 참조
- RED, GREEN, refactor/check artifact 참조
- Evidence Manifest coverage 참조
- waiver 또는 non-TDD justification 참조
- `tdd_trace` 관련 design-quality validator 결과
- 읽기용 보기 최신성(projection freshness) 입력

## 렌더링 섹션

- Identity
- Red
- Green
- Refactor
- Non-TDD Justification
- Evidence Refs

## 전체 템플릿

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

> Projection 보기: `source_state_version`와 `updated_at` 기준으로 렌더링되며 TDD record와 ref를 표시합니다. Plan text는 기록된 artifact 또는 result ref가 뒷받침하기 전까지 RED evidence가 아닙니다.

## Identity
- task_id:
- change_unit_id:
- trace 상태: required | recorded | waived | not_required
- 요구/출처:
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
````

## 메모

이 template은 렌더링 결과일 뿐 기준 상태가 아닙니다. RED target 또는 plan은 계획 context이며, 실제 RED evidence는 여전히 기록된 artifact 또는 result ref에서 나와야 합니다.
