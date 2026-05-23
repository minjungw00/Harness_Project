# Authoring Guide

## Document Role

This document owns the rules that keep the harness documentation set small, implementable, and correctly layered.

It also owns the docs-maintenance conformance checklist used to detect documentation drift.

It does not own runtime behavior, user procedure, conformance fixture content, MCP schemas, SQLite DDL, or projection templates.

## Ownership Boundaries

Use exactly one canonical owner for each concept. Other documents may include a one-sentence summary and a link.

| Layer | Canonical owner |
|---|---|
| one-sentence definition, reader paths, document list, target tree summary | `README.md` |
| shared reader mental model, three-space summary, core concepts introduction | `00-introduction.md` |
| project purpose, target users, values, scope, non-goals, automation philosophy | `01-project-charter.md` |
| why, failure model, MVP boundary, Strategic Invariants, Kernel Authority Invariants, Design Stewardship Defaults | `02-strategy.md` |
| entity meanings, lifecycle, gates, state transitions, close semantics, `prepare_write` and `close_task` logic | `03-kernel-spec.md` |
| three spaces, runtime authority flow, artifact architecture, projection/reconcile architecture, guarantee levels | `04-runtime-architecture.md` |
| MCP resources/tools, request/response schemas, error taxonomy, validator result schema, artifact ref shape | `05-mcp-api-and-schemas.md` |
| reference MVP implementation order, SQLite DDL, migrations, storage layout, validator runner skeleton | `06-reference-mvp.md` |
| Markdown projection principles, authority matrix, managed blocks, human-editable sections, artifact reference rendering, template tiers, projection freshness/failure rules | `reference/document-projection.md` |
| shared design, decision quality, autonomy boundary, domain language, vertical slice, feedback loop/TDD, module/interface, codebase stewardship, Manual QA, context hygiene policies | `08-design-quality-policy-pack.md` |
| agent surface capability profile, common connector contract, fallback semantics | `reference/agent-integration.md` |
| user-facing conversation, status reading, resume procedure, approval/assurance/QA/acceptance explanation | `10-user-guide.md` |
| connect, doctor, serve MCP, projection refresh, reconcile, recover, export, artifact integrity, runtime conformance, docs-maintenance smoke reporting | `11-operations-and-conformance.md` |
| all projection template bodies and display card shapes | `reference/templates/*.md` |
| surface-specific cookbooks | `reference/surface-cookbook.md` |
| later automation and derived analytics | `roadmap.md` |
| document ownership, authoring rules, docs-maintenance conformance checklist | `99-authoring-guide.md` |
| official term definitions | `glossary.md` |

## Bilingual Sync

The English and Korean documentation sets mirror the same file structure and heading structure.

Any semantic change to `docs/en` must be reflected in `docs/ko` in the same batch. Translation may be idiomatic, but authority boundaries, stable terms, schema names, DDL names, error codes, and validator IDs must match.

## Principle Group Language

The strategy owns three principle groups: Strategic Invariants, Kernel Authority Invariants, and Design Stewardship Defaults. Do not promote helpful practices into Kernel Authority Invariants unless the owner doc is updated.

Strategic Invariants wording should preserve the differentiated promise:

```text
Strategic agency stays with the user.
The work journey remains followable from current state.
```

Kernel Authority Invariants wording should sound mandatory and structural:

```text
Product write requires an active scoped Change Unit.
Blocking product judgment requires a recorded Decision Packet.
Projection cannot override canonical state.
```

Design Stewardship Defaults wording should name applicability, waiver, record, validator, and close impact:

```text
Vertical slice is the default for feature work when it applies.
A horizontal exception may be recorded with a reason and follow-up.
```

Current Design Stewardship Defaults are shared design, domain language consistency, vertical slice default, TDD trace for suitable work, module/interface review, codebase stewardship, Manual QA, feedback loops, and context hygiene.

## MVP, v1, And Later Labels

Use these labels consistently:

