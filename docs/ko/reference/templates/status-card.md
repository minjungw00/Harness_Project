# 상태 카드 템플릿

## 사용 시점

MVP-1에서 사용자가 현재 상태를 짧게 읽어야 할 때 `status-card`를 사용합니다. 상태 카드는 지금 무엇을 하는지, 무엇이 범위 안인지, 사용자가 무엇을 판단해야 하는지, 어떤 증거가 있거나 빠졌는지, 닫기를 무엇이 막는지, 다음 안전한 행동이 무엇인지를 보여줍니다.

구현 계층: MVP-1 사용자 작업 루프 보기입니다. 내부 엔지니어링 점검은 이 카드 대신 plain structured status/차단 사유 output을 반환해도 됩니다.

경계: 이 템플릿은 렌더링된 표시일 뿐입니다. Core 상태, 증거, 민감 동작 승인, 최종 수락, 잔여 위험 수락, Write Authorization, 닫기 준비 상태가 아닙니다. 오래된 대화가 아니라 현재 Core 소유 상태와 참조에서 렌더링해야 합니다.

## 기준 기록

- 현재 Task 요약, 작업 모양, 다음 안전한 행동
- 사용자가 이해하는 데 필요한 현재 범위, 하지 않을 일, active Change Unit 요약
- 사용자에게 읽히는 라벨로 렌더링한 대기 중인 판단
- 진행 또는 닫기가 보류된 평이한 이유와 활성 차단 사유
- 현재 증거 요약, 뒷받침 참조, 가림 처리 또는 가용성 메모, 증거 공백
- 관련 있을 때 닫기 차단 사유, 최종 수락 필요 여부, 잔여 위험 표시, 잔여 위험 수락 상태
- 보이는 다음 행동을 바꿀 때만 설계 품질 routed action
- 보장 수준 또는 unavailable capability 상태
- 짧은 출처 참조, 렌더링 시각, 최신성 상태

## 렌더링 섹션

- 작업
- 범위
- 판단
- 차단 사유
- 증거
- 확인
- 닫기
- 다음 안전한 행동
- 출처와 최신성

## 전체 템플릿

````text
{task_id} {title}
표시 전용: Core 상태와 ref에서 파생된 보기이며 Core 상태나 쓰기 승인 기록이 아닙니다.

작업: {work_shape}. {current_task_summary}
범위: {scope_summary}
범위 밖: {non_goals|none}
차단 사유: {active_blocked_reason|none}
사용자가 결정할 것: {pending_user_judgments_with_localized_labels|none}
증거: {evidence_status}. {known_evidence_summary|none}
증거 공백: {evidence_gaps|none}
확인: {check_summary|none}
닫기: {close_readiness_summary}; 차단 사유={close_blockers|none}
설계 품질 조치: {design_quality_routed_action|none}
남은 위험: {residual_risk_visibility|none}
다음 안전한 행동: {next_safe_action}
보장 수준: {guarantee_level_or_unavailable}; {guarantee_note}
출처/최신성: {source_freshness_summary}
````

## 메모

하네스 내부를 모르는 사용자도 읽을 수 있게 유지합니다. Schema, DDL, event log, 전체 artifact, 전체 보고서 본문, 전체 template, future catalog, 상세 증거 목록, 상세 평가 본문, later assurance record를 쏟아내지 않습니다.

기준 기록이 없으면 상태를 만들어내지 말고 `none`, `unknown`, `not_required`, 또는 명시적인 차단 사유로 렌더링합니다.

보장 수준 줄은 항상 렌더링합니다. MVP-1 기본 동작에서는 실제 한계가 협력형 보류이면 그렇게 적고, 사후 보고라면 그 한계를 note에 적어야 합니다. Core/MCP가 unavailable이면 stale하거나 추측한 guarantee 대신 unavailable condition을 렌더링합니다.

설계 품질 내용은 한 줄에 맞춥니다. 현재 routed action과, 차단일 때는 하나의 다음 행동만 보여줍니다. MVP-1 상태 카드에는 full domain-language, module/interface, TDD, stewardship, feedback-loop, QA, assurance catalog를 나열하지 않습니다.

에이전트 전용 참조와 행동 경계 세부사항은 [agent-context-packet](agent-context-packet.md)에 둡니다. 사용자가 판단하거나 차단 사유를 이해하거나 출처 최신성을 살피는 데 도움이 될 때만 상태 카드에 ref를 넣습니다.
