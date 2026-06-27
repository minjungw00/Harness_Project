# Agent Host Setup

Use this guide when you need to connect, verify, inspect, or remove a Volicord MCP host connection for Codex, Claude Code, or a generic MCP configuration export.

Start with [Installation](../getting-started/installation.md) to build or locate `volicord` and `volicord-mcp`, then [Quickstart](../getting-started/quickstart.md) for the shortest first setup. Exact command behavior belongs to [Administrative CLI](../reference/admin-cli.md). Exact Agent Connection behavior belongs to [Agent Connection Reference](../reference/agent-integration.md). Exact process behavior belongs to [MCP Transport](../reference/mcp-transport.md).

Volicord is not an OS security product. Agent Connections, Write Checks, and host configuration do not provide OS sandboxing, filesystem ACLs, network policy, or secret isolation.

## Executable Convention

The command examples assume one absolute directory contains both executables:

```sh
export VOLICORD_BIN="/absolute/path/to/selected/bin"
```

When building from this repository root, a debug build can use:

```sh
export VOLICORD_BIN="$(pwd)/target/debug"
```

`VOLICORD_BIN` is only a shell convenience variable for these examples. Volicord does not read it as runtime or host configuration.

## Responsibilities

| Part | Owns | Notes |
|---|---|---|
| Volicord installation | `volicord` and `volicord-mcp` executables. | Source builds write under `target/`; installed executables may live elsewhere. |
| `Volicord Runtime Home` | Project registry, Agent Connections, connected Projects, and Volicord runtime data. | Keep it separate from every `Product Repository`. |
| `Product Repository` | Product files and explicitly selected project-scoped host configuration. | It is not Core authority and must not contain Runtime Home databases. |
| Codex or Claude Code | Host configuration loading, project trust, project MCP approval, reload/restart behavior, the process environment used for MCP servers, and model tool choice. | Volicord cannot bypass host-owned decisions. |
| `volicord-mcp` process | One connection-bound stdio server started with `--connection <connection_id>`. | Project routing is per public tool call. |

## Setup Sequence

`volicord agent connect` follows this operator-visible order:

1. Parse host, scope, project, connection, mode, Runtime Home, executable, output, and approval options.
2. Resolve or register exactly one selected Project.
3. Create or reuse one Agent Connection with mode `read_only` or `workflow`.
4. Add the selected Project to `connection_projects`.
5. Run `volicord-mcp --check --connection <connection_id>` with the selected Runtime Home.
6. Install or export host configuration that starts `volicord-mcp --connection <connection_id>`.
7. Verify host readiness where the selected host exposes enough state.

`--dry-run` previews the plan and writes nothing.

## Argument Selection

| Decision | Selection rule |
|---|---|
| Host and scope | Always choose `--host` and `--scope` together from the supported matrix: Codex uses `user` or `project`; Claude Code uses `local`, `project`, or `user`; generic setup uses `export`. |
| Project selection | For a new Project registration, provide both `--project-id` and `--repo-root`. For an already registered Project, `--project-id` alone can reuse the registered path. |
| Connection identity | `--connection-id` is optional. Provide it when later status, verify, uninstall, scripts, or examples need a stable name. |
| Connection mode | Omit `--mode` only for a read-only connection. Use `--mode workflow` when the agent host should use workflow tools. |
| Runtime Home | Use `--runtime-home` or `VOLICORD_HOME` when selecting a non-default Runtime Home or making an example repeatable. Project-scoped host files do not persist a personal Runtime Home path. |
| MCP executable | For user, local, and export scope, either let the CLI discover `volicord-mcp` or provide an explicit absolute `--mcp-command`. For project scope, omit `--mcp-command`; generated shared configuration uses portable `volicord-mcp` and relies on the host launch `PATH`. |
| Repository write approval | Include `--allow-repository-write` on real project-scoped connections. Do not add it to dry-run commands merely for symmetry; dry-run writes nothing. |
| Export destination | For `generic` `export`, use `--export-path` for one exact file or `--export-dir` for a generated file name in a chosen directory. |

## Result States

| State | Meaning |
|---|---|
| `complete` | Durable Agent Connection state exists, managed host configuration matches its expected fingerprint, required host gates are satisfied, connection preflight succeeds, MCP initialization succeeds, and tool discovery exposes the required tools. |
| `action_required` | Durable Agent Connection state and host configuration are present, but host trust, project approval, OAuth, reload, restart, or a comparable user-controlled host action remains. |
| `failed` | The requested connection or verification did not establish usable durable connection state or host configuration. |

A successful `volicord-mcp --check --connection <connection_id>` is startup validation only. It is not proof that Codex, Claude Code, or a generic host loaded, trusted, approved, or exposed the server.

## Dry-Run Before Writes

Use dry-run when a command might write host configuration or project-scoped configuration:

```sh
"$VOLICORD_BIN/volicord" agent connect \
  --host codex \
  --scope user \
  --server-name volicord-main \
  --connection-id conn-codex-team \
  --mode workflow \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --runtime-home /Users/alex/.volicord \
  --mcp-command "$VOLICORD_BIN/volicord-mcp" \
  --dry-run \
  --output json
```

