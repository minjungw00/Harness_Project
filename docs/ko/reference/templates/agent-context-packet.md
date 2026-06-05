# 에이전트 맥락 패킷 템플릿

## 사용 시점

다음 안전한 행동에 필요한 현재 맥락을 에이전트가 작고 정확하게 받아야 할 때 `agent-context-packet`을 사용합니다. 이 보기는 사용자용 문장이나 전체 보고서가 아니라 최신성, 참조, 막힘, 다음 행동에 최적화됩니다.

구현 계층: MVP-1 지원 보기입니다. Structured payload나 prompt 크기의 text로 반환할 수 있습니다. Persisted Markdown projection이 필수는 아닙니다.

경계: 에이전트 맥락 패킷은 행동을 돕는 맥락일 뿐입니다. 쓰기를 허가하거나, gate를 충족하거나, 근거를 만들거나, 민감 동작 승인을 부여하거나, 최종 수락을 기록하거나, 잔여 위험을 수락하거나, 닫기 준비 상태를 만들거나, Task를 닫을 수 없습니다.

## 기준 기록

- 현재 Task, 작업 모양, lifecycle, 다음 안전한 행동
- active scope, 하지 않을 일, Change Unit 참조, 쓰기 준비 막힘
- 대기 중인 사용자 판단과 판단 요청 참조
- 근거 참조, Run 참조, ArtifactRefs, `redaction_state`, 근거 공백
- 닫기 막힘, 잔여 위험 요약, 최종 수락 필요 여부/상태, 관련 owner 참조
- 다음 안전한 행동이 의존할 때 design-quality routed action과 owner ref
- source clock, 최신성 상태, MCP/Core availability, 보장 수준
- 다음 행동에 필요한 owner 문서 또는 owner section pointer

## 렌더링 섹션

- 현재 Task
- active scope
- 대기 중인 판단
- 막힘
- 근거 상태
- 닫기와 잔여 위험 상태
- 다음 안전한 행동
- 최신성과 출처 참조
- 필요할 때 불러올 pointer

## 전체 템플릿

````text
agent_context_packet:
  display_only: true
  authority: none; authority는 current Core state를 사용
  task: {task_id} {task_summary}
  work_shape: {work_shape}
  scope: {scope_summary}
  non_goals: {non_goals|none}
  pending_judgments: {pending_user_judgment_refs|none}
  active_blockers: {active_blockers|none}
  evidence: {evidence_refs_and_gaps}
  design_quality: {design_quality_routed_action|none}
  close: {close_blockers_and_acceptance_state}
  residual_risk: {residual_risk_summary}
  next_safe_action: {next_safe_action}
  guarantee: {guarantee_level_or_unavailable}
  sources:
    state_version: {source_state_version}
    refs: {source_refs}
    freshness: {freshness_state}
    rendered_at: {updated_at}
  pull_if_needed: {owner_section_refs_for_next_action|none}
````

## 메모

에이전트 맥락 패킷은 한 화면 안팎으로 유지합니다. 전체 schema, 전체 Reference 문서, 전체 historical event log, 등록된 아티팩트 파일 본문, 전체 report body, 전체 template, 관련 없는 template, full design-quality catalog, future catalog material을 기본으로 넣지 않습니다.

보장 수준 field는 필수 맥락입니다. Core/MCP가 unavailable이면 unavailable/capability condition을 넣고, refresh 전까지 하네스에 의존하는 state, write, evidence, 최종 수락, 잔여 위험, close claim을 unavailable로 다룹니다.
