# Translation policy

Use this policy when editing paired English and Korean Harness documentation. It
owns bilingual semantic-parity guidance, natural Korean technical prose,
identifier preservation practice, terminology-map usage, and paired-language
completion expectations.

This is a maintenance policy. It does not define product behavior, API
behavior, storage effects, security guarantees, runtime behavior, schema
contracts, glossary entries, or owner-routing indexes.

## Semantic Parity

English and Korean documents are both maintained. Neither language is an
archive, appendix, or translation-only copy.

Maintain parity by meaning unit, not by line count or sentence count. A meaning
unit can be a rule, warning, paragraph, table row, example, route, list item, or
heading. Korean may change sentence order, split or combine sentences, or use a
different paragraph rhythm when that makes the meaning clearer.

Semantic parity requires the same reader purpose, normative strength, baseline
and out-of-scope boundary, owner routing, user-judgment boundary, evidence
boundary, verification boundary, acceptance boundary, residual-risk boundary,
security guarantee level, exact identifiers, and exact product labels where
those items are present in the paired material.

Do not finish a meaning-changing edit with only one language updated. If Korean
editing exposes an English problem, fix the English too. If English editing
introduces a product concept, add the natural Korean equivalent in the paired
Korean document during the same documentation update.

## Korean Prose

Korean documentation should read as native Korean technical documentation, not
mirrored English. Use natural headings, short explanatory sentences, Korean
concept-first phrasing, consistent terminology, enough reader context, and exact
identifiers in backticks where needed.

Translate ordinary English nouns and noun phrases into Korean prose. Keep
English unchanged only when it is an exact identifier, file path, anchor, code
literal, schema value, enum value, table or field name, API method, intentional
Harness product label, or a natural technical borrowing such as API, SDK, MCP,
YAML, JSON, QA, or CLI.

Avoid English noun chains with Korean particles when the English is not an
identifier or product label. Put the Korean concept first, then add the exact
English value only when the reader needs contract precision or searchability.

## Identifiers And Terminology

Use [`docs/terminology-map.yaml`](../../terminology-map.yaml) before adding or
changing product terms, Korean prose terms, identifier explanations, exact
product labels, or Korean mixed-language controls.

Preserve exact identifiers unchanged in English and Korean. Put code-like,
schema-like, route-like, or search-critical values in backticks when they appear
in prose. Do not translate exact strings inside code blocks, schema examples,
API examples, file paths, field lists, literal-value tables, or machine-readable
metadata.

Apply the terminology map's distinctions, including:

- Harness is the local work-authority product/system for AI-assisted product
  work; Core is the local authority record for Harness state.
- Use "verification criteria" for user-visible criteria used to check work, and
  "검증 기준" in Korean.
- Use "current scope" or "currently applied scope" in prose context, and
  "현재 적용 범위" in Korean. Preserve exact identifiers and status values that
  contain `active`.
- Keep the exact label `Write Authorization` distinct from ordinary write
  approval. In Korean explanatory prose, use "쓰기 권한 부여" for
  `Write Authorization` and "쓰기 승인" for ordinary write approval.
- In Korean reference prose, use "닫기 준비 상태" for close readiness.

Some English words can be both code values and ordinary prose. Preserve
`complete` in backticks only when it is an identifier, such as
`intent=complete`. When the English means full or entire, English prose should
prefer "full" or "entire" and Korean prose should use the terminology map's
ordinary-prose replacement.

## Strength And Structure

Preserve negative clauses, non-claims, prohibitions, exceptions, and guarantee
strength by meaning. A Korean sentence may move a clause for readability, but it
must not soften, strengthen, or drop the paired English meaning.

Headings, tables, lists, and examples must be equivalent by meaning. They do not
need identical line breaks or sentence counts, but they must preserve the same
scope, conditions, consequences, identifiers, links, and examples. Do not add
Korean-only labels such as `조건`, `결과`, `비주장`, or `허용되지 않는 것`
unless the English document has the same meaning unit.

Commands, flags, identifiers, status meanings, negative clauses, limitations,
warnings, and local reader routes must remain equivalent by meaning.

When examples appear in paired documents, preserve the same scenario meaning
while writing Korean naturally. Keep refs, paths, method names, schema fields,
status values, and product labels unchanged.

Stable English anchors may be needed for existing links or external bookmarks.
In Korean, preserve those anchors with hidden HTML anchors before a natural
Korean heading. Do not make the visible Korean heading unnatural to match the
English anchor.

## Completion

Every maintained paired Markdown document under `docs/en/` and `docs/ko/` must
use mirrored language-relative paths and have an indexed pair in
`doc-index.yaml`. Its version 3 entry carries the paired paths plus required
maintenance `owner_area`, date, and `applies_to` metadata for maintainers;
ordinary readers still use the language entry pages and reader-facing routes.

The exact root pair `README.md` and `README.ko.md` is also a maintained
semantic-parity pair when it is registered in `doc-index.yaml`. This exception
applies only to that exact root README pair; it does not permit arbitrary
root-level language-pair names.

For normal lookup, read the language that matches the request or the default
language in `doc-index.yaml`. For translation review, bilingual parity review,
or terminology-affecting edits, read both language versions and compare by
meaning unit.
