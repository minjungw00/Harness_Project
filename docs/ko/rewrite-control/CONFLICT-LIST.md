# Conflict List와 Resolution

이 문서는 기존 문서 세트에서 발견된 모순, 불명확성, 과밀 지점을 기록하고 새 문서 세트에서의 해결 방침을 고정한다.

## 경로 규칙

```text
docs/README.md:
  harness documentation entrypoint

root README.md:
  repository landing page

All target documentation paths are interpreted under docs/
unless explicitly stated otherwise.
```

## Status Label

```text
RESOLVED:
  KERNEL-DECISIONS.md에서 해결됨.

REWRITE:
  기존 설명의 의도는 보존하되 구조를 다시 씀.

MOVE_TO_OWNER:
  내용은 유지하되 canonical owner 문서로 이동.

MOVE_TO_APPENDIX:
  내용은 유지하되 appendix로 이동.

LATER:
  설계 재료로 보존하되 MVP 본문 요구에서 제거.

DELETE:
  기준 문서 본문에서 제거.

TODO_DECISION:
  아직 결정하지 않음. 본문에 임의 반영 금지.
```

## Conflict Type Label

```text
CONTRADICTION:
  두 문서가 동시에 참일 수 없는 기준을 말한다.

AMBIGUITY:
  구현자나 작성자가 둘 이상으로 해석할 수 있다.

OVERLOAD:
  한 문서나 개념이 여러 층위의 책임을 동시에 가진다.

SCOPE_LEAK:
  later 또는 appendix 내용이 MVP 본문 요구처럼 보인다.
```

## Conflict Index

| ID | Type | Short Name | Resolution Status |
|---|---|---|---|
| C-01 | AMBIGUITY | `state.sqlite + event log` | RESOLVED |
| C-02 | CONTRADICTION | User Notes authority | RESOLVED |
| C-03 | CONTRADICTION | Domain Language authority | RESOLVED |
| C-04 | CONTRADICTION | Module/Interface authority | RESOLVED |
| C-05 | CONTRADICTION | Verification invariant vs accepted exception | RESOLVED |
| C-06 | AMBIGUITY | Scope and approval conflation | RESOLVED |
| C-07 | OVERLOAD | Core invariant overload | RESOLVED |
| C-08 | OVERLOAD | State axis density | REWRITE |
| C-09 | OVERLOAD | Strategy owning state machine | MOVE_TO_OWNER |
| C-10 | OVERLOAD | Reference implementation too broad | REWRITE |
| C-11 | SCOPE_LEAK | Full template overload | MOVE_TO_APPENDIX |
| C-12 | OVERLOAD | Surface details in integration core | MOVE_TO_APPENDIX |
| C-13 | AMBIGUITY | Guarantee level naming and placement | REWRITE |
| C-14 | AMBIGUITY | MCP unavailable write hold | RESOLVED |
| C-15 | AMBIGUITY | `record_run` payload | RESOLVED |
| C-16 | AMBIGUITY | `request_user_decision` payload | RESOLVED |
| C-17 | AMBIGUITY | Direct detached verification | RESOLVED |
| C-18 | AMBIGUITY | Projection stale close behavior | RESOLVED |
| C-19 | OVERLOAD | Design validators as core | REWRITE |
| C-20 | SCOPE_LEAK | Long-term metrics | LATER |
| C-21 | SCOPE_LEAK | Browser QA/dashboard/cross-surface automation | LATER |
| C-22 | OVERLOAD | User Guide length | REWRITE |
| C-23 | CONTRADICTION | Authoring ownership table vs target tree | REWRITE |
| C-24 | AMBIGUITY | Missing gate/guarantee glossary terms | REWRITE |
| C-25 | OVERLOAD | Schema ownership | RESOLVED |
| C-26 | AMBIGUITY | Root README vs docs README path | RESOLVED |
| C-27 | CONTRADICTION | Runtime/artifact layout drift | REWRITE |
| C-28 | CONTRADICTION | MVP template tier vs optional design projections | RESOLVED |
| C-29 | AMBIGUITY | Sensitive category prose vs enum | RESOLVED |
| C-30 | AMBIGUITY | Artifact/report/projection boundary | RESOLVED |
| C-31 | OVERLOAD | Conformance scenario tables vs fixtures | REWRITE |
| C-32 | AMBIGUITY | Security boundary vs guarantee level | REWRITE |
| C-33 | SCOPE_LEAK | User guide setup/install detail | MOVE_TO_OWNER |
| C-34 | AMBIGUITY | Capability gate vs validator boundary | RESOLVED |
| C-35 | AMBIGUITY | EVAL verdict vs verification gate vs assurance | RESOLVED |
| C-36 | AMBIGUITY | Manual QA state vs QA gate | RESOLVED |
| C-37 | AMBIGUITY | Evidence gate applicability | RESOLVED |
| C-38 | SCOPE_LEAK | Legacy docs remaining canonical | RESOLVED |

