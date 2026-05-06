# Target Document Tree

This document defines the target file structure for v2 of the harness documentation set and the ownership responsibility of each document.

## 0. Path Convention

```text
docs/README.md:
  harness documentation entrypoint

root README.md:
  repository landing page

All target documentation paths in this file are relative to docs/
unless explicitly stated otherwise.
```

## 1. Target Tree

```text
docs/
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

  rewrite-control/
    REWRITE-BRIEF.md
    KERNEL-DECISIONS.md
    TARGET-DOC-TREE.md
    DOC-OWNERSHIP-MAP.md
    CONFLICT-LIST.md
    PRESERVE-MOVE-LATER.md
    CODEX-BATCHES.md
    REVIEW-CHECKLIST.md

README.md
  repository landing page, not the harness documentation entrypoint
```

## 2. Main Documents

### README.md

Owner role:

```text
The front door of the documentation set.
Guides the harness one-sentence definition, core principles, reader paths, and target tree.
```

Path:

```text
docs/README.md
```

Owns:

```text
- one-sentence definition
- reader paths
- document list
- MVP/v1/later orientation summary
```

Does not own:

```text
- state machine
- MCP schema
- projection template
- connector details
- conformance fixtures
```

### 00-introduction.md

Owner role:

```text
A shared mental model for users and implementers.
```

Owns:

```text
- summary of the problems the harness reduces
- summary of the three-space model
- basic concepts for Task / Change Unit / Evidence / Projection
- introduction to advisor/direct/work
- status card example
- source-of-truth summary
```

Does not own:

```text
- implementation schemas
- state transition tables
- tool schema
- full template text
```

### 01-project-charter.md

Owner role:

```text
Project purpose, audience, values, scope, and non-goals.
```

Owns:

```text
- project purpose
- target users
- core values
- current non-goals
- automation philosophy
```

Does not own:

```text
- strategy invariant details
- state model
- API contract
- operating procedures
```

### 02-strategy.md

Owner role:

```text
Strategic thesis, failure model, core invariants, and policy defaults.
```

Owns:

```text
- strategic thesis
- failure model
- minimal harness kernel concept
- 7 core invariants
- policy default list
- human judgment model summary
- source-of-truth principle summary
- strategic meaning of guarantee level
- MVP boundary summary
```

Does not own:

```text
- lifecycle transition table
- gate enum details
- MCP request/response schema
- SQLite DDL
- full template text
- surface-specific connector addenda
```

### 03-kernel-spec.md

Owner role:

```text
Harness operating kernel specification.
```

Owns:

```text
- entity model
- conceptual contract for Task / Change Unit / Run / Approval / Evidence / Eval / QA / Artifact / Reconcile Item
- lifecycle model
- gate model
- state compatibility matrix
- transition table
- close semantics
- waiver semantics
- state-level logic for the prepare_write decision algorithm
- state-level logic for the close_task decision algorithm
- invariant enforcement mapping
```

Does not own:

```text
- MCP wire schema
- SQLite DDL column list
- projection template
- design-quality playbook procedure
- connector capability details
- first-class capability_gate
```

### 04-runtime-architecture.md

Owner role:

```text
Three spaces, runtime home, Core flow, authority, and projection/reconcile architecture.
```

Owns:

```text
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
```

Does not own:

```text
- per-tool schemas
- DB DDL
- full CLI commands
- conformance fixtures
- surface-specific addenda
```

### 05-mcp-api-and-schemas.md

Owner role:

```text
Owner document for MCP resources/tools, schemas, errors, idempotency, and validator result schema.
```

Owns:

```text
- MCP resources
- public MCP tools
- common envelope
- per-tool request schema
- per-tool response schema
- state transition summary per tool
- events emitted per tool
- projection jobs enqueued per tool
- error code taxonomy
- idempotency and retry
- state conflict behavior
- validator result schema
- artifact ref schema
```

Does not own:

```text
- why strategy
- full state transition table
- SQLite DDL implementation details
- user-facing conversation examples
```

### 06-reference-mvp.md

Owner role:

```text
MVP implementation sequence and reference implementation details.
```

Owns:

```text
- MVP-0 to MVP-5 implementation sequence
- SQLite DDL draft
- migration/versioning
- lock policy
- artifact directory layout
- baseline capture format
- projection job table
- reference surface behavior
- validator runner skeleton
- minimal CLI implementation plan
```

Does not own:

```text
- strategic rationale
- public MCP schema source of truth
- full projection template library
- surface cookbook
```

### 07-document-projection.md

Owner role:

```text
Markdown projection, managed/human-editable areas, artifact refs, and template tiers.
```

Owns:

```text
- projection principles
- authority matrix for documents
- managed block rules
- human-editable section rules
- artifact ref rules
- Markdown report projection boundary
- required MVP templates
- optional template summary
- projection freshness rules
```

Does not own:

```text
- DB schema
- full template library for all report types
- API schema
- design-quality policy applies_when rules
```

### 08-design-quality-policy-pack.md

Owner role:

```text
Defines design-quality principles as policy contracts.
```

Owns:

```text
- shared design policy
- domain language policy
- vertical slice policy
- TDD trace policy
- deep module/interface policy
- manual QA policy
- context hygiene policy
- waiver rules
- policy-to-validator mapping
```

Does not own:

```text
- core state machine
- tool schema
- full template text
- user guide examples beyond short policy examples
```

### 09-agent-integration.md

Owner role:

```text
Common integration contract and capability profile for agent surfaces.
```

Owns:

```text
- common integration structure
- capability tier
- capability profile schema
- generated file manifest concept
- push/pull context principle
- MCP unavailable fallback
- cooperative/detective/preventive/isolated guarantee expression
- reference surface contract
- connector conformance overview
```

