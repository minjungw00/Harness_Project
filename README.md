# Harness

Harness is the local work-authority product/system for AI-assisted product work. It helps users and agents keep scope, user-owned judgment, evidence, verification criteria, final acceptance, residual-risk acceptance, and close readiness visible while product work moves through an AI-assisted workflow.

Harness exists for local product work where chat momentum can otherwise blur important boundaries: what is in scope, what the user actually decided, what evidence supports a claim, what was checked, and what still carries risk.

## Current Repository Surface

This repository currently contains:

- maintained English and Korean documentation
- a Cargo Rust workspace
- the `harness` administrative/bootstrap executable from package `harness-cli`
- the `harness-mcp` local MCP stdio executable from package `harness-mcp`
- implementation, integration, and conformance test paths

The baseline local MCP process is stdio-based. It is launched as a local child process by an MCP host; it is not a TCP, HTTP, socket, or other network listener.

## Prerequisites

For the source build and local setup path, you need:

- a Rust toolchain with Cargo that can build this workspace
- a local checkout of this repository
- a local `Product Repository` directory to bind
- an absolute path for any `Product Repository` or `Harness Runtime Home` you pass to setup
- an MCP host that can launch a local stdio MCP process when you are ready to connect Harness to a host

## Build The Executables

From this repository root:

```sh
cargo build -p harness-cli -p harness-mcp
```

That builds:

- `target/debug/harness`
- `target/debug/harness-mcp`

For release executable paths and build verification, see [Installation](docs/en/getting-started/installation.md).

## Shortest Local MCP Setup

From this repository root, choose an existing product repository and a disposable or durable runtime home:

```sh
target/debug/harness setup local-mcp \
  --repo-root /absolute/path/to/product-repository \
  --runtime-home /absolute/path/to/harness-runtime-home \
  --project-id demo \
  --mcp-command "$(pwd)/target/debug/harness-mcp"
```

A successful first setup includes human-readable lines like:

```text
setup: complete
preflight: passed
agent_config_json:
```

The printed `agent_config_json` is a host-neutral MCP configuration fragment. Copy it into the wrapper shape and configuration location used by the MCP host you operate. Do not configure a URL, port, HTTP endpoint, or socket path for the baseline local MCP process.

For the complete first-run path, use [Quickstart](docs/en/getting-started/quickstart.md). For all setup options, dry-run preview, JSON output, configuration files, interactive setup, connection checks, and troubleshooting, use [Local MCP Setup](docs/en/guides/local-mcp-setup.md).

## Documentation Routes

- English documentation: [docs/en/README.md](docs/en/README.md)
- Korean documentation: [docs/ko/README.md](docs/ko/README.md)
- Documentation directory guide: [docs/README.md](docs/README.md)

Reader paths:

- Product users: [Getting Started Overview](docs/en/getting-started/overview.md), then [User Guide](docs/en/guides/user-workflow.md)
- Local MCP operators: [Installation](docs/en/getting-started/installation.md), [Quickstart](docs/en/getting-started/quickstart.md), then [Local MCP Setup](docs/en/guides/local-mcp-setup.md)
- Source-code learners: [Implementation Guide](docs/en/development/change-guide.md), then [Architecture](docs/en/development/architecture.md)
- Reference readers: [Reference Index](docs/en/reference/README.md)

Reader documentation explains and sequences the product. Exact contracts live in Reference documents, including [Administrative CLI](docs/en/reference/admin-cli.md), [MCP Transport](docs/en/reference/mcp-transport.md), [Runtime Boundaries](docs/en/reference/runtime-boundaries.md), and [API Methods](docs/en/reference/api/methods.md). `docs/doc-index.yaml` is maintenance metadata for exact owner routing, not an ordinary reader's first step.
