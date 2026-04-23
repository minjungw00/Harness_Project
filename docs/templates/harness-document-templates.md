# 하네스 문서 템플릿 v04

## 1. 문서 목적

이 문서는 저장소 안에 남는 Markdown 문서와 공통 projection 파일의 기본 템플릿을 정의한다.

이 문서는 다음을 다룬다.

- 문서별 목적
- 최소 front matter
- 사람이 읽는 본문 구조
- managed 영역과 human-editable 영역
- `TASK` 문서의 Current Summary + Rolling Spine 구조
- Evidence Manifest 템플릿
- report 문서와 approval/decision/design 문서의 기본 템플릿
- `AGENTS.md` 최소 템플릿
- 공통 상태 카드 템플릿

이 문서는 다음을 직접 다루지 않는다.

- authoritative state schema
- DB 테이블 정의
- approval, validator, adapter 구현 로직
- 사용자 절차의 전체 설명
- 외부 제품별 실제 설정 파일 위치
- 표면별 connector template 전문

front matter는 projection이다. authoritative 운영 상태는 상태 저장소가 가진다.

## 2. 문서 projection 원칙

1. 문서는 사람이 읽는 표면이다.
2. front matter는 최소한의 식별과 projection version만 담는다.
3. 현재 상태를 이해하는 데 필요한 정보는 본문 summary에 둔다.
4. raw logs, large trace, patch 전문은 문서에 직접 넣지 않고 artifact로 참조한다.
5. `TASK`는 현재를 이해하는 중심 문서이고, 상세 이력은 `RUN-SUMMARY`, `EVAL`, `APR`, `DEC`, `EVIDENCE-MANIFEST`로 넘긴다.
6. resolved item을 계속 쌓아 `TASK`를 거대 문서로 만들지 않는다.
7. `Current Summary`와 `Rolling Spine`은 compact하게 유지한다.
8. approval, assurance, acceptance를 같은 결정으로 섞지 않는다.
9. 문서의 최신성은 `projection_version`과 `updated_at`으로 표시한다.
10. 항상 읽히는 agent rule 파일은 짧고 안정적인 원칙만 담는다.
11. 긴 절차는 Skill이나 playbook으로 분리한다.
12. Skill과 rule은 정책 집행 장치가 아니라 에이전트 안내 문서다.

## 3. managed 영역과 human-editable 영역

### 3.1 managed 영역

하네스 projector가 생성하고 갱신하는 영역이다.

```md
<!-- HARNESS:BEGIN managed -->
...
<!-- HARNESS:END managed -->
```

사용자는 이 영역을 읽을 수 있지만 직접 수정한 내용은 자동으로 운영 상태가 되지 않는다.

### 3.2 human-editable 영역

사용자가 직접 메모, 제안, 질문을 남길 수 있는 영역이다.

```md
## User Notes and Proposals
- 
```

Projector는 이 영역을 보존한다.

하네스는 필요한 경우 이 영역의 항목을 reconcile 대상으로 올린다.

## 4. 최소 front matter 규격

### 4.1 TASK

```yaml
---
doc_type: task
task_id: TASK-0001
display_state: verify_pending
projection_version: 18
updated_at: 2026-04-23T10:05:00+09:00
---
```

### 4.2 DEC

```yaml
---
doc_type: decision
decision_id: DEC-0001
task_id: TASK-0001
status: accepted
projection_version: 3
updated_at: 2026-04-23T10:05:00+09:00
---
```

### 4.3 APR

```yaml
---
doc_type: approval
approval_id: APR-0001
task_id: TASK-0001
category: dependency_change
status: pending
projection_version: 2
updated_at: 2026-04-23T10:05:00+09:00
---
```

### 4.4 EVIDENCE-MANIFEST

```yaml
---
doc_type: evidence_manifest
evidence_manifest_id: EM-0001
task_id: TASK-0001
change_unit_id: CU-01
status: sufficient
projection_version: 4
updated_at: 2026-04-23T10:05:00+09:00
---
```

### 4.5 RUN-SUMMARY

```yaml
---
doc_type: run_summary
run_id: RUN-20260423-093015-LEAD-01
task_id: TASK-0001
change_unit_id: CU-01
profile: lead
action: implement
surface_id: reference_local_mcp
projection_version: 2
updated_at: 2026-04-23T10:05:00+09:00
---
```

### 4.6 EVAL

```yaml
---
doc_type: eval
eval_id: EVAL-0001
task_id: TASK-0001
change_unit_id: CU-01
verdict: passed
surface_id: reference_local_mcp
projection_version: 2
updated_at: 2026-04-23T10:05:00+09:00
---
```

### 4.7 DIRECT-RESULT

```yaml
---
doc_type: direct_result
direct_result_id: DIR-0001
task_id: TASK-0001
result: passed
assurance_level: self_checked
surface_id: reference_local_mcp
projection_version: 1
updated_at: 2026-04-23T10:05:00+09:00
---
```

