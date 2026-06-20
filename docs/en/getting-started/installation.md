# Installation

This page owns Stage 1 of initial setup: preparing the Harness Server
executables. It covers source prerequisites, build commands, executable paths,
and build verification for the current repository executables. It does not
define package-manager distribution, operating-system support, public API
behavior, storage effects, Product Repository registration, external host
configuration, or MCP wire behavior.

## Prerequisites

For the source build path, you need:

- a local checkout of this repository
- a Rust toolchain with Cargo that can build the workspace
- a shell that can run Cargo and local executables
- a local `Product Repository` directory for the next setup stage
- a separate `Harness Runtime Home` for the next setup stage

An MCP host is only needed when you are ready to connect the generated host-neutral configuration to a real host. The build and setup preflight can run without naming a specific external host.

## Build From The Repository Root

Working directory: Harness Server source repository root.

For a quick local build:

```sh
cargo build -p harness-cli -p harness-mcp
```

Expected debug executables:

- `target/debug/harness`
- `target/debug/harness-mcp`

For release executables:

```sh
cargo build --release -p harness-cli -p harness-mcp
```

Expected release executables:

- `target/release/harness`
- `target/release/harness-mcp`

The Cargo package names are `harness-cli` and `harness-mcp`. The executable names are `harness` and `harness-mcp`.

## Verify The Build

Working directory: Harness Server source repository root after the quick local
build.

```sh
target/debug/harness --version
target/debug/harness setup local-mcp --help
target/debug/harness-mcp --version
target/debug/harness-mcp --help
```

The version commands print `harness <version>` and `harness-mcp <version>`. The help commands should print local administrative setup usage and MCP environment usage.

## Executable Discovery During Setup

`harness setup local-mcp` performs setup in the next stage. `harness-mcp` is
the child process that an MCP host launches after setup.

Setup can discover `harness-mcp` when either:

- `harness-mcp` is beside the running `harness` executable, as in `target/debug/` or `target/release/`
- `harness-mcp` is on `PATH`

When you want setup to use one exact executable, pass `--mcp-command /absolute/path/to/harness-mcp`. The generated host-neutral configuration records the selected command path.

Installation location is not runtime state. The Harness Server source
repository or installation contains the executables; the `Harness Runtime Home`
contains Harness runtime records; the `Product Repository` contains product
files; and the external MCP host owns its actual configuration file.

## Next Step

Continue to [Quickstart](quickstart.md). It starts from the `Product
Repository` root and uses `--repo-root .` for the shortest local MCP setup
path.

Exact command behavior belongs to [Administrative CLI](../reference/admin-cli.md). Exact `harness-mcp` startup, environment, stdio transport, preflight, and shutdown behavior belongs to [MCP Transport](../reference/mcp-transport.md).
