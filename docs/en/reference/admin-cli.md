# Administrative CLI reference

This document owns the local `volicord` administrative and bootstrap CLI contract. The CLI initializes a `Volicord Runtime Home`, registers local projects, provides the local `User Channel` path for selected user-facing Core actions, manages Agent Connections, installs host configuration for supported coding-agent hosts, and verifies host connection state. These commands are not public Volicord API methods.

It does not define public API method behavior, API schemas, storage record layout, security guarantees, Core authority semantics, or MCP stdio transport behavior.

## Owns / does not own

This document owns:

- `volicord` command names, command-line arguments, defaults, stdout/stderr routing, and process exit codes
- Runtime Home path selection for `volicord` administrative commands
- administrative project registration defaults
- local `User Channel` command names and command output
- Agent Connection command behavior
- Connection Project membership command behavior
- host connection, status, verification, and uninstall command behavior for Codex, Claude Code, and generic export
- setup result states, dry-run behavior, machine-readable output, and noninteractive approval behavior
- the boundary between administrative commands, local `User Channel` commands, and public Volicord API methods

This document does not own:

- public Volicord API methods; see [API Methods](api/methods.md)
- Agent Connection, Connection Projects, connection mode, and actor provenance meanings; see [Agent Connection](agent-connection.md)
- runtime data boundary meaning and `Product Repository` file-boundary exceptions; see [Runtime Boundaries](runtime-boundaries.md)
- MCP process startup, stdio framing, wire behavior, response wrapping, preflight internals, and shutdown; see [MCP Transport](mcp-transport.md)
- storage record layout, SQLite DDL, general storage migration definitions, Core authority semantics, and security guarantee meanings

## Command model

`volicord` is a local administrative/bootstrap executable. It is not a long-running server. The `volicord user` command group is the local `User Channel` CLI adapter over selected Core methods; its command names remain administrative CLI commands, not public Volicord API methods.

Supported baseline commands:

```text
volicord --help
volicord --version
volicord init [--runtime-home-id ID]
volicord project register --project-id ID --repo-root PATH [--status active]
volicord project list
volicord user status --project-id ID [--task-id ID] [--runtime-home PATH] [--output text|json]
volicord user judgment list --project-id ID [--task-id ID] [--runtime-home PATH] [--output text|json]
volicord user judgment show --project-id ID --judgment-id ID [--runtime-home PATH] [--output text|json]
volicord user judgment record --project-id ID --judgment-id ID --option-id ID [--request-id ID] [--idempotency-key KEY] [--expected-state-version VERSION] [--note TEXT] [--runtime-home PATH] [--output text|json]
volicord agent connect --host codex|claude-code|claude_code|generic --scope user|project|local|export [--project-id ID] [--repo-root PATH] [--connection-id ID] [--mode read_only|workflow] [--server-name NAME] [--mcp-command PATH] [--runtime-home PATH] [--export-path PATH|--export-dir PATH] [--output text|json] [--dry-run] [--allow-repository-write] [--replace-managed]
volicord agent list [--runtime-home PATH] [--output text|json]
volicord agent status --connection-id ID [--runtime-home PATH] [--output text|json]
volicord agent enable --connection-id ID [--runtime-home PATH] [--output text|json]
volicord agent disable --connection-id ID [--runtime-home PATH] [--output text|json]
volicord agent project add --connection-id ID --project-id ID [--repo-root PATH] [--runtime-home PATH] [--output text|json] [--dry-run]
volicord agent project remove --connection-id ID --project-id ID [--runtime-home PATH] [--output text|json] [--dry-run]
volicord agent verify --connection-id ID [--runtime-home PATH] [--output text|json]
volicord agent uninstall --connection-id ID [--runtime-home PATH] [--output text|json] [--dry-run] [--allow-repository-write]
```

Exit and stream behavior:

