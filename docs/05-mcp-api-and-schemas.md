# MCP API And Schemas

## Document Role

MCP resource/tool, schema, errors, idempotency, validator result schema의 소유 문서.

## Owns

- MCP resources
- public MCP tools
- common envelope
- tool별 request schema
- tool별 response schema
- state transition summary per tool
- events emitted per tool
- projection jobs enqueued per tool
- error code taxonomy
- idempotency and retry
- state conflict behavior
- validator result schema
- artifact ref schema

## Does Not Own

- why strategy
- full state transition table
- SQLite DDL implementation details
- user-facing conversation examples

## Sections

### API Scope

TODO_REWRITE: Migrate public MCP surface material from `docs/legacy-v1/04-reference-implementation.md`.

### Resources

TODO_CONTENT: Add MCP resource list after schema rewrite.

### Common Tool Envelope

TODO_REWRITE: Define the state-changing tool envelope and idempotency fields.

### Public Tools

TODO_REWRITE: Add the canonical public tool list and per-tool skeletons.

### Request Schemas

TODO_CONTENT: Add request schemas in Batch C.

### Response Schemas

TODO_CONTENT: Add response schemas in Batch C.

### Errors And Conflicts

TODO_REWRITE: Migrate error taxonomy, state conflict behavior, and retry semantics.

### Events And Projection Jobs

TODO_CONTENT: Record emitted events and projection jobs per tool after kernel transitions are fixed.

### Validator Result Schema

TODO_REWRITE: Migrate validator result schema ownership here.

### Artifact Reference Schema

TODO_REWRITE: Migrate artifact ref payload shape here.
