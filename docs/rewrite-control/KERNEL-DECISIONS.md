# Kernel Decisions

이 문서는 하네스 문서 세트 재작성 전에 고정하는 설계 결정이다.

본문 문서 작성자는 이 결정을 다시 논쟁하지 않는다. 새 결정이 필요하면 기준 문서 본문에 임의로 쓰지 않고 `TODO_DECISION`으로 표시한다.

## KD-01. Event Log 물리 위치

### 결정

MVP에서 event log는 별도 canonical store가 아니라 `state.sqlite.task_events` append-only table이다.

권장 표현:

```text
운영 상태의 canonical source는 state.sqlite이다.
state.sqlite는 current state table과 append-only task_events table을 가진다.
```

피할 표현:

```text
state.sqlite + event log
```

위 표현은 별도 event store가 있는 것처럼 읽힐 수 있으므로, 필요한 경우 다음처럼 쓴다.

```text
state.sqlite current tables + state.sqlite.task_events
```

### 근거

운영 상태와 event history를 같은 SQLite transaction 경계 안에 두어 MVP 구현을 작게 만든다. 외부 event stream은 v1/later 확장으로 둔다.

### 영향

- `03-kernel-spec.md`: event schema와 state transition event 정의
- `04-runtime-architecture.md`: authority flow에서 event 위치 명시
- `06-reference-mvp.md`: SQLite DDL에 `task_events` 포함
- `07-document-projection.md`: projection freshness와 event relation 설명

## KD-02. 세 공간은 고정한다

### 결정

하네스는 계속 세 공간을 분리한다.

```text
Product Repository
Harness Server / Installation
Harness Runtime Home
```

### 의미

```text
Product Repository:
  제품 코드, 테스트, 사람이 읽는 projection, human-editable notes

Harness Server / Installation:
  MCP server, Core, validator, connector, projector, CLI

Harness Runtime Home:
  registry.sqlite, project.yaml, state.sqlite, artifacts
```

### 근거

제품 코드, 하네스 실행 코드, 운영 상태가 섞이면 source-of-truth와 projection이 다시 혼동된다.

### 영향

- `04-runtime-architecture.md`가 canonical explanation을 소유한다.
- `00-introduction.md`, `README.md`, `10-user-guide.md`는 짧은 요약만 둔다.

## KD-03. 상태 모델: Lifecycle + Gates

### 결정

상태 모델은 단일 긴 축 목록이 아니라 `lifecycle + gates` 구조로 재구성한다.

### Lifecycle

```yaml
mode: advisor | direct | work
lifecycle_phase:
  intake | shaping | ready | executing | verifying | qa |
  waiting_user | blocked | completed | cancelled
result:
  none | advice_only | passed | failed | cancelled
close_reason:
  none | completed_verified | completed_self_checked |
  completed_with_risk_accepted | cancelled | superseded
```

### Gates

```yaml
scope_gate:
  not_required | required | pending | passed | failed | blocked
approval_gate:
  not_required | required | pending | granted | denied | expired
  # display alias allowed: passed = granted when no drift exists
design_gate:
  not_required | required | pending | passed | partial | waived | stale | blocked
evidence_gate:
  not_required | none | partial | sufficient | stale | blocked
verification_gate:
  not_required | required | pending | passed | failed | waived_by_user | blocked
qa_gate:
  not_required | required | pending | passed | failed | waived
acceptance_gate:
  not_required | required | pending | accepted | rejected
```

### 파생 Display

Compact status card는 canonical fields에서 파생한다. Display state는 canonical source가 아니다.

### Evidence Gate 적용 여부

```text
not_required:
  evidence gate does not apply, for example advisor-only work

none:
  evidence is required but no evidence has been recorded yet
```

### 근거

기존 상태 축은 표현력은 좋지만 조합 규칙이 닫혀 있지 않다. Gate model은 completion 판단을 구현 가능하게 만든다.

## KD-04. Scope Gate와 Approval Gate는 분리한다

### 결정

Scope gate와 approval gate를 분리한다.

```text
scope_gate:
  모든 write-capable run에 적용한다.

approval_gate:
  민감 변경 범주가 있을 때만 required가 된다.

capability:
  MVP에서는 first-class kernel gate가 아니다.
  surface_capability_check validator, prepare_write blocked_reasons,
  guarantee level display로 표현한다.
```

