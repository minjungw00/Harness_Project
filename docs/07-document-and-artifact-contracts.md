# 07. Document and Artifact Contracts

## 1. 문서 역할

이 문서는 Product Repository 안에 남는 Markdown projection과 artifact reference 계약을 정의한다. Projection 원칙, document authority, managed 영역, human-editable 영역, artifact ref, stale 조건, 템플릿, 상태 card를 소유한다.

Authoritative DB schema는 `04-reference-implementation.md`가 소유한다. Surface별 connector 설정 전문은 `06-agent-integration.md`가 소유한다.

## 2. Projection 원칙

1. 문서는 사람이 읽는 표면이다.
2. 운영 상태 전이의 canonical source는 `state.sqlite`와 event log다.
3. Raw evidence의 canonical source는 artifact store다.
4. Front matter는 식별과 projection version을 최소로 담는다.
5. 현재 상태를 이해하는 정보는 본문 summary에 둔다.
6. Raw logs, large trace, patch 전문은 artifact ref로 둔다.
7. `TASK`는 현재 continuity를 읽는 중심 projection이다.
8. 오래된 세부 이력은 `RUN-SUMMARY`, `EVAL`, `APR`, `DEC`, `EVIDENCE-MANIFEST`, `TDD-TRACE`, `MANUAL-QA`로 분리한다.
9. Approval, assurance, manual QA, acceptance를 섞지 않는다.
10. 문서 최신성은 `projection_version`과 `updated_at`으로 표시한다.
11. Rule/context 파일은 짧은 원칙만 담는다.
12. 긴 절차는 Skill이나 playbook으로 분리한다.
13. Domain language, module map, interface contract 변경은 reconcile 또는 MCP tool로 반영한다.

## 3. Document authority matrix

| 항목 | 사람이 읽는 표면 | canonical source | 생성/갱신 주체 |
|---|---|---|---|
| 현재 Task 상태 | `TASK` Current Summary, status card | `state.sqlite` + event log | projector |
| Task continuity | `TASK` Rolling Spine | state + events + doc refs | projector/reconcile |
| 사용자 메모 | `TASK` User Notes | reconcile item | user/reconcile |
| shared design | `TASK`, `DESIGN`, `DEC` | design context record + event log | core/projector |
| domain language | `DOMAIN-LANGUAGE` | domain language record + reconciled doc | core/projector |
| module map | `MODULE-MAP` | module map records + reconciled doc | core/projector |
| interface contract | `INTERFACE-CONTRACT` | interface contract records | core/projector |
| approval | `APR` | approval record + event log | core/projector |
| raw diff/log/checkpoint | artifact ref | artifact store | artifact registry |
| TDD trace | `TDD-TRACE`, `RUN-SUMMARY` | tdd_trace record + artifact refs | core/projector |
| manual QA | `MANUAL-QA`, `EVAL` | QA record + artifact refs | core/projector |
| evidence coverage | `EVIDENCE-MANIFEST` | evidence manifest record + artifact refs | core/projector |
| implementation summary | `RUN-SUMMARY` | run record + artifact refs | core/projector |
| verification verdict | `EVAL` | eval record + artifact refs | core/projector |
| direct result | `DIRECT-RESULT` | direct result record + artifact refs | core/projector |
| projection freshness | front matter, status card | projection job state | projector |

## 4. Managed 영역과 human-editable 영역

Managed 영역은 projector가 생성하고 갱신한다.

```md
<!-- HARNESS:BEGIN managed -->
...
<!-- HARNESS:END managed -->
```

Human-editable 영역은 사용자가 메모, 제안, 질문을 남기는 곳이다.

```md
## User Notes and Proposals
-
```

규칙:

- Managed 영역 직접 수정은 projection drift로 감지한다.
- Human-editable 영역 수정은 user observation 또는 proposal로 본다.
- 상태 반영은 MCP tool, reconcile action, operator action을 통해 수행한다.
- Projector는 human-editable 영역을 보존한다.

## 5. Artifact reference 규칙

문서에는 artifact 전문을 넣지 않고 stable reference를 둔다.

