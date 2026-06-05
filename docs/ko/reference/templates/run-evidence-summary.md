# 실행/증거 요약 템플릿

## 사용 시점

조언, 실행, 확인, 변경 뒤 무엇이 일어났고 현재 주장에 어떤 증거가 생겼는지 최소한으로 보여줘야 할 때 `run-evidence-summary`를 사용합니다.

구현 계층: MVP-1 사용자 작업 루프 보기입니다. 상세 실행 보고서와 상세 증거 목록은 later/full-profile 템플릿입니다.

경계: 이 템플릿은 Run과 증거 참조를 표시할 뿐입니다. 증거 자체, 상세 증거 목록, 검증, QA, 최종 수락, 잔여 위험 수락, 닫기 준비 상태 권한이 아닙니다.

## 기준 기록

- Run 참조와 command/check 요약
- 변경 경로 또는 파일 변경 없음 결과
- 관련 있을 때 소비된 Write Authorization 참조, no-write basis, 또는 attempted invalid authorization context
- 증거 참조, artifact 참조, 가림 처리, 가용성 메모
- 증거가 뒷받침하는 완료 주장, 수용 기준, 닫기 관련 주장
- 증거 공백, 오래된 입력, 아직 해소되지 않은 뒷받침 부족
- 다음 안전한 증거 행동

## 렌더링 섹션

- 실행 또는 행동
- 변경 경로
- 확인
- 증거 참조
- 뒷받침하는 주장
- 공백 또는 오래된 증거
- 가림 처리와 가용성
- 다음 안전한 증거 행동

## 전체 템플릿

````text
실행/증거 요약
표시 전용: ref와 요약일 뿐이며 증거, 검증, QA, 최종 수락, 잔여 위험 수락, 닫기가 아닙니다.

행동: {run_or_action_summary}
변경 경로: {changed_paths|none}
확인: {checks_run_or_reason_not_run}
쓰기 확인: {write_check_summary|no product write}
증거: {evidence_status}. {evidence_summary}
증거 참조: {evidence_refs|none}
아티팩트 참조: {artifact_ref_summary|none}
가림 처리 또는 가용성: {redaction_availability_summary|none}
뒷받침하는 것: {supported_claims_or_criteria|none}
아직 빠졌거나 오래된 것: {evidence_gaps_or_stale_inputs|none}
다음 안전한 증거 행동: {next_evidence_action|none}
출처/최신성: {source_freshness_summary}
````

## 메모

증거 충분성은 양이 아니라 coverage입니다. 현재 뒷받침하는 참조가 없거나 critical artifact ref에 owner relation, integrity metadata, redaction state, availability가 없으면 공백과 현재 증거 상태를 보여줘야 합니다. 긴 artifact 목록이나 report 문장을 증명처럼 취급하면 안 됩니다.

Product-write Run의 제품 쓰기 호환성 기록으로 표시할 수 있는 것은 compatible하게 소비된 Write Authorization뿐입니다. Attempted invalid authorization ref는 violation/audit 또는 validator-finding context로만 보여줄 수 있으며, consumed Write Authorization이나 완료 증거처럼 렌더링하면 안 됩니다.

이 요약은 전체 증거 보고서보다 작게 유지합니다. 사용자의 다음 판단에 필요한 증거 참조와 보이는 공백만 보여주고, 전체 artifact inventory나 원본 artifact body를 펼치지 않습니다.