### 규칙

제품 파일 쓰기 전에는 항상 scope gate를 확인한다. Sensitive category가 있으면 approval gate도 확인한다.

`03-kernel-spec.md`의 canonical gate list에는 `capability_gate`를 추가하지 않는다.

### 예시

```text
오타 수정:
  scope_gate=passed
  approval_gate=not_required

dependency 추가:
  scope_gate=passed 또는 pending
  approval_gate=required/pending/granted
```

### 근거

기존 문서에서 “scope와 approval 확인”이 함께 표현되어 두 개념이 섞일 수 있었다. 모든 write에는 scope가 필요하지만 모든 write에 approval이 필요한 것은 아니다.

## KD-05. Verification Waiver는 Detached Verification이 아니다

### 결정

사용자가 verification exception을 수용해 작업을 닫을 수는 있다. 그러나 waiver는 `detached_verified`로 표시하지 않는다.

### 필수 표현

```yaml
verification_gate: waived_by_user
assurance_level: none | self_checked
close_reason: completed_with_risk_accepted
```

금지 표현:

```yaml
verification_gate: waived_by_user
assurance_level: detached_verified
```

### 근거

“work는 실행자의 자기 보고만으로 닫지 않는다”는 원칙을 유지하되, 현실 운영의 risk-accepted close를 정직하게 표현한다.

### Close Semantics

- `completed_verified`: detached verification이 실제로 통과한 close
- `completed_with_risk_accepted`: 사용자가 남은 verification risk를 수용한 close
- 두 상태는 사용자 카드와 export에서 구분한다.

## KD-06. Direct Work는 선택적으로 검증할 수 있다

### 결정

`direct` 작업은 기본적으로 `self_checked`로 닫을 수 있다. 사용자가 원하거나 정책상 필요하면 optional detached verification을 붙일 수 있다.

### 규칙

```text
direct:
  verification_gate=not_required by default
  assurance_level=self_checked by default

optional direct verify passed:
  verification_gate=passed
  assurance_level=detached_verified allowed
```

### 근거

작은 direct 작업에 항상 detached verification을 요구하면 기본 경험이 무거워진다. 단, direct 결과를 독립 검증한 경우 그 사실을 숨길 이유는 없다.

## KD-07. User Notes Authority

### 결정

User Notes는 세 단계 authority로 표현한다.

```text
입력 원본:
  human-editable document section

반영 후보의 canonical record:
  state.sqlite.reconcile_items

반영 후 운영 사실:
  state.sqlite event + target record
```

### 규칙

Human-editable section은 사용자 입력 표면이다. 그 자체가 Task state를 바꾸지 않는다.

### 근거

기존 문서에서 사용자 메모의 canonical source가 `human-editable 문서 영역` 또는 `reconcile item`으로 흔들렸다. 새 모델은 입력 표면과 운영 반영을 분리한다.

## KD-08. Domain Language Authority

### 결정

Domain Language의 canonical source는 `state.sqlite.domain_terms`다.

```text
canonical source:
  state.sqlite.domain_terms

사람이 읽는 projection:
  DOMAIN-LANGUAGE

사용자 제안:
  human-editable section → reconcile_items → domain_terms
```

### 규칙

`DOMAIN-LANGUAGE` 문서는 사람용 projection이다. Accepted human edit은 reconcile을 거쳐 `domain_terms`에 반영된다.

### 근거

`domain language record + reconciled doc` 표현은 source-of-truth 원칙을 약하게 만든다.

## KD-09. Module and Interface Authority

### 결정

Module Map과 Interface Contract도 운영 record가 canonical source다.

```text
module map canonical source:
  state.sqlite.module_map_items

interface contract canonical source:
  state.sqlite.interface_contracts

projection:
  MODULE-MAP
  INTERFACE-CONTRACT
```

### 규칙

Design projection의 human-editable proposal은 reconcile을 통해 record로 승격된다.

## KD-10. Core Invariants

### 결정

Core invariant는 다음 7개로 제한한다.

```text
1. Chat is not state.
2. Product write requires an active scoped Change Unit.
3. Sensitive change requires explicit approval.
4. Completion requires evidence coverage where evidence is required.
5. Work cannot self-certify detached verification.
6. Required QA and acceptance are separate gates.
7. Projection cannot override canonical state.
```

