# 런타임 아키텍처

## 문서 역할

이 문서는 하네스의 로컬 runtime architecture를 담당한다. 세 공간, runtime layer, Core process model, state transaction flow, artifact store architecture, projection/reconcile flow, guarantee level, failure/recovery overview를 정의한다.

Public MCP request/response schema, SQLite DDL, full CLI command semantic, conformance fixture, surface-specific connector cookbook은 정의하지 않는다.

## 아키텍처 범위

하네스는 AI 지원 product work를 위한 로컬 운영 커널이다. Architecture는 세 관심사를 분리한다.

```text
Product Repository:
  product code, tests, human-readable projections, and human-editable proposal areas

Harness Server / Installation:
  MCP server, Core, validators, connectors, projector, reconcile worker, and operator tools

Harness Runtime Home:
  registry.sqlite, project.yaml, state.sqlite, and the artifact store
```

이 분리는 chat, Markdown report, generated connector file, product source file이 우연히 operational state가 되는 것을 막는다. Canonical operational state는 `state.sqlite` current record와 `state.sqlite.task_events`에 있다. Raw evidence는 artifact store에서 canonical하다. Product Repository의 Markdown file은 projection 또는 proposal surface다.

## Product Repository

Product Repository는 사용자의 실제 product workspace다. Product source code, test, repository-level agent rule, human-readable harness projection을 포함한다.

일반적인 repository-owned path:

```text
repo/
  AGENTS.md
  docs/
    tasks/
    approvals/
    reports/
    design/
  .harness/
    agent/generated/
    reconcile/pending/
```

Repository는 생성된 TASK, APR, RUN-SUMMARY, EVAL, DIRECT-RESULT, EVIDENCE-MANIFEST, TDD-TRACE, MANUAL-QA, DOMAIN-LANGUAGE, MODULE-MAP, INTERFACE-CONTRACT Markdown report를 가질 수 있다. 이 file들은 사람과 agent가 작업을 읽는 데 도움을 주지만 canonical state는 아니다. Human-editable section은 input surface다. Accepted change는 reconcile 또는 MCP tool을 통해 흐른 뒤에야 state record가 된다.

## Harness Server / Installation

Harness Server / Installation은 control plane이다. MVP는 이것을 여러 service의 집합이 아니라 internal module을 가진 하나의 local process로 구현할 수 있다.

Core runtime responsibility:

- MCP server를 통해 read resource와 public tool expose
- Core에서 kernel state transition 실행
- write 전, run 후, close 전에 validator 실행
- artifact와 integrity metadata 기록
- projection job enqueue 및 render
- human edit 또는 managed-block drift에서 reconcile candidate detect
- diagnostic, recovery, export, conformance entrypoint 제공

MCP server는 shell command의 얇은 wrapper가 아니다. Core가 state transition, validator, artifact record, projection job으로 변환하는 high-level intent call을 expose한다.

## Harness Runtime Home

Harness Runtime Home은 local operational authority를 저장한다. Reference location은 `~/.harness`지만, 정확한 MVP layout은 reference MVP document가 담당한다.

Runtime Home 포함 항목:

- project registration, connected surface, connector manifest를 위한 `registry.sqlite`
- static project configuration을 위한 registered project별 `project.yaml`
- current operational record와 `state.sqlite.task_events`를 위한 project별 `state.sqlite`
- durable evidence file을 위한 artifact directory

Runtime Home은 chat history가 사라지거나 Product Repository projection이 stale이어도 operational state를 recover하기에 충분해야 한다. Product Repository document는 state record와 artifact ref에서 regenerate할 수 있으며, 그 record를 대체하지 않는다.

## Runtime Layers

```text
User conversation surface
  ↓
Agent surface
  ↓
Harness rules / skill / local instructions
  ↓
Harness MCP server
  ↓
Harness Core
  ↓
state.sqlite / artifact store / validators / projector / reconcile worker
```

Conversation surface는 user intent, decision, approval, QA judgment, acceptance를 수집한다. Agent surface는 reading, editing, checking을 수행한다. Harness rule과 skill은 agent가 방향을 잃지 않도록 한다. MCP server는 tool boundary를 제공한다. Core는 state machine을 담당한다. Validator, artifact capture, projection, reconcile은 state transition에 evidence와 readable output을 연결한다.

