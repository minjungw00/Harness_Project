# Agent 통합

## 문서 역할

이 문서는 agent surface를 하네스에 연결하기 위한 common integration contract를 담당한다. Capability tier, capability profile, generated manifest expectation, context push/pull principle, fallback semantic, reference surface contract, connector conformance overview를 정의한다.

본문은 product-name-neutral하다. Surface-specific recipe는 [Appendix B](appendix/B-surface-cookbook.md)에 있다.

이 문서는 kernel state transition, MCP request/response schema, SQLite DDL, capability gate, operational fixture detail, surface-specific cookbook을 정의하지 않는다.

## 통합 목표

Integration goal은 사용자가 agent와 자연스럽게 대화하는 동안 하네스가 bounded work, state recording, evidence, verification, Manual QA, acceptance, projection, reconcile flow를 뒤에서 제공하는 것이다.

Integrated surface는 agent가 다음을 하도록 도와야 한다.

- status 또는 intake로 시작
- advisor, direct, work mode classify
- work를 scoped Change Unit으로 shaping
- design-quality policy가 적용될 때 check
- state change를 위해 MCP tool call
- product write 전에 `prepare_write` 존중
- run, artifact, evidence, decision, QA, acceptance 기록
- detached verification launch 또는 package
- projection refresh 또는 reconcile

## 공통 통합 구조

```text
user conversation surface
  -> short always-on rules/context
  -> harness skill, command, or playbook
  -> harness MCP server
  -> harness Core
  -> adapter, hook, sidecar, validator, or isolation layer
```

### Always-On Rules

Always-on rule은 짧아야 한다. Agent에게 언제 harness를 사용하는지, status를 어디서 읽는지, product write에는 `prepare_write`가 필요하다는 점을 알려야 한다.

Full state transition table, MCP schema, full template, long design playbook, 모든 historical project context를 포함해서는 안 된다.

### Skill Or Playbook Layer

Skill/playbook layer는 procedure를 가르친다.

- status, intake, next를 언제 call할지
- advisor/direct/work를 classify하는 법
- shaping question을 묻는 법
- Change Unit을 form하는 법
- approval을 request하는 법
- TDD trace, evidence, Manual QA, acceptance를 record하는 법
- work verification이 detached되어야 하는 이유
- stale projection과 reconcile을 처리하는 법

Core와 validator가 policy를 enforce한다. Skill은 guidance이지 authority가 아니다.

### MCP Layer

MCP는 preferred state boundary다. Public tool name과 schema는 MCP API document가 담당한다. Integration doc은 tool intent를 reference할 수 있지만, connector는 `05-mcp-api-and-schemas.md`의 schema를 사용해야 한다.

### Adapter, Hook, Sidecar, Validator, Isolation

Adapter와 sidecar는 surface behavior를 observable fact 또는 stronger enforcement로 변환한다.

- artifact capture
- command output capture
- changed-path detection
- generated file drift detection
- projection freshness detection
- approval and scope guard support
- same-session verification guard support
- evaluator read-only or fresh-context support
- Manual QA capture support

이 layer는 guarantee level을 improve할 수 있지만 kernel capability gate를 만들지는 않는다.

## Capability Tier

| Tier | Meaning | Typical capability |
|---|---|---|
| `T0 Context` | Surface가 harness principle을 read할 수 있음 | rules/context file |
| `T1 Skill` | Surface가 harness procedure를 follow할 수 있음 | skill, command, prompt, playbook |
| `T2 MCP` | Surface가 harness tool과 resource를 call할 수 있음 | MCP server connection |
| `T3 Capture` | Surface가 diff, log, run output을 reliably return할 수 있음 | structured output, wrapper, adapter |
| `T4 Guard` | Surface가 execution 전에 out-of-scope file, command, network, secret을 block 또는 interrupt할 수 있음 | hook, permission system, policy engine, sidecar |
| `T5 Isolation` | Surface가 verification 또는 risky work를 별도 boundary에서 run할 수 있음 | worktree, sandbox, fresh process, isolated runner |
| `T6 QA Capture` | Surface가 browser, screenshot, walkthrough, Manual QA artifact를 structure할 수 있음 | browser runner, screenshot capture, QA note capture |

