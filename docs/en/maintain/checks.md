# Checks

Use these read-only documentation checks after documentation edits. This index routes to focused maintenance procedures only. It does not define API, storage, schema, security, runtime, projection, evidence, QA, acceptance, close-readiness, or residual-risk contracts.

Check pages may name forbidden strings as search patterns. Treat those strings as review inputs only, not as documentation statements to keep or publish outside explicit search-pattern lists.

In check pages, `Evidence to inspect` means documentation evidence for the reviewer. `Failure` means a documentation quality failure, and `Fix` means a direction for repairing documentation.

Run the checks that match the edit. For most documentation batches, start with [Structure checks](checks/structure.md) and [Links and indexes checks](checks/links-and-indexes.md), then add the focused pages that match the changed content.

## Check pages

| Page | Use for |
|---|---|
| [Structure checks](checks/structure.md) | owner boundaries, owner granularity, split-owner size, blocker-routing boundaries, route-page shape, route/index negative-rule accumulation, index-as-owner errors, check-card label taxonomy, semantic label-content consistency, conditional prohibition clarity, required/prohibited classification, effect/non-effect classification, route/contract classification, display wording boundaries, storage record family references, owner-map placement, baseline/out-of-scope wording, implementation wording, reference-claim placement, reference semantic skeletons, final-tree leftovers, work-specific residue search patterns, readability, and final reports |
| [Language parity checks](checks/language-parity.md) | English/Korean semantic, semantic-strength, meaning-unit skeleton, conditional prohibition skeleton, negative-clause, heading, table, and list parity, both-languages-wrong semantic label detection, Korean structural drift, removed-concept translation residue, identifier preservation, Korean structure, Korean technical style, and nonliteral Korean prose |
| [Terminology checks](checks/terminology.md) | terminology-map owner targets, Harness/Core distinction, verification criteria wording, current-scope wording, `Write Authorization` distinction, curated glossary subset rules, glossary entry and terminology-map primary-owner parity, single-primary-owner glossary entries, terminology-map and glossary owner synchronization, glossary entry focus, glossary link route semantics, terminology-map-only and detailed value contexts that must not route to the glossary, glossary overload, owner-routing label usage, mixed-language Korean, compressed Korean owner-link and blocker-routing prose, documentation-routing terms, owner-path actor wording, Korean blocker terminology, `active` wording, `complete` ambiguity, retired or unsupported concept names, close-readiness wording, and access/security wording terms |
| [API examples checks](checks/api-examples.md) | durable self-contained scenarios, schema and value-set audit, field-name consistency, response snapshot consistency, refs, timestamps, cross-method scenario spine detection, no fixed shared sample task requirement, and API owner routing in examples |
| [Links and indexes checks](checks/links-and-indexes.md) | relative links, anchors, moved-concept anchors, owner-boundary anchor IDs, `README` routes, `doc-index.yaml` as the canonical machine-readable owner route, `doc-index.yaml` structure references, terminology and metadata owner targets, glossary Markdown links only for included glossary terms, terminology-map-only and reserved/profile-gated value routes, doc-index/glossary owner consistency, index-as-owner errors, owner gaps, API error owner routing, method-router placement, and LLM retrieval routes |

## Result labels

Use `PASS`, `WARN`, `FAIL`, or `SKIP` only as documentation-maintenance check outcomes. A passing documentation check is not runtime conformance, implementation acceptance, QA completion, close readiness, product guarantee, or residual-risk acceptance.

Tie findings to file paths, owner documents, and suggested documentation fixes. If a check is skipped, state the reason.

## Report shape

Use a compact report shape after meaningful documentation edits:

- Scope:
- Changed files:
- Checks run:
- Findings:
- Skipped checks:
- Residual documentation risks:

The report should identify results as documentation-maintenance findings when that distinction could be unclear.