## C-01. `state.sqlite + event log` 모호성

### 기존 문제

여러 문서는 운영 상태의 기준을 `state.sqlite`와 event log라고 쓴다. 이 표현은 event log가 SQLite 내부 table인지, 별도 파일인지, 외부 stream인지 불명확하다.

### 해결

Status: `RESOLVED`

MVP event log는 `state.sqlite.task_events` append-only table이다.

Canonical phrasing:

```text
운영 상태의 canonical source는 state.sqlite이다.
state.sqlite는 current state table과 append-only task_events table을 가진다.
```

### 적용 대상

```text
README.md
00-introduction.md
02-strategy.md
03-kernel-spec.md
04-runtime-architecture.md
06-reference-mvp.md
07-document-projection.md
99-authoring-guide.md
glossary.md
```

## C-02. User Notes Authority 충돌

### 기존 문제

일부 문서는 사용자 메모의 canonical source를 human-editable 영역처럼 표현하고, 다른 문서는 reconcile item으로 표현한다.

### 해결

Status: `RESOLVED`

```text
입력 원본:
  human-editable document section

반영 후보의 canonical record:
  state.sqlite.reconcile_items

반영 후 운영 사실:
  state.sqlite event + target record
```

### 적용 대상

```text
03-kernel-spec.md
07-document-projection.md
11-operations-and-conformance.md
glossary.md
```

## C-03. Domain Language Authority 충돌

### 기존 문제

`domain language record + reconciled doc` 표현은 source-of-truth와 projection을 섞는다.

### 해결

Status: `RESOLVED`

```text
canonical source:
  state.sqlite.domain_terms

projection:
  DOMAIN-LANGUAGE

proposal path:
  human-editable section → reconcile_items → domain_terms
```

### 적용 대상

```text
03-kernel-spec.md
07-document-projection.md
08-design-quality-policy-pack.md
glossary.md
```

## C-04. Module Map / Interface Contract Authority 충돌

### 기존 문제

Module Map과 Interface Contract도 `record + reconciled doc` 또는 `design records + reconciled docs`로 표현되어 projection과 record의 경계가 흐리다.

### 해결

Status: `RESOLVED`

```text
module map canonical source:
  state.sqlite.module_map_items

interface contract canonical source:
  state.sqlite.interface_contracts

projection:
  MODULE-MAP, INTERFACE-CONTRACT
```

## C-05. Detached Verification Invariant vs Accepted Exception

### 기존 문제

전략은 work가 실행자의 자기 보고만으로 닫히지 않는다고 한다. 구현 문서는 detached verification passed 또는 accepted exception으로 close 가능하다고 한다. 이 둘은 충돌해 보인다.

### 해결

Status: `RESOLVED`

Exception close는 가능하다. 하지만 detached verification처럼 표시하지 않는다.

```yaml
verification_gate: waived_by_user
assurance_level: self_checked
close_reason: completed_with_risk_accepted
```

### 필수 문구

```text
사용자는 남은 verification risk를 수용해 작업을 닫을 수 있다.
그 경우 하네스는 이 작업을 detached_verified로 표시하지 않는다.
```

## C-06. Approval과 Scope Confirmation 혼합

### 기존 문제

제품 파일 쓰기 전 “scope와 approval을 확인한다”는 표현은 모든 scope confirmation이 approval처럼 보이게 만들 수 있다.

