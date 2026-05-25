# Compact Status Card Template

## 사용 시점

상시 Harness context envelope를 짧은 현재 상태 표시로 보여줄 때 Compact Status Card를 사용합니다. 여기에는 Task 상태, next safe action, active Change Unit, 대기 중인 user decision, 쓰기 권한, 보장 수준, gate, Manual QA, residual risk, projection freshness, latest refs가 포함됩니다.

## 기준 기록

- 현재 Task 상태와 lifecycle phase
- active Change Unit summary
- 대기 중인 Decision Packet summary
- Write Authority summary
- 연결 profile의 보장 수준
- 위험 summary
- scope, approval, decision, design, evidence, verification, QA, acceptance gate
- close blocker와 Manual QA summary
- API error, close blocker, gate, ref에서 파생한 가장 먼저 해소할 막힘, 추가 막힘, 가장 작은 해소 방법 표시 summary
- projection freshness와 `source_state_version`
- 최신 report, Evidence Manifest, Run, Eval, Manual QA, ArtifactRef refs

이 card의 summary placeholder는 위 기록에서 파생한 표시 binding입니다. Decision, close-blocker, residual-risk, freshness summary는 ref 또는 명시적인 absence를 보여줘야 하며, judgment context나 권한을 만들지 않습니다.

## 렌더링 섹션

- task identity
- mode와 lifecycle phase
- next safe action
- 가장 먼저 해소할 막힘과 가장 작은 해소 방법
- 추가 막힘
- active Change Unit
- 사용자 decision
- 쓰기 권한
- 보장 수준
- gate summary
- Manual QA
- residual risk
- projection freshness
- latest refs

## 전체 템플릿

````text
TASK-{id} {title}
표시 전용: 현재 상태를 보여주는 읽기용 보기이며 기준 상태나 쓰기 권한이 아닙니다.
모드: {mode} / {lifecycle_phase}
다음 safe action: {next_safe_action}
가장 먼저 해소할 막힘: {primary_blocker_label|none}; 가장 작은 해소 방법: {smallest_unblocker|none}
추가로 막는 것: {secondary_blockers_summary|none}
Change Unit: {active_change_unit_summary|none}
막는 Decision: {blocking_decision_summary|none}
쓰기 권한: {write_authority_status}
보장 수준: {guarantee_level}; {guard_or_detection_summary}
권한 gate: scope={scope_gate}; approval={approval_gate}; decision={decision_gate}
품질 gate: design={design_gate}; evidence={evidence_gate}; verification={verification_gate}; QA={qa_gate}; acceptance={acceptance_gate}
Manual QA: {manual_qa_summary|not_required}
Close 막힘: {close_blockers|none}
남은 위험: {residual_risk_summary|none}
Projection freshness (읽기용 보기): {current|stale|failed|unknown}; source_state_version={source_state_version|unknown}; {refresh_or_reconcile_needed|none}
최신 refs: report={latest_report_ref|none}; evidence={evidence_manifest_ref|none}; run/eval/QA={latest_check_refs|none}
````

## 메모

이 template은 렌더링 결과인 카드 형태일 뿐 기준 상태가 아닙니다. Gate value는 기준 상태가 계속 담당하며, projection freshness는 읽기용 보기의 최신성만 뜻합니다. Task result, state freshness, evidence freshness, Approval, acceptance, 쓰기 권한이 아닙니다.

가장 먼저 해소할 막힘은 API response가 제공하는 primary `ToolError`에서 가져오거나, failed `harness.close_task` response를 렌더링할 때는 첫 close blocker에서 가져와야 합니다. 추가 막힘은 compact하게 묶고, next action, close 준비 상태, user judgment를 바꿀 때만 보여줍니다. 이 라벨들은 표시 문구일 뿐 새 schema value나 `ErrorCode`가 아닙니다.

이것은 judgment-context가 아닙니다. 사용자 판단이 필요하면 options, recommendation, uncertainty, 미룰 때의 영향, relevant refs가 있는 decision prompt를 별도로 렌더링합니다.

큰 기록은 refs-first로 둡니다. Evidence, Run, Eval, Manual QA, artifact, log, screenshot, diff, large trace는 default로 본문에 포함하지 않습니다.
