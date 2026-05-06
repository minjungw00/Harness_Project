# Appendix D: Migration Notes

## Document Role

Old-to-new migration notes, compatibility guidance, and version comparison during the v2 documentation rewrite.

## Owns

- old file to new file migration notes
- removed/renamed sections
- compatibility guidance for existing docs
- version comparison and change rationale

## Does Not Own

- canonical runtime contracts
- kernel state semantics
- MCP schemas
- projection templates
- final user procedures

## Migration Status

Legacy documentation is archived under `docs/legacy-v1/`, and v2 skeleton files live at the target paths under `docs/`. Content migration is in progress; archived legacy files are retained as source material and must not be treated as deleted or fully migrated yet.

## Sections

### Migration Scope

TODO_REWRITE: Track file and section migration as each later batch rewrites canonical content.

### Old-To-New Mapping

TODO_REWRITE: Keep the old-to-new file mapping aligned with `docs/rewrite-control/TARGET-DOC-TREE.md`.

Current archive mapping:

| Archived Legacy Source | v2 Target Destination |
|---|---|
| `docs/legacy-v1/00-overview.md` | `docs/00-introduction.md` |
| `docs/legacy-v1/01-project-charter.md` | `docs/01-project-charter.md` |
| `docs/legacy-v1/02-strategy.md` | `docs/02-strategy.md`, `docs/03-kernel-spec.md`, `docs/08-design-quality-policy-pack.md` |
| `docs/legacy-v1/03-architecture.md` | `docs/04-runtime-architecture.md` |
| `docs/legacy-v1/04-reference-implementation.md` | `docs/03-kernel-spec.md`, `docs/05-mcp-api-and-schemas.md`, `docs/06-reference-mvp.md`, `docs/appendix/C-later-roadmap.md` |
| `docs/legacy-v1/05-user-guide.md` | `docs/10-user-guide.md` |
| `docs/legacy-v1/06-agent-integration.md` | `docs/09-agent-integration.md`, `docs/appendix/B-surface-cookbook.md` |
| `docs/legacy-v1/07-document-and-artifact-contracts.md` | `docs/07-document-projection.md`, `docs/appendix/A-template-library.md` |
| `docs/legacy-v1/08-operations-and-conformance.md` | `docs/11-operations-and-conformance.md`, `docs/appendix/C-later-roadmap.md` |
| `docs/legacy-v1/09-design-quality-playbooks.md` | `docs/08-design-quality-policy-pack.md` |
| `docs/legacy-v1/99-authoring-guide.md` | `docs/99-authoring-guide.md` |
| `docs/legacy-v1/glossary.md` | `docs/glossary.md` |
| `docs/legacy-v1/REWRITE-MANIFEST.md` | `docs/appendix/D-migration-notes.md` |

### Removed Or Renamed Sections

TODO_CONTENT: Record removed, renamed, or stubbed sections after migration decisions are applied.

### Compatibility Guidance

TODO_CONTENT: Add guidance for readers encountering legacy file names during the migration window.

### Version Comparison

TODO_CONTENT: Record high-level differences between the old document set and the v2 target set.
