# Translation guide

Use this guide when editing paired English and Korean Harness documentation. It owns bilingual meaning parity, natural Korean technical prose, identifier preservation guidance, mixed-language Korean style, and hidden-anchor practice.

This is a documentation-maintenance guide. It does not define product behavior, API behavior, storage effects, security guarantees, projection behavior, runtime behavior, schema contracts, glossary entries, or owner-routing indexes. When contract detail is needed, link to the focused owner instead of restating it here.

Complete structured terminology metadata lives in [`docs/terminology-map.yaml`](../../terminology-map.yaml). The [Glossary](../reference/glossary.md) is a compact reader-facing subset of selected terms. Check the terminology map before adding or changing product terms, Korean prose terms, identifier explanations, product labels, or Korean mixed-language controls.

## 1. Meaning Parity

English and Korean documents are both maintained. Neither language is an archive, appendix, or translation-only copy.

Maintain parity by meaning unit, not by line count. A meaning unit can be a rule, warning, paragraph, table row, example, route, or checklist item. Korean may change sentence order, split or combine sentences, or use a different paragraph rhythm when that makes the meaning clearer.

Semantic parity requires the same reader purpose, normative strength, baseline and out-of-scope boundary, owner routing, user-judgment boundary, evidence boundary, verification boundary, acceptance boundary, residual-risk boundary, security guarantee level, exact identifiers, and exact product labels where those items are present in the paired material.

Do not finish a meaning-changing edit with only one language updated. If Korean editing exposes an English problem, fix the English too. If English editing introduces a product concept, add the natural Korean equivalent in the paired Korean document during the same documentation update.

Do not add Korean-only structure just to make a sentence easier to scan. Labels such as `조건`, `결과`, `비주장`, or `허용되지 않는 것` belong in Korean only when the English document has the same meaning unit.

## 2. Routes And Owners

Every major maintained page should have an English/Korean pair at the matching route under `docs/en/` and `docs/ko/`. Paired documents do not need matching line numbers, but they must keep matching scope and reader intent.

Preserve route and owner references by meaning. Do not recreate full owner maps, API method maps, storage-effect summaries, schema field tables, security rules, or product contract text in this guide. Owner lookup belongs to the [Authoring Guide](authoring-guide.md), [`docs/doc-index.yaml`](../../doc-index.yaml), and the applicable reference owner.

During normal lookup, load the language that matches the request or the default language in `doc-index.yaml`. Load both languages for translation, bilingual parity review, or terminology-affecting edits.

## 3. Terminology Decisions

Use [`docs/terminology-map.yaml`](../../terminology-map.yaml) as the complete structured terminology source. This section is only a compact writing aid.

Apply these decisions consistently:

- Harness is the local work-authority product/system for AI-assisted product work. Core is the local authority record for Harness state.
- Use "verification criteria" when the meaning is user-visible criteria for checking work. Use "검증 기준" in Korean.
- Use "current scope" or "currently applied scope" in prose context. Use "현재 적용 범위" in Korean. Preserve exact identifiers and status values that contain `active`.
- Keep the exact label `Write Authorization` distinct from ordinary write approval. In Korean explanatory prose, use "쓰기 권한 부여" for `Write Authorization` and "쓰기 승인" for ordinary write approval.
- In Korean reference prose, use "닫기 준비 상태" for close readiness.

Do not copy the terminology map's preferred-expression or discouraged-expression lists into this guide or the glossary. Add or update the terminology map when a durable term, product label, identifier explanation, or Korean mixed-language control changes.

## 4. Identifiers And Labels

Preserve exact identifiers unchanged in English and Korean. Put code-like, schema-like, route-like, or search-critical values in backticks when they appear in prose.

Do not translate exact strings inside code blocks, schema examples, API examples, file paths, field lists, literal-value tables, or machine-readable metadata. Localized Korean display labels are reader-facing text; they do not replace canonical identifiers or exact product labels.

Use this distinction:

- Exact identifier: `ArtifactRef`
- Korean explanation of the identifier: 아티팩트 참조 스키마
- Ordinary prose concept: 아티팩트
- Exact product label: `Product Repository`, `Harness Runtime Home`, `Projection`, or `Write Authorization` when naming the label itself
- Korean reader-facing prose: 제품 저장소, 런타임 홈, 상태 보기, or 쓰기 권한 부여 when the exact label is not the subject

Some English words can be both code values and ordinary adjectives. Determine the context before preserving the word. Preserve `complete` in backticks only when it is an identifier, such as `intent=complete`. When the English means full or entire, English prose should prefer "full" or "entire" and Korean prose should use the terminology map's ordinary-prose replacement.

## 5. Korean Prose

Korean documents should read as native Korean technical documentation, not mirrored English.

Write Korean with natural headings, short explanatory sentences, Korean concept-first phrasing, consistent terms from the terminology map, enough context for Korean readers, and exact identifiers preserved in backticks where needed.

Translate ordinary English nouns and noun phrases into Korean prose. Keep English unchanged only when it is an exact identifier, file path, anchor, code literal, schema value, enum value, table or field name, API method, intentional Harness product label, or a natural technical borrowing such as API, SDK, MCP, YAML, JSON, QA, or CLI.

Avoid English noun chains with Korean particles when the English is not an identifier or product label. Put the Korean concept first, then add the exact English value only when the reader needs contract precision or searchability.

Maintenance and documentation-check prose should stay stable after the editing context is gone. Use durable Korean conditions such as `변경된`, `편집된`, `문서 변경 시`, and `점검 대상` for general maintenance rules.

## 6. Examples Across Languages

When examples appear in paired documents, preserve the same scenario meaning while writing the Korean naturally. Korean examples may shorten repeated scenario phrases after the scenario is introduced, but related examples should stay consistent unless the scenario intentionally differs.

Do not preserve English noun order in Korean examples merely because the English source uses a compact noun chain. Keep exact refs, paths, method names, schema fields, status values, and product labels unchanged.

Use the [Authoring Guide](authoring-guide.md) and [API examples checks](checks/api-examples.md) for example quality rules, durable scenario checks, and internal consistency checks.

## 7. Hidden Anchors

Stable English anchors may be needed for existing links or external bookmarks. Preserve those anchors with hidden HTML anchors before a natural Korean heading.

Use this pattern:

```markdown
<a id="close-readiness"></a>
## 닫기 준비 상태
```

Do not make the visible Korean heading unnatural to match the English anchor. The anchor preserves link stability; the heading serves the reader.

Link text must match the visible heading's meaning. If the visible heading is `## 닫기 준비 상태`, use link text such as "닫기 준비 상태".

When changing headings in one language, check paired-language links and anchors in the same documentation update.

## 8. Review

After translation edits, run the focused Maintain checks instead of using this guide as a checklist:

- [Language parity checks](checks/language-parity.md) for meaning-unit parity, natural Korean structure, headings, tables, lists, and identifier preservation.
- [Terminology checks](checks/terminology.md) for terminology-map alignment, mixed-language Korean, documentation-routing terms, current-context wording, `complete` ambiguity, close-readiness wording, and related term controls.
- [Links and indexes checks](checks/links-and-indexes.md) when headings, anchors, relative links, terminology targets, or route metadata changed.