### 규칙

문서 본문에서 이 외의 원칙을 core invariant처럼 표현하지 않는다.

### 근거

기존 문서의 17개 불변식은 가치가 있으나 모두 kernel invariant는 아니다. Kernel invariant는 깨지면 하네스가 하네스가 아니게 되는 것만 둔다.

## KD-11. Policy Defaults

### 결정

다음 항목은 core invariant가 아니라 design-quality policy default다.

```text
- shared design for work
- domain language consistency
- vertical slice default
- TDD trace for suitable work
- module/interface review
- manual QA for UI/UX/copy
- context hygiene
```

### 규칙

Policy default는 applies_when, default_requirement, allowed_waiver, required_record, validator, close_impact로 정의한다.

### 근거

이 원칙들은 설계 품질에 중요하지만 작업 유형에 따라 waiver와 예외가 필요하다.

## KD-12. Guarantee Levels

### 결정

보장 수준은 다음 네 단계로 통일한다.

```text
cooperative:
  agent surface가 절차를 따르는 전제의 보장

detective:
  위반을 감지하고 state를 blocked/stale/partial로 바꾸는 보장

preventive:
  위반을 실행 전에 차단하는 보장

isolated:
  별도 worktree/sandbox/process로 위험을 격리하는 보장
```

### 규칙

MVP reference surface는 기본적으로 cooperative/detective다. T4 guard가 있는 profile에서만 preventive 일부를 주장할 수 있다. T5 isolation이 있는 profile에서만 isolated guarantee를 주장한다.

### 근거

모든 surface에서 product write를 사전에 완전히 막을 수 있다는 암시를 제거한다.

## KD-13. MCP Public Surface

### 결정

Public MCP tools는 유지하되 schema를 엄격화한다.

```text
harness.status
harness.intake
harness.next
harness.prepare_write
harness.record_run
harness.request_user_decision
harness.record_user_decision
harness.launch_verify
harness.record_eval
harness.record_manual_qa
harness.close_task
```

### 규칙

Tool 이름은 high-level intent로 유지한다. Tool별 schema, errors, events, validators, projection jobs는 `05-mcp-api-and-schemas.md`가 소유한다.

### 필수 구분

```yaml
harness.record_run:
  kind: shaping_update | implementation | direct | verification_input

harness.request_user_decision:
  decision_kind:
    approval | scope_confirmation | design_choice |
    qa_waiver | acceptance | reconcile
```

### 근거

Public tool 수를 늘리지 않으면서 over-broad payload를 막는다.

## KD-14. Common Tool Envelope

### 결정

State-changing MCP tool은 공통 envelope를 가진다.

```yaml
request_id: string
idempotency_key: string
expected_state_version: integer
project_id: string
task_id: optional string
surface_id: string
run_id: optional string
actor_kind: user | lead_agent | evaluator | operator
dry_run: boolean
```

### 규칙

`expected_state_version`이 맞지 않으면 `STATE_CONFLICT`를 반환한다. Retry는 idempotency key로 판정한다.

## KD-15. Projection Template Tiers

### 결정

Projection template은 세 등급으로 나눈다.

```text
Required MVP:
  TASK
  APR
  RUN-SUMMARY
  EVIDENCE-MANIFEST
  EVAL
  DIRECT-RESULT

Optional design-quality:
  DOMAIN-LANGUAGE
  MODULE-MAP
  INTERFACE-CONTRACT
  TDD-TRACE
  MANUAL-QA

Appendix:
  DEC
  DESIGN
  EXPORT
  full report variants
```

### 규칙

`07-document-projection.md` 본문에는 required MVP template과 운영 규칙만 둔다. 전문 template library는 `appendix/A-template-library.md`가 소유한다.

## KD-16. Reference Surface Scope

### 결정

MVP는 하나의 reference agent surface만 대상으로 한다.

```text
MVP:
  one reference surface
  MCP T2
  artifact capture 최소
  manual verify bundle
  cooperative prepare_write

v1:
  sidecar file watcher
  worktree verify
  second connector

later:
  native hooks
  browser QA capture
  cross-surface verify
```

### 근거

MVP의 성공 기준은 많은 surface 지원이 아니라 kernel invariant 검증이다.

## KD-17. Conformance Fixture Format

