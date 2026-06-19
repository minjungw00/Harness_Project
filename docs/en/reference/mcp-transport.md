# MCP transport reference

This document owns the local `harness-mcp` process contract: process startup, stdio transport framing, startup binding, configuration preflight, MCP response wrapping, and shutdown/reconnection behavior.

It does not define public Harness API method behavior, public request or response schemas, access-class meanings, surface registration meaning, storage record layout, security guarantees, or Core authority semantics.

## Owns / does not own

This document owns:

- `harness-mcp` command-line behavior and exit codes
- required and optional process environment variables
- stdio JSON-RPC framing and supported MCP request methods
- MCP startup validation, fixed process binding, and instance selection
- `harness-mcp --check` diagnostic output
- MCP `tools/call` response wrapping
- process shutdown and reconnection behavior

This document does not own:

- the public Harness method list or method owner table; see [API Methods](api/methods.md)
- public Harness request and response schemas; see [API Schema Core](api/schema-core.md)
- access-class value meanings; see [API Value Sets](api/schema-value-sets.md#access-class-values)
- surface registration meaning, access derivation, fixed surface-context meaning, and actor provenance; see [Agent Integration](agent-integration.md)
- Runtime Home path resolution; see [Administrative CLI](admin-cli.md#runtime-home-selection)
- storage layout, migrations, and storage effects; see the storage owners through [Storage](storage.md)

## Process model

`harness-mcp` is a local MCP stdio process. With no command-line arguments, an MCP host starts it as a child process and communicates through stdin/stdout. It is not a TCP listener or HTTP listener.

Command-line behavior:

- No command-line arguments starts the stdio MCP loop.
- `harness-mcp --help` exits with code `0` and requires no Harness environment variables.
- `harness-mcp --version` prints `harness-mcp <package-version>`, exits with code `0`, and requires no Harness environment variables.
- `harness-mcp --check` validates configuration without entering the stdin loop.
- Unknown options or extra positional arguments are usage errors.

Exit and stream behavior:

- Successful help, version, preflight, or normal EOF shutdown exits with code `0`.
- Usage errors write diagnostics to stderr and exit with code `2`.
- Startup or preflight environment/storage failures write diagnostics to stderr and exit with code `1`.

## Process environment

Required:

- `HARNESS_PROJECT_ID`
- `HARNESS_SURFACE_ID`

Optional:

- `HARNESS_HOME`
- `HARNESS_SURFACE_INSTANCE_ID`

`harness-mcp --help` and `harness-mcp --version` do not read these variables. The stdio loop and `harness-mcp --check` use the configured Runtime Home and binding variables.

Runtime Home path resolution is shared with the administrative CLI and is owned by [Administrative CLI](admin-cli.md#runtime-home-selection). Both executables use the same `HARNESS_HOME` and default user-home rules.

## Startup validation

Before entering the stdio loop, `harness-mcp` validates the fixed process binding and the local registration records it depends on.

Startup validation requires:

- the Runtime Home registry exists and is valid
- the configured project is registered
- the project status is `active`
- the project state database is valid
- the configured surface is registered
- the configured surface instance exists, or can be selected unambiguously
- the registered `interaction_role` is recognized
- `capability_profile` and metadata are JSON objects
- local access metadata is valid and grants at least one access class

Instance selection when `HARNESS_SURFACE_INSTANCE_ID` is absent:

1. Use the registered project default only when it belongs to the configured `surface_id`.
2. Otherwise use one usable candidate only when exactly one exists.
3. Fail on no candidate or multiple candidates.

## Fixed process binding

One `harness-mcp` process is bound to:

- one `project_id`
- one `surface_id`
- one `surface_instance_id`

These values remain fixed for the process lifetime. Changing project, surface, or surface instance requires another process.

The public `ToolEnvelope.project_id` and `ToolEnvelope.surface_id` values in each public Harness request must match the fixed binding. They are request echoes for protocol consistency, not caller-selected authority. The fixed surface-context meaning, access derivation, and actor provenance boundaries are owned by [Agent Integration](agent-integration.md#current-surface-context).

## Configuration preflight

`harness-mcp --check` validates configuration without entering the stdin loop.

Stable successful output is line-oriented and uses this order:

```text
configuration: valid
transport: stdio
runtime_home: <absolute path>
project_id: <value>
surface_id: <value>
surface_instance_id: <value>
interaction_role: agent|user_interaction
access_classes: <comma-separated registered grants>
baseline_workflow_access: full|partial|not_applicable
missing_access_classes: <comma-separated values or empty>
```

For an `agent` surface:

- `full` means all five `baseline-workflow` access classes are present.
- `partial` means at least one is absent.
- `missing_access_classes` uses the canonical profile order: `read_status`, `core_mutation`, `write_authorization`, `artifact_registration`, `run_recording`.

For a `user_interaction` surface:

- `baseline_workflow_access` is `not_applicable`.
- `missing_access_classes` is empty.

This output is diagnostic registration information. It does not itself grant authority.

`--check` behavior:

- It does not read stdin.
- It does not create a `Task`.
- It does not increment `state_version`.
- It does not register a project or surface.
- It does not create application records.
- It may perform already-defined storage schema validation or migration as part of normal database opening.

## MCP wire behavior

Framing:

- Each input line contains one JSON value.
- Each output line contains one JSON response.
- Stdin EOF ends the process after stdout is flushed.
- No readiness message is emitted before `initialize`.

Supported MCP request methods:

- `initialize`
- `ping`
- `tools/list`
- `tools/call`

Notifications receive no response. Unsupported requests return JSON-RPC `-32601`. Malformed JSON returns JSON-RPC `-32700`.

The transport exposes exactly the public Harness method set owned by [API Methods](api/methods.md). This document does not create a second independently owned method list.

## `tools/call` response wrapping

`tools/call` wraps the Harness response JSON inside the MCP result:

- Harness response JSON is serialized as the string in `result.content[0].text`.
- Clients must parse that string as JSON to inspect the Harness response.
- Successful MCP transport returns `isError: false`, including Harness domain-level rejected responses.
- Harness domain success or rejection is determined from the parsed Harness response, especially `base.response_kind` and `errors`.
- JSON-RPC `error` is reserved for protocol, invalid-parameter, or adapter/internal failures.

Harness response branch shapes and error meanings stay with their owners:

- shared response branches: [API Schema Core](api/schema-core.md#common-response)
- response branch routing: [API Error Routing](api/error-routing.md)
- public error codes: [API Error Codes](api/error-codes.md)
- machine-readable error details: [API Error Details](api/error-details.md)

## Shutdown and reconnection

Closing stdin or terminating the child process ends the MCP session.

Shutdown and reconnection rules:

- SQLite state remains in the Runtime Home.
- Restarting with the same binding reconnects to the same stored project state.
- Changing binding values requires a new process.

Runtime data location boundaries are owned by [Runtime Boundaries](runtime-boundaries.md), and storage record details are owned by the storage owners routed from [Storage](storage.md).
