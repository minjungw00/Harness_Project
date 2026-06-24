# Agent host setup

Use this guide when you need to install, verify, inspect, guide, or remove a Volicord MCP integration for Codex, Claude Code, or an unsupported host.

Start with [Installation](../getting-started/installation.md) to build or locate `volicord` and `volicord-mcp`, then [Quickstart](../getting-started/quickstart.md) for the shortest first setup. This guide covers the operational path after that.

Exact command behavior belongs to [Administrative CLI](../reference/admin-cli.md). Exact Agent Integration Profile, Host Installation, project selection, and guidance boundaries belong to [Agent Integration](../reference/agent-integration.md). Exact process behavior belongs to [MCP Transport](../reference/mcp-transport.md). Runtime and Product Repository write boundaries belong to [Runtime Boundaries](../reference/runtime-boundaries.md).

For symptom-specific recovery, use [Agent host troubleshooting](agent-host-troubleshooting.md). This setup guide keeps the normal operator flow first.

## Executable Convention

The command examples assume you have selected one absolute directory containing both `volicord` and `volicord-mcp`, then exported it in the current shell:

```sh
export VOLICORD_BIN="/absolute/path/to/selected/bin"
```

When building from the Volicord source repository root, a debug build can use:

```sh
export VOLICORD_BIN="$(pwd)/target/debug"
```

Replace `/absolute/path/to/selected/bin` with your real selected directory; do not copy it literally. `VOLICORD_BIN` is only a shell convenience variable for these examples. Volicord does not read it as runtime or host configuration. For release builds and installed-directory choices, see [Installation](../getting-started/installation.md).

Administrative commands use `"$VOLICORD_BIN/volicord"`. User-scope Codex, local-scope Claude Code, and generic export examples pass `--mcp-command "$VOLICORD_BIN/volicord-mcp"` so generated configuration stores the resolved absolute executable path. Project-scope examples keep generated project files portable by running with `PATH="$VOLICORD_BIN:$PATH"` for the administrative preflight and omitting `--mcp-command`; omission selects the portable `volicord-mcp` command for project scope.

Inline `PATH` and `VOLICORD_HOME` values on an administrative `volicord agent install` or `volicord agent verify` command apply to that command invocation and its checks. For project scope, the shared host configuration intentionally does not carry those command-local values forward: it stores `volicord-mcp` and no personal `VOLICORD_HOME`. A future project-scoped Codex or Claude Code process must start from a shell, launcher, service configuration, user environment, or equivalent execution environment whose `PATH` resolves `volicord-mcp`; if that host process would otherwise resolve a different Runtime Home, provide the intended `VOLICORD_HOME` through that same execution environment.

User and local scopes are different. Their managed host entries may persist the selected Runtime Home as `VOLICORD_HOME` and may store an absolute `volicord-mcp` executable path. Do not read the project-scope launch-environment requirement as a universal rule that every later host process always needs the same inline shell values configured again.

Generated configuration examples below use `/absolute/path/to/selected/bin/volicord-mcp` to stand in for the resolved selected path. Actual generated configuration contains the expanded path for user, local, and export scope, or the portable command for project scope, not the literal `VOLICORD_BIN` variable. Project-scoped shared configuration intentionally omits personal build paths and personal `VOLICORD_HOME`.

## Responsibilities

| Part | Owns | Notes |
|---|---|---|
| Volicord installation | `volicord` and `volicord-mcp` executables. | Source builds write under `target/`; installed executables may live elsewhere. |
| `Volicord Runtime Home` | Project registry, Agent Integration Profiles, integration project memberships, Host Installation inventory, and Volicord runtime data. | Keep it separate from every `Product Repository`. |
| `Product Repository` | Product files and explicitly selected project-scoped integration files. | Volicord runtime databases and runtime records are never stored there. |
| Codex or Claude Code | Host configuration, project trust, project MCP approval, reload/restart behavior, the environment used when starting MCP servers, and model tool choice. | Volicord cannot bypass host-owned decisions. |
| `volicord-mcp` process | One integration-bound stdio server started with `--integration <integration_id>`. | Project selection happens per public tool call. |

## Setup Sequence