Native hook, sidecar, command wrapper, file watcher, worktree isolation은 capability-dependent enforcement layer다. Concrete capability profile이 더 강한 enforcement를 입증하지 않는 한 MVP는 reference surface에 대해 cooperative/detective behavior에 의존한다.

## Core Process Model

MVP Core는 다음 internal module을 가진 single process로 실행될 수 있다.

| Module | Runtime responsibility |
|---|---|
| State store | current records, state versions, locks, and `state.sqlite.task_events` |
| Task workflow | intake, mode selection, next action, gate updates, close decisions |
| Approval module | scope-bound approval request, decision, expiry, and drift handling |
| Evidence module | run records, artifact refs, evidence manifests, and coverage checks |
| Verification module | verification bundles, evaluator runs, Eval records, and independence checks |
| Manual QA module | QA records and `qa_gate` aggregation |
| Projection module | projection jobs, managed blocks, freshness, and report paths |
| Reconcile module | human-editable proposals, managed drift, and accepted-state routing |
| Validator runner | core, design-quality, artifact, projection, and connector checks |
| Connector adapter | reference surface registration, capability reporting, and capture hints |

Core는 canonical operational state를 update하는 유일한 component다. Agent, CLI command, projector, reconnect/recovery flow는 Core logic을 통해 들어오거나 동일한 state compatibility rule을 보존하는 recovery code를 사용해야 한다.

## State Transaction Flow

모든 state-changing operation은 current record와 event history에 대해 하나의 SQLite transaction을 사용한다.

```text
1. validate request envelope and expected state version
2. acquire the project/task lock needed for the transition
3. read current state records
4. run pre-transition validators
5. update current records
6. append one or more rows to state.sqlite.task_events
7. increment state/projection versions as needed
8. enqueue projection jobs
9. commit
10. render Markdown projections after commit
```

Projection rendering은 transaction 뒤에 일어난다. Projection failure는 projection freshness를 stale 또는 failed로 mark하고 committed state는 그대로 둔다. Projection은 passed task를 failed task로 바꿀 수 없고, 이후 reconcile decision 없이 canonical state를 repair할 수도 없다.

## Artifact Store Architecture

Artifact store는 durable evidence file을 보관한다. Raw artifact에는 integrity metadata와 함께 저장되는 diff, log, screenshot, checkpoint, bundle, captured manifest, exported bundle component, 기타 evidence file이 포함된다.

Artifact는 두 부분으로 이루어진다.

- artifact store 안의 raw file
- kind, path, hash, size, redaction state, task/run relation, retention class를 명명하는 `state.sqlite`의 artifact state record

Core는 run, evidence manifest, Eval record, Manual QA record, projection report, export에 artifact ref를 기록한다. 큰 log와 patch는 raw artifact로 두어야 하며, Markdown report는 unbounded evidence를 embed하는 대신 artifact ref에 link해야 한다.

Raw secret은 artifact로 저장하면 안 된다. Secret-related evidence가 required라면 Core는 redacted artifact, secret handle, relevant validator를 통과한 operator note를 기록한다.

## Raw Artifacts, State Records, Markdown Reports

경계는 다음과 같다.

| Item | Authority | Examples |
|---|---|---|
| Raw artifact | Artifact store의 durable evidence file | diff, log, screenshot, checkpoint, bundle, manifest file |
| State record | `state.sqlite`의 canonical structured record | Task, Change Unit, Run, Approval, Eval, Manual QA record, Evidence Manifest, Artifact record |
| Markdown report | Record와 artifact ref에서 생성되는 human-readable projection | TASK, APR, RUN-SUMMARY, EVAL, DIRECT-RESULT, EVIDENCE-MANIFEST |

이 named report kind는 기본적으로 projection 또는 state-backed record다. Artifact store의 evidence file을 참조할 수 있고 export가 그 snapshot을 포함할 수 있지만, 그렇다고 Markdown report가 canonical evidence file이 되지는 않는다.

