# Conflict List and Resolutions

이 문서는 기존 문서 세트에서 발견된 모순, 불명확성, 과밀 지점을 기록하고 새 문서 세트에서의 해결 방침을 고정한다.

## Path Convention

```text
docs/README.md:
  harness documentation entrypoint

root README.md:
  repository landing page

All target documentation paths are interpreted under docs/
unless explicitly stated otherwise.
```

## Status Labels

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

## Conflict Type Labels

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

## C-01. `state.sqlite + event log` Ambiguity

### Existing Problem

여러 문서는 운영 상태의 기준을 `state.sqlite`와 event log라고 쓴다. 이 표현은 event log가 SQLite 내부 table인지, 별도 파일인지, 외부 stream인지 불명확하다.

### Resolution

Status: `RESOLVED`

MVP event log는 `state.sqlite.task_events` append-only table이다.

Canonical phrasing:

```text
운영 상태의 canonical source는 state.sqlite이다.
state.sqlite는 current state table과 append-only task_events table을 가진다.
```

### Apply To

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

## C-02. User Notes Authority Conflict

### Existing Problem

일부 문서는 사용자 메모의 canonical source를 human-editable 영역처럼 표현하고, 다른 문서는 reconcile item으로 표현한다.

### Resolution

Status: `RESOLVED`

```text
입력 원본:
  human-editable document section

반영 후보의 canonical record:
  state.sqlite.reconcile_items

반영 후 운영 사실:
  state.sqlite event + target record
```

### Apply To

```text
03-kernel-spec.md
07-document-projection.md
11-operations-and-conformance.md
glossary.md
```

## C-03. Domain Language Authority Conflict

### Existing Problem

`domain language record + reconciled doc` 표현은 source-of-truth와 projection을 섞는다.

### Resolution

Status: `RESOLVED`

```text
canonical source:
  state.sqlite.domain_terms

projection:
  DOMAIN-LANGUAGE

proposal path:
  human-editable section → reconcile_items → domain_terms
```

### Apply To

```text
03-kernel-spec.md
07-document-projection.md
08-design-quality-policy-pack.md
glossary.md
```

## C-04. Module Map / Interface Contract Authority Conflict

### Existing Problem

Module Map과 Interface Contract도 `record + reconciled doc` 또는 `design records + reconciled docs`로 표현되어 projection과 record의 경계가 흐리다.

### Resolution

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

### Existing Problem

전략은 work가 실행자의 자기 보고만으로 닫히지 않는다고 한다. 구현 문서는 detached verification passed 또는 accepted exception으로 close 가능하다고 한다. 이 둘은 충돌해 보인다.

### Resolution

Status: `RESOLVED`

Exception close는 가능하다. 하지만 detached verification처럼 표시하지 않는다.

```yaml
verification_gate: waived_by_user
assurance_level: self_checked
close_reason: completed_with_risk_accepted
```

### Required Wording

```text
사용자는 남은 verification risk를 수용해 작업을 닫을 수 있다.
그 경우 하네스는 이 작업을 detached_verified로 표시하지 않는다.
```

## C-06. Approval and Scope Confirmation Conflation

### Existing Problem

제품 파일 쓰기 전 “scope와 approval을 확인한다”는 표현은 모든 scope confirmation이 approval처럼 보이게 만들 수 있다.

### Resolution

Status: `RESOLVED`

```text
scope_gate:
  모든 write-capable run에 적용

approval_gate:
  sensitive category가 있을 때만 required
```

### Apply To

```text
02-strategy.md
03-kernel-spec.md
05-mcp-api-and-schemas.md
10-user-guide.md
```

## C-07. Core Invariant Overload

### Existing Problem

기존 전략 문서의 17개 불변식은 좋은 원칙을 많이 담지만, MVP kernel invariant와 design-quality policy default가 섞여 있다.

### Resolution

Status: `RESOLVED`

Core invariant는 7개로 축소한다. Shared design, domain language, vertical slice, TDD, module/interface review, manual QA, context hygiene는 policy default로 이동한다.

### Apply To

```text
02-strategy.md
08-design-quality-policy-pack.md
99-authoring-guide.md
```

## C-08. State Axis Density

### Existing Problem

기존 상태 축은 `mode`, `phase`, `result`, `assurance`, `verification_independence`, `approval`, `manual_qa`, `acceptance`, `risk`, `evidence`, `design_alignment`, `architecture` 등 많다. 조합 불가능 조건이 충분히 닫혀 있지 않다.

### Resolution

Status: `REWRITE`

`03-kernel-spec.md`에서 lifecycle + gates로 재작성한다.

```text
Lifecycle:
  mode, lifecycle_phase, result, close_reason

Gates:
  scope, approval, design, evidence, verification, QA, acceptance
```

