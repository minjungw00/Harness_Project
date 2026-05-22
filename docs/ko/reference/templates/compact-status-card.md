# Compact Status Card Template

## 사용 시점

Task 상태, 다음 action, 대기 중인 user decision, 위험, gate, Manual QA, 최신 report를 짧은 현재 상태 표시로 보여줄 때 Compact Status Card를 사용합니다.

## 기준 기록

- 현재 Task 상태와 lifecycle phase
- 대기 중인 Decision Packet summary
- 위험 summary
- evidence, design, QA gate
- 최신 report 참조

## 렌더링 섹션

- task identity
- 상태
- next action
- 사용자 decision
- 위험
- evidence gate
- design gate
- Manual QA
- 최신 report

## 전체 템플릿

````text
TASK-{id} {title}
상태: {mode} / {lifecycle_phase}
다음 action: {next_action}
사용자 decision: {pending_decision_summary|none}
Risk: {risk_summary}
Evidence gate: {evidence_gate}
Design gate: {design_gate}
Manual QA: {qa_gate display: pending|passed|failed|waived|not_required}
최신 report: {latest_report|none}
````

## 메모

이 template은 렌더링 결과인 카드 형태일 뿐 기준 상태가 아닙니다. Gate value는 기준 상태가 계속 담당합니다.
