# 04. Reference Implementation

## 1. 문서 역할

이 문서는 하네스 전략과 아키텍처를 실제 시스템으로 구현하기 위한 참조 계약을 정의한다.

다룬다:

- 구현 원칙
- MVP와 후속 범위
- 저장 모델
- Core 내부 모듈
- canonical state contract
- 상태 전이 불변식
- public MCP surface
- approval, evidence, artifact, verification, design-quality 구현 계약
- validator runner
- security boundary
- adapter, hook, sidecar의 구현 단계
- recovery와 failure semantics

사용자 대화 예시, 표면별 설정 파일 위치, Markdown 템플릿 전문, 운영자 명령 상세는 전용 문서가 소유한다.

## 2. 구현 원칙

1. 코어 불변식을 먼저 구현한다.
2. agent surface 수보다 state, evidence, approval, verification 집행을 우선한다.
3. 일상 조작 API는 Harness MCP server로 제공한다.
4. 상태 변경은 idempotent하고 감사 가능해야 한다.
5. 저장소 문서는 사람이 읽는 projection으로 유지한다.
6. generated file과 사람이 편집한 내용은 reconcile로 분리한다.
7. detached verification은 실행자 자기 보고와 구분한다.
8. surface capability는 제품명으로 가정하지 않고 profile로 검증한다.
9. `work / shaping`은 shared design concept을 만들 수 있어야 한다.
10. Change Unit은 vertical slice 기본값과 예외 기록을 지원한다.
11. TDD trace, module/interface review, manual QA는 evidence와 completion 조건에 연결된다.

참조 구현의 첫 목표는 다음 문장을 실제로 강제하는 것이다.

```text
제품 파일 쓰기 전 scope와 approval을 확인한다.
변경 후 evidence를 남긴다.
work 작업을 실행자의 자기 보고만으로 닫지 못하게 한다.
AI 구현 단위가 설계 품질과 feedback loop를 우회하지 못하게 한다.
```

## 3. 구현 우선순위

### 3.1 MVP

MVP는 agent surface 통합 프로젝트가 아니라 core invariant 검증 프로젝트다.

MVP 범위:

- 단일 로컬 프로젝트 등록
- 하나의 reference agent surface 연결
- Harness MCP server
- `state.sqlite`와 append-only event log
- artifact registry
- baseline capture
- projection outbox와 freshness 표시
- document reconcile 감지
- public MCP tools
- `TASK`, `APR`, `RUN-SUMMARY`, `EVAL`, `DIRECT-RESULT`, `EVIDENCE-MANIFEST` projection
- `DOMAIN-LANGUAGE`, `MODULE-MAP`, `INTERFACE-CONTRACT`, `TDD-TRACE`, `MANUAL-QA` 최소 projection
- `changed_paths`, `approval_scope`, `evidence_sufficiency`, `same_session_verify_guard`, `docs_consistency` validator
- `vertical_slice_shape`, `tdd_trace`, `module_boundary_review`, `manual_qa_required` 최소 validator
- detached verification bundle 생성
- fresh evaluator run 또는 manual evaluator instruction bundle
- setup, doctor, reconcile, recover, export 최소 CLI

MVP 범위 밖:

- 모든 agent surface connector 완성
- cross-surface orchestration
- dashboard
- team profile export/import
- 복잡한 multi-agent policy
- 장기 analytics
- 완전 자동 병렬 실행
- browser QA artifact 자동 캡처

### 3.2 v1 stable

v1 stable 범위:

- approval scope drift 자동 만료
- projection outbox retry/reconcile 안정화
- bundle integrity validator
- acceptance review validator
- public interface change validator
- domain language consistency validator
- test boundary quality validator
- worktree 기반 fresh verify
- reference surface adapter의 capture와 guard 안정화
- 두 번째 agent surface connector
- conformance suite 자동화
- generated file drift repair
- derived metrics export

### 3.3 later

