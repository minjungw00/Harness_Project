# TDD-TRACE 템플릿

## 사용 시점

작업 조각(Change Unit)에서 TDD가 필요하거나 선택 또는 기록된 상태이고 RED, GREEN, 리팩터/확인, 면제, 증거 참조를 읽기 쉬운 상태 보기(projection)로 볼 때 `TDD-TRACE`를 사용합니다.

경계: 상태 보기 템플릿(projection template)일 뿐이며 하네스 서버/런타임 구현이나 생성된 운영 산출물에 권한을 주지 않습니다. 공통 단계와 상태 보기 규칙은 [템플릿 참조](README.md#사용-시점)를 따릅니다.

구현 계층: 향후/진단용 상태 보기(projection)입니다. TDD 트레이스 출력은 나중 정책 또는 진단 프로필용이며 첫 구현 범위를 키우면 안 됩니다.

## 기준 기록

- `tdd_traces`
- 선택된 `feedback_loops`
- Task와 작업 조각(Change Unit) 참조
- RED, GREEN, 리팩터/확인 아티팩트 참조
- 증거 목록(Evidence Manifest) 뒷받침 범위 참조
- 면제 또는 비 TDD 증거 참조
- 해당되는 경우 증거 목록(Evidence Manifest), 사용자 판단(User Judgment), 작업 조각(Change Unit), 잔여 위험(Residual Risk), 수동 QA, Eval(분리 검증 결과), 닫기 차단 사유, 후속 조치 참조를 통한 발견 사항 라우팅
- `tdd_trace` 관련 design-quality 검증기 결과
- 읽기용 보기 최신성(projection freshness) 입력

## 렌더링 섹션

- 식별 정보
- 실패 단계(Red)
- 통과 단계(Green)
- 정리 단계(Refactor)
- 비 TDD(Non-TDD) 증거
- 증거 참조
- 발견 사항 라우팅

## 전체 템플릿

````md
---
doc_type: tdd_trace
tdd_trace_id: TDD-0001
task_id: TASK-0001
change_unit_id: CU-01
status: recorded
source_state_version: 43
updated_at: 2026-05-06T09:40:00+09:00
---

# TDD-0001 트레이스 제목

> 상태 보기(Projection): `source_state_version`와 `updated_at` 기준으로 렌더링되며 TDD 기록과 참조를 표시합니다. 계획 문구는 기록된 아티팩트 또는 결과 참조가 뒷받침하기 전까지 RED 증거가 아닙니다.

## 식별 정보
- task_id:
- change_unit_id:
- 트레이스 상태: required | recorded | waived | not_required
- 요구/출처:
- 피드백 루프 참조:
- 증거 목록(Evidence Manifest) 뒷받침 범위 참조:

## 실패 단계(Red)
- 대상 / 계획:
- 실패 테스트 참조:
- 명령:
- 결과: failed_as_expected | failed_unexpectedly | missing
- 로그 참조:
- 테스트 외(non-test) 구현 전 기록 여부: yes | no | waived
- 대상 / 계획은 증거 목록(Evidence Manifest) 뒷받침 범위로 계산됨: no

## 통과 단계(Green)
- 명령:
- 결과: passed | failed | missing
- 로그 참조:

## 정리 단계(Refactor)
- 수행 여부: yes | no
- 메모:
- 검증 명령:
- 로그 참조:

## 비 TDD(Non-TDD) 증거
- 이유:
- 피드백 루프 참조:
- 대체 피드백 루프:
- 테스트 외 구현 전 면제 기록 여부: yes | no

## 증거 참조
- 테스트:
- RED 로그:
- GREEN 로그:
- 리팩터/확인 로그:
- 증거 목록(Evidence Manifest):
- 변경 차이:

## 발견 사항 라우팅
- 증거 공백 또는 뒷받침:
- 사용자 판단 후보 또는 참조:
- 작업 조각(Change Unit) 업데이트 또는 후속 조치:
- 잔여 위험 후보 또는 참조:
- 수동 QA 또는 Eval(분리 검증 결과) 참조:
- 닫기 차단 사유:
````

## 메모

이 템플릿은 렌더링 결과일 뿐 기준 상태가 아닙니다. RED 대상 또는 계획은 계획 맥락이며, 실제 RED 증거는 여전히 기록된 아티팩트 또는 결과 참조에서 나와야 합니다.

TDD가 권고(advisory)일 뿐 `required` 또는 `selected`가 아니라면 TDD 면제는 필요하지 않습니다. `required`, `selected`, `recorded`, `waived` TDD는 owner 기록에서만 렌더링하고, 발견 사항은 템플릿 전용 상태를 추가하지 말고 기존 owner 참조로 라우팅합니다.
