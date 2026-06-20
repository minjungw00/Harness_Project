# Quickstart

This page owns the shortest supported local MCP success path. It assumes you are starting from a local checkout of this repository and have a `Product Repository` directory you want to bind.

For build details, release executable locations, and executable discovery rules, see [Installation](installation.md). For every setup option and troubleshooting path, see [Local MCP Setup](../guides/local-mcp-setup.md).

## 1. Build The Two Executables

From the Harness repository root:

```sh
cargo build -p harness-cli -p harness-mcp
```

This provides:

- `target/debug/harness`
- `target/debug/harness-mcp`

## 2. Select A Product Repository

Choose an existing project directory, or create one outside the Harness documentation tree. Use its absolute path as the `Product Repository`.

In the commands below, replace:

- `/absolute/path/to/product-repository` with the product repository path
- `/absolute/path/to/harness-runtime-home` with the runtime home path for this local setup

## 3. Run Local MCP Setup

From the Harness repository root:

```sh
target/debug/harness setup local-mcp \
  --repo-root /absolute/path/to/product-repository \
  --runtime-home /absolute/path/to/harness-runtime-home \
  --project-id demo \
  --mcp-command "$(pwd)/target/debug/harness-mcp"
```

This registers the selected `Product Repository`, creates or reuses the local MCP agent surface, runs MCP preflight, and prints host-neutral MCP configuration. It does not install, discover, or edit an external MCP host.

## 4. Read The Generated Configuration

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

Copy the fragment into the wrapper shape and configuration location used by the MCP host you operate. The baseline local MCP process uses stdio. Do not configure a URL, TCP port, HTTP endpoint, or socket path.

## 5. Recognize Success

A successful setup includes lines like:

```text
setup: complete
project_id: demo
agent_surface_id: agent_mcp
agent_surface_instance_id: agent_mcp_local
preflight: passed
agent_preflight: passed
```

Treat this as human-readable command output, not as a public API schema. `preflight: passed` means the local MCP process binding validated. Later MCP transport success is still separate from Harness domain acceptance; clients must inspect the parsed Harness response for domain result or rejection.

## 6. Continue

- Full setup operations, dry-run preview, JSON output, configuration files, interactive setup, recovery, and troubleshooting: [Local MCP Setup](../guides/local-mcp-setup.md)
- Agent workflow: [Agent Guide](../guides/agent-workflow.md)
- Exact `harness` setup behavior: [Administrative CLI](../reference/admin-cli.md#local-mcp-setup-orchestration)
- Exact `harness-mcp` process behavior: [MCP Transport](../reference/mcp-transport.md)
