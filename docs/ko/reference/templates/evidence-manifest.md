# EVIDENCE-MANIFEST Template

## Used when

acceptance criteria와 completion condition이 어떤 supporting evidence로 뒷받침되는지 보여줘야 할 때 `EVIDENCE-MANIFEST`를 사용합니다.

## Source records

- evidence manifest record
- acceptance criteria
- changed file coverage
- design-quality coverage
- approval ref
- artifact ref
- related Run, Eval, Feedback Loop, Manual QA, TDD trace ref

## Rendered sections

- Identity
- Summary
- Acceptance Criteria Coverage
- Changed File Coverage
- Design Quality Coverage
- Approval Refs
- Evidence Refs
- Stale If

## Full template

````md
---
doc_type: evidence_manifest
evidence_manifest_id: EM-0001
task_id: TASK-0001
change_unit_id: CU-01
status: partial
source_state_version: 44
updated_at: 2026-05-06T09:50:00+09:00
---

# EM-0001 Evidence Manifest

## Identity
- task_id:
- change_unit_id:
- baseline_ref:
- run_summary:
- latest_eval:

## Summary
- evidence state:
- unsupported criteria:
- stale conditions:
- next evidence action:

## Acceptance Criteria Coverage
| AC ID | Statement | Status | Supporting Evidence | Notes |
|---|---|---|---|---|
| AC-01 | | supported | test:, tdd:, log:, diff: | |
| AC-02 | | unsupported | | |

## Changed File Coverage
| Path | Covered Criteria | Evidence Refs |
|---|---|---|
| `src/...` | AC-01 | DIFF-0001, LOG-0001 |

## Design Quality Coverage
| Item | Status | Evidence Refs | Notes |
|---|---|---|---|
| vertical_slice_shape | passed | CU-01 | |
| decision_quality_check | passed | DEC-0001 | |
| autonomy_boundary_check | passed | CU-01 | |
| feedback_loop_check | passed | FBL-0001, TDD-0001, LOG-0001 | |
| tdd_trace_required | passed | TDD-0001, RED-LOG-0001, GREEN-LOG-0001 | RED, GREEN, relevant refactor/check coverage가 acceptance criteria 및 changed files로 link된다. |
| module_interface_review | passed | module_map_item: MMI-0001, interface_contract: IFACE-0001, DEC-0001 | |
| codebase_stewardship_check | passed | domain_term: TERM-0001, module_map_item: MMI-0001, interface_contract: IFACE-0001, feedback_loop: FBL-0001 | |
| residual_risk_visibility_check | pending | RR-0001 | |
| manual_qa_required | pending | qa_gate; no satisfying Manual QA record yet | |

## Approval Refs
- APR-0001:

## Evidence Refs
- run summary:
- feedback loop:
- TDD trace:
- TDD RED target / plan:
- TDD red:
- TDD green:
- TDD refactor/check:
- Manual QA:
- diff:
- logs:
- bundle:
- checkpoint:
- tests:
- build:

## Stale If
- baseline head changes
- changed files are modified after eval
- approval scope expires
- relevant config changes
- domain term records change
- interface contract records change
````

## Notes

Evidence가 required인 곳에서 close는 report text alone이 아니라 canonical `evidence_gate`를 기준으로 결정됩니다.
