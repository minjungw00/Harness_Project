# 전략

## 문서 역할

이 문서는 하네스의 전략 계층을 담당한다. 하네스가 왜 존재하는지, 어떤 failure mode를 막는지, 어떤 원칙이 진짜 kernel invariant인지, 어떤 quality rule이 policy default인지 정의한다. Operational state machine은 `docs/03-kernel-spec.md`가 정의하고, design-quality policy 세부 내용은 `docs/08-design-quality-policy-pack.md`가 확장한다.

이 문서는 lifecycle transition table, gate enum detail, MCP request/response schema, SQLite DDL, full projection template, surface-specific connector behavior를 정의하지 않는다.

## 전략적 논지

하네스는 AI 지원 개발을 위한 작은 로컬 운영 커널이다. 목적은 chat transcript를 더 길게 만들거나 모든 task를 무거운 의식으로 바꾸는 것이 아니다. 목적은 product work를 명시적인 state, scope, evidence, human judgment 경계 안에 두는 것이다.

중심 논지는 다음과 같다.

```text
AI agents can move quickly when the kernel keeps the durable truth small, explicit, and checkable.
```

사용자는 평범한 언어로 시작할 수 있어야 한다. Agent는 clarifying question을 묻고, work를 shaping하고, change를 만들고, evidence를 기록하고, decision을 요청할 수 있어야 한다. 하지만 작업의 durable fact는 chat transcript 밖에 있다. Completion은 대화 속 느낌이 아니라 kernel이 판단하는 state transition이다.

따라서 하네스는 세 관심사를 분리한다.

- Conversation은 operating surface다.
- Kernel state는 canonical operating record다.
- Markdown document는 human-readable projection이자 proposal surface다.

## Failure Model

하네스는 AI development workflow에서 반복적으로 나타나는 실패를 중심으로 설계된다.

### Context Failure

현재 state, next action, open decision, evidence가 conversation 안에 묻혀 사용자가 흐름을 잃는다. Chat이 사라지거나 agent session이 빈 상태로 resume되면 task를 신뢰성 있게 재구성할 수 없다.

하네스는 Task state, Change Unit, run, decision, evidence, close status를 canonical record에 유지하고, 사람이 읽을 수 있도록 projection을 생성해 대응한다.

### Scope And Approval Failure

작업은 conversation 중에 커진다. 작은 요청이 큰 rewrite가 되거나, sensitive change가 명시적 approval 없이 진행된다. Approval은 한 scope에 대해 부여됐는데 실제 write가 다른 path, command, network target, secret, baseline을 건드릴 수 있다.

하네스는 product write에 scoped Change Unit을 요구하고, sensitive category에는 explicit approval을 요구해 대응한다.

### Evidence Failure

Agent가 acceptance criteria에 연결된 durable evidence 없이 작업이 끝났다고 보고한다. Log, diff, check, evaluation report가 chat 안에 남거나 session과 함께 사라진다.

하네스는 evidence가 필요한 곳에 evidence coverage를 요구하고 raw evidence를 artifact store에 저장해 대응한다.

### Verification Failure

작업을 구현한 같은 agent가 self-review하고, system이 그것을 independent verification처럼 취급한다. 이는 confidence와 independence를 혼동한다.

하네스는 self-check와 detached verification을 분리하고, same-session review만으로 assurance를 올리지 않음으로써 대응한다.

### Human Judgment Failure

Approval, technical assurance, Manual QA, acceptance가 모호한 "looks good" 하나로 합쳐진다. 사용자는 어떤 질문에 답이 끝났는지 알 수 없다.

하네스는 이 판단을 분리해 대응한다.

- Approval: 이 sensitive change를 진행해도 되는가?
- Assurance: 결과가 기술적으로 어떻게 확인되었는가?
- Manual QA: 필요한 경우 사람이 experiential result를 inspected했는가?
- Acceptance: 사용자가 결과와 남은 trade-off를 받아들이는가?

### Projection Failure

생성된 document, stale summary, human-edited note가 canonical state처럼 취급된다. Document change가 조용히 operational truth를 바꾼다.

하네스는 Markdown report를 projection으로 취급해 대응한다. Human-editable area는 input surface이며, reconcile과 accepted state event를 통해서만 state가 된다.

## Minimal Harness Kernel

Minimal kernel은 core invariant를 보존하는 가장 작은 구현 가능한 mechanism이다.

- continuity와 write scope를 위한 Task 및 Change Unit record.
- state compatibility를 위한 lifecycle plus gates.
- 구별된 judgment를 위한 approval, evidence, verification, QA, acceptance record.
- product-write decision point로서의 `prepare_write`.
- completion decision point로서의 `close_task`.
- operational history를 위한 `state.sqlite` current record와 `state.sqlite.task_events`.
- raw evidence를 위한 artifact store.
- human-readable report와 user proposal surface를 위한 projection.

Kernel specification은 entity semantics, lifecycle field, gate enum, transition rule, close semantics, waiver semantics, invariant enforcement를 담당한다.

<a id="core-invariants"></a>
## 핵심 불변식

