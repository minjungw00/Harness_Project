# Authoring Guide

## What this guide helps you do

Use this guide when you add, rewrite, split, rename, or review Harness documentation.

It helps you keep the redesigned documentation readable for the intended reader, clear about where each kind of detail belongs, and aligned between English and Korean.

This guide governs documentation maintenance only. It does not authorize runtime behavior, server implementation, product state changes, generated operational files, evidence records, QA results, acceptance decisions, or task closure.

## Documentation principles

Write from the reader's next useful step. A document should make it easier for the reader to understand, decide, use, build, verify, or maintain something specific.

Prefer a small number of strong ideas over a complete inventory of internal machinery. Move strict contracts to Reference docs, and link to them when another document needs precision.

Introduce unfamiliar concepts with a concrete situation first. Readers should understand why a concept exists before they are asked to memorize its exact definition.

Keep the opening of every document predictable. A reader should quickly know what the document helps them do, when to read it, what they need first, and what idea will organize the rest of the page.

Write current documentation as current truth. Migration history, removed structures, and old names must stay out of the main explanation. If a dedicated migration note exists during a migration, keep that history there; otherwise rely on Git history or a clearly labeled non-active migration record.

## Document types

### Learn

Learn docs build the reader's mental model.

They explain purpose, concepts, examples, and trade-offs before implementation details. Use them when the reader needs orientation more than a command, schema, or checklist.

### Use

Use docs help a person operate Harness during an AI-assisted work session.

They should emphasize user-facing flow, status interpretation, decisions, and recovery paths. Mention internal gates only when they explain why the user sees a block or next action.

### Build

Build docs help an implementer construct the reference system after the documentation redesign is accepted.

They should explain implementation order, module boundaries, runnable slices, and verification strategy. Link to Reference docs for exact schemas, DDL, and invariants.

### Reference

Reference docs own exact contracts.

They are the place for strict schemas, gates, DDL, enum values, state transitions, invariants, API shapes, storage rules, projection rules, fixture semantics, and official definitions.

### Maintain

Maintain docs govern the documentation system itself.

They define authoring rules, translation policy, review checklists, link hygiene, ownership maps, and documentation-maintenance expectations. They must not become runtime conformance specs or product implementation plans.

## Standard opening pattern

Every redesigned document should begin with a short, predictable opening. Keep it compact, but make the reader's path clear.

### What this document helps you do

State the useful outcome in plain language. Avoid saying only what the document "covers."

### Read this when

Name the situation that makes the document relevant. This can be a short list.

### Before you read

Name the assumed context, prior document, or prerequisite. If there is no prerequisite, say so briefly.

### Main idea

Give the reader the central model or claim that will make the rest of the page easier to follow.

### Reference scope, only for reference docs

Reference docs should state the exact contract they own and what they deliberately do not own. This prevents strict details from spreading across Learn, Use, and Build docs.

## Concept introduction rule

Introduce concepts through examples before strict definitions.

Start with a concrete situation, show what problem the concept solves, and then name the concept. Put the strict definition after the reader has seen why it matters.

Preferred shape:

```text
When an agent wants to change product state, Harness first needs to know which scoped implementation unit the write belongs to. That unit is the Change Unit. The larger user-value item the user wants finished or answered is the Task.
```

Avoid opening a Learn doc with a dense definition list unless the page is explicitly a glossary or reference table.

## Reference contract rule

Strict schemas, gates, DDL, enum values, state transitions, invariants, API shapes, storage rules, projection contract details, and fixture semantics belong in Reference docs.

Learn, Use, Build, and Maintain docs may summarize a contract in one or two sentences when needed, then link to the owning Reference document. They should not duplicate full tables, schema bodies, transition matrices, DDL blocks, or fixture mini-languages.

## Repetition rule

Do not repeat long source-of-truth paragraphs across docs.

When another document needs the same idea, write a short local summary and link to the owner. If the source text changes, update the owner first, then check summaries for drift.

Repeated explanatory examples are allowed when they serve different readers, but repeated normative contract language is a drift risk.

## Diagram rule