## Projection And Reconcile Flow

Projection은 outbox-style flow다.

```text
state transition committed
→ projection job queued
→ managed block rendered from state records and artifact refs
→ projected version and managed hash recorded
→ human-editable area preserved
```

Projector는 managed area만 write하고 human-editable area를 보존한다. Managed area가 직접 edited되면 projector는 그 edit를 state로 조용히 취급하지 않고 reconcile candidate를 기록한다. Human-editable area에 proposal이 있으면 reconcile은 candidate record를 만들고 explicit decision을 요청한다.

Reconcile authority path:

```text
human-editable input
→ state.sqlite.reconcile_items
→ accepted state event/record or rejected/deferred note
```

Reconcile은 merge, reject, note로 convert, decision 생성, design support record 생성 또는 update, defer를 할 수 있다. Accepted operational change는 Core를 통해 기록되고 `state.sqlite.task_events`에 appended된다.

## Validators And Adapter Placement

Validator는 Core 옆에 위치하며 structured result를 Core에 반환한다. Core는 그 result가 transition을 block할지, gate를 stale/partial/blocked로 mark할지, user decision을 request할지, display에만 영향을 줄지 결정한다.

MVP validator category:

- state and envelope validation
- active Task and active Change Unit checks
- changed path and scope checks
- baseline freshness
- approval scope
- evidence sufficiency
- same-session verification guard
- projection freshness and managed-hash checks
- minimal design-quality checks
- `surface_capability_check`

Adapter와 sidecar는 surface capability를 observable fact로 변환한다. Capability에 대한 kernel gate를 만들지 않는다. Capability는 validator result, `prepare_write` blocked reason, guarantee display를 통해 나타난다.

## Guarantee Levels

하네스는 enforcement strength를 정직하게 드러내기 위해 guarantee level을 report한다.

| Level | Meaning |
|---|---|
| `cooperative` | agent surface가 harness instruction과 MCP decision을 따를 것으로 기대된다 |
| `detective` | 하네스가 observation 후 violation을 detect하고 state를 blocked, stale, partial, failed로 mark할 수 있다 |
| `preventive` | connector 또는 runtime이 violating action을 execution 전에 block할 수 있다 |
| `isolated` | risky work가 worktree, sandbox, process boundary 또는 동등한 isolation으로 분리된다 |

MVP reference behavior는 connected surface에 concrete pre-tool guard나 isolation layer가 없는 한 cooperative/detective다. Native hook expansion, advanced sidecar watching, broad isolated execution은 MVP reference surface에 명시적으로 구현되지 않는 한 later roadmap item이다.

Guarantee level은 display와 risk context다. Approval, verification, acceptance, kernel gate가 아니다.

## Failure And Recovery Overview

Failure는 숨기지 않고 기록한다.

| Failure | Architecture-level handling |
|---|---|
| Agent crash during write | active run을 interrupted로 mark; 가능하면 diff/log snapshot capture; artifact register |
| Baseline drift after approval | approval 또는 evidence를 stale로 mark; scope가 영향받으면 reconfirmation 요구 |
| Evaluator observes repo drift | verification을 block 또는 stale 처리; fresh baseline 또는 new bundle 요구 |
| Artifact file missing | artifact/evidence를 stale로 mark; recovery로 rescan 또는 restore |
| Projection job failed | state는 current로 유지; projection failed로 mark하고 retry 또는 reconcile |
| Managed Markdown edited directly | reconcile item 생성; state를 직접 mutate하지 않음 |
| MCP unavailable | cooperative surface에서는 product write를 instruction으로 hold; stronger guard는 preventively enforce 가능 |
| Surface capability mismatch | validator result 기록, guarantee display 조정, required check가 만족될 수 없으면 unsafe write block |

Recovery tool은 projection freshness repair, artifact rescan, stale run interrupt, drifted approval expire, reconcile item 생성을 수행할 수 있다. 동일한 authority rule을 보존해야 한다. `state.sqlite`는 operational state이고, `state.sqlite.task_events`는 그 state store 안의 event history이며, raw evidence는 artifact store에 있고, Markdown report는 projection으로 남는다.
