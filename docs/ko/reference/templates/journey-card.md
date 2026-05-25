# JOURNEY-CARD Template

## 사용 시점

작업의 현재 위치, 범위와 범위 밖, 다음 움직임을 막는 것, 대기 중인 판단, Autonomy Boundary, Write Authority Summary, 근거와 확인, 남은 위험, close context, gate, projection 최신성을 현재 위치 카드로 보여줄 때 `JOURNEY-CARD`를 사용합니다.

## 기준 기록

- 현재 Task 상태와 gate
- scope와 out-of-bounds summary
- active Change Unit
- Autonomy Boundary summary
- Write Authorization, approval, baseline, 보장 수준 참조
- active Decision Packet 참조
- 가장 먼저 해소할 막힘, 추가 막힘, 가장 작은 해소 방법 표시 summary
- blocker owner 표시 summary
- evidence coverage, verification, Manual QA summary
- residual-risk summary와 참조
- 수락과 close-reason summary
- 최신 evidence, Eval, Manual QA, 보고서 참조
- projection 최신성 입력

Judgment, write-authority, close-impact, residual-risk, freshness placeholder는 위 기록에서 파생한 표시 binding입니다. 실제 사용자 decision이 필요하면 이 card를 judgment-context source로 취급하지 말고 Decision Packet 또는 decision prompt를 렌더링합니다.

## 렌더링 섹션

- 현재 위치와 next action
- scope와 out of bounds
- 현재 막는 것
- 판단 context
- Autonomy boundary
- Write Authority Summary
- 근거와 확인
- 잔여 위험
- close context
- Gates
- projection 최신성

## 전체 템플릿

````text
TASK-{id} {title}
표시 전용: 현재 위치를 보여주는 읽기용 보기이며 기준 상태나 쓰기 권한이 아닙니다.
현재 위치: {mode} / {lifecycle_phase} / {current_position}
범위: {scope_summary|none}
범위 밖: {out_of_bounds_summary|none}
다음 action: {next_action}

현재 막는 것:
- 가장 먼저 해소할 막힘: {primary_blocker_label|none}
- 소유자: {primary_blocker_owner_label|none}
- 가장 작은 해소 방법: {smallest_unblocker|none}
- 추가로 막는 것: {secondary_blockers_summary|none}

판단 context:
- 대기 중인 Decision: {decision_packet_ref|none}
- 사용자가 판단할 것: {what_user_is_deciding|none}
- agent가 판단해도 되는 것: {what_agent_may_decide_without_user}

Autonomy Boundary:
- profile: {autonomy_profile}
- agent가 할 수 있는 일: {agent_may_do}
- 필요한 사용자 판단: {user_judgment_required}
- AFK stop conditions: {afk_stop_conditions}

Write Authority Summary:
- active Change Unit: {active_change_unit_ref|none}
- Write Authorization: {write_authorization_ref|none}
- allowed paths: {allowed_paths}
- allowed tools: {allowed_tools}
- allowed commands: {allowed_commands}
- allowed network targets: {allowed_network_targets}
- secret scope: {secret_scope}
- sensitive categories: {sensitive_categories}
- approval status: {approval_status}
- baseline: {baseline_ref|none}
- 보장 수준: {guarantee_display}
- note: Autonomy Boundary는 판단 재량이지 쓰기 권한이 아니다.

근거와 확인:
- 행동: {next_evidence_action}
- 필요한 이유: {evidence_needed_for}
- 최신 evidence: {latest_evidence_ref|none}
- 검증: {verification_summary|none}
- Manual QA: {manual_qa_summary|not_required}
- 생략/차단된 근거 영향: {redaction_availability_summary|none}

남은 위험:
- 상태: {residual_risk_status}
- 닫기 영향: {residual_risk_close_impact}
- accepted residual-risk record refs: {accepted_residual_risk_record_refs|none}

Close context:
- blockers: {close_blockers|none}
- 수락: {acceptance_summary|not_required}
- close reason: {close_reason|none}

Gates:
- scope: {scope_gate}
- decision: {decision_gate}
- approval: {approval_gate}
- evidence: {evidence_gate}
- verification: {verification_gate}
- Manual QA: {qa_gate display: pending|passed|failed|waived|not_required}
- acceptance: {acceptance_gate}

Projection freshness: {projection_freshness}; source_state_version={source_state_version|unknown} (읽기용 보기의 최신성, Task result 아님)
````

## 메모

이 template은 렌더링 결과일 뿐 기준 상태가 아닙니다. Persisted `JOURNEY-CARD` Markdown은 선택 사항입니다. `status`, `next`, 중요한 resume flow에서 보여주는 현재 위치 Journey Card output도 읽기/표시용 접점입니다.

Journey Card의 Close context는 compact status/resume 표시입니다. `TASK`는 진행 중이거나 최근 닫힌 `work` Task의 continuity Close Summary를 담당하고, `DIRECT-RESULT`는 direct work의 가벼운 close impact summary를 담당합니다. 이 표시들은 close state, 수락, QA, verification, 남은 위험을 받아들이는 판단, Write Authorization을 만들지 않습니다.

Blocker 줄은 API와 state record를 사용자에게 보이는 status로 바꿔 보여줍니다. 가장 먼저 해소할 막힘은 next action이 먼저 해소해야 하는 blocker여야 하며, 소유자 라벨은 다음 움직임이 사용자 소유인지, 에이전트가 해소 가능한지, 접점/시스템 소유인지 분명히 해야 합니다. 가장 먼저 해소할 막힘이 없으면 소유자는 `none`으로 렌더링하거나 생략할 수 있습니다. 추가 막힘은 후속 경로에 영향을 줄 때만 계속 보여줍니다. Raw `ErrorCode` 값만으로 설명을 끝내면 안 됩니다.

Latest 또는 next evidence에 `secret_omitted`나 `blocked` artifact ref가 포함되면 이 card는 사용 가능성 영향만 표시해야 합니다. 생략된 값 또는 차단된 원본 payload 내용을 포함하면 안 됩니다.
