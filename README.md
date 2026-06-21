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

- Rust 1.85 or newer with Cargo; Rust 1.85 is the minimum compiler
  version verified for the current workspace
- a local checkout of this repository, or another Harness Server installation
  that provides `harness` and `harness-mcp`
- a local `Product Repository` directory to bind
- a `Harness Runtime Home` that is separate from the `Product Repository`
- an MCP host that can launch a local stdio MCP process when you are ready to connect Harness to a host

## Initial Setup Shape

Initial setup has three separate stages:

1. Prepare Harness Server by building or locating the `harness` and
   `harness-mcp` executables.
2. From the `Product Repository`, bind that repository to a separate
   `Harness Runtime Home` with an explicit `--repo-root .`.
3. Apply the generated host-neutral MCP configuration fragment to the external
   MCP host's own configuration mechanism.

The four locations stay distinct:

| Location | Owner | Typical contents | Setup writes there automatically? |
|---|---|---|---|
| Harness Server source or installation | Harness Server maintainer or installer | `harness`, `harness-mcp`, source files or installed executable resources. | Only a source build writes Cargo build output under this repository's `target/`. |
| `Harness Runtime Home` | Local Harness operator | Harness registry, project state, and runtime data. | Yes. Setup creates or reuses records there. |
| `Product Repository` | Product project owner | The user's product files and project workspace. | No. Setup registers its path in Runtime Home; selecting it does not edit its contents or place Harness databases inside it. |
| External MCP host configuration | External MCP host operator | Host-owned settings that start `harness-mcp` with the generated environment. | No. Harness prints or writes a host-neutral fragment; the host owns its actual settings file and wrapper shape. |

`--config-dir` is an explicitly selected output directory for generated
host-neutral fragments, not the external host configuration location itself.

## Build The Executables

Working directory: Harness Server source repository root.

```sh
cargo build -p harness-cli -p harness-mcp
```

That builds:

- `target/debug/harness`
- `target/debug/harness-mcp`

For release executable paths and build verification, see [Installation](docs/en/getting-started/installation.md).

## Shortest Local MCP Setup

After building, go to the `Product Repository` you want Harness to bind. Invoke
`harness` by explicit path or by an installed command, and pass `--repo-root .`
so the current directory selection is deliberate.

Working directory: `Product Repository` root.

```sh
/absolute/path/to/harness setup local-mcp \
  --repo-root . \
  --runtime-home /absolute/path/to/harness-runtime-home \
  --project-id demo \
  --mcp-command /absolute/path/to/harness-mcp
```

A successful first setup includes human-readable lines like:

```text
setup: complete
preflight: passed
agent_config_json:
```

The printed `agent_config_json` is a host-neutral MCP configuration fragment. Copy it into the wrapper shape and configuration location used by the MCP host you operate. Do not configure a URL, port, HTTP endpoint, or socket path for the baseline local MCP process.

If you intentionally use this Harness Server source repository as a
`Product Repository` for dogfooding, still select it explicitly with
`--repo-root .` from that checkout or with its path. That is not the normal
installation flow.

For the complete first-run path, use [Quickstart](docs/en/getting-started/quickstart.md). For all setup options, dry-run preview, JSON output, configuration files, interactive setup, connection checks, and troubleshooting, use [Local MCP Setup](docs/en/guides/local-mcp-setup.md).

## Documentation Routes

- English documentation: [docs/en/README.md](docs/en/README.md)
- Korean documentation: [docs/ko/README.md](docs/ko/README.md)
- Documentation directory guide: [docs/README.md](docs/README.md)

Reader paths:

- Product users: [Getting Started Overview](docs/en/getting-started/overview.md), then [User Guide](docs/en/guides/user-workflow.md)
- Local MCP operators: [Installation](docs/en/getting-started/installation.md), [Quickstart](docs/en/getting-started/quickstart.md), then [Local MCP Setup](docs/en/guides/local-mcp-setup.md)
- Source-code learners: [Developer Documentation](docs/en/development/README.md), then [Codebase Tour](docs/en/development/codebase-tour.md), [Request Lifecycle](docs/en/development/request-lifecycle.md), and [Architecture](docs/en/development/architecture.md)
- Reference readers: [Reference Index](docs/en/reference/README.md)

Reader documentation explains and sequences the product. Exact contracts live in Reference documents, including [Administrative CLI](docs/en/reference/admin-cli.md), [MCP Transport](docs/en/reference/mcp-transport.md), [Runtime Boundaries](docs/en/reference/runtime-boundaries.md), and [API Methods](docs/en/reference/api/methods.md). `docs/doc-index.yaml` is maintenance metadata for exact owner routing, not an ordinary reader's first step.