```text
DIFF-0001
LOG-0001
BUNDLE-0001
CHECKPOINT-0001
MANIFEST-0001
TDD-0001
QA-0001
```

Artifact ref는 다음을 만족한다.

- artifact registry에 존재한다.
- sha256 또는 content hash가 있다.
- task_id와 run_id가 연결된다.
- retention class가 명시된다.
- 민감 데이터 redaction 상태가 기록된다.

## 6. 최소 front matter

### TASK

```yaml
---
doc_type: task
task_id: TASK-0001
display_state: verify_pending
projection_version: 18
updated_at: 2026-05-06T10:05:00+09:00
---
```

### DOMAIN-LANGUAGE

```yaml
---
doc_type: domain_language
project_id: PRJ-0001
status: active
projection_version: 4
updated_at: 2026-05-06T10:05:00+09:00
---
```

### MODULE-MAP

```yaml
---
doc_type: module_map
project_id: PRJ-0001
status: active
projection_version: 4
updated_at: 2026-05-06T10:05:00+09:00
---
```

### INTERFACE-CONTRACT

```yaml
---
doc_type: interface_contract
interface_contract_id: IFACE-0001
task_id: TASK-0001
status: reviewed
updated_at: 2026-05-06T10:05:00+09:00
---
```

### Report 계열

`doc_type`, id, task_id, status 또는 verdict, updated_at을 최소로 가진다.

## 7. 생성 시점과 stale 조건

| 문서 | 생성 시점 | stale 조건 |
|---|---|---|
| `TASK` | Task 생성 또는 재개 | state_version > projected_version, managed block drift |
| `DOMAIN-LANGUAGE` | 프로젝트 연결 또는 도메인 용어 필요 시 | term conflict, reconciled doc drift, code representation 변경 |
| `MODULE-MAP` | 프로젝트 연결, architecture review, module-impact work | module path 변경, public interface 변경, dependency direction 변경 |
| `INTERFACE-CONTRACT` | public interface 변경 또는 review 필요 시 | linked interface 변경, caller 변경, compatibility impact 변경 |
| `APR` | approval request 생성 | scope drift, approval status 미반영 |
| `DEC` | 결정 초안 또는 결정 기록 | decision status 미반영 |
| `DESIGN` | 설계가 필요한 work 또는 advisor | linked Task scope 변경, module map drift |
| `RUN-SUMMARY` | run 완료 | artifact ref 누락, state relation 변경 |
| `TDD-TRACE` | TDD required/recommended CU 구현 | test file 변경, red/green log 누락, baseline drift |
| `MANUAL-QA` | manual QA required 또는 수행 시 | linked UI/code 변경, screenshot/log/note 누락, finding unresolved |
| `EVIDENCE-MANIFEST` | evidence mapping 생성/갱신 | baseline drift, changed files modified, approval expired, domain/interface drift |
| `EVAL` | verification 결과 기록 | baseline 재변경, evidence stale, verdict relation 변경 |
| `DIRECT-RESULT` | direct 종료 | changed file drift, escalation state 변경 |

## 8. TASK 템플릿

`TASK`는 현재 요약과 continuity를 유지하는 중심 문서다. 기본형과 상세 확장형을 구분한다.

### 8.1 Compact TASK

````md
---
doc_type: task
task_id: TASK-0001
display_state: executing
projection_version: 7
updated_at: 2026-05-06T09:30:15+09:00
---

# TASK-0001 작업 제목

<!-- HARNESS:BEGIN managed -->
## Current Summary
- mode:
- phase:
- next action:
- pending decision:
- risk:
- evidence:
- design alignment:
- architecture:
- assurance:
- approval:
- manual QA:
- acceptance:
- active change unit:
- latest report:
- projection freshness:

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
| ID | 목적 | 상태 | Slice Type | TDD | Manual QA | 핵심 검증 |
|---|---|---|---|---|---|---|
| CU-01 | | | vertical | required | pending | |

## Pending Decisions
- 

## Evidence and Reports
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

### 8.2 Detailed TASK sections

Work 또는 long-running Task에서 다음 섹션을 추가한다.

