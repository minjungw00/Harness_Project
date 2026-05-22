# RUN-SUMMARY Template

## 사용 시점

`record_run`으로 execution Run이 기록된 뒤, 무엇을 실행했고 무엇이 바뀌었는지, check 또는 validator가 무엇을 보고했는지, raw evidence가 어떤 artifact에 남았는지 요약해야 할 때 `RUN-SUMMARY`를 사용합니다.

## 기준 기록

- run 기록
- actor/surface identity
- baseline
- Change Unit
- 있는 경우 consumed Write Authorization 참조
- changed path
- command result
- validator 결과
- 기록된 Review Stage finding이 있는 경우
- artifact 참조
- evidence update와 follow-up

## 렌더링 섹션

- Run Identity
- Scope
- Changed Files
- Commands And Checks
- Checks And Validator Outcomes
- Review Stages
- TDD Trace Summary
- Key Changes
- Issues And Follow-Ups
- Journey Spine Updates
- Evidence Refs

## 전체 템플릿

````md
---
doc_type: run_summary
run_id: RUN-20260506-093015-LEAD-01
task_id: TASK-0001
change_unit_id: CU-01
profile: lead
kind: implementation
surface_id: reference
source_state_version: 43
updated_at: 2026-05-06T09:45:10+09:00
---

# RUN-SUMMARY

## Run Identity
- run_id:
- actor kind:
- surface:
- baseline_ref:
- state_version:
- status:

## Scope
- task_id:
- change_unit_id:
- slice type:
- write authorization:
- allowed paths:
- allowed tools:
- allowed commands:
- allowed network targets:
- secret scope:
- sensitive categories:
- approval refs:

## Changed Files
- `path/to/file`

## Commands And Checks
```bash
npm test -- --runInBand
```

## Checks And Validator Outcomes
### Core Checks And Command Checks
- changed_paths:
- approval_scope:
- lint:
- test:
- build:
- evidence_sufficiency:

### ValidatorResult IDs
- vertical_slice_shape:
- shared_design_alignment:
- decision_quality_check:
- autonomy_boundary_check:
- feedback_loop_check:
- tdd_trace_required:
- domain_language_consistency:
- module_interface_review:
- codebase_stewardship_check:
- residual_risk_visibility_check:
- manual_qa_required:

## Review Stages
- note: run-local review display only; same-session review는 `detached_verified` assurance를 만들 수 없다.

### Spec Compliance Review
- acceptance criteria coverage:
- Change Unit completion conditions:
- scope / Write Authority compatibility:
- Decision Packet compatibility:
- evidence coverage:
- residual-risk visibility:
- outcome refs:

### Code Quality / Stewardship Review
- domain language:
- module / interface boundary:
- vertical slice shape:
- feedback loop / TDD:
- codebase stewardship:
- context hygiene:
- follow-up risk:
- outcome refs:

## TDD Trace Summary
- required:
- feedback loop ref:
- RED target / plan:
- RED evidence (actual):
- green evidence:
- refactor notes:
- waiver / alternate loop:
- trace ref:

## Key Changes
-

## Issues And Follow-Ups
-

## Journey Spine Updates
- new facts:
- rejected options:
- domain language update:
- module/interface update:
- watchpoint changes:
- next run should know:

## Evidence Refs
- evidence manifest:
- TDD trace:
- Manual QA:
- diff:
- logs:
- bundle:
- checkpoint:
````

## 메모

Raw log와 diff는 artifact로 남기고, report에는 link만 둡니다. `RUN-SUMMARY`에 담긴 same-session review content에 해당하는 내용은 self-check 또는 stewardship signal로만 취급합니다. Detached verification으로 표시하면 안 됩니다.