Use diagrams only when they reduce cognitive load.

A diagram is useful when it shows a relationship, sequence, boundary, or lifecycle more clearly than prose. Do not add a diagram as decoration, as a second copy of an already clear list, or as a way to hide unresolved structure.

Every diagram should have nearby prose that explains what to notice. If a diagram and the prose disagree, the owning prose or reference contract is the source to fix first.

## English/Korean semantic parity rule

English and Korean docs must preserve the same active file map, semantic section coverage, and contractual detail.

Paired English/Korean files keep the same active file map and semantic section coverage. Heading text and minor grouping may be idiomatic when owner links, stable identifiers, and reviewability remain clear. Korean headings and prose may be natural Korean; different but semantically equivalent Korean headings are not an automatic docs-maintenance failure. Official identifiers, API names, schema names, enum values, DDL names, file names, error codes, validator IDs, code identifiers, and product terms listed in the translation guide must remain exact.

Any semantic change in `docs/en` must be mirrored in `docs/ko` in the same batch, and the reverse is also true.

## Link and rename rule

When you rename, move, split, or merge a document, update links in both languages in the same batch.

Prefer links to the owner document or owner section instead of links to secondary summaries. Do not point active owner links to removed migration context.

After a rename, search for old paths, old anchors, old headings, and old title text. Update the README path, nearby cross-references, appendix links, and paired-language links together.

## Docs-maintenance checks

Docs-maintenance checks are read-only documentation maintenance. They are not Core fixture conformance, runtime validators, canonical state transitions, projection refresh, generated operational reports, QA results, acceptance records, evidence artifacts, residual-risk acceptance, close readiness, or implementation readiness.

A docs-maintenance review or future checker should report the category, file path, heading or anchor when available, owner document, observed drift, suggested fix, and a statement that no canonical state transition was performed. Resolve drift by updating the owner first, then replacing non-owner duplicates with a short summary plus owner link.

Use these result meanings:

| Result | Meaning |
|---|---|
| `FAIL` | Drift can make active docs contradictory or non-actionable, such as broken owner links, schema/DDL/enum/stable event/`ValidatorResult`/`ProjectionKind` mismatch, missing paired active files, missing semantic section coverage, or non-owner text redefining an owner contract. Idiomatic heading text or minor grouping differences are not failures when owner links, stable identifiers, and reviewability remain clear. |
| `WARN` | Drift should be cleaned up but does not yet contradict an owner contract, such as minor glossary phrasing drift, duplicate explanatory prose that is not normative, stale but non-blocking cross-reference wording, or incomplete TODO metadata that is still understandable. |
| `PASS` | No relevant drift is found for the category. |

Required check categories:

| Category | Required check |
|---|---|
| English/Korean file structure parity | `docs/en` and `docs/ko` keep the same active document paths unless an exception is explicitly documented. |
| English/Korean semantic section parity | Paired files keep the same active file map and semantic section coverage. Heading text and minor grouping may be idiomatic when owner links, stable identifiers, schema names, enum values, DDL names, validator IDs, code identifiers, and reviewability remain clear. |
| Broken cross-reference detection | Markdown links, heading anchors, appendix links, and paired-language entry links resolve to active docs. |
| Owner-boundary drift | Exact contracts stay in their active owners, including `reference/kernel.md`, `reference/mcp-api-and-schemas.md`, `reference/storage-and-ddl.md`, `reference/document-projection.md`, `reference/templates/*.md`, `reference/design-quality-policies.md`, `reference/operations-and-conformance.md`, and `reference/glossary.md`. |
| Fixture/action schema drift | Operations fixture examples keep `action` and executable `input` aligned with public MCP request schemas in `reference/mcp-api-and-schemas.md` and the `ToolEnvelope` expansion convention in `reference/operations-and-conformance.md`; fixture semantics are linked, not restated here. |
| Enum, event, validator, and projection drift | State/gate/result values and Kernel Stable Event Catalog names match `reference/kernel.md`; error and stable `ValidatorResult` IDs match `reference/mcp-api-and-schemas.md`; storage values match `reference/storage-and-ddl.md`; `ProjectionKind` tiers match `reference/document-projection.md` and `reference/templates/*.md`. |
| Glossary and source-of-truth phrasing drift | Official terms, capitalization, record ID prefixes, and source-of-truth boundaries match `reference/glossary.md` and do not imply extra state authorities. |
| TODO compliance | `TODO_DECISION` and `TODO_IMPLEMENT` use the allowed meanings, name the gap clearly, and do not leave `TODO_REWRITE` markers in finished canonical sections. |
| Non-owner duplicate full contracts | Full schemas, DDL, transition tables, fixture mini-languages, template bodies, or glossary definitions outside the owner doc are replaced with a short summary plus owner link. |

