# Agent Host Troubleshooting

Use this guide when Codex, Claude Code, or a generic MCP host connection does not reach the state you expected after `volicord agent connect`, `volicord agent verify`, `volicord agent status`, project membership changes, or uninstall.

For the normal setup path, use [Agent Host Setup](agent-host-setup.md). For one user-scope connection serving multiple repositories, use [Multi-Repository Agent Setup](multi-repository-agent-setup.md).

This guide helps you identify the observed state, check likely causes without making another change when possible, take a bounded recovery action, and verify the result. Exact behavior remains with [Administrative CLI](../reference/admin-cli.md), [MCP Transport](../reference/mcp-transport.md), [Runtime Boundaries](../reference/runtime-boundaries.md), [Agent Connection Reference](../reference/agent-connection.md), and storage owners routed from [Storage](../reference/storage.md).

## Before You Change Anything

Keep the same values you used during setup:

- `VOLICORD_BIN` is the selected directory containing `volicord` and `volicord-mcp`.
- `VOLICORD_HOME` or `--runtime-home` is the selected `Volicord Runtime Home`.
- `<connection_id>`, `<project_id>`, `<repo_root>`, and `<server_name>` are the actual values from setup output.

Start with read-only or non-mutating checks:

```sh
"$VOLICORD_BIN/volicord" agent status \
  --connection-id <connection_id> \
  --runtime-home <runtime_home>

VOLICORD_HOME=<runtime_home> \
"$VOLICORD_BIN/volicord-mcp" --check --connection <connection_id>
```

`volicord agent status` reports stored Agent Connection and connected Project state. It does not prove that Codex or Claude Code loaded the MCP server. `volicord-mcp --check` validates local startup for the MCP process only.

<a id="missing-volicord-mcp"></a>
## `volicord-mcp` Is Missing, Not Executable, Or Cannot Be Resolved

Observable symptom: setup, verification, or host startup reports that `volicord-mcp` is missing, unavailable, not executable, or not found on `PATH`.

Most likely causes: the selected executable directory does not contain both `volicord` and `volicord-mcp`; the file is not executable by the selected user; project-scoped host configuration stores portable `volicord-mcp`, but the future host process does not receive a `PATH` that can resolve it.

Bounded recovery:

```sh
test -x "$VOLICORD_BIN/volicord-mcp"
"$VOLICORD_BIN/volicord-mcp" --version
```

For user, local, or generic export scope, rerun `volicord agent connect` for the same `--connection-id` with a valid absolute `--mcp-command`. For project scope, keep the generated host entry portable and fix the host launch `PATH`.

Verification:

```sh
VOLICORD_HOME=<runtime_home> \
"$VOLICORD_BIN/volicord-mcp" --check --connection <connection_id>
```

Do not delete the Runtime Home, project state, Product Repository files, or unrelated host configuration just because the executable could not be resolved.

<a id="wrong-absolute-mcp-command"></a>
## An Absolute `--mcp-command` Is Wrong

Observable symptom: the CLI rejects `--mcp-command`, or verification later reports that the configured command is missing, changed, unavailable, or cannot be launched.

Most likely causes: the path is not absolute, points at a stale build output, points at `volicord` instead of `volicord-mcp`, or no longer exists after a rebuild or move.

Bounded recovery: run `test -x /absolute/path/to/volicord-mcp` and `/absolute/path/to/volicord-mcp --help`, then rerun `volicord agent connect` for the same `--connection-id`, host, scope, and server name with the corrected absolute command.

<a id="portable-project-command-not-on-path"></a>
## A Portable Project-Scoped Command Is Not On Host `PATH`

Observable symptom: project-scoped Codex or Claude Code configuration contains `command = "volicord-mcp"` or `"command": "volicord-mcp"`, but a later host session cannot start Volicord.

Most likely causes: project-scoped configuration intentionally omits personal build paths and personal `VOLICORD_HOME`; the future host process was started from an environment that cannot resolve `volicord-mcp`.

Bounded recovery: change the host launch environment, shell startup, service configuration, or equivalent host-owned path so it can resolve `volicord-mcp`. Keep the project-scoped host file portable.

<a id="status-action_required"></a>
## `status: action_required`

Meaning: durable Agent Connection state and host configuration are present, but a host-owned trust, project approval, OAuth, reload, restart, or comparable user-controlled action remains.

Bounded recovery:

1. Read the reported action and host details.
2. Complete only that host-owned action.
3. Rerun verification.

```sh
"$VOLICORD_BIN/volicord" agent verify \
  --connection-id <connection_id> \
  --runtime-home <runtime_home>
```

Do not treat `action_required` as a Core failure or product acceptance result.

<a id="status-failed"></a>
## `status: failed`

Meaning: the requested connection or verification did not establish usable durable Agent Connection state or host configuration.

Bounded recovery: inspect stderr or JSON `warnings`, `verification`, and host detail fields. Fix the named executable, Runtime Home, Product Repository path, host target, or host gate issue, then rerun the same command. Do not delete unrelated state as a first response.

<a id="status-partial_failure"></a>
## Partial Setup Or Cleanup Result

Some older guides link to this anchor. Current Agent Connection result states are `complete`, `action_required`, and `failed`; if output reports partial cleanup details, read the listed effects and residual effects before retrying.

<a id="ambiguous-project-selection"></a>
## More Than One Connected Project Exists Without A Usable Selector

Observable symptom: an MCP workflow call is rejected because more than one connected Project is available and the request did not provide `project_id`.

Most likely cause: the Agent Connection is correctly connected to multiple Projects, but the agent sent a project-routed tool call without explicit `project_id`.

Bounded recovery: call `volicord.list_projects`, choose the intended Project, and retry the project-routed tool call with `project_id`.

<a id="host-config-remains-zero-projects"></a>
## Host Configuration Remains While No Project Is Currently Connected

Observable symptom: host configuration exists, but project-routed MCP tools cannot proceed, or `volicord.list_projects` returns an empty Project list.

Meaning: an Agent Connection can exist without any connected Project, but that state is not project-tool eligibility.

Bounded recovery:

```sh
"$VOLICORD_BIN/volicord" agent project add \
  --connection-id <connection_id> \
  --project-id <project_id> \
  --repo-root <repo_root> \
  --runtime-home <runtime_home>
```

Then rerun `volicord-mcp --check --connection <connection_id>` and the host verification command.

<a id="partial-removal"></a>
## Removal Completed Only Partially

Observable symptom: uninstall reports that some cleanup succeeded but another selected host target or stored connection record could not be removed.

Bounded recovery: run `volicord agent status --connection-id <connection_id>` and rerun uninstall after fixing the named host target, path permission, or ownership mismatch. Do not remove project state, artifact storage, or unrelated host entries by hand.

## Security Boundary

Agent Connections identify local MCP host connection context. They are not OS accounts, sandboxes, filesystem ACLs, network policy, or secret isolation. `Write Check` is Core-state compatibility for one product-file write attempt, not OS permission.