### 해결

Status: `RESOLVED`

```text
scope_gate:
  모든 write-capable run에 적용

approval_gate:
  sensitive category가 있을 때만 required
```

### 적용 대상

```text
02-strategy.md
03-kernel-spec.md
05-mcp-api-and-schemas.md
10-user-guide.md
```

## C-07. Core Invariant 과부하

### 기존 문제

기존 전략 문서의 17개 불변식은 좋은 원칙을 많이 담지만, MVP kernel invariant와 design-quality policy default가 섞여 있다.

### 해결

Status: `RESOLVED`

Core invariant는 7개로 축소한다. Shared design, domain language, vertical slice, TDD, module/interface review, manual QA, context hygiene는 policy default로 이동한다.

### 적용 대상

```text
02-strategy.md
08-design-quality-policy-pack.md
99-authoring-guide.md
```

## C-08. State Axis Density

### 기존 문제

기존 상태 축은 `mode`, `phase`, `result`, `assurance`, `verification_independence`, `approval`, `manual_qa`, `acceptance`, `risk`, `evidence`, `design_alignment`, `architecture` 등 많다. 조합 불가능 조건이 충분히 닫혀 있지 않다.

### 해결

Status: `REWRITE`

`03-kernel-spec.md`에서 lifecycle + gates로 재작성한다.

```text
Lifecycle:
  mode, lifecycle_phase, result, close_reason

Gates:
  scope, approval, design, evidence, verification, QA, acceptance
```

## C-09. Strategy가 State Machine을 소유함

### 기존 문제

기존 전략 문서가 상태 축과 작업 모델을 많이 소유한다. 새 구조에서는 전략 문서가 구현 상태기계를 소유하면 과밀해진다.

### 해결

Status: `MOVE_TO_OWNER`

```text
02-strategy.md:
  why, failure model, core invariants, policy defaults

03-kernel-spec.md:
  state machine, gates, transitions, close semantics
```

## C-10. Reference Implementation 범위 과다

### 기존 문제

기존 reference implementation은 MVP, storage, Core, state contract, MCP, approval, evidence, artifact, verification, design-quality, validators, security, adapter, recovery를 모두 담는다.

### 해결

Status: `REWRITE`

Split into:

```text
03-kernel-spec.md:
  entity/state/gate/transition

05-mcp-api-and-schemas.md:
  MCP resources/tools/schema/error/validator result

06-reference-mvp.md:
  implementation sequence, SQLite DDL, artifact layout, reference surface

appendix/C-later-roadmap.md:
  later automation
```

## C-11. Full Template 과부하

### 기존 문제

Document contracts 문서에 많은 full templates가 모두 같은 무게로 들어 있다. MVP 구현자가 모든 projection을 필수로 오해할 수 있다.

### 해결

Status: `MOVE_TO_APPENDIX`

```text
07-document-projection.md:
  projection principles, authority, managed/human-editable, required MVP templates

appendix/A-template-library.md:
  full templates and expanded variants
```

## C-12. Core Integration 안의 Surface-Specific Connector Detail

### 기존 문제

Agent integration 문서에 Codex, Claude Code, Gemini, Copilot, Cursor addendum이 들어 있어 core connector contract와 cookbook이 섞인다.

### 해결

Status: `MOVE_TO_APPENDIX`

```text
09-agent-integration.md:
  capability profile, connector contract, fallback, reference surface

appendix/B-surface-cookbook.md:
  surface-specific notes
```

## C-13. Guarantee Level이 너무 늦게 드러남

### 기존 문제

Architecture에 advisory/detective/preventive/isolated 보장 수준이 있으나 사용자/구현자에게 충분히 앞에서 드러나지 않는다.

### 해결

Status: `REWRITE`

새 용어는 다음과 같이 통일한다.

```text
cooperative
detective
preventive
isolated
```

`02-strategy.md`, `04-runtime-architecture.md`, `09-agent-integration.md`, `10-user-guide.md`에서 각기 자기 층위에 맞게 설명한다.

## C-14. MCP Unavailable Write Hold 모호성

### 기존 문제