- Successful commands write success output to stdout and exit with code `0`.
- `action_required` is a successful administrative result and exits `0`.
- `failed`, runtime errors, storage errors, preflight failures, verification failures, and conflicts exit `1`.
- Usage errors write diagnostics to stderr and exit with code `2`.
- `volicord --version` writes `volicord <version>` to stdout and does not require Runtime Home resolution.
- `--output json` writes exactly one JSON document to stdout and does not mix human explanation into stdout.
- Errors remain stderr diagnostics under the existing CLI exit-code model.

Not supported:

- `volicord setup` and `volicord setup local-mcp` are not supported commands.
- The CLI has no `serve`, `server`, or `connect` command.
- The public `volicord agent` contract has no broad automatic-confirmation flag. Use the explicit approval and replacement flags this contract requires.
- Administrative commands are not public Volicord API methods and must not be added to the public method list.

## Runtime Home selection

The `volicord` administrative CLI uses these Runtime Home path resolution rules. `volicord-mcp` process environment and current MCP Runtime Home resolution are owned by [MCP Transport](mcp-transport.md#process-environment).

Resolution order:

1. Command-specific `--runtime-home` when the command defines it.
2. `VOLICORD_HOME`.
3. The first non-empty home source in this order: `HOME`, `USERPROFILE`, then `HOMEDRIVE` plus `HOMEPATH`, with `.volicord` appended.

Rules:

- A present but empty `VOLICORD_HOME` is an error.
- A command-specific `--runtime-home` value must be absolute when the command performs setup, installation, verification, or migration planning.
- A relative `VOLICORD_HOME` is resolved against the process current working directory without requiring the path to exist.
- `volicord init` may create or validate the selected Runtime Home registry.
- Other administrative commands require the selected Runtime Home to contain the records needed for the requested operation.

<a id="user-channel-commands"></a>
<a id="user-interaction-commands"></a>
## User Channel commands

`volicord user` commands provide a local CLI path for a human user to inspect task status and answer pending user judgments through the `User Channel`. They do not require a setup command, user-facing adapter registration, or adapter identifiers. They do not create an Agent Connection, install MCP host configuration, or make an Agent Connection eligible to act as the user.

`volicord user status` shows user-oriented task status through `volicord.status` with `actor_source=local_user`, `operation_category=read`, and `verification_basis=cli_direct_user_channel`.

`volicord user judgment list` reads pending judgments for the active or selected task and lists their task, judgment kind, status, question, and options.

`volicord user judgment show` reads one pending or historical judgment and displays the stored request, context summary, and Core-generated options.

`volicord user judgment record` requires a pending judgment and a `--option-id` that names one of that judgment's stored Core-generated options. It records the selection through `volicord.record_user_judgment` with `actor_source=local_user`, `operation_category=user_only`, `verification_basis=cli_direct_user_channel`, and `assurance_level=local_user_channel`. The selected option's `machine_action` and `resolution_outcome` determine the recorded answer; `--note` is stored only as a note. When `--request-id`, `--idempotency-key`, or `--expected-state-version` are omitted, the command supplies a local request id, a local idempotency key, and the current project state version. Agent Connections are not eligible for `volicord user judgment record`.

Recording one judgment records only the addressed judgment. Final acceptance and residual-risk acceptance remain separate judgment kinds and actions; this command must not collapse one into the other.

Stable judgment workflow:

1. Run `volicord user status --project-id ID [--task-id ID]` to inspect current
   task status, pending judgment count, close state, and next actions.
2. Run `volicord user judgment list --project-id ID [--task-id ID]` to see the
   pending judgments for the active or selected task.
3. Run `volicord user judgment show --project-id ID --judgment-id ID` to inspect
   the stored request, context summary, and Core-generated options.
4. Run `volicord user judgment record --project-id ID --judgment-id ID --option-id ID`
   to record one selected Core-generated option for that judgment.

Status, list, and show output expose selected owner state for the user's next
action. They do not create evidence, final acceptance, residual-risk acceptance,
or close readiness. Only `volicord user judgment record` mutates the addressed
pending judgment, and it does so only through the selected Core-generated
option.

## Host and scope support

Supported host and scope values:

| `--host` | Supported `--scope` values | Baseline target |
|---|---|---|
| `codex` | `user`, `project` | User config is Codex user `config.toml`. Project config is `.codex/config.toml` in the associated `Product Repository`. |
| `claude_code` | `local`, `project`, `user` | Local and user config are Claude Code user-owned configuration targets. Project config is `.mcp.json` in the associated `Product Repository`. The CLI may accept `claude-code` as an alias, but stored records use `claude_code`. |
| `generic` | `export` | Export an explicit MCP configuration object without claiming direct installation. |

Scope rules:

- `project` and `local` scopes permit exactly the associated `Product Repository`.
- `user` scope may permit multiple explicitly added projects, but each `volicord agent connect` invocation still selects exactly one project by the connection project-selection rules.
- `generic export` writes or prints only an explicit configuration export and does not claim host loading.
- Unsupported host/scope combinations are usage errors.

Host configuration shape:

- Codex connection writes an MCP server table equivalent to `[mcp_servers.<server_name>]` with `command`, `args = ["--connection", "<connection_id>"]`, and optional `env.VOLICORD_HOME` when the selected host scope permits persisting the selected Runtime Home path.
- Claude Code connection writes an MCP server entry under `mcpServers.<server_name>` with `command`, `args`, and optional `env.VOLICORD_HOME` when the selected host scope permits persisting the selected Runtime Home path.
- Generic export emits the same command, args, and environment values in a host-neutral JSON object.
- When generic export writes to an export directory instead of an explicit `--export-path`, the generated file name is `volicord-<connection>.mcp.json`.
- User and local scopes may use a discovered canonical `volicord-mcp` executable path or an explicit valid absolute path.
- Project-scoped shared configuration must use the portable command `volicord-mcp` and rely on `PATH` in the host environment. It must not embed a personal build path, home-directory path, or personal `VOLICORD_HOME`.
- Generic export may emit an explicitly selected absolute command path, but exported configuration is still `action_required` until a user-managed host loads and verifies it.
- New baseline host configuration must not require legacy routing environment variables.

Host trust boundary:

- Writing configuration is distinct from the host loading and exposing the MCP server.
- Codex project-scoped configuration may require Codex project trust before it loads.
- Claude Code project-scoped MCP configuration may require project MCP approval before it loads.
- Volicord must not claim that host trust, project trust, project MCP approval, OAuth, or comparable user-controlled host actions can be bypassed.

<a id="agent-connection-result-states"></a>
<a id="agent-setup-result-states"></a>
## Agent Connection result states

The agent command family uses these connection result states:

| State | Meaning |
|---|---|
| `complete` | Durable Agent Connection state exists, managed host configuration exists and matches its expected fingerprint, the host-specific loadability gate is satisfied, no required trust or approval action remains, connection preflight succeeds, MCP initialization succeeds, and `tools/list` succeeds with the required tools. |
| `action_required` | Durable Agent Connection state and host configuration are present, but host trust, project approval, OAuth, reload, restart, or a comparable user-controlled host action remains. |
| `failed` | The requested connection or verification did not establish usable durable Agent Connection state or host configuration. |

`dry_run` is an output status, not a setup result state.

A successful `volicord-mcp --check --connection <connection_id>` alone must not be described as a `complete` Agent Connection. It is only startup validation for the MCP process.

Host-specific state rules:

- Codex project scope remains `action_required` while Codex project trust cannot be confirmed.
- Claude Code project scope remains `action_required` while project MCP approval is pending.
- Rejected, missing, changed, unavailable, and unknown host states must not become `complete`.
- Generic export remains `action_required` because Volicord cannot prove that an external host loaded the exported configuration.

<a id="volicord-agent-connect"></a>
<a id="volicord-agent-install"></a>
## `volicord agent connect`

`volicord agent connect` creates or reuses an Agent Connection, explicitly connects one selected project, installs or exports host configuration, and verifies the result where the host can be checked.

Argument requiredness and omission behavior:

| Argument | Requiredness | Meaning, applicability, and omission behavior |
|---|---|---|
| `--host codex|claude-code|claude_code|generic` | Always required | Selects the host adapter. The value must be valid with the selected `--scope`; `claude-code` is accepted as an alias for `claude_code`. |
| `--scope user|project|local|export` | Always required | Selects the host configuration or export target. The value must be valid with the selected `--host`. |
| `--project-id ID` | Conditionally required | Names the selected project. It is required for an unregistered project, required when `--repo-root` matches no existing registration, and required when a supplied `--repo-root` is ambiguous. It may be omitted only when `--repo-root` uniquely matches one existing executable project registration. |
| `--repo-root PATH` | Conditionally required | Identifies the selected project's `Product Repository` for project selection and registration. It is required with `--project-id` when that project is not already registered. When omitted for an already registered `--project-id`, the command reuses the registered repository path. |
| `--connection-id ID` | Optional | Selects an existing Agent Connection or the desired id for a new Agent Connection. When omitted, the command derives a stable deterministic connection id from the selected host, scope, project, target, and server name. |
| `--mode read_only|workflow` | Optional | Selects the tool exposure mode for the Agent Connection. Omission defaults to `read_only`; workflow tools require `--mode workflow`. |
| `--server-name NAME` | Optional | Selects the host MCP server name. When omitted, the command uses `volicord`. |
| `--mcp-command PATH` | Optional | Selects the `volicord-mcp` executable where an explicit command is allowed. Project scope defaults to the portable `volicord-mcp` command and must keep that portable command. User, local, and export scopes discover an executable from the current `volicord` executable's sibling directory and then `PATH` when omitted; an explicit command for those scopes must satisfy the existing executable-path rules. |
| `--runtime-home PATH` | Optional | Selects the `Volicord Runtime Home` used by the administrative command. When omitted, the command uses the Runtime Home resolution order above. For non-project host scopes, the selected Runtime Home may be persisted in managed host configuration as `VOLICORD_HOME`. For project scope, shared host configuration must not embed a developer-specific Runtime Home path; a project-scoped host process that must use a non-default Runtime Home must receive `VOLICORD_HOME` through its actual execution environment. An environment variable set only for the administrative connection command is not automatically inherited by future host processes. |
| `--export-path PATH` | Optional | For `generic` `export`, selects an explicit output path for the exported MCP configuration. When omitted, the export path is derived from `--export-dir` or the current working directory. |
| `--export-dir PATH` | Optional | For `generic` `export`, selects the directory used with the generated `volicord-<connection>.mcp.json` file name when `--export-path` is omitted. When neither export destination is supplied, the command uses the current working directory and derives that file name. |
| `--output text|json` | Optional | Selects human-readable text or machine-readable JSON output. When omitted, output is `text`. |
| `--dry-run` | Optional | Previews the connect plan under the zero-write dry-run contract. When omitted, the command performs the real connection. Dry-run does not require `--allow-repository-write`. |
| `--allow-repository-write` | Conditionally required authorization | Required for a non-dry-run project-scoped Agent Connection because the command writes host configuration in the `Product Repository`. |
| `--replace-managed` | Optional | Authorizes replacement only where the existing managed-ownership restrictions permit replacement of matching previously managed content. Omission does not authorize replacement. |

Project selection and registration:

- A connection must resolve exactly one selected project.
- For an unregistered project, both `--project-id` and `--repo-root` are required.
- For an already registered project, `--project-id` alone may reuse its registered repository path.
- If `--project-id` and `--repo-root` are both supplied for an already registered project, the supplied repository path must match the registration.
- `--repo-root` alone may select a project only when it uniquely matches one existing executable project registration.
- If a supplied `--repo-root` matches no existing registration, `--project-id` is required so the project can be registered.
- If a supplied `--repo-root` matches more than one existing registration, the user must provide `--project-id`.

Connection rules:

- The command must not connect every project in the Runtime Home.
- Project and local scopes allow one connected project.
- User scope may add more projects later through `volicord agent project add`.
- Host configuration writes use managed ownership markers or an equivalent managed fingerprint.
- Managed host-entry fingerprints use the format identifier `volicord-host-entry-v1`.
- Existing unmanaged configuration for the same host target and server name is a conflict unless `--replace-managed` applies to a previously managed block with a matching ownership marker.
- A non-dry-run project-scoped Agent Connection requires `--allow-repository-write`; dry-run does not.
- `--dry-run` previews every storage and file action under the zero-write contract in [Dry run and machine-readable output](#dry-run).

Verification:

- Verification must attempt MCP preflight, MCP initialization, and `tools/list` discovery when the host can be launched from the installed configuration.
- If configuration is installed but host trust or approval prevents loading, the result is `action_required`, not `failed`.
- If `volicord-mcp --check --connection <connection_id>` passes but MCP initialization or tool discovery has not succeeded, the result cannot be `complete`.
- A direct Volicord-spawned MCP handshake does not prove that Codex or Claude Code loaded, trusted, approved, or exposed the server.

## Connection Project membership commands

`volicord agent project add` adds or restores one connected project for an existing Agent Connection.

Rules:

- `--connection-id` and `--project-id` are required.
- If the project is already registered in the selected Runtime Home with a valid current project registration, the command reuses that registration.
- If the project is not registered, the command can register it when the required `--repo-root` value is supplied, then add the Connection Project record.
- If the project is not registered and the required repository information is absent, the command fails instead of inventing a repository location.
- Adding a project does not make inactive or otherwise execution-ineligible projects available at execution time.
- Adding a second project to a `project` or `local` scoped Agent Connection is a conflict.
- The command does not rewrite host configuration; access revocation and addition are registry changes.

`volicord agent project remove` removes one connected project from an existing Agent Connection.

Rules:

- `--connection-id` and `--project-id` are required.
- Removing membership does not delete project state, Core records, host configuration, or Product Repository guidance files.
- Removing the final Connection Project leaves any remaining Agent Connection record unusable for project-scoped workflow until a project is connected again.

## Status, enablement, and verification commands

`volicord agent list` lists Agent Connections in the selected Runtime Home.

`volicord agent status` reports one Agent Connection without launching the host.

It reports at least:

- `connection_id`
- `host_kind`
- `host_scope`
- `mode`
- enabled state
- connected projects
- `last_verified_status`
- `server_name`
- `config_target`

`volicord agent enable` and `volicord agent disable` toggle the stored enabled state for one Agent Connection. They do not rewrite host configuration and do not make user-owned judgments.

`volicord agent verify` refreshes verification state for one Agent Connection.

Verification must check:

- the Agent Connection exists and is enabled
- connected projects are readable when the verification path needs project context
- host configuration target exists and still matches the managed fingerprint, when direct host configuration owns a target
- `volicord-mcp --check --connection <connection_id>` succeeds
- MCP initialization succeeds
- `tools/list` exposes the tools required by the connection mode

Verification records one of `complete`, `action_required`, or `failed` into the Agent Connection's `last_verified_status`. The command output reports the host check, preflight check, and MCP handshake check when those checks ran.

## Uninstall

`volicord agent uninstall` removes the selected managed Agent Connection host configuration when ownership and safety checks permit removal, removes the corresponding Connection Project records, and removes the Agent Connection record when it is no longer used.

Rules:

- Uninstall must preview managed file edits before applying them.
- It must remove only blocks, files, or entries with matching Volicord ownership markers or managed fingerprints.
- It must not delete a `Product Repository`, project state, Core records, the `Volicord Runtime Home` location itself, artifact storage, or unrelated host configuration.
- Project-scoped file edits require `--allow-repository-write` in noninteractive execution.
- If host files were already changed by the user, uninstall must report the conflict rather than removing unrelated content.

## Product Repository guidance boundary

The current `volicord` administrative CLI exposes no repository-guidance subcommand. Product Repository guidance files, generated host instructions, MCP server instructions, and ordinary repository guidance can help tool selection and workflow consistency, but they are not Core authority records and are not an enforcement mechanism.

Such guidance must not claim to grant access control, security enforcement, User Channel authority, user-owned judgment, evidence, acceptance, close readiness, a `Write Check`, or proof that a model will choose Volicord tools. Exact `Product Repository` write boundaries belong to [Runtime Boundaries](runtime-boundaries.md#explicit-integration-files-in-product-repositories).

<a id="dry-run"></a>
## Dry run and machine-readable output

`--dry-run` performs planning, validation, conflict detection, host target rendering, and output shaping without persistent changes.

Dry-run does not:

- create a `Volicord Runtime Home`
- create or modify SQLite databases
- create SQLite WAL or SHM files
- apply registry or project-state migrations
- register or update projects, Agent Connections, Connection Projects, or verification status rows
- create, modify, or remove host configuration files
- create, modify, or remove `Product Repository` files or directories
- create, modify, or remove generic export files
- invoke `volicord-mcp --check`
- perform MCP initialization or tool discovery

When a selected Runtime Home has a current registry under the current storage profile, dry-run may inspect it without migration and reports no registry migration planned. It must not migrate the registry, create new registry tables, create project-state databases, or write migration metadata. Unsupported registry versions or storage profiles fail without being converted or repaired.

Text output must be human-readable and identify each resource action using `created`, `reused`, `updated`, `removed`, `skipped`, `conflict`, or `planned`.

<a id="setup-output"></a>
When a `volicord agent` command returns the agent result object, JSON output has
these top-level keys:

```text
action
status
connection
verification
```

`volicord agent list --output json` returns one top-level `connections` array
instead of the single-connection result object.

Required JSON values:

- `status`: `complete`, `action_required`, `failed`, `not_verified`, or `dry_run`
- `host_kind`: `codex`, `claude_code`, or `generic`
- `host_scope`: `user`, `project`, `local`, or `export`
- `mode`: `read_only` or `workflow`
- `verification_status`: `not_verified`, `complete`, `action_required`, `failed`, or `dry_run`

JSON output is administrative CLI output, not a public Volicord API response schema.

<a id="noninteractive-approval-behavior"></a>
## Noninteractive approval behavior

Noninteractive commands must fail instead of prompting when explicit user authorization is missing.

Rules:

- `--allow-repository-write` is required for any command that writes, replaces, or removes project-scoped host configuration.
- `--replace-managed` applies only to Volicord-managed content with matching ownership markers or managed fingerprints.
- A broad shell approval, write approval, host trust decision, sensitive-action approval, or Write Check does not substitute for the explicit administrative flag required by this CLI contract.
- Host trust, project trust, project MCP approval, OAuth, restart, or reload actions remain user-controlled host actions and cannot be supplied by the CLI.

## Project registration

`volicord project register --project-id ID --repo-root PATH [--status active]` registers a local `Product Repository` with the selected Runtime Home.

Rules:

- `--project-id` is required.
- `--repo-root` is required.
- `--status` defaults to `active`.
- Baseline registration accepts `status=active`.
- `--repo-root` identifies the local repository root for the project registration.
- The selected Runtime Home and `--repo-root` must satisfy the [Runtime Home/Product Repository separation contract](runtime-boundaries.md#runtime-home-product-repository-separation) before registration is recorded.

`volicord project list` lists current valid registered projects for the selected Runtime Home.

If any selected project registry row is malformed or violates the current registration invariants, `volicord project list` fails through the standard CLI error channel instead of omitting the row or returning it as a normal project. Raw malformed registry content remains diagnosable through the inspection layer.

Runtime location boundaries, including the distinction between `Product Repository` and `Volicord Runtime Home`, are owned by [Runtime Boundaries](runtime-boundaries.md#runtime-home-product-repository-separation).

## Administrative boundary

The administrative CLI can initialize and register local resources. It does not create public Volicord API methods and does not by itself create Core authority, Write Check compatibility, evidence sufficiency, close readiness, user-owned judgment, acceptance, residual-risk acceptance, artifact authority, or security guarantees.

Owner routes:

- Public method list and method routing: [API Methods](api/methods.md).
- Shared request and response schemas: [API Schema Core](api/schema-core.md).
- Agent Connection, Connection Projects, and actor context meaning: [Agent Connection](agent-connection.md).
- MCP process behavior: [MCP Transport](mcp-transport.md).
- Runtime location and repository write boundaries: [Runtime Boundaries](runtime-boundaries.md).
