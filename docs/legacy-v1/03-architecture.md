# 03. Harness Architecture

## 1. 문서 역할

이 문서는 하네스의 로컬 배치와 런타임 아키텍처를 정의한다. 세 공간, 런타임 계층, 컴포넌트 책임, authority 흐름, 핵심 시퀀스, 장애와 복구 흐름을 소유한다.

MCP tool schema와 상태 필드는 `04-reference-implementation.md`가 소유한다. Markdown 템플릿은 `07-document-and-artifact-contracts.md`가 소유한다. 표면별 connector 세부는 `06-agent-integration.md`가 소유한다.

## 2. 로컬 배치 모델

하네스는 세 공간을 분리한다.

```text
Product Repository
Harness Server / Installation
Harness Runtime Home
```

이 분리는 제품 코드, 하네스 실행 코드, 운영 상태를 섞지 않기 위한 핵심 아키텍처다.

### 2.1 Product Repository

Product Repository는 사용자의 실제 제품 저장소다.

책임은 다음이다.

- 제품 코드와 테스트 보관
- 저장소 공통 agent rule 보관
- 사람이 읽는 하네스 projection 문서 보관
- human-editable 메모와 proposal 입력 표면 제공
- domain language, module map, interface contract projection 제공

참조 레이아웃:

```text
repo/
  AGENTS.md
  docs/
    tasks/
      active/
      completed/
    decisions/
    approvals/
    reports/
      runs/
      evals/
      directs/
      qa/
      tdd/
    evidence/
    design/
      domain-language.md
      module-map.md
      interface-contracts.md
  .harness/
    agent/
      generated/
      surface-status.json
    reconcile/
      pending/
```

Markdown 문서는 사람이 읽는 projection이다.

### 2.2 Harness Server / Installation

Harness Server / Installation은 하네스 제어면을 실행하는 설치물이다.

책임은 다음이다.

- Harness MCP server 실행
- Harness Core 실행
- validator runner 실행
- adapter와 connector 실행
- projector와 reconcile 실행
- recovery와 conformance 도구 제공

참조 레이아웃:

```text
harness-server/
  mcp/
  core/
  validators/
  adapters/
  connectors/
  templates/
  cli/
```

### 2.3 Harness Runtime Home

Harness Runtime Home은 운영 상태와 artifact를 저장한다. 기본 위치는 `~/.harness`다.

```text
~/.harness/
  config.yaml
  registry.sqlite
  projects/
    PRJ-0001/
      project.yaml
      state.sqlite
      artifacts/
        traces/
        checkpoints/
        bundles/
        logs/
        diffs/
        manifests/
        tdd/
        qa/
        exports/
```

Runtime Home은 운영 기준 저장소다. Product Repository 문서와 agent chat이 없어도 상태와 evidence를 복구할 수 있어야 한다.

## 3. 런타임 계층

```text
사용자 대화 표면
  ↓
에이전트 표면
  ↓
Harness rules / Skill / custom agent
  ↓
Harness MCP server
  ↓
Harness Core
  ↓
state store / artifact store / validator / projector / sidecar
```

### 3.1 사용자 대화 표면

사용자는 agent surface에서 자연어로 요청하고, 질문에 답하고, 승인 또는 거절하고, QA와 acceptance를 판단한다.

### 3.2 에이전트 표면

에이전트 표면은 코드를 읽고 수정하는 LLM 개발 도구다. Rule/context를 읽고, Harness Skill을 사용하고, MCP tool을 호출하고, 상태 카드를 보여준다.

### 3.3 Rule / Skill 계층

Rule은 짧은 항상-on 원칙이다. Skill은 절차 안내다. 이 계층은 정책을 설명하고, 집행은 MCP, Core, validator, hook, sidecar가 담당한다.

### 3.4 MCP server

MCP server는 에이전트가 하네스를 조작하는 기본 API다. CLI 명령의 원격 노출이 아니라 상태기계 기반 high-level intent API를 제공한다.

### 3.5 Harness Core

Core는 상태 전이와 운영 로직을 실행한다. MVP에서는 독립 서비스 군이 아니라 하나의 core 안의 내부 모듈로 구현한다.

```text
harness-core
  state store
  task workflow
  approval module
  evidence module
  design-quality module
  projection module
  verification module
  validator runner
```

### 3.6 집행 계층

집행 계층은 guard와 capture를 담당한다.

- allowed path guard
- approval scope guard
- command/network/secret guard
- evaluator write guard
- same-session verify guard
- baseline capture
- artifact capture
- validator 실행
- worktree isolation
- generated file drift detection

표면이 native hook을 제공하지 않으면 sidecar와 validator가 보완한다.

## 4. 핵심 컴포넌트