“MCP unavailable이면 product file write 보류”는 맞지만, surface 능력에 따라 preventive하게 강제할 수 없는 경우가 있다.

### 해결

Status: `RESOLVED`

```text
T2 MCP + no guard:
  cooperative guarantee

T3/T4 sidecar/guard:
  detective/preventive guarantee 가능

T5 isolation:
  isolated guarantee 가능
```

문서에는 “보류해야 한다”와 “강제할 수 있다”를 구분한다.

## C-15. `record_run` Payload 범위 과다

### 기존 문제

`harness.record_run`이 design update, changed files, commands, logs, TDD trace, evidence mapping, run summary를 모두 받을 수 있어 over-broad JSON blob이 될 위험이 있다.

### 해결

Status: `RESOLVED`

`kind` 필드를 필수로 둔다.

```yaml
kind: shaping_update | implementation | direct | verification_input
```

Tool별 schema는 `05-mcp-api-and-schemas.md`에서 kind별 discriminated union으로 정의한다.

## C-16. `request_user_decision` Payload 범위 과다

### 기존 문제

approval, scope, unresolved decision, QA, acceptance, reconcile 판단이 하나의 tool 안에 있어 의미가 섞일 수 있다.

### 해결

Status: `RESOLVED`

`decision_kind` 필드를 필수로 둔다.

```yaml
decision_kind:
  approval | scope_confirmation | design_choice |
  qa_waiver | acceptance | reconcile
```

## C-17. Direct Detached Verification 모호성

### 기존 문제

Direct는 self_checked로 닫을 수 있으나, conformance에는 direct 후 detached verify 가능성이 있다. Direct가 `detached_verified`를 가질 수 있는지 불명확하다.

### 해결

Status: `RESOLVED`

Direct는 기본적으로 verification required가 아니다. 그러나 optional fresh verification이 수행되면 `assurance_level=detached_verified`를 가질 수 있다.

## C-18. Close Blocker로서의 Projection Stale

### 기존 문제

Projection 실패가 state 실패인지, close를 막는지 불명확할 수 있다.

### 해결

Status: `RESOLVED`

Projection stale/failed는 state failure가 아니며 기본적으로 close를 막지 않는다. 단, 사용자에게 반드시 표시한다.

```text
state current / projection stale
state current / projection failed
```

## C-19. MVP Core로 보이는 Design-Quality Validator

### 기존 문제

vertical slice, TDD trace, module boundary, manual QA validators가 core invariant처럼 보인다.

### 해결

Status: `REWRITE`

MVP에는 최소 validator hook만 둔다. 적용 조건과 waiver는 `08-design-quality-policy-pack.md`가 소유한다.

## C-20. Operations 안의 Long-Term Metrics

### 기존 문제

운영 지표 목록이 MVP 운영 필수처럼 보일 수 있다.

### 해결

Status: `LATER`

MVP operations에는 conformance와 doctor/recover 중심만 둔다. 장기 analytics와 metrics는 `appendix/C-later-roadmap.md`로 이동한다.

## C-21. Main Docs 안의 Browser QA Capture와 Dashboard

### 기존 문제

browser QA 자동 캡처, dashboard, cross-surface verify가 앞쪽 문서에 등장하면 MVP 요구처럼 보인다.

### 해결

Status: `LATER`

모든 later automation은 `appendix/C-later-roadmap.md`로 보낸다.

## C-22. User Guide가 너무 김

### 기존 문제

기존 User Guide는 실제 문구가 풍부하지만 너무 길어 quick-start 문서로 읽기 어렵다.

### 해결

Status: `REWRITE`

`10-user-guide.md`는 짧고 대화 중심으로 재작성한다. 대표 예시만 남기고 반복되는 긴 examples는 제거한다.

## C-23. Authoring Guide Ownership Table 오래됨

### 기존 문제

기존 Authoring Guide는 old document tree를 기준으로 ownership을 설명한다.

### 해결

Status: `REWRITE`

`99-authoring-guide.md`는 새 tree, 새 owner map, schema ownership, template ownership, appendix ownership을 반영한다.

## C-24. Glossary에 Gate와 Guarantee 용어 부족

