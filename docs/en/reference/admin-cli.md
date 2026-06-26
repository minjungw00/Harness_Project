# Administrative CLI reference

This document owns the local `volicord` administrative and bootstrap CLI contract. The CLI initializes a `Volicord Runtime Home`, registers local projects and surfaces, provides a verified local `user_interaction` path for selected user-facing Core actions, manages Agent Integration Profiles, installs host configuration for supported coding-agent hosts, and verifies host integration state. These commands are not public Volicord API methods.

It does not define public API method behavior, API schemas, access-class value meanings, storage record layout, security guarantees, Core authority semantics, or MCP stdio transport behavior.

## Owns / does not own

This document owns:

- `volicord` command names, command-line arguments, defaults, stdout/stderr routing, and process exit codes
- Runtime Home path selection for `volicord` administrative commands
- administrative project and surface registration defaults
- local user interaction command names, registered surface defaults, and command output
- Agent Integration Profile command behavior
- integration project membership command behavior
- host installation, status, verification, and uninstall command behavior for Codex, Claude Code, and generic export
- setup result states, dry-run behavior, machine-readable output, and noninteractive approval behavior
- optional repository-guidance apply, status, and remove command behavior
- `baseline-workflow` local registration profile expansion
- the boundary between administrative commands, local user interaction commands, and public Volicord API methods

This document does not own:

- public Volicord API methods; see [API Methods](api/methods.md)
- API value meanings for `access_class` values; see [API Value Sets](api/schema-value-sets.md#access-class-values)
- Agent Integration Profile, Host Installation, verified surface context, actor provenance, and capability declaration meanings; see [Agent Integration](agent-integration.md)
- runtime data boundary meaning and `Product Repository` file-boundary exceptions; see [Runtime Boundaries](runtime-boundaries.md)
- MCP process startup, stdio framing, wire behavior, response wrapping, preflight internals, and shutdown; see [MCP Transport](mcp-transport.md)
- storage record layout, SQLite DDL, general storage migration definitions, Core authority semantics, and security guarantee meanings

## Command model

`volicord` is a local administrative/bootstrap executable. It is not a long-running server. The `volicord user` command group is a local CLI adapter over selected Core methods for a registered `user_interaction` surface; its command names remain administrative CLI commands, not public Volicord API methods.

Supported baseline commands:

```text
volicord --help
volicord --version
volicord init [--runtime-home-id ID]
volicord project register --project-id ID --repo-root PATH [--status active]
volicord project list
volicord surface register --project-id ID --surface-id ID [--surface-instance-id ID] [--kind KIND] [--name NAME] [--interaction-role agent|user_interaction] [--access-class ACCESS_CLASS ...] [--profile baseline-workflow] [--capability-profile JSON]
volicord surface list --project-id ID
volicord user setup --project-id ID [--surface-id ID] [--surface-instance-id ID] [--name NAME]
volicord user status --project-id ID [--task-id ID] [--surface-id ID] [--surface-instance-id ID]
volicord user judgment list --project-id ID [--task-id ID] [--surface-id ID] [--surface-instance-id ID]
volicord user judgment show --project-id ID --judgment-id ID [--surface-id ID] [--surface-instance-id ID]
volicord user judgment record --project-id ID --judgment-id ID --option-id ID [--surface-id ID] [--surface-instance-id ID] [--note TEXT] [--request-id ID] [--idempotency-key KEY] [--expected-state-version VERSION]
volicord agent install --host codex|claude-code|claude_code|generic --scope user|project|local|export [--project-id ID] [--repo-root PATH] [--integration-id ID] [--default-project-id ID] [--server-name NAME] [--surface-id ID] [--surface-instance-id ID] [--mcp-command PATH] [--runtime-home PATH] [--export-path PATH|--export-dir PATH] [--guidance none|codex|claude-code|claude_code|both] [--output text|json] [--dry-run] [--allow-repository-write] [--replace-managed]
volicord agent project add --integration-id ID --project-id ID [--repo-root PATH] [--default] [--runtime-home PATH] [--output text|json] [--dry-run]
volicord agent project remove --integration-id ID --project-id ID [--runtime-home PATH] [--output text|json] [--dry-run]
volicord agent project default set --integration-id ID --project-id ID [--runtime-home PATH] [--output text|json] [--dry-run]
volicord agent project default clear --integration-id ID [--runtime-home PATH] [--output text|json] [--dry-run]
volicord agent status --integration-id ID [--runtime-home PATH] [--output text|json]
volicord agent verify --integration-id ID [--installation-id ID] [--runtime-home PATH] [--output text|json]
volicord agent uninstall --integration-id ID [--installation-id ID] [--runtime-home PATH] [--output text|json] [--dry-run] [--allow-repository-write] [--remove-managed]
volicord agent guidance apply --integration-id ID --project-id ID --host codex|claude-code|claude_code [--runtime-home PATH] [--output text|json] [--dry-run] [--allow-repository-write] [--replace-managed]
volicord agent guidance status --integration-id ID --project-id ID [--runtime-home PATH] [--output text|json]
volicord agent guidance remove --integration-id ID --project-id ID [--host codex|claude-code|claude_code] [--runtime-home PATH] [--output text|json] [--dry-run] [--allow-repository-write] [--remove-managed]
```

Exit and stream behavior:

- Successful commands write success output to stdout and exit with code `0`.
- `action_required` is a successful administrative result and exits `0`.
- `partial_failure`, `failed`, runtime errors, storage errors, preflight failures, verification failures, and conflicts exit `1`.
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

## User interaction commands

`volicord user` commands provide a local CLI path for a human user to inspect task status and answer pending user judgments through a registered `user_interaction` surface. They do not create an Agent Integration Profile, install MCP host configuration, or make an agent surface eligible to act as a user.

`volicord user setup` creates or updates one local `user_interaction` surface for the selected project. When omitted, `--surface-id` defaults to `surface_user_cli`, `--surface-instance-id` defaults to `surface_instance_user_cli`, `--name` defaults to `Local user CLI`, and the surface kind is `cli`. The command grants exactly the local access classes `read_status` and `core_mutation` for that registered surface.

`volicord user status` selects a registered `user_interaction` surface, then shows user-oriented task status through `volicord.status` with `actor_kind=user` and the CLI direct surface-binding verification basis.

`volicord user judgment list` selects a registered `user_interaction` surface, verifies status readability for the active or selected task, and lists pending user judgments with their task, judgment kind, status, and question.

`volicord user judgment show` selects a registered `user_interaction` surface, verifies status readability for the judgment's task, and displays the stored judgment request, context summary, and Core-generated options.

`volicord user judgment record` requires a pending judgment and a `--option-id` that names one of that judgment's stored Core-generated options. It records the selection through `volicord.record_user_judgment` with `actor_kind=user`, the registered `user_interaction` surface role, the CLI direct surface-binding verification basis, and `core_mutation` access. The selected option's `machine_action` and `resolution_outcome` determine the recorded answer; `--note` is stored only as a note. When `--request-id`, `--idempotency-key`, or `--expected-state-version` are omitted, the command supplies a local request id, a local idempotency key, and the current project state version. Agent-role surfaces are not eligible for `volicord user judgment record`.

Recording one judgment records only the addressed judgment. Final acceptance and residual-risk acceptance remain separate judgment kinds and actions; this command must not collapse one into the other.

## Host and scope support

Supported host and scope values:

| `--host` | Supported `--scope` values | Baseline target |
|---|---|---|
| `codex` | `user`, `project` | User config is Codex user `config.toml`. Project config is `.codex/config.toml` in the associated `Product Repository`. |
| `claude_code` | `local`, `project`, `user` | Local and user config are Claude Code user-owned configuration targets. Project config is `.mcp.json` in the associated `Product Repository`. The CLI may accept `claude-code` as an alias, but stored records use `claude_code`. |
| `generic` | `export` | Export an explicit MCP configuration object without claiming direct installation. |

Scope rules:

- `project` and `local` scopes permit exactly the associated `Product Repository`.
- `user` scope may permit multiple explicitly added projects, but each `volicord agent install` invocation still selects exactly one project by the install project-selection rules.
- `generic export` writes or prints only an explicit configuration export and does not create a Host Installation that claims host loading.
- Unsupported host/scope combinations are usage errors.

Host configuration shape:

- Codex installation writes an MCP server table equivalent to `[mcp_servers.<server_name>]` with `command`, `args = ["--integration", "<integration_id>"]`, and optional `env.VOLICORD_HOME` when the selected host scope permits persisting the selected Runtime Home path.
- Claude Code installation writes an MCP server entry under `mcpServers.<server_name>` with `command`, `args`, and optional `env.VOLICORD_HOME` when the selected host scope permits persisting the selected Runtime Home path.
- Generic export emits the same command, args, and environment values in a host-neutral JSON object.
- When generic export writes to an export directory instead of an explicit `--export-path`, the generated file name is `volicord-<integration>.mcp.json`.
- User and local scopes may use a discovered canonical `volicord-mcp` executable path or an explicit valid absolute path.
- Project-scoped shared configuration must use the portable command `volicord-mcp` and rely on `PATH` in the host environment. It must not embed a personal build path, home-directory path, or personal `VOLICORD_HOME`.
- Generic export may emit an explicitly selected absolute command path, but exported configuration is still `action_required` until a user-managed host loads and verifies it.
- New baseline host configuration must not require `VOLICORD_PROJECT_ID`, `VOLICORD_SURFACE_ID`, or `VOLICORD_SURFACE_INSTANCE_ID`.

Host trust boundary:

- Installing configuration is distinct from the host loading and exposing the MCP server.
- Codex project-scoped configuration may require Codex project trust before it loads.
- Claude Code project-scoped MCP configuration may require project MCP approval before it loads.
- Volicord must not claim that host trust, project trust, project MCP approval, OAuth, or comparable user-controlled host actions can be bypassed.

## Agent setup result states

The agent command family uses these setup result states:

| State | Meaning |
|---|---|
| `complete` | Durable integration state exists, managed host configuration exists and matches its expected fingerprint, the host-specific loadability gate is satisfied, no required trust or approval action remains, integration preflight succeeds, MCP initialization succeeds, and `tools/list` succeeds with the required tools. |
| `action_required` | Durable integration state and host configuration are present, but host trust, project approval, OAuth, reload, restart, or a comparable user-controlled host action remains. |
| `partial_failure` | Some durable administrative action succeeded, but a later installation, verification, host target, rollback, or cleanup step failed. The result must identify applied, rolled-back, and residual effects and be rerunnable. |
| `failed` | The requested installation or verification did not establish usable durable integration state or host configuration. |

`dry_run` is an output status, not a setup result state.

A successful `volicord-mcp --check --integration <integration_id>` alone must not be described as `complete` host integration. It is only startup validation for the MCP process.

Host-specific state rules:

- Codex project scope remains `action_required` while Codex project trust cannot be confirmed.
- Claude Code project scope remains `action_required` while project MCP approval is pending.
- Rejected, missing, changed, unavailable, and unknown host states must not become `complete`.
- Generic export remains `action_required` because Volicord cannot prove that an external host loaded the exported configuration.

## `volicord agent install`

`volicord agent install` creates or reuses an Agent Integration Profile, explicitly allows one selected project, installs or exports host configuration, and verifies the result where the host can be checked.

Argument requiredness and omission behavior:

| Argument | Requiredness | Meaning, applicability, and omission behavior |
|---|---|---|
| `--host codex|claude-code|claude_code|generic` | Always required | Selects the host adapter. The value must be valid with the selected `--scope`; `claude-code` is accepted as an alias for `claude_code`. |
| `--scope user|project|local|export` | Always required | Selects the host configuration or export target. The value must be valid with the selected `--host`. |
| `--project-id ID` | Conditionally required | Names the selected project. It is required for an unregistered project, required when `--repo-root` matches no existing registration, and required when a supplied `--repo-root` is ambiguous. It may be omitted only when `--repo-root` uniquely matches one existing executable project registration. |
| `--repo-root PATH` | Conditionally required | Identifies the selected project's `Product Repository` for project selection and registration. It is required with `--project-id` when that project is not already registered. When omitted for an already registered `--project-id`, the command reuses the registered repository path. |
| `--integration-id ID` | Optional | Selects an existing integration or the desired id for a new integration. When omitted, the command derives a stable deterministic integration id from the selected host, scope, and project. |
| `--default-project-id ID` | Optional | Selects the integration default project and must name an allowed integration project. For a new integration, omission uses the selected project. For an existing integration, omission retains its existing default when present, otherwise uses the selected project. |
| `--server-name NAME` | Optional | Selects the host MCP server name. When omitted, the command derives a stable server name from the integration id using the `volicord-<integration>` form. |
| `--surface-id ID` | Optional | Selects the integration surface id. When omitted, the command reuses an existing integration value when available, otherwise generates a stable identifier. |
| `--surface-instance-id ID` | Optional | Selects the integration surface instance id. When omitted, the command reuses an existing integration value when available, otherwise generates a stable identifier. |
| `--mcp-command PATH` | Optional | Selects the `volicord-mcp` executable where an explicit command is allowed. Project scope defaults to the portable `volicord-mcp` command and must keep that portable command. User, local, and export scopes discover an executable from the current `volicord` executable's sibling directory and then `PATH` when omitted; an explicit command for those scopes must satisfy the existing absolute executable-path rules. |
| `--runtime-home PATH` | Optional | Selects the `Volicord Runtime Home` used by the administrative command. When omitted, the command uses the Runtime Home resolution order above. For non-project host scopes, the selected Runtime Home may be persisted in managed host configuration as `VOLICORD_HOME`. For project scope, shared host configuration must not embed a developer-specific Runtime Home path; a project-scoped host process that must use a non-default Runtime Home must receive `VOLICORD_HOME` through its actual execution environment. An environment variable set only for the administrative installation command is not automatically inherited by future host processes. |
| `--export-path PATH` | Optional | For `generic` `export`, selects an explicit output path for the exported MCP configuration. When omitted, the export path is derived from `--export-dir` or the current working directory. |
| `--export-dir PATH` | Optional | For `generic` `export`, selects the directory used with the generated `volicord-<integration>.mcp.json` file name when `--export-path` is omitted. When neither export destination is supplied, the command uses the current working directory and derives that file name. |
| `--guidance none|codex|claude-code|claude_code|both` | Optional | Previews and applies optional `Product Repository` guidance for the selected project. Omitted or `none` writes no guidance. Non-dry-run guidance writes require `--allow-repository-write`. |
| `--output text|json` | Optional | Selects human-readable text or machine-readable JSON output. When omitted, output is `text`. |
| `--dry-run` | Optional | Previews the install plan under the zero-write dry-run contract. When omitted, the command performs the real installation. Dry-run does not require `--allow-repository-write`. |
| `--allow-repository-write` | Conditionally required authorization | Required for a non-dry-run project-scoped install and for a non-dry-run install that applies repository guidance. Omission is accepted only when no applicable non-dry-run repository write requires it. |
| `--replace-managed` | Optional | Authorizes replacement only where the existing managed-ownership restrictions permit replacement of matching previously managed content. Omission does not authorize replacement. |

Project selection and registration:

- An install must resolve exactly one selected project.
- For an unregistered project, both `--project-id` and `--repo-root` are required.
- For an already registered project, `--project-id` alone may reuse its registered repository path.
- If `--project-id` and `--repo-root` are both supplied for an already registered project, the supplied repository path must match the registration.
- `--repo-root` alone may select a project only when it uniquely matches one existing executable project registration.
- If a supplied `--repo-root` matches no existing registration, `--project-id` is required so the project can be registered.
- If a supplied `--repo-root` matches more than one existing registration, the user must provide `--project-id`.

Installation rules:

- The command must not grant access to every project in the Runtime Home.
- The command must register, reuse, or validate the integration surface for each allowed project before verification can be `complete`.
- A default project must be allowed.
- Project/local scopes fail if more than one project would be allowed.
- User scope may add more projects later through `volicord agent project add`.
- Host configuration writes use managed ownership markers or an equivalent managed fingerprint.
- Managed host-entry fingerprints use the format identifier `volicord-host-entry-v1`.
- Existing unmanaged configuration for the same host target and server name is a conflict unless `--replace-managed` applies to a previously managed block with a matching ownership marker.
- A non-dry-run project-scoped install requires `--allow-repository-write`; dry-run does not.
- A non-dry-run install that applies repository guidance requires `--allow-repository-write`; dry-run does not.
- `--dry-run` previews every storage and file action under the zero-write contract in [Dry run and machine-readable output](#dry-run).

Verification:

- Verification must attempt MCP initialization and `tools/list` discovery when the host can be launched from the installed configuration.
- If configuration is installed but host trust or approval prevents loading, the result is `action_required`, not `failed`.
- If `volicord-mcp --check` passes but MCP initialization or tool discovery has not succeeded, the result cannot be `complete`.
- A direct Volicord-spawned MCP handshake does not prove that Codex or Claude Code loaded, trusted, approved, or exposed the server.

## Integration project membership commands

`volicord agent project add` adds or restores one allowed project for an existing integration.

Rules:

- `--integration-id` and `--project-id` are required.
- If the project is already registered in the selected Runtime Home with a valid current project registration, the command reuses that registration.
- If the project is not registered, the command can register it when the required `--repo-root` value is supplied, then add the integration membership and required project-facing state.
- If the project is not registered and the required repository information is absent, the command fails instead of inventing a repository location.
- Adding a project does not make inactive or otherwise execution-ineligible projects available at execution time.
- `--default` sets the integration default to the added project.
- Adding a second project to a `project` or `local` scoped integration is a conflict.
- The command does not rewrite host configuration; access revocation and addition are registry changes.

`volicord agent project remove` removes one allowed project from an existing integration.

Rules:

- Removing a project that is still `default_project_id` must fail until the default is cleared or changed.
- Removing the only project from an installed integration is allowed only when the command reports the integration as not executable until a project is added again.
- Removing membership does not delete project state, surface records, Core records, host configuration, or guidance files.

`volicord agent project default set` sets the default project for an existing integration.

Rules:

- `--integration-id` and `--project-id` are required.
- The project must already be allowed for that integration.
- Repeating a set operation for the current default is idempotent.
- Setting a different already allowed project changes the default without rewriting host configuration.

`volicord agent project default clear` clears the default project for an existing integration.

Rules:

- `--integration-id` is required.
- Clearing an already absent default is idempotent.
- A current default project cannot be removed until the default is changed or cleared.
- After the default is cleared, the final project membership may be removed.
- An integration with no allowed projects may remain stored, but it is not executable until a project is added again.

## Status and verification commands

`volicord agent status` reports registry and host-inventory state without launching the host unless a host owner defines a cheap status check.

It reports at least:

- `integration_id`
- enabled state
- `surface_id`
- `surface_instance_id`
- allowed projects with availability and default status
- Host Installation records
- `last_verified_status`
- guidance status

`volicord agent verify` refreshes verification state for one integration or one installation.

Selection rules:

- `volicord agent verify --installation-id <id>` verifies exactly that Host Installation and fails if it belongs to another integration.
- Without `--installation-id`, verification selects every Host Installation associated with `--integration-id`.
- Each selected installation uses its own `host_kind`, `host_scope`, `config_target`, repository root, command, arguments, environment, managed fingerprint, and host-specific status checks.
- One installation's result must not overwrite another installation's verification state. Per-installation output must identify the `installation_id` and resulting `last_verified_status`.

Verification must check:

- integration exists and is enabled
- allowed projects are readable and classified as available or unavailable
- default project, when present, is allowed and available
- host configuration target exists and still matches the managed fingerprint, when direct installation owns a target
- `volicord-mcp --check --integration <integration_id>` succeeds
- MCP initialization succeeds
- `tools/list` exposes the nine public Volicord tools and `volicord.list_projects`

Verification records one of `complete`, `action_required`, `partial_failure`, or `failed` into each selected Host Installation's `last_verified_status`.

Aggregate result status:

| Selected installation results | Aggregate command status |
|---|---|
| Every selected installation is `complete` | `complete` |
| At least one selected installation is `action_required`, and none is `partial_failure` or `failed` | `action_required` |
| At least one selected installation is `partial_failure`, and none is `failed` | `partial_failure` |
| At least one selected installation is `failed` | `failed` |

The aggregate status is never `complete` while any selected installation is not `complete`.

## Uninstall

`volicord agent uninstall` removes the selected managed Host Installation or installations. A successful uninstall removes matching managed host configuration when ownership and safety checks permit removal, removes each corresponding Host Installation inventory record, and may disable the Agent Integration Profile when no Host Installations remain for that profile.

Rules:

- Uninstall must preview managed file edits before applying them.
- It must remove only blocks, files, or entries with matching Volicord ownership markers or managed fingerprints.
- Each Host Installation inventory record for the selected managed installation or installations is removed when uninstall completes successfully.
- If no Host Installations remain for the Agent Integration Profile after successful removal, the integration may be disabled. Disabling an Agent Integration Profile does not delete it.
- It must not delete a `Product Repository`, project state, Core records, the `Volicord Runtime Home` location itself, artifact storage, or unrelated host configuration.
- Project-scoped file edits require `--allow-repository-write` in noninteractive execution.
- `--remove-managed` is required for noninteractive removal of managed `Product Repository` guidance.
- If host files were already changed by the user, uninstall must report the conflict rather than removing unrelated content.

## Repository guidance commands

Repository guidance is optional. It is installed only after explicit user authorization and is not an enforcement mechanism.

Supported guidance targets:

- Codex: a Volicord-managed block in `AGENTS.md`.
- Claude Code: a Volicord-managed Markdown rule file at `.claude/rules/volicord.md`.

Rules:

- `volicord agent guidance apply` requires `--integration-id`, `--project-id`, `--host`, and `--allow-repository-write` in noninteractive execution.
- The command must preview the exact file path and managed content.
- The command must detect unmanaged conflicts and require `--replace-managed` only for matching previously managed content.
- Managed guidance must include ownership markers that identify Volicord management and the integration.
- Managed repository-guidance fingerprints use the format identifier `volicord-repository-guidance-v1`.
- Codex managed guidance blocks use the exact markers `<!-- BEGIN VOLICORD MANAGED GUIDANCE v1 -->` and `<!-- END VOLICORD MANAGED GUIDANCE v1 -->`.
- `volicord agent guidance status` reads managed marker state and reports whether guidance is absent, present, changed, or conflicted.
- `volicord agent guidance remove` removes only matching managed content and requires `--remove-managed` in noninteractive execution.
- Guidance must state that Volicord MCP server instructions and repository guidance can help tool selection but cannot guarantee model behavior.

Exact `Product Repository` write boundaries belong to [Runtime Boundaries](runtime-boundaries.md#explicit-integration-files-in-product-repositories).

<a id="dry-run"></a>
## Dry run and machine-readable output

`--dry-run` performs planning, validation, conflict detection, host target rendering, and output shaping without persistent changes.

Dry-run does not:

- create a `Volicord Runtime Home`
- create or modify SQLite databases
- create SQLite WAL or SHM files
- apply registry or project-state migrations
- register or update projects, surfaces, integrations, memberships, installations, or verification status rows
- create, modify, or remove host configuration files
- create, modify, or remove `Product Repository` files or directories, including guidance files
- create, modify, or remove generic export files
- invoke `volicord-mcp --check`
- perform MCP initialization or tool discovery

When a selected Runtime Home has a current registry under the current storage profile, dry-run may inspect it without migration and reports no registry migration planned. It must not migrate the registry, create new registry tables, create project-state databases, or write migration metadata. Unsupported registry versions or storage profiles fail without being converted or repaired.

Text output must be human-readable and identify each resource action using `created`, `reused`, `updated`, `removed`, `skipped`, `conflict`, or `planned`.

<a id="setup-output"></a>
When a `volicord agent` command returns the agent result object, JSON output has
these top-level keys:

```text
status
runtime
project
integration
allowed_projects
installations
installation_verifications
guidance
host
verification
actions
effects
residual_effects
action_required
warnings
```

The top-level `effects` array records relevant planned, applied, reused,
compensated, or rollback outcomes according to the implemented result model.
The top-level `residual_effects` array is always emitted. It is empty when no
residual effects are known. When populated, each entry records a known
persistent or external effect that remains after a failed or partially
compensated operation and requires operator awareness or action. Residual
effect entries include `component`, `target`, `current_state`, `reason`, and
`recommended_action`.

Required JSON values:

- `status`: `complete`, `action_required`, `partial_failure`, `failed`, or `dry_run`
- `host_kind`: `codex`, `claude_code`, or `generic`
- `host_scope`: `user`, `project`, `local`, or `export`
- `last_verified_status`: `not_verified`, `complete`, `action_required`, `partial_failure`, or `failed`

JSON output is administrative CLI output, not a public Volicord API response schema.

Partial-failure output:

- Human-readable text output must identify each applied effect, rolled-back effect, and residual effect.
- JSON output must expose the same facts in machine-readable entries.
- JSON residual-effect entries appear in the top-level `residual_effects` array. They are not nested under `effects`, `warnings`, `verification`, or `action_required`, and `effects` and `residual_effects` are not interchangeable.
- Each effect entry must include the target location or record identity, the effect classification, and enough detail to rerun or inspect the target.
- Each residual effect must include why rollback was not performed or why rollback failed, plus the recommended operator action.
- When `partial_failure` is caused by incomplete compensation, populated `residual_effects` entries identify the remaining effects. A non-`complete` result does not by itself imply that residual effects exist; when none are known, `residual_effects` is an empty array.
- A generic statement such as `registry changes may remain` is insufficient unless paired with exact residual-effect entries.

<a id="noninteractive-approval-behavior"></a>
## Noninteractive approval behavior

Noninteractive commands must fail instead of prompting when explicit user authorization is missing.

Rules:

- `--allow-repository-write` is required for any command that writes, replaces, or removes project-scoped host configuration or repository guidance.
- `--replace-managed` applies only to Volicord-managed content with matching ownership markers or managed fingerprints.
- `--remove-managed` applies only to safe removal of Volicord-managed content.
- A broad shell approval, write approval, host trust decision, or sensitive-action approval is not a `Write Authorization` and does not substitute for the explicit administrative flag required by this CLI contract.
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

## Surface registration

`volicord surface register` records one local surface instance for a registered project.

Surface registration and listing require the project registration to remain eligible under the Runtime Home/Product Repository separation contract owned by [Runtime Boundaries](runtime-boundaries.md#runtime-home-product-repository-separation).

Defaults:

- `surface_kind` defaults to `cli`.
- `interaction_role` defaults to `agent`.
- Default access is only `read_status`.
- Generated Runtime Home IDs and generated `surface_instance_id` values are implementation-generated opaque values.

Registration profile:

- `--profile baseline-workflow` must be explicitly selected.
- `baseline-workflow` expands to `read_status`, `core_mutation`, `write_authorization`, `artifact_registration`, and `run_recording`.
- Explicit and profile-derived access classes form a deterministic de-duplicated union.
- The `baseline-workflow` profile does not include `artifact_read`.

`user_interaction` constraints:

- `user_interaction` requires `core_mutation`.
- `user_interaction` may have only `read_status` and `core_mutation`.
- `baseline-workflow` is therefore invalid for a `user_interaction` surface.

MCP registration guidance:

- For a coding-agent MCP integration, prefer `volicord agent install` because it creates or validates the integration profile, project membership, host installation, and per-project surface binding together.
- Low-level `volicord surface register --kind mcp` remains available for explicit administrative repair or custom automation.

Access-class value names and meanings are owned by [API Value Sets](api/schema-value-sets.md#access-class-values). Surface registration meaning and verified context boundaries are owned by [Agent Integration](agent-integration.md).

## Surface listing

`volicord surface list --project-id ID` lists registered surfaces for one project in the selected Runtime Home.

Rules:

- `--project-id` is required.
- Listing output is diagnostic registration information.
- Listing output does not grant authority, prove local reachability, or replace owner-returned verified surface context.

## Administrative boundary

The administrative CLI can initialize and register local resources. It does not create public Volicord API methods and does not by itself create Core authority, `Write Authorization`, evidence sufficiency, close readiness, user-owned judgment, acceptance, residual-risk acceptance, artifact authority, or security guarantees.

Owner routes:

- Public method list and method routing: [API Methods](api/methods.md).
- Shared request and response schemas: [API Schema Core](api/schema-core.md).
- Access-class values: [API Value Sets](api/schema-value-sets.md#access-class-values).
- Agent Integration Profile, project membership, surface and actor context meaning: [Agent Integration](agent-integration.md).
- MCP process behavior: [MCP Transport](mcp-transport.md).
- Runtime location and repository write boundaries: [Runtime Boundaries](runtime-boundaries.md).