후속 범위:

- native hook 확장
- sidecar file watcher 고도화
- browser QA capture
- cross-surface verify
- artifact dashboard
- connector profile 확장
- team profile export/import
- status/approval/acceptance/manual QA card UX 개선
- Change Unit DAG 기반 병렬 후보 추천

## 4. 저장 모델

기본 runtime home은 `~/.harness`다.

```text
~/.harness/
  config.yaml
  registry.sqlite
  projects/
    PRJ-0001/
      project.yaml
      state.sqlite
      artifacts/
        bundles/
        diffs/
        logs/
        manifests/
        tdd/
        qa/
        exports/
```

Source-of-truth 분리:

| 정보 | canonical source |
|---|---|
| 프로젝트 등록과 surface 연결 | `registry.sqlite` |
| 정적 프로젝트 설정 | `project.yaml` |
| 운영 상태와 관계 | `state.sqlite` + event log |
| raw evidence | artifact store |
| 문서 projection 상태 | projection job state |
| connector 생성물 | connector registry + generated manifest |

Markdown 문서는 사람이 읽는 projection이다.

## 5. transaction 모델

모든 상태 변경은 하나의 SQLite transaction 안에서 다음 순서로 처리한다.

```text
1. current state 갱신
2. 관련 event append
3. projection_version 증가
4. projection job enqueue
```

Markdown projection은 transaction 뒤에 비동기로 수행한다.

State-changing request는 다음을 지원한다.

- `request_id`
- `idempotency_key`
- `expected_state_version`
- replay 감지
- conflict 반환

`expected_state_version`이 맞지 않으면 `STATE_CONFLICT`를 반환한다.

## 6. Core 내부 모듈

MVP 구현은 하나의 프로세스 안에서 다음 내부 모듈을 가진다.

| 모듈 | 책임 |
|---|---|
| state store | tasks, change_units, runs, events, locks, projection jobs |
| task workflow | intake, mode 분류, next action, state transition |
| approval module | APR, decision, expiry, scope drift |
| design-quality module | shared design, domain language, module map, interface contract, TDD, manual QA |
| evidence module | run summary, artifact registry, evidence manifest |
| verification module | bundle, evaluator launch, EVAL, independence qualifier |
| projection module | Markdown projection, managed block, freshness |
| reconcile module | human proposal, managed drift, generated drift |
| validator runner | core, design, connector validator 실행 |
| MCP facade | resources, prompts, public tools |
| connector/adapter layer | surface profile, capture, guard, generated file 관리 |
| recovery module | crash, stale, missing artifact, projection failure 복구 |

## 7. `project.yaml`

`project.yaml`은 정적 프로젝트 설정만 가진다. 현재 Task 상태는 담지 않는다.

```yaml
project_id: PRJ-0001
display_name: my-app
repo_root: /Users/me/work/my-app
default_agent_surface: reference

agent_surfaces:
  reference:
    enabled: true
    capability_profile_id: SURF-PROFILE-0001

default_checks:
  lint:
    - pnpm lint
  test:
    - pnpm test --runInBand
  build:
    - pnpm build

design_quality:
  vertical_slice_default: true
  tdd_required_for:
    - domain_logic
    - service_module
    - bug_fix
    - parser_validator
    - state_transition
    - deep_module_internal
  manual_qa_default_for:
    - ui
    - ux
    - copy
    - accessibility
    - visual_output

network_policy:
  default_write: deny
  allowed_read_domains: []
  allowed_write_targets: []

secret_policy:
  env_allowlist: []
  allow_secret_access_without_approval: false
```

## 8. canonical state contract

Task state:

