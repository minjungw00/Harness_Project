# Harness Documentation Directory

This directory contains the maintained English and Korean Harness documentation plus the metadata used by maintainers for owner routing and terminology control.

Choose a language first:

- English: [en/README.md](en/README.md)
- Korean: [ko/README.md](ko/README.md)

Fast reader routes:

- Product orientation: [English](en/getting-started/overview.md) / [Korean](ko/getting-started/overview.md)
- Installation: [English](en/getting-started/installation.md) / [Korean](ko/getting-started/installation.md)
- Local MCP quickstart: [English](en/getting-started/quickstart.md) / [Korean](ko/getting-started/quickstart.md)
- Detailed local MCP setup: [English](en/guides/local-mcp-setup.md) / [Korean](ko/guides/local-mcp-setup.md)
- Reference navigation: [English](en/reference/README.md) / [Korean](ko/reference/README.md)

Shared metadata:

- [doc-index.yaml](doc-index.yaml) is the canonical machine-readable route for documentation owners and paired paths.
- [terminology-map.yaml](terminology-map.yaml) is the terminology and identifier-preservation source of truth.

Those metadata files support maintainers, translators, and agents. Ordinary readers should start from the language entry pages, Getting Started, Guides, Development, or Reference pages.

Maintainers and implementation agents should read [../AGENTS.md](../AGENTS.md), then the relevant documentation, translation, and validation policies under `docs/*/maintain/`.

This README is an entry route. It does not define API behavior, storage effects, security guarantees, schemas, Core authority semantics, or detailed owner maps.