다음만 core invariant다. 이 중 하나를 위반하는 system은 더 이상 harness kernel을 구현하는 것이 아니다.

1. Chat은 state가 아니다.
2. Product write에는 활성 scoped Change Unit이 필요하다.
3. Sensitive change에는 explicit approval이 필요하다.
4. Completion에는 evidence가 필요한 곳의 evidence coverage가 필요하다.
5. Work는 detached verification을 self-certify할 수 없다.
6. Required QA와 acceptance는 별도의 gate다.
7. Projection은 canonical state를 override할 수 없다.

## Policy Defaults

다음은 core invariant가 아니라 design-quality policy default다. Product quality를 높인다는 점에서 중요하지만, 적용 규칙, 허용 waiver, required record, validator, close impact는 policy pack이 정의한다.

- Shared design for work.
- Domain language consistency.
- Vertical slice default.
- TDD trace for suitable work.
- Module and interface review.
- UI, UX, copy, accessibility, visual output, product taste를 위한 Manual QA.
- Context hygiene.

Strategy는 product experience를 shaping하기 때문에 이 default를 드러낸다. 세부 contract는 policy pack이 담당한다.

## Human Judgment Model

하네스는 사람이 direction과 judgment를 제공하고, agent가 option, implementation, evidence, structured status를 제공한다고 가정한다.

사람은 다음을 담당한다.

- goal과 priority
- scope confirmation
- sensitive-change approval
- product trade-off decision
- human inspection이 필요한 곳의 Manual QA result
- final acceptance 또는 rejection

Agent는 다음을 담당한다.

- choice와 risk 드러내기
- Change Unit 제안하기
- approved scope 안에 머물기
- run과 evidence 기록하기
- gate가 요구할 때 decision 요청하기
- required일 때 detached verification 시작 또는 manual evaluator instruction bundle 준비하기

Kernel은 다음을 담당한다.

- write 허용 여부
- task close 가능 여부
- evidence, verification, QA, acceptance state의 compatibility
- projection이 display로 믿을 만큼 최신인지 여부

이 모델은 사용자가 모든 file write나 status claim을 직접 감시하지 않아도 사용자의 통제권을 유지한다.

## Source-Of-Truth 요약

Canonical operating state는 `state.sqlite`다. 여기에는 current state record와 append-only `state.sqlite.task_events` table이 포함된다. MVP에는 별도의 event store가 없다.

Raw evidence는 artifact store 안에서 canonical하다. Artifact record와 reference는 durable file을 Task, Run, Evidence Manifest, Eval, Manual QA record, projection에 연결한다.

Markdown report는 state record와 artifact reference에서 생성되는 projection이다. Projection은 유용할 수도, stale일 수도, failed일 수도 있지만 canonical state를 override하지 않는다.

Human-editable section은 input surface다. User Notes는 다음 authority path를 따른다.

```text
human-editable input -> reconcile_items -> accepted state event/record
```

Domain Language, Module Map, Interface Contract projection도 같은 source-of-truth 원칙을 따른다. Canonical record는 kernel state 안에 있고, Markdown form은 human-readable projection이자 proposal surface다.

## Guarantee Level 요약

Guarantee level은 연결된 agent surface가 harness rule enforcement를 얼마나 강하게 도울 수 있는지를 설명한다.

- Cooperative guarantee: surface가 harness instruction과 MCP result를 따를 것으로 기대된다.
- Detective guarantee: 하네스가 관찰 후 violation을 detect하고 state를 blocked, stale, partial로 mark할 수 있다.
- Preventive guarantee: guard가 execution 전에 violation을 block할 수 있다.
- Isolated guarantee: 별도 worktree, sandbox, process boundary가 risky work를 isolate한다.

MVP reference surface는 주로 cooperative 및 detective다. Preventive와 isolated guarantee에는 더 강한 connector 또는 runtime capability가 필요하다. Capability는 kernel gate가 아니다. Surface capability validation, `prepare_write` blocked reason, user-facing guarantee display에 나타난다.

## MVP 경계

MVP는 broad agent-integration platform이 아니라 core invariant validation project다.

MVP 포함 항목:

- 하나의 local project registration
- 하나의 reference agent surface
- `state.sqlite` current record와 `state.sqlite.task_events`
- artifact registry와 artifact store
- public MCP tool surface
- `prepare_write` gatekeeping
- approval, evidence, verification, Manual QA, acceptance gate enforcement
- Task status, approval, run, evidence, Eval, direct result를 위한 required MVP report projection
- detached verification bundle 또는 manual evaluator instruction bundle
- basic doctor, recover, reconcile, export, conformance smoke path

MVP 제외 항목:

- reference surface를 넘어서는 broad connector coverage
- UI control plane과 automatic capture feature
- reference surface를 넘어서는 native hook expansion
- fully automatic parallel execution
- long-term analytics
- team workflow management

이후 자동화 항목은 [appendix/C-later-roadmap.md](appendix/C-later-roadmap.md)가 담당한다. 이후 자동화는 guarantee level을 강화할 수 있지만 core invariant model을 약화해서는 안 된다.
