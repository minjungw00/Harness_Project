# Authoring Guide

Use this guide before changing Harness documentation. It is a documentation editing rulebook only. It does not authorize Harness Server/runtime implementation, product-repository writes, generated operational files, runtime state, projections, evidence records, QA records, acceptance records, close records, residual-risk records, executable fixtures, or conformance runners.

The repository is documentation-only today and remains in documentation review unless the maintainer handoff owner says otherwise in [MVP Plan](../build/mvp-plan.md). Treat the docs as source material for a future Harness Server, not as accepted implementation-ready runtime behavior.

## 1. Reading And Scope

- Read root `AGENTS.md` before working in this repository.
- Read this guide before English-facing documentation edits.
- For bilingual or terminology-affecting edits, read [Translation Guide](translation-guide.md).
- Before touching Korean docs, read [Korean Authoring Guide](../../ko/maintain/authoring-guide.md) and [Korean Translation Guide](../../ko/maintain/translation-guide.md).
- Keep the work documentation-only.
- Prefer small batches. Report changed and deleted files.
- Do not create commits unless the user explicitly asks.

When stale prose conflicts with the current product thesis, owner boundaries, Korean quality rules, active/later boundaries, or honest security wording, rewrite it. Preserve the durable rule, not the former section shape.

## 2. Route Documents Route

README, Start, Use, Build, Maintain, and index documents route readers. They must not become secondary sources of truth for technical contracts.

Use these route families for navigation:

- `docs/doc-index.yaml`
- `docs/*/start.md`
- `docs/*/use/user-guide.md`
- `docs/*/use/agent-guide.md`
- `docs/*/use/judgment-examples.md`
- `docs/*/use/surface-recipes.md`
- `docs/*/build/mvp-plan.md`
- `docs/*/reference/README.md`
- `docs/*/later/index.md`
- `docs/*/maintain/authoring-guide.md`
- `docs/*/maintain/translation-guide.md`
- `docs/*/maintain/checks.md`

Use [Reference Index](../reference/README.md) to choose exact contract owners. Use [Later Index](../later/index.md) for unpromoted later candidates. Use [doc-index.yaml](../../doc-index.yaml) only as documentation retrieval metadata; it is not runtime configuration, implementation state, or permission to load both languages for one `doc_id`.

## 3. One Owner Per Contract

Every strict contract has one canonical owner. Exact fields, enum values, DDL, schemas, algorithms, state transitions, gate rules, fixture body shapes, template bodies, storage rules, security guarantees, error precedence, and official definitions belong in the owner named by [Reference Index](../reference/README.md).

When adding or changing technical contract content:

- add the normative content to the canonical owner first
- update both English and Korean owner files for meaning-changing edits
- replace non-owner duplicates with a short reader-visible consequence and a route to the owner
- keep README, Maintain, route, and example docs free of copied schema tables, DDL, storage effects, public error precedence, security guarantee definitions, and template bodies

Maintain docs own writing rules and checks. They do not own API, storage, security, projection, runtime, or product contracts.

## 4. Active/Later Boundary

Active docs must not make later candidates look like current MVP requirements. A value, method, table, fixture family, command, template, connector behavior, or security guarantee is active only when its canonical owner promotes it with scope, fallback behavior, and proof expectations.

Do not list profile-gated values as default active MVP values. Do not describe later candidates as active requirements. If an example needs a later candidate, label it as later-only and route the normative detail to [Later Index](../later/index.md) or the owner that promoted it.

## 5. User Judgment Boundary

Harness preserves user-owned judgment. Product behavior, material technical direction, scope expansion, sensitive-action approval, final acceptance, residual-risk acceptance, and cancellation remain distinct. Broad approval does not substitute for a specific judgment.

Final acceptance and residual-risk acceptance do not create missing evidence or erase evidence gaps. Sensitive-action approval does not become product-file write scope or Write Authorization.

## 6. Security Wording

Security wording must match the documented guarantee level.

- Use cooperative wording when Harness can guide or record expected behavior but cannot technically block the action.
- Use detective wording only when Harness can detect or report a supported observable fact after the action and the relevant capability check has passed for that scope.
- Use preventive or isolated wording only when the exact mechanism and proof path are documented by the owner.

Do not imply OS permissions, arbitrary-tool sandboxing, tamper-proof files, default pre-tool blocking, or security isolation unless the security owner documents and proves that exact mechanism.

## 7. Korean Quality

Korean documentation must read as natural Korean technical prose, not line-by-line English. Preserve exact identifiers such as file paths, `doc_id` values, API method names, schema fields, enum values, error codes, table names, validator IDs, and template names.

Use terms from [Translation Guide](translation-guide.md). Prefer "한영 문서 동시 유지", "의미 일치", "줄 단위 번역 아님", "에이전트 중복 주입 금지", "현재 MVP", "담당 문서", "사용자 소유 판단", "최종 수락", "잔여 위험 수락", "협력형 보장", "탐지형 보장", and "profile-gated 값" where they fit.

## 8. Stale Content

Maintain docs should guide future editing. They should not preserve historical rewrite reviews, closed issue records, obsolete acceptance notes, obsolete alias history, past translation problem records, or temporary migration plans.

Use these actions:

| Category | Action |
|---|---|
| `preserve` | Keep useful owner-aligned meaning. |
| `shrink` | Keep the reader-visible consequence and route to the owner. |
| `move` | Put the meaning in the owner and remove the duplicate. |
| `delete` | Remove obsolete, misleading, historical, or duplicative prose. |
| `decision-needed` | Route unresolved implementation-readiness decisions to [MVP Plan](../build/mvp-plan.md) and contract decisions to the owner. |

## 9. Post-Edit Checklist

- [ ] The edit stayed documentation-only.
- [ ] Meaning-changing edits were mirrored across English and Korean.
- [ ] Korean prose is natural and preserves exact identifiers.
- [ ] Strict contracts remain in one canonical owner.
- [ ] README, Maintain, route, and example docs summarize instead of defining technical contracts.
- [ ] Active/later boundaries and profile-gated values remain clear.
- [ ] User-owned judgments remain distinct.
- [ ] Security wording matches the documented guarantee level.
- [ ] Links and paired-language routes resolve.
- [ ] No temporary migration notes, scratch files, generated runtime records, or archive copies remain.
- [ ] Relevant checks in [Checks](checks.md) were run or reported as skipped.
