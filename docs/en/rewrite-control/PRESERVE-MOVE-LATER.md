# Preserve / Rewrite / Move-to-Appendix / Later / Delete Matrix

This document decides how content from the existing harness documentation set should be treated in the new document structure.

## 1. Categories

```text
PRESERVE:
  Keep the core meaning. Sentences may be rewritten.

REWRITE:
  Keep the intent, but change structure, layer, and wording.

MOVE_TO_APPENDIX:
  Move from body text to appendix.

LATER:
  Preserve as design material, but remove from MVP body requirements.

DELETE:
  Do not keep in current canonical documents. Keep only in migration notes if needed.
```

Every content item must use exactly one of these disposition labels. Canonical ownership is controlled by `DOC-OWNERSHIP-MAP.md`; this file records treatment during rewrite.

## 2. Preserve

The following must be preserved in the new documentation set.

| Content Item | Disposition | Canonical Owner | Notes |
|---|---|---|---|
| Three-space model | PRESERVE | `04-runtime-architecture.md` | README, introduction, and user guide provide summaries only. |
| Product Repository projection concept | PRESERVE | `07-document-projection.md` | Separate from source-of-truth. |
| Harness Runtime Home concept | PRESERVE | `04-runtime-architecture.md` | Exact `~/.harness` layout is specified in `06-reference-mvp.md`. |
| Principle that state/evidence outrank chat | PRESERVE | `02-strategy.md` | Connect to core invariant 1. |
| Source-of-truth/projection separation | PRESERVE | `02-strategy.md` | Document authority matrix is owned by `07-document-projection.md`. |
| Approval / assurance / manual QA / acceptance separation | PRESERVE | `02-strategy.md` | Gate mechanics are implemented in `03-kernel-spec.md`. |
| Task and Current Summary | PRESERVE | `03-kernel-spec.md` | `TASK` projection shape is owned by `07-document-projection.md`. |
| Change Unit concept | PRESERVE | `03-kernel-spec.md` | Vertical slice moves to policy default. |
| Evidence Manifest concept | PRESERVE | `03-kernel-spec.md` | Keep AC-to-evidence connection in core. |
| Detached verification concept | PRESERVE | `03-kernel-spec.md` | State explicitly that a waiver is not `detached_verified`. |
| Reduced public MCP tool surface | PRESERVE | `05-mcp-api-and-schemas.md` | Add per-tool schemas. |
| Capability profile | PRESERVE | `09-agent-integration.md` | Judge by profile instead of product name. |
| Managed/human-editable areas | PRESERVE | `07-document-projection.md` | Fix User Notes authority. |
| Reconcile flow | PRESERVE | `04-runtime-architecture.md` | Separate proposals from accepted state. |
| Shared design policy | PRESERVE | `08-design-quality-policy-pack.md` | Keep as work policy default. |
| Domain language policy | PRESERVE | `08-design-quality-policy-pack.md` | Canonical source is `domain_terms`. |
| Module/interface review policy | PRESERVE | `08-design-quality-policy-pack.md` | Keep as policy + validator mapping. |
| TDD trace policy | PRESERVE | `08-design-quality-policy-pack.md` | Define required/waived semantics. |
| Manual QA policy | PRESERVE | `08-design-quality-policy-pack.md` | Separate QA gate from policy. |
| Context hygiene policy | PRESERVE | `08-design-quality-policy-pack.md` | Keep the principle that old docs are not pushed blindly. |

## 3. Rewrite

The following keep their intent but are rewritten comprehensively.