Does not own:

```text
- surface-specific detailed cookbook
- Core state machine
- kernel gate list
- MCP schema details
- operational fixtures
```

### 10-user-guide.md

Owner role:

```text
How users actually speak and read.
```

Owns:

```text
- quick start
- common phrases
- reading status cards
- differences among direct/work/advisor
- differences among approval/assurance/manual QA/acceptance
- resuming stopped work
- handling missing evidence, verification, QA, and acceptance situations
```

Does not own:

```text
- DB schema
- MCP schema
- connector installation details beyond user-level summary
- full playbook
```

### 11-operations-and-conformance.md

Owner role:

```text
Installation, diagnostics, recovery, export, and fixture-based conformance.
```

Owns:

```text
- connect
- doctor
- serve mcp
- projection refresh
- reconcile
- recover
- export
- artifact integrity
- conformance fixture format
- core conformance fixtures
- connector conformance fixtures
- design-quality conformance fixtures
```

Does not own:

```text
- daily user workflow
- MCP schema
- long-term analytics as MVP requirement
```

### 99-authoring-guide.md

Owner role:

```text
Authoring rules that prevent the documentation from becoming bloated again.
```

Owns:

```text
- document layer rules
- canonical ownership rules
- source-of-truth phrasing rules
- MVP/v1/later label rules
- schema ownership rules
- template ownership rules
- contradiction review checklist
```

Does not own:

```text
- runtime contract itself
- user procedure itself
- conformance fixture content itself
```

### glossary.md

Owner role:

```text
Official term definitions.
```

Owns:

```text
- official definitions
- enum meaning summaries
- differences between confusing terms
```

Does not own:

```text
- full policy or implementation contract
```

## 3. Appendices

### appendix/A-template-library.md

Owns:

```text
- full DEC template
- full DESIGN template
- full DOMAIN-LANGUAGE template
- full MODULE-MAP template
- full INTERFACE-CONTRACT template
- full TDD-TRACE template
- full MANUAL-QA template
- full EXPORT manifest template
- expanded report variants
```

### appendix/B-surface-cookbook.md

Owns:

```text
- Codex-specific notes
- Claude Code-specific notes
- Gemini-specific notes
- GitHub Copilot-specific notes
- Cursor-specific notes
- surface-specific generated file details
- profile examples
```

### appendix/C-later-roadmap.md

Owns:

```text
- dashboard
- browser QA capture
- cross-surface verify
- native hook expansion
- sidecar advanced watcher
- parallel Change Unit orchestration
- long-term analytics
- team profile export/import
```

### appendix/D-migration-notes.md

Owns:

```text
- old file to new file migration notes
- removed/renamed sections
- compatibility guidance for existing docs
- version comparison and change rationale
```

## 4. Old-to-New Mapping

| Current File | New Destination |
|---|---|
| root `README.md` | repository landing page, outside canonical harness docs |
| `docs/README.md` | `docs/README.md` rewritten as documentation entrypoint |
| `docs/00-overview.md` | `docs/00-introduction.md` |
| `docs/01-project-charter.md` | `docs/01-project-charter.md` retained and tightened |
| `docs/02-strategy.md` | Split into `docs/02-strategy.md`, `docs/03-kernel-spec.md`, `docs/08-design-quality-policy-pack.md` |
| `docs/03-architecture.md` | `docs/04-runtime-architecture.md` |
| `docs/04-reference-implementation.md` | Split into `docs/03-kernel-spec.md`, `docs/05-mcp-api-and-schemas.md`, `docs/06-reference-mvp.md`, `docs/appendix/C-later-roadmap.md` |
| `docs/05-user-guide.md` | `docs/10-user-guide.md`, with long examples reduced or deleted |
| `docs/06-agent-integration.md` | `docs/09-agent-integration.md`, surface details to `docs/appendix/B-surface-cookbook.md` |
| `docs/07-document-and-artifact-contracts.md` | `docs/07-document-projection.md`, full templates to `docs/appendix/A-template-library.md` |
| `docs/08-operations-and-conformance.md` | `docs/11-operations-and-conformance.md`, metrics to `docs/appendix/C-later-roadmap.md` if not MVP |
| `docs/09-design-quality-playbooks.md` | `docs/08-design-quality-policy-pack.md`, detailed examples retained selectively |
| `docs/99-authoring-guide.md` | `docs/99-authoring-guide.md` updated |
| `docs/glossary.md` | `docs/glossary.md` updated |

## 5. Legacy Cleanup Requirement

Legacy docs replaced by v2 docs must not remain as canonical docs after content migration.

Legacy docs:

```text
docs/00-overview.md
docs/03-architecture.md
docs/04-reference-implementation.md
docs/05-user-guide.md
docs/06-agent-integration.md
docs/07-document-and-artifact-contracts.md
docs/08-operations-and-conformance.md
docs/09-design-quality-playbooks.md
```

Allowed treatments after migration:

```text
delete the legacy file
replace it with a short migration stub
move historical notes to docs/appendix/D-migration-notes.md
```

`docs/README.md` must not link to legacy docs except migration notes.

## 6. Ownership Constraints

```text
- State transition belongs only to 03-kernel-spec.md.
- MCP request/response schema belongs only to 05-mcp-api-and-schemas.md.
- SQLite DDL belongs only to 06-reference-mvp.md.
- Projection template tiers belong to 07-document-projection.md.
- Full template text belongs to appendix/A-template-library.md.
- Surface-specific connector details belong to appendix/B-surface-cookbook.md.
- Later automation belongs to appendix/C-later-roadmap.md.
- Historical/version comparison belongs to appendix/D-migration-notes.md.
```
