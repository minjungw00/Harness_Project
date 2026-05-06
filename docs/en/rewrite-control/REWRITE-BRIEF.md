# Harness Documentation Rewrite Brief

## 1. Mission

Restructure the harness documentation set into an **implementable local operating kernel specification**.

This work is not copyediting or simple relocation of existing text. Existing documents are design material, but the document structure and canonical ownership are being fixed anew. Rewrite body text from scratch where needed.

The target output is this combination.

```text
small implementable harness kernel
+ closed state/gate model
+ explicit MCP API and schemas
+ reference MVP implementation plan
+ design-quality policy pack
+ lightweight user and integration docs
+ fixture-based conformance
```

## 2. Problem Being Fixed

The current documentation set captures the harness's core direction well. Preserve the three-space execution model, source-of-truth/projection separation, narrowed public MCP tool surface, SQLite-centered runtime, MVP/later split, and separation of approval/assurance/manual QA/acceptance.

However, the current documents leave these problems.

```text
- Strategy, state model, API/schema, implementation contract, templates, connectors, and operating criteria are overcrowded inside single documents.
- Many good design principles all look like core invariants.
- The boundary is blurry between the kernel that MVP must actually implement and later automation.
- State transitions, gates, waivers, and close semantics leave room for implementer interpretation.
- Wording for event log, User Notes, and Domain Language authority shifts across documents.
- MCP tool names are organized, but per-tool request/response schemas are not closed enough.
- Conformance is not concrete enough as fixture-based implementation testing.
```

## 3. Primary Goal

The new documentation set must answer these questions precisely.

```text
Users:
  How do I use the harness?

Implementers:
  Can I build the MVP from these documents alone?

Connector authors:
  How do I attach a specific agent surface?

Operators:
  How do I verify installation, diagnostics, recovery, and conformance?

Design owners:
  How do I control whether AI works inside good design boundaries?
```

## 4. Non-negotiable Principles

The new documentation set does not weaken these principles.

```text
1. Chat is not state.
2. Product write requires an active scoped Change Unit.
3. Sensitive change requires explicit approval.
4. Completion requires evidence coverage where evidence is required.
5. Work cannot self-certify detached verification.
6. Required QA and acceptance are separate gates.
7. Projection cannot override canonical state.
```

## 5. Core Rewrite Thesis

Do not discard the good principles in the current documents. Separate their layers instead.

```text
Kernel:
  Task, Change Unit, lifecycle, gates, state transition, evidence, verification, close semantics

Policy Pack:
  shared design, domain language, vertical slice, TDD, module/interface review, manual QA, context hygiene

Projection:
  human-readable Markdown, managed blocks, human-editable areas, artifact refs

Integration:
  agent surface capability profile, connector contract, fallback semantics

Operations:
  doctor, recover, reconcile, export, fixture-based conformance
```

## 6. MVP Boundary

MVP is a **core invariant validation project**, not an agent surface integration project.

MVP aims for the following.

```text
- single local project registration
- one reference agent surface connection
- state storage based on state.sqlite
- state.sqlite.task_events append-only event table
- artifact registry
- public MCP tools
- prepare_write gate
- minimal implementation of approval/evidence/verification/manual QA/acceptance gates
- TASK, APR, RUN-SUMMARY, EVIDENCE-MANIFEST, EVAL, DIRECT-RESULT projection
- detached verification bundle or manual evaluator instruction bundle
- doctor, recover, reconcile, export, conformance smoke
```

MVP does not aim for the following.

```text
- completing every agent surface connector at once
- dashboard
- automatic browser QA capture
- cross-surface orchestration
- native hook coverage for every surface
- fully automated parallel execution
- long-term analytics
- team workflow system
```

## 7. Fixed Architectural Direction

The following direction is fixed.

```text
Product Repository:
  repository containing product code and human-readable projections

Harness Server / Installation:
  installation that runs the MCP server and Core

Harness Runtime Home:
  local operating home containing registry.sqlite, state.sqlite, and artifacts
```