| 컴포넌트 | 책임 |
|---|---|
| State Store | Task, Change Unit, Run, approval, QA, eval 현재 상태 저장 |
| Event Log | 상태 변경 event를 append-only로 기록 |
| Artifact Registry | diff, logs, bundle, checkpoint, manifest, TDD, QA artifact 등록 |
| Projector | state와 artifact ref를 Markdown projection으로 생성 |
| Reconcile | 문서 또는 generated file drift를 상태 변경 후보로 정리 |
| Approval Module | scope-bound approval 생성, 결정, 만료, drift 관리 |
| Design-quality Module | shared design, domain language, module map, interface contract, TDD, QA record 관리 |
| Evidence Module | acceptance criteria와 evidence mapping 관리 |
| Verification Module | fresh verify, worktree verify, sandbox, manual bundle 준비 |
| Validator Runner | scope, approval, evidence, docs, design-quality, verify integrity 검사 |
| Connector Registry | surface profile, generated manifest, setup 상태 관리 |
| Adapter / Sidecar | 표면별 실행, capture, guard 보완 |

## 5. Design-quality subsystem

Design-quality subsystem은 AI가 좋은 설계 경계 안에서 구현하도록 돕는다.

```text
Shared Design
  질문, 결정, 가정, 비범위, acceptance criteria

Domain Language
  제품 용어, 의미, 코드 표현, 혼동 금지 의미

Module Map
  module responsibility, public interface, dependency direction, test boundary

Interface Contract
  public interface 변경과 compatibility impact

TDD Trace
  red, green, refactor evidence

Manual QA
  사람이 보는 UX, workflow, copy, visual quality, accessibility, taste

Design Validators
  vertical slice, tdd trace, module boundary, public interface, domain mismatch, manual QA required
```

## 6. 데이터와 authority 흐름

상태 변경의 기본 흐름은 다음이다.

```text
사용자 대화
→ MCP request
→ Core state transition
→ state.sqlite + event log
→ artifact store
→ projection outbox
→ Product Repository Markdown projection
```

상태 변경 transaction은 다음 순서로 처리한다.

```text
1. current state 갱신
2. event append
3. projection_version 증가
4. projection job enqueue
5. transaction commit
6. Markdown projection 비동기 갱신
```

Markdown 파일 쓰기는 state transaction 뒤에 수행한다. Projection 실패는 state 실패와 구분한다.

## 7. Authority matrix

| 정보 | canonical source | 표시 |
|---|---|---|
| 프로젝트 등록 | `registry.sqlite` | status, connector report |
| 정적 프로젝트 설정 | `project.yaml` | project summary |
| 현재 운영 상태 | `state.sqlite` + event log | status card, `TASK` |
| shared design | state + design/decision records | `TASK`, `DESIGN`, `DEC` |
| domain language | domain language record + reconciled doc | `DOMAIN-LANGUAGE` |
| module/interface | design records + interface contract records | `MODULE-MAP`, `INTERFACE-CONTRACT` |
| raw evidence | artifact store | artifact ref |
| approval | approval record + event log | `APR` |
| evidence mapping | evidence manifest record | `EVIDENCE-MANIFEST` |
| TDD trace | tdd_trace record + artifact refs | `TDD-TRACE`, `RUN-SUMMARY` |
| manual QA | QA record + artifact refs | `MANUAL-QA`, `EVAL` |
| verification verdict | eval record + artifact refs | `EVAL` |
| projection freshness | projection job state | front matter, status card |
| 사용자 proposal | human-editable section | reconcile item |

## 8. 새 work 작업 시퀀스

```text
User
  → Agent: "이메일 로그인 플로우 추가해줘. 하네스 기준으로 진행해."
  → MCP: harness.intake
  → Core: Task 생성, mode=work, phase=shaping
  → Projector: TASK projection 생성
  → Agent: compact status card와 첫 shaping 질문 표시
  → User: 질문 답변
  → MCP: harness.record_run 또는 design 기록 포함한 high-level record
  → Core: scope, acceptance criteria, domain language, module impact 기록
  → MCP: harness.prepare_write
  → Core: baseline, allowed path, approval, design guard 검사
  → Agent: 구현과 짧은 check 수행
  → MCP: harness.record_run
  → Artifact Registry: diff/log/checkpoint/TDD artifact 등록
  → Core: Evidence Manifest 갱신
  → MCP: harness.launch_verify
  → Evaluator: bundle 기반 fresh verify
  → MCP: harness.record_eval
  → Manual QA 필요 시 harness.record_manual_qa
  → User acceptance 필요 시 waiting_user
  → MCP: harness.close_task
```

## 9. Direct와 advisor 시퀀스

### Direct

```text
User request
→ harness.intake
→ mode=direct, phase=ready
→ harness.prepare_write
→ edit/check
→ harness.record_run
→ DIRECT-RESULT projection
→ completed 또는 work escalation
```

### Advisor

```text
User request
→ harness.intake
→ mode=advisor
→ 설명, 비교, 리뷰, 결정 초안
→ 필요 시 DEC/DESIGN/DOMAIN-LANGUAGE/MODULE-MAP projection
→ code write 없음
```

