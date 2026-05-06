# Agent Integration

## Document Role

This document owns the common integration contract for connecting an agent surface to the harness. It defines capability tiers, capability profiles, generated manifest expectations, context push/pull principles, fallback semantics, the reference surface contract, and connector conformance overview.

The main body is product-name-neutral. Surface-specific recipes live in [Appendix B](appendix/B-surface-cookbook.md).

This document does not define kernel state transitions, MCP request/response schemas, SQLite DDL, a capability gate, operational fixture details, or surface-specific cookbooks.

## Integration Goal

The integration goal is that a user can talk naturally with an agent while the harness supplies bounded work, state recording, evidence, verification, Manual QA, acceptance, projection, and reconcile flow behind the scenes.

An integrated surface should help the agent:

- start with status or intake
- classify advisor, direct, or work mode
- shape work into scoped Change Units
- check design-quality policies when they apply
- call MCP tools for state changes
- respect `prepare_write` before product writes
- record runs, artifacts, evidence, decisions, QA, and acceptance
- launch or package detached verification
- refresh or reconcile projections

## Common Integration Structure

```text
user conversation surface
  -> short always-on rules/context
  -> harness skill, command, or playbook
  -> harness MCP server
  -> harness Core
  -> adapter, hook, sidecar, validator, or isolation layer
```

### Always-On Rules

Always-on rules should be short. They should tell the agent when to use the harness, where to read status, and that product writes require `prepare_write`.

They should not contain full state transition tables, MCP schemas, full templates, long design playbooks, or all historical project context.

### Skill Or Playbook Layer

The skill/playbook layer teaches procedure:

- when to call status, intake, and next
- how to classify advisor/direct/work
- how to ask shaping questions
- how to form a Change Unit
- how to request approval
- how to record TDD trace, evidence, Manual QA, and acceptance
- why work verification must be detached
- how to handle stale projection and reconcile

Core and validators enforce policy. The skill is guidance, not authority.

### MCP Layer

MCP is the preferred state boundary. Public tool names and schemas are owned by the MCP API document. Integration docs may reference tool intent, but connectors must use the schemas from `05-mcp-api-and-schemas.md`.

### Adapter, Hook, Sidecar, Validator, Isolation

Adapters and sidecars translate surface behavior into observable facts or stronger enforcement:

- artifact capture
- command output capture
- changed-path detection
- generated file drift detection
- projection freshness detection
- approval and scope guard support
- same-session verification guard support
- evaluator read-only or fresh-context support
- Manual QA capture support

These layers can improve guarantee level, but they do not create a kernel capability gate.

## Capability Tiers

| Tier | Meaning | Typical capability |
|---|---|---|
| `T0 Context` | Surface can read harness principles | rules/context file |
| `T1 Skill` | Surface can follow a harness procedure | skill, command, prompt, playbook |
| `T2 MCP` | Surface can call harness tools and resources | MCP server connection |
| `T3 Capture` | Surface can return diffs, logs, and run output reliably | structured output, wrapper, adapter |
| `T4 Guard` | Surface can block or interrupt out-of-scope files, commands, network, or secrets before execution | hook, permission system, policy engine, sidecar |
| `T5 Isolation` | Surface can run verification or risky work in a separate boundary | worktree, sandbox, fresh process, isolated runner |
| `T6 QA Capture` | Surface can structure browser, screenshot, walkthrough, or Manual QA artifacts | browser runner, screenshot capture, QA note capture |

Normal interactive harness use is most natural at `T2` or higher. Reliable detached verification usually needs `T3` capture plus a real independence boundary. High-risk work should use `T4` guard or `T5` isolation when available. `T6` improves UI/UX evidence but is not required for MVP when a human QA note can be recorded.

## Capability Profile

Harness connectors must use a capability profile rather than assuming behavior from a product or surface name.

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

Target profile values may include:

- `local_cli`
- `ide_chat`
- `ide_agent`
- `cloud_agent`
- `extension`
- `custom_agent`
- `manual_bundle`

Capability profiles must be refreshed when version, MCP config, hooks, permissions, workspace policy, generated files, conformance result, capture method, or QA capture method changes.

## Guarantee Levels

Integration must report enforcement strength honestly:

