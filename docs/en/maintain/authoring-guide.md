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

English and Korean docs must preserve the same meaning, file map, and contractual detail.

Korean headings and prose may be natural Korean. They do not need to preserve English word order or sentence structure. Official identifiers, API names, schema names, enum values, DDL names, file names, error codes, validator IDs, and product terms listed in the translation guide must remain exact.

Any semantic change in `docs/en` must be mirrored in `docs/ko` in the same batch, and the reverse is also true.

## Link and rename rule

When you rename, move, split, or merge a document, update links in both languages in the same batch.

Prefer links to the owner document or owner section instead of links to secondary summaries. Do not point active owner links to migration notes unless the subject is migration history.

After a rename, search for old paths, old anchors, old headings, and old title text. Update the README path, nearby cross-references, appendix links, and paired-language links together.

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

Use this map when deciding where exact detail belongs. The map preserves the old one-owner discipline, but it is a backstop for maintainers rather than the first principle a reader should meet.

During the redesign, some target reader-path files may not exist yet. Until a target file exists, keep the current numbered source document as the owner and update this map when the migration lands.

After a target reference file exists, that target is the active owner; the numbered file becomes migration source material until final cleanup.

| Subject | Owner in redesigned structure | Previous/current source during redesign |
|---|---|---|
| Entrypoint, reader paths, document list, target tree summary | `README.md` | `README.md` |
| Shared reader mental model and three-space overview | `learn/overview.md` | `00-introduction.md` |
| Small core concept introduction | `learn/concepts.md` | `00-introduction.md`, `glossary.md` |
| Project purpose, target users, values, scope, non-goals, automation philosophy | `learn/purpose-and-principles.md` | `01-project-charter.md` |
| Strategic thesis, failure model, MVP boundary, principle groups | `learn/purpose-and-principles.md` for reader explanation; `reference/design-quality-policies.md` and `reference/kernel.md` for exact contract impact | `02-strategy.md` |
| Kernel entities, lifecycle, gates, state transitions, close semantics, `prepare_write`, `close_task` | `reference/kernel.md` | `03-kernel-spec.md` as previous source material |
| Runtime architecture, three spaces in implementation detail, Core process model, artifact architecture, projection/reconcile architecture, guarantee levels | `reference/runtime-architecture.md` | `04-runtime-architecture.md` as previous source material |
| MCP resources/tools, request/response schemas, error taxonomy, validator result schema, artifact ref shape | `reference/mcp-api-and-schemas.md` | `05-mcp-api-and-schemas.md` as previous source material |
| SQLite DDL, migrations, storage layout, lock policy, artifact directory layout, baseline capture format, projection job table | `reference/storage-and-ddl.md` | `06-reference-mvp.md` as previous source material |
| MVP implementation order and stage exit criteria | `build/mvp-plan.md` | `06-reference-mvp.md` |
| First runnable implementation slice | `build/first-runnable-slice.md` | `06-reference-mvp.md` |
| Markdown projection principles, authority matrix, managed blocks, human-editable sections, artifact reference rendering, template tiers, projection freshness/failure rules | `reference/document-projection.md` | `07-document-projection.md` as migration source material |
| All projection template bodies and display card shapes | `reference/templates/*.md` | `appendix/A-template-library.md` as migration source material until final cleanup |
| Design-quality policy contracts, validators, severity composition, waiver semantics, evidence expectations, close impact | `reference/design-quality-policies.md` | `08-design-quality-policy-pack.md` as previous source material |
| User-facing conversation, status reading, user judgments, close checklist | `use/user-guide.md` | `10-user-guide.md` |
| User/agent session procedure | `use/agent-session-flow.md` | `09-agent-integration.md` and `10-user-guide.md` as migration source material |
| Agent surface capability profiles, common connector contract, fallback semantics, Role Lens, connector conformance overview | `reference/agent-integration.md` | `09-agent-integration.md` as migration source material |
| Surface-specific recipes | `reference/surface-cookbook.md` | `appendix/B-surface-cookbook.md` as migration source material |
| Generic capability profile examples | `reference/agent-integration.md` | `appendix/B-surface-cookbook.md` as migration source material |
| Operator procedures, conformance, doctor/recover/reconcile/export/artifact integrity | `reference/operations-and-conformance.md` | `11-operations-and-conformance.md` |
| Official term definitions | `reference/glossary.md` | `glossary.md` |
| Post-MVP roadmap | `roadmap.md` | `appendix/C-later-roadmap.md` |
| Documentation authoring rules | `maintain/authoring-guide.md` | `99-authoring-guide.md` |
| Translation and bilingual prose rules | `maintain/translation-guide.md` | none; new in redesign |
