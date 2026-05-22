# APR Template

## Used when

Approval request가 commit된 뒤, 민감한 변경 요청과 결정을 읽기 쉽게 보여줘야 할 때 `APR`을 사용합니다.

## Source records

- approval record
- related approval-shaped Decision Packet
- 구현이 유지하는 경우 optional decision request routing/replay record
- Change Unit scope
- sensitive category
- allowed path, tool, command, network target, secret
- baseline, expiry, alternative, decision note

`prepare_write`가 반환한 non-mutating `approval_request_candidate`는 `APR` source가 아닙니다. 표시가 필요하면 candidate display로만 보여줍니다.

## Rendered sections

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

## Full template

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
- approval은 product judgment를 resolve하지 않고, correctness를 prove하지 않고, verification이나 Manual QA를 replace하지 않고, acceptance를 imply하지 않으며, residual risk를 accept하지 않는다.
- approval은 Write Authorization이 아니다. 이후 compatible `prepare_write` retry가 write를 allow해야 implementation 또는 direct `record_run`이 authorization을 consume할 수 있다.
````

## Notes

Approval은 product judgment를 해결하거나, correctness를 증명하거나, evidence를 충족하거나, verification이나 Manual QA를 대체하거나, acceptance를 암시하거나, residual risk를 수용하지 않습니다. Decision request routing record는 decision authority가 아닙니다. Linked compatible Decision Packet을 통하지 않고는 `decision_gate`에 영향을 줄 수 없습니다.
