# 수동 QA 카드 템플릿

## 사용 시점

수동 QA가 필요할 때 기록, 관문, 프로필, 대상, 확인 목록, 기록할 근거, 면제와 위험 표시를 사람이 확인하기 쉬운 간결한 안내 카드로 보여주기 위해 수동 QA 카드를 사용합니다.

경계: 상태 보기 템플릿(projection template)일 뿐이며 하네스 서버/런타임 구현이나 생성된 운영 산출물에 권한을 주지 않습니다. 공통 단계와 상태 보기 규칙은 [템플릿 참조](README.md#사용-시점)를 따릅니다.

구현 계층: 보증 프로필 보고서입니다. 수동 QA 프로필이 명시적으로 활성화된 경우 사용하며 전체 수동 QA 정책 뒷받침 범위는 이후 단계적 범위(staged scope)입니다.

## 기준 기록

- 수동 QA 요구사항과 `qa_gate`
- 존재하는 경우 수동 QA 기록
- 수동 QA 프로필
- 사람 검사자 또는 역할과 요청되는 사람의 판단
- 대상 화면 또는 흐름
- 확인 목록 항목
- 예상 스크린샷, 워크스루 메모, 브라우저 로그, 브라우저 QA 아티팩트, 수동 제공 아티팩트 근거
- QA가 면제되거나 미뤄질 때 면제 사유, 필요한 경우 QA 면제 사용자 판단 참조, 잔여 위험(Residual Risk) 참조
- 검증, 최종 수락, 닫기 영향 요약
- 수동 QA 기록, QA 면제 사용자 판단, 근거 목록(Evidence Manifest), Eval(분리 검증 결과), 최종 수락 맥락, 잔여 위험(Residual Risk), 아티팩트 참조, 가림 상태, 읽기용 보기 최신성(projection freshness)을 위한 간결한 참조

닫기 맥락과 면제 자리표시자는 QA 기록, `qa_gate`, 관련 관문 상태, 사용자 판단 참조, 잔여 위험(Residual Risk) 참조에서 파생한 표시 전용 요약입니다. 면제 경로는 그런 참조를 렌더링하거나 아직 기록이 필요하다고 표시해야 합니다.

## 렌더링 섹션

- 수동 QA 필요 여부
- 기록
- 관문
- 프로필
- 대상
- 확인 목록
- 기록할 근거
- 닫기 맥락
- 면제 기록
- 결과 안내

## 전체 템플릿

````text
수동 QA가 필요합니다.
표시 전용: `qa_gate`와 QA 기록이 기준으로 남습니다.
사람의 확인만 수동 QA입니다. 자동 검사, 스크린샷, 브라우저 로그, 브라우저 QA 아티팩트는 맥락을 뒷받침할 수 있지만 그 자체로 수동 QA가 되지는 않습니다.
브라우저 QA 캡처(Browser QA Capture): 승격되고 지원될 때 유용합니다. 최종 수락이 아니며, 독립적인 Eval(분리 검증 결과) 없이는 분리 검증이 아니고, 필요한 사람의 확인을 대체하지 않습니다.

기록: {manual_qa_record_id|recorded 전까지 none}
관문(Gate): {qa_gate display: not_required|required|pending|passed|failed|waived}
참조: 수동QA={manual_qa_record_id|none}; QA면제판단={qa_waiver_user_judgment_ref|none}; 근거={evidence_manifest_ref|none}; Eval={eval_ref|none}; 최종수락={acceptance_context_ref|none}; 잔여위험={residual_risk_refs|none}; 아티팩트={artifact_refs|none}; 가림={redaction_availability_summary|none}; 최신성={projection_freshness}
프로필: {profile}
요청되는 사람의 확인: {human_inspection_summary}
대상: {screen_or_flow}
확인 목록:
- {checklist_item}

기록할 근거:
- 스크린샷 또는 워크스루 메모
- 승격되고 지원될 때 qa_capture 아티팩트
- 관련 있을 때 브라우저 로그
- 브라우저 캡처가 지원되지 않을 때 수동 제공 아티팩트 또는 사람이 작성한 메모
- 근거를 원본 내용으로 기록할 수 없을 때의 가림/생략/차단 메모

닫기 맥락:
- 자동 검사: {check_refs|none; 수동 QA 결과 아님}
- 브라우저 QA 아티팩트: {artifact_refs|none; 뒷받침 참조만}
- QA 면제 표시: {qa_gate=waived; 면제 참조|none}
- 검증 영향: {verification_impact}
- 최종 수락 영향: {acceptance_impact; 이 카드가 기록하지 않음}
- 잔여 위험(Residual Risk) 또는 후속 작업: {residual_risk_or_follow_up|none}

면제 기록:
- 생략한 수동 QA 대상:
- 면제 전에 표시된 위험:
- 수락하는 위험:
- 후속 작업:
- 관련 참조:
- 닫기 영향:
- 면제 출처: {manual_qa_record_id와 waiver_reason; 사용자 소유 위험이 있으면 qa_waiver_user_judgment_ref}

수동 QA 결과를 기록하거나, 허용된 낮은 위험의 QA 면제 사유를 기록하거나, 사용자 소유 위험이 있으면 QA 면제 사용자 판단을 요청하시겠습니까?
````

## 메모

이 템플릿은 렌더링 결과인 카드 형태일 뿐 기준 QA 상태가 아닙니다. `qa_gate`는 닫기 관련 관문으로 남습니다.

수동 QA는 사람이 확인한 기록입니다. 테스트 통과, 브라우저 스모크, 스크린샷 캡처, 브라우저 QA 캡처 아티팩트(Browser QA Capture artifact), 검증, 사용자의 최종 수락은 닫기 맥락을 뒷받침할 수 있지만, `record_manual_qa`가 수동 QA 결과를 기록했거나 유효한 QA 면제가 면제 사유와 함께 `qa_gate=waived`를 갱신하고, 사용자 소유 위험이 있으면 호환되는 QA 면제 사용자 판단을 포함한 경우가 아니면 수동 QA가 되지 않습니다. 브라우저 QA 캡처(Browser QA Capture)는 owner 문서가 명시적으로 승격하고 증명하기 전까지 로드맵 후보이며, 캡처된 아티팩트는 별도 Eval(분리 검증 결과) 경로가 독립성을 충족하지 않는 한 최종 수락 또는 분리 검증을 기록하지 않습니다. 면제에 닫기 영향이나 위험을 수락하는 판단이 걸려 있는 경우 가벼운 채팅 문장만으로는 충분하지 않습니다.

이 카드는 `pending` QA, `passed` QA, `failed` QA, `waived` QA를 별도 표시 상태로 렌더링해야 합니다. 면제된 QA는 수동 QA 기록 또는 면제 사유, 필요한 경우 QA 면제 사용자 판단, 해당되는 잔여 위험(Residual Risk) 참조, 닫기 영향을 인용하며 통과한 확인이 아닙니다.

결과 안내는 수동 QA 결과 또는 QA 면제 경로만 물어야 합니다. 최종 수락이나 잔여 위험 수락을 같은 답변처럼 요청하면 안 됩니다.

아티팩트가 `secret_omitted` 또는 `blocked`라면 이 카드는 대체 근거 또는 면제 기록을 요청할 수 있지만, 생략된 값 또는 차단된 원본 캡처 내용을 표시하면 안 됩니다. 브라우저 캡처가 해당 접점에서 지원되지 않으면 이 카드는 캡처 부재를 QA 결과로 다루지 말고 사람이 작성한 수동 QA 메모와 수동 제공 아티팩트를 요청해야 합니다.