## C-09. Strategy Owning State Machine

### Existing Problem

기존 전략 문서가 상태 축과 작업 모델을 많이 소유한다. 새 구조에서는 전략 문서가 구현 상태기계를 소유하면 과밀해진다.

### Resolution

Status: `MOVE_TO_OWNER`

```text
02-strategy.md:
  why, failure model, core invariants, policy defaults

03-kernel-spec.md:
  state machine, gates, transitions, close semantics
```

## C-10. Reference Implementation Too Broad

### Existing Problem

기존 reference implementation은 MVP, storage, Core, state contract, MCP, approval, evidence, artifact, verification, design-quality, validators, security, adapter, recovery를 모두 담는다.

### Resolution

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

## C-11. Full Template Overload

### Existing Problem

Document contracts 문서에 많은 full templates가 모두 같은 무게로 들어 있다. MVP 구현자가 모든 projection을 필수로 오해할 수 있다.

### Resolution

Status: `MOVE_TO_APPENDIX`

```text
07-document-projection.md:
  projection principles, authority, managed/human-editable, required MVP templates

appendix/A-template-library.md:
  full templates and expanded variants
```

## C-12. Surface-Specific Connector Details in Core Integration

### Existing Problem

Agent integration 문서에 Codex, Claude Code, Gemini, Copilot, Cursor addendum이 들어 있어 core connector contract와 cookbook이 섞인다.

### Resolution

Status: `MOVE_TO_APPENDIX`

```text
09-agent-integration.md:
  capability profile, connector contract, fallback, reference surface

appendix/B-surface-cookbook.md:
  surface-specific notes
```

## C-13. Guarantee Level Hidden Too Late

### Existing Problem

Architecture에 advisory/detective/preventive/isolated 보장 수준이 있으나 사용자/구현자에게 충분히 앞에서 드러나지 않는다.

### Resolution

Status: `REWRITE`

새 용어는 다음과 같이 통일한다.

```text
cooperative
detective
preventive
isolated
```

`02-strategy.md`, `04-runtime-architecture.md`, `09-agent-integration.md`, `10-user-guide.md`에서 각기 자기 층위에 맞게 설명한다.

## C-14. MCP Unavailable Write Hold Ambiguity

### Existing Problem

“MCP unavailable이면 product file write 보류”는 맞지만, surface 능력에 따라 preventive하게 강제할 수 없는 경우가 있다.

### Resolution

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

## C-15. `record_run` Payload Too Broad

### Existing Problem

`harness.record_run`이 design update, changed files, commands, logs, TDD trace, evidence mapping, run summary를 모두 받을 수 있어 over-broad JSON blob이 될 위험이 있다.

### Resolution

Status: `RESOLVED`

`kind` 필드를 필수로 둔다.

```yaml
kind: shaping_update | implementation | direct | verification_input
```

Tool별 schema는 `05-mcp-api-and-schemas.md`에서 kind별 discriminated union으로 정의한다.

## C-16. `request_user_decision` Payload Too Broad

### Existing Problem

approval, scope, unresolved decision, QA, acceptance, reconcile 판단이 하나의 tool 안에 있어 의미가 섞일 수 있다.

### Resolution

Status: `RESOLVED`

`decision_kind` 필드를 필수로 둔다.

```yaml
decision_kind:
  approval | scope_confirmation | design_choice |
  qa_waiver | acceptance | reconcile
```

## C-17. Direct Detached Verification Ambiguity

### Existing Problem

Direct는 self_checked로 닫을 수 있으나, conformance에는 direct 후 detached verify 가능성이 있다. Direct가 `detached_verified`를 가질 수 있는지 불명확하다.

### Resolution

Status: `RESOLVED`

Direct는 기본적으로 verification required가 아니다. 그러나 optional fresh verification이 수행되면 `assurance_level=detached_verified`를 가질 수 있다.

## C-18. Projection Stale as Close Blocker

### Existing Problem

Projection 실패가 state 실패인지, close를 막는지 불명확할 수 있다.

### Resolution

Status: `RESOLVED`

Projection stale/failed는 state failure가 아니며 기본적으로 close를 막지 않는다. 단, 사용자에게 반드시 표시한다.

```text
state current / projection stale
state current / projection failed
```

## C-19. Design-Quality Validators as MVP Core

### Existing Problem

vertical slice, TDD trace, module boundary, manual QA validators가 core invariant처럼 보인다.

### Resolution

Status: `REWRITE`

MVP에는 최소 validator hook만 둔다. 적용 조건과 waiver는 `08-design-quality-policy-pack.md`가 소유한다.

## C-20. Long-Term Metrics in Operations

### Existing Problem

운영 지표 목록이 MVP 운영 필수처럼 보일 수 있다.

### Resolution

