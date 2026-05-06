# Appendix A: Template Library

## 문서 역할

이 appendix는 전체 Markdown projection template variant를 담당한다. Projection rule과 template tier는 `07-document-projection.md`가 담당하며, 이 appendix는 그 rule을 구현하는 complete body를 제공한다.

Template은 rendered shape의 예시다. Canonical state가 아니며 kernel field, MCP schema, SQLite DDL을 재정의하면 안 된다.

## Template Rules

1. Front matter는 identity, task/project relation, projection version 또는 status, timestamp로 최소화한다.
2. Generated state는 managed block 안에 둔다.
3. Refresh 사이에도 human-editable section을 preserve한다.
4. Raw evidence는 artifact ref로 link한다.
5. Large log, diff, trace, bundle, screenshot, secret을 paste하지 않는다.
6. Approval, verification, Manual QA, acceptance를 visible하게 분리한다.
7. Card가 `Manual QA: pending/passed/failed/waived`라고 말하더라도 `qa_gate`를 canonical로 취급한다.
8. Template change는 projection change로 versioning한다.

## Required MVP Templates

### TASK

````md
---
doc_type: task
task_id: TASK-0001
display_state: executing
projection_version: 7
updated_at: 2026-05-06T09:30:15+09:00
---

# TASK-0001 Task Title

<!-- HARNESS:BEGIN managed -->
## 현재 요약
- mode:
- lifecycle phase:
- result:
- close reason:
- assurance:
- next action:
- pending decision:
- risk:
- scope gate:
- approval gate:
- design gate:
- evidence gate:
- verification gate:
- Manual QA:
- acceptance gate:
- active change unit:
- latest report:
- projection freshness:

## 목표
-

## 범위
### 포함
-

### 제외
-

## Acceptance Criteria
- [ ] AC-01:
- [ ] AC-02:

## Active Change Unit
| ID | Purpose | Status | Slice Type | TDD | Manual QA | Core Verification |
|---|---|---|---|---|---|---|
| CU-01 | | | vertical | required | pending | |

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

#### 확장 TASK Section

Long-running `work` task에는 이 section을 사용한다. 명시적으로 human-editable로 표시하지 않는 한 managed로 유지한다.

````md
<!-- HARNESS:BEGIN managed -->
## Shared Design Concept
### 해결된 질문
| ID | Question | User Answer | Decision / Assumption |
|---|---|---|---|

### 남은 모호함
- item / owner / stop condition:

## Domain Language Refs
- Terms in force:
  - Term:

## Module and Interface Refs
- MODULE-MAP:
- INTERFACE-CONTRACT:
- DESIGN:

## Change Unit Dependencies
| ID | blocked_by | unblocks | parallelizable_with | merge risk |
|---|---|---|---|---|

## Rolling Spine
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

#### Change Unit Block

이는 별도의 canonical projection tier가 아니라 TASK sub-template이다.

````md
### CU-01 Title
- purpose:
- non-goals:
- slice type: vertical | enabling | cleanup | horizontal-exception
- horizontal exception reason:
- follow-up vertical CU:
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
- validator profile:
  - changed_paths
  - approval_scope
  - vertical_slice_shape
  - tdd_trace
  - evidence_sufficiency
- sensitive categories:
  - none
- TDD:
  - required: yes | no | recommended
  - red evidence:
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

### APR

````md
---
doc_type: approval
approval_id: APR-0001
task_id: TASK-0001
category: dependency_change
status: pending
updated_at: 2026-05-06T09:30:15+09:00
---

# APR-0001 Approval Request

## Request Summary
- proposed action:

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
````

### RUN-SUMMARY

````md
---
doc_type: run_summary
run_id: RUN-20260506-093015-LEAD-01
task_id: TASK-0001
change_unit_id: CU-01
profile: lead
kind: implementation
surface_id: reference
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
- allowed paths:
- allowed tools:
- approval refs:

## Changed Files
- `path/to/file`

## Commands And Checks
```bash
npm test -- --runInBand
```