### 4.8 DESIGN

```yaml
---
doc_type: design
design_id: DESIGN-0001
task_id: TASK-0001
status: draft
projection_version: 1
updated_at: 2026-04-23T10:05:00+09:00
---
```

## 5. TASK 템플릿

`TASK`는 모든 이력을 장문으로 누적하는 문서가 아니라, 현재 요약과 continuity를 유지하는 중심 문서다.

````md
---
doc_type: task
task_id: TASK-0001
display_state: execute_pending
projection_version: 7
updated_at: 2026-04-23T09:30:15+09:00
---

# TASK-0001 작업 제목

<!-- HARNESS:BEGIN managed -->
## Current Summary
- mode:
- phase:
- result:
- assurance:
- verification independence:
- approval:
- acceptance:
- risk:
- evidence:
- active change unit:
- next action:
- pending decision:
- latest report:
- latest approval:
- latest decision:
- latest evidence manifest:
- projection freshness:

## Goal
- 해결하려는 문제
- 기대 결과

## Scope
### In
- 항목

### Out
- 항목

## Acceptance Criteria
- [ ] AC-01:
- [ ] AC-02:

## Active Change Units
| ID | 목적 | 상태 | 허용 경로 | 핵심 검증 |
|---|---|---|---|---|
| CU-01 | | | | |

## Rolling Spine
### Facts in Force
- 사실 / evidence ref:

### Assumptions in Force
- 가정 / 만료 조건:

### Decisions in Force
- DEC-0001: 요약

### Rejected Options
- 버린 선택지 / 이유 / 관련 DEC:

### Watchpoints
- 회귀 가능성:
- 보안/성능/운영 주의점:

### Resume Notes
- 다음 세션이 바로 알아야 할 점:
- 지금 막히는 지점:

## Open Decisions
### 항목 1
- 선택지:
  - 옵션 A
  - 옵션 B
- 권장안:
- 판단 근거:
- 관련 문서:

## Approval and Acceptance
### Approval
- current approval state:
- latest APR:
- scope note:

### Acceptance
- current acceptance state:
- pending trade-off:
- user decision needed:

## Evidence and References
- Evidence Manifest:
- Decision:
- Approval:
- Design:
- Run Summary:
- Eval:
- Direct Result:
- Bundle:
- Logs:
- Diff:
- Checkpoint:

## Snapshot References
- RUN-...
- EVAL-...
- APR-...
<!-- HARNESS:END managed -->

## User Notes and Proposals
- 
````

### TASK 작성 규칙

- `Current Summary`는 가장 먼저 읽히는 블록이다.
- `Rolling Spine`에는 현재 유효한 항목만 남긴다.
- 오래된 세부 이력은 `Snapshot References`와 관련 report 문서로 넘긴다.
- `Facts in Force`에는 가능한 한 evidence ref를 붙인다.
- `Assumptions in Force`에는 만료 조건을 붙인다.
- `Open Decisions`는 사용자가 실제로 결정해야 하는 항목만 둔다.
- approval과 acceptance를 같은 결정으로 섞지 않는다.
- `User Notes and Proposals`는 projector가 보존한다.
- managed 영역 직접 수정은 reconcile 대상으로 본다.

## 6. Change Unit 블록 템플릿

````md
### CU-01 제목
- 목적:
- 비목표:
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
  - evidence_sufficiency
  - test
- approval categories:
  - none
- 완료 조건:
  - [ ]
- evaluator focus:
  - 항목
````

## 7. EVIDENCE-MANIFEST 템플릿

````md
---
doc_type: evidence_manifest
evidence_manifest_id: EM-0001
task_id: TASK-0001
change_unit_id: CU-01
status: partial
projection_version: 2
updated_at: 2026-04-23T09:50:00+09:00
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
| AC-01 | | supported | test:, log:, diff: | |
| AC-02 | | unsupported | | |

## Changed File Coverage
| Path | Covered Criteria | Evidence Refs |
|---|---|---|
| `src/...` | AC-01 | DIFF-..., TEST-... |

## Approval Refs
- APR-...

## Evidence Refs
- run summary:
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

## Validator Notes
- evidence_sufficiency:
- bundle_integrity:
- acceptance_review:
````

## 8. DEC 템플릿

````md
---
doc_type: decision
decision_id: DEC-0001
task_id: TASK-0001
status: proposed
projection_version: 1
updated_at: 2026-04-23T09:30:15+09:00
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

## 9. APR 템플릿

`APR`은 승인 요청의 사람용 요약 문서다.

````md
---
doc_type: approval
approval_id: APR-0001
task_id: TASK-0001
category: dependency_change
status: pending
projection_version: 1
updated_at: 2026-04-23T09:30:15+09:00
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

