# MCP transport reference

This document owns the local `harness-mcp` process contract: process startup, process environment, stdio transport framing, startup binding and validation, MCP response wrapping, and shutdown/reconnection behavior.

It does not define public Harness API method behavior, public request or response schemas, access-class meanings, surface registration meaning, storage record layout, security guarantees, or Core authority semantics.

## Owns / does not own

This document owns:

- `harness-mcp` process startup and exit behavior
- required and optional process environment variables, including MCP Runtime Home resolution
- stdio JSON-RPC framing and supported MCP request methods
- MCP startup validation, fixed process binding, and instance selection
- MCP `tools/call` response wrapping
- process shutdown and reconnection behavior

This document does not own:

- the public Harness method list or method owner table; see [API Methods](api/methods.md)
- public Harness request and response schemas; see [API Schema Core](api/schema-core.md)
- access-class value meanings; see [API Value Sets](api/schema-value-sets.md#access-class-values)
- surface registration meaning, access derivation, fixed surface-context meaning, and actor provenance; see [Agent Integration](agent-integration.md)
- administrative Runtime Home commands, project registration, and surface registration; see [Administrative CLI](admin-cli.md)
- storage layout, migrations, and storage effects; see the storage owners through [Storage](storage.md)

## Process model

`harness-mcp` is a local MCP stdio process. An MCP host starts it as a child process and communicates through stdin/stdout. It is not a TCP listener, HTTP listener, Unix-domain socket listener, or other network listener.

Command-line behavior:

- Launch `harness-mcp` without command-line arguments.
- No command-line flag mode is baseline-implemented for help, version, server startup, or preflight.
- The current Rust entry point does not use command-line arguments as a supported mode selector.

Exit and stream behavior:

- Normal stdin EOF shutdown flushes stdout and exits with code `0`.
- Startup environment, JSON, or storage failures write diagnostics to stderr and exit with code `1`.
- Once the stdio loop is running, malformed JSON and unsupported JSON-RPC requests return JSON-RPC errors when a response can be written.

## Process environment

Required:

- `HARNESS_PROJECT_ID`
- `HARNESS_SURFACE_ID`

Optional:

- `HARNESS_HOME`
- `HARNESS_SURFACE_INSTANCE_ID`

The stdio process uses these variables before entering the stdin loop.

Current MCP Runtime Home resolution:

1. If `HARNESS_HOME` is present, use it as supplied.
2. If `HARNESS_HOME` is absent, require `HOME` and use `HOME/.harness`.
3. The MCP process does not use `USERPROFILE`, `HOMEDRIVE`, or `HOMEPATH` fallback rules.
4. The MCP process does not canonicalize the selected Runtime Home before startup validation.

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

The current Rust executable does not implement a separate `harness-mcp --check` preflight mode. Startup validation happens as part of process startup before entering the stdin loop.

Startup validation failure:

- writes a diagnostic to stderr through the process entry point
- exits with code `1`
- does not enter the stdio loop

Startup validation may perform already-defined storage schema validation or migration as part of normal database opening. It does not by itself register a project or surface, create a `Task`, increment `state_version`, or create application records.

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