일반 interactive harness use는 `T2` 이상에서 가장 자연스럽다. Reliable detached verification에는 보통 `T3` capture와 real independence boundary가 필요하다. High-risk work는 가능하면 `T4` guard 또는 `T5` isolation을 사용해야 한다. `T6`는 UI/UX evidence를 개선하지만, human QA note를 기록할 수 있다면 MVP에 required는 아니다.

## Capability Profile

Harness connector는 product 또는 surface name에서 behavior를 가정하지 않고 capability profile을 사용해야 한다.

```yaml
surface_id: SURF-0001
surface_kind: generic_agent
target_profile: local_cli
detected_version: optional string
capability_profile_version: 1
last_verified_at: 2026-05-06T10:05:00+09:00
support_tier: T2
guarantee_level: cooperative
capabilities:
  project_rules: true
  skills_or_commands: true
  mcp_tools: true
  mcp_resources: true
  structured_output: false
  artifact_capture: manual
  hooks: false
  pre_tool_guard: false
  explicit_permissions: false
  changed_path_detection: validator
  fresh_verify: manual_bundle
  worktree_isolation: false
  local_sidecar: false
  browser_qa_capture: false
  screenshot_capture: false
risks:
  - no pre-tool guard
fallbacks:
  - cooperative prepare_write discipline
  - changed_paths validator
  - manual verification bundle
```

Target profile value 예시:

- `local_cli`
- `ide_chat`
- `ide_agent`
- `cloud_agent`
- `extension`
- `custom_agent`
- `manual_bundle`

Capability profile은 version, MCP config, hook, permission, workspace policy, generated file, conformance result, capture method, QA capture method가 바뀌면 refresh해야 한다.

## Guarantee Level

Integration은 enforcement strength를 정직하게 report해야 한다.

| Level | Integration meaning |
|---|---|
| `cooperative` | Surface가 harness instruction과 MCP result를 따를 것으로 기대된다. |
| `detective` | 하네스가 사후 violation을 observe하고 state를 blocked, stale, partial, failed로 mark할 수 있다. |
| `preventive` | Connector 또는 runtime이 violating action을 execution 전에 block할 수 있다. |
| `isolated` | Risky work가 worktree, sandbox, process boundary 또는 동등한 수단으로 분리된다. |

Guarantee level은 risk context와 display다. Approval, verification, acceptance, kernel gate가 아니다.

## Guarantee Display Requirements

Surface behavior에 의존하는 모든 status 또는 `prepare_write` result는 actual guarantee level을 보여야 한다. Level은 surface name에서 추론한 약속이 아니라 connected profile과 current enforcement path의 property로 표시한다.

User-visible examples:

| Level | Example display text |
|---|---|
| `cooperative` | "This surface is expected to follow Harness decisions, but Harness may not physically block an out-of-scope write before it happens. Changed-path validation can detect violations afterward." |
| `detective` | "Harness can observe changed paths or artifacts after action and mark scope/evidence/projection stale or blocked." |
| `preventive` | "A hook, wrapper, permission layer, or sidecar can block a violating write before execution." |
| `isolated` | "Risky work or verification runs in a separate worktree, sandbox, process, or equivalent boundary." |

Rules:

- Cooperative가 preventive라는 뜻으로 보이면 안 된다.
- Surface name이 level을 보장한다는 뜻으로 보이면 안 된다.
- Guarantee level은 approval, verification, QA, acceptance, kernel gate가 아니다.

## Generated Manifest Concept

Connector는 rule, skill, MCP config snippet, prompt, local adapter file을 generate할 수 있다. 모든 generated 또는 managed path는 connector manifest에 기록해야 한다.

Manifest responsibility:

- generated path naming
- managed block hash 기록
- generated 시 사용한 capability profile 기록
- surface target profile 기록
- creation/update time 기록
- human edit를 overwrite하기 전에 drift detect
- 필요할 때 drift를 reconcile로 route

Manifest concept는 common하다. Surface-specific generated filename은 Appendix B에 둔다.

## Push/Pull Context

Implementation agent는 작은 current context를 받고, 큰 reference는 필요할 때만 pull해야 한다.

보통 push:

- active Task status
- next action
- active Change Unit scope
- acceptance criteria snapshot
- allowed path와 tool
- relevant할 때 approval status
- latest evidence manifest와 run summary ref
- relevant policy warning

보통 pull:

- coding standard
- domain language
- module map
- interface contract
- TDD guidance
- architecture playbook
- old PRD, old design, closed issue
- raw log와 large diff