| Existing Area | Disposition | Rewrite Target | Required Change |
|---|---|---|---|
| 17 strategy invariants | REWRITE | `02-strategy.md` | Split into 7 core invariants + policy defaults |
| Existing state axes | REWRITE | `03-kernel-spec.md` | lifecycle + gates + compatibility matrix |
| Reference Implementation | REWRITE | `03`, `05`, `06` split | Separate state/API/DDL/implementation order |
| Source-of-truth matrix | REWRITE | `07-document-projection.md` | Fix User Notes, Domain Language, Module/Interface authority |
| Artifact/report/projection boundary | REWRITE | `07-document-projection.md` | Separate raw artifacts, state records, and Markdown reports |
| User Guide | REWRITE | `10-user-guide.md` | Reduce to quick start + conversation phrases |
| Long user examples | REWRITE | `10-user-guide.md` | Keep representative examples only; remove repetitive examples |
| Operations conformance | REWRITE | `11-operations-and-conformance.md` | Rewrite into fixture-based format |
| Agent Integration | REWRITE | `09-agent-integration.md` | Reduce around capability profile |
| Design Quality Playbooks | REWRITE | `08-design-quality-policy-pack.md` | playbook prose → policy contract format |
| Runtime/artifact layout variants | REWRITE | `06-reference-mvp.md` | Unify into one exact layout |
| Security boundary prose | REWRITE | `04-runtime-architecture.md` | Express separately from guarantee level |
| Glossary | REWRITE | `glossary.md` | Update gate/waiver/guarantee/source terms |
| Authoring Guide | REWRITE | `99-authoring-guide.md` | Reflect new owner map and appendix structure |

## 4. Move to Appendix

The following should move to appendix rather than be discarded.

| Content | Disposition | Destination | Reason |
|---|---|---|---|
| Full DEC template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | Not an MVP required template |
| Full DESIGN template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | Advanced design document |
| Full DOMAIN-LANGUAGE template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | optional design-quality projection |
| Full MODULE-MAP template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | optional design-quality projection |
| Full INTERFACE-CONTRACT template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | optional design-quality projection |
| Full TDD-TRACE template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | optional/full variant |
| Full MANUAL-QA template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | optional/full variant |
| Expanded card templates | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | Keep only compact MVP cards in 07. |
| AGENTS.md full template | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Integration doc keeps principles only. |
| Harness Skill full template | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Managed in the surface cookbook. |
| Codex addendum | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Surface-specific detail |
| Claude Code addendum | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Surface-specific detail |
| Gemini addendum | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Surface-specific detail |
| GitHub Copilot addendum | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Surface-specific detail |
| Cursor addendum | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Surface-specific detail |
| Version comparison notes | MOVE_TO_APPENDIX | `appendix/D-migration-notes.md` | Remove from canonical document body |
| `REWRITE-MANIFEST.md` simplification history | MOVE_TO_APPENDIX | `appendix/D-migration-notes.md` | Migration input, not a canonical document in the final main tree |

## 5. Later Roadmap

The following are preserved in `appendix/C-later-roadmap.md`. Do not write them as implementation requirements in MVP body text.

| Later Item | Disposition | Reason |
|---|---|---|
| dashboard | LATER | UI after state/evidence/projection stabilize |
| browser QA artifact automatic capture | LATER | After T6 capability |
| cross-surface verify | LATER | After reference surface and bundles stabilize |
| native hook expansion | LATER | High per-surface variability |
| advanced sidecar file watcher | LATER | MVP starts with a detective minimum |
| worktree-based fresh verify automation | LATER | v1 stable candidate |
| parallel Change Unit orchestration | LATER | After DAG, baseline, and approval scope stabilize |
| long-term analytics | LATER | Calculate as derived metrics after the event model stabilizes |
| team profile export/import | LATER | After individual/small-team local kernel |
| artifact dashboard | LATER | After artifact schema and retention stabilize |
| advanced architecture drift validator | LATER | After baseline validators |
| public interface advanced validator | LATER | v1 stable candidate |
| domain language semantic consistency advanced check | LATER | v1/later candidate |

## 6. Delete or De-emphasize

The following are removed from canonical document body text or strongly reduced.