## Validator Outcomes
- changed_paths:
- approval_scope:
- vertical_slice_shape:
- tdd_trace:
- module_boundary_review:
- lint:
- test:
- build:
- evidence_sufficiency:

## TDD Trace Summary
- required:
- red evidence:
- green evidence:
- refactor notes:
- trace ref:

## Key Changes
-

## Issues And Follow-Ups
-

## Spine Updates
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

### EVIDENCE-MANIFEST

````md
---
doc_type: evidence_manifest
evidence_manifest_id: EM-0001
task_id: TASK-0001
change_unit_id: CU-01
status: partial
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
| tdd_trace | passed | TDD-0001 | |
| module_boundary_review | passed | DESIGN-0001 | |
| manual_qa_required | pending | QA-0001 | |

## Approval Refs
- APR-0001:

## Evidence Refs
- run summary:
- TDD trace:
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
- domain language changes
- interface contract changes
````

### EVAL

````md
---
doc_type: eval
eval_id: EVAL-0001
task_id: TASK-0001
change_unit_id: CU-01
verdict: passed
surface_id: reference
updated_at: 2026-05-06T10:05:00+09:00
---

# EVAL-0001 Verification Result

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

## Checks Performed
- [ ] changed_paths
- [ ] approval_scope
- [ ] same_session_verify_guard
- [ ] evidence_sufficiency
- [ ] bundle_integrity
- [ ] acceptance_review
- [ ] baseline_freshness
- [ ] vertical_slice_shape
- [ ] tdd_trace
- [ ] module_boundary_review
- [ ] public_interface_change_review
- [ ] manual_qa_required
- [ ] lint
- [ ] test
- [ ] build

## Evidence Reviewed
- task summary:
- rolling spine:
- domain language:
- module map:
- interface contract:
- run summary:
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

### DIRECT-RESULT

````md
---
doc_type: direct_result
task_id: TASK-0001
run_id: RUN-20260506-093015-LEAD-01
result: passed
assurance_level: self_checked
surface_id: reference
updated_at: 2026-05-06T09:40:00+09:00
---

# DIRECT-RESULT

## Request
- user request:

## Scope
- direct run scope:
- limits:

## Changed Files
- `path/to/file`

## Checks Performed
- changed_paths:
- approval_scope:
- test:
- build:
- docs_consistency:

## Outcome
- result summary:

## Assurance
- assurance_level:
- meaning:
- detached verify needed:

## Escalation
- escalated_to_work: yes | no
- reason:

## Evidence Refs
- logs:
- diff:
- follow-up report:
````

## Optional Design-Quality Templates

### DOMAIN-LANGUAGE

````md
---
doc_type: domain_language
project_id: PRJ-0001
status: active
projection_version: 1
updated_at: 2026-05-06T09:30:15+09:00
---

# Domain Language

<!-- HARNESS:BEGIN managed -->
## Summary
- current status:
- latest reconciled task:
- stale conditions:

## Terms
| Term | Meaning | Code Representation | Not This | Related Terms | Source | Status |
|---|---|---|---|---|---|---|
| Account | login-capable user identity | `src/auth/account.ts` | Profile | User, Session | TASK-0001 | active |

## Pending Term Decisions
| Term | Question | Options | Recommendation | Owner |
|---|---|---|---|---|

## Deprecated Terms
| Term | Replaced By | Reason | Since |
|---|---|---|---|
<!-- HARNESS:END managed -->

## User Notes and Proposals
-
````

### MODULE-MAP

````md
---
doc_type: module_map
project_id: PRJ-0001
status: active
projection_version: 1
updated_at: 2026-05-06T09:30:15+09:00
---

# Module Map

<!-- HARNESS:BEGIN managed -->
## Summary
- architecture state:
- latest review:
- stale conditions:

## Modules
| Module | Role | Public Interface | Internal Complexity | Dependencies | Test Boundary | Owner Decision |
|---|---|---|---|---|---|---|
| AuthService | verifies auth and issues sessions | `login`, `logout` | credential validation, session issue | UserRepo, SessionStore | service interface tests | human_reviewed |