### 기존 문제

새 모델에 필요한 Gate, Scope Gate, Approval Gate, Evidence Gate, Verification Gate, QA Gate, Acceptance Gate, Close Reason, Waiver, Guarantee Level 용어가 부족하다.

### 해결

Status: `REWRITE`

`glossary.md`에 새 용어를 추가하고 기존 Source-of-truth, Projection, Domain Language, Reconcile, Assurance, Detached Verification 정의를 수정한다.

## C-25. Schema Ownership 모호성

### 기존 문제

MCP schema, validator result schema, artifact schema, SQLite DDL이 여러 문서에 분산될 수 있다.

### 해결

Status: `RESOLVED`

```text
MCP wire schema:
  05-mcp-api-and-schemas.md

validator result schema:
  05-mcp-api-and-schemas.md

artifact ref schema:
  05-mcp-api-and-schemas.md

SQLite DDL:
  06-reference-mvp.md

state transition:
  03-kernel-spec.md
```

## C-26. Root README vs Docs README Path 모호성

### 기존 문제

현재 저장소에는 root `README.md`와 `docs/README.md`가 모두 있다. root `README.md`는 짧은 프로젝트 설명이고, `docs/README.md`는 하네스 문서 세트의 현관이다. 사용자 요청과 `TARGET-DOC-TREE.md`는 `README.md`라고만 쓰기 때문에 rewrite 대상 경로가 모호하다.

### 해결

Status: `RESOLVED`

Use Option A.

```text
docs/README.md:
  harness documentation entrypoint

root README.md:
  repository landing page

Path convention:
  All target documentation paths are interpreted under docs/
  unless explicitly stated otherwise.
```

### 적용 대상

```text
TARGET-DOC-TREE.md
DOC-OWNERSHIP-MAP.md
CODEX-BATCHES.md
REVIEW-CHECKLIST.md
PRESERVE-MOVE-LATER.md
```

## C-27. Runtime Home과 Artifact Directory Layout Drift

### 기존 문제

`00-overview.md`, `03-architecture.md`, `04-reference-implementation.md`, `05-user-guide.md`, `08-operations-and-conformance.md`가 `~/.harness`와 artifact 하위 디렉터리를 조금씩 다르게 보여준다. 예시는 `config.yaml`, `traces`, `checkpoints`, `exports`, `manifests` 포함 여부가 문서마다 다르다.

### 해결

Status: `REWRITE`

Exact runtime/artifact directory layout은 `06-reference-mvp.md`가 소유한다. `04-runtime-architecture.md`는 세 공간과 artifact store의 high-level 구조만 설명한다. README, introduction, user guide는 `~/.harness` 기본 위치와 `state.sqlite`, `artifacts/` 존재만 요약한다.

## C-28. MVP Template Tier vs Optional Design-Quality Projection

### 기존 문제

`04-reference-implementation.md`는 MVP 범위에 `DOMAIN-LANGUAGE`, `MODULE-MAP`, `INTERFACE-CONTRACT`, `TDD-TRACE`, `MANUAL-QA` 최소 projection을 포함한다. `KERNEL-DECISIONS.md`는 template tier를 Required MVP와 Optional design-quality로 나눈다. 이 둘을 그대로 두면 구현자가 design-quality projection 전체를 required MVP로 오해할 수 있다.

### 해결

Status: `RESOLVED`

`KD-15`를 따른다.

```text
Required MVP:
  TASK, APR, RUN-SUMMARY, EVIDENCE-MANIFEST, EVAL, DIRECT-RESULT

Optional design-quality:
  DOMAIN-LANGUAGE, MODULE-MAP, INTERFACE-CONTRACT, TDD-TRACE, MANUAL-QA
```

Design-quality projection은 policy/gate를 지원하는 optional projection으로 설명한다. Required/optional/full variant 구분은 `07-document-projection.md`와 `appendix/A-template-library.md`가 소유한다.

## C-29. Sensitive Category Prose vs Canonical Enum

### 기존 문제

