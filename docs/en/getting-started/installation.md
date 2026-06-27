# Installation

This tutorial prepares the Volicord executables for first host setup. It covers choosing an executable source, verifying `volicord` and `volicord-mcp`, and recognizing when the selected binaries are ready for [Quickstart](quickstart.md).

It does not define public API behavior, storage effects, `Product Repository` registration, host trust, or MCP wire behavior.

## Audience, Goal, And Completion

Audience: first-time users, operators, or implementers who need a working local `volicord` administrative CLI and `volicord-mcp` MCP adapter before connecting an agent host.

Goal: select either a source-build output directory or a separately installed executable directory, then prove that both executables can run from the same POSIX-style shell.

Completion state: `VOLICORD_BIN` names one absolute directory containing executable `volicord` and `volicord-mcp` files, and the version/help checks below all succeed. That means the executables are ready for Agent Connection setup. It does not mean a `Volicord Runtime Home`, `Product Repository`, or host configuration has been created.

## Prerequisites

Read [System Requirements](../reference/system-requirements.md) before choosing a path. The command examples on this page use POSIX-style shell syntax: `export`, `$(pwd)`, quoted variable expansion, inline `PATH=...`, and `test -x`.

Use one of these setup paths:

| Path | Use when | Before continuing |
|---|---|---|
| Source build | You have this repository checkout and want to build current workspace executables. | Rust 1.85 or newer with Cargo is available, and Cargo can resolve workspace dependencies. |
| Separately installed executables | You already have a Volicord installation directory. | One absolute directory contains both `volicord` and `volicord-mcp`. |

For the next setup stage, also have a local `Product Repository`, a separate `Volicord Runtime Home`, and a supported host path such as Codex or Claude Code.

## Path A: Build From Source

Working directory: Volicord source repository root.

Run non-mutating toolchain checks first:

```sh
cargo --version
rustc --version
```

If either command is unavailable, or if the selected Rust compiler is older than 1.85, stop and fix the toolchain before building.

For a debug build:

```sh
cargo build -p volicord-cli -p volicord-mcp
export VOLICORD_BIN="$(pwd)/target/debug"
```

For a release build:

```sh
cargo build --release -p volicord-cli -p volicord-mcp
export VOLICORD_BIN="$(pwd)/target/release"
```

Choose one build output for the rest of the shell session. The Cargo package names are `volicord-cli` and `volicord-mcp`; the executable names are `volicord` and `volicord-mcp`.

## Path B: Select Installed Executables

Use this path when the executables were installed separately from the source checkout:

```sh
export VOLICORD_BIN="/absolute/path/to/installed/bin"
```

Replace `/absolute/path/to/installed/bin` with the real absolute directory that contains both executables. Do not copy the example value literally.

## Verify The Selected Directory

From the same shell where `VOLICORD_BIN` is set:

```sh
test -x "$VOLICORD_BIN/volicord"
test -x "$VOLICORD_BIN/volicord-mcp"

"$VOLICORD_BIN/volicord" --version
"$VOLICORD_BIN/volicord" agent --help
"$VOLICORD_BIN/volicord-mcp" --version
"$VOLICORD_BIN/volicord-mcp" --help
```

The version commands print `volicord <version>` and `volicord-mcp <version>`. The help commands should show the `volicord agent connect` command family and `volicord-mcp --connection <connection_id>` process usage.

`VOLICORD_BIN` is only a shell convenience variable for these examples. Volicord does not read it as configuration, and it is not persisted into generated host configuration. If you open a new shell, set it again or use the absolute paths directly.

## How Host Setup Uses This Choice

`volicord agent connect` installs or exports host configuration that starts `volicord-mcp --connection <connection_id>`.

For user-scope Codex or user/local-scope Claude Code setup, pass the selected absolute executable path with `--mcp-command "$VOLICORD_BIN/volicord-mcp"`, or put `volicord-mcp` beside `volicord` or on `PATH` so the CLI can discover it. The persisted host configuration receives the resolved absolute command path, not the shell variable.

For project-scoped Codex or Claude Code setup, the generated project file must remain shareable. Run setup with `PATH="$VOLICORD_BIN:$PATH"` and omit `--mcp-command`. The project file keeps the portable command name, and the later host process must be able to find `volicord-mcp` on its own `PATH`.

Installation location is not runtime state. Volicord source or installation files contain executables, `Volicord Runtime Home` contains Volicord runtime records, `Product Repository` contains product files and selected project-scoped host configuration, and the agent host owns its actual configuration and trust state.

## Failure Routing

| Symptom | Safe next action | Route |
|---|---|---|
| `cargo` or `rustc` is unavailable. | Install or select Rust 1.85+ with Cargo, then rerun the preflight checks. | [System Requirements](../reference/system-requirements.md#toolchain-requirements) |
| Rust is older than 1.85. | Select a Rust 1.85+ toolchain before running `cargo build`. | [System Requirements](../reference/system-requirements.md#toolchain-requirements) |
| `cargo build` fails. | Read the Cargo diagnostic, fix the reported toolchain, dependency, or source issue, then rerun the same build command. Do not delete Runtime Homes or Product Repositories as a first response. | [System Requirements](../reference/system-requirements.md#toolchain-requirements) |
| `target/debug` or `target/release` does not contain both executables. | Confirm which build command succeeded, select the matching output directory, and rerun the `test -x` checks. | [System Requirements](../reference/system-requirements.md#executable-layout-and-discovery) |
| `test -x` or a help/version command fails. | Select the directory that actually contains runnable `volicord` and `volicord-mcp`, or repair executable permissions for the selected user. | [Agent Host Troubleshooting](../guides/agent-host-troubleshooting.md#missing-volicord-mcp) |
| `VOLICORD_BIN` points to the wrong directory. | Export the correct absolute directory in the same shell, then rerun every verification command. | [Agent Host Troubleshooting](../guides/agent-host-troubleshooting.md#wrong-absolute-mcp-command) |
| A later project-scoped host cannot find `volicord-mcp`. | Keep the project file portable and fix the host launch environment `PATH`. | [Agent Host Troubleshooting](../guides/agent-host-troubleshooting.md#portable-project-command-not-on-path) |

## Next Step

Continue to [Quickstart](quickstart.md) after all verification commands on this page succeed.

Exact command behavior belongs to [Administrative CLI](../reference/admin-cli.md). Exact `volicord-mcp` startup, environment, stdio transport, preflight, and shutdown behavior belongs to [MCP Transport](../reference/mcp-transport.md).
