# 판단 요청 템플릿

## 사용 시점

진행, 범위, 민감 동작 승인, 작업 수락, 잔여 위험 수용에 영향을 주는 선택을 사용자가 소유할 때 `judgment-request`를 사용합니다. 이것은 일반 사용자 소유 판단을 위한 MVP-1 prompt shape입니다.

구현 계층: MVP-1 사용자 작업 루프 보기입니다. 전체 Decision Packet presentation은 later/full-profile 범위이며 [later-profile/decision-packet.md](later-profile/decision-packet.md)에 있습니다.

경계: 이 템플릿은 대기 중이거나 기록된 `user_judgment`를 표시합니다. 이 표시만으로 판단 기록을 만들거나, Write Authorization을 부여하거나, QA 또는 검증을 수행하거나, 작업 수락을 기록하거나, 잔여 위험을 수용하거나, Task를 닫지 않습니다.

## 기준 기록

- 대기 중이거나 기록된 `user_judgment`
- `judgment_type`, `presentation`, `display_label`
- 영향을 받는 Task, 범위, Change Unit, 기준, 경로, gate 또는 민감 동작 범위
- 선택지 또는 선택된 결과
- 결과 영향, 불확실성, 에이전트가 사용자 대신 판단하지 않는 것
- 영향을 받는 작업을 식별하는 데 필요한 최소 출처 참조
- 판단에 영향을 줄 때만 근거, 위험, 민감 동작 승인, QA, 검증, 닫기 참조

## 렌더링 섹션

- 판단 요청
- 판단 유형
- 판단 내용
- 선택지 또는 선택된 결과
- 결과 영향
- 불확실성
- 에이전트가 사용자 대신 판단하지 않는 것
- 다음 안전한 행동 또는 미룰 때 영향
- 참조

## 전체 템플릿

````text
판단 요청: {short_title}
판단 유형: {display_label} (`{judgment_type}`)
판단 내용: {decision}
선택지: {options_or_selected_outcome}
결과 영향: {consequence}
불확실성: {uncertainty}
에이전트가 사용자 대신 판단하지 않는 것: {not_deciding}
미룬다면: {deferral_effect|not_applicable}
답변 뒤 다음 안전한 행동: {next_safe_action}
참조: judgment={user_judgment_ref}; task={task_ref}; scope={scope_ref|none}; evidence={evidence_refs|none}; risk={risk_refs|none}
````

## 메모

작은 판단은 한 화면에 들어가야 합니다. 더 자세한 장단점, 추천, 영향을 받는 gate, 근거/위험 참조, 미룰 때의 분석은 active profile이나 판단 복잡도가 요구할 때만 `presentation=full`로 보여줍니다.

민감 동작 승인, 작업 수락, 잔여 위험 수용을 하나의 넓은 승인 질문으로 합치지 않습니다.