`02-strategy.md`, `05-user-guide.md`, `04-reference-implementation.md`가 민감 변경 범주를 반복한다. 일부는 사람이 읽는 설명이고 일부는 enum이다. 같은 목록처럼 보이지만 granularity가 다르다.

### 해결

Status: `RESOLVED`

Canonical enum은 `05-mcp-api-and-schemas.md`가 소유한다. `10-user-guide.md`는 user-facing summary만 둔다. `02-strategy.md`는 “sensitive change requires explicit approval” 원칙만 둔다. `KD-21`의 minimum categories를 API schema 작성 기준으로 사용한다.

## C-30. Artifact / Report / Projection Boundary 모호성

### 기존 문제

일부 문서는 Artifact 예시에 run summary, eval report, TDD trace, manual QA record를 포함한다. 다른 문서는 이를 state record와 Markdown projection으로 다룬다. Raw artifact, state record, projection document의 경계가 흐려질 수 있다.

### 해결

Status: `RESOLVED`

경계는 다음과 같이 고정한다.

```text
Raw artifacts:
  durable evidence files in the artifact store

State records:
  canonical structured records in state.sqlite

Markdown reports:
  projections generated from records and artifact refs

RUN-SUMMARY, EVAL, TDD-TRACE, MANUAL-QA,
EVIDENCE-MANIFEST, DIRECT-RESULT:
  not raw artifacts by default

Export bundles:
  may include projections with hashes, but that does not make
  those projections canonical raw evidence artifacts
```

### 적용 대상

```text
03-kernel-spec.md
04-runtime-architecture.md
05-mcp-api-and-schemas.md
07-document-projection.md
11-operations-and-conformance.md
glossary.md
```

## C-31. Conformance Scenario Table vs Fixture-Based Conformance

### 기존 문제

`04-reference-implementation.md`, `06-agent-integration.md`, `08-operations-and-conformance.md`는 conformance를 시나리오 목록이나 체크리스트로 많이 설명한다. Rewrite brief와 `KD-17`은 fixture-based conformance를 요구한다.

### 해결

Status: `REWRITE`

`11-operations-and-conformance.md`는 fixture format을 소유한다.

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

기존 scenario/checklist 내용은 fixture examples의 source material로 보존하되, 본문 기준은 fixture shape로 재작성한다.

## C-32. Security Boundary vs Guarantee Level 모호성

### 기존 문제

`04-reference-implementation.md`는 filesystem/process/network/credential/data boundary를 집행한다고 말한다. `06-agent-integration.md`는 surface capability와 fallback을 설명한다. Surface capability가 낮은 환경에서도 preventive security boundary가 항상 강제되는 것처럼 읽힐 수 있다.

### 해결

Status: `REWRITE`

Security boundary의 desired policy는 보존한다. 하지만 guarantee는 capability에 맞춰 표시한다.

```text
cooperative:
  surface가 절차를 따르는 전제

detective:
  validator/sidecar가 위반을 감지

preventive:
  hook/policy engine이 실행 전 차단

isolated:
  worktree/sandbox/process 격리
```

`04-runtime-architecture.md`는 boundary placement를, `09-agent-integration.md`는 capability expression을, `06-reference-mvp.md`는 MVP reference surface의 실제 guarantee를 소유한다.

## C-33. User Guide Setup/Install Detail 과부하

### 기존 문제

`05-user-guide.md`는 대화 중심 quick start와 setup/connect/CLI 설명을 함께 가진다. User guide가 operator procedure를 소유하는 것처럼 보일 수 있다.

### 해결

Status: `MOVE_TO_OWNER`

`10-user-guide.md`는 사용자가 말하고 읽는 흐름만 소유한다. Setup/connect command semantics와 doctor/recover/export는 `11-operations-and-conformance.md`가 소유한다. User guide는 “처음 한 번 연결한다” 수준의 짧은 안내만 둔다.

## C-34. Capability Gate vs Validator Boundary

### 기존 문제

`KD-04`와 일부 architecture/API wording이 capability를 별도 gate로 둘 수 있는 것처럼 읽힐 수 있다. 그러면 `03-kernel-spec.md`의 canonical gate list가 scope, approval, design, evidence, verification, QA, acceptance 밖으로 확장될 위험이 있다.

