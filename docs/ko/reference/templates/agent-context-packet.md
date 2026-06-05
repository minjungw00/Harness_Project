# 에이전트 맥락 패킷 템플릿

## 사용 시점

다음 안전한 행동에 필요한 현재 맥락을 에이전트가 작고 정확하게 받아야 할 때 `agent-context-packet`을 사용합니다. 이 보기는 사용자용 문장이나 전체 보고서가 아니라 최신성, Core 기반 참조, 활성 차단 사유, 해결되지 않은 사용자 판단, 증거 공백, 닫기 차단 사유, 보장 수준, 하나의 다음 행동에 최적화됩니다.

구현 계층: MVP-1 지원 보기입니다. Structured payload나 prompt 크기의 text로 반환할 수 있습니다. Persisted Markdown projection이 필수는 아닙니다.

경계: 에이전트 맥락 패킷은 행동을 돕는 맥락일 뿐입니다. 쓰기를 승인하거나, gate를 충족하거나, 증거를 만들거나, 민감 동작 승인을 부여하거나, 최종 수락을 기록하거나, 잔여 위험을 수락하거나, 닫기 준비 상태를 만들거나, Task를 닫을 수 없습니다.

## 기준 기록

- Task와 active Change Unit 참조
- 현재 state version과 source ref
- 활성 범위와 하지 않을 일
- 해결되지 않은 사용자 판단
- 활성 차단 사유
- 증거 공백
- 닫기 차단 사유
- active일 때 잔여 위험 요약
- 보장 수준 또는 unavailable capability 상태
- 정확히 하나의 다음 안전한 행동

## 렌더링 섹션

- Task와 Change Unit 참조
- state version과 source ref
- 활성 범위
- 해결되지 않은 사용자 판단
- 차단 사유
- 다음 안전한 행동
- 증거 공백
- 닫기 차단 사유
- 잔여 위험 요약
- 보장 수준

## 전체 템플릿

````text
agent_context_packet:
  display_only: true
  authority: none; authority는 current Core state를 사용
  task_ref: {task_ref}
  change_unit_ref: {change_unit_ref|none}
  state_version: {source_state_version}
  source_refs: {source_refs}
  freshness: {freshness_state}
  active_scope: {scope_summary}
  unresolved_user_judgments: {pending_user_judgment_refs_with_kind_labels|none}
  blockers: {active_blockers|none}
  next_safe_action: {next_safe_action}
  evidence_gaps: {evidence_gaps|none}
  close_blockers: {close_blockers|none}
  residual_risk_summary: {residual_risk_summary_if_active|none}
  guarantee_level: {guarantee_level_or_unavailable}
````

## 메모

에이전트 맥락 패킷은 한 화면 안팎으로 유지합니다. 현재 다음 행동에 필요한 상태만 담습니다. 전체 스키마, 전체 Reference 문서, 전체 historical event log, 등록된 아티팩트 파일 본문, 전체 report body, 전체 template, 관련 없는 template, full design-quality catalog, future catalog material을 기본으로 넣지 않습니다.

다음 행동에 더 자세한 owner section이 필요하면 그 section을 패킷에 넣지 말고 필요할 때 따로 불러옵니다.

`guarantee_level` 필드는 필수 맥락입니다. Core/MCP가 unavailable이면 unavailable/capability condition을 넣고, refresh 전까지 하네스에 의존하는 state, write, evidence, 최종 수락, 잔여 위험, close claim을 unavailable로 다룹니다.