Status: `LATER`

MVP operations에는 conformance와 doctor/recover 중심만 둔다. 장기 analytics와 metrics는 `appendix/C-later-roadmap.md`로 이동한다.

## C-21. Browser QA Capture and Dashboard in Main Docs

### Existing Problem

browser QA 자동 캡처, dashboard, cross-surface verify가 앞쪽 문서에 등장하면 MVP 요구처럼 보인다.

### Resolution

Status: `LATER`

All later automation goes to `appendix/C-later-roadmap.md`.

## C-22. User Guide Too Long

### Existing Problem

기존 User Guide는 실제 문구가 풍부하지만 너무 길어 quick-start 문서로 읽기 어렵다.

### Resolution

Status: `REWRITE`

`10-user-guide.md`는 짧고 대화 중심으로 재작성한다. 대표 예시만 남기고 반복되는 긴 examples는 제거한다.

## C-23. Authoring Guide Ownership Table Outdated

### Existing Problem

기존 Authoring Guide는 old document tree를 기준으로 ownership을 설명한다.

### Resolution

Status: `REWRITE`

`99-authoring-guide.md`는 새 tree, 새 owner map, schema ownership, template ownership, appendix ownership을 반영한다.

## C-24. Glossary Missing Gate and Guarantee Terms

### Existing Problem

새 모델에 필요한 Gate, Scope Gate, Approval Gate, Evidence Gate, Verification Gate, QA Gate, Acceptance Gate, Close Reason, Waiver, Guarantee Level 용어가 부족하다.

### Resolution

Status: `REWRITE`

`glossary.md`에 새 용어를 추가하고 기존 Source-of-truth, Projection, Domain Language, Reconcile, Assurance, Detached Verification 정의를 수정한다.

## C-25. Schema Ownership Ambiguity

### Existing Problem

MCP schema, validator result schema, artifact schema, SQLite DDL이 여러 문서에 분산될 수 있다.

### Resolution

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

## C-26. Root README vs Docs README Path Ambiguity

### Existing Problem

현재 저장소에는 root `README.md`와 `docs/README.md`가 모두 있다. root `README.md`는 짧은 프로젝트 설명이고, `docs/README.md`는 하네스 문서 세트의 현관이다. 사용자 요청과 `TARGET-DOC-TREE.md`는 `README.md`라고만 쓰기 때문에 rewrite 대상 경로가 모호하다.

### Resolution

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

### Apply To

```text
TARGET-DOC-TREE.md
DOC-OWNERSHIP-MAP.md
CODEX-BATCHES.md
REVIEW-CHECKLIST.md
PRESERVE-MOVE-LATER.md
```

## C-27. Runtime Home and Artifact Directory Layout Drift

### Existing Problem

`00-overview.md`, `03-architecture.md`, `04-reference-implementation.md`, `05-user-guide.md`, `08-operations-and-conformance.md`가 `~/.harness`와 artifact 하위 디렉터리를 조금씩 다르게 보여준다. 예시는 `config.yaml`, `traces`, `checkpoints`, `exports`, `manifests` 포함 여부가 문서마다 다르다.

### Resolution

Status: `REWRITE`

Exact runtime/artifact directory layout은 `06-reference-mvp.md`가 소유한다. `04-runtime-architecture.md`는 세 공간과 artifact store의 high-level 구조만 설명한다. README, introduction, user guide는 `~/.harness` 기본 위치와 `state.sqlite`, `artifacts/` 존재만 요약한다.

## C-28. MVP Template Tier vs Optional Design-Quality Projections

### Existing Problem

`04-reference-implementation.md`는 MVP 범위에 `DOMAIN-LANGUAGE`, `MODULE-MAP`, `INTERFACE-CONTRACT`, `TDD-TRACE`, `MANUAL-QA` 최소 projection을 포함한다. `KERNEL-DECISIONS.md`는 template tier를 Required MVP와 Optional design-quality로 나눈다. 이 둘을 그대로 두면 구현자가 design-quality projection 전체를 required MVP로 오해할 수 있다.

### Resolution

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

### Existing Problem

`02-strategy.md`, `05-user-guide.md`, `04-reference-implementation.md`가 민감 변경 범주를 반복한다. 일부는 사람이 읽는 설명이고 일부는 enum이다. 같은 목록처럼 보이지만 granularity가 다르다.

### Resolution

Status: `RESOLVED`

Canonical enum은 `05-mcp-api-and-schemas.md`가 소유한다. `10-user-guide.md`는 user-facing summary만 둔다. `02-strategy.md`는 “sensitive change requires explicit approval” 원칙만 둔다. `KD-21`의 minimum categories를 API schema 작성 기준으로 사용한다.

## C-30. Artifact / Report / Projection Boundary Ambiguity

