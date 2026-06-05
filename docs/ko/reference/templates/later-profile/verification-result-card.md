# 검증 결과 카드(Verification Result Card) 템플릿

## 사용 시점

Eval 결과의 판정(verdict), 보장 영향, 독립성 경계, 검토한 근거, 남은 작업, 사용자 후속 조치를 간결하게 보여줄 때 검증 결과 카드(Verification Result Card)를 사용합니다.

경계: 상태 보기 템플릿(projection template)일 뿐이며 하네스 서버/런타임 구현이나 생성된 운영 산출물에 권한을 주지 않습니다. 공통 단계와 상태 보기 규칙은 [템플릿 참조](README.md#사용-시점)를 따릅니다.

구현 계층: 보증 프로필 보고서입니다. 검증/Eval 표시가 활성화된 경우 사용하며 상세 `EVAL` 상태 보기는 향후/진단(future/diagnostic) 범위입니다.

## 기준 기록

- Eval(분리 검증 결과) 기록
- 보장 영향과 검증 독립성 상태
- 분리 검증 후보(detached-candidate), 자체 확인(self-checked), 분리 검증됨(detached-verified), 수락된 위험으로 면제됨(waived-with-accepted-risk) 표시 문구
- 수동 QA와 최종 수락 영향
- 검토된 Task, 실행(Run), 근거 목록(Evidence Manifest), TDD 트레이스, 변경 차이, 로그, 민감 동작 승인, 설계 참조
- 막힘 또는 재작업
- 사용자 후속 조치
- 닫기 맥락이 렌더링될 때 수동 QA, 최종 수락, 잔여 위험(Residual Risk), 검증 위험 수락 사용자 판단 참조, `verification_gate` 상태
- Eval(분리 검증 결과), 근거 목록(Evidence Manifest), 수동 QA, 최종 수락 맥락, 잔여 위험(Residual Risk), 검증 위험 수락 사용자 판단, 아티팩트 참조, `redaction_state`, 읽기용 보기 최신성(projection freshness)을 위한 간결한 참조

닫기 맥락과 검증 위험 수락 자리표시자는 Eval 기록, 관문 상태, QA/최종 수락 상태, 잔여 위험(Residual Risk) 참조, 검증 위험 수락 사용자 판단 참조에서 파생한 표시 전용 요약입니다. 검증 위험 수락 경로는 그런 참조를 렌더링하거나 아직 기록이 필요하다고 표시해야 합니다.

## 렌더링 섹션

- 검증 완료
- Eval(분리 검증 결과) 식별 정보
- 판정
- 보증 영향
- 검증 독립성
- 수동 QA
- 최종 수락
- 검토한 근거
- 닫기 맥락
- 남은 작업
- 사용자 후속 조치

## 전체 템플릿

````text
검증이 완료되었습니다.
표시 전용: Eval 기록과 관문 상태가 기준으로 남습니다.

{eval_id}
참조: Eval={eval_id}; 근거={evidence_manifest_ref|none}; 수동QA={manual_qa_ref|none}; 최종수락={acceptance_context_ref|none}; 잔여위험={residual_risk_refs|none}; 검증위험수락={verification_risk_acceptance_user_judgment_ref|none}; 아티팩트={artifact_refs|none}; 가림={redaction_availability_summary|none}; 최신성={projection_freshness}
판정(Verdict): {verdict}
보증 영향: {assurance_impact}
사용자 표시 검증 상태: {자체 확인|분리 검증 후보|분리 검증됨|수락된 위험으로 면제됨}
검증 독립성: {verification_independence}
자체 확인과 분리 검증 경계: {self_check_or_detached_boundary}
수동 QA: {manual_qa_impact}
최종 수락: {acceptance_impact; 별도 사용자 판단이며 이 카드가 기록하지 않음}

검토한 근거:
- Task 요약: {task_summary_ref}
- 실행 요약: {run_summary_ref}
- 근거 목록(Evidence Manifest): {evidence_manifest_ref}
- TDD 트레이스: {tdd_trace_ref}
- 변경 차이: {diff_ref}
- 로그: {logs_ref}
- 민감 동작 승인 참조: {approval_refs|none}
- 민감 동작 승인 참조(`approval_refs`)는 나중의 민감 동작 승인(Approval) 담당 프로필이 활성화되지 않은 한 최소 MVP-1에서 `none`입니다.
- 설계 참조: {design_refs}
- 가림 또는 차단 입력: {redaction_availability_summary|none}

닫기 맥락:
- 검증한 내용:
- 검증하지 않은 내용:
- 번들 또는 기준선 최신성: {current|stale|not_applicable}
- 수동 QA: {manual_qa_status_or_needed}
- QA 면제 표시: {qa_gate=waived; 수동 QA 또는 면제 참조|none}
- 최종 수락: {acceptance_status_or_needed}
- 잔여 위험(Residual Risk): {residual_risk_summary|none}
- 검증 위험 수락 표시: {필요한 경우 user judgment ref; waived이면 `verification_gate=waived_by_user`|none}
- 관련 참조: {verification_risk_acceptance_refs|none}
- 닫기 영향: {verification_risk_acceptance_close_impact|none}

남은 작업:
{blockers_or_rework}

사용자 후속 작업:
{user_followup}
````

## 메모

이 템플릿은 렌더링 결과인 카드 형태일 뿐 검증 권한 자체가 아닙니다. Eval 기록과 관문 상태가 기준입니다.

검증(Verification)은 기록된 검토 경계에서 정확성을 확인합니다. 수동 QA를 기록하거나, 사용자 최종 수락을 암시하거나, 잔여 위험을 수락하지 않습니다. 같은 세션의 자체 검토(self-review)는 자체 확인(self-check) 또는 검토 메모로 보여줄 수 있지만 분리 검증으로 렌더링하면 안 됩니다. 검증 위험 수락 표시는 `required`인 경우 사용자 소유 위험 경로를 기록한 사용자 판단, `verification_gate=waived_by_user`, 생략한 확인, 수락하는 검증 위험, 후속 작업, 관련 참조, 닫기 영향을 보여줘야 하며, 분리 검증을 만들거나 보장 수준을 높이지 않습니다.

검증 통과는 최종 수락이 기록됐다는 뜻이 아닙니다. 최종 수락이 `required`이면 이 카드는 최종 수락 상태나 필요한 행동을 보여줄 수 있지만, 최종 수락은 계속 사용자 판단 경로에 남습니다.

검증(Verification)을 표시하는 동안 QA가 면제됐다면 QA 면제는 Eval 판정(verdict)과 보장 영향 줄과 분리해 둡니다. QA 면제 표시는 `qa_gate=waived`, 수동 QA 기록 또는 면제 사유, 필요한 경우 QA 면제 사용자 판단을 인용합니다. 통과한 수동 QA 결과나 분리 검증이 아닙니다.

사용자 표시 문구는 신중하게 사용합니다. "자체 확인(self-checked)"은 구현 경로가 자기 작업을 확인했다는 뜻입니다. "분리 검증 후보(detached candidate)"는 경계가 조건을 충족할 수 있지만 아직 분리 검증 보장을 만들지 않았다는 뜻입니다. "분리 검증됨(detached verified)"은 통과한(passed) Eval이 유효한 독립성과 현재 입력을 갖는다는 뜻입니다. "수락된 위험으로 면제됨(waived with accepted risk)"은 닫기가 수락된 보이는 위험에 의존하며 잔여 위험 수락 닫기 경로를 사용해야 한다는 뜻입니다. 이 표현들은 표시 문구이며 `assurance_level` 값을 추가하지 않습니다.

이 카드는 오래된 평가자 번들(evaluator bundle) 또는 기준선 drift(불일치)를 보장 수준 막힘으로 보여야 합니다. 오래된 번들(bundle)은 검토된 아티팩트로 남을 수 있지만 대체 근거 또는 호환되는 재검증이 기록되지 않았다면 분리 검증됨(detached verified)으로 표시하면 안 됩니다.

이 카드는 생략되었거나 차단된 원본 바이트(bytes)를 검토한 것처럼 암시하면 안 됩니다. `secret_omitted`는 보이는 비밀 정보가 아닌 주장만 뒷받침할 수 있고, `blocked`는 문서화된 해소가 없는 한 사용할 수 없는 입력입니다.