`Product Repository` documents are human-readable projections. The canonical source for operational state is `state.sqlite` in the Runtime Home. The canonical source for raw evidence is the artifact store.

## 8. Fixed Rewrite Decisions

Detailed decisions are owned by `docs/rewrite-control/KERNEL-DECISIONS.md`. Summary:

```text
- In MVP, the event log is state.sqlite.task_events.
- Restructure the state model as lifecycle + gates.
- Separate the scope gate from the approval gate.
- A verification waiver is not detached_verified.
- Split User Notes authority into input surface / reconcile_items / accepted state.
- The Domain Language canonical source is state.sqlite.domain_terms.
- Design-quality is a policy pack, not a kernel invariant.
- Keep public MCP tools, but tighten schemas.
- Split templates into required / optional / appendix.
- Conformance is fixture-based.
```

## 9. New Document Layers

The new documentation set follows these layers.

```text
README.md
00-introduction.md
01-project-charter.md
02-strategy.md
03-kernel-spec.md
04-runtime-architecture.md
05-mcp-api-and-schemas.md
06-reference-mvp.md
07-document-projection.md
08-design-quality-policy-pack.md
09-agent-integration.md
10-user-guide.md
11-operations-and-conformance.md
99-authoring-guide.md
glossary.md

appendix/
  A-template-library.md
  B-surface-cookbook.md
  C-later-roadmap.md
  D-migration-notes.md
```

## 10. Editing Rules

Documentation authors follow these rules.

```text
- Do not preserve old sentences merely to preserve them.
- Do not invent new architecture decisions.
- If a decision is needed, leave TODO_DECISION.
- Do not mix later automation into MVP body text.
- Move surface-specific details to appendix, not core documents.
- Put the canonical explanation of a concept in only one document.
- Other documents should provide only a one-sentence summary and a reference.
- Put schemas only in the designated areas of 05-mcp-api-and-schemas.md or 06-reference-mvp.md.
- State transitions are owned only by 03-kernel-spec.md.
- Send full template text to appendix/A-template-library.md.
```

## 11. Output Quality Bar

The rewritten documentation set must satisfy the following.

```text
- First-time users understand the harness as a conversation-centered tool.
- Implementers can build state.sqlite, the MCP server, projections, and the validator skeleton.
- Connector authors can connect a surface based on a capability profile.
- Operators can verify doctor/recover/export/conformance with fixtures.
- Design owners can distinguish core invariants from design-quality policy defaults.
```

## 12. Definition of Done

```text
[ ] core invariants remain the approved set of 7.
[ ] policy defaults and core invariants are separated.
[ ] event log location is clearly state.sqlite.task_events.
[ ] lifecycle + gates state model exists.
[ ] state transition table and impossible combinations exist.
[ ] close_task algorithm exists.
[ ] prepare_write algorithm exists.
[ ] approval gate and scope gate are separated.
[ ] detached verification waiver is not displayed as detached_verified.
[ ] User Notes authority is organized into three stages.
[ ] Domain Language canonical source is organized as the domain_terms table.
[ ] guarantee level is displayed as cooperative/detective/preventive/isolated.
[ ] per-MCP-tool request/response schemas exist.
[ ] error code taxonomy exists.
[ ] validator result schema exists.
[ ] SQLite DDL draft exists.
[ ] artifact schema and redaction rules exist.
[ ] projection job schema exists.
[ ] required/optional/appendix templates are separated.
[ ] connector documents are centered on capability profiles.
[ ] surface-specific details are in docs/appendix.
[ ] User Guide is short and conversation-oriented.
[ ] Operations conformance is fixture-based.
[ ] `docs/README.md` reader paths match the new document structure.
[ ] all official terms in Glossary match the new model.
[ ] later features do not blur MVP body text.
```
