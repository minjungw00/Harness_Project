# Local MCP quickstart

This guide lets an operator build the local executables, initialize a `Harness Runtime Home`, register one `Product Repository`, bind two MCP surfaces, preflight the bindings, connect an MCP host, verify tool discovery, interpret `tools/call` responses, and reconnect the process.

This page owns only the executable local setup sequence. Exact `harness` command behavior belongs to [Administrative CLI](../reference/admin-cli.md). Exact `harness-mcp` process behavior, stdio framing, response wrapping, preflight, shutdown, and reconnection belong to [MCP Transport](../reference/mcp-transport.md). Runtime location boundaries belong to [Runtime Boundaries](../reference/runtime-boundaries.md). Surface role and actor-provenance boundaries belong to [Agent Integration](../reference/agent-integration.md).

## Before you start

Use one coherent example and replace the placeholders with absolute paths:

```text
HARNESS_HOME=/absolute/path/to/harness-runtime-home
PRODUCT_REPO=/absolute/path/to/product-repository
HARNESS_BIN=/absolute/path/to/Harness_Project/target/release/harness
HARNESS_MCP_BIN=/absolute/path/to/Harness_Project/target/release/harness-mcp
```

The command examples below use POSIX-like shell variable and environment syntax to keep the flow readable. Other shells use different syntax, and some platforms add executable suffixes such as `.exe`. Keep the same command names, arguments, and environment meanings.

