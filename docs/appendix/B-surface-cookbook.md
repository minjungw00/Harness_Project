# Appendix B: Surface Cookbook

## Document Role

This appendix owns surface-specific connector notes, generated file details, and profile examples. The common integration contract is owned by `09-agent-integration.md`.

Use this cookbook only for local differences that depend on a concrete surface. Do not repeat kernel state rules, MCP schemas, or generic policy contracts here.

## Cookbook Scope

Each surface recipe should describe:

- target profiles that are plausible for the surface
- generated files or instructions
- MCP configuration hints
- capture, guard, and isolation options
- common fallbacks
- conformance risks

The connector must still declare a capability profile. A surface name does not imply a guarantee level.

## Codex Notes

```yaml
surface_kind: codex
target_profiles:
  - local_cli
  - ide_chat
  - custom_agent
primary_strengths:
  - repository instruction files
  - code editing workflow
  - MCP-capable profiles can call harness tools directly
common_fallbacks:
  - cooperative prepare_write discipline
  - sidecar changed-file watcher
  - changed_paths validator
  - manual verification bundle
profile_risks:
  - pre-tool guard strength depends on host environment
  - artifact capture may need wrapper or explicit record_run discipline
```

Generated files may include:

- `AGENTS.md` or a managed harness section inside it
- local skill or command instructions when supported
- MCP config snippet
- connector manifest entry

Codex-specific connector work should keep `AGENTS.md` short and put the procedural workflow in a skill, command, or MCP resource. Where pre-tool blocking is unavailable, rely on `prepare_write`, detective changed-path validation, and a sidecar if risk warrants it.

## Claude Code Notes

```yaml
surface_kind: claude_code
target_profiles:
  - local_cli
  - ide_chat
  - custom_agent
primary_strengths:
  - CLAUDE.md and skill-style procedures
  - hook candidates for guard and capture
  - fresh evaluator profile candidates
common_fallbacks:
  - read-only evaluator profile
  - fresh worktree evaluator
  - stop-hook report draft
profile_risks:
  - hook behavior is version and configuration dependent
  - read-only verification profile must be tested by conformance
```

Hook mapping candidates:

| Hook point | Harness use |
|---|---|
| `SessionStart` | inject active Task and status card |
| `UserPromptSubmit` | guide intake and shaping |
| `PreToolUse` | check edit/write/bash/network/secret access against scope and approval |
| `PostToolUse` | register changed files, command output, and log artifact candidates |
| `Stop` | draft run summary and show verify/QA needs |
| `PreCompact` | preserve Task summary and artifact refs |

Evaluator profiles should be read-only by default. A profile may claim preventive or isolated guarantees only after the connector conformance proves those hooks or boundaries are active.

## Gemini Notes

```yaml
surface_kind: gemini
target_profiles:
  - local_cli
  - extension
  - ide_chat
  - custom_agent
primary_strengths:
  - extension or prompt package
  - MCP-capable profiles
  - sidecar-friendly local workflows
common_fallbacks:
  - CLI wrapper
  - sidecar-controlled run
  - Manual QA note artifact
profile_risks:
  - extension context can become too large
  - capture and guard behavior varies by host
```

Gemini connectors should keep extension context small. Push the active Task card and Change Unit scope, then let the agent pull longer standards, domain language, module maps, and interface contracts through MCP resources.

## GitHub Copilot Notes

```yaml
surface_kind: github_copilot
target_profiles:
  - vscode_chat
  - vscode_agent
  - cloud_agent
  - custom_agent
primary_strengths:
  - workspace custom instructions
  - VS Code task and terminal integration
  - MCP-capable profiles where available
common_fallbacks:
  - VS Code task wrapper
  - sidecar adapter
  - explicit approval card
profile_risks:
  - cloud and IDE profiles may differ materially
  - write guard and artifact capture need profile-specific verification
```

Copilot connectors should prioritize status card display, MCP tool invocation, approval card display, Manual QA card display, and acceptance prompts. For terminal/task execution, prefer wrappers that can capture output and associate it with the active Run.

## Cursor Notes

