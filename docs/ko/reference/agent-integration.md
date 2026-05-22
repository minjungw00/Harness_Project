# Agent 통합 참조

## 이 문서로 할 수 있는 일

이 참조는 agent surface를 Harness에 연결할 때, 그 surface가 실제로 보장할 수 있는 수준을 과장하지 않도록 돕습니다.

이 문서는 공통 connector contract를 담당합니다. Capability tier, capability profile, generated manifest expectation, context push/pull 원칙, fallback semantic, Role Lens behavior, reference surface contract, connector conformance 개요를 정의합니다.

사용자에게 보이는 agent 절차는 [에이전트 세션 흐름](../use/agent-session-flow.md)을 봅니다. surface별 설정 메모는 [Surface Cookbook](surface-cookbook.md)을 봅니다.

## 이런 때 읽기

- agent surface용 connector를 구현하거나 검토할 때.
- surface capability profile을 선언하거나 점검할 때.
- 연결된 profile이 guarantee level, guard, freeze, fallback, MCP availability를 어떻게 표시해야 하는지 정할 때.
- connector conformance coverage를 작성할 때.
- 공통 contract와 surface recipe의 경계를 확인해야 할 때.

## 통합을 쉬운 말로 설명하면

Agent surface는 사용자가 agent와 대화하는 접점입니다. Harness는 task state, write authority, evidence, verification, Manual QA, acceptance, projection, reconcile 동작을 chat transcript 밖에 두는 로컬 authority layer입니다.

Connector는 agent에게 작고 최신인 context를 주고, state change를 Harness MCP tool로 route하고, surface가 할 수 있으면 실제로 일어난 일을 capture하며, 연결된 profile의 실제 guarantee level을 이름 붙여야 합니다. Surface 이름만으로 capability를 claim하면 안 됩니다.

공통 구조는 다음과 같습니다.

```text
user conversation surface
  -> short always-on rules/context
  -> harness skill, command, or playbook
  -> harness MCP server
  -> harness Core
  -> adapter, hook, sidecar, validator, or isolation layer
```

Always-on rule은 짧게 둡니다. 언제 Harness를 쓰는지, status 또는 Journey Card를 어디서 읽는지, product write에는 `prepare_write`가 필요하다는 점, product judgment는 Decision Packet으로 route한다는 점, 현재 guarantee level을 정직하게 보여야 한다는 점, authoritative MCP가 unavailable이면 product write를 hold한다는 점만 알려주면 충분합니다. 세션 절차 자체는 [에이전트 세션 흐름](../use/agent-session-flow.md)이 담당합니다.

## Use 문서와 이 Reference 문서의 경계

