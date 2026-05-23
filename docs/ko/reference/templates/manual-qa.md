# MANUAL-QA Template

## 사용 시점

Manual QA가 required, performed, waived, pending 상태이거나 `qa_gate`에 반영되어 있고 해당 기록을 읽기 쉬운 projection으로 볼 때 `MANUAL-QA`를 사용합니다.

## 기준 기록

- `manual_qa_records`
- Task와 Change Unit 참조
- `qa_gate`
- Manual QA profile, setup, checklist, result, waiver, finding
- screenshot, browser log, video, note artifact 참조
- QA waiver 또는 failure와 관련된 Residual Risk와 Decision Packet 참조
- `manual_qa` 관련 design-quality validator 결과
- projection 최신성 입력

## 렌더링 섹션

- Identity
- Setup
- Checklist
- Result
- Waiver And Risk
- Findings
- Evidence Refs

## 전체 템플릿

````md
---
doc_type: manual_qa
manual_qa_record_id: null
task_id: TASK-0001
change_unit_id: CU-01
qa_gate: pending
result: null
source_state_version: 45
updated_at: 2026-05-06T10:05:00+09:00
---

# Manual QA

## Identity
- manual_qa_record_id: QA-0001 | null
- task_id:
- change_unit_id: CU-01 | null
- qa_profile: ui_quality | workflow | copy | accessibility | browser_smoke | performance_smoke | other
- required: yes | no
- performed by:

## Setup
- build/run command:
- test account/data:
- route or screen:

## Checklist
- [ ] primary workflow works
- [ ] errors are understandable
- [ ] visual layout acceptable
- [ ] accessibility smoke check
- [ ] no obvious regression

## Result
- record result: passed | failed | waived | null when no record exists
- qa_gate: pending | passed | failed | waived | not_required
- qa_gate note: 기준 close-relevant gate; 이 projection은 표시 전용
- summary:
- waiver reason:

## Waiver And Risk
- waiver Decision Packet:
- residual risk refs:
- accepted residual-risk summary:

## Findings
| Severity | Finding | Suggested Action | Follow-up CU |
|---|---|---|---|
| minor | | | |

## Evidence Refs
- screenshot:
- browser log:
- video:
- note:
````

## 메모

이 template은 렌더링 결과일 뿐 기준 상태가 아닙니다. `qa_gate`가 기준 close-relevant gate이며, 이 projection은 그 값을 표시만 합니다.
