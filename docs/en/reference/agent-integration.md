# Agent Integration Reference

## What this document helps you do

Use this reference to connect an agent surface to Harness without overstating what that surface can enforce.

It owns the common connector contract: capability tiers, capability profiles, generated manifest expectations, context push/pull principles, fallback semantics, Role Lens behavior, the reference surface contract, and connector conformance overview.

For the user-facing agent procedure, read [Agent Session Flow](../use/agent-session-flow.md). For surface-specific setup notes, read [Surface Cookbook](surface-cookbook.md).

## Read this when

- You are implementing or reviewing a connector for an agent surface.
- You need to declare or audit a surface capability profile.
- You need to decide how a connected profile should display guarantee level, guard, freeze, fallback, or MCP availability.
- You are writing connector conformance coverage.
- You need to know which parts belong in a surface recipe instead of the common contract.

## Integration in plain language

An agent surface is where the user talks to an agent. Harness is the local authority layer that keeps task state, write authority, evidence, verification, Manual QA, acceptance, projections, and reconcile behavior outside the chat transcript.

A connector should give the agent small current context, route state changes through Harness MCP tools, capture what happened when the surface can do so, and name the actual guarantee level for the connected profile. A surface name is never enough to claim a capability.

The common structure is:

```text
user conversation surface
  -> short always-on rules/context
  -> harness skill, command, or playbook
  -> harness MCP server
  -> harness Core
  -> adapter, hook, sidecar, validator, or isolation layer
```

Always-on rules should stay short. They should say when to use Harness, where to read status or the Journey Card, that product writes require `prepare_write`, that product judgment routes through Decision Packets, that the current guarantee level must be shown honestly, and that product writes hold when authoritative MCP is unavailable. The session procedure itself belongs in [Agent Session Flow](../use/agent-session-flow.md).

## What belongs in Use docs vs this Reference doc

