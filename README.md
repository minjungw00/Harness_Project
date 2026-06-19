# Harness

This repository contains maintained documentation and executable Rust implementation for Harness, the local work-authority product/system for AI-assisted product work.

It includes:

- maintained English and Korean documentation
- a Cargo Rust workspace
- the `harness` administrative/bootstrap executable
- the `harness-mcp` local MCP stdio executable
- implementation, integration, and conformance test paths

Start by choosing a language:

- English: [`docs/en/README.md`](docs/en/README.md)
- 한국어: [`docs/ko/README.md`](docs/ko/README.md)
- Documentation directory guide: [`docs/README.md`](docs/README.md)

Repository organization:

- `docs/en/` contains maintained English documentation.
- `docs/ko/` contains maintained Korean documentation.
- `crates/` contains the Rust implementation crates.
- `tests/` contains Rust integration and conformance test crates.
- `docs/doc-index.yaml` is the canonical machine-readable owner route and paired-path metadata.
- `docs/terminology-map.yaml` is the terminology and identifier-preservation source of truth.
- `AGENTS.md` gives repository working rules for agents and maintainers.

Implementation and local runtime routes:

- Build architecture: [`docs/en/build/architecture.md`](docs/en/build/architecture.md)
- Administrative CLI: [`docs/en/reference/admin-cli.md`](docs/en/reference/admin-cli.md)
- MCP transport: [`docs/en/reference/mcp-transport.md`](docs/en/reference/mcp-transport.md)

README files and human-readable indexes are entry routes. For exact owner routing, start from `docs/doc-index.yaml`; for contract details, follow the language entry pages to the applicable owner document.
