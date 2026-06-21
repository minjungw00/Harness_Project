# Legacy local MCP setup

Use this page only when you must understand or recover the compatibility command `harness setup local-mcp`.

New Codex and Claude Code setup examples must use `harness agent install`; start with [Quickstart](../getting-started/quickstart.md), then [Agent Host Setup](agent-host-setup.md). Multi-repository user-scope topology belongs in [Multi-Repository Agent Setup](multi-repository-agent-setup.md).

`harness setup local-mcp` is a non-baseline compatibility command for legacy fixed-project MCP configuration. It may still be useful for diagnosing older local configurations or scripts that predate Agent Integration Profiles.

## Compatibility Shape

Legacy setup generated host-neutral MCP fragments that used fixed project and surface environment variables:

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

That shape is not the current baseline for direct Codex or Claude Code installation. Current host setup uses:

```text
harness-mcp --integration <integration_id>
```

and selects the project per public tool call.

## When To Use The Current Path Instead

Use `harness agent install` when you need:

- direct Codex installation
- direct Claude Code installation
- one user-scope integration serving multiple explicitly allowed projects
- Host Installation status and verification
- optional repository guidance
- safe uninstall of managed host configuration and guidance
- generic export for an unsupported host

Exact compatibility behavior remains in [Administrative CLI](../reference/admin-cli.md#local-mcp-setup-orchestration). Exact current process behavior remains in [MCP Transport](../reference/mcp-transport.md).
