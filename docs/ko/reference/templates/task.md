# TASK Template

## Used when

진행 중인 작업을 이어서 파악할 수 있는 projection이 필요할 때 `TASK`를 사용합니다. 이 template은 작업의 현재 위치와 판단 맥락을 보여줍니다. 또한 Autonomy Boundary, Write Authority Summary, Implementation Micro-Plan, Review Stages, Stewardship Impact, Residual Risk, gate, active Change Unit, pending decision을 요약합니다. 다음 evidence, 관련 report ref, projection freshness도 함께 보여줍니다.

## Source records

- `state.sqlite` Task와 task gate
- active Change Unit과 Change Unit dependency
- Write Authorization record와 Write Authority Summary 표시 input
- Decision Packet과 Residual Risk
- latest Run, Evidence Manifest, Eval, Manual QA record, approval record
- Journey Spine source record
- `domain_terms`, `module_map_items`, `interface_contracts`, `feedback_loops`
- TDD가 선택된 경우 `tdd_traces`
- design-quality validator result
- 예상되는 evidence 필요 항목
- Review Stage 표시 input
- artifact ref와 projection freshness

## Rendered sections

- Current Summary
- Where We Are
- Judgment Context
- Autonomy Boundary
- Write Authority Summary
- Implementation Micro-Plan
- Review Stages
- Next Evidence
- Residual Risk
- Stewardship Impact
- Goal
- Scope
- Acceptance Criteria
- Active Change Unit
- Pending Decisions
- Evidence And Reports
- User Notes and Proposals

장기 `work` Task는 shared design, domain term ref, module/interface ref, Change Unit dependency, implementation detail, Journey Spine을 위한 expanded managed section을 표시할 수 있습니다.

## Full template

````md
---
doc_type: task
task_id: TASK-0001
display_state: executing
projection_version: 7
source_state_version: 42
updated_at: 2026-05-06T09:30:15+09:00
---

# TASK-0001 Task Title

<!-- HARNESS:BEGIN managed -->
## Current Summary
- mode:
- lifecycle phase:
- result:
- close reason:
- assurance:
- next action:
- pending decision:
- risk:
- scope gate:
- decision gate:
- approval gate:
- design gate:
- evidence gate:
- verification gate:
- Manual QA:
- acceptance gate:
- active change unit:
- write authority summary:
- latest report:
- projection freshness:

## Where We Are
- current position:
- active path:
- current blocker:
- latest meaningful evidence:
- next state transition:

## Judgment Context
- pending decision packets:
- what user is deciding:
- what agent may decide without user:
- recommendation:
- main trade-off:
- reversibility:
- uncertainty:
- minimum context to judge:
- affected gates:

## Autonomy Boundary
- profile:
- agent may do:
- user judgment required:
- AFK stop conditions:
- boundary status:

## Write Authority Summary
- active Change Unit:
- write authorization:
- allowed paths:
- allowed tools:
- allowed commands:
- allowed network targets:
- secret scope:
- sensitive categories:
- approval status:
- baseline:
- guarantee:
- note: Autonomy Boundary is judgment latitude, not write authority.

## Implementation Micro-Plan
- note: execution aid only; active Change Unit scope bounds writes and `prepare_write` creates Write Authorization.
- TDD note: required이면 selected feedback loop, RED target, GREEN target, non-test implementation이 actual RED evidence 또는 waiver를 기다리는지 표시한다.

| Step / Slice | Purpose | Active Change Unit Scope / Likely Paths | Feedback Loop / TDD | Expected Evidence | Stop / Ask User When |
|---|---|---|---|---|---|
| 1 | | | | | |

## Review Stages
- note: managed display only; same-session review는 detached verification이 아니다.

### Spec Compliance Review
- acceptance criteria coverage:
- Change Unit completion conditions:
- scope / Write Authority compatibility:
- Decision Packet compatibility:
- evidence coverage:
- residual-risk visibility:
- routed outcome:

### Code Quality / Stewardship Review
- domain language:
- module / interface boundary:
- vertical slice shape:
- feedback loop / TDD:
- codebase stewardship:
- context hygiene:
- follow-up risk:
- routed outcome:

## Next Evidence
- next evidence action:
- evidence needed for:
- TDD RED target / plan:
- TDD RED evidence:
- TDD GREEN evidence:
- TDD refactor/check evidence:
- expected artifact refs:
- stale or missing evidence:

## Residual Risk
- close-relevant risk:
- visibility status:
- accepted residual-risk refs:
- follow-up required:
- close impact:

## Stewardship Impact
- summary shape: StewardshipImpactSummary
- domain_language_impact: none | updated | conflict | unresolved
- module_boundary_impact: none | local | public_boundary | unresolved
- interface_contract_impact: none | compatible | breaking | unresolved
- feedback_loop_status: defined | missing | waived
- future_change_risk: none | visible | accepted | unresolved
- close_impact: none | blocks_close | requires_decision | residual_risk
- refs:
  - domain term refs:
  - module map item refs:
  - interface contract refs:
  - feedback loop refs:
  - TDD trace refs when selected:
  - residual risk:
  - Decision Packets:

