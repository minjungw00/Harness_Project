# EVAL Template

## Used when

검증 결과와 독립성 맥락을 함께 읽기 쉽게 보여줘야 할 때 `EVAL`을 사용합니다.

## Source records

- Eval record
- verification target
- verdict
- independence qualifier
- baseline relationship
- performed check
- reviewed evidence
- blocker
- artifact ref

## Rendered sections

- Target
- Verdict
- Environment And Independence
- Checks And Validator Outcomes
- Evidence Reviewed
- Acceptance Criteria Review
- Design Quality Review
- Rationale
- Blockers Or Rework
- User Follow-Up

## Full template

````md
---
doc_type: eval
eval_id: EVAL-0001
task_id: TASK-0001
change_unit_id: CU-01
verdict: passed
surface_id: reference
source_state_version: 45
updated_at: 2026-05-06T10:05:00+09:00
---

# EVAL-0001 Verification Result

## Target
- task_id:
- change_unit_id: CU-01 | null
- target_run_id:
- evaluator_run_id:

## Verdict
- verdict: passed | failed | blocked | inconclusive
- assurance impact:
- verification gate impact:
- Manual QA impact:
- acceptance impact:
- next action:

## Environment And Independence
- fresh run:
- evaluator surface:
- context independence: same_session | subagent_context | fresh_session | fresh_worktree | sandbox | manual_bundle
- write capable:
- product file write allowed:
- baseline verified:
- repo drift observed:
- source input: chat_history | task_summary | bundle | raw_artifacts
- source bundle:
- parent run:

## Checks And Validator Outcomes
### Core Checks And Preconditions
- [ ] changed_paths
- [ ] approval_scope
- [ ] same_session_verify_guard
- [ ] evidence_sufficiency
- [ ] bundle_integrity
- [ ] acceptance_review
- [ ] baseline_freshness
- [ ] public_interface_change_review
- [ ] lint
- [ ] test
- [ ] build

### ValidatorResult IDs
- [ ] vertical_slice_shape
- [ ] shared_design_alignment
- [ ] decision_quality_check
- [ ] autonomy_boundary_check
- [ ] feedback_loop_check
- [ ] tdd_trace_required
- [ ] domain_language_consistency
- [ ] module_interface_review
- [ ] codebase_stewardship_check
- [ ] residual_risk_visibility_check
- [ ] manual_qa_required
- [ ] surface_capability_check

## Evidence Reviewed
- task summary:
- Journey Spine:
- Decision Packets:
- Residual Risks:
- Autonomy Boundary:
- domain term refs:
- module map item refs:
- interface contract refs:
- run summary:
- feedback loop:
- TDD trace:
- Manual QA:
- evidence manifest:
- diff:
- bundle:
- logs:
- approvals:
- decisions:

## Acceptance Criteria Review
| AC ID | Statement | Evidence Reviewed | Result | Notes |
|---|---|---|---|---|

## Design Quality Review
- vertical slice:
- Decision Packets:
- Autonomy Boundary:
- Residual Risks:
- feedback loop:
- TDD trace:
- module/interface:
- architecture drift:
- domain language consistency:

## Rationale
-

## Blockers Or Rework
-

## User Follow-Up
- trade-off needing confirmation:
- remaining options:
- Manual QA need:
````

## Notes

Eval verdict alone으로는 assurance가 upgrade되지 않습니다. `detached_verified`에는 valid independence와 passed verification, same-session self-review violation 부재가 필요합니다.
