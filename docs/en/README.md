# Harness Documentation

This is the English routing page for the Harness documentation set.

This repository is currently a documentation-only redesign/review repository. After documentation acceptance, it is intended to become the Harness Server source repository. No Harness Server/runtime implementation exists here yet. These docs are source material for understanding and implementing Harness; they are not Harness runtime objects governed by the lifecycle they describe.

Harness is a local authority record and judgment-routing layer for AI-assisted product work. It keeps scope, user-owned judgments, evidence, verification, QA expectations, final acceptance, and residual-risk status outside fragile chat context.

Harness solves four recurring problems:

- Scope drifts or becomes implicit.
- User-owned judgment is silently replaced by agent judgment.
- Evidence, verification, QA, and completion claims get mixed.
- Chat or Markdown output is mistaken for operational truth.

## Primary Reader Path

Use this path when you do not know where to start:

1. [Overview](learn/overview.md) for the first mental model.
2. [User Guide](use/user-guide.md) for how to interact with Harness during work.
3. [Concepts](learn/concepts.md) for the vocabulary that appears in examples, status, and specs.
4. [Implementation Overview](build/implementation-overview.md) and [MVP Plan](build/mvp-plan.md) when you are reviewing or building the server plan.
5. [Reference](#reference) only when you need exact contracts, schemas, gates, storage, projection, security, or template details.

## Reader Paths By Role

| Reader | Start | Then use |
|---|---|---|
| User | [Overview](learn/overview.md) | [User Guide](use/user-guide.md), [Concepts](learn/concepts.md), then [Decision Packet Cookbook](use/decision-packet-cookbook.md) only when decisions get complex. |
| Agent integrator | [Overview](learn/overview.md) | [User Guide](use/user-guide.md), [Agent Session Flow](use/agent-session-flow.md), [Agent Integration Reference](reference/agent-integration.md), [Surface Cookbook](reference/surface-cookbook.md), and [MCP API And Schemas](reference/mcp-api-and-schemas.md). |
| Implementer | [Overview](learn/overview.md) | [Concepts](learn/concepts.md), [Implementation Overview](build/implementation-overview.md), [First Runnable Slice](build/first-runnable-slice.md), [MVP Plan](build/mvp-plan.md), [Runtime Walkthrough](build/runtime-walkthrough.md), then the relevant Reference owner. |
| Reviewer / maintainer | [Overview](learn/overview.md) | [Authoring Guide](maintain/authoring-guide.md), [Translation Guide](maintain/translation-guide.md), [Roadmap](roadmap.md), and Reference owners when checking strict meaning. |

Operators and conformance authors usually begin in Reference: [Operations And Conformance Reference](reference/operations-and-conformance.md), [Conformance Fixtures Reference](reference/conformance-fixtures.md), [Runtime Architecture Reference](reference/runtime-architecture.md), [Security Threat Model Reference](reference/security-threat-model.md), [MCP API And Schemas](reference/mcp-api-and-schemas.md), [Storage And DDL](reference/storage-and-ddl.md), and [Kernel Reference](reference/kernel.md).

## Document Roles

The Learn and Use pages are kept separate, but each has a narrower job:

| Page | Role |
|---|---|
| [Overview](learn/overview.md) | Primary first read. Explains the product thesis, the three spaces, what Harness records, and what Harness is not. |
| [Purpose and Principles](learn/purpose-and-principles.md) | Values, non-goals, failure model, and MVP boundary. Use it when reviewing whether wording or scope still matches the thesis. |
| [Concepts](learn/concepts.md) | Vocabulary bridge from ordinary user language to implementation terms. It is not another overview or tutorial. |
| [Harness in 15 Minutes](learn/harness-in-15-minutes.md) | Scenario sampler. Six short examples show common Harness moments before strict specs. |
| [Harness in One Task](learn/harness-in-one-task.md) | Tutorial walkthrough. One small change and one tracked task show the full work journey. |
| [User Guide](use/user-guide.md) | Primary user-facing entry for starting, resuming, unblocking, and closing work. |
| [Decision Packet Cookbook](use/decision-packet-cookbook.md) | Advanced usage and reference-adjacent examples for writing focused user-decision prompts. |
| [Agent Session Flow](use/agent-session-flow.md) | Agent/integration guidance for presentation, context, blockers, writes, and close. It is not a required user read. |

## Where Am I?

Harness keeps three spaces separate:

| Space | What belongs there |
|---|---|
| Product Repository | The user's product workspace: product code, tests, product docs, and human-readable Harness projections. |
| Harness Server source repository | The future codebase for the local Harness Server / Installation: API surface, request validation, Core state transitions, validators, projection, reconcile, and operator tools. |
| Harness Runtime Home | Per-user/per-installation operational data: state database, artifact store, projection output, logs, and local registration/configuration. |

This repository's current role is documentation review/redesign. Its intended future role is the Harness Server source repository. It is not the Product Repository or the Harness Runtime Home. After documentation acceptance, the Harness Server / Installation implementation is expected to be built here.

## Documentation Redesign Scope

Documentation acceptance and implementation-planning status are tracked in [Implementation Overview](build/implementation-overview.md#documentation-acceptance-status).

The redesign may change terminology, MVP staging, schema structure, projection structure, security wording, and document organization. Preserve the clarified product thesis and feasible implementation path over continuity with existing prose.

The [Authoring Guide](maintain/authoring-guide.md#current-redesign-scope) owns the full redesign scope, preserved principles, document-family guidance, and [known redesign issues tracker](maintain/authoring-guide.md#known-redesign-issues-tracker).

## What Harness Is Not

Harness is not the same kind of thing as agent instructions, MCP, reusable workflows, tests, review, or specs. Those pieces can be useful around Harness, but they do not become the local operational record or the owner of user judgment.

Harness is also not a prompt pack, chat script, evaluation harness, dashboard, or broad hosted agent platform.

## Comparison

| Nearby piece | Role it plays | Harness role |
|---|---|---|
| AGENTS.md / agent instruction files | Tell agents how to behave in a repository or session. | Harness may rely on those instructions, but it keeps the local record of scope, judgment, evidence, close readiness, and risk. |
| MCP | Defines a protocol boundary for tools and resources. | Harness may expose MCP tools or resources, but its authority comes from Core-owned local state and artifact references. |
| Skills / reusable workflows | Package repeated instructions or procedures for an agent to follow. | Harness can be used by those workflows, but it records the current work state and routes judgments for this task. |
| Test runners | Execute checks and produce results. | Harness links relevant results as evidence and keeps verification strength separate from final acceptance. |
| Code review | Provides human or team review of changes. | Harness can reference review outcomes, but it does not replace review or turn review into final acceptance, residual-risk acceptance, or close. |
| Specs | Describe intended behavior, design, or constraints. | Harness can use specs as input, but it records operational state for live work: scope, decisions, evidence, QA expectations, final acceptance, and remaining risk. |

## Ownership Rule

Reference docs own exact contracts: schemas, DDL, gates, state transitions, enum values, fixture semantics, template bodies, and official definitions. Learn, Use, and Build docs explain the idea for their reader and link to Reference instead of copying strict contract blocks.

Documentation-maintenance checks are editorial quality checks for drift, owner boundaries, links, and language parity. They are not runtime conformance or implementation readiness. Use the [Authoring Guide](maintain/authoring-guide.md#docs-maintenance-checks) for drift categories and owner-first resolution; use [Operations And Conformance](reference/operations-and-conformance.md#docs-maintenance-profile) only for the docs-maintenance profile reporting boundary.

## Learn

Use Learn when you want the mental model before exact contracts.

- [Overview](learn/overview.md)
- [Purpose and Principles](learn/purpose-and-principles.md)
- [Concepts](learn/concepts.md)
- [Harness in 15 Minutes](learn/harness-in-15-minutes.md)
- [Harness in One Task](learn/harness-in-one-task.md)

## Use

Use this path when you want to run an AI-assisted development session under Harness. The primary user-facing entry is [User Guide](use/user-guide.md). [Decision Packet Cookbook](use/decision-packet-cookbook.md) is for advanced decision examples. [Agent Session Flow](use/agent-session-flow.md) is agent/integration guidance, not a required user read.

- [User Guide](use/user-guide.md)
- [Decision Packet Cookbook](use/decision-packet-cookbook.md)
- [Agent Session Flow](use/agent-session-flow.md)

## Build

Use this path for implementation orientation and planning review. Start with the [Documentation Acceptance Status](build/implementation-overview.md#documentation-acceptance-status). Until maintainers deliberately accept implementation planning there, Build pages remain planning guidance and do not authorize runtime/server implementation.

- [Implementation Overview](build/implementation-overview.md)
- [First Runnable Slice](build/first-runnable-slice.md)
- [Runtime Walkthrough](build/runtime-walkthrough.md)
- [MVP Plan](build/mvp-plan.md)

## Reference

Use this path to look up strict contracts. If another path summarizes a strict rule, update the Reference owner first.

- [Kernel Reference](reference/kernel.md)
- [Runtime Architecture Reference](reference/runtime-architecture.md)
- [Security Threat Model Reference](reference/security-threat-model.md)
- [MCP API And Schemas](reference/mcp-api-and-schemas.md)
- [Storage And DDL](reference/storage-and-ddl.md)
- [Document Projection Reference](reference/document-projection.md)
- [Design Quality Policies](reference/design-quality-policies.md)
- [Agent Integration Reference](reference/agent-integration.md)
- [Surface Cookbook](reference/surface-cookbook.md)
- [Operations And Conformance Reference](reference/operations-and-conformance.md)
- [Conformance Fixtures Reference](reference/conformance-fixtures.md)
- [Glossary Reference](reference/glossary.md)
- [Template Reference](reference/templates/README.md)

## Maintain

Use this path to keep the docs and future Harness system coherent over time. Maintain docs govern documentation maintenance, not runtime behavior.

- [Authoring Guide](maintain/authoring-guide.md)
- [Translation Guide](maintain/translation-guide.md)

## Roadmap

- [Roadmap](roadmap.md)

Post-MVP items live in the roadmap. The roadmap is not part of Build-owned staged delivery unless a future owner explicitly promotes an item with scope, fixtures, and fallback behavior.

## Language Parity

The English and Korean documentation sets keep the same file map and semantic content. Korean headings and prose may be natural Korean rather than sentence-by-sentence mirrors of English.
