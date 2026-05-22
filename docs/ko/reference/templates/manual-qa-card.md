# Manual QA Card Template

## 사용 시점

필수 Manual QA의 기록, gate, profile, 대상, checklist, 기록할 근거를 간결한 안내 카드로 보여줄 때 Manual QA Card를 사용합니다.

## 기준 기록

- Manual QA requirement와 `qa_gate`
- 존재하는 경우 Manual QA 기록
- QA profile
- 대상 screen 또는 flow
- checklist item
- 예상 screenshot, walkthrough note, browser log 근거

## 렌더링 섹션

- Manual QA requirement
- 기록
- gate
- profile
- 대상
- checklist
- 기록할 근거
- 결과 안내

## 전체 템플릿

````text
Manual QA가 필요합니다.

Record: {manual_qa_record_id|none until recorded}
Gate: {qa_gate display: pending|passed|failed|waived|not_required}
Profile: {profile}
Target: {screen_or_flow}
Checklist:
- {checklist_item}

기록할 evidence:
- screenshot or walkthrough note
- browser log when relevant

QA result를 기록하시겠습니까?
````

## 메모

이 template은 렌더링 결과인 카드 형태일 뿐 기준 QA 상태가 아닙니다. `qa_gate`는 close-relevant gate로 남습니다.
