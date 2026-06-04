# INTERFACE-CONTRACT 템플릿

## 사용 시점

모듈 인터페이스, 호출자 영향, 호환성 위험, 테스트 경계를 읽기 쉬운 상태 보기(projection)로 볼 때 `INTERFACE-CONTRACT`를 사용합니다.

경계: 상태 보기 템플릿(projection template)일 뿐이며 하네스 서버/런타임 구현이나 생성된 운영 산출물에 권한을 주지 않습니다. 공통 단계와 상태 보기 규칙은 [템플릿 참조](README.md#사용-시점)를 따릅니다.

구현 계층: 향후/진단용 상태 보기(projection)입니다. 인터페이스 계약(Interface Contract) 출력은 담당 프로필이 명시적으로 승격하지 않는 한 나중 참조 보기입니다.

## 기준 기록

- `interface_contracts`
- 영향받는 호출자 참조
- 관련 모듈 맵 항목
- 관련 사용자 판단 또는 설계 참조
- 경계, 통합, 계약 테스트 참조
- `deep_module_interface` 관련 설계 품질 검증기 결과
- 표시되는 경우 인터페이스 또는 호환성 참조에 영향을 주는 라우팅된 스튜어드십 발견 사항
- 읽기용 보기 최신성(projection freshness) 입력

## 렌더링 섹션

- 식별 정보
- 계약
- 영향받는 호출자
- 테스트 경계
- 검토
- 참조
- 사용자 메모와 제안

## 전체 템플릿

````md
---
doc_type: interface_contract
interface_contract_id: IFACE-0001
task_id: TASK-0001
review_status: pending
projection_version: 1
source_state_version: 42
updated_at: 2026-05-06T09:30:15+09:00
---

# IFACE-0001 인터페이스 제목

> 상태 보기(Projection): `interface_contracts`와 관련 참조를 `source_state_version` / `updated_at` 기준으로 렌더링한 보기입니다. 관리 섹션은 생성된 표시 영역이며, 조정(reconcile) 입력은 `사용자 메모와 제안`에 적습니다.

<!-- HARNESS:BEGIN managed -->
## 식별 정보
- 모듈:
- 인터페이스:
- 변경 유형: new | changed | deprecated | removed

## 계약
- 입력:
- 출력:
- 오류:
- 부수 효과:
- 호환성 영향: none | minor | breaking

## 영향받는 호출자
- 호출자:

## 테스트 경계
- 경계 테스트:
- 통합 테스트:
- 계약 테스트:

## 검토
- review_status: pending | reviewed
- 검토한 사람:
- 결정:
- 면제 사유:

## 참조
- TASK:
- DESIGN:
- DEC:
- EVIDENCE-MANIFEST:
<!-- HARNESS:END managed -->

## 사용자 메모와 제안
<!-- 사람이 편집 가능: 여기의 인터페이스 제안은 조정(reconcile)/Core를 통해 수락(accepted)되기 전에는 기준 Interface Contract 기록이 아닙니다. -->
-
````

## 메모

이 템플릿은 렌더링 결과일 뿐 기준 상태가 아닙니다. 기준 인터페이스 참조는 `StateRecordRef.record_kind=interface_contract`를 사용합니다. `검토` 섹션은 인터페이스, 검증기, 판단 참조 위의 상태 보기 표시이며 민감 동작 승인(Approval), 근거, QA, 검증, 작업 수락, 잔여 위험 수용, 닫기, 쓰기 허가 기록(Write Authorization)이 아닙니다.

공개 인터페이스 변경, 호환성 위험, 호환성 깨짐 변경(breaking change), 호출자 영향 선택에 사용자 소유 제품 판단이나 기술 판단이 필요하면 기존 설계 품질 및 사용자 판단 경로로 라우팅합니다. 계약을 여기에 렌더링하는 것만으로 `design_gate`, `decision_gate`, 닫기 영향이 해소되지는 않습니다.