### APR 작성 규칙

- approval은 범주만 적지 않고 scope를 함께 적는다.
- scope가 바뀌면 기존 APR을 조용히 재사용하지 않는다.
- approval은 verification이나 acceptance를 대신하지 않는다.
- user decision note는 상태 전이에 필요한 최소 근거를 남긴다.

## 10. DESIGN 템플릿

````md
---
doc_type: design
design_id: DESIGN-0001
task_id: TASK-0001
status: draft
projection_version: 1
updated_at: 2026-04-23T09:30:15+09:00
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

## Proposed Shape
- 구성 요소
- 경계와 책임
- 데이터 흐름

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
- 어떤 기준으로 성공을 판단하는가
- 어떤 회귀를 경계하는가
- 어떤 evidence가 필요할 것인가

## References
- TASK:
- DEC:
- APR:
- EVIDENCE-MANIFEST:
````

## 11. RUN-SUMMARY 템플릿

````md
---
doc_type: run_summary
run_id: RUN-20260423-093015-LEAD-01
task_id: TASK-0001
change_unit_id: CU-01
profile: lead
action: implement
surface_id: reference_local_mcp
projection_version: 1
updated_at: 2026-04-23T09:45:10+09:00
---

# RUN-SUMMARY

## Run Identity
- run_id:
- profile:
- action:
- surface:
- mcp_session_ref:
- baseline_ref:
- state_version:

## Scope
- task_id:
- change_unit_id:
- allowed paths:
- allowed tools:
- approval refs:

## Changed Files
- `path/to/file`

## Commands and Checks
```bash
pnpm test
```

## Outcomes
- changed_paths:
- approval_scope:
- lint:
- test:
- build:
- docs_consistency:
- evidence_sufficiency:

## Key Changes
- 항목

## Issues and Follow-ups
- 항목

## Spine Updates
- 새로 확정된 사실:
- 새로 버린 선택지:
- watchpoint 변경:
- 다음 run이 알아야 할 점:

## Evidence Refs
- evidence manifest:
- diff:
- logs:
- bundle:
- checkpoint:
````

## 12. EVAL 템플릿

````md
---
doc_type: eval
eval_id: EVAL-0001
task_id: TASK-0001
change_unit_id: CU-01
verdict: passed
surface_id: reference_local_mcp
projection_version: 1
updated_at: 2026-04-23T10:05:00+09:00
---

# EVAL-0001 제목

## Verdict
- verdict:
- assurance impact:
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
- [ ] lint
- [ ] test
- [ ] build
- [ ] docs_consistency

## Evidence Reviewed
- task summary:
- rolling spine:
- run summary:
- evidence manifest:
- diff:
- bundle:
- logs:
- approvals:
- decisions:

## Acceptance Criteria Review
| AC ID | Statement | Evidence Reviewed | Result | Notes |
|---|---|---|---|---|
| AC-01 | | | pass | |

## Rationale
- 왜 이 verdict인가

## Blockers or Rework
- 항목

## User Follow-up
- 확인이 필요한 trade-off
- 남은 선택지
````

## 13. DIRECT-RESULT 템플릿

````md
---
doc_type: direct_result
direct_result_id: DIR-0001
task_id: TASK-0001
result: passed
assurance_level: self_checked
surface_id: reference_local_mcp
projection_version: 1
updated_at: 2026-04-23T09:40:00+09:00
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

## 14. AGENTS.md 최소 템플릿

````md
# AGENTS.md

## Repository Summary
- 이 저장소의 목적:
- 주요 실행 경로:
- 주의할 모듈:

## Harness Rule
제품 코드 변경, 검증, 승인, 재개, 완료 판단을 다룰 때는 Harness를 사용한다.

## Read First
- 현재 active TASK
- 관련 DEC
- 관련 DESIGN
- 최신 EVAL 또는 DIRECT-RESULT
- 최신 EVIDENCE-MANIFEST

## Working Rules
- 가장 작은 유효 워크플로를 사용한다.
- 현재 active TASK를 먼저 읽는다.
- 작은 저위험 변경은 direct로 처리한다.
- 기능 추가, 구조 변경, 비국소 수정은 work로 처리한다.
- 하나의 구현 run은 하나의 Change Unit만 다룬다.
- 제품 파일 쓰기 전 Harness MCP를 통해 scope와 approval을 확인한다.
- 허용 범위를 넘으면 approval 또는 decision으로 분리한다.
- auth, schema, dependency, public API, destructive write, network write, secret access, production config, CI/CD, infra, privacy, data export, telemetry, license, billing 영향 변경은 approval 없이 진행하지 않는다.
- detached verify 없이 work를 완료로 닫지 않는다.
- direct 결과는 assurance와 함께 기록한다.
- 대화 기억보다 Harness 상태, 문서, evidence를 우선한다.
- 사람 조직 직함을 흉내 내는 하위 역할을 만들지 않는다.