| Label | Meaning |
|---|---|
| MVP | required for the reference implementation to validate Kernel Authority Invariants and Agency Conformance |
| v1 | plausible next version after MVP, still requiring fixtures and ownership |
| later | useful future automation that must not read as an MVP requirement |

Rules:

- Main docs may mention later work only as non-MVP context and should point to `roadmap.md`.
- Do not put roadmap later-automation items or team workflow expansion into MVP requirements.
- Treat Browser QA Capture, preventive `T4` guard expansion, Context Index, analytics or derived metrics, deployment/canary/rollback automation, team workflow, and parallel orchestration as v1-or-later unless an owner document explicitly promotes them with fixture coverage and implementation ownership.
- If a later item becomes v1, add conformance expectations and an owner before changing main docs.
- Derived metrics are analytics unless explicitly promoted as MVP-critical conformance signals.

## Source-Of-Truth Phrasing

Use this phrasing family:

```text
Operational state is canonical in state.sqlite current records plus state.sqlite.task_events.
Raw evidence is canonical in the artifact store.
Markdown reports are projections generated from state records and artifact refs.
Human-editable sections are input surfaces.
Accepted human edits become state only through reconcile or a Core state-changing action.
```

Avoid phrasing that implies a separate MVP event store:

```text
phrases that put state.sqlite beside a separate event log
```

If historical comparison needs that idea, immediately clarify that MVP event history is `state.sqlite.task_events`.

Do not use wording that treats:

```text
TASK, Journey, Markdown, or report text as the state authority.
Rendering output as if it mutates state.
User Notes as more than human-editable input.
DOMAIN-LANGUAGE Markdown as the vocabulary owner.
Report projections as raw evidence files by default.
```

Preferred authority paths:

```text
User Notes: human-editable input -> reconcile_items -> accepted state event/record
Domain Language: domain_terms -> DOMAIN-LANGUAGE projection
Module Map: module_map_items -> MODULE-MAP projection
Interface Contract: interface_contracts -> INTERFACE-CONTRACT projection
```

## Judgment Surface, Not Lecture

User-facing docs should reveal the context, choices, trade-offs, evidence, risk, recommendation, uncertainty, and next action needed for judgment.

Do not teach every internal gate. Name a gate only when it explains why progress, write, close, QA, acceptance, or risk acceptance is blocked.

The user owns the work judgment. The agent and harness expose current state and options; they do not replace the user's decision.

## Schema And Template Ownership

MCP tool request/response schemas, common envelope, error taxonomy, validator result schema, and artifact ref shape belong only in `05-mcp-api-and-schemas.md`.

SQLite DDL, migration/versioning, lock policy, artifact directory layout, and reference implementation storage details belong only in `06-reference-mvp.md`.

When documenting JSON `TEXT` fields, keep the split explicit: API payload validation shapes stay in `05-mcp-api-and-schemas.md`, SQLite column and storage details stay in `06-reference-mvp.md`, and doctor/recover/conformance expectations stay in `11-operations-and-conformance.md`. A boundary note that Core validates storage JSON before commit may be repeated, but do not duplicate schema bodies or DDL.

Projection rules and template tiers belong in `reference/document-projection.md`. All projection template bodies and display card shapes belong in `reference/templates/*.md`. `appendix/A-template-library.md` is migration source material only until final cleanup.

Conformance fixture bodies, suite catalog assertion-mode metadata, and fixture assertion semantics belong in `11-operations-and-conformance.md`. Other docs may point to that owner, but must not redefine the comparison mini-language.

User-facing examples may show Journey Cards or short report snippets, but they must not become schema definitions.

## Current-State Writing

Write canonical docs as current truth, not as rewrite history.

Preferred:

```text
The harness uses lifecycle fields plus gates.
```

Avoid in main docs:

```text
Unlike the old version, the harness now uses lifecycle fields plus gates.
```

Keep version comparison, removed sections, and old file names out of current-state docs. Git history is sufficient for old migration mapping.

## Cross-Reference Rules