````md
<!-- HARNESS:BEGIN managed -->
## Shared Design Concept
### Questions Resolved
| ID | Question | User Answer | Decision / Assumption |
|---|---|---|---|

### Remaining Ambiguity
- 항목 / owner / stop condition

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
- 사실 / evidence ref:

### Assumptions in Force
- 가정 / 만료 조건:

### Decisions in Force
- DEC-0001: 요약

### Domain Terms in Force
- Term: meaning / code representation

### Module / Interface Impacts
- Module: impact / interface / test boundary

### Rejected Options
- 버린 선택지 / 이유 / 관련 DEC:

### Watchpoints
- 회귀 가능성:
- 보안/성능/운영 주의점:
- architecture drift 주의점:

### Resume Notes
- 다음 세션이 바로 알아야 할 점:
- 지금 막히는 지점:
<!-- HARNESS:END managed -->
````

## 9. Change Unit 템플릿

````md
### CU-01 제목
- 목적:
- 비목표:
- slice type: vertical | enabling | cleanup | horizontal-exception
- horizontal exception reason:
- follow-up vertical CU:
- end-to-end path:
  - trigger / input:
  - domain logic:
  - persistence:
  - API / caller boundary:
  - UI / observable output:
- 허용 경로:
  - `src/...`
  - `tests/...`
- 허용 도구:
  - read
  - edit
  - shell: `pnpm test ...`
- validator profile:
  - changed_paths
  - approval_scope
  - vertical_slice_shape
  - tdd_trace
  - evidence_sufficiency
- approval categories:
  - none
- TDD:
  - required: yes | no | recommended
  - red evidence:
  - green evidence:
  - non-TDD justification:
- manual QA:
  - required: yes | no
  - profile: ui_quality | workflow | copy | accessibility | browser_smoke | none
- dependencies:
  - blocked_by:
  - unblocks:
  - parallelizable_with:
  - merge risk:
- 완료 조건:
  - [ ]
- evaluator focus:
  - 항목
````

## 10. DOMAIN-LANGUAGE 템플릿

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
| Account | 로그인 가능한 사용자 identity | `src/auth/account.ts` | Profile | User, Session | TASK-0001 | active |

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

## 11. MODULE-MAP 템플릿

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
| AuthService | 인증 검증과 세션 생성 | `login`, `logout` | credential validation, session issue | UserRepo, SessionStore | service interface tests | human_reviewed |

## Deep Module Candidates
| Candidate | Current Pain | Proposed Boundary | Expected Test Boundary | Priority |
|---|---|---|---|---|

## Architecture Watchpoints
- shallow module 증가:
- dependency direction 위험:
- public interface drift:
<!-- HARNESS:END managed -->

## User Notes and Proposals
-
````

## 12. INTERFACE-CONTRACT 템플릿

````md
---
doc_type: interface_contract
interface_contract_id: IFACE-0001
task_id: TASK-0001
status: proposed
updated_at: 2026-05-06T09:30:15+09:00
---

# IFACE-0001 제목

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
````

## 13. EVIDENCE-MANIFEST 템플릿

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
- evidence_state:
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
| `src/...` | AC-01 | DIFF-..., TEST-..., TDD-... |

## Design Quality Coverage
| Item | Status | Evidence Refs | Notes |
|---|---|---|---|
| vertical_slice_shape | passed | CU-01 | |
| tdd_trace | passed | TDD-0001 | |
| module_boundary_review | passed | DESIGN-0001 | |
| manual_qa_required | pending | QA-0001 | |

## Approval Refs
- APR-...

## Evidence Refs
- run summary:
- TDD trace:
- manual QA:
- diff:
- logs:
- bundle:
- checkpoint:
- tests:
- build:

## Stale If
- baseline head changes
- changed files modified after eval
- approval scope expires
- relevant config changes
- domain language changes
- interface contract changes
````

## 14. DEC 템플릿

````md
---
doc_type: decision
decision_id: DEC-0001
task_id: TASK-0001
status: proposed
updated_at: 2026-05-06T09:30:15+09:00
---

# DEC-0001 제목

## Problem
- 지금 결정해야 하는 문제

