# 00. Harness Overview

## 1. 문서 역할

이 문서는 하네스 문서 세트의 진입점이다. 하네스의 목적, 로컬 배치, 사용자가 실제로 보는 흐름, 요청 처리 순서, 핵심 개념, source-of-truth 관계를 설명한다.

구현 schema는 `04-reference-implementation.md`가 소유한다. 문서 템플릿은 `07-document-and-artifact-contracts.md`가 소유한다. 운영 명령과 conformance는 `08-operations-and-conformance.md`가 소유한다.

## 2. 하네스는 무엇인가

하네스는 AI 개발 작업을 다음 성질을 가진 운영 단위로 바꾼다.

- 현재 상태가 읽힌다.
- 작업 범위와 리스크가 명시된다.
- 구현 전에 요구사항과 설계 개념이 정렬된다.
- 제품 도메인 용어와 모듈 경계가 기록된다.
- Change Unit은 가능한 한 vertical slice로 나뉜다.
- 테스트, 타입 체크, lint, QA 같은 피드백 루프가 evidence로 남는다.
- 민감 변경은 사용자 approval을 요구한다.
- 큰 작업은 실행자의 자기 보고로 완료되지 않는다.
- 기술 검증, manual QA, 사용자 acceptance가 분리된다.
- 채팅 기록 없이도 작업을 이어갈 수 있다.

Codex, Claude Code, Gemini, GitHub Copilot, Cursor 같은 도구는 agent surface다. Agent surface는 사용자가 대화하고 에이전트가 코드를 읽고 수정하는 표면이다. 하네스의 기준 상태와 raw evidence는 Runtime Home과 artifact store에 있다.

## 3. 하네스가 줄이는 문제

AI와 함께 개발할 때 다음 문제가 반복된다.

- 작업 범위가 대화 흐름에 따라 커진다.
- 사람과 AI가 같은 설계 개념 없이 구현을 시작한다.
- 제품 용어가 대화, 코드, 테스트에서 서로 다른 의미로 쓰인다.
- DB, API, UI를 수평적으로 나누어 실제 통합 검증이 늦어진다.
- 테스트가 구현 뒤에 맞춰져 assurance가 약해진다.
- public interface와 모듈 경계가 사람의 검토 없이 누적 변경된다.
- 프론트엔드, UX, copy 품질이 사람의 QA 없이 통과된다.
- 중요한 선택의 이유와 trade-off가 채팅 안에만 남는다.
- 오래된 문서가 현재 코드와 어긋난 채 context에 들어간다.
- approval, assurance, manual QA, acceptance가 하나의 완료 보고로 섞인다.

하네스는 이 문제를 상태기계, MCP tool, artifact registry, validator, sidecar, projection, design-quality artifact로 다룬다.

## 4. 로컬에서 하네스가 놓이는 세 공간

```text
Product Repository
  제품 코드와 사람이 읽는 하네스 projection 문서가 있는 저장소

Harness Server / Installation
  MCP server, Core, validator, adapter, connector, projector를 실행하는 설치물

Harness Runtime Home
  registry, project config, state DB, event log, artifact를 저장하는 운영 홈
```

### 4.1 Product Repository

Product Repository는 사용자가 여는 제품 프로젝트 저장소다.

```text
repo/
  src/
  tests/
  AGENTS.md
  docs/
    tasks/
    approvals/
    decisions/
    reports/
    evidence/
    design/
      domain-language.md
      module-map.md
      interface-contracts.md
  .harness/
    agent/
    reconcile/
```

이 저장소에는 제품 코드, 테스트, agent rule, 사람이 읽는 Markdown projection이 있다. 사용자는 평소 이 저장소에서 에이전트와 대화한다.

### 4.2 Harness Server / Installation

Harness Server / Installation은 하네스 제어면을 실행한다.

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

구현자는 이 설치물을 별도 Git 리포지토리로 둘 수 있다. 일반 사용자는 패키지, 바이너리, 로컬 entrypoint로 접한다.

### 4.3 Harness Runtime Home

Harness Runtime Home은 운영 상태와 artifact의 기준 저장소다. 기본 위치는 `~/.harness`다.

```text
~/.harness/
  registry.sqlite
  projects/
    PRJ-0001/
      project.yaml
      state.sqlite
      artifacts/
        bundles/
        diffs/
        logs/
        checkpoints/
        manifests/
        tdd/
        qa/
```

## 5. 기본 사용 흐름

처음 연결은 CLI로 수행한다.

```bash
harness connect agents --auto
harness doctor agents
```

일상 작업은 Product Repository에서 대화로 시작한다.

```text
이메일 로그인 플로우 추가해줘. 하네스 기준으로 진행해.
```

에이전트는 내부 MCP tool을 사용해 다음을 처리한다.

```text
상태 확인
→ Task 생성 또는 재개
→ advisor/direct/work 분류
→ work면 shared design shaping
→ domain language와 module/interface 영향 확인
→ Change Unit 작성
→ write 전 scope와 approval 확인
→ 구현과 짧은 피드백 루프
→ evidence 기록
→ work면 detached verification
→ 필요한 manual QA
→ 사용자 acceptance
→ close
```

## 6. 요청 하나의 표준 work 흐름

