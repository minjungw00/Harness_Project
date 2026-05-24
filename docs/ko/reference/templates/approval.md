# APR Template

## 사용 시점

Approval request가 기록된 뒤, 민감한 변경 요청과 결정을 읽기 쉽게 보여줘야 할 때 `APR`을 사용합니다.

## 기준 기록

- approval 기록
- related approval-shaped Decision Packet
- 구현이 유지하는 경우 선택적 decision request routing/replay 기록
- Change Unit scope
- sensitive category
- 허용된 path, tool, command, network target, secret
- baseline, expiry, alternative, decision note

`prepare_write`가 반환한 non-mutating `approval_request_candidate`는 `APR` source가 아닙니다. 표시가 필요하면 candidate display로만 보여줍니다.

## 렌더링 섹션

- Request Summary
- Related Decision Packet
- Requested Scope
- Why This Is Needed
- Impact
- Risks
- Alternatives
- Recommendation
- Decision
- Boundary

## 전체 템플릿

````md
---
doc_type: approval
approval_id: APR-0001
task_id: TASK-0001
category: dependency_change
status: pending
source_state_version: 42
updated_at: 2026-05-06T09:30:15+09:00
---

# APR-0001 Approval Request

## Request Summary
- proposed action:

## Related Decision Packet
- approval-shaped Decision Packet:
- separate product-judgment Decision Packet, if required:
- decision gate impact:
- approval gate impact:

## Requested Scope
- sensitive categories:
- allowed paths:
- allowed tools:
- allowed commands:
- allowed network targets:
- required secrets:
- baseline ref:
- expected diff envelope:
- expires on scope drift:

## Why This Is Needed
- purpose:
- relation to current task:

## Impact
- code/docs:
- user/operations:
- security/privacy:
- cost/deployment:
- domain language:
- module/interface:

## Risks
- main risk:
- failure impact:
- scope drift condition:

## Alternatives
### Alternative A
- description:
- benefit:
- cost/risk:

### Alternative B
- description:
- benefit:
- cost/risk:

## Recommendation
- recommendation:
- reason:

## Decision
- status: pending | granted | denied | expired
- decision note:
- decided by:
- decided at:

## Boundary
- approval은 사용자 소유의 제품 판단이나 중요한 기술 판단을 resolve하지 않고, correctness를 prove하지 않고, verification이나 Manual QA를 replace하지 않고, acceptance를 imply하지 않으며, residual risk를 accept하지 않는다.
- approval은 Write Authorization이 아니다. 이후 compatible `prepare_write` retry가 write를 allow해야 implementation 또는 direct `record_run`이 authorization을 consume할 수 있다.
````

## 메모

Approval은 사용자 소유의 제품 판단이나 중요한 기술 판단을 해결하지 않고, 정확성을 증명하지 않으며, 근거를 충족하거나 verification 또는 Manual QA를 대체하지 않습니다. 또한 acceptance 암시나 잔여 위험 수용도 하지 않습니다. Decision request routing 기록만으로는 decision 권한이 생기지 않습니다. Linked compatible Decision Packet을 통하지 않고는 `decision_gate`에 영향을 줄 수 없습니다.
