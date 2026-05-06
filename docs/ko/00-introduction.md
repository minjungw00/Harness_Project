# 소개

## 문서 역할

사용자와 구현자가 공유하는 정신 모델.

## 담당 범위

- 하네스가 줄이는 문제 요약
- 세 공간 모델 요약
- Task / Change Unit / Evidence / Projection 기본 개념
- advisor/direct/work 소개
- status card 예시
- source-of-truth 요약

## 담당하지 않는 범위

- 구현 schema
- 상태 전이표
- tool schema
- template 전문

## 섹션

### 하네스가 필요한 이유

AI 지원 개발은 빠르게 움직이지만, 중요한 작업 사실은 종종 chat 안에 갇혀 있다. 사용자가 무엇을 요청했는지, 어떤 범위에 합의했는지, 무엇이 바뀌었는지, 어떤 근거가 있는지, 아직 어떤 승인이 필요한지, 결과를 실제로 확인했는지가 대화 속에 흩어진다.

하네스는 이런 작업에 작은 로컬 운영 커널을 제공한다. 대화는 자연스럽게 유지하되, 오래 남아야 하는 작업 상태는 chat 밖에 기록한다. 그래서 task는 기억이 아니라 현재 상태를 기준으로 resume, verify, reconcile, close될 수 있다.

짧게 말하면:

```text
Harness keeps AI development inside explicit state, scope, evidence, verification, and human judgment.
```

### 세 공간

하네스는 세 공간을 분리한다.

| 공간 | 독자 수준의 의미 |
|---|---|
| Product Repository | 사용자의 실제 product workspace: code, test, 생성된 readable report, 사람이 편집할 수 있는 proposal area. |
| Harness Server / Installation | 로컬 harness process와 tool: MCP server, Core, validator, projector, connector, operator command. |
| Harness Runtime Home | 로컬 운영 저장소: project registration, project별 state, 오래 보존되는 evidence artifact. |

이 분리는 product file, 생성된 Markdown, chat text, operational state가 서로 혼동되는 것을 막는다. 정식 architecture 세부 내용은 [04-runtime-architecture.md](04-runtime-architecture.md)가 담당한다.

### 핵심 개념

- Task는 사용자 가치 단위다. 사용자가 끝내거나 답받고 싶은 일이다.
- Change Unit은 product write를 허가하는 범위 지정 구현 단위다.
- Evidence는 작업에 대한 주장, 예를 들어 diff, log, test, screenshot, run summary, Eval record, Manual QA record 등을 뒷받침하기 위해 기록된 근거다.
- Raw artifact는 artifact store 안에 보존되는 durable evidence file이다.
- Projection은 state record와 artifact ref를 사람이 읽을 수 있는 Markdown으로 렌더링한 것이다.
- Reconcile은 human-editable note나 projection drift를 accepted state change, rejected proposal, note, decision, deferred item으로 바꾸는 명시적 경로다.

자세한 entity와 gate 모델은 [03-kernel-spec.md](03-kernel-spec.md)가 담당한다. Projection rule은 [07-document-projection.md](07-document-projection.md)가 담당한다.

### 작업 모드

하네스는 세 가지 작업 모드를 사용한다.

| 모드 | 사용 대상 | Product write |
|---|---|---|
| `advisor` | 설명, 비교, review, planning, decision support. | 허용되지 않음. |
| `direct` | 범위와 결과가 명확한 작고 위험이 낮은 변경. | 활성 scoped Change Unit 안에서만 허용. |
| `work` | feature work, structural change, risky work, multi-step implementation. | 활성 scoped Change Unit 안에서만 허용하며, 보통 더 강한 evidence와 verification이 필요. |

Task는 작게 시작할 수 있다. 범위가 커지면 하네스는 그 사실을 드러내고, 안전하게 실행할 수 있는 형태로 작업을 옮겨야 한다.

### Status Card 읽기

Status card는 canonical state가 아니라 파생 display다. 목적은 독자의 네 가지 질문에 빠르게 답하는 것이다.

- 지금 어떤 task를 진행 중인가?
- 다음으로 안전한 action은 무엇인가?
- 어떤 user decision이나 gate가 진행을 막고 있는가?
- readable projection은 믿을 만큼 최신인가?

예:

```text
TASK-0044 Email login flow
State: work / shaping
Next action: failed-login UX 결정
Scope: login form, login API call, session storage
Approval: dependency_change 필요
Evidence: 없음
Verification: 시작 전
Manual QA: 대기
Acceptance: 대기
Projection: 최신
```

`Manual QA: pending` 같은 친근한 label은 display text다. Canonical field와 close rule은 kernel이 정의한다.

### Source Of Truth 요약

Source-of-truth 경계는 다음과 같다.

```text
Operational state:
  state.sqlite current records plus state.sqlite.task_events

Raw evidence:
  durable files in the artifact store

Markdown reports:
  projections generated from state records and artifact refs

Human-editable sections:
  input surfaces for notes and proposals
```

Human-editable input은 reconcile 또는 Core state-changing action이 accepted state event나 record를 기록한 뒤에만 operational truth가 된다.

정식 규칙은 [03-kernel-spec.md](03-kernel-spec.md), [04-runtime-architecture.md](04-runtime-architecture.md), [07-document-projection.md](07-document-projection.md)를 참고한다.
