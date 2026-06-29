# Installation

This tutorial prepares the local `volicord` and `volicord-mcp` executables and
records the setup profile used by later project, connection, export, and
`User Channel` commands. It is the setup step before the
[Quickstart](quickstart.md).

Exact command behavior belongs to
[Administrative CLI Reference](../reference/admin-cli.md). Runtime location and
repository separation belong to [Runtime Boundaries](../reference/runtime-boundaries.md).

## Prerequisites

- Rust 1.85 or newer, as listed in
  [System Requirements](../reference/system-requirements.md).
- A shell that can run Cargo and local binaries.
- A product repository that is a Git repository when you are ready to connect a
  host.

## Build From Source

From the Volicord source repository:

```sh
cargo build --workspace --bins
```

This builds both local executables:

- `./target/debug/volicord`
- `./target/debug/volicord-mcp`

Then create the setup profile:

```sh
./target/debug/volicord setup --link-bin ~/.local/bin
```

`volicord setup` prepares the default `Volicord Runtime Home`, discovers
`volicord-mcp`, records the setup profile, and links the `volicord` command when
`--link-bin` is supplied. Add `~/.local/bin` to your shell `PATH` if it is not
already there.

Check setup readiness:

```sh
volicord doctor
```

`doctor` reports `complete` when the Runtime Home, setup profile, and MCP
command are ready. `action_required` means the command found a specific local
repair action, such as rerunning setup or fixing an executable path.

## Use Installed Executables

If `volicord` and `volicord-mcp` already exist on `PATH`, run:

```sh
volicord setup
volicord doctor
```

Setup discovers the MCP command from the running installation. Use setup options
only when you intentionally need a non-default Runtime Home or MCP executable
location. Ordinary first-time connection commands do not require project ids,
internal host or registry values.

## What Setup Does Not Do

Setup does not register a product repository and does not install host
configuration. Project registration happens when you run `volicord project use`
or a command such as `volicord connect` from inside a Git repository.

The repository project name is derived from the repository directory and made
unique inside the selected Runtime Home when needed. Internal ids are stored by
Volicord and are not first-time setup inputs.

## Next Step

Move into the product repository and connect a host:

```sh
cd /work/acme-api
volicord connect codex
```

For the full first-run path, continue with the [Quickstart](quickstart.md). For
host-specific details, see [Agent Host Setup](../guides/agent-host-setup.md).
