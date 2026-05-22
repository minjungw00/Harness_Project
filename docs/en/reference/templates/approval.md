# APR Template

## Used when

Use `APR` after an approval request has been committed and Harness needs a readable approval request and decision record for sensitive change.

## Source records

- approval record
- related approval-shaped Decision Packet
- optional decision request routing/replay record, if implementation keeps one
- Change Unit scope
- sensitive categories
- allowed paths, tools, commands, network targets, and secrets
- baseline, expiry, alternatives, and decision note

A non-mutating `approval_request_candidate` returned by `prepare_write` is not an `APR` source and must be displayed, if at all, as candidate display.

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
- approval does not resolve product judgment, prove correctness, replace verification, replace Manual QA, imply acceptance, or accept residual risk.
- approval is not Write Authorization; a later compatible `prepare_write` retry must allow the write before implementation or direct `record_run` can consume authorization.
````

## Notes

Approval does not resolve product judgment, prove correctness, satisfy evidence, replace verification, replace Manual QA, imply acceptance, or accept residual risk. Decision request routing records are not decision authority and cannot affect `decision_gate` except through a linked compatible Decision Packet.