## Deep Module Candidates
| Candidate | Current Pain | Proposed Boundary | Expected Test Boundary | Priority |
|---|---|---|---|---|

## Architecture Watchpoints
- shallow module growth:
- dependency direction risk:
- public interface drift:
<!-- HARNESS:END managed -->

## User Notes and Proposals
-
````

### INTERFACE-CONTRACT

````md
---
doc_type: interface_contract
interface_contract_id: IFACE-0001
task_id: TASK-0001
status: proposed
projection_version: 1
updated_at: 2026-05-06T09:30:15+09:00
---

# IFACE-0001 Interface Title

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
- status:
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
-
````

### TDD-TRACE

````md
---
doc_type: tdd_trace
tdd_trace_id: TDD-0001
task_id: TASK-0001
change_unit_id: CU-01
status: recorded
updated_at: 2026-05-06T09:40:00+09:00
---

# TDD-0001 Trace Title

## Identity
- task_id:
- change_unit_id:
- required: yes | no | recommended

## Red
- failing test ref:
- command:
- result: failed_as_expected | failed_unexpectedly | missing
- log ref:

## Green
- command:
- result: passed | failed | missing
- log ref:

## Refactor
- performed: yes | no
- notes:
- verification command:
- log ref:

## Non-TDD Justification
- reason:
- alternate feedback loop:

## Evidence Refs
- test:
- red log:
- green log:
- diff:
````

### MANUAL-QA

````md
---
doc_type: manual_qa
manual_qa_record_id: QA-0001
task_id: TASK-0001
change_unit_id: CU-01
result: pending
updated_at: 2026-05-06T10:05:00+09:00
---

# QA-0001 Manual QA

## Identity
- task_id:
- change_unit_id:
- profile: ui_quality | workflow | copy | accessibility | browser_smoke | performance_smoke | none
- required: yes | no
- performed by:

## Setup
- build/run command:
- test account/data:
- route or screen:

## Checklist
- [ ] primary workflow works
- [ ] errors are understandable
- [ ] visual layout acceptable
- [ ] accessibility smoke check
- [ ] no obvious regression

## Result
- result: pending | passed | failed | waived
- summary:
- waiver reason:

## Findings
| Severity | Finding | Suggested Action | Follow-up CU |
|---|---|---|---|
| minor | | | |

## Evidence Refs
- screenshot:
- browser log:
- video:
- note:
````

## Appendix Templates

### DEC

````md
---
doc_type: decision
decision_id: DEC-0001
task_id: TASK-0001
status: proposed
updated_at: 2026-05-06T09:30:15+09:00
---

# DEC-0001 Decision Title

## Problem
- decision needed:

## Options
### Option A
- suitable when:
- benefits:
- cost/risk:

### Option B
- suitable when:
- benefits:
- cost/risk:

## Recommendation
- recommendation:
- reason:

## Final Decision
- status:
- decision:
- decided at:
- decided by:

## Impact
- code:
- domain language:
- module/interface:
- docs:
- operations:
- tests:
- user experience:

## Follow-Up
- [ ]

## References
- TASK:
- DESIGN:
- APR:
- EVIDENCE-MANIFEST:
````

### DESIGN

````md
---
doc_type: design
design_id: DESIGN-0001
task_id: TASK-0001
status: draft
updated_at: 2026-05-06T09:30:15+09:00
---

# DESIGN-0001 Design Title

## Problem
- design problem:

## Goals
- goal:

## Non-Goals
- non-goal:

## Constraints
- technical:
- operational:
- compatibility:
- security/privacy:

## Shared Design Summary
- resolved questions:
- remaining assumptions:
- rejected options:

## Domain Language Impact
| Term | Impact | Action |
|---|---|---|

## Module And Interface Plan
| Module | Current Role | Proposed Change | Public Interface | Test Boundary | Risk |
|---|---|---|---|---|---|

## Proposed Shape
- components:
- boundaries and responsibilities:
- data flow:
- dependency direction:

## Alternatives
### Alternative A
- benefits:
- drawbacks:

### Alternative B
- benefits:
- drawbacks:

## Recommendation
- recommendation:
- remaining trade-off:

## Verification Considerations
- success criteria:
- regression watchpoint:
- required TDD trace:
- required Manual QA:
- required evidence:

## References
- TASK:
- DEC:
- APR:
- DOMAIN-LANGUAGE:
- MODULE-MAP:
- INTERFACE-CONTRACT:
- EVIDENCE-MANIFEST:
````

### EXPORT Manifest

````md
---
doc_type: export_manifest
export_id: EXPORT-0001
project_id: PRJ-0001
status: complete
updated_at: 2026-05-06T10:30:00+09:00
---

# EXPORT-0001 Harness Export

## Scope
- project_id:
- task_ids:
- included state version range:
- created by:
- created at:

## State Snapshots
- tasks:
- task gates:
- change units:
- runs:
- approvals:
- evidence manifests:
- Eval records:
- Manual QA records:
- reconcile items:

## Projection Snapshots
- TASK:
- APR:
- RUN-SUMMARY:
- EVIDENCE-MANIFEST:
- EVAL:
- DIRECT-RESULT:
- optional design projections:

## Artifact Refs
| Artifact ID | Kind | URI | SHA256 | Redaction State | Retention |
|---|---|---|---|---|---|

## Redaction Summary
- secrets omitted:
- redacted artifacts:
- blocked artifacts:

## Integrity
- export hash:
- manifest hash:
- generated at:
````

## Expanded Cards

### Compact Status Card

````text
TASK-{id} {title}
State: {mode} / {lifecycle_phase}
Next action: {next_action}
User decision: {pending_decision_summary|none}
Risk: {risk_summary}
Evidence gate: {evidence_gate}
Design gate: {design_gate}
Manual QA: {qa_gate display: pending|passed|failed|waived|not_required}
Latest report: {latest_report|none}
````

### Approval Card

````text
Approval is required.

{approval_id} {category}
Request: {summary}
Purpose: {why_needed}
Allowed paths:
{allowed_paths}

Allowed tools:
{allowed_tools}

Network:
{allowed_network}

Required secrets:
{required_secrets}

Baseline:
{baseline_ref}

Risks:
{risks}

Alternatives:
{alternatives}

Recommendation:
{recommendation}

Do you approve this scope?
````

### Verification Result Card

````text
Verification complete.

{eval_id}
Verdict: {verdict}
Assurance: {assurance_impact}
Verification independence: {verification_independence}
Manual QA: {manual_qa_impact}
Acceptance: {acceptance_impact}

Evidence reviewed:
- task summary: {task_summary_ref}
- run summary: {run_summary_ref}
- evidence manifest: {evidence_manifest_ref}
- TDD trace: {tdd_trace_ref}
- diff: {diff_ref}
- logs: {logs_ref}
- approvals: {approval_refs}
- design refs: {design_refs}

Remaining work:
{blockers_or_rework}

User follow-up:
{user_followup}
````

### Manual QA Card

````text
Manual QA is required.

{manual_qa_record_id}
Profile: {profile}
Target: {screen_or_flow}
Checklist:
- {checklist_item}

Evidence to record:
- screenshot or walkthrough note
- browser log when relevant

Record the QA result?
````

## Template Change Notes

- `DOMAIN-LANGUAGE`, `MODULE-MAP`, `INTERFACE-CONTRACT`는 canonical document가 아니라 canonical record에서 만든 projection이다.
- `MANUAL-QA`는 record projection이다. Close-relevant gate는 `qa_gate`로 남는다.
- `EVAL`은 independence context를 보여줘야 한다. Passed verdict만으로는 `detached_verified`가 생기지 않기 때문이다.
- `RUN-SUMMARY`, `EVIDENCE-MANIFEST`, `DIRECT-RESULT`는 large evidence를 embed하지 않고 artifact ref로 evidence file에 link한다.
