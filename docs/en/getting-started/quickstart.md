# Quickstart

This tutorial is the shortest supported first setup path for a real local agent host. It starts after [Installation](installation.md), uses one `Product Repository`, and gives you a clear choice between a personal Codex user-scope connection and a project-scoped Claude Code `.mcp.json` connection.

For complete host setup options, dry-run previews, multi-project operation, removal, and troubleshooting, see [Agent Host Setup](../guides/agent-host-setup.md).

## Audience, Goal, And Completion

Audience: first-time users or operators who have already verified local `volicord` and `volicord-mcp` executables and want one agent host path to work before expanding setup.

Goal: create one Agent Connection, recognize whether the first result is `complete` or `action_required`, and run an independent verification command for the chosen path.

Completion state: the chosen path is complete when `volicord agent verify --connection-id <connection_id>` reports `status: complete`. If the command reports `action_required`, complete the named host-owned trust, approval, reload, or restart action and rerun verification.

## Starting State And Example Values

Before running these commands:

- Complete [Installation](installation.md) in a POSIX-style shell.
- Keep `VOLICORD_BIN` set to the verified absolute directory containing both executables.
- Choose a `Product Repository` that is not the `Volicord Runtime Home` and is not inside or above it.
- Replace every example path and ID below with your real value.

Check focused command help before applying setup:

```sh
"$VOLICORD_BIN/volicord" agent connect --help
```

The examples use these values:

| Value | Kind | How this walkthrough uses it |
|---|---|---|
| `VOLICORD_BIN="/absolute/path/to/selected/bin"` | Tutorial shell variable | Selected absolute directory containing both `volicord` and `volicord-mcp`. |
| `/Users/alex/.volicord` | Example path | `Volicord Runtime Home`; keep it distinct from the `Product Repository`. |
| `/work/acme-api` | Example path | Product Repository A. |
| `acme-api` | Example identifier | Stable logical project ID for Product Repository A. |
| `conn-codex-team`, `conn-claude-acme` | Example identifiers | Predictable `connection_id` values used by later verify, status, configuration, and related commands. |
| `volicord-main` | Example server name | Human-readable host MCP server key. |

## Choose One Host Path

| Path | Choose when | Consequence |
|---|---|---|
| Path A: Codex `user` scope | One personal Codex MCP entry should serve this repository now and may later serve more explicitly connected repositories. | Host configuration lives in the Codex user config and stores an absolute `volicord-mcp` command path plus `VOLICORD_HOME`. |
| Path B: Claude Code `project` scope | Product Repository A should carry a team-shared Claude Code `.mcp.json` entry. | The project file uses portable `volicord-mcp`, omits personal `VOLICORD_HOME`, requires `--allow-repository-write` on the real apply command, and may remain `action_required` until Claude Code approval is complete. |

Use `--mode workflow` when the agent host should expose workflow tools. Omit it only when a read-only connection is intended.

## Path A: Codex User-Scope Setup

Use this when one personal Codex MCP entry should serve one or more explicitly connected `Product Repository` registrations.

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

Locations that may change:

| Location | What may change |
|---|---|
| `/Users/alex/.volicord` | Runtime Home registry, Agent Connection, connected project, and project state records. |
| Codex user config, normally `~/.codex/config.toml` or `CODEX_HOME/config.toml` | A `[mcp_servers.volicord-main]` table. |
| `/work/acme-api` | No file change from this command. |

Expected generated Codex shape:

```toml
[mcp_servers.volicord-main]
command = "/absolute/path/to/selected/bin/volicord-mcp"
args = ["--connection", "conn-codex-team"]

[mcp_servers.volicord-main.env]
VOLICORD_HOME = "/Users/alex/.volicord"
```

Independent completion check:

```sh
"$VOLICORD_BIN/volicord" agent verify \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord
```

Path A is complete when verification reports `status: complete`. If verification reports `action_required`, read the named action. A common Codex user-scope cause is that `codex` is missing from the administrative command `PATH` or cannot run `codex --version`.

## Path B: Claude Code Project-Scope Setup

Use this when Product Repository A should carry a team-shared Claude Code `.mcp.json` entry.

Optional dry-run before writing the project file:

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
  --dry-run \
  --output json
```

Apply the setup:

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

Expected `.mcp.json` shape:

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

The project file is not Core authority. It is selected host configuration stored inside the `Product Repository`. Runtime records remain in the Runtime Home.

Independent completion check:

```sh
VOLICORD_HOME=/Users/alex/.volicord \
PATH="$VOLICORD_BIN:$PATH" \
"$VOLICORD_BIN/volicord" agent verify \
  --connection-id conn-claude-acme
```

Project-scoped Claude Code setup may report `action_required` until Claude Code approves the project MCP server, reloads, or restarts. Complete only the named host-owned action, then rerun verification.

## MCP Startup Check

For either path, `volicord-mcp --check` validates only local adapter startup for one Agent Connection:

```sh
VOLICORD_HOME=/Users/alex/.volicord \
"$VOLICORD_BIN/volicord-mcp" --check --connection conn-codex-team
```

A successful startup check is not proof that Codex or Claude Code loaded, trusted, approved, or exposed the server.

## What The Agent Can Do

The Agent Connection can access only explicitly connected Projects. In this quickstart, there is exactly one connected Project, so MCP calls may omit `project_id`. After more Projects are connected to the same connection, MCP calls must include an explicit `project_id` unless the agent is calling `volicord.list_projects`.

`read_only` mode exposes read and project-discovery operations. `workflow` mode exposes agent workflow operations, but it does not expose `volicord.record_user_judgment`; user judgment recording belongs to the User Channel.

`Write Check` is Core-state compatibility for one product-file write attempt. It is not OS permission, OS sandboxing, filesystem ACLs, network policy, or secret isolation.

## Common Next Steps

| Need | Route |
|---|---|
| Add another Project to a user-scope connection. | [Multi-Repository Agent Setup](../guides/multi-repository-agent-setup.md) |
| Inspect connection state. | `volicord agent status --connection-id <connection_id>` |
| See available Projects from MCP. | `volicord.list_projects` |
| Record a pending user judgment. | [User Channel commands](../reference/admin-cli.md#user-channel-commands) |
| Setup cannot resolve `volicord-mcp`. | [Missing executable troubleshooting](../guides/agent-host-troubleshooting.md#missing-volicord-mcp) |
| Result is `action_required`. | [Action-required troubleshooting](../guides/agent-host-troubleshooting.md#status-action_required) |
| Result is `failed`. | [Failed troubleshooting](../guides/agent-host-troubleshooting.md#status-failed) |
