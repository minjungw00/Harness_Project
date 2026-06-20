# Quickstart

This page owns the shortest supported local MCP success path. It assumes you
can build or locate the Harness Server executables and that you have a
`Product Repository` you want to bind.

For build details, release executable locations, and executable discovery rules,
see [Installation](installation.md). For every setup option and troubleshooting
path, see [Local MCP Setup](../guides/local-mcp-setup.md).

## Stage 1: Prepare Harness Server

Working directory: Harness Server source repository root.

```sh
cargo build -p harness-cli -p harness-mcp
```

This provides:

- `target/debug/harness`
- `target/debug/harness-mcp`

Use those files by absolute path in the next stage, or use installed
executables that provide the same `harness` and `harness-mcp` commands.

## Stage 2: Bind A Product Repository

Start from the project workspace you want Harness to register. The current
directory is selected only because the command passes `--repo-root .`.

Working directory: `Product Repository` root.

```sh
/absolute/path/to/harness setup local-mcp \
  --repo-root . \
  --runtime-home /absolute/path/to/harness-runtime-home \
  --project-id demo \
  --mcp-command /absolute/path/to/harness-mcp
```

Use a `Harness Runtime Home` outside the `Product Repository`. If
`--runtime-home` is omitted, setup uses the documented `HARNESS_HOME` or
user-home fallback, but the selected Runtime Home still must stay separate from
the `Product Repository`.

Setup registers the `Product Repository` path in Runtime Home, creates or
reuses the local MCP agent surface, runs MCP preflight, and prints host-neutral
MCP configuration. It does not install, discover, or edit an external MCP host.

Setup does not place Harness databases or runtime artifacts inside the
`Product Repository`, and selecting the repository does not by itself edit
product files. If you pass `--config-dir`, setup may write generated
host-neutral configuration fragments to that explicit directory.

## Stage 3: Configure The External MCP Host

The text output includes `agent_config_json` with a host-neutral fragment like:

```json
{
  "mcpServers": {
    "harness-agent": {
      "command": "/absolute/path/to/harness-mcp",
      "env": {
        "HARNESS_HOME": "/absolute/path/to/harness-runtime-home",
        "HARNESS_PROJECT_ID": "demo",
        "HARNESS_SURFACE_ID": "agent_mcp",
        "HARNESS_SURFACE_INSTANCE_ID": "agent_mcp_local"
      }
    }
  }
}
```

Apply that fragment according to the MCP host's supported configuration
mechanism. The external host owns the actual settings file, directory, and
wrapper shape. The baseline local MCP process uses stdio, so do not configure a
URL, TCP port, HTTP endpoint, or socket path.

## Locations And Ownership

| Location | Owner | Typical contents | Setup writes there automatically? |
|---|---|---|---|
| Harness Server source or installation | Harness Server maintainer or installer | `harness`, `harness-mcp`, source files or installed executable resources. | Source builds write Cargo output under `target/`; local MCP setup only reads or invokes the executables. |
| `Harness Runtime Home` | Local Harness operator | Harness registry, project state, surface registrations, and runtime data. | Yes. Setup creates or reuses local records there. |
| `Product Repository` | Product project owner | Product source, tests, docs, and project configuration. | No. Setup records its path in Runtime Home; it does not put Harness databases or runtime artifacts there merely because it is selected. |
| MCP host configuration location | External MCP host operator | Host-specific settings that launch `harness-mcp` with the generated environment. | No. Harness prints or writes a host-neutral fragment; the host's own settings remain host-owned. |

`--config-dir` is an explicit output location for generated host-neutral
fragments such as `harness-agent.mcp.json`. It is not the external host's
configuration location unless the host operator deliberately copies or adapts
the fragment there.

## Recognize Success

A successful setup includes lines like:

```text
setup: complete
project_id: demo
repo_root: /absolute/path/to/product-repository
agent_surface_id: agent_mcp
agent_surface_instance_id: agent_mcp_local
preflight: passed
agent_preflight: passed
```

Treat this as human-readable command output, not as a public API schema.
`preflight: passed` means the local MCP process binding validated. Later MCP
transport success is still separate from Harness domain acceptance; clients
must inspect the parsed Harness response for domain result or rejection.

## Intentional Self-Hosting

You may intentionally select the Harness Server source repository itself as a
`Product Repository` for dogfooding. Do that only by explicit selection, either
from that checkout with `--repo-root .` or from another directory with its
path. This is not the normal installation flow.

## Continue

- Full setup operations, dry-run preview, JSON output, configuration files, interactive setup, recovery, and troubleshooting: [Local MCP Setup](../guides/local-mcp-setup.md)
- Agent workflow: [Agent Guide](../guides/agent-workflow.md)
- Exact `harness` setup behavior: [Administrative CLI](../reference/admin-cli.md#local-mcp-setup-orchestration)
- Exact `harness-mcp` process behavior: [MCP Transport](../reference/mcp-transport.md)
- Exact runtime location boundaries: [Runtime Boundaries](../reference/runtime-boundaries.md)