### Existing Problem

일부 문서는 Artifact 예시에 run summary, eval report, TDD trace, manual QA record를 포함한다. 다른 문서는 이를 state record와 Markdown projection으로 다룬다. Raw artifact, state record, projection document의 경계가 흐려질 수 있다.

### Resolution

Status: `RESOLVED`

The boundary is fixed as follows.

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

### Apply To

```text
03-kernel-spec.md
04-runtime-architecture.md
05-mcp-api-and-schemas.md
07-document-projection.md
11-operations-and-conformance.md
glossary.md
```

## C-31. Conformance Scenario Tables vs Fixture-Based Conformance

### Existing Problem

`04-reference-implementation.md`, `06-agent-integration.md`, `08-operations-and-conformance.md`는 conformance를 시나리오 목록이나 체크리스트로 많이 설명한다. Rewrite brief와 `KD-17`은 fixture-based conformance를 요구한다.

### Resolution

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

## C-32. Security Boundary vs Guarantee Level Ambiguity

### Existing Problem

`04-reference-implementation.md`는 filesystem/process/network/credential/data boundary를 집행한다고 말한다. `06-agent-integration.md`는 surface capability와 fallback을 설명한다. Surface capability가 낮은 환경에서도 preventive security boundary가 항상 강제되는 것처럼 읽힐 수 있다.

### Resolution

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

## C-33. User Guide Setup/Install Detail Overload

### Existing Problem

`05-user-guide.md`는 대화 중심 quick start와 setup/connect/CLI 설명을 함께 가진다. User guide가 operator procedure를 소유하는 것처럼 보일 수 있다.

### Resolution

Status: `MOVE_TO_OWNER`

`10-user-guide.md`는 사용자가 말하고 읽는 흐름만 소유한다. Setup/connect command semantics와 doctor/recover/export는 `11-operations-and-conformance.md`가 소유한다. User guide는 “처음 한 번 연결한다” 수준의 짧은 안내만 둔다.

## C-34. Capability Gate vs Validator Boundary

### Existing Problem

`KD-04`와 일부 architecture/API wording이 capability를 별도 gate로 둘 수 있는 것처럼 읽힐 수 있다. 그러면 `03-kernel-spec.md`의 canonical gate list가 scope, approval, design, evidence, verification, QA, acceptance 밖으로 확장될 위험이 있다.

### Resolution

Status: `RESOLVED`

MVP does not have a first-class `capability_gate`.

Capability is represented by:

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

Required wording:

```text
Capability can block or downgrade a run through validator results,
prepare_write blocked reasons, and guarantee display. It is not a
kernel gate in MVP.
```

## C-35. EVAL Verdict vs Verification Gate vs Assurance Level

### Existing Problem

An `EVAL` projection can say `verdict: passed`, while the kernel also tracks `verification_gate` and `assurance_level`. Without an explicit rule, writers may treat any passed EVAL as enough to set `assurance_level=detached_verified`.

### Resolution

Status: `RESOLVED`

`EVAL` verdict alone does not upgrade assurance.

```text
assurance_level=detached_verified requires:
  - a passed verification result
  - a valid independence qualifier
  - same-session self-review guard not violated
```

Same-session review cannot produce `detached_verified`.

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

### Existing Problem

Existing docs use `manual_qa_state`, `MANUAL-QA.result`, and user card text interchangeably. This can make the record-level QA result look like the canonical kernel state.

### Resolution

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

## C-37. Evidence Gate Applicability

### Existing Problem

`evidence_gate` previously lacked a value for tasks where evidence coverage does not apply, such as advisor-only work. This made `none` ambiguous: it could mean evidence is irrelevant or evidence is required but absent.

### Resolution

Status: `RESOLVED`

Use this enum:

```yaml
evidence_gate:
  not_required | none | partial | sufficient | stale | blocked
```

Meaning:

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

## C-38. Legacy Docs Remaining Canonical

### Existing Problem

After v2 target docs are created, replaced legacy docs can remain in `docs/` and still look canonical. This risks stale links from `docs/README.md`, duplicate ownership, and consistency grep treating archived migration material as active main documentation.

Legacy docs include:

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

### Resolution

Status: `RESOLVED`

After content migration, legacy docs replaced by v2 docs must not remain as canonical docs.

Allowed treatments:

```text
DELETE:
  remove the legacy doc after migration

MIGRATION_STUB:
  replace with a short stub pointing to the v2 owner and migration notes

MOVE_TO_APPENDIX:
  move historical notes to docs/appendix/D-migration-notes.md
```

Rules:

```text
docs/README.md must not link to legacy docs except migration notes.
Final consistency grep must scan active canonical docs separately from
docs/appendix/D-migration-notes.md.
Archived migration notes are not active canonical docs.
```
