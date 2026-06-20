# Installation

This page owns the source prerequisites and build procedure for the current repository executables. It does not define package-manager distribution, operating-system support, public API behavior, storage effects, or MCP wire behavior.

## Prerequisites

For the source build path, you need:

- a local checkout of this repository
- a Rust toolchain with Cargo that can build the workspace
- a shell that can run Cargo and local executables
- a local `Product Repository` directory for first-run setup
- absolute paths when passing `Product Repository`, `Harness Runtime Home`, or executable paths to setup

An MCP host is only needed when you are ready to connect the generated host-neutral configuration to a real host. The build and setup preflight can run without naming a specific external host.

## Build From The Repository Root

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

From the repository root after the quick local build:

```sh
target/debug/harness --version
target/debug/harness setup local-mcp --help
target/debug/harness-mcp --version
target/debug/harness-mcp --help
```

The version commands print `harness <version>` and `harness-mcp <version>`. The help commands should print local administrative setup usage and MCP environment usage.

## Executable Discovery During Setup

`harness setup local-mcp` performs setup. `harness-mcp` is the child process that an MCP host launches after setup.

Setup can discover `harness-mcp` when either:

- `harness-mcp` is beside the running `harness` executable, as in `target/debug/` or `target/release/`
- `harness-mcp` is on `PATH`

When you want setup to use one exact executable, pass `--mcp-command /absolute/path/to/harness-mcp`. The generated host-neutral configuration records the selected command path.

## Next Step

Continue to [Quickstart](quickstart.md) for the shortest local MCP setup path.

Exact command behavior belongs to [Administrative CLI](../reference/admin-cli.md). Exact `harness-mcp` startup, environment, stdio transport, preflight, and shutdown behavior belongs to [MCP Transport](../reference/mcp-transport.md).