### 결정

Conformance는 scenario 설명표가 아니라 fixture 기반으로 작성한다.

```yaml
scenario_id:
name:
initial_state:
input:
action:
expected_state:
expected_events:
expected_artifacts:
expected_projection:
expected_error:
```

### 규칙

`11-operations-and-conformance.md`는 fixture 형식을 소유한다. 각 suite는 fixture examples를 제공한다.

## KD-18. Projection Staleness는 State를 다시 쓰지 않는다

### 결정

Projection stale/failed는 state failure가 아니다. Close를 기본적으로 막지 않는다. 단, 사용자 카드와 export에 표시해야 한다.

### 규칙

```text
state current / projection stale
state current / projection failed
```

위 상태를 명확히 구분한다.

### 예외

Projection 자체가 사용자 결정 표면으로 필요한 경우에는 task가 `waiting_user` 또는 `blocked`가 될 수 있다. 이때도 reason은 projection failure가 아니라 required human decision delivery failure로 기록한다.

## KD-19. Prepare Write Authority

### 결정

`harness.prepare_write`는 product write 전 gatekeeper다.

### 필수 Decision Value

```yaml
decision: allowed | blocked | approval_required | state_conflict
```

### 필수 Check

```text
- active Task
- active Change Unit
- mode write eligibility
- baseline freshness
- intended paths
- intended tools
- intended commands
- network targets
- secret access
- sensitive categories
- approval scope
- surface capability profile
- design policy preconditions
```

### 규칙

Agent가 제품 파일 쓰기 가능 여부를 자연어로 임의 판단하지 않는다.

## KD-20. Close Task Authority

### 결정

`harness.close_task`가 completion 조건의 단일 판정 지점이다.

### 규칙

`close_task`는 다음을 판정한다.

```text
- active run 없음
- active Change Unit 상태
- scope gate
- approval gate
- design gate
- evidence gate
- verification gate
- QA gate
- acceptance gate
- close_reason
- result
- projection freshness reporting
```

### 근거

Agent의 완료 보고가 close를 대체하지 못하게 한다.

## KD-21. Sensitive Categories

### 결정

민감 변경 범주는 유지하되 API schema와 approval contract에서 canonical enum으로 정의한다.

Minimum categories:

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

## KD-22. Later Automation 위치

### 결정

다음 항목은 core docs 본문에서 구현 범위처럼 쓰지 않는다.

```text
- dashboard
- browser QA automatic capture
- cross-surface verify
- native hook coverage for every surface
- parallel Change Unit execution
- long-term analytics
- team profile export/import
```

이 항목들은 `appendix/C-later-roadmap.md`가 소유한다.

## KD-23. Documentation Ownership Rule

### 결정

각 개념은 하나의 canonical owner 문서를 가진다. 다른 문서는 한 문장 요약과 참조만 둔다.

소유권 상세는 `docs/rewrite-control/DOC-OWNERSHIP-MAP.md`가 소유한다.

## KD-24. Artifact / Report / Projection 경계

### 결정

Raw artifact, state record, Markdown report는 서로 구분된다.

```text
Raw artifacts:
  durable evidence files in the artifact store

State records:
  canonical structured records in state.sqlite

Markdown reports:
  projections generated from records and artifact refs
```

`RUN-SUMMARY`, `EVAL`, `TDD-TRACE`, `MANUAL-QA`, `EVIDENCE-MANIFEST`, `DIRECT-RESULT`는 기본적으로 raw artifact가 아니다.

Export bundle은 hash가 있는 projection을 포함할 수 있지만, 그것이 projection을 canonical raw evidence artifact로 만들지는 않는다.

## KD-25. EVAL Verdict, Verification Gate, Assurance Level

### 결정

`EVAL` verdict만으로 assurance를 upgrade하지 않는다.

```text
assurance_level=detached_verified requires:
  - a passed verification result
  - a valid independence qualifier
  - same-session self-review guard not violated
```

Same-session review는 `detached_verified`를 만들 수 없다.

## KD-26. QA Gate와 Manual QA Record Result

### 결정

`qa_gate`가 canonical kernel gate다.

`manual_qa_record.result`는 record-level result다.

User-facing card는 다음처럼 말할 수 있다.

```text
Manual QA: pending/passed/failed/waived
```