## Review checklist

```text
[ ] Does the document serve a clear reader situation?
[ ] Does the opening follow the standard pattern?
[ ] Are concepts introduced through examples before strict definitions?
[ ] Are strict schemas, gates, DDL, enums, and invariants kept in Reference docs?
[ ] Are long source-of-truth paragraphs summarized and linked instead of repeated?
[ ] Do diagrams reduce cognitive load?
[ ] Are English and Korean files semantically aligned?
[ ] Are official identifiers preserved exactly?
[ ] Are renamed paths, anchors, and README links updated in both languages?
[ ] Is current truth separated from migration history?
[ ] Are Maintain docs limited to documentation governance, not runtime behavior?
```

## Reference ownership map

Use this map when deciding where exact detail belongs. It identifies the active owner in the redesigned documentation structure, so retired paths do not remain part of the authoring workflow.

| Subject | Active owner |
|---|---|
| Entrypoint, reader paths, document list, target tree summary | `README.md` |
| Shared reader mental model and three-space overview | `learn/overview.md` |
| Small core concept introduction | `learn/concepts.md` |
| Project purpose, target users, values, scope, non-goals, automation philosophy | `learn/purpose-and-principles.md` |
| Strategic thesis, failure model, MVP boundary, principle groups | `learn/purpose-and-principles.md` for reader explanation; `reference/design-quality-policies.md` and `reference/kernel.md` for exact contract impact |
| Kernel entities, lifecycle, gates, state transitions, close semantics, `prepare_write`, `close_task` | `reference/kernel.md` |
| Runtime architecture, three spaces in implementation detail, Core process model, artifact architecture, projection/reconcile architecture, guarantee levels | `reference/runtime-architecture.md` |
| MCP resources/tools, request/response schemas, error taxonomy, validator result schema, artifact ref shape | `reference/mcp-api-and-schemas.md` |
| SQLite DDL, migrations, storage layout, lock policy, artifact directory layout, baseline capture format, projection job table | `reference/storage-and-ddl.md` |
| MVP implementation order and stage exit criteria | `build/mvp-plan.md` |
| First runnable implementation slice | `build/first-runnable-slice.md` |
| Markdown projection principles, authority matrix, managed blocks, human-editable sections, artifact reference rendering, template tiers, projection freshness/failure rules | `reference/document-projection.md` |
| All projection template bodies and display card shapes | `reference/templates/*.md` |
| Design-quality policy contracts, validators, severity composition, waiver semantics, evidence expectations, close impact | `reference/design-quality-policies.md` |
| User-facing conversation, status reading, user judgments, close checklist | `use/user-guide.md` |
| User/agent session procedure | `use/agent-session-flow.md` |
| Agent surface capability profiles, common connector contract, fallback semantics, Role Lens, connector conformance overview | `reference/agent-integration.md` |
| Surface-specific recipes | `reference/surface-cookbook.md` |
| Generic capability profile examples | `reference/agent-integration.md` |
| Operator procedures, conformance fixture bodies, fixture assertion semantics, doctor/recover/reconcile/export/artifact integrity, docs-maintenance reporting | `reference/operations-and-conformance.md` |
| Official term definitions and capitalization | `reference/glossary.md` |
| Post-MVP roadmap | `roadmap.md` |
| Documentation authoring rules | `maintain/authoring-guide.md` |
| Translation and bilingual prose rules | `maintain/translation-guide.md` |