```text
사용자 자연어 요청
→ harness.intake
→ mode=work, phase=shaping
→ 상태 카드 표시
→ shared design 질문
→ scope, out-of-scope, acceptance criteria 정리
→ domain language와 module/interface impact 기록
→ Change Unit 작성
→ harness.prepare_write
→ approval 필요 시 APR 생성과 사용자 결정
→ 구현
→ harness.record_run
→ evidence manifest 갱신
→ harness.launch_verify
→ fresh evaluator 또는 bundle 기반 검증
→ harness.record_eval
→ manual QA 필요 시 QA 기록
→ 사용자 acceptance 필요 시 대기
→ harness.close_task
```

작은 변경은 `direct`로 처리된다. Direct는 self-check와 `DIRECT-RESULT`를 남기고 빠르게 닫을 수 있다. Direct가 커지면 같은 Task를 `work`로 전환한다.

## 7. 핵심 개념

### Task

Task는 사용자 가치 단위다. 목적, 범위, 비범위, acceptance criteria, 현재 요약, 다음 행동, 남은 사용자 판단, 관련 evidence를 가진다.

### Shared Design Concept

Shared Design Concept은 사람과 AI가 구현 전에 공유하는 설계 이해다. 질문, 결정, 가정, rejected option, acceptance criteria, domain language, module impact, Change Unit에 분산 기록된다.

### Domain Language

Domain Language는 제품 도메인 용어와 의미의 공통 사전이다. 같은 용어가 코드, 테스트, 문서, 대화에서 같은 의미로 쓰이도록 한다.

### Change Unit

Change Unit은 실제 구현 단위다. 목적, 비목표, 허용 경로, 허용 도구, validator profile, TDD 요구, manual QA 요구, 완료 조건, evaluator focus를 가진다.

기능 작업의 기본 Change Unit은 vertical slice다. 입력, 도메인 로직, persistence, API 또는 caller boundary, UI 또는 system-observable output이 얇게 연결된다.

### TDD Trace

TDD Trace는 red → green → refactor 근거를 기록한다. Domain logic, service module, bug fix, parser/validator, state transition, deep module 내부 구현에서 기본값으로 사용한다.

### Evidence Manifest

Evidence Manifest는 acceptance criteria와 evidence의 대응 관계를 기록한다. 각 criterion은 supported, unsupported, not_applicable 중 하나로 표시된다.

### Approval

Approval은 민감 변경을 진행해도 되는지에 대한 사전 사용자 결정이다. Approval은 scope-bound contract다.

### Assurance

Assurance는 기술적으로 어느 수준까지 확인되었는지 나타낸다. 기본 값은 `none`, `self_checked`, `detached_verified`다.

### Manual QA

Manual QA는 사람이 실제 결과를 보고 UX, workflow, copy, visual quality, accessibility, product taste를 확인하는 절차다.

### Acceptance

Acceptance는 사용자가 결과와 남은 trade-off를 받아들이는지에 대한 사후 판단이다.

### Projection

Projection은 운영 상태와 artifact reference를 사람이 읽는 Markdown 문서로 투영한 결과다.

## 8. source-of-truth 요약

| 정보 | 사람이 보는 표면 | canonical source |
|---|---|---|
| 프로젝트 등록과 surface 연결 | status card, connector report | `registry.sqlite` |
| 정적 프로젝트 설정 | project summary | `project.yaml` |
| 현재 Task 상태 | status card, `TASK` Current Summary | `state.sqlite` + event log |
| raw diff/log/checkpoint | artifact ref | artifact store |
| approval | `APR` | approval record + event log |
| evidence coverage | `EVIDENCE-MANIFEST` | evidence manifest record |
| verification verdict | `EVAL` | eval record + artifact refs |
| TDD trace | `TDD-TRACE`, `RUN-SUMMARY` | tdd_trace record + artifact refs |
| manual QA | `MANUAL-QA`, `EVAL` | QA record + artifact refs |
| domain language | `DOMAIN-LANGUAGE` | domain language records + reconciled doc |
| module/interface | `MODULE-MAP`, `INTERFACE-CONTRACT` | design records + reconciled docs |
| 사용자 메모 | `TASK` User Notes | reconcile item |

운영 상태 전이의 canonical source는 `state.sqlite`와 event log다. 문서는 사람이 읽는 projection이다. Artifact store는 raw evidence의 canonical source다. Human-editable 문서 영역은 사용자 입력 표면이며, 상태 반영은 reconcile 또는 MCP tool을 통해 수행한다.

## 9. 기본 상태 카드

```text
TASK-0044 이메일 로그인 플로우 추가
상태: work / shaping
다음 행동: 로그인 실패 UX 결정
사용자 판단: inline 오류 vs toast
리스크: medium
증거: none
설계: domain language 필요, module impact pending
QA: none
최신 보고: none
```

검증 이후의 예시는 다음이다.

```text
TASK-0044 이메일 로그인 플로우 추가
상태: work / waiting_user
다음 행동: manual QA 결과 수용 여부 확인
사용자 판단: 오류 copy와 세션 만료 UX 수용 여부
리스크: medium
증거: sufficient
설계: aligned / reviewed
QA: passed
최신 보고: EVAL-0020, QA-0003
```

## 10. 핵심 요약

```text
사용자는 Product Repository에서 에이전트와 대화한다.
에이전트는 Harness MCP server를 호출한다.
Harness Server / Installation은 MCP와 Core를 실행한다.
Harness Runtime Home은 운영 상태와 raw evidence를 저장한다.
Product Repository의 문서는 사람이 읽는 projection이다.
```