Use links to point to owners instead of duplicating contracts.

Minimum references:

- Strategy references kernel and policy pack.
- Kernel references API and reference MVP.
- Runtime architecture references kernel, projection, and integration.
- API references kernel and reference MVP.
- Reference MVP references kernel, API, and operations.
- Projection references kernel and Template Reference. Appendix A is referenced only as migration source material until final cleanup.
- Policy pack references kernel and projection.
- Integration references API and Appendix B.
- Operations references API and reference MVP.

## Docs-Maintenance Conformance

Docs-maintenance conformance is a read-only review/check suite for this documentation corpus. It is not Core fixture conformance, a runtime validator, a canonical state transition, projection refresh, generated operational report, QA result, acceptance record, evidence artifact, or residual-risk acceptance.

The rule bodies live in this guide. [Operations And Conformance](11-operations-and-conformance.md#docs-maintenance-smoke-profile) may describe how an operator-maintenance profile reports these checks, but it must link back here instead of duplicating the full rule bodies.

A later automated checker should report the check category, file path, heading or anchor when available, canonical owner document, observed drift, and suggested fix. Resolve drift by updating the canonical owner first, then replacing non-owner duplicates with a summary plus link. If the correct product or architecture rule is unknown, use `TODO_DECISION`; if the rule is known but checker wiring, fixture coverage, CLI behavior, or implementation detail is missing, use `TODO_IMPLEMENT`.

Report severity guidance:

| Severity | Meaning |
|---|---|
| `FAIL` | Drift can make active docs contradictory or non-actionable, such as broken owner links, schema/DDL/enum/stable event/`ValidatorResult`/`ProjectionKind` mismatch, missing English/Korean paired active files, missing semantic section coverage, or non-owner text redefining an owner contract. Idiomatic heading text or minor grouping differences are not failures when owner links, stable identifiers, and reviewability remain clear. |
| `WARN` | Drift should be cleaned up but does not yet contradict an owner contract, such as minor glossary phrasing drift, duplicate explanatory prose that is not normative, stale but non-blocking cross-reference wording, or incomplete TODO metadata that is still understandable. |
| `PASS` | No relevant drift is found for the category. |

Required check categories:

| Category | Required check |
|---|---|
| English/Korean file structure parity | `docs/en` and `docs/ko` keep the same active document paths unless an exception is explicitly documented in this guide. |
| English/Korean semantic section parity | Paired English/Korean files keep the same active file map and semantic section coverage. Heading text and minor grouping may be idiomatic when owner links, stable identifiers, schema names, enum values, DDL names, validator IDs, code identifiers, and reviewability remain clear. |
| Broken cross-reference detection | Markdown links, heading anchors, appendix links, and paired-language entry links resolve to active docs. Links to owner sections should not point to removed migration context. |
| Owner-boundary drift | Exact contracts stay in their active owners, including `reference/kernel.md`, `reference/mcp-api-and-schemas.md`, `reference/storage-and-ddl.md`, `reference/document-projection.md`, `reference/templates/*.md`, `reference/design-quality-policies.md`, `reference/operations-and-conformance.md`, and `reference/glossary.md`. |
| Fixture/action schema and code drift | Operations fixture examples keep `action` and executable `input` aligned with public MCP request schemas in `reference/mcp-api-and-schemas.md` and the `ToolEnvelope` expansion convention in `reference/operations-and-conformance.md`. Required fixture events remain Kernel Stable Event Catalog names from `reference/kernel.md`, and `expected_error.code` plus `CloseTaskResponse.blockers[].code` remain API `ErrorCode` values. The check links to Operations, API, and Kernel owners instead of restating fixture semantics here. |
| Enum drift across owners | State/gate/result values match `reference/kernel.md`; error and stable `ValidatorResult` IDs match `reference/mcp-api-and-schemas.md`; storage values match `reference/storage-and-ddl.md`; projection values match `reference/document-projection.md` and `reference/templates/*.md`. Non-owner docs may summarize values only when needed and must link to the owner. |
| Stable Event Catalog drift | Any event name required by Operations fixtures, API tool descriptions, or storage/conformance text as fixture-stable appears in the Kernel Stable Event Catalog in `reference/kernel.md`. Non-catalog names must be marked as illustrative, implementation-local detail/audit, future extension, or promoted through the kernel owner. |
| Stable ValidatorResult ID drift | Stable `ValidatorResult` IDs match the API-owned list in `reference/mcp-api-and-schemas.md` and any validator-runner references in `reference/storage-and-ddl.md`. Core checks and preconditions must not drift into validator IDs unless the API or storage owner promotes them. |
| ProjectionKind tier drift | `ProjectionKind` values and tiers match across `reference/mcp-api-and-schemas.md`, `reference/document-projection.md`, `reference/templates/*.md`, `reference/storage-and-ddl.md`, `reference/operations-and-conformance.md`, and `reference/glossary.md`. Extension / appendix values must not become MVP-required by repetition outside the owner docs. |
| Glossary term drift | Official terms, capitalization, record ID prefixes, and source-of-truth meanings match `reference/glossary.md`. A recurring new term needs a glossary entry or an explicit decision to keep it local. |
| Source-of-truth phrasing drift | State, raw evidence, Markdown projections, human-editable sections, reconcile, and accepted human edits use the phrasing family in this guide and do not imply separate state authorities. |
| `TODO_DECISION` and `TODO_IMPLEMENT` compliance | TODOs use the allowed labels, include the needed decision or known implementation gap, name affected docs when useful, and do not leave actual `TODO_REWRITE` markers in finished canonical sections. |
| Non-owner duplicate full contracts | Paragraphs that restate full schemas, DDL, transition tables, fixture mini-languages, template bodies, or glossary definitions outside the owner doc should be replaced with a one-sentence summary plus owner link. |

## TODO Rules

Use `TODO_DECISION` only when a real product or architecture decision is unresolved. Include the decision needed, affected docs, and likely owner.

Use `TODO_IMPLEMENT` only when the decision is already made but implementation detail, DDL, fixture coverage, or CLI behavior is not yet filled in.

Do not use `TODO_REWRITE` in finished v2 canonical sections. A remaining `TODO_REWRITE` marker means the section is still a migration stub. Explanatory mentions of `TODO_REWRITE` in this Authoring Guide rule text are allowed and should not be counted as leftover migration stubs.

## Authoring Checklist

```text
[ ] Does this concept have exactly one canonical owner?
[ ] Are schema and DDL kept in their owner docs?
[ ] Are Strategic Invariants, Kernel Authority Invariants, and Design Stewardship Defaults kept separate?
[ ] Are Design Stewardship Defaults written with applicability and waiver boundaries?
[ ] Are MVP, v1, and later labels clear?
[ ] Are long-term analytics kept out of MVP requirements?
[ ] Does source-of-truth phrasing preserve state/artifact/projection boundaries?
[ ] Are semantic changes mirrored across `docs/en` and `docs/ko` in the same batch?
[ ] Do user-facing docs expose judgment context without teaching unnecessary internal gates?
[ ] Does the user guide avoid DB/API/connector internals?
[ ] Does operations use fixture-based conformance with executable assertions instead of prose-only matching?
[ ] Has docs-maintenance checked fixture/action schema drift and event/error-code drift through Operations, API, and Kernel owner links instead of duplicating fixture semantics?
[ ] Has docs-maintenance conformance been considered for bilingual parity, links, owner boundaries, stable catalogs, glossary terms, source-of-truth phrasing, and TODO rules?
[ ] Are docs-maintenance conformance references read-only documentation maintenance, not runtime validators or canonical state transitions?
[ ] Are non-owner full-contract paragraphs reduced to summaries plus owner links?
[ ] Are legacy names absent from current-state docs unless they are needed as source-material labels?
[ ] Are official terms aligned with glossary?
```
