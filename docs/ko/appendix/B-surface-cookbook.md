# Appendix B: Surface Cookbook

## 문서 역할

이 appendix는 surface-specific connector note, generated file detail, profile example을 담당한다. Common integration contract는 `09-agent-integration.md`가 담당한다.

Concrete surface에 의존하는 local difference에만 이 cookbook을 사용한다. Kernel state rule, MCP schema, generic policy contract를 여기서 반복하지 않는다.

## Cookbook Scope

각 surface recipe는 다음을 설명해야 한다.

- surface에 plausible한 target profile
- generated file 또는 instruction
- MCP configuration hint
- capture, guard, isolation option
- common fallback
- conformance risk

Connector는 여전히 capability profile을 declare해야 한다. Surface name은 guarantee level을 imply하지 않는다.

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

Generated file에는 다음이 포함될 수 있다.

- `AGENTS.md` 또는 그 안의 managed harness section
- 지원되는 경우 local skill 또는 command instruction
- MCP config snippet
- connector manifest entry

Codex-specific connector work는 `AGENTS.md`를 짧게 유지해야 한다. 절차 workflow는 skill, command, MCP resource에 두는 편을 선호한다. Pre-tool blocking이 unavailable이면 cooperative `prepare_write` discipline과 changed-path validator를 사용하고, risk가 warrant할 경우 sidecar 또는 manual verification bundle을 사용한다.

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

Hook mapping candidate:

| Hook point | Harness use |
|---|---|
| `SessionStart` | active Task와 status card inject |
| `UserPromptSubmit` | intake와 shaping 안내 |
| `PreToolUse` | edit/write/bash/network/secret access를 scope와 approval에 대해 check |
| `PostToolUse` | changed file, command output, log artifact candidate register |
| `Stop` | run summary draft 및 verify/QA need 표시 |
| `PreCompact` | Task summary와 artifact ref preserve |

Evaluator profile은 기본적으로 read-only여야 한다. Connector conformance가 해당 hook 또는 boundary가 active임을 prove한 뒤에만 profile이 preventive 또는 isolated guarantee를 claim할 수 있다.

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

Gemini connector는 extension context를 작게 유지해야 한다. Active Task card와 Change Unit scope를 push한 뒤 agent가 MCP resource를 통해 longer standard, domain language, module map, interface contract를 pull하게 한다.

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

Copilot connector는 status card display, MCP tool invocation, approval card display, Manual QA card display, acceptance prompt를 우선해야 한다. Terminal/task execution에는 output을 capture하고 active Run에 associate할 수 있는 wrapper를 선호한다.

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

Cursor connector는 project rule을 짧게 유지하고 skill/playbook과 MCP로 procedural depth를 제공해야 한다. Generated project rule은 connector manifest로 cover해야 하며, local edit는 조용히 overwrite되지 않고 reconcile candidate가 되어야 한다.

## Generated File Details

### Always-On Rule File

`AGENTS.md`, `CLAUDE.md`, Gemini instruction, Copilot custom instruction, Cursor rule 같은 surface rule file에는 이 shape를 사용한다. Specific surface에 필요한 line만 유지한다.

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
Before editing product files, call `harness.prepare_write`. If `prepare_write` is blocked, do not edit product files. If MCP is unavailable, hold product writes and report the guarantee limitation.

## Workflow

### Minimal Happy Path
1. 상태 확인 또는 intake.
2. `advisor`, `direct`, `work`로 분류.
3. 범위와 Change Unit 확인.
4. Before editing product files, call `harness.prepare_write`.
5. 변경 후 runs, changed paths, commands, artifacts, evidence 기록.
6. 필요한 경우 verify, Manual QA, acceptance 요청.
7. Close.

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
- If `prepare_write` is blocked, do not edit product files.
- If MCP is unavailable, hold product writes and report that the surface cannot provide an authoritative write decision.
- Respect allowed paths, tools, commands, network, and secret scope.
- Stop when approval or scope confirmation is required.
- Request approval through `harness.request_user_decision`.

### 5. During Implementation
- Prefer TDD when suitable.
- Keep feedback loops short.
- Avoid changes outside the active Change Unit.

### 6. After Changing
- Call `harness.record_run` with changed paths, commands, logs, diff refs, artifacts, TDD trace, evidence mapping, and design updates.
- Record evidence after changes; changed paths, commands, artifacts, evidence를 chat에만 남기면 안 된다.

### 7. Finish
- For work verification, call `harness.launch_verify` or record a fresh evaluator result through `harness.record_eval`.
- Work cannot self-certify detached verification.
- For Manual QA, call `harness.record_manual_qa`.
- Record user decisions through `harness.record_user_decision`.
- Call `harness.close_task` after required verification, Manual QA, evidence, and acceptance are resolved.
````

### MCP Config Snippet

각 surface에는 자체 config format이 있다. Connector manifest는 generated path와 managed hash를 기록해야 한다. Local stdio가 default MVP transport다. Profile에 따라 local HTTP를 허용할 수 있다.

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

각 connector recipe는 declared capability tier에 맞는 operations-owned fixture로 test되어야 한다. Surface version 또는 host profile이 바뀌면 previous guarantee level을 reuse하기 전에 conformance를 rerun한다.