| Area | Owner |
|---|---|
| What the agent shows, asks, and says during a user session | [Agent Session Flow](../use/agent-session-flow.md) |
| User-facing explanation of scope, evidence, verification, QA, residual risk, and close | [User Guide](../use/user-guide.md) |
| Common connector contract, capability profiles, manifests, context model, fallback semantics, Role Lens, reference surface, conformance overview | This reference |
| Concrete surface recipes for Codex, Claude Code, Gemini, GitHub Copilot, and Cursor | [Surface Cookbook](surface-cookbook.md) |
| Public MCP request/response schemas | [MCP API And Schemas](mcp-api-and-schemas.md) |
| Kernel state transitions and write/close rules | [Kernel Reference](kernel.md) |
| Runtime guarantee level definitions | [Runtime Architecture Reference](runtime-architecture.md#guarantee-levels) |

## Capability Tiers

| Tier | Meaning | Typical capability |
|---|---|---|
| `T0 Context` | Surface can read Harness principles. | rules/context file |
| `T1 Skill` | Surface can follow a Harness procedure. | skill, command, prompt, playbook |
| `T2 MCP` | Surface can call Harness tools and resources. | MCP server connection |
| `T3 Capture` | Surface can return diffs, logs, and run output reliably. | structured output, wrapper, adapter |
| `T4 Guard` | Surface can block or interrupt out-of-scope files, commands, network, or secrets before execution. | hook, permission system, policy engine, sidecar |
| `T5 Isolation` | Surface can run verification or risky work in a separate boundary. | worktree, sandbox, fresh process, isolated runner |
| `T6 QA Capture` | Surface can structure browser, screenshot, walkthrough, workflow-recording, or Manual QA artifacts. | browser runner, screenshot capture, console/network capture, accessibility snapshot, QA note capture |

Normal interactive Harness use is most natural at `T2` or higher. Reliable detached verification usually needs `T3` capture plus a real independence boundary. High-risk work should use `T4` guard or `T5` isolation when available. `T6` improves UI/UX evidence, but it does not replace Manual QA judgment and is not required for MVP when a human QA note can be recorded.

`T6 QA Capture` profiles must name supported capture types and fallback behavior. Candidate capture types include screenshot, console log, network trace, accessibility snapshot, and workflow recording. Captured files must follow redaction and secret/PII handling before durable storage and should be registered as artifact refs attached to the Manual QA record or feedback loop execution.

## Capability Profiles

Harness connectors must use a capability profile instead of assuming behavior from a product or surface name.

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

Target profile values may include:

- `local_cli`
- `ide_chat`
- `ide_agent`
- `cloud_agent`
- `extension`
- `custom_agent`
- `manual_bundle`

Capability profiles must be refreshed when version, MCP config, hooks, permissions, workspace policy, generated files, conformance result, capture method, QA capture method, browser test environment, redaction policy, or artifact retention behavior changes.

## Capability Profile Examples

These are examples of profile shapes. A tier or example does not automatically upgrade a concrete surface's guarantee level. A concrete connector must prove the capability for its actual host/profile before claiming it.

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

Integration uses the guarantee levels defined in [Runtime Architecture Reference](runtime-architecture.md#guarantee-levels) and applies them to connected surface profiles, current enforcement paths, and fallback choices.

This reference owns how connector profiles report and display those levels. It must not infer a stronger level from a surface name, and it must not treat guarantee level as approval, verification, QA, acceptance, or a kernel gate.

| Level | Display responsibility |
|---|---|
| `cooperative` | Show that the surface is expected to follow Harness decisions, but Harness may not physically block a violation before it happens. |
| `detective` | Show that Harness can observe changed paths, logs, artifacts, or projection drift after action and mark state stale, blocked, partial, or failed. |
| `preventive` | Show the hook, wrapper, permission layer, policy engine, or sidecar path that can block a covered violation before execution. |
| `isolated` | Show the separate worktree, sandbox, process, evaluator bundle, or equivalent boundary used for risky work or verification. |

Guard, freeze, and careful-mode labels are safety-control labels over the actual profile.

| User wording | Actual boundary |
|---|---|
| Freeze | A visible hold or narrowed posture around current work. Persistent owner-record changes still route through the normal Core path. |
| Guard | Cooperative, detective, preventive, or isolated protection according to the proven profile and current enforcement path. |
| Careful mode | Stricter `prepare_write`, scope, evidence, status refresh, and user-question posture. It is not a new authority tier. |

## Generated Manifest Expectations

Connectors may generate rules, skills, MCP config snippets, prompts, or local adapter files. Every generated or managed path must be recorded in a connector manifest.

The manifest must:

- name generated paths
- record managed block hashes
- record the capability profile used when generated
- record the target surface profile
- record creation and update times
- detect drift before overwriting human edits
- route drift to reconcile when needed

The manifest concept is common. Surface-specific generated filenames belong in [Surface Cookbook](surface-cookbook.md).

## Context Push/Pull Principles

Implementation agents should receive small current context and pull larger references only when needed.

Usually push:

- Journey Card or status card
- active Decision Packet summary
- Autonomy Boundary summary
- Write Authority Summary when writes are near
- active scoped Change Unit
- acceptance criteria snapshot
- approval status
- latest evidence manifest and run refs
- residual-risk summary when close or acceptance is near

Usually pull:

- older PRDs
- old designs
- closed issues
- long logs
- module maps
- interface contracts
- domain language
- coding standards
- TDD guidance

Evaluators should receive a tighter verification bundle: acceptance criteria, changed files, approval scope, relevant Decision Packets, residual risk summary, Autonomy Boundary, deferred decisions, codebase stewardship refs, evidence manifest, required TDD trace, Manual QA requirement, artifact refs, and forbidden patterns.

A later Context Index may help retrieve relevant projections, artifact refs, repo files, docs, or notes. It is a read-only context provider, not a connector authority path.

## Fallback Semantics

Fallbacks are described by guarantee level and risk, not by surface name.

| Fallback | Use when | Boundary |
|---|---|---|
| Cooperative | The surface can follow instructions but cannot enforce them. | Tell the agent to use `prepare_write`, hold on blocked decisions, and record runs. Product writes pause if authoritative MCP is unavailable or write scope cannot be checked. |
| Detective | Harness can observe changed files, logs, projection drift, or artifact gaps after action. | Validators may mark state stale, partial, blocked, or failed and require repair, reconcile, or fresh verification. |
| Preventive | A hook, permission layer, wrapper, policy engine, or sidecar can block before execution. | Claim only the operations that the blocking path actually covers. |
| Isolated | Risk requires separation. | Launch work or verification in a separate worktree, sandbox, process, or manual evaluator bundle. |

If MCP is unavailable, the connector must not claim authoritative state updates. `MCP_SERVER_UNAVAILABLE` and `SURFACE_MCP_UNAVAILABLE` are diagnostic conditions, not additional public `ErrorCode` values. `MCP_UNAVAILABLE` remains the stable public availability code.

`MCP_SERVER_UNAVAILABLE` means the tool call cannot reach Core, so no authoritative Core response is possible. `SURFACE_MCP_UNAVAILABLE` means Core or an operator can observe that the connected surface lacks usable MCP, has stale MCP configuration, or cannot call required tools. Product/runtime/code writes hold until MCP is reconnected or diagnosed, unless the work is an explicit pre-MVP documentation-authoring batch under `DOCS_AUTHORING_OVERRIDE` with an exact path allowlist. That override is a documentation-maintainer override only; it is not Core authorization, Write Authorization, evidence, verification, QA, acceptance, residual-risk acceptance, close, or a canonical state transition.

If MCP works but pre-tool guard is weak, low-risk direct work may proceed with cooperative `prepare_write` and detective changed-path validation. Medium/high-risk work should require stricter validation, sidecar guard, explicit approval, detached verification, or isolation.

Projection staleness is reported separately from state. A connector may continue from canonical state if it can read state directly, but actions that depend on Markdown projection should refresh or reconcile first.

## Role Lens Behavior

Role Lens is a non-authoritative skill or playbook surface that helps the user steer the agent from a familiar review posture. Initial lenses are:

- `product-review`
- `eng-review`
- `design-review`
- `security-review`
- `qa-review`
- `release-handoff`

A connector may expose these as slash commands, buttons, prompt snippets, or recommended playbooks. The lens name selects a review posture; it does not select an authority path.

Role Lens output may produce:

- a `DecisionPacketCandidate` or a route to an existing Decision Packet
- a validator finding candidate or suggested `ValidatorResult` route for an actual validator/check to emit
- an evidence requirement
- a Manual QA requirement
- a residual-risk candidate
- release handoff report input
- a recommended next playbook

Role Lens output must not mutate canonical state by itself, authorize writes, grant approval, satisfy a Decision Packet, waive QA or verification, accept residual risk, accept the result, close a Task, or upgrade assurance. When a lens identifies work that needs a state change, the surface routes through the normal MCP tool and Core path.

Two-stage review display should keep the stages visibly separate:

| Stage | Question |
|---|---|
| Spec Compliance Review | Is the requested work complete under current Harness authority: acceptance criteria, Change Unit completion conditions, scope/write authority compatibility, Decision Packet compatibility, evidence coverage, and residual-risk visibility? |
| Code Quality / Stewardship Review | Is the implementation maintainable: domain language, module/interface boundary, vertical slice shape, feedback loop or TDD trace, codebase stewardship, context hygiene, and follow-up risk? |

Same-session review may be useful self-checking, but it is not detached verification and must not display `assurance_level=detached_verified`.

## Reference Surface Contract

The MVP targets one reference surface. The reference surface should demonstrate the kernel rather than broad ecosystem support.

Minimum reference expectations:

- `T2 MCP` available for public tools and resources
- cooperative `prepare_write` before product writes
- detective changed-path and artifact validation after runs
- run summary and artifact capture sufficient for evidence manifests
- manual verification bundle or fresh evaluator instructions
- Manual QA note artifact support
- connector manifest for generated files
- conformance smoke covering common state and fallback paths

Reference surface behavior details and surface-specific setup belong in [Surface Cookbook](surface-cookbook.md) only when they name a concrete surface.

## Connector Conformance Overview

Connector conformance should prove that a profile can uphold the common contract at its declared capability tier.

Overview scenarios:

- status with and without an active Task
- current Journey Card shown before significant work resumes when required by the Use procedure
- intake classification into `advisor`, `direct`, or `work`
- work shaping with shared design and decisions
- Change Unit scope and vertical/horizontal exception handling
- one blocking question with recommendation and uncertainty when available
- Decision Packet shown instead of broad approval for blocking product judgment
- Autonomy Boundary breach stops or routes to Decision Packet
- AFK work remains covered by active Change Unit scope, Autonomy Boundary latitude, any granted sensitive approval that applies, and compatible `prepare_write` / Write Authorization before actual product writes
- `prepare_write` allowed and blocked paths
- Write Authorization created for allowed writes and exposed through Write Authority Summary
- write-capable `record_run` consumes a compatible Write Authorization
- sensitive approval request, granted, denied, and expired paths
- `record_run` with artifacts and evidence update
- direct result projection
- verification launch or manual verification bundle
- same-session verification guard
- Manual QA required, passed, failed, and waived
- QA waiver with product/user risk routes through Decision Packet
- acceptance required and recorded
- close-relevant residual risk visible before acceptance or successful close
- risk-accepted close additionally requires accepted Residual Risk refs
- stale projection and reconcile flow
- generated file drift detection
- capability fallback when a required tier is missing
- MCP unavailable product-write hold

Exact fixture format and operational commands are owned by the operations and conformance docs.