## Options
### Option A
- 적합한 경우:
- 장점:
- 비용/리스크:

### Option B
- 적합한 경우:
- 장점:
- 비용/리스크:

## Recommendation
- 권장안:
- 권장 이유:

## Final Decision
- 상태:
- 결정 내용:
- 결정 시각:
- 결정자:

## Impact
- 코드 영향:
- 도메인 언어 영향:
- 모듈/interface 영향:
- 문서 영향:
- 운영 영향:
- 테스트 영향:
- 사용자 경험 영향:

## Follow-up
- [ ]

## References
- TASK:
- DESIGN:
- APR:
- EVIDENCE-MANIFEST:
````

## 15. APR 템플릿

````md
---
doc_type: approval
approval_id: APR-0001
task_id: TASK-0001
category: dependency_change
status: pending
updated_at: 2026-05-06T09:30:15+09:00
---

# APR-0001 승인 요청

## Request Summary
- 무엇을 하려는가:

## Requested Scope
- category:
- allowed paths:
- allowed tools:
- allowed network:
- required secrets:
- baseline ref:
- expected diff envelope:
- expires on scope drift:

## Why This Is Needed
- 목적:
- 현재 작업과의 관계:

## Impact
- 코드/문서 영향:
- 사용자/운영 영향:
- 개인정보/보안 영향:
- 비용/배포 영향:
- domain language/module/interface 영향:

## Risks
- 주요 리스크:
- 실패 시 영향:
- scope drift 조건:

## Alternatives
### Alternative A
- 설명:
- 장점:
- 비용/리스크:

### Alternative B
- 설명:
- 장점:
- 비용/리스크:

## Recommendation
- 권장안:
- 권장 이유:

## Decision
- status:
- decision note:
- decided by:
- decided at:
````

## 16. DESIGN 템플릿

````md
---
doc_type: design
design_id: DESIGN-0001
task_id: TASK-0001
status: draft
updated_at: 2026-05-06T09:30:15+09:00
---

# DESIGN-0001 제목

## Problem
- 설계가 필요한 문제

## Goals
- 달성해야 하는 목표

## Non-Goals
- 이번 설계에서 다루지 않는 것

## Constraints
- 기술 제약
- 운영 제약
- 호환성 제약
- 보안/개인정보 제약

## Shared Design Summary
- 해결한 질문:
- 남은 가정:
- rejected options:

## Domain Language Impact
| Term | Impact | Action |
|---|---|---|

## Module and Interface Plan
| Module | Current Role | Proposed Change | Public Interface | Test Boundary | Risk |
|---|---|---|---|---|---|

## Proposed Shape
- 구성 요소
- 경계와 책임
- 데이터 흐름
- dependency direction

## Alternatives
### Alternative A
- 장점:
- 단점:

### Alternative B
- 장점:
- 단점:

## Recommendation
- 권장안:
- 남는 trade-off:

## Verification Considerations
- 성공 판단 기준:
- 회귀 watchpoint:
- 필요한 TDD trace:
- 필요한 manual QA:
- 필요한 evidence:

## References
- TASK:
- DEC:
- APR:
- DOMAIN-LANGUAGE:
- MODULE-MAP:
- INTERFACE-CONTRACT:
- EVIDENCE-MANIFEST:
````

## 17. RUN-SUMMARY 템플릿

````md
---
doc_type: run_summary
run_id: RUN-20260506-093015-LEAD-01
task_id: TASK-0001
change_unit_id: CU-01
profile: lead
action: implement
surface_id: reference
updated_at: 2026-05-06T09:45:10+09:00
---

# RUN-SUMMARY

## Run Identity
- run_id:
- profile:
- action:
- surface:
- baseline_ref:
- state_version:

## Scope
- task_id:
- change_unit_id:
- slice type:
- allowed paths:
- allowed tools:
- approval refs:

## Changed Files
- `path/to/file`

## Commands and Checks
```bash
pnpm lint
pnpm test --runInBand
```

## Outcomes
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
- 항목

## Issues and Follow-ups
- 항목