```yaml
mode: advisor | direct | work
phase: intake | shaping | ready | executing | verifying | qa | waiting_user | blocked | completed | cancelled
result: none | advice_only | passed | failed | cancelled
assurance_level: none | self_checked | detached_verified
approval_state: not_required | pending | granted | denied | expired
manual_qa_state: none | pending | passed | failed | waived
acceptance_state: not_requested | pending | accepted | rejected
risk_level: low | medium | high
evidence_state: none | partial | sufficient | stale
design_alignment_state: none | partial | aligned | stale
architecture_state: none | review_required | reviewed | drift_detected
```

Operational fields:

```yaml
state_version: integer
current_change_unit_id: optional string
active_run_id: optional string
active_surface_id: optional string
next_action: string
pending_decision_summary: optional string
latest_report_refs: string[]
latest_approval_id: optional string
latest_evidence_manifest_id: optional string
latest_tdd_trace_id: optional string
latest_manual_qa_id: optional string
projection_version: integer
projected_version: integer
last_baseline_ref: optional string
repo_task_doc_path: optional string
```

`display_state`는 canonical fields에서 파생한다.

## 9. Change Unit contract

```yaml
change_unit_id: CU-01
task_id: TASK-0001
title: string
purpose: string
non_goals: string[]
slice_type: vertical | enabling | cleanup | horizontal_exception
horizontal_exception_reason: optional string
follow_up_vertical_cu: optional string
end_to_end_path:
  trigger_or_input: optional string
  domain_logic: optional string
  persistence: optional string
  api_or_caller: optional string
  ui_or_observable_output: optional string
allowed_paths: string[]
allowed_tools: string[]
validator_profile: string[]
approval_categories: string[]
tdd: required | recommended | optional | waived
manual_qa_required: true | false
manual_qa_profile: optional string
blocked_by: string[]
unblocks: string[]
parallelizable_with: string[]
merge_conflict_risk: low | medium | high
completion_conditions: string[]
evaluator_focus: string[]
```

Change Unit이 하나뿐이면 dependency fields는 사용자 compact card에서 생략할 수 있다.

## 10. verification independence

```yaml
verification_independence:
  context: same_session | subagent_context | fresh_session | fresh_worktree | sandbox
  write_capable: true | false
  product_file_write_allowed: true | false
  source_input: chat_history | task_summary | bundle | raw_artifacts
  baseline_reverified: true | false
  evaluator_surface_id: string
  parent_run_id: optional string
```

사용자 표면의 assurance는 `none`, `self_checked`, `detached_verified`로 단순하게 유지한다. 운영 판정은 independence qualifier를 함께 사용한다.

## 11. 상태 전이와 completion 조건

기본 전이:

| 조건 | 다음 상태 |
|---|---|
| 새 요청 intake | `phase=intake` |
| 설명/비교 중심 | `mode=advisor`, `phase=executing` |
| 작은 저위험 변경 | `mode=direct`, `phase=ready` |
| 구조화 필요 | `mode=work`, `phase=shaping` |
| work shaping 완료 | `phase=ready` |
| direct 변경과 self-check 완료 | `phase=completed`, `result=passed`, `assurance_level=self_checked` |
| direct 범위 확장 | 같은 Task에서 `mode=work`, `phase=shaping` |
| work 구현 완료 | `phase=verifying` |
| detached verify passed, QA 필요 | `phase=qa`, `manual_qa_state=pending` |
| detached verify passed, 사용자 판단 남음 | `phase=waiting_user`, `acceptance_state=pending` |
| QA failed | `phase=shaping` 또는 `phase=executing` |
| acceptance accepted | `phase=completed`, `result=passed` |
| evidence 부족 | `phase=blocked`, `evidence_state=partial` 또는 `stale` |

`work` Task를 닫으려면 다음 조건이 필요하다.

- active Change Unit 완료 또는 명시적 보류
- required approval이 granted이고 scope drift 없음
- `design_alignment_state=aligned` 또는 accepted waiver
- `evidence_state=sufficient`
- required TDD trace recorded 또는 accepted waiver
- detached verification passed 또는 accepted exception
- manual QA가 `none`, `passed`, 또는 `waived`
- `acceptance_state=not_requested` 또는 `accepted`
- projection stale/failed 상태가 사용자에게 표시됨