### 해결

Status: `RESOLVED`

MVP에는 first-class `capability_gate`가 없다.

Capability는 다음으로 표현한다.

```text
surface_capability_check validator
prepare_write blocked_reasons
guarantee level display
```

Ownership:

```text
09-agent-integration.md:
  capability profile

04-runtime-architecture.md:
  guarantee level architecture

03-kernel-spec.md:
  canonical gate list, excluding capability_gate
```

필수 문구:

```text
Capability can block or downgrade a run through validator results,
prepare_write blocked reasons, and guarantee display. It is not a
kernel gate in MVP.
```

## C-35. EVAL Verdict vs Verification Gate vs Assurance Level

### 기존 문제

`EVAL` projection은 `verdict: passed`라고 말할 수 있고, kernel은 동시에 `verification_gate`와 `assurance_level`을 track한다. Explicit rule이 없으면 writer는 passed EVAL이면 언제나 `assurance_level=detached_verified`로 충분하다고 취급할 수 있다.

### 해결

Status: `RESOLVED`

`EVAL` verdict만으로 assurance를 upgrade하지 않는다.

```text
assurance_level=detached_verified requires:
  - a passed verification result
  - a valid independence qualifier
  - same-session self-review guard not violated
```

Same-session review는 `detached_verified`를 만들 수 없다.

Owner:

```text
03-kernel-spec.md:
  assurance update rule

05-mcp-api-and-schemas.md:
  record_eval request/response fields and errors

07-document-projection.md:
  EVAL projection wording
```

## C-36. Manual QA State vs QA Gate

### 기존 문제

Existing docs use `manual_qa_state`, `MANUAL-QA.result`, and user card text interchangeably. This can make the record-level QA result look like the canonical kernel state.

### 해결

Status: `RESOLVED`

```text
qa_gate:
  canonical kernel gate

manual_qa_record.result:
  record-level result

User-facing cards:
  may say Manual QA: pending/passed/failed/waived
```

Owner:

```text
03-kernel-spec.md:
  qa_gate and close semantics

07-document-projection.md:
  MANUAL-QA projection and card wording

10-user-guide.md:
  user-facing explanation
```

## C-37. Evidence Gate 적용 여부

### 기존 문제

`evidence_gate` previously lacked a value for tasks where evidence coverage does not apply, such as advisor-only work. This made `none` ambiguous: it could mean evidence is irrelevant or evidence is required but absent.

### 해결

Status: `RESOLVED`

이 enum을 사용한다.

```yaml
evidence_gate:
  not_required | none | partial | sufficient | stale | blocked
```

의미:

```text
not_required:
  evidence gate does not apply, for example advisor-only work

none:
  evidence is required but no evidence has been recorded yet
```

Owner:

```text
03-kernel-spec.md:
  evidence_gate enum and close semantics

07-document-projection.md:
  user-facing projection and card wording
```

## C-38. Legacy Docs가 Canonical로 남는 문제

### 기존 문제

After v2 target docs are created, replaced legacy docs can remain in `docs/` and still look canonical. This risks stale links from `docs/README.md`, duplicate ownership, and consistency grep treating archived migration material as active main documentation.

Legacy doc 포함:

```text
docs/00-overview.md
docs/03-architecture.md
docs/04-reference-implementation.md
docs/05-user-guide.md
docs/06-agent-integration.md
docs/07-document-and-artifact-contracts.md
docs/08-operations-and-conformance.md
docs/09-design-quality-playbooks.md
```

### 해결

Status: `RESOLVED`

Content migration 후 v2 doc으로 replaced된 legacy doc은 canonical doc으로 남으면 안 된다.

허용 처리:

```text
DELETE:
  remove the legacy doc after migration

MIGRATION_STUB:
  replace with a short stub pointing to the v2 owner and migration notes

MOVE_TO_APPENDIX:
  move historical notes to docs/appendix/D-migration-notes.md
```

규칙:

```text
docs/README.md must not link to legacy docs except migration notes.
Final consistency grep must scan active canonical docs separately from
docs/appendix/D-migration-notes.md.
Archived migration notes are not active canonical docs.
```