## Spine Updates
- 새로 확정된 사실:
- 새로 버린 선택지:
- domain language update:
- module/interface update:
- watchpoint 변경:
- 다음 run이 알아야 할 점:

## Evidence Refs
- evidence manifest:
- TDD trace:
- manual QA:
- diff:
- logs:
- bundle:
- checkpoint:
````

## 18. TDD-TRACE 템플릿

````md
---
doc_type: tdd_trace
tdd_trace_id: TDD-0001
task_id: TASK-0001
change_unit_id: CU-01
status: recorded
updated_at: 2026-05-06T09:40:00+09:00
---

# TDD-0001 제목

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

## 19. MANUAL-QA 템플릿

````md
---
doc_type: manual_qa
manual_qa_id: QA-0001
task_id: TASK-0001
change_unit_id: CU-01
result: pending
updated_at: 2026-05-06T10:05:00+09:00
---

# QA-0001 제목

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
- result: passed | failed | waived
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

## 20. EVAL 템플릿

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

# EVAL-0001 제목

## Verdict
- verdict:
- assurance impact:
- manual QA impact:
- acceptance impact:
- next action:

## Environment and Independence
- fresh run:
- evaluator surface:
- context independence: same_session | subagent_context | fresh_session | fresh_worktree | sandbox
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
- manual QA:
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
- 왜 이 verdict인가

## Blockers or Rework
- 항목

## User Follow-up
- 확인이 필요한 trade-off
- 남은 선택지
- manual QA 필요 여부
````

## 21. DIRECT-RESULT 템플릿

````md
---
doc_type: direct_result
task_id: TASK-0001
result: passed
assurance_level: self_checked
surface_id: reference
updated_at: 2026-05-06T09:40:00+09:00
---

# DIRECT-RESULT

## Request
- 사용자의 요청

## Scope
- 이번 direct run이 다룬 범위
- 변경 한계

## Changed Files
- `path/to/file`

## Checks Performed
- changed_paths:
- approval_scope:
- test:
- build:
- docs_consistency:

## Outcome
- 요약 결과

## Assurance
- assurance_level:
- 의미:
- detached verify 필요 여부:

## Escalation
- escalated_to_work: yes | no
- reason:

## Evidence Refs
- logs:
- diff:
- follow-up report:
````

## 22. Card 템플릿

### Compact status card

````text
TASK-{id} {title}
상태: {mode} / {phase}
다음 행동: {next_action}
사용자 판단: {pending_decision_summary|none}
리스크: {risk_level}
증거: {evidence_state}
설계: {design_alignment_state} / {architecture_state}
QA: {manual_qa_state}
최신 보고: {latest_report|none}
````

### Approval card

````text
승인이 필요합니다.

{approval_id} {category}
요청: {summary}
목적: {why_needed}
허용 경로:
{allowed_paths}

허용 도구:
{allowed_tools}

network:
{allowed_network}

required secrets:
{required_secrets}

baseline:
{baseline_ref}

리스크:
{risks}

대안:
{alternatives}

권장:
{recommendation}

승인하시겠습니까?
````

### Verification result card

````text
검증 완료.

{eval_id}
verdict: {verdict}
assurance: {assurance_impact}
verification independence: {verification_independence}
manual QA: {manual_qa_impact}
acceptance: {acceptance_impact}

검토한 증거:
- task summary: {task_summary_ref}
- run summary: {run_summary_ref}
- evidence manifest: {evidence_manifest_ref}
- TDD trace: {tdd_trace_ref}
- diff: {diff_ref}
- logs: {logs_ref}
- approvals: {approval_refs}
- design refs: {design_refs}

남은 작업:
{blockers_or_rework}

사용자 확인:
{user_followup}
````

### Manual QA card

````text
manual QA가 필요합니다.

{manual_qa_id}
profile: {profile}
대상: {screen_or_flow}
확인할 항목:
- {checklist_item}

증거로 남길 것:
- screenshot 또는 walkthrough note
- browser log 필요 시

QA 결과를 기록하시겠습니까?
````

## 23. AGENTS.md 최소 템플릿

````md
# AGENTS.md

## Repository Summary
- 이 저장소의 목적:
- 주요 실행 경로:
- 주의할 모듈:

## Harness Rule
제품 코드 변경, 검증, 승인, QA, 재개, 완료 판단을 다룰 때는 Harness를 사용한다.

## Working Rules
- 현재 active TASK를 먼저 읽는다.
- 작은 저위험 변경은 direct로 처리한다.
- 기능 추가, 구조 변경, 비국소 수정은 work로 처리한다.
- work는 shared design 질문으로 시작한다.
- 하나의 구현 run은 하나의 Change Unit만 다룬다.
- 기능 Change Unit은 vertical slice를 기본값으로 둔다.
- 제품 파일 쓰기 전 Harness MCP를 통해 scope와 approval을 확인한다.
- 허용 범위를 넘으면 approval 또는 decision으로 분리한다.
- 민감 변경은 approval 없이 진행하지 않는다.
- 가능한 경우 TDD trace를 남긴다.
- detached verify 없이 work를 닫지 않는다.
- manual QA가 필요한 작업은 QA 상태를 남긴다.
- 대화 기억보다 Harness 상태, 문서, evidence를 우선한다.
- 오래된 PRD/DESIGN은 기본 context에 push하지 않는다.

## Default Checks
- lint:
- test:
- build:
````

## 24. Harness Skill 최소 템플릿

````md
---
name: harness
description: Use this when the user asks to modify code, verify work, resume a task, request approval, perform QA, close a task, inspect project work state, or record a development decision.
---

# Harness Skill

## Purpose
Use Harness to keep AI-assisted development visible, bounded, evidenced, verifiable, and aligned with good software design.

## Core rule
Before changing product files, call the Harness MCP server.

## Workflow

### 1. Status or intake
- If the user asks for status, call `harness.status`.
- If the user asks for a new task, call `harness.intake`.
- If the user asks to resume, call `harness.status` and `harness.next`.

### 2. Classify
- `advisor`: explanation, comparison, review, decision draft
- `direct`: small, low-risk, clear check
- `work`: feature, structural change, multi-file fix, refactor, high-risk change

### 3. Shape work
- Ask one question at a time when requirements are ambiguous.
- Provide recommendation and trade-off for each question.
- Record decisions, assumptions, rejected options, scope, acceptance criteria.
- Check domain language and module/interface impact.
- Propose Change Units, preferring vertical slices.

### 4. Before writing
- Call `harness.prepare_write`.
- Respect allowed paths and tools.
- Stop if approval is required.
- Request approval through `harness.request_user_decision`.

### 5. During implementation
- Prefer TDD when suitable.
- Keep feedback loops short.
- Avoid changing outside the active Change Unit.

### 6. After changing
- Call `harness.record_run` with changed files, commands, logs, diff, TDD trace, evidence mapping, design updates.

### 7. Finish
- For work verification, call `harness.launch_verify` or record the fresh evaluator result through `harness.record_eval`.
- For manual QA, call `harness.record_manual_qa`.
- Record user decisions through `harness.record_user_decision`.
- Call `harness.close_task` after verification, required manual QA, and required acceptance are resolved.
````

## 25. 템플릿 운영 규칙

1. 템플릿은 canonical state contract와 같은 의미를 사용한다.
2. Front matter는 최소로 유지한다.
3. 본문 summary가 사용자에게 먼저 읽히게 한다.
4. `TASK`의 오래된 세부 이력은 report와 decision 문서로 내린다.
5. Managed 영역과 human-editable 영역을 구분한다.
6. Approval, manual QA, acceptance를 섞지 않는다.
7. `RUN-SUMMARY`, `TDD-TRACE`, `MANUAL-QA`, `EVAL`, `EVIDENCE-MANIFEST`는 서로 참조한다.
8. Rule/context 파일은 짧게 유지한다.
9. Skill은 하네스를 사용하는 절차를 담는다.
10. MCP tool 이름과 의미가 바뀌면 Skill과 surface template을 함께 갱신한다.
11. 템플릿 변경은 projection 규격 변경으로 간주하고 버전 관리한다.
12. Domain language와 module map은 제품별 문서이므로 reconcile로 관리한다.