| Level | Integration meaning |
|---|---|
| `cooperative` | The surface is expected to follow harness instructions and MCP results. |
| `detective` | The harness can observe violations after the fact and mark state blocked, stale, partial, or failed. |
| `preventive` | The connector or runtime can block a violating action before execution. |
| `isolated` | Risky work is separated by a worktree, sandbox, process boundary, or equivalent. |

Guarantee level is risk context and display. It is not approval, verification, acceptance, or a kernel gate.

## Generated Manifest Concept

Connectors may generate rules, skills, MCP config snippets, prompts, or local adapter files. Every generated or managed path must be recorded in a connector manifest.

Manifest responsibilities:

- name generated paths
- record managed block hashes
- record capability profile used when generated
- record surface target profile
- record creation and update times
- detect drift before overwriting human edits
- route drift to reconcile when needed

The manifest concept is common. Surface-specific generated filenames belong in Appendix B.

## Push And Pull Context

Implementation agents should receive small current context and pull larger references only when needed.

Usually push:

- active Task status
- next action
- active Change Unit scope
- acceptance criteria snapshot
- allowed paths and tools
- approval status when relevant
- latest evidence manifest and run summary refs
- relevant policy warnings

Usually pull:

- coding standards
- domain language
- module map
- interface contracts
- TDD guidance
- architecture playbooks
- old PRDs, old designs, closed issues
- raw logs and large diffs

Evaluators should receive a tighter verification bundle that includes acceptance criteria, changed files, approval scope, relevant domain/module/interface records, evidence manifest, TDD trace if required, Manual QA requirement, artifact refs, and forbidden patterns.

This context model supports the Context Hygiene policy: current state and evidence are preferred over stale chat or old docs.

## Fallback Semantics

Fallbacks are described by guarantee level and risk, not by surface name.

### Cooperative Fallback

Use when the surface can follow instructions but cannot enforce them. The connector tells the agent to call `prepare_write`, hold on blocked decisions, and record runs. Product writes must be paused if MCP is unavailable or the write scope cannot be checked.

### Detective Fallback

Use when the harness can observe changed files, logs, projection drift, or artifact gaps after the action. Validators may mark state stale, partial, blocked, or failed and require repair, reconcile, or fresh verification.

### Preventive Fallback

Use when a hook, permission layer, wrapper, policy engine, or sidecar can block a violating edit, command, network call, or secret access before it happens.

### Isolated Fallback

Use when risk requires separation. The connector launches work or verification in a separate worktree, sandbox, process, or manual evaluator bundle. This is the preferred fallback for detached verification when same-session review would not qualify.

### MCP Unavailable

If MCP is unavailable, the connector must not claim authoritative state updates. For product writes, the safe behavior is to hold the write and direct the user/operator to reconnect or diagnose MCP. Stronger profiles may also enforce a preventive block.

### Weak Guard

If MCP works but pre-tool guard is weak, low-risk direct work may proceed with cooperative `prepare_write` and detective changed-path validation. Medium/high-risk work should require stricter validation, sidecar guard, explicit approval, detached verification, or isolation.

### Projection Stale

Projection staleness is reported separately from state. A connector may continue from canonical state if it can read state directly, but actions that depend on the Markdown projection should refresh or reconcile first.

### Capability Insufficient

The connector should name the missing capability, not the product name. Example:

```text
The connected profile does not provide pre-tool guard. This work needs sidecar guard, another profile, or a smaller approved Change Unit.
```

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

Reference surface behavior details and product-specific setup belong in Appendix B only when they name a concrete surface.

## Connector Conformance Overview

Connector conformance should prove that a profile can uphold the common contract at its declared capability tier.

Overview scenarios:

- status with and without an active Task
- intake classification into advisor/direct/work
- work shaping with shared design and decisions
- Change Unit scope and vertical/horizontal exception handling
- `prepare_write` allowed and blocked paths
- sensitive approval request, granted, denied, and expired paths
- `record_run` with artifacts and evidence update
- direct result projection
- verification launch or manual verification bundle
- same-session verification guard
- Manual QA required, passed, failed, and waived
- acceptance required and recorded
- stale projection and reconcile flow
- generated file drift detection
- capability fallback when a required tier is missing
- MCP unavailable product-write hold

Exact fixture format and operational commands are owned by operations and conformance docs.