## Goal
-

## Scope
### In
-

### Out
-

## Acceptance Criteria
- [ ] AC-01:
- [ ] AC-02:

## Active Change Unit
| ID | Purpose | Status | Slice Type | TDD | Manual QA | Core Verification |
|---|---|---|---|---|---|---|
| CU-01 | | | vertical | required: red_pending \| red_recorded \| green_recorded \| waived | pending | |

## Pending Decisions
-

## Evidence And Reports
- Evidence Manifest:
- Run Summary:
- Eval:
- Direct Result:
- TDD Trace:
- Manual QA:
- Approval:
- Decision:
- Diff:
- Logs:
<!-- HARNESS:END managed -->

## User Notes and Proposals
-
````

Long-running `work` task를 위한 expanded TASK section:

````md
<!-- HARNESS:BEGIN managed -->
## Shared Design Concept
### Questions Resolved
| ID | Question | User Answer | Decision / Assumption |
|---|---|---|---|

### Remaining Ambiguity
- item / owner / stop condition:

## Domain Term Refs
- Terms in force:
  - Term:

## Module and Interface Refs
- module map item refs:
- interface contract refs:
- rendered projection refs, if shown: MODULE-MAP, INTERFACE-CONTRACT
- DESIGN:

## Change Unit Dependencies
| ID | blocked_by | unblocks | parallelizable_with | merge risk |
|---|---|---|---|---|

## Implementation Micro-Plan Details
- source alignment: current Task, active Change Unit, gates, related refs
- boundary: not canonical state, not scope authority, not approval, not Write Authorization; active Change Unit remains the scope source

### Step Queue
| Step | State Alignment | Scope Alignment / Likely Paths | Feedback Loop / TDD Status | Evidence Target | Stop Condition |
|---|---|---|---|---|---|

## Journey Spine
### Facts in Force
- fact / evidence ref:

### Assumptions in Force
- assumption / expiry condition:

### Decisions in Force
- DEC-0001:

### Domain Terms in Force
- term / meaning / code representation:

### Module / Interface Impacts
- module / impact / interface / test boundary:

### Rejected Options
- option / reason / DEC:

### Watchpoints
- regression:
- security/performance/operations:
- architecture drift:

### Resume Notes
- next session should know:
- current blocker:
<!-- HARNESS:END managed -->
````

Change Unit block sub-template:

````md
### CU-01 Title
- purpose:
- non-goals:
- slice type: vertical | enabling | cleanup | horizontal-exception
- horizontal exception reason:
- follow-up vertical CU:
- autonomy profile:
- agent may do:
  - implementation detail:
  - local refactor inside scope:
  - evidence collection:
- user judgment required:
  - product direction:
  - public interface or compatibility commitment:
  - residual risk acceptance:
- AFK stop conditions:
  - boundary exceeded:
  - evidence cannot be produced:
  - close-relevant risk discovered:
- end-to-end path:
  - trigger / input:
  - domain logic:
  - persistence:
  - API / caller boundary:
  - UI / observable output:
- allowed paths:
  - `src/...`
  - `tests/...`
- allowed tools:
  - read
  - edit
  - shell: `npm test -- ...`
- check profile:
  - changed_paths
  - approval_scope
  - evidence_sufficiency
- ValidatorResult IDs:
  - vertical_slice_shape
  - shared_design_alignment
  - decision_quality_check
  - autonomy_boundary_check
  - feedback_loop_check
  - tdd_trace_required
  - domain_language_consistency
  - module_interface_review
  - codebase_stewardship_check
  - residual_risk_visibility_check
  - manual_qa_required
- sensitive categories:
  - none
- TDD:
  - required: yes | no | recommended
  - RED target / plan:
  - RED evidence (actual):
  - green evidence:
  - non-TDD justification:
- Manual QA:
  - required: yes | no
  - profile: ui_quality | workflow | copy | accessibility | browser_smoke | none
- dependencies:
  - blocked_by:
  - unblocks:
  - parallelizable_with:
  - merge risk:
- completion conditions:
  - [ ]
- evaluator focus:
  - item:
````

## Notes

`TASK`의 Stewardship Impact는 owner record, validator result, ref에서 derive되는 `StewardshipImpactSummary` display입니다. Domain Language, Module Map, Interface Contract, Feedback Loop, TDD Trace, residual-risk, Decision Packet owner record를 replace하지 않습니다.

`TASK`의 Implementation Micro-Plan은 current Task와 Change Unit state에서 생성되거나 그 state와 aligned된 lightweight execution aid입니다. Product write를 authorize하거나, scope를 넓히거나, approval을 satisfy하거나, evidence를 만들거나, edit만으로 state를 변경하거나, `prepare_write`를 replace하지 않습니다.

`TASK`의 Review Stages는 managed display section입니다. Gates를 satisfy하거나, writes를 authorize하거나, risk를 accept하거나, Task를 close하거나, `detached_verified` assurance를 만들지 않습니다.
