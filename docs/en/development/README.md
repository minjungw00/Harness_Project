# Developer documentation

This is the source-code learning entry point for developers who want to
understand the current Rust implementation. It explains where to start, what to
read next, and where exact product contracts live.

These pages teach implementation structure. They do not define or override
public API behavior, request or response schemas, storage effects, security
guarantees, runtime boundaries, Core authority semantics, or product contracts.
For exact behavior, follow the links to the focused Reference owners.

Harness is the local work-authority product/system for AI-assisted product
work. Core is the local authority record for Harness state.

## Reading order

1. Workspace and crate responsibilities: start with the
   [Codebase Tour](codebase-tour.md). It names every Cargo workspace member,
   the first source file to open, important symbols, relevant tests, and the
   next component to read.
2. Representative request flow: read the
   [Request Lifecycle](request-lifecycle.md). It follows `harness.status`,
   `harness.intake`, and `harness.prepare_write` from MCP `tools/call` through
   Core and Store behavior to the MCP response wrapper.
3. Architecture and boundaries: use
   [Implementation Architecture](architecture.md) for the durable workspace
   shape, dependency direction, execution-flow maps, effect paths, commit
   boundaries, administrative CLI setup flow, and test topology.
4. Storage and transaction concepts: read the architecture sections on
   [Core pipeline and Store boundary](architecture.md#core-pipeline-and-store-boundary)
   and [Effect and commit boundaries](architecture.md#effect-and-commit-boundaries),
   then route exact storage questions to [Storage](../reference/storage.md),
   [Storage Effects](../reference/storage-effects.md),
   [Storage Records](../reference/storage-records.md),
   [Storage DDL](../reference/storage-ddl.md), and
   [Storage Versioning](../reference/storage-versioning.md).
5. Test topology: use the tour's per-member test pointers and the architecture
   [Test topology](architecture.md#test-topology). The main paths are
   `crates/harness-core/src/methods/tests.rs`,
   `crates/harness-mcp/tests/binary_transport.rs`,
   `crates/harness-cli/tests/binary_admin.rs`,
   `tests/integration/mcp_surface.rs`, and
   `tests/conformance/baseline.rs`.
6. Change workflow: use the [Implementation Guide](change-guide.md) when you
   are ready to classify a change, locate the owner document, inspect the
   implementation boundary, and choose validation.
7. Exact Reference contracts: use the
   [Reference Index](../reference/README.md) and
   [API Methods](../reference/api/methods.md). Learning documents can explain
   how the current code is arranged, but Reference documents own exact method
   behavior, schemas, storage effects, security wording, runtime boundaries,
   error meaning, and Core authority semantics.

## Learning documents versus owners

| Question | Start here | Exact owner route |
|---|---|---|
| Which crate should I open first? | [Codebase Tour](codebase-tour.md) | [Implementation Architecture](architecture.md) owns guide-level implementation structure. |
| How does a method call move through MCP, Core, Store, and back? | [Request Lifecycle](request-lifecycle.md) | Method behavior is owned by [API Methods](../reference/api/methods.md) and the linked method owner. |
| Why does Core not depend on CLI or MCP? | [Implementation Architecture](architecture.md) | Dependency-boundary guidance stays in architecture; public behavior still routes to Reference owners. |
| What storage mutation is committed? | [Request Lifecycle](request-lifecycle.md) and [Implementation Architecture](architecture.md) | Exact storage effects route to [Storage Effects](../reference/storage-effects.md) and adjacent storage owners. |
| What should I edit for a change? | [Implementation Guide](change-guide.md) | The focused Reference owner selected from [Reference Index](../reference/README.md) or `docs/doc-index.yaml`. |

## Source-reading shortcuts

For public method work, the shortest useful source path is:

1. [`crates/harness-types/src/methods.rs`](../../../crates/harness-types/src/methods.rs)
2. [`crates/harness-mcp/src/lib.rs`](../../../crates/harness-mcp/src/lib.rs)
3. [`crates/harness-core/src/pipeline.rs`](../../../crates/harness-core/src/pipeline.rs)
4. [`crates/harness-core/src/methods/`](../../../crates/harness-core/src/methods/)
5. [`crates/harness-store/src/core_pipeline.rs`](../../../crates/harness-store/src/core_pipeline.rs)
6. [`tests/integration/mcp_surface.rs`](../../../tests/integration/mcp_surface.rs)
7. [`tests/conformance/baseline.rs`](../../../tests/conformance/baseline.rs)

For local setup and operator behavior, start instead with
[`crates/harness-cli/src/main.rs`](../../../crates/harness-cli/src/main.rs),
then
[`crates/harness-cli/src/local_mcp_command.rs`](../../../crates/harness-cli/src/local_mcp_command.rs)
and [`crates/harness-cli/src/setup.rs`](../../../crates/harness-cli/src/setup.rs).

## Boundary reminders

- Core-facing code is independent of CLI and MCP adapter crates.
- `harness-mcp` may use Store directly for startup and session validation. That
  direct Store use is not alternate public-method semantics.
- `Harness Runtime Home` and `Product Repository` are separate locations.
- Tests verify owner-defined facts, but tests and fixtures are not product
  contract owners.
- Learning pages should name source files and symbols, not unstable line
  numbers.
