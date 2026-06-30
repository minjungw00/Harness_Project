# Installation

This tutorial prepares the local `volicord` executable and records the
installation profile used by later project, connection, export, MCP, and
`User Channel` commands. It is the setup step before the
[Quickstart](quickstart.md).

Exact command behavior belongs to
[Administrative CLI Reference](../reference/admin-cli.md). Runtime location and
repository separation belong to [Runtime Boundaries](../reference/runtime-boundaries.md).

## Prerequisites

- Rust 1.85 or newer, as listed in
  [System Requirements](../reference/system-requirements.md).
- A shell that can run Cargo and local binaries.
- A Git repository to use as the Product Repository when you are ready to
  connect a host.

## Build From Source

From the Volicord source repository:

```sh
cargo build --workspace --bins
```

This builds the local executable:

- `./target/debug/volicord`

Then run guided setup from the freshly built CLI:

```sh
./target/debug/volicord setup
```

`volicord setup` creates or verifies the selected `Volicord Runtime Home` and
saves the installation profile. It discovers the running `volicord` executable,
stores the MCP launch command, and checks whether the selected command is
available on `PATH` for future terminals and agent hosts. Exact setup options,
MCP launch command behavior, and output behavior belong to
[Administrative CLI Reference](../reference/admin-cli.md#runtime-home-selection).
Its status answers whether the guided first-run setup experience still needs a
named user action, so `action_required` can appear even after the installation
profile has been saved.

In an interactive terminal, setup may offer command-availability choices when
the selected executable is not ready on `PATH`:

- create command links in a suggested directory that setup can verify is
  writable
- create a conventional user command directory such as `~/.local/bin` when it
  is missing and safe to create, then verify writability before linking
- create command links and, after explicit approval, add a managed `PATH` block
  to a supported shell startup file
- create command links and print the shell command to run yourself
- print a shell command for manual `PATH` repair
- skip command linking for now

Shell startup file changes are never implicit. If setup can identify a
supported shell startup file, it shows the target file and managed block and
asks for approval before writing. The managed block is Volicord-owned and does
not rewrite unrelated shell configuration. Unsupported shells or platforms
require manual action.

Setup cannot change the parent shell's current `PATH`. A printed
`export PATH=...` command affects only the terminal where you run it. If setup
writes or asks you to update a shell startup file, open a new shell or restart
or reload existing agent host processes before expecting them to see the
commands.

For automation or deterministic local layouts, use explicit setup options:

| Option | When to use it |
|---|---|
| `--link-bin PATH` | Create the directory if needed, verify it is writable, then create or update command links there. This does not by itself edit shell startup files. |
| `--mcp-command PATH` | Store a specific `volicord` command for generated MCP launch entries when setup should not use the running executable. |
| `--home PATH` | Select a non-default `Volicord Runtime Home`. |

For example, a noninteractive link step can choose the link directory:

```sh
./target/debug/volicord setup --link-bin ~/.local/bin
```

After completing any prompt or action-required command-availability step, check
setup readiness:

```sh
volicord doctor
```

`doctor` reports installation-profile health, not first-run setup progress. It
reports `complete` when the saved profile is usable, even if it also reports
command-availability warnings or recommended `PATH` and command-link actions
for future shells or agent hosts. `action_required` names a blocking local
repair action, such as rerunning setup or fixing an executable path.

## Use Installed Executables

If `volicord` already exists on `PATH`, run:

```sh
volicord setup
volicord doctor
```

Setup uses the same installation-profile contract whether the executable came
from a source build or an installed command directory. Use
`volicord setup --mcp-command PATH` only when generated host configuration
should start MCP through a different `volicord` command path.
If setup reports `action_required`, complete the named local action before
starting new terminals or agent hosts. Ordinary `volicord connect` commands use
the saved installation profile.

## Docker Image

Docker support is for local container layouts and localhost MCP access. Build
the image from the Volicord source repository:

```sh
docker build -t volicord:local .
```

Use a Runtime Home volume and mount the Product Repository at the same container
path whenever you run setup, project, connection, and serve commands. Project
registrations store repository roots, so a Runtime Home prepared for one path
layout should not be reused with a different container workspace path.

For example, prepare or inspect the Docker Runtime Home with the same mounts:

```sh
docker run --rm -it \
  -v volicord-home:/var/lib/volicord \
  -v "$PWD:/workspace" \
  volicord:local setup
```

After the Runtime Home contains the project registration and Agent Connection
you want to serve, start the local HTTP MCP endpoint with an operator-provided
token:

```sh
VOLICORD_HTTP_TOKEN="$(openssl rand -hex 32)"
docker run --rm \
  -p 127.0.0.1:8765:8765 \
  -v volicord-home:/var/lib/volicord \
  -v "$PWD:/workspace" \
  volicord:local serve --transport streamable-http \
    --listen 0.0.0.0:8765 \
    --allow-nonlocal-listen \
    --token "$VOLICORD_HTTP_TOKEN" \
    --project /workspace
```

The container listens on `0.0.0.0` only inside Docker so Docker can publish the
port. The host publish address remains `127.0.0.1`, and Volicord still requires
`--allow-nonlocal-listen` plus bearer authentication. Do not store
`VOLICORD_HTTP_TOKEN` in repository files.

## What Setup Does Not Do

Setup does not register a Product Repository and does not install host
configuration. Project registration happens when you run `volicord project use`
or a command such as `volicord connect` from inside a Git repository.

Project naming and internal identity behavior are owned by the
[Administrative CLI Reference](../reference/admin-cli.md#project-commands).
Internal identities are stored by Volicord and are not first-time setup inputs.

## Next Step

Move into the Product Repository and connect a host:

```sh
cd /path/to/your-product-repo
volicord connect codex
```

`/path/to/your-product-repo` is an example path for the Product Repository where
you want the agent to work.

For the full first-run path, continue with the [Quickstart](quickstart.md). For
host-specific details, see [Agent Host Setup](../guides/agent-host-setup.md).
