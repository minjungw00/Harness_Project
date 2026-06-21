# Installation

This page owns the first setup stage: preparing the Harness Server executables. It covers source prerequisites, build commands, executable paths, and build verification for the current repository executables. It does not define package-manager distribution, operating-system support, public API behavior, storage effects, Product Repository registration, host trust, or MCP wire behavior.

## Prerequisites

For the source build path, you need:

- a local checkout of this repository
- Rust 1.85 or newer with Cargo; Rust 1.85 is the minimum compiler version verified for the current workspace
- a shell that can run Cargo and local executables

For the next setup stage, you also need:

- a local `Product Repository` directory
- a separate `Harness Runtime Home`
- Codex, Claude Code, or another MCP host when you are ready to connect an agent host

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

Working directory: Harness Server source repository root after the quick local build.

```sh
target/debug/harness --version
target/debug/harness agent --help
target/debug/harness-mcp --version
target/debug/harness-mcp --help
```

The version commands print `harness <version>` and `harness-mcp <version>`. The help commands should print the `harness agent` command family and the integration-bound `harness-mcp --integration <integration_id>` process usage.

## Executable Discovery During Setup

`harness agent install` installs or exports host configuration that starts `harness-mcp --integration <integration_id>`.

For user-scope Codex or user/local-scope Claude Code setup, pass an existing absolute executable path with `--mcp-command /absolute/path/to/harness-mcp`, or put `harness-mcp` beside `harness` or on `PATH` so the CLI can discover it.

For project-scoped Codex or Claude Code setup, the generated project file must remain shareable. Use `--mcp-command harness-mcp` or omit `--mcp-command`, and make sure the host environment can find `harness-mcp` on `PATH`.

Installation location is not runtime state. Harness Server source or installation files contain executables, `Harness Runtime Home` contains Harness runtime records, `Product Repository` contains product files and selected project-scoped integration files, and the agent host owns its actual configuration and trust state.

## Next Step

Continue to [Quickstart](quickstart.md). It starts from a real supported host path for Codex or Claude Code.

Exact command behavior belongs to [Administrative CLI](../reference/admin-cli.md). Exact `harness-mcp` startup, environment, stdio transport, preflight, and shutdown behavior belongs to [MCP Transport](../reference/mcp-transport.md).