```yaml
surface_kind: cursor
target_profiles:
  - ide_agent
  - local_cli
  - custom_agent
primary_strengths:
  - project rules
  - MCP-capable profiles
  - IDE agent workflow with sidecar support
common_fallbacks:
  - sidecar changed-file detection
  - generated file drift detection
  - manual verification bundle
profile_risks:
  - project rules can become too verbose
  - guard behavior depends on IDE profile and permissions
```

Cursor connectors should keep project rules short and use the skill/playbook plus MCP for procedural depth. Generated project rules should be covered by the connector manifest so local edits become reconcile candidates instead of being overwritten silently.

## Generated File Details

### Always-On Rule File

Use this shape for surface rule files such as `AGENTS.md`, `CLAUDE.md`, Gemini instructions, Copilot custom instructions, or Cursor rules. Keep only the lines that the specific surface needs.

````md
# Harness Rules

## Repository Summary
- purpose:
- main execution path:
- modules to treat carefully:

## Harness Rule
Use Harness for product code changes, verification, approval, Manual QA, acceptance, resume, and close decisions.

## Working Rules
- Read current Harness status before changing product files.
- Small low-risk changes may be `direct`.
- Feature, structural, risky, or multi-file changes are `work`.
- Work starts with enough shared design to define scope and acceptance criteria.
- A product write requires `harness.prepare_write`.
- Sensitive categories require approval before proceeding.
- Stay inside the active Change Unit.
- Record runs, commands, changed files, artifacts, and evidence.
- Work cannot self-certify detached verification.
- Required Manual QA and acceptance are separate close checks.
- Prefer current Harness state and evidence over chat memory.

## Default Checks
- lint:
- test:
- build:
````

### Harness Skill Or Command Template

````md
---
name: harness
description: Use this when the user asks to modify code, verify work, resume a task, request approval, perform QA, close a task, inspect project work state, or record a development decision.
---

# Harness Skill

## Purpose
Use Harness to keep AI-assisted development visible, bounded, evidenced, verifiable, and aligned with product design.

## Core Rule
Before changing product files, call the Harness MCP server.

## Workflow

### 1. Status Or Intake
- If the user asks for status, call `harness.status`.
- If the user asks for a new task, call `harness.intake`.
- If the user asks to resume, call `harness.status` and `harness.next`.

### 2. Classify
- `advisor`: explanation, comparison, review, or decision support.
- `direct`: small, low-risk, clear change.
- `work`: feature, structural change, non-local fix, refactor, or high-risk change.

### 3. Shape Work
- Ask one blocking question at a time when requirements are ambiguous.
- Record decisions, assumptions, rejected options, scope, and acceptance criteria.
- Check domain language and module/interface impact.
- Propose Change Units, preferring vertical slices.

### 4. Before Writing
- Call `harness.prepare_write`.
- Respect allowed paths, tools, commands, network, and secret scope.
- Stop when approval or scope confirmation is required.
- Request approval through `harness.request_user_decision`.

### 5. During Implementation
- Prefer TDD when suitable.
- Keep feedback loops short.
- Avoid changes outside the active Change Unit.

### 6. After Changing
- Call `harness.record_run` with changed files, commands, logs, diff refs, TDD trace, evidence mapping, and design updates.

### 7. Finish
- For work verification, call `harness.launch_verify` or record a fresh evaluator result through `harness.record_eval`.
- For Manual QA, call `harness.record_manual_qa`.
- Record user decisions through `harness.record_user_decision`.
- Call `harness.close_task` after required verification, Manual QA, evidence, and acceptance are resolved.
````

### MCP Config Snippet

Each surface has its own config format. The connector manifest should record the generated path and managed hash. Local stdio is the default MVP transport; local HTTP may be allowed by profile.

```yaml
mcp_server:
  name: harness
  transport: stdio
  command: harness
  args:
    - serve
    - mcp
  project_id: PRJ-0001
```

## Profile Examples

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

## Surface Conformance Notes

Each connector recipe should be tested against the operations-owned fixtures for its declared capability tier. When a surface version or host profile changes, rerun conformance before reusing the previous guarantee level.