| Content | Disposition | Reason |
|---|---|---|
| Origin-story-centered wording | DELETE | Principle of directly describing current state |
| Draft comparison/version comparison | MOVE_TO_APPENDIX | Move to `appendix/D-migration-notes.md` |
| Implication that all connectors are completed at once | DELETE | Conflicts with MVP boundary |
| Implication that preventive write block is guaranteed on every surface | REWRITE | Clarify with guarantee level |
| Implication that heavy TDD/design docs are mandatory for every task | REWRITE | Express through policy applies_when/waiver |
| `domain language record + reconciled doc` | REWRITE | KD-08 |
| Wording that makes Projection look like canonical state | REWRITE | Violates source-of-truth principle |
| Wording that makes event log look like a separate store | REWRITE | KD-01 |
| `user notes = human-editable source-of-truth` | REWRITE | KD-07 |
| Duplicate report-reading tables | DELETE | Keep only representative user procedure in `10-user-guide.md`. |

## 7. Legacy File Cleanup

After content migration, legacy docs replaced by v2 docs must not remain as canonical docs.

| Legacy File | Disposition | Replacement / Notes |
|---|---|---|
| `docs/legacy-v1/00-overview.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/00-introduction.md`; optional migration note in `docs/appendix/D-migration-notes.md`. |
| `docs/legacy-v1/01-project-charter.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/01-project-charter.md`. |
| `docs/legacy-v1/02-strategy.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/02-strategy.md`, `docs/03-kernel-spec.md`, and `docs/08-design-quality-policy-pack.md`. |
| `docs/legacy-v1/03-architecture.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/04-runtime-architecture.md`; optional migration note in `docs/appendix/D-migration-notes.md`. |
| `docs/legacy-v1/04-reference-implementation.md` | DELETE_AFTER_MIGRATION | Content split into `docs/03-kernel-spec.md`, `docs/05-mcp-api-and-schemas.md`, `docs/06-reference-mvp.md`, and `docs/appendix/C-later-roadmap.md`. |
| `docs/legacy-v1/05-user-guide.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/10-user-guide.md`; long examples may be deleted or summarized in migration notes. |
| `docs/legacy-v1/06-agent-integration.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/09-agent-integration.md`; surface notes move to `docs/appendix/B-surface-cookbook.md`. |
| `docs/legacy-v1/07-document-and-artifact-contracts.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/07-document-projection.md`; full templates move to `docs/appendix/A-template-library.md`. |
| `docs/legacy-v1/08-operations-and-conformance.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/11-operations-and-conformance.md`; metrics move to `docs/appendix/C-later-roadmap.md` if not MVP. |
| `docs/legacy-v1/09-design-quality-playbooks.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/08-design-quality-policy-pack.md`; examples retained only selectively. |
| `docs/legacy-v1/99-authoring-guide.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/99-authoring-guide.md`. |
| `docs/legacy-v1/glossary.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/glossary.md`. |
| `docs/legacy-v1/REWRITE-MANIFEST.md` | MOVE_TO_MIGRATION_NOTES | Historical rewrite summary moves to `docs/appendix/D-migration-notes.md`. |

If repository history or user preference requires a visible file at a legacy path, replace the legacy file with a short migration stub. The stub is not canonical content and must point to the v2 owner doc or `docs/appendix/D-migration-notes.md`.

## 8. Source Document Treatment

### `REWRITE-MANIFEST.md`

Treatment: `MOVE_TO_APPENDIX`

Destination: `appendix/D-migration-notes.md`

Not part of final main tree. Use as control/migration input while writing migration notes.

Use it to confirm original simplification goals:

```text
- three-space execution model
- source-of-truth/projection
- reduced public MCP tool surface
- SQLite runtime center
- MVP/later split
- four judgment separation
- detached verification/design-quality principles
```

### `README.md`

Treatment: `REWRITE`

Path note: `docs/README.md` is the harness documentation entrypoint. Root `README.md` is the repository landing page.

Keep:

```text
- one-sentence definition
- three-space summary
- document list
- core principles
```

Change:

```text
- reflect the new target doc tree
- reflect state.sqlite.task_events wording
- update reader paths
```

### `00-overview.md`

Treatment: `REWRITE`

Target: `00-introduction.md`

Keep:

```text
- problems the harness reduces
- three spaces
- basic flow
- core concepts
- status card example
```

REWRITE targets:

```text
- source-of-truth matrix details → 07
- implementation flow details → 03/04
- template/schema mentions → owner docs
```

### `01-project-charter.md`

Treatment: `PRESERVE`

Keep most content. Add:

```text
- MVP is a cooperative/detective local kernel
- not all connectors are completed at once
- not a fully automated blocking system
```

### `02-strategy.md`

Treatment: `REWRITE`

Split:

```text
- why/failure/core invariants → 02-strategy
- state axes/gates/transitions → 03-kernel-spec
- design quality details → 08-design-quality-policy-pack
```

### `03-architecture.md`

Treatment: `REWRITE`

Target: `04-runtime-architecture.md`

Keep:

```text
- three spaces
- runtime layers
- transaction flow
- projection/reconcile
- detached verification flow
- guarantee level
```

Change:

```text
- clarify event log location
- place guarantee level earlier
- distinguish sidecar/native hook as later
```

### `04-reference-implementation.md`

Treatment: `REWRITE`

Split into:

```text
03-kernel-spec.md
05-mcp-api-and-schemas.md
06-reference-mvp.md
appendix/C-later-roadmap.md
```

### `05-user-guide.md`

Treatment: `REWRITE`

Target: `10-user-guide.md`

Keep:

```text
- common phrases
- status card reading
- four judgment separation
- resume guidance
```

REWRITE/remove:

```text
- long work examples
- detailed report reading tables
- verbose operational habits
```

### `06-agent-integration.md`

Treatment: `REWRITE`

Keep in main:

```text
- common integration structure
- capability tiers
- capability profile
- fallback principles
- connector conformance overview
```

MOVE_TO_APPENDIX:

```text
- Codex/Claude/Gemini/Copilot/Cursor addenda
```

### `07-document-and-artifact-contracts.md`

Treatment: `REWRITE`

Keep in main:

```text
- projection principles
- authority matrix corrected
- managed/human-editable rules
- artifact refs
- required MVP templates
```

MOVE_TO_APPENDIX:

```text
- full templates
```

### `08-operations-and-conformance.md`

Treatment: `REWRITE`

Keep:

```text
- setup/connect
- doctor
- projection refresh
- reconcile
- recover
- export
- conformance
```

Change:

```text
- conformance → fixture-based
- long-term metrics → later roadmap
```

### `09-design-quality-playbooks.md`

Treatment: `REWRITE`

Keep concepts:

```text
- Shared Design
- Domain Language
- Vertical Slice
- TDD
- Deep Module
- Manual QA
- Context Hygiene
```

Change structure:

```yaml
policy:
  applies_when:
  default_requirement:
  allowed_waiver:
  required_record:
  validator:
  close_impact:
```

### `99-authoring-guide.md`

Treatment: `REWRITE`

Update:

```text
- new doc tree
- new ownership map
- core invariant vs policy default rule
- schema/template/appendix ownership
```

### `glossary.md`

Treatment: `REWRITE`

Add:

```text
Gate
Scope Gate
Approval Gate
Evidence Gate
Verification Gate
QA Gate
Acceptance Gate
Close Reason
Waiver
Guarantee Level
Cooperative Guarantee
Detective Guarantee
Preventive Guarantee
Reference Surface
Raw Artifact
State Record
Markdown Report
Report Projection
```

Modify:

```text
Source-of-truth
Projection
Artifact
Raw Artifact
State Record
Markdown Report
Report Projection
Evidence Manifest
Domain Language
Human-editable Area
Reconcile
Detached Verification
Assurance
```
