# INTERFACE-CONTRACT Template

## 사용 시점

모듈 인터페이스, 호출자 영향, 호환성 위험, 테스트 경계를 읽기 쉬운 projection으로 볼 때 `INTERFACE-CONTRACT`를 사용합니다.

## 기준 기록

- `interface_contracts`
- impacted caller 참조
- 관련 module map item
- 관련 Decision Packet 또는 design 참조
- boundary, integration, contract test 참조
- `deep_module_interface` 관련 design-quality validator 결과
- 읽기용 보기 최신성(projection freshness) 입력

## 렌더링 섹션

- Identity
- Contract
- Callers Impacted
- Test Boundary
- Review
- References
- User Notes and Proposals

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

# IFACE-0001 Interface Title

> Projection 보기: `interface_contracts`와 관련 ref를 `source_state_version` / `updated_at` 기준으로 렌더링한 보기입니다. Managed section은 생성된 표시 영역이며, reconcile 입력은 `User Notes and Proposals`에 적습니다.

<!-- HARNESS:BEGIN managed -->
## Identity
- module:
- interface:
- change type: new | changed | deprecated | removed

## Contract
- inputs:
- outputs:
- errors:
- side effects:
- compatibility impact: none | minor | breaking

## Callers Impacted
- caller:

## Test Boundary
- boundary tests:
- integration tests:
- contract tests:

## Review
- review_status: pending | reviewed
- reviewed by:
- decision:
- waiver reason:

## References
- TASK:
- DESIGN:
- DEC:
- EVIDENCE-MANIFEST:
<!-- HARNESS:END managed -->

## User Notes and Proposals
<!-- Human-editable: 여기의 interface proposal은 reconcile/Core를 통해 accepted되기 전에는 기준 Interface Contract record가 아닙니다. -->
-
````

## 메모

이 template은 렌더링 결과일 뿐 기준 상태가 아닙니다. 기준 interface 참조는 `StateRecordRef.record_kind=interface_contract`를 사용합니다.