## 12. Harness MCP server 계약

MCP resource는 read-only다. 상태 변경은 tool로 수행한다.

Resources:

```text
harness://project/current
harness://project/surfaces
harness://task/active
harness://task/{task_id}
harness://task/{task_id}/summary
harness://task/{task_id}/spine
harness://task/{task_id}/reports/latest
harness://task/{task_id}/evidence-manifest
harness://task/{task_id}/bundle/current
harness://design/domain-language
harness://design/module-map
harness://design/interface-contracts
harness://policy/sensitive-categories
harness://status/card
```

Public tools:

| Tool | 책임 |
|---|---|
| `harness.status` | 현재 프로젝트와 active Task 상태 카드 반환 |
| `harness.intake` | 사용자 요청을 받아 Task 생성/재개, mode 분류 |
| `harness.next` | 현재 Task의 다음 행동과 instruction bundle 반환 |
| `harness.prepare_write` | 제품 파일 쓰기 전 scope, approval, baseline, design guard 확인 |
| `harness.record_run` | design update, changed files, commands, logs, TDD trace, evidence mapping, run summary 기록 |
| `harness.request_user_decision` | approval, scope, unresolved decision, QA, acceptance, reconcile 판단 요청 생성 |
| `harness.record_user_decision` | approval/denial, scope confirmation, acceptance/rejection, QA waiver, reconcile decision 기록 |
| `harness.launch_verify` | fresh evaluator 또는 verification bundle 생성 |
| `harness.record_eval` | 검증 결과와 EVAL 생성 |
| `harness.record_manual_qa` | manual QA 결과와 artifact/note 기록 |
| `harness.close_task` | completion 조건 확인 후 Task close |

State-changing tool은 `request_id`, `idempotency_key`, `expected_state_version`, `surface_id`, `run_id`, `actor_kind`, `dry_run`을 공통 envelope로 가진다.

## 13. approval 구현 계약

Approval category:

```text
auth_change
permission_model_change
schema_change
dependency_change
public_api_change
destructive_write
network_write
external_service_write
secret_access
production_config_change
ci_cd_change
infra_or_deployment_change
privacy_or_pii_change
data_export
telemetry_or_logging_change
license_or_compliance_change
billing_or_cost_change
model_or_prompt_policy_change
policy_override
```

Approval record는 변경 요약, allowed paths, allowed tools, allowed network target, required secret scope, baseline ref, expected diff envelope, expires-on-drift 조건, alternatives, recommendation, user decision note를 가진다.

## 14. design-quality 구현 계약

Shared design record는 질문, recommendation, trade-off, user answer, assumptions, rejected options, scope, acceptance criteria를 가진다.

Domain language record는 term, meaning, code representation, not-this, related terms, source, status를 가진다.

Module map record는 name, role, public interface, internal complexity, dependencies, test boundary, owner decision을 가진다.

Interface contract record는 change type, public interface, inputs, outputs, errors, compatibility impact, callers impacted, tests at boundary, review status를 가진다.

TDD trace record는 red, green, refactor, non-TDD justification, status를 가진다. TDD required 작업에서 red evidence가 없으면 validator는 partial 또는 failed를 반환한다.

Manual QA record는 profile, required, performed_by, result, artifacts, findings, waiver_reason, next_action을 가진다. Manual QA가 required이고 result가 passed 또는 waived가 아니면 close는 차단된다.

## 15. evidence와 artifact 구현 계약

Evidence Manifest는 acceptance criteria와 evidence를 연결한다.