| 영역 | 담당 문서 |
|---|---|
| 사용자 세션에서 agent가 무엇을 보여주고, 묻고, 말해야 하는지 | [에이전트 세션 흐름](../use/agent-session-flow.md) |
| scope, evidence, verification, QA, residual risk, close에 대한 사용자용 설명 | [사용자 가이드](../use/user-guide.md) |
| 공통 connector contract, capability profile, manifest, context model, fallback semantic, Role Lens, reference surface, conformance overview | 이 참조 |
| Codex, Claude Code, Gemini, GitHub Copilot, Cursor의 구체적인 surface별 recipe | [Surface Cookbook](surface-cookbook.md) |
| Public MCP request/response schema | [MCP API와 스키마](mcp-api-and-schemas.md) |
| Kernel state transition과 write/close rule | [커널 참조](kernel.md) |
| Runtime guarantee level 정의 | [런타임 아키텍처 참조](runtime-architecture.md#guarantee-levels) |

## Capability Tiers

| Tier | 의미 | 대표 capability |
|---|---|---|
| `T0 Context` | Surface가 Harness 원칙을 읽을 수 있습니다. | rules/context file |
| `T1 Skill` | Surface가 Harness 절차를 따를 수 있습니다. | skill, command, prompt, playbook |
| `T2 MCP` | Surface가 Harness tool과 resource를 call할 수 있습니다. | MCP server connection |
| `T3 Capture` | Surface가 diff, log, run output을 신뢰할 만하게 반환할 수 있습니다. | structured output, wrapper, adapter |
| `T4 Guard` | Surface가 out-of-scope file, command, network, secret을 실행 전에 block 또는 interrupt할 수 있습니다. | hook, permission system, policy engine, sidecar |
| `T5 Isolation` | Surface가 verification 또는 risky work를 별도 boundary에서 run할 수 있습니다. | worktree, sandbox, fresh process, isolated runner |
| `T6 QA Capture` | Surface가 browser, screenshot, walkthrough, workflow-recording, Manual QA artifact를 구조화할 수 있습니다. | browser runner, screenshot capture, console/network capture, accessibility snapshot, QA note capture |

일반적인 interactive Harness 사용은 `T2` 이상에서 가장 자연스럽습니다. Reliable detached verification에는 보통 `T3` capture와 실제 independence boundary가 필요합니다. High-risk work에는 가능하면 `T4` guard 또는 `T5` isolation을 사용해야 합니다. `T6`는 UI/UX evidence를 보강하지만 Manual QA judgment를 대체하지 않으며, human QA note를 기록할 수 있다면 MVP 필수 조건은 아닙니다.

`T6 QA Capture` profile은 supported capture type과 fallback behavior를 이름으로 밝혀야 합니다. Candidate capture type에는 screenshot, console log, network trace, accessibility snapshot, workflow recording이 있습니다. Captured file은 durable storage 전에 redaction과 secret/PII handling을 따라야 하며, Manual QA record 또는 feedback loop execution에 붙는 artifact ref로 register되어야 합니다.

## Capability Profiles

Harness connector는 product 또는 surface name에서 behavior를 가정하지 않고 capability profile을 사용해야 합니다.

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
  console_log_capture: false
  network_trace_capture: false
  accessibility_snapshot_capture: false
  workflow_recording_capture: false
risks:
  - no pre-tool guard
fallbacks:
  - cooperative prepare_write discipline
  - changed_paths validator
  - manual verification bundle
  - human Manual QA notes and manually supplied QA artifacts
```

Target profile 값에는 다음이 포함될 수 있습니다.

- `local_cli`
- `ide_chat`
- `ide_agent`
- `cloud_agent`
- `extension`
- `custom_agent`
- `manual_bundle`

Capability profile은 version, MCP config, hook, permission, workspace policy, generated file, conformance result, capture method, QA capture method, browser test environment, redaction policy, artifact retention behavior가 바뀌면 refresh해야 합니다.

## Capability Profile 예시

다음은 profile shape 예시입니다. Tier 또는 예시가 구체적인 surface의 guarantee level을 자동으로 올려 주지는 않습니다. 구체적인 connector는 실제 host/profile에서 capability를 입증한 뒤에만 그 capability를 claim할 수 있습니다.

### Cooperative MCP Profile

```yaml
surface_id: SURF-0001
surface_kind: generic_agent
target_profile: ide_chat
support_tier: T2
guarantee_level: cooperative
capabilities:
  project_rules: true
  skills_or_commands: true
  mcp_tools: true
  mcp_resources: true
  structured_output: false
  artifact_capture: manual
  pre_tool_guard: false
  changed_path_detection: validator
  fresh_verify: manual_bundle
  worktree_isolation: false
fallbacks:
  - cooperative prepare_write
  - changed_paths validator
  - manual verify bundle
```

### Detective Capture Profile

```yaml
surface_id: SURF-0002
surface_kind: generic_agent
target_profile: local_cli
support_tier: T3
guarantee_level: detective
capabilities:
  project_rules: true
  skills_or_commands: true
  mcp_tools: true
  mcp_resources: true
  structured_output: true
  artifact_capture: wrapper
  pre_tool_guard: false
  changed_path_detection: sidecar
  command_output_capture: wrapper
  fresh_verify: manual_bundle
  worktree_isolation: false
fallbacks:
  - sidecar changed-file watcher
  - artifact integrity check
  - fresh evaluator instructions
```

### Guarded Local Profile

```yaml
surface_id: SURF-0003
surface_kind: generic_agent
target_profile: local_cli
support_tier: T4
guarantee_level: preventive
capabilities:
  project_rules: true
  skills_or_commands: true
  mcp_tools: true
  mcp_resources: true
  structured_output: true
  artifact_capture: wrapper
  hooks: true
  pre_tool_guard: true
  explicit_permissions: true
  changed_path_detection: sidecar
  command_output_capture: wrapper
  fresh_verify: fresh_session
  worktree_isolation: optional
fallbacks:
  - sidecar guard
  - approval card
  - fresh evaluator profile
```

### Isolated Verification Profile

```yaml
surface_id: SURF-0004
surface_kind: manual_bundle
target_profile: manual_bundle
support_tier: T5
guarantee_level: isolated
capabilities:
  mcp_tools: false
  mcp_resources: false
  structured_output: true
  artifact_capture: bundle
  pre_tool_guard: read_only_bundle
  changed_path_detection: bundle_manifest
  fresh_verify: fresh_worktree
  worktree_isolation: true
fallbacks:
  - read-only evaluator bundle
  - operator record_eval
```

## Guarantee Levels

Integration은 [런타임 아키텍처 참조](runtime-architecture.md#guarantee-levels)의 guarantee level 정의를 사용하고, 이를 connected surface profile, current enforcement path, fallback choice에 적용합니다.

이 참조는 connector profile이 그 level을 어떻게 report하고 display하는지 담당합니다. Surface 이름에서 더 강한 level을 추론하면 안 되며, guarantee level을 approval, verification, QA, acceptance, kernel gate로 취급하면 안 됩니다.

| Level | 표시 책임 |
|---|---|
| `cooperative` | Surface가 Harness 결정을 따르도록 지시받지만, Harness가 violation을 실행 전에 물리적으로 막지 못할 수 있음을 보여줍니다. |
| `detective` | Harness가 action 뒤에 changed path, log, artifact, projection drift를 observe하고 state를 stale, blocked, partial, failed로 표시할 수 있음을 보여줍니다. |
| `preventive` | Covered violation을 실행 전에 block할 수 있는 hook, wrapper, permission layer, policy engine, sidecar path를 보여줍니다. |
| `isolated` | Risky work 또는 verification에 쓰는 별도 worktree, sandbox, process, evaluator bundle 또는 equivalent boundary를 보여줍니다. |

Guard, freeze, careful-mode label은 실제 profile 위에 얹힌 safety-control label입니다.

| 사용자 wording | 실제 boundary |
|---|---|
| Freeze | 현재 work 주변의 visible hold 또는 narrowed posture입니다. Persistent owner-record change는 여전히 normal Core path를 거칩니다. |
| Guard | Proven profile과 current enforcement path에 따른 cooperative, detective, preventive, isolated protection입니다. |
| Careful mode | 더 엄격한 `prepare_write`, scope, evidence, status refresh, user-question posture입니다. 새로운 authority tier가 아닙니다. |

## Generated Manifest Expectations

Connector는 rule, skill, MCP config snippet, prompt, local adapter file을 generate할 수 있습니다. Generate되거나 managed되는 모든 path는 connector manifest에 기록해야 합니다.

Manifest는 다음을 해야 합니다.

- generated path 이름 기록
- managed block hash 기록
- generated 당시 사용한 capability profile 기록
- target surface profile 기록
- creation/update time 기록
- human edit을 overwrite하기 전에 drift detect
- 필요하면 drift를 reconcile로 route

Manifest concept은 공통입니다. Surface별 생성 파일 이름은 [Surface Cookbook](surface-cookbook.md)이 담당합니다.

## Context Push/Pull Principles

Implementation agent에게는 작고 최신인 context를 주고, 긴 reference는 필요할 때 pull하게 해야 합니다.

보통 push하는 것:

- Journey Card 또는 status card
- active Decision Packet summary
- Autonomy Boundary summary
- write가 가까울 때 Write Authority Summary
- active scoped Change Unit
- acceptance criteria snapshot
- approval status
- latest evidence manifest와 run ref
- close 또는 acceptance가 가까울 때 residual-risk summary

보통 pull하는 것:

- 오래된 PRD
- 오래된 design
- closed issue
- 긴 log
- module map
- interface contract
- domain language
- coding standard
- TDD guidance

Evaluator는 더 좁은 verification bundle을 받아야 합니다. 여기에는 acceptance criteria, changed file, approval scope, relevant Decision Packet, residual risk summary, Autonomy Boundary, deferred decision, codebase stewardship ref, evidence manifest, required TDD trace, Manual QA requirement, artifact ref, forbidden pattern이 포함됩니다.

이후 Context Index는 relevant projection, artifact ref, repo file, docs, note를 retrieve하는 데 도움을 줄 수 있습니다. 하지만 read-only context provider일 뿐 connector authority path가 아닙니다.

## Fallback Semantics

Fallback은 surface name이 아니라 guarantee level과 risk로 설명합니다.

| Fallback | 쓰는 경우 | Boundary |
|---|---|---|
| Cooperative | Surface가 instruction을 따를 수 있지만 enforce할 수 없을 때. | Agent에게 `prepare_write`를 쓰고, blocked decision에서 hold하고, run을 record하라고 지시합니다. Authoritative MCP가 unavailable이거나 write scope를 확인할 수 없으면 product write를 멈춥니다. |
| Detective | Harness가 action 뒤에 changed file, log, projection drift, artifact gap을 observe할 수 있을 때. | Validator가 state를 stale, partial, blocked, failed로 표시하고 repair, reconcile, fresh verification을 요구할 수 있습니다. |
| Preventive | Hook, permission layer, wrapper, policy engine, sidecar가 실행 전에 block할 수 있을 때. | Blocking path가 실제로 cover하는 operation에 대해서만 claim합니다. |
| Isolated | Risk가 separation을 요구할 때. | 별도 worktree, sandbox, process, manual evaluator bundle에서 work 또는 verification을 launch합니다. |

MCP가 unavailable이면 connector는 authoritative state update를 claim하면 안 됩니다. `MCP_SERVER_UNAVAILABLE`과 `SURFACE_MCP_UNAVAILABLE`은 diagnostic condition이지 추가 public `ErrorCode` 값이 아닙니다. `MCP_UNAVAILABLE`은 stable public availability code로 남습니다.

`MCP_SERVER_UNAVAILABLE`은 tool call이 Core에 닿지 못해 authoritative Core response가 없다는 뜻입니다. `SURFACE_MCP_UNAVAILABLE`은 Core 또는 operator가 connected surface에 usable MCP가 없거나 MCP configuration이 stale이거나 required tool을 call할 수 없다고 observe할 수 있다는 뜻입니다. Product/runtime/code write는 MCP가 reconnect되거나 diagnose될 때까지 hold합니다. 예외는 exact path allowlist가 있는 명시적 pre-MVP documentation-authoring batch인 `DOCS_AUTHORING_OVERRIDE`뿐입니다. 이 override는 documentation-maintainer override일 뿐이며 Core authorization, Write Authorization, evidence, verification, QA, acceptance, residual-risk acceptance, close, canonical state transition이 아닙니다.

MCP는 동작하지만 pre-tool guard가 약하면 low-risk direct work는 cooperative `prepare_write`와 detective changed-path validation으로 진행할 수 있습니다. Medium/high-risk work에는 stricter validation, sidecar guard, explicit approval, detached verification, isolation을 요구해야 합니다.

Projection staleness는 state와 별도로 report합니다. Connector가 canonical state를 직접 읽을 수 있으면 거기서 계속할 수 있지만, Markdown projection에 의존하는 action은 먼저 refresh 또는 reconcile해야 합니다.

## Role Lens Behavior

Role Lens는 사용자가 익숙한 review posture로 agent를 steer할 수 있게 하는 non-authoritative skill 또는 playbook surface입니다. Initial lenses는 다음과 같습니다.

- `product-review`
- `eng-review`
- `design-review`
- `security-review`
- `qa-review`
- `release-handoff`

Connector는 이를 slash command, button, prompt snippet, recommended playbook으로 expose할 수 있습니다. Lens name은 review posture를 고를 뿐 authority path를 고르지 않습니다.

Role Lens output은 다음을 만들 수 있습니다.

- `DecisionPacketCandidate` 또는 existing Decision Packet route
- 실제 validator/check가 emit할 validator finding candidate 또는 suggested `ValidatorResult` route
- evidence requirement
- Manual QA requirement
- residual-risk candidate
- release handoff report input
- recommended next playbook

Role Lens output은 그 자체로 canonical state를 mutate하거나, write를 authorize하거나, approval을 grant하거나, Decision Packet을 satisfy하거나, QA 또는 verification을 waive하거나, residual risk를 accept하거나, result를 accept하거나, Task를 close하거나, assurance를 upgrade하면 안 됩니다. Lens가 state change가 필요한 일을 찾아내면 surface는 normal MCP tool과 Core path로 route합니다.

Two-stage review display는 stage를 visibly separate하게 유지해야 합니다.

| Stage | 질문 |
|---|---|
| Spec Compliance Review | Requested work가 current Harness authority 안에서 complete한가: acceptance criteria, Change Unit completion condition, scope/write authority compatibility, Decision Packet compatibility, evidence coverage, residual-risk visibility? |
| Code Quality / Stewardship Review | Implementation이 maintainable한가: domain language, module/interface boundary, vertical slice shape, feedback loop 또는 TDD trace, codebase stewardship, context hygiene, follow-up risk? |

Same-session review는 useful self-checking일 수 있지만 detached verification이 아니며 `assurance_level=detached_verified`로 표시하면 안 됩니다.

## Reference Surface Contract

MVP는 하나의 reference surface를 목표로 합니다. Reference surface는 broad ecosystem support가 아니라 kernel을 증명해야 합니다.

Minimum reference expectations:

- public tool과 resource에 `T2 MCP` 사용 가능
- product write 전 cooperative `prepare_write`
- run 이후 detective changed-path와 artifact validation
- evidence manifest에 충분한 run summary와 artifact capture
- manual verification bundle 또는 fresh evaluator instruction
- Manual QA note artifact support
- 생성 파일용 connector manifest
- common state와 fallback path를 다루는 conformance smoke

Reference surface behavior detail과 surface별 설정은 concrete surface를 이름으로 부를 때만 [Surface Cookbook](surface-cookbook.md)에 둡니다.

## Connector Conformance Overview

Connector conformance는 profile이 선언한 capability tier에서 공통 contract를 지킬 수 있음을 입증해야 합니다.

Overview scenario:

- active Task가 있을 때와 없을 때의 status
- Use procedure가 요구할 때 significant work 재개 전 current Journey Card 표시
- intake를 `advisor`, `direct`, `work`로 분류
- shared design과 decision을 포함한 work shaping
- Change Unit scope와 vertical/horizontal exception handling
- 가능할 때 recommendation과 uncertainty가 있는 one blocking question
- blocking product judgment에 broad approval 대신 Decision Packet 표시
- Autonomy Boundary breach가 stop하거나 Decision Packet으로 route
- AFK work가 active Change Unit scope, Autonomy Boundary latitude, 적용되는 granted sensitive approval, 실제 product write 전 compatible `prepare_write` / Write Authorization 안에 머무름
- `prepare_write` allowed/blocked path
- allowed write에 Write Authorization 생성 및 Write Authority Summary 표시
- write-capable `record_run`이 compatible Write Authorization consume
- sensitive approval request, granted, denied, expired path
- artifact와 evidence update를 포함한 `record_run`
- direct result projection
- verification launch 또는 manual verification bundle
- same-session verification guard
- Manual QA required, passed, failed, waived
- product/user risk가 있는 QA waiver를 Decision Packet으로 route
- acceptance required와 recorded
- acceptance 또는 successful close 전 close-relevant residual risk visible
- risk-accepted close에는 accepted Residual Risk refs 추가 요구
- stale projection과 reconcile flow
- generated file drift detection
- required tier가 없을 때 capability fallback
- MCP unavailable product-write hold

정확한 fixture format과 operational command는 operations and conformance docs가 담당합니다.
