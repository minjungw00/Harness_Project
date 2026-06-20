# Local MCP setup

Use this guide when you want the ordinary local MCP setup path for one `Product Repository`.

`harness` performs local setup. `harness-mcp` is the child process that an MCP host launches after setup. The local MCP process communicates over stdio, not a network port, URL, socket, or listener. The setup command generates host-neutral configuration that you copy into the appropriate host-specific configuration; it does not install, discover, or edit an unknown external host.

This page owns the task-oriented onboarding order only. Exact `harness` command behavior belongs to [Administrative CLI](../reference/admin-cli.md#local-mcp-setup-orchestration). Exact `harness-mcp` process behavior, stdio framing, response wrapping, preflight, shutdown, and reconnection belong to [MCP Transport](../reference/mcp-transport.md). Runtime location boundaries belong to [Runtime Boundaries](../reference/runtime-boundaries.md). Surface role and actor-provenance boundaries belong to [Agent Integration](../reference/agent-integration.md).

## 1. Build the executables

From the Harness repository root:

```sh
cargo build --release -p harness-cli -p harness-mcp
```

Expected release executables:

- `target/release/harness`
- `target/release/harness-mcp`

Option-free setup can discover `harness-mcp` when either:

- `harness-mcp` is beside `harness`, as in the release build output
- `harness-mcp` is on `PATH`

Use `--mcp-command` only when you need to point setup at a specific MCP executable.

## 2. Run the common setup path

From the product repository you want to bind:

```sh
cd /absolute/path/to/product-repository
/absolute/path/to/harness setup local-mcp
```

The option-free path uses these guide-level defaults:

- the current directory is the `Product Repository`
- the Runtime Home uses the normal shared selection rules
- an existing active project with the exact repository path is reused
- otherwise the project ID is derived from the final directory name
- the agent MCP surface uses the baseline workflow profile
- user interaction is not created
- preflight runs automatically after registration
- host-neutral agent configuration is printed

Before storage mutation, setup rejects invalid project IDs and structurally impossible configuration output paths. A real non-dry-run setup may initialize or migrate a recognized existing Runtime Home after setup begins. Runtime failures, permission changes, storage exhaustion, operating-system errors, and MCP preflight failures can still occur after storage preparation or registration; setup reports completed actions where relevant and remains rerunnable.

For exact defaults, conflict behavior, exit codes, and stream behavior, use [Administrative CLI](../reference/admin-cli.md#local-mcp-setup-orchestration).

## 3. Use explicit paths when needed

Operators who need fixed locations can provide them directly:

```sh
/absolute/path/to/harness setup local-mcp \
  --runtime-home /absolute/path/to/harness-runtime-home \
  --repo-root /absolute/path/to/product-repository \
  --project-id demo \
  --mcp-command /absolute/path/to/harness-mcp
```

Route detailed option behavior to [Administrative CLI](../reference/admin-cli.md#local-mcp-setup-orchestration).

## 4. Read the setup result

A successful text result includes important lines like these. Treat them as human-readable command output, not as a public API schema:

```text
setup: complete
runtime_home: ...
project_id: ...
agent_surface_id: agent_mcp
agent_surface_instance_id: agent_mcp_local
mcp_command: ...
preflight: passed
```

The actions section identifies what setup did:

- `created` means setup added a missing Runtime Home, project, or surface record.
- `reused` means an existing compatible record was kept.
- `updated` means setup replaced a target surface only through the explicit conflict-handling path owned by the Administrative CLI.

The printed `agent_config_json` is the host-neutral fragment for the ordinary agent process:

```json
{
  "mcpServers": {
    "harness-agent": {
      "command": "/absolute/path/to/harness-mcp",
      "env": {
        "HARNESS_HOME": "/absolute/path/to/runtime-home",
        "HARNESS_PROJECT_ID": "project-id",
        "HARNESS_SURFACE_ID": "agent_mcp",
        "HARNESS_SURFACE_INSTANCE_ID": "agent_mcp_local"
      }
    }
  }
}
```

Place that fragment in the configuration location and wrapper shape used by the MCP host you operate. Do not configure a URL, TCP port, HTTP endpoint, or socket path for the baseline local MCP process.

## 5. Optional user-interaction connector setup

Only add the user-interaction binding when a real user-facing UI or connector will submit user actions:

```sh
harness setup local-mcp --with-user-interaction
```

This creates and preflights a separate `user_interaction` surface, then prints a separate `harness-user-interaction` configuration. It does not merge that binding into the ordinary agent configuration.

The user-interaction configuration is for the appropriate UI or connector, not for the ordinary agent host:

```json
{
  "mcpServers": {
    "harness-user-interaction": {
      "command": "/absolute/path/to/harness-mcp",
      "env": {
        "HARNESS_HOME": "/absolute/path/to/runtime-home",
        "HARNESS_PROJECT_ID": "project-id",
        "HARNESS_SURFACE_ID": "user_ui",
        "HARNESS_SURFACE_INSTANCE_ID": "user_ui_local"
      }
    }
  }
}
```

`actor_kind=user` alone is insufficient. Authority-bearing user actions require the appropriate user-facing UI or connector to use the separate `user_interaction` binding. Exact actor-provenance rules are in [Agent Integration](../reference/agent-integration.md#current-surface-context).

## 6. Write configuration files

To write host-neutral fragments to a directory:

```sh
harness setup local-mcp --config-dir /absolute/path/to/generated-mcp-config
```

Expected files:

```text
harness-agent.mcp.json
harness-user-interaction.mcp.json
```

`harness-user-interaction.mcp.json` exists only when `--with-user-interaction` is requested.

Existing files are not overwritten by default. Use `--overwrite-config` only when you intentionally want setup to replace generated files in the selected configuration directory. These files are host-neutral fragments; setup does not guess an external host's settings location.

Setup validates whether the configuration directory and all requested target files are structurally usable before storage mutation. Dry-run performs the same structural checks without creating the directory or files. Detailed path rules are in [Administrative CLI](../reference/admin-cli.md#host-neutral-configuration).

## 7. Preview or automate setup

Preview setup:

```sh
harness setup local-mcp --dry-run
```

Dry run performs path resolution, planning, executable discovery, configuration rendering, and conflict analysis. It performs no registration, preflight, Runtime Home creation, database writes, migration, or configuration-file writes.

Dry-run inspection is read-only and safe for preview. If the repository directory name cannot become a valid project ID, rerun with an explicit valid `--project-id`.

For automation, request JSON output:

```sh
harness setup local-mcp --output json
```

JSON mode emits one machine-readable success object on stdout. Automation should use JSON instead of parsing human-readable text. Errors continue to use stderr and process status. JSON output is administrative CLI output, not a public Harness API response schema.

## 8. Optional interactive wizard

Use the wizard when you want setup to prompt for the same inputs:

```sh
harness setup local-mcp --interactive
```

The wizard is optional and requires a terminal. It displays the agent binding and access classes, defaults the user-interaction connector to no, requires explicit confirmation for destructive replacement and configuration overwrite, and creates no persistent setup writes when final confirmation is cancelled. It uses the same setup engine as the non-interactive command. The exact prompt behavior stays in [Administrative CLI](../reference/admin-cli.md#local-mcp-setup-orchestration).

Before final confirmation, the wizard uses read-only planning. You can cancel, decline a conflicting-surface replacement, decline configuration overwrite, or decline the final plan without Runtime Home initialization, storage migration, preflight, registration, or configuration-file creation. After final confirmation, real setup may migrate a recognized existing Runtime Home.

## 9. Verify connection and tool discovery

After the host launches the agent process, verify this MCP sequence:

1. Send `initialize`.
2. Send `notifications/initialized`.
3. Send `tools/list`.
4. Call `harness.status`.

Expected observations:

- `initialize` returns `serverInfo.name` as `harness-mcp`.
- `tools/list` exposes exactly nine public Harness tools.
- `harness.status` returns MCP text content whose `result.content[0].text` contains serialized Harness JSON.
- Clients parse `result.content[0].text` and inspect `base.response_kind` and `errors`.
- `isError: false` means MCP transport success; it does not prove Harness domain acceptance.

The exact public method list is owned by [API Methods](../reference/api/methods.md). Exact MCP wire behavior and response wrapping are owned by [MCP Transport](../reference/mcp-transport.md).

A raw stdio smoke test uses one JSON value per line:

```text
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"harness-quickstart","version":"0.0.0"}}}
{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"harness.status","arguments":{"envelope":{"project_id":"demo","task_id":null,"actor_kind":"agent","surface_id":"agent_mcp","request_id":"req_quickstart_status","idempotency_key":null,"expected_state_version":null,"dry_run":false,"locale":"en-US"},"include":{"task":true,"pending_user_judgments":true,"write_authority":false,"evidence":false,"close":true,"guarantees":true}}}}
```

## 10. Stop and reconnect

An MCP host stops a local session by closing stdin or terminating the child process. Stdin EOF ends the stdio session after stdout is flushed.

SQLite state remains in the Runtime Home. Starting a new `harness-mcp` child process with the same `HARNESS_HOME`, `HARNESS_PROJECT_ID`, `HARNESS_SURFACE_ID`, and `HARNESS_SURFACE_INSTANCE_ID` reconnects to the same stored project state. Changing project, surface, or surface instance requires another process.

## Advanced manual setup and recovery

Use the lower-level commands when you need custom IDs, need to diagnose conflicts, repair a partial setup, inspect registration, or support automation that cannot use `harness setup local-mcp`.

Initialize the Runtime Home:

```sh
HARNESS_HOME=/absolute/path/to/harness-runtime-home \
/absolute/path/to/harness init
```

Register a product repository:

```sh
HARNESS_HOME=/absolute/path/to/harness-runtime-home \
/absolute/path/to/harness project register \
  --project-id demo \
  --repo-root /absolute/path/to/product-repository
```

Register the agent MCP surface:

```sh
HARNESS_HOME=/absolute/path/to/harness-runtime-home \
/absolute/path/to/harness surface register \
  --project-id demo \
  --surface-id agent_mcp \
  --surface-instance-id agent_mcp_local \
  --kind mcp \
  --interaction-role agent \
  --profile baseline-workflow
```

Optionally register a separate user-interaction surface:

```sh
HARNESS_HOME=/absolute/path/to/harness-runtime-home \
/absolute/path/to/harness surface register \
  --project-id demo \
  --surface-id user_ui \
  --surface-instance-id user_ui_local \
  --kind mcp \
  --interaction-role user_interaction \
  --access-class read_status \
  --access-class core_mutation
```

Inspect registration:

```sh
HARNESS_HOME=/absolute/path/to/harness-runtime-home \
/absolute/path/to/harness surface list --project-id demo
```

Run a direct MCP preflight:

```sh
HARNESS_HOME=/absolute/path/to/harness-runtime-home \
HARNESS_PROJECT_ID=demo \
HARNESS_SURFACE_ID=agent_mcp \
HARNESS_SURFACE_INSTANCE_ID=agent_mcp_local \
/absolute/path/to/harness-mcp --check
```

For the agent binding, expect `configuration: valid`, `transport: stdio`, `interaction_role: agent`, and `baseline_workflow_access: full`. Detailed startup validation and failure conditions stay in [MCP Transport](../reference/mcp-transport.md#configuration-preflight).

## Troubleshooting

| Symptom | Likely cause | Next action |
|---|---|---|
| `harness-mcp` is not found. | It is not beside `harness` and is not on `PATH`. | Build both release executables, add `harness-mcp` to `PATH`, or pass `--mcp-command /absolute/path/to/harness-mcp`. |
| The repository directory has no usable derived ID. | Setup cannot derive a valid path-component project ID from the final directory name. | Re-run with an explicit valid `--project-id`. |
| Multiple projects match the same repository. | More than one registered project points to the canonical repository path. | Re-run with the intended `--project-id`, or inspect registration with Administrative CLI commands. |
| The project ID points to another repository. | The selected `--project-id` is already registered for a different `repo_root`. | Choose the correct project ID or repository; setup does not rebind project IDs. |
| The existing agent surface is incompatible. | A target surface exists with a different role, kind, access set, or MCP startup metadata. | Inspect `harness surface list`; use Administrative CLI conflict handling only when replacing that target surface is intentional. |
| A generated file already exists. | `--config-dir` points at an existing generated fragment. | Choose another directory or re-run with `--overwrite-config` when replacement is intentional. |
| Preflight fails after registration. | Registration succeeded but `harness-mcp --check` rejected the binding or environment. | Read the preflight diagnostic, fix the binding or executable path, and rerun setup. Setup is designed to be safely rerunnable. |
| The agent is unexpectedly read-only. | The agent surface was created manually without the baseline workflow profile or equivalent access set. | Re-run setup or register the agent surface with `--profile baseline-workflow`. |
| The user-interaction connector is not configured. | The common setup path creates only the agent binding. | Re-run with `--with-user-interaction` and connect the separate `harness-user-interaction` fragment through the real UI or connector. |
| JSON-RPC success is confused with Harness acceptance. | The client checked only `isError`. | Parse `result.content[0].text` and inspect `base.response_kind` plus `errors`. |

Route exact setup behavior to [Administrative CLI](../reference/admin-cli.md#local-mcp-setup-orchestration) and exact MCP process behavior to [MCP Transport](../reference/mcp-transport.md).