```yaml
evidence_manifest_id: EM-0001
task_id: TASK-0001
change_unit_id: CU-01
baseline_ref: BASE-0001
acceptance_criteria:
  - id: AC-01
    statement: string
    status: supported | unsupported | not_applicable
    supporting_evidence:
      - kind: test | log | diff | tdd_trace | manual_qa | run_summary | eval
        ref: string
changed_files:
  - path: src/auth/LoginForm.tsx
    covered_by: [AC-01]
approvals: [APR-0003]
stale_if:
  - baseline_head_changes
  - changed_files_modified_after_eval
  - approval_scope_expires
  - domain_language_changes
  - interface_contract_changes
```

Artifact registry는 raw evidence의 canonical source를 관리한다. Artifact record는 `artifact_id`, `task_id`, `run_id`, `kind`, `path`, `sha256`, `size_bytes`, `content_type`, `retention_class`, `created_at`, `redaction_state`를 가진다. Raw secret은 artifact에 저장하지 않는다.

## 16. bundle 계약

Evaluator와 후속 run은 구조화된 bundle을 기본 입력으로 사용한다.

Bundle 최소 내용:

- task id
- mode, phase, result
- active change unit
- acceptance criteria snapshot
- task summary snapshot
- rolling spine snapshot
- domain language refs
- module map refs
- interface contract refs
- allowed paths
- changed files
- baseline ref
- run summary ref
- evidence manifest ref
- TDD trace ref
- manual QA requirement
- diff ref
- command results and logs
- related decisions
- related approvals
- known issues
- next checks
- surface capability metadata

## 17. 상태 저장 스키마

`registry.sqlite` 핵심 테이블:

```text
projects
project_surfaces
connector_manifests
```

`state.sqlite` 핵심 테이블:

```text
tasks
change_units
runs
approvals
evidence_manifests
decisions
design_contexts
domain_terms
module_map_items
interface_contracts
tdd_traces
manual_qa_records
artifacts
validator_runs
task_events
projection_jobs
reconcile_items
doc_refs
mcp_sessions
locks
```

운영 지표는 event와 run record에서 계산한다. 집계값은 새로운 authority가 아니다.

## 18. validator runner

MVP validators:

| Validator | 책임 |
|---|---|
| `changed_paths` | allowed paths 밖 수정과 evaluator write 감지 |
| `approval_scope` | 변경 파일, 도구, network, secret scope가 approval 범위 안인지 확인 |
| `evidence_sufficiency` | AC snapshot과 evidence ref 대응 확인 |
| `same_session_verify_guard` | self-review를 detached verify로 승격하지 못하게 함 |
| `docs_consistency` | projection version, latest ref, managed hash 확인 |
| `vertical_slice_shape` | vertical slice 또는 exception reason 확인 |
| `tdd_trace` | required TDD의 red/green/refactor evidence 또는 waiver 확인 |
| `module_boundary_review` | module boundary와 test boundary 검토 여부 확인 |
| `manual_qa_required` | QA required 작업의 QA state 확인 |
| `baseline_freshness` | baseline 이후 repo drift 확인 |
| `lint` | 프로젝트 lint 실행 |
| `test` | 프로젝트 test 실행 |
| `build` | 프로젝트 build 실행 |

v1 validators는 `bundle_integrity`, `acceptance_review`, `public_interface_change_review`, `domain_language_consistency`, `test_boundary_quality`, `architecture_drift`, `diff_surface_vs_scope`, `surface_capability_check`, `reconcile_required`를 추가한다.

Validator 실패는 자연어 보고로 숨기지 않고 상태 전이에 반영한다.

## 19. security boundary

하네스는 다음 경계를 집행한다.

- filesystem scope: allowed paths, evaluator read-only, write-capable run 구분
- process scope: shell command policy, tool allowlist, worktree 또는 process isolation
- network scope: 기본 network write deny, approval과 allowlist 필요
- credential scope: env allowlist, secret handle만 기록, raw secret 저장 금지
- data/privacy scope: PII, data export, telemetry/logging 변경 approval 필요
- untrusted input: 사용자 입력, 저장소 문서, 외부 검색, tool output, agent memory, generated files는 모두 검증 대상