Dry-run reports planned Runtime Home actions, host target paths, and connection details. It creates or modifies nothing.

## Codex User-Scope Connection

Use user scope when one personal Codex configuration should load the same Volicord connection across one or more explicitly connected Projects.

```sh
"$VOLICORD_BIN/volicord" agent connect \
  --host codex \
  --scope user \
  --server-name volicord-main \
  --connection-id conn-codex-team \
  --mode workflow \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --runtime-home /Users/alex/.volicord \
  --mcp-command "$VOLICORD_BIN/volicord-mcp"
```

Expected generated Codex shape:

```toml
[mcp_servers.volicord-main]
command = "/absolute/path/to/selected/bin/volicord-mcp"
args = ["--connection", "conn-codex-team"]

[mcp_servers.volicord-main.env]
VOLICORD_HOME = "/Users/alex/.volicord"
```

## Codex Or Claude Code Project Scope

Project scope writes host configuration into the selected `Product Repository`. Use it only when that repository should carry a shared host entry.

```sh
VOLICORD_HOME=/Users/alex/.volicord \
PATH="$VOLICORD_BIN:$PATH" \
"$VOLICORD_BIN/volicord" agent connect \
  --host claude-code \
  --scope project \
  --server-name volicord-main \
  --connection-id conn-claude-acme \
  --mode workflow \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --allow-repository-write
```

Expected Claude Code `.mcp.json` shape:

```json
{
  "mcpServers": {
    "volicord-main": {
      "command": "volicord-mcp",
      "args": ["--connection", "conn-claude-acme"]
    }
  }
}
```

The project-scoped file is a product-file boundary exception for selected host configuration. It is not Core authority and does not store Runtime Home records.

## Generic Export

Use `generic` `export` when Volicord should write an MCP configuration object for a host you manage yourself.

```sh
"$VOLICORD_BIN/volicord" agent connect \
  --host generic \
  --scope export \
  --server-name volicord-main \
  --connection-id conn-generic-acme \
  --mode workflow \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --runtime-home /Users/alex/.volicord \
  --mcp-command "$VOLICORD_BIN/volicord-mcp" \
  --export-path /tmp/volicord-main.mcp.json
```

Generic export remains `action_required` until the user-managed host loads and verifies the exported configuration.

## Inspect, Verify, Enable, Disable

```sh
"$VOLICORD_BIN/volicord" agent list \
  --runtime-home /Users/alex/.volicord

"$VOLICORD_BIN/volicord" agent status \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord

"$VOLICORD_BIN/volicord" agent verify \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord

"$VOLICORD_BIN/volicord" agent disable \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord

"$VOLICORD_BIN/volicord" agent enable \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord
```

Disable and enable change stored Agent Connection eligibility. They do not rewrite host configuration and do not make user-owned judgments.

## Connected Projects

User-scope connections may connect more than one Project:

```sh
"$VOLICORD_BIN/volicord" agent project add \
  --connection-id conn-codex-team \
  --project-id billing-api \
  --repo-root /work/billing-api \
  --runtime-home /Users/alex/.volicord
```

When exactly one Project is connected, MCP calls may omit `project_id`. When multiple Projects are connected, MCP calls must include explicit `project_id` unless they are using `volicord.list_projects`.

Remove one connected Project with:

```sh
"$VOLICORD_BIN/volicord" agent project remove \
  --connection-id conn-codex-team \
  --project-id billing-api \
  --runtime-home /Users/alex/.volicord
```

Project and local scopes are single-Project scopes; use user scope for multi-repository operation.

## Uninstall

```sh
"$VOLICORD_BIN/volicord" agent uninstall \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord \
  --dry-run

"$VOLICORD_BIN/volicord" agent uninstall \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord
```

Project-scoped uninstall that removes a repository file requires `--allow-repository-write` on the real command. Uninstall removes selected managed host configuration when ownership and safety checks permit it, removes associated Connection Project records, and removes the Agent Connection record when unused. It preserves Product Repository contents, project registration and project state, Core task/evidence/decision/run/artifact records, artifact storage, and unrelated host entries according to their owners.

## Tool Exposure

`read_only` mode exposes read/project discovery operations: `volicord.status`, `volicord.close_task` for close-readiness checks, and `volicord.list_projects`.

`workflow` mode exposes read operations plus agent workflow operations: `volicord.intake`, `volicord.update_scope`, `volicord.status`, `volicord.prepare_write`, `volicord.stage_artifact`, `volicord.record_run`, `volicord.request_user_judgment`, `volicord.close_task`, and `volicord.list_projects`.

`workflow` mode does not expose `volicord.record_user_judgment`. User judgment recording belongs to the User Channel.

## Troubleshooting Routes

- `action_required`: [status: action_required](agent-host-troubleshooting.md#status-action_required)
- `failed`: [status: failed](agent-host-troubleshooting.md#status-failed)
- missing `volicord-mcp`: [volicord-mcp is missing](agent-host-troubleshooting.md#missing-volicord-mcp)
- multiple connected Projects and no selector: [ambiguous project selection](agent-host-troubleshooting.md#ambiguous-project-selection)