For operators, `volicord agent install` follows this durable order. The detailed implementation map is in [Administrative agent setup flow](../development/architecture.md#administrative-agent-setup-flow).

1. The command parses host, scope, repository-write, guidance, Runtime Home, repository, integration, and executable inputs, then reads existing registry and host state to build project, integration, host, and optional guidance plans. Conflicts are rejected before persistent setup.
2. With `--dry-run`, the command returns the plan only and does not create Runtime Home state, write SQLite, run `volicord-mcp --check`, change host configuration, apply guidance, initialize MCP, or discover tools.
3. Without `--dry-run`, the command initializes or reuses Runtime Home and project state, then creates or reuses the agent surface, Agent Integration Profile, project membership, and default-project routing.
4. The command runs `volicord-mcp --check --integration <integration_id>` with the resolved Runtime Home before applying host configuration.
5. It applies the planned host configuration, then registers or updates Host Installation inventory before optional repository guidance.
6. Optional guidance is applied only when selected and explicitly authorized; final verification checks host readiness and, when the host gate permits it, performs MCP initialization and tool discovery. The Host Installation verification state is updated from that result, which may still be `action_required` when host-owned action remains.
7. If a failure happens after durable effects begin, output reports compensated effects and residual effects from the install journal. This is not one atomic rollback across Runtime Home, SQLite, Product Repository, and host boundaries.

## Install Argument Selection

Use this table to decide which install arguments belong in an operator command. Exact requiredness, validation edge cases, and defaults stay in [Administrative CLI](../reference/admin-cli.md#volicord-agent-install).

| Decision | Selection rule |
|---|---|
| Host and scope | Always choose `--host` and `--scope` together from the supported matrix: Codex uses `user` or `project`, Claude Code uses `local`, `project`, or `user`, and generic setup uses `export`. |
| Project selection | For a new project registration, provide both stable `--project-id` and repository path `--repo-root`. For an already registered project, `--project-id` alone can reuse the registered path. `--repo-root` alone can select a project only when it uniquely matches one existing executable registration; if it matches none or more than one, provide `--project-id`. |
| Integration identity | `--integration-id` is optional. Provide it when later status, verify, uninstall, generated host snippets, scripts, or multi-repository examples need a stable name. Omit it when deterministic generation is sufficient. |
| Default project | For a new integration, omit `--default-project-id` unless you are intentionally selecting a different already allowed default; the selected project becomes the default. For an existing integration, omission retains its existing default when present. Use `volicord agent project default set` for later default changes. |
| Runtime Home | Use `--runtime-home` or `VOLICORD_HOME` when selecting a non-default Runtime Home or making an operational example repeatable. Otherwise rely on normal Runtime Home resolution. Project-scoped host files do not persist a personal Runtime Home path. |
| MCP executable | For user, local, and export scope, either let the CLI discover `volicord-mcp` or provide an explicit absolute `--mcp-command` to pin a verified executable. For project scope, omit `--mcp-command`; the generated shared entry uses portable `volicord-mcp` and relies on the host launch `PATH`. |
| Repository-write authorization | Include `--allow-repository-write` on real project-scoped installs and on real installs that apply repository guidance. Do not add it to dry-run commands merely for symmetry; dry-run writes nothing. |
| Export destination | For `generic` `export`, use `--export-path` when you need one exact output file, `--export-dir` when the generated `volicord-<integration>.mcp.json` name is acceptable in a chosen directory, or omit both to use the current working directory. |
| Host server key | `--server-name` is optional. Keep it explicit when an example needs a short predictable host configuration key; otherwise let the CLI derive `volicord-<integration>`. |
| Preview and output | `--dry-run` is optional zero-write planning. `--output json` is optional machine-readable administrative output, useful for checking planned paths and actions. |

## Setup State Semantics

| State | Meaning |
|---|---|
| `complete` | Durable integration state exists, managed host configuration matches its fingerprint, the host-specific loadability gate is satisfied, no required trust or approval action remains, integration preflight succeeded, MCP initialization succeeded, and tool discovery exposed the required tools. |
| `action_required` | Durable integration state and host configuration are present, but host trust, project approval, OAuth, reload, restart, or a comparable user-controlled host action remains. |
| `partial_failure` | Some durable administrative action succeeded, but a later install, verify, host target, or cleanup step failed. Rerun after fixing the reported issue. |
| `failed` | The requested install or verification did not establish usable durable integration state or host configuration. |

Codex project scope remains `action_required` while Codex project trust cannot be confirmed. Claude Code project scope remains `action_required` while project MCP approval is pending. Rejected, missing, changed, unavailable, and unknown host states are not `complete`. Generic export remains `action_required` because Volicord cannot prove that a user-managed host loaded the exported configuration.

`volicord-mcp --check --integration <integration_id>` is only MCP startup validation. A direct Volicord-spawned MCP handshake is not proof that Codex or Claude Code loaded, trusted, approved, or exposed the server. Tool discovery does not guarantee every future model decision will choose Volicord tools. Repository guidance improves discoverability, but it is advisory context rather than enforcement.

When a result is not `complete`, use [Agent host troubleshooting](agent-host-troubleshooting.md) for the matching status or observed host state before repeating a write command.

## Dry-Run Before Writes

Use dry-run when a command might write host configuration or `Product Repository` guidance:

```sh
"$VOLICORD_BIN/volicord" agent install \
  --host codex \
  --scope user \
  --server-name volicord-main \
  --integration-id int-codex-team \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --runtime-home /Users/alex/.volicord \
  --mcp-command "$VOLICORD_BIN/volicord-mcp" \
  --dry-run \
  --output json
```

Dry-run reports planned Runtime Home actions, host target paths, and guidance target paths. It creates or modifies nothing: no Runtime Home directories, SQLite databases or rows, WAL or SHM files, registry migrations, host configuration, `Product Repository` guidance, generic export files, MCP host state, `volicord-mcp --check`, MCP initialization, or tool discovery.

With the current storage profile, registry schema version `1` is already the latest supported registry schema version. A dry-run against an existing current registry reports `registry_schema_version: 1`, `registry_latest_supported_schema_version: 1`, and `registry_migration_planned: false`, and writes no migration metadata.

The examples below pin `--server-name volicord-main` so the host snippets have a stable, human-readable key. The option is not required; if it is omitted, the CLI derives a stable server name from `integration_id` and reports it in the result.

## Codex User-Scope Install

Use user scope when one personal Codex configuration should load the same Volicord integration across Codex projects.

```sh
"$VOLICORD_BIN/volicord" agent install \
  --host codex \
  --scope user \
  --server-name volicord-main \
  --integration-id int-codex-team \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --runtime-home /Users/alex/.volicord \
  --mcp-command "$VOLICORD_BIN/volicord-mcp"
```

This may write:

- Runtime Home records under `/Users/alex/.volicord`
- a Codex user `config.toml` entry such as `[mcp_servers.volicord-main]`

It does not write `/work/acme-api` unless `--guidance codex`, `--guidance both`, or a separate guidance command is selected with `--allow-repository-write`.

Because this example creates a new integration and omits `--default-project-id`, `acme-api` becomes the default project. Keep `--server-name volicord-main` when you want that predictable host key; omit it when the derived `volicord-<integration>` name is sufficient.

Expected generated Codex shape:

```toml
[mcp_servers.volicord-main]
command = "/absolute/path/to/selected/bin/volicord-mcp"
args = ["--integration", "int-codex-team"]

[mcp_servers.volicord-main.env]
VOLICORD_HOME = "/Users/alex/.volicord"
```

The actual generated `command` value is the resolved absolute path selected through `VOLICORD_BIN`; generated TOML does not contain `VOLICORD_BIN`.

Codex project scope is also supported, but it writes `/work/acme-api/.codex/config.toml`, requires `--allow-repository-write` in noninteractive execution, uses `volicord-mcp` from `PATH`, and may report `action_required` until Codex trusts the project. The generated project entry stays portable with `command = "volicord-mcp"` and no personal `VOLICORD_HOME`. Launch or restart Codex for that project from an environment whose `PATH` resolves `volicord-mcp`, and provide `VOLICORD_HOME` there if that Codex process would otherwise resolve a different Runtime Home. Setting those values only on `volicord agent install` or `volicord agent verify` affects those administrative invocations, not later Codex processes.

## Claude Code Project Or Local Install

Project scope writes a team-shared `.mcp.json` file in the `Product Repository`.

```sh
VOLICORD_HOME=/Users/alex/.volicord \
PATH="$VOLICORD_BIN:$PATH" \
"$VOLICORD_BIN/volicord" agent install \
  --host claude-code \
  --scope project \
  --server-name volicord-main \
  --integration-id int-claude-acme \
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
      "args": ["--integration", "int-claude-acme"]
    }
  }
}
```

The `.mcp.json` entry intentionally stays portable: it stores `volicord-mcp` and no personal `VOLICORD_HOME`. The command intentionally omits `--mcp-command`; for project scope, omission selects portable `volicord-mcp`. The inline `VOLICORD_HOME` and `PATH` on the install command let that administrative command select `/Users/alex/.volicord` and find `volicord-mcp` for preflight. Because project scope omits those values from the shared entry, start or restart Claude Code from an environment that can resolve `volicord-mcp`, and provide `VOLICORD_HOME=/Users/alex/.volicord` if that host process would otherwise resolve a different Runtime Home.

Claude Code normally requires project MCP approval before it loads a project-scoped `.mcp.json` server. That result is `action_required`.

Local scope keeps the MCP server private to the current Claude Code project and uses Claude Code's own `claude mcp add --scope local` path through the CLI adapter:

```sh
VOLICORD_HOME=/Users/alex/.volicord \
"$VOLICORD_BIN/volicord" agent install \
  --host claude-code \
  --scope local \
  --server-name volicord-main \
  --integration-id int-claude-acme-local \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --mcp-command "$VOLICORD_BIN/volicord-mcp"
```

Local and project scopes are single-repository scopes. Use user scope when one explicitly allowed integration should serve multiple repositories.

## Optional Repository Guidance

Repository guidance is optional and must be explicitly authorized.

Codex guidance writes a Volicord-managed block in `AGENTS.md`:

```sh
"$VOLICORD_BIN/volicord" agent guidance apply \
  --integration-id int-codex-team \
  --project-id acme-api \
  --host codex \
  --runtime-home /Users/alex/.volicord \
  --dry-run \
  --allow-repository-write \
  --output json
```

Claude Code guidance writes `.claude/rules/volicord.md`:

```sh
"$VOLICORD_BIN/volicord" agent guidance apply \
  --integration-id int-codex-team \
  --project-id acme-api \
  --host claude-code \
  --runtime-home /Users/alex/.volicord \
  --allow-repository-write
```

Before guidance, the target file is either absent or has no Volicord-managed block:

```text
# Existing repository instructions
```

After Codex guidance, `AGENTS.md` contains a managed block:

```md
# Existing repository instructions

<!-- BEGIN VOLICORD MANAGED GUIDANCE v1 -->
## Volicord MCP guidance for Codex

...
<!-- END VOLICORD MANAGED GUIDANCE v1 -->
```

After Claude Code guidance, `.claude/rules/volicord.md` contains the same managed marker shape with `## Volicord MCP guidance for Claude Code`.

The managed content tells the host to use Volicord for scope, state, write preparation, run evidence, user judgment, and close-readiness tracking; to call `volicord.list_projects` when the target repository is unclear; and not to invent Volicord state in prose. The guidance also states that MCP server instructions and repository guidance cannot guarantee model behavior.

Guidance files are host configuration or advisory context. They are not Volicord runtime state, Core authority, evidence, acceptance, close readiness, residual-risk acceptance, or a security guarantee.

## Status And Verification

Inspect registry and host inventory:

```sh
"$VOLICORD_BIN/volicord" agent status \
  --integration-id int-codex-team \
  --runtime-home /Users/alex/.volicord
```

Refresh verification. This is another administrative invocation: provide its Runtime Home with `--runtime-home` or `VOLICORD_HOME`, and keep the selected directory on `PATH` when verifying an installation whose host configuration stores the portable `volicord-mcp` command. These values let verification launch its own check; they do not change what a later host process receives beyond values already persisted in its managed host entry or supplied by its own launch environment.

```sh
PATH="$VOLICORD_BIN:$PATH" \
"$VOLICORD_BIN/volicord" agent verify \
  --integration-id int-codex-team \
  --runtime-home /Users/alex/.volicord
```

Verification is performed per Host Installation. Add `--installation-id <id>` to verify exactly one installation; omit it to verify every Host Installation associated with the integration. Each installation keeps its own `last_verified_status`, and one installation's result does not overwrite another's.

Aggregate command status follows the selected installations:

| Selected installation results | Command status |
|---|---|
| All selected installations are `complete` | `complete` |
| At least one is `action_required`, and none is `partial_failure` or `failed` | `action_required` |
| At least one is `partial_failure`, and none is `failed` | `partial_failure` |
| At least one is `failed` | `failed` |

The aggregate status is never `complete` while any selected installation is not `complete`.

Direct MCP startup inspection:

```sh
VOLICORD_HOME=/Users/alex/.volicord \
"$VOLICORD_BIN/volicord-mcp" --check --integration int-codex-team
```

`--check` should report `configuration: valid`, `transport: stdio`, the `integration_id`, allowed project counts, and `verification_scope: startup_check_only`. It does not prove the host loaded or exposed tools.

## Troubleshooting Routing

Use [Agent host troubleshooting](agent-host-troubleshooting.md) when setup or verification does not reach `complete`.

- `action_required` usually means host-owned trust, approval, reload, restart, or executable availability remains. Start with [`status: action_required`](agent-host-troubleshooting.md#status-action_required).
- `partial_failure` means a durable administrative action succeeded before a later step failed. Start with [`status: partial_failure`](agent-host-troubleshooting.md#status-partial_failure) and inspect `effects` and `residual_effects`.
- `failed` means the requested install or verification did not establish usable durable integration state or host configuration. Start with [`status: failed`](agent-host-troubleshooting.md#status-failed).
- Missing executables, path mistakes, host file conflicts, empty allowlists, ambiguous project selection, and managed fingerprint conflicts each have focused recovery paths in the troubleshooting guide.

Do not treat status inventory or `volicord-mcp --check` as proof that the external host loaded the server. Use `volicord agent verify` after the host-owned recovery action has been completed.

## Safe Removal

A project that is still `default_project_id` cannot be removed. In a two-project integration, first change the default to the project that should remain:

```sh
"$VOLICORD_BIN/volicord" agent project default set \
  --integration-id int-codex-team \
  --project-id billing-api \
  --runtime-home /Users/alex/.volicord
```

Expected result includes:

```text
prior_default_project_id: acme-api
resulting_default_project_id: billing-api
```

After the default has moved, remove the formerly default project without rewriting host configuration:

```sh
"$VOLICORD_BIN/volicord" agent project remove \
  --integration-id int-codex-team \
  --project-id acme-api \
  --runtime-home /Users/alex/.volicord
```

Expected result includes:

```text
allowed_projects:
  billing-api
verification_detail: project membership removed; host configuration was not rewritten
```

To remove the final allowed project, clear the default first:

```sh
"$VOLICORD_BIN/volicord" agent project default clear \
  --integration-id int-codex-team \
  --runtime-home /Users/alex/.volicord
```

Then remove the final membership:

```sh
"$VOLICORD_BIN/volicord" agent project remove \
  --integration-id int-codex-team \
  --project-id billing-api \
  --runtime-home /Users/alex/.volicord
```

Expected result includes:

```text
allowed_project_count: 0
not executable until one is added
```

The Agent Integration Profile, Host Installation inventory, and host configuration can remain while no projects are allowed, but that retained state is not startup eligibility. For recovery from this state, see [Host configuration remains while no project is currently allowed](agent-host-troubleshooting.md#host-config-remains-zero-projects).

Fully remove managed host configuration and managed guidance:

```sh
"$VOLICORD_BIN/volicord" agent uninstall \
  --integration-id int-codex-team \
  --runtime-home /Users/alex/.volicord \
  --allow-repository-write \
  --remove-managed
```

Uninstall removes selected Volicord-managed host entries, blocks, files, or fingerprints only when ownership and safety checks permit removal. With `--remove-managed`, managed `Product Repository` guidance is removed only when selected and safely owned. A successful uninstall also removes the corresponding Host Installation inventory; if no Host Installations remain for the Agent Integration Profile, the profile can be disabled, which is not deletion. It preserves `Product Repository` contents, project registration and project state, Core task, evidence, decision, run, and artifact-related records, artifact storage, and unrelated host configuration. User-modified or unmanaged host entries are reported or preserved rather than removed.

## Generic Export Fallback

Use generic export only for a host that Volicord does not install directly:

```sh
"$VOLICORD_BIN/volicord" agent install \
  --host generic \
  --scope export \
  --server-name volicord-main \
  --integration-id int-generic-acme \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --runtime-home /Users/alex/.volicord \
  --mcp-command "$VOLICORD_BIN/volicord-mcp" \
  --export-dir /tmp/volicord-mcp-export
```

The exported JSON contains one `mcpServers.volicord-main` entry with `command`, `args = ["--integration", "int-generic-acme"]`, and `VOLICORD_HOME` when applicable:

```json
{
  "mcpServers": {
    "volicord-main": {
      "command": "/absolute/path/to/selected/bin/volicord-mcp",
      "args": ["--integration", "int-generic-acme"],
      "env": {
        "VOLICORD_HOME": "/Users/alex/.volicord"
      }
    }
  }
}
```

Generic export does not claim the host loaded the server; install and verify results remain `action_required` until a future host-specific owner defines an observable loadability gate.