## Default Checks
- lint:
- test:
- build:

## Sensitive Categories
- auth / permission
- schema / migration
- dependency change
- public API change
- destructive write
- network write
- external service write
- secret access
- production config / deployment / CI/CD / infra
- privacy / PII / data export / telemetry
- license / compliance / billing
- policy override
````

## 15. 공통 Harness Skill 템플릿

````md
---
name: harness
description: Use this when the user asks to modify code, verify work, resume a task, request approval, close a task, inspect project work state, or record a development decision.
---

# Harness Skill

## Purpose
Use Harness to keep AI-assisted development visible, bounded, evidenced, and verifiable.

## Core rule
Before changing product files, call the Harness MCP server.

## Workflow

### 1. Status or intake
- If the user asks for status, call `harness.status`.
- If the user asks for a new task, call `harness.intake`.
- If the user asks to resume, call `harness.status` and `harness.next`.

### 2. Classify
Use Harness classification:

- `advisor`: explanation, comparison, review, decision draft
- `direct`: small, low-risk, clear check
- `work`: feature, structural change, multi-file fix, refactor, high-risk change

### 3. Before writing
Before file edits or shell commands that may modify state:

- call `harness.prepare_write`
- respect allowed paths and tools
- stop if approval is required
- request approval through `harness.request_approval`

### 4. After changing
After edits or commands:

- call `harness.record_change`
- call `harness.update_evidence_manifest` when acceptance criteria are affected
- include changed files, commands, logs, diff, and summary

### 5. Finish
- For direct: call `harness.finish_direct`.
- For work implementation: call `harness.finish_implementation`.
- For work verification: call `harness.launch_verify` or record the fresh evaluator result through `harness.record_eval`.

### 6. User decisions
Use `harness.user_decision` for approval, denial, acceptance, rejection, scope confirmation, and reconcile decision.

### 7. Completion
Do not close a work task unless Harness says completion conditions are satisfied.

Call `harness.close_task` only after verification and required acceptance are resolved.

## User reporting format
Always show a short status card:

```text
TASK-XXXX title
state: mode / phase
next action:
pending decision:
risk:
evidence:
latest report:
```

## Do not
- Do not ask the user to compose Harness CLI commands during normal work.
- Do not treat same-session self-review as detached verification.
- Do not proceed with sensitive changes without approval.
- Do not rely on chat history over Harness state and evidence.
````

## 16. MCP status card template

### 16.1 Compact card

````text
TASK-{id} {title}
상태: {mode} / {phase}
다음 행동: {next_action}
사용자 판단: {pending_decision_summary|none}
리스크: {risk_level}
증거: {evidence_state}
최신 보고: {latest_report|none}
````

### 16.2 Detailed card

````text
TASK-{id} {title}
mode: {advisor|direct|work}
phase: {phase}
result: {result}
assurance: {assurance_level}
verification independence: {qualifier|none}
approval: {approval_state}
acceptance: {acceptance_state}
risk: {risk_level}
evidence: {evidence_state}
active change unit: {change_unit_id|none}
next action: {next_action}
pending decision: {pending_decision_summary|none}
latest report: {latest_report|none}
latest approval: {latest_approval|none}
latest evidence manifest: {latest_evidence_manifest|none}
projection: {current|stale|failed}
````

## 17. MCP approval card template

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

## 18. MCP verification result card template

````text
검증 완료.

{eval_id}
verdict: {verdict}
assurance: {assurance_impact}
verification independence: {verification_independence}
acceptance: {acceptance_impact}

검토한 증거:
- task summary: {task_summary_ref}
- run summary: {run_summary_ref}
- evidence manifest: {evidence_manifest_ref}
- diff: {diff_ref}
- logs: {logs_ref}
- approvals: {approval_refs}

남은 작업:
{blockers_or_rework}

사용자 확인:
{user_followup}
````

## 19. 템플릿 운영 규칙

1. 템플릿은 canonical state contract와 같은 의미를 사용한다.
2. front matter는 최소한으로 유지한다.
3. 본문 summary가 사용자에게 먼저 읽히게 한다.
4. `TASK`의 오래된 세부 이력은 report와 decision 문서로 내린다.
5. `TASK`의 managed 영역과 human-editable 영역을 구분한다.
6. approval과 acceptance는 섞지 않는다.
7. `RUN-SUMMARY`, `EVAL`, `EVIDENCE-MANIFEST`는 서로 참조한다.
8. rule/context 파일은 짧게 유지한다.
9. Skill은 하네스를 사용하는 절차를 담는다.
10. MCP tool 이름과 의미가 바뀌면 Skill과 surface template을 함께 갱신한다.
11. 템플릿 변경은 projection 규격 변경으로 간주하고 버전 관리한다.