Evaluator는 acceptance criteria, changed file, approval scope, relevant domain/module/interface record, evidence manifest, required인 경우 TDD trace, Manual QA requirement, artifact ref, forbidden pattern을 포함한 더 tight한 verification bundle을 받아야 한다.

이 context model은 Context Hygiene policy를 지원한다. Current state와 evidence는 stale chat이나 old doc보다 우선된다.

## Fallback Semantics

Fallback은 surface name이 아니라 guarantee level과 risk로 설명한다.

### Cooperative Fallback

Surface가 instruction을 따를 수 있지만 enforce할 수 없을 때 사용한다. Connector는 agent에게 `prepare_write`를 call하고, blocked decision에서는 hold하고, run을 record하라고 알려준다. MCP가 unavailable이거나 write scope를 check할 수 없으면 product write를 pause해야 한다.

### Detective Fallback

Harness가 action 후 changed file, log, projection drift, artifact gap을 observe할 수 있을 때 사용한다. Validator는 state를 stale, partial, blocked, failed로 mark하고 repair, reconcile, fresh verification을 요구할 수 있다.

### Preventive Fallback

Hook, permission layer, wrapper, policy engine, sidecar가 violating edit, command, network call, secret access를 발생 전에 block할 수 있을 때 사용한다.

### Isolated Fallback

Risk에 separation이 필요할 때 사용한다. Connector는 별도 worktree, sandbox, process, manual evaluator bundle에서 work 또는 verification을 launch한다. Same-session review가 qualify하지 않는 detached verification에는 이것이 preferred fallback이다.

### MCP Unavailable

MCP가 unavailable이면 connector는 authoritative state update를 claim하면 안 된다. Product write의 safe behavior는 write를 hold하고 user/operator에게 MCP reconnect 또는 diagnose를 안내하는 것이다. Stronger profile은 preventive block도 enforce할 수 있다.

### Weak Guard

MCP는 동작하지만 pre-tool guard가 weak하면 low-risk direct work는 cooperative `prepare_write`와 detective changed-path validation으로 진행할 수 있다. Medium/high-risk work에는 stricter validation, sidecar guard, explicit approval, detached verification, isolation이 필요해야 한다.

### Projection Stale

Projection staleness는 state와 별도로 report된다. Connector가 canonical state를 직접 read할 수 있다면 계속 진행할 수 있지만, Markdown projection에 의존하는 action은 먼저 refresh 또는 reconcile해야 한다.

### Capability Insufficient

Connector는 product name이 아니라 missing capability를 말해야 한다. 예:

```text
The connected profile does not provide pre-tool guard. This work needs sidecar guard, another profile, or a smaller approved Change Unit.
```

## Reference Surface Contract

MVP는 하나의 reference surface를 target한다. Reference surface는 broad ecosystem support가 아니라 kernel을 demonstrate해야 한다.

Minimum reference expectation:

- public tool과 resource를 위한 `T2 MCP` available
- product write 전 cooperative `prepare_write`
- run 후 detective changed-path와 artifact validation
- evidence manifest에 충분한 run summary와 artifact capture
- manual verification bundle 또는 fresh evaluator instruction
- Manual QA note artifact support
- generated file을 위한 connector manifest
- common state와 fallback path를 cover하는 conformance smoke

Reference surface behavior detail과 product-specific setup은 concrete surface를 name할 때만 Appendix B에 둔다.

## Connector Conformance Overview

Connector conformance는 profile이 declared capability tier에서 common contract를 uphold할 수 있음을 prove해야 한다.

Overview scenario:

- active Task 유무에 따른 status
- advisor/direct/work로 intake classification
- shared design과 decision을 포함한 work shaping
- Change Unit scope와 vertical/horizontal exception handling
- `prepare_write` allowed 및 blocked path
- sensitive approval request, granted, denied, expired path
- artifact와 evidence update가 있는 `record_run`
- direct result projection
- verification launch 또는 manual verification bundle
- same-session verification guard
- Manual QA required, passed, failed, waived
- acceptance required 및 recorded
- stale projection과 reconcile flow
- generated file drift detection
- required tier가 missing일 때 capability fallback
- MCP unavailable product-write hold

정확한 fixture format과 operational command는 operations/conformance doc이 담당한다.
