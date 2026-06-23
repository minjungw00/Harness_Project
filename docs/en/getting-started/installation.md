# Installation

This tutorial prepares the Harness Server executables for first host setup. It
covers choosing an executable source, verifying `harness` and `harness-mcp`, and
recognizing when the selected binaries are ready for [Quickstart](quickstart.md).
It does not define package-manager distribution, operating-system support,
public API behavior, storage effects, `Product Repository` registration, host
trust, or MCP wire behavior.

## Audience, Goal, And Completion

Audience: first-time users, operators, or implementers who need a working local
`harness` administrative CLI and `harness-mcp` MCP adapter before connecting an
agent host.

Goal: select either a source-build output directory or a separately installed
executable directory, then prove that both executables can run from the same
POSIX-style shell.

Completion state: `HARNESS_BIN` names one absolute directory containing
executable `harness` and `harness-mcp` files, and the version/help checks below
all succeed. That means the executables are ready for host setup. It does not
mean a `Harness Runtime Home`, `Product Repository`, or host configuration has
been created.

## Prerequisites

Read [System Requirements](../reference/system-requirements.md) before choosing
a path. The command examples on this page use POSIX-style shell syntax:
`export`, `$(pwd)`, quoted variable expansion, inline `PATH=...`, and `test -x`.
If your shell cannot run that syntax, translate the examples yourself and verify
each translated command before continuing.

Use one of these setup paths:

| Path | Use when | Before continuing |
|---|---|---|
| Source build | You have this repository checkout and want to build current workspace executables. | Rust 1.85 or newer with Cargo is available, and Cargo can resolve workspace dependencies. |
| Separately installed executables | You already have a Harness Server installation directory. | One absolute directory contains both `harness` and `harness-mcp`. |

For the next setup stage, also have a local `Product Repository`, a separate
`Harness Runtime Home`, and a supported host path such as Codex or Claude Code.

## Path A: Build From Source

Working directory: Harness Server source repository root.

Run non-mutating toolchain checks first:

```sh
cargo --version
rustc --version
```

If either command is unavailable, or if the selected Rust compiler is older than
1.85, stop and fix the toolchain before building.

For a debug build:

```sh
cargo build -p harness-cli -p harness-mcp
export HARNESS_BIN="$(pwd)/target/debug"
```

For a release build:

```sh
cargo build --release -p harness-cli -p harness-mcp
export HARNESS_BIN="$(pwd)/target/release"
```

Choose one build output for the rest of the shell session. The Cargo package
names are `harness-cli` and `harness-mcp`; the executable names are `harness`
and `harness-mcp`.

## Path B: Select Installed Executables

Use this path when the executables were installed separately from the source
checkout:

```sh
export HARNESS_BIN="/absolute/path/to/installed/bin"
```

Replace `/absolute/path/to/installed/bin` with the real absolute directory that
contains both executables. Do not copy the example value literally.

## Verify The Selected Directory

From the same shell where `HARNESS_BIN` is set:

```sh
test -x "$HARNESS_BIN/harness"
test -x "$HARNESS_BIN/harness-mcp"

"$HARNESS_BIN/harness" --version
"$HARNESS_BIN/harness" agent --help
"$HARNESS_BIN/harness-mcp" --version
"$HARNESS_BIN/harness-mcp" --help
```

The version commands print `harness <version>` and `harness-mcp <version>`. The
help commands should show the `harness agent` command family and the
integration-bound `harness-mcp --integration <integration_id>` process usage.

`HARNESS_BIN` is only a shell convenience variable for these examples. Harness
does not read it as configuration, and it is not persisted into generated host
configuration. If you open a new shell, set it again or use the absolute paths
directly.

## How Host Setup Uses This Choice

`harness agent install` installs or exports host configuration that starts
`harness-mcp --integration <integration_id>`.

For user-scope Codex or user/local-scope Claude Code setup, pass the selected
absolute executable path with `--mcp-command "$HARNESS_BIN/harness-mcp"`, or put
`harness-mcp` beside `harness` or on `PATH` so the CLI can discover it. The
persisted host configuration receives the resolved absolute command path, not
the shell variable.

For project-scoped Codex or Claude Code setup, the generated project file must
remain shareable. Run setup with `PATH="$HARNESS_BIN:$PATH"` and use
`--mcp-command harness-mcp` or omit `--mcp-command`. The project file keeps the
portable command name, and the later host process must be able to find
`harness-mcp` on its own `PATH`.

Installation location is not runtime state. Harness Server source or
installation files contain executables, `Harness Runtime Home` contains Harness
runtime records, `Product Repository` contains product files and selected
project-scoped integration files, and the agent host owns its actual
configuration and trust state.

## Failure Routing

| Symptom | Safe next action | Route |
|---|---|---|
| `cargo` or `rustc` is unavailable. | Install or select Rust 1.85+ with Cargo, then rerun the preflight checks. | [System Requirements](../reference/system-requirements.md#toolchain-requirements) |
| Rust is older than 1.85. | Select a Rust 1.85+ toolchain before running `cargo build`. | [System Requirements](../reference/system-requirements.md#toolchain-requirements) |
| `cargo build` fails. | Read the Cargo diagnostic, fix the reported toolchain, dependency, or source issue, then rerun the same build command. Do not delete Runtime Homes or Product Repositories as a first response. | [System Requirements](../reference/system-requirements.md#toolchain-requirements) |
| `target/debug` or `target/release` does not contain both executables. | Confirm which build command succeeded, select the matching output directory, and rerun the `test -x` checks. | [System Requirements](../reference/system-requirements.md#executable-layout-and-discovery) |
| `test -x` or a help/version command fails. | Select the directory that actually contains runnable `harness` and `harness-mcp`, or repair executable permissions for the selected user. | [Agent Host Troubleshooting](../guides/agent-host-troubleshooting.md#missing-harness-mcp) |
| `HARNESS_BIN` points to the wrong directory. | Export the correct absolute directory in the same shell, then rerun every verification command. | [Agent Host Troubleshooting](../guides/agent-host-troubleshooting.md#wrong-absolute-mcp-command) |
| A later project-scoped host cannot find `harness-mcp`. | Keep the project file portable and fix the host launch environment `PATH`. | [Agent Host Troubleshooting](../guides/agent-host-troubleshooting.md#portable-project-command-not-on-path) |

## Next Step

Continue to [Quickstart](quickstart.md) after all verification commands on this
page succeed.

Exact command behavior belongs to
[Administrative CLI](../reference/admin-cli.md). Exact `harness-mcp` startup,
environment, stdio transport, preflight, and shutdown behavior belongs to
[MCP Transport](../reference/mcp-transport.md).