## 10. Write guard 흐름

제품 파일 쓰기 전에는 `prepare_write`를 통과한다.

```text
write intent
→ active Task와 Change Unit 확인
→ baseline capture or reuse
→ sensitive category classification
→ allowed paths/tools check
→ approval scope check
→ surface capability check
→ module/interface impact check
→ allowed or blocked
```

차단 결과는 error code와 blocked reason으로 표면화한다.

## 11. Evidence와 artifact 흐름

```text
Implementation step completed
→ changed files collected
→ commands and outputs captured
→ diff/log/checkpoint artifact stored
→ record_run
→ validators run
→ TDD trace recorded if applicable
→ Evidence Manifest updated
→ projection queued
```

Artifact store는 raw evidence의 canonical source다. Markdown 문서에는 raw logs나 large patch를 넣지 않고 artifact ref를 둔다.

## 12. Detached verification 흐름

```text
finish implementation
→ verify required
→ launch_verify
→ source bundle 생성
→ fresh session / fresh worktree / sandbox / manual bundle
→ evaluator reads bundle
→ evaluator checks baseline, changed files, approval scope, evidence manifest, TDD trace, tests
→ record_eval
→ assurance update
→ QA pending, acceptance pending, or completed
```

Detached verification은 lead run과 다른 검증 맥락, 구조화된 evidence, baseline 재확인, changed files 확인, approval scope 확인, evidence manifest 검토를 요구한다.

## 13. Manual QA 흐름

```text
manual_qa_required
→ QA profile 선택
→ run/browser/manual walkthrough 준비
→ screenshot/log/note artifact 등록
→ pass/fail/waive 판정
→ MANUAL-QA projection
→ acceptance pending 또는 rework
```

QA profile 예시는 `ui_quality`, `workflow`, `copy`, `accessibility`, `browser_smoke`, `performance_smoke`다.

## 14. Projection과 reconcile 흐름

Projector는 managed 영역만 갱신하고 human-editable 영역을 보존한다.

```text
state changed
→ projection job queued
→ managed block regenerated
→ front matter version updated
→ human-editable area preserved
```

사람이 managed 영역을 수정하면 reconcile item을 만든다.

```text
managed block hash mismatch
→ reconcile item
→ merge / reject / convert-to-note / create-decision / defer
→ explicit state event or projection repair
```

Human-editable 영역의 새 항목은 user observation, proposal, pending decision으로 승격될 수 있다.

## 15. Context hygiene

하네스는 현재 Task와 최신 evidence를 우선한다.

- `TASK Current Summary`와 `Rolling Spine`을 우선한다.
- 최신 `RUN-SUMMARY`, `EVAL`, `EVIDENCE-MANIFEST`를 우선한다.
- 완료된 PRD, DESIGN, issue는 필요할 때 pull한다.
- 오래된 문서가 현재 코드와 충돌하면 stale 또는 reconcile 상태로 표시한다.

## 16. 장애와 복구 흐름

| 장애 | 처리 |
|---|---|
| write 중 agent crash | active run interrupted, diff/log snapshot artifact 등록 |
| approval 후 baseline drift | approval expired 후보, 재확인 요구 |
| evaluator 중 repo drift | evidence stale, EVAL blocked |
| TDD red evidence 누락 | tdd_trace partial |
| vertical slice requirement 불충족 | Change Unit blocked 또는 exception 요청 |
| public interface drift | interface review required |
| domain term mismatch | domain language warning 또는 reconcile item |
| manual QA required but missing | close blocked, manual_qa pending |
| TASK managed 영역 직접 수정 | reconcile item 생성 |
| artifact directory 불일치 | artifact registry rescan, missing artifact stale |
| generated file 수동 수정 | connector drift, repair flow |
| projection job 실패 | state current / projection failed 분리 표시 |
| MCP 연결 손실 | write 중단, last known state 기준 조회 제한 |

## 17. 보장 수준

| Level | 의미 |
|---|---|
| advisory | 위반을 알리지만 실행 전 차단은 보장하지 않음 |
| detective | 위반을 감지하고 상태를 blocked/stale로 바꿈 |
| preventive | 위반을 실행 전에 차단함 |
| isolated | 별도 worktree/sandbox로 위험을 격리함 |

사용자에게 보여주는 risk와 assurance는 표면별 guarantee level을 반영한다.

## 18. 아키텍처 요약

```text
제품 코드와 문서 projection: Product Repository
하네스 실행 코드: Harness Server / Installation
운영 상태와 raw evidence: Harness Runtime Home
```

에이전트 표면은 사용자 요청을 MCP intent call로 바꾼다. Core는 상태기계 transaction을 실행한다. Validator와 sidecar는 위반을 차단하거나 감지한다. Projector는 결과를 사람이 읽는 문서로 투영한다.