`PRODUCT_REPO` must already be an accessible directory. `HARNESS_HOME` should be an absolute Runtime Home path so the administrative CLI and MCP child processes use the same local runtime data location. Exact path-selection rules are in [Administrative CLI](../reference/admin-cli.md#runtime-home-selection).

## 1. Build the executables

From the repository root:

```sh
cargo build --release -p harness-cli -p harness-mcp
```

Expected release executables:

- `target/release/harness`
- `target/release/harness-mcp`

`harness` performs local administrative setup. `harness-mcp` is the executable an MCP host launches as a child process. `harness-mcp` uses stdio; no network port or URL is involved.

## 2. Initialize the Runtime Home

Check the administrative executable, then initialize the selected Runtime Home:

```sh
"$HARNESS_BIN" --version
HARNESS_HOME="$HARNESS_HOME" "$HARNESS_BIN" init
```

`harness init` creates or validates the selected Runtime Home registry. At a high level, the command output includes a `registry_db` path ending in `registry.sqlite`.

## 3. Register the Product Repository

Register the local project and inspect it:

```sh
HARNESS_HOME="$HARNESS_HOME" "$HARNESS_BIN" project register --project-id demo --repo-root "$PRODUCT_REPO"
HARNESS_HOME="$HARNESS_HOME" "$HARNESS_BIN" project list
```

`--repo-root` must name an existing accessible directory. The `Product Repository` and the `Harness Runtime Home` are separate locations; see [Runtime Boundaries](../reference/runtime-boundaries.md) for the exact boundary.

## 4. Register MCP surfaces

### Agent MCP surface

Register the agent-role MCP surface with the baseline workflow profile:

```sh
HARNESS_HOME="$HARNESS_HOME" "$HARNESS_BIN" surface register \
  --project-id demo \
  --surface-id agent_mcp \
  --surface-instance-id agent_mcp_local \
  --kind mcp \
  --interaction-role agent \
  --profile baseline-workflow
```

Warning: omitting `--profile baseline-workflow` and omitting explicit `--access-class` options creates a `read_status`-only surface. That surface can answer status reads but cannot support the baseline workflow calls that require `core_mutation`, `write_authorization`, `artifact_registration`, or `run_recording`.

### User-interaction MCP surface

Register a separate user-interaction surface:

```sh
HARNESS_HOME="$HARNESS_HOME" "$HARNESS_BIN" surface register \
  --project-id demo \
  --surface-id user_ui \
  --surface-instance-id user_ui_local \
  --kind mcp \
  --interaction-role user_interaction \
  --access-class read_status \
  --access-class core_mutation
```

Keep this surface separate from the agent surface:

- `actor_kind=user` alone does not establish user authority.
- An `agent` role surface cannot become a `user_interaction` surface by changing request text.
- Authority-bearing user judgments require a process bound to a registered `user_interaction` surface.
- The actual user-facing UI or connector must invoke that binding for the user action.

Exact actor-provenance rules are in [Agent Integration](../reference/agent-integration.md#current-surface-context).

## 5. Inspect registration

List the registered surfaces:

```sh
HARNESS_HOME="$HARNESS_HOME" "$HARNESS_BIN" surface list --project-id demo
```

Confirm that `agent_mcp` has instance `agent_mcp_local`, and `user_ui` has instance `user_ui_local`. Explicit instance IDs make host startup deterministic because each MCP process can bind to one known `surface_instance_id` instead of relying on implicit instance selection.

## 6. Run MCP preflight

Preflight the agent binding before connecting a host:

```sh
HARNESS_HOME="$HARNESS_HOME" \
HARNESS_PROJECT_ID=demo \
HARNESS_SURFACE_ID=agent_mcp \
HARNESS_SURFACE_INSTANCE_ID=agent_mcp_local \
"$HARNESS_MCP_BIN" --check
```

The report should include:

- `configuration: valid`
- `transport: stdio`
- the absolute `runtime_home`
- `project_id: demo`
- `surface_id: agent_mcp`
- `surface_instance_id: agent_mcp_local`
- `interaction_role: agent`
- registered `access_classes`
- `baseline_workflow_access: full`

Preflight the user-interaction binding too:

```sh
HARNESS_HOME="$HARNESS_HOME" \
HARNESS_PROJECT_ID=demo \
HARNESS_SURFACE_ID=user_ui \
HARNESS_SURFACE_INSTANCE_ID=user_ui_local \
"$HARNESS_MCP_BIN" --check
```

For the user-interaction surface, expect `interaction_role: user_interaction`, `access_classes: read_status,core_mutation`, and `baseline_workflow_access: not_applicable`.

Detailed startup validation and failure conditions stay in [MCP Transport](../reference/mcp-transport.md#configuration-preflight).

## 7. Configure an MCP host

Configure the host to launch two local child processes. This JSON-shaped example is host-neutral; actual host file names, keys around the child-process entries, and wrapper syntax are host-specific. The command path and environment meanings are Harness-defined.

```json
{
  "mcpServers": {
    "harness-agent": {
      "command": "/absolute/path/to/Harness_Project/target/release/harness-mcp",
      "env": {
        "HARNESS_HOME": "/absolute/path/to/harness-runtime-home",
        "HARNESS_PROJECT_ID": "demo",
        "HARNESS_SURFACE_ID": "agent_mcp",
        "HARNESS_SURFACE_INSTANCE_ID": "agent_mcp_local"
      }
    },
    "harness-user-interaction": {
      "command": "/absolute/path/to/Harness_Project/target/release/harness-mcp",
      "env": {
        "HARNESS_HOME": "/absolute/path/to/harness-runtime-home",
        "HARNESS_PROJECT_ID": "demo",
        "HARNESS_SURFACE_ID": "user_ui",
        "HARNESS_SURFACE_INSTANCE_ID": "user_ui_local"
      }
    }
  }
}
```

Do not configure a URL, TCP port, HTTP endpoint, or socket path for the baseline local MCP process.

## 8. Verify connection and tool discovery

After the host launches the agent process, verify this MCP sequence:

1. Send `initialize`.
2. Send the initialized notification.
3. Send `tools/list`.
4. Call `harness.status`.

Expected observations:

- `initialize` returns `serverInfo.name` as `harness-mcp`.
- `tools/list` exposes exactly nine public Harness tools.
- The exact public method list is owned by [API Methods](../reference/api/methods.md).
- `harness.status` returns MCP text content containing serialized Harness JSON.

A raw stdio smoke test uses one JSON value per line:

```text
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"harness-quickstart","version":"0.0.0"}}}
{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"harness.status","arguments":{"envelope":{"project_id":"demo","task_id":null,"actor_kind":"agent","surface_id":"agent_mcp","request_id":"req_quickstart_status","idempotency_key":null,"expected_state_version":null,"dry_run":false,"locale":"en-US"},"include":{"task":true,"pending_user_judgments":true,"write_authority":false,"evidence":false,"close":true,"guarantees":true}}}}
```

## 9. Interpret `tools/call` responses

There are two parsing layers:

1. Read `result.content[0].text` from the MCP `tools/call` result.
2. Parse that string as Harness JSON.

Then inspect the parsed Harness response:

- `base.response_kind`
- `errors`

`isError: false` means the MCP transport call succeeded. It does not mean the Harness request was accepted. A Harness domain-level `rejected` response also uses successful MCP transport and can appear with `isError: false`.

JSON-RPC `error` is different: it is for protocol, invalid-parameter, or adapter/internal failures. Exact response schemas and error meanings stay with [API Schema Core](../reference/api/schema-core.md#common-response), [API Error Routing](../reference/api/error-routing.md), [API Error Codes](../reference/api/error-codes.md), and [API Error Details](../reference/api/error-details.md).

## 10. Stop and reconnect

An MCP host stops a local session by closing stdin or terminating the child process. Stdin EOF ends the stdio session after stdout is flushed.

SQLite state remains in the Runtime Home. Starting a new `harness-mcp` child process with the same `HARNESS_HOME`, `HARNESS_PROJECT_ID`, `HARNESS_SURFACE_ID`, and `HARNESS_SURFACE_INSTANCE_ID` reconnects to the same stored project state. Changing project, surface, or surface instance requires another process.

## Troubleshooting

| Symptom | Likely cause | Next action |
|---|---|---|
| Runtime Home is not initialized. | The MCP process uses a `HARNESS_HOME` without a valid registry. | Run `harness init` with the same absolute `HARNESS_HOME`; see [Administrative CLI](../reference/admin-cli.md#runtime-home-selection). |
| Project is not registered. | `HARNESS_PROJECT_ID=demo` has no project record in that Runtime Home. | Run `harness project register` and then `harness project list`. |
| Project is inactive. | The project record is present but not usable for startup. | Use the administrative project registration contract and MCP startup validation owner to resolve it. |
| Surface is not registered. | `HARNESS_SURFACE_ID` does not match a registered surface for the project. | Register the surface or correct the environment value. |
| Explicit instance is unknown. | `HARNESS_SURFACE_INSTANCE_ID` does not match a registered instance. | Check `harness surface list --project-id demo` and update the binding. |
| Implicit instance selection is ambiguous. | No explicit instance was provided and more than one candidate exists. | Set `HARNESS_SURFACE_INSTANCE_ID` explicitly. |
| Agent surface behaves as read-only. | The surface was registered without `--profile baseline-workflow` or explicit access classes. | Register an agent MCP surface with the baseline workflow profile. |
| Local-access metadata is invalid. | The stored surface access metadata is missing, malformed, or grants no access class. | Re-register through the administrative CLI or route details to [Agent Integration](../reference/agent-integration.md) and [MCP Transport](../reference/mcp-transport.md). |
| JSON-RPC success is confused with Harness acceptance. | The client checked only `isError`. | Parse `result.content[0].text` and inspect `base.response_kind` plus `errors`. |
