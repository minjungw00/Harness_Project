# Runtime Architecture

## Document Role

세 공간, runtime home, Core 흐름, authority와 projection/reconcile 아키텍처.

## Owns

- Product Repository / Harness Server / Runtime Home canonical explanation
- runtime layers
- Core process model
- state transaction model
- artifact store architecture
- raw artifact vs state record vs Markdown report boundary summary
- projection outbox architecture
- reconcile flow
- validator runner placement
- adapter/sidecar boundary
- guarantee levels architecture
- failure and recovery flow overview

## Does Not Own

- tool별 schema
- DB DDL
- full CLI commands
- conformance fixtures
- surface별 addendum

## Sections

### Architecture Scope

TODO_REWRITE: Migrate the canonical three-space architecture from `docs/legacy-v1/03-architecture.md`.

### Product Repository

TODO_REWRITE: Describe the repository role and projection placement without making documents canonical state.

### Harness Server / Installation

TODO_REWRITE: Describe MCP server, Core, validator, connector, projector, and CLI placement.

### Harness Runtime Home

TODO_REWRITE: Describe runtime home authority, including `registry.sqlite`, `project.yaml`, `state.sqlite`, and artifacts at architecture level only.

### Core Process Model

TODO_CONTENT: Add process and transaction flow after kernel and MVP details are rewritten.

### Artifact And Projection Boundaries

TODO_REWRITE: Summarize raw artifact, state record, and Markdown report boundaries.

### Projection Outbox And Reconcile Flow

TODO_REWRITE: Migrate projection job and reconcile architecture at high level.

### Validator And Adapter Placement

TODO_REWRITE: Summarize validator runner, adapter, and sidecar boundaries without surface-specific cookbooks.

### Guarantee Levels

TODO_REWRITE: Migrate cooperative, detective, preventive, and isolated guarantee architecture.

### Failure And Recovery Overview

TODO_CONTENT: Add architecture-level recovery overview, leaving commands and fixtures to operations.