Prompt injection 대응은 prompt 문구보다 policy engine과 tool boundary에서 집행한다.

## 20. adapter, hook, sidecar 단계

MVP:

- `prepare_write`
- SQLite state
- git diff 기반 changed_paths validator
- approval_scope validator
- artifact registry
- manual verify bundle
- 하나의 reference surface

v1:

- sidecar file watcher
- command wrapper
- generated file drift detection
- worktree verify
- bundle integrity

later:

- 표면별 native hook
- browser QA capture
- cross-surface verify
- dashboard
- parallel Change Unit execution

Adapter capability는 profile로 선언한다.

```yaml
supports_project_rules: true
supports_skills: true
supports_mcp_tools: true
supports_mcp_resources: true
supports_structured_output: false
supports_hooks: false
supports_pre_tool_guard: false
supports_fresh_verify: true
supports_worktree_isolation: false
supports_local_sidecar: true
supports_browser_qa_capture: false
```

## 21. recovery와 failure semantics

| 시나리오 | 처리 |
|---|---|
| write 중 agent crash | active run interrupted, diff/log snapshot artifact 등록 |
| approval granted 후 baseline drift | approval_state expired 후보, 재확인 요구 |
| evaluator 중 repo drift | evidence_state stale, EVAL verdict blocked |
| TASK managed 영역 직접 수정 | reconcile item 생성, 자동 state 변경 금지 |
| User Notes에 제안 추가 | pending decision 또는 user observation 승격 가능 |
| state와 artifact directory 불일치 | artifact registry rescan, missing artifact stale 표시 |
| connector generated file 수동 수정 | generated drift, merge/reinstall 선택 요구 |
| projection job 실패 | state current / projection failed 분리 표시 |
| MCP 연결 손실 | write 중단, 상태 조회는 last known state로 제한 |
| TDD required but missing red evidence | tdd_trace partial, verify warning 또는 blocked |
| manual QA required but not performed | close blocked, manual_qa pending |
| public interface change without contract | interface_review_required, work blocked |
| domain language conflict | reconcile item 또는 domain_language_mismatch |
| vertical slice shape missing | vertical_slice_required 또는 horizontal exception 요청 |

## 22. 구현 완료 기준

참조 구현은 최소 다음 시나리오를 통과해야 한다.

- active Task 없음 상태 조회
- advisor 종료
- direct 성공
- direct에서 work 전환
- dependency 추가 요청이 approval 없이 blocked
- approval scope 밖 수정 시 fail
- work shaping에서 unresolved decision이 있으면 ready 차단 가능
- vertical slice 요구가 있는 Change Unit이 horizontal이면 예외 사유 요구
- TDD required인데 red evidence가 없으면 tdd_trace partial
- public interface 변경인데 review가 없으면 blocked
- manual QA required인데 QA 결과 없으면 close blocked
- work 구현 완료 후 completed 불가, verifying 필요
- same-session self-review가 detached_verified 불가
- fresh evaluator + bundle 검증으로 assurance 승격 가능
- 검증 통과 후 trade-off가 남으면 acceptance pending
- projection job 실패 시 state current / projection failed 분리 표시
- 사용자 TASK 직접 수정 시 reconcile pending 감지
- repo drift after baseline 시 evidence stale 또는 verify blocked
- MCP 없이 Skill만 있을 때 product file write 보류

## 23. 요약

```text
MCP는 일상 조작 API다.
state.sqlite와 event log는 운영 상태의 기준이다.
문서는 사람이 읽는 projection이다.
approval은 scope-bound contract다.
evidence는 acceptance criteria와 연결된다.
TDD trace와 manual QA는 evidence와 completion 조건에 연결된다.
verification은 fresh context와 bundle을 기준으로 독립성을 기록한다.
module/interface 설계 품질은 validator와 design artifact로 관리한다.
```
