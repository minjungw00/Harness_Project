# Template Reference

## Used when

Use these files when you need the rendered Markdown shape for MVP-required projection templates. The projection rules, authority boundaries, and freshness behavior are defined in [Document Projection Reference](../document-projection.md).

## Template tiering

Projection templates match the API `ProjectionKind` tiers:

| Tier | Templates | Rule |
|---|---|---|
| MVP-required | `TASK`, `APR`, `RUN-SUMMARY`, `EVIDENCE-MANIFEST`, `EVAL`, `DIRECT-RESULT` | MVP projector must render these. |
| MVP-optional | `MANUAL-QA`, `TDD-TRACE`, `DOMAIN-LANGUAGE`, `MODULE-MAP`, `INTERFACE-CONTRACT` | Render when policy applies, records exist, or the user/operator enables the projection. |
| Extension / appendix | `DEC`, `DESIGN`, `EXPORT`, `JOURNEY-CARD` | Render only when the corresponding extension or appendix projection is enabled. |

Templates are rendered shapes, not canonical state. They must not redefine kernel fields, MCP schemas, SQLite DDL, gate behavior, or artifact integrity rules.

## MVP-required templates

- [TASK](task.md)
- [APR](approval.md)
- [RUN-SUMMARY](run-summary.md)
- [EVIDENCE-MANIFEST](evidence-manifest.md)
- [EVAL](eval.md)
- [DIRECT-RESULT](direct-result.md)

## Notes

This directory is the active reference location for split MVP-required template bodies. Optional and extension template bodies remain in the legacy consolidated [Appendix A](../../appendix/A-template-library.md) until they move.
