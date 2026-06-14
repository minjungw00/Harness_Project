# Translation guide

Use this guide when editing paired English and Korean Harness documentation. This guide is only a documentation-maintenance standard. It is not runtime conformance, implementation authority, a QA result, an acceptance decision, close-readiness state, or a generated Harness artifact.

The canonical terminology map is [`docs/terminology-map.yaml`](../../terminology-map.yaml). Check it before adding or changing product terms, Korean prose terms, identifier explanations, or mixed-language bans. If this guide and the terminology map disagree, stop and align them in the same documentation batch.

## 1. Semantic parity

English and Korean documents are both maintained. Neither language is an archive, appendix, or translation-only copy.

Maintain parity by meaning unit, not by line count. A meaning unit can be a rule, warning, paragraph, table row, example, route, or checklist item. The Korean page may change sentence order, split or combine sentences, or use different paragraph rhythm when that makes the meaning clearer in Korean.

Semantic parity requires:

- the same reader purpose
- the same normative strength
- the same baseline/out-of-scope boundary
- the same treatment of owner references already present in the paired material
- the same treatment of user-judgment, evidence, verification, acceptance, and residual-risk boundaries already present in the paired material
- the same treatment of security guarantee wording already present in the paired material
- the same exact identifiers

Do not finish a meaning-changing batch with only one language updated. If Korean editing exposes an English problem, fix the English too. If English editing introduces a product concept, add the natural Korean equivalent in the paired Korean document during the same batch.

## 2. Document pair and route parity

Every major maintained page should have an English/Korean pair at the matching route under `docs/en/` and `docs/ko/`. Paired documents do not need matching line numbers, but they must keep matching scope and reader intent.

Route and navigation text must use the compact maintained structure:

- `docs/doc-index.yaml`
- `docs/*/start.md`
- `docs/*/use/user-guide.md`
- `docs/*/use/agent-guide.md`
- `docs/*/use/judgment-examples.md`
- `docs/*/build/implementation-guide.md`
- `docs/*/reference/README.md`
- `docs/*/maintain/authoring-guide.md`
- `docs/*/maintain/translation-guide.md`
- `docs/*/maintain/checks.md`
- `docs/*/maintain/checks/*.md`

Owner lookup belongs to the Authoring Guide and `docs/doc-index.yaml`. In translation work, preserve existing owner references by meaning and do not add contract detail to this guide.

During normal agent work, load only one language for the same `doc_id`. Load both languages only for translation, parity review, or a bilingual edit where comparison is necessary.

## 3. Identifier preservation

`docs/terminology-map.yaml` owns the systematic identifier classes, exact identifier lists, and exact product labels. Preserve those values unchanged in English and Korean. Put code-like, schema-like, route-like, or search-critical values in backticks when they appear in prose.

Do not translate exact strings inside code blocks, schema examples, API examples, file paths, field lists, literal-value tables, or machine-readable metadata. Localized Korean display labels are reader-facing text; they do not replace canonical identifiers.

Use this distinction:

- Identifier: `ArtifactRef`
- Korean explanation: 아티팩트 참조 스키마
- Prose term: 아티팩트

Some English words can be both code values and ordinary adjectives. Determine the context before preserving the word. Preserve `complete` in backticks only when it is an identifier, such as `intent=complete`. When the English means full or entire, English prose should prefer "full" or "entire" and Korean prose should use natural phrases such as 전체, 전체 평가, or 전체 평가 순서.

## 4. Product concept terminology

Use [docs/terminology-map.yaml](../../terminology-map.yaml) as the canonical terminology map for product concepts and mixed-language bans. This guide gives prose-level translation guidance; the map is the machine-readable control file.

Use one Korean term for one concept unless the terminology map explicitly distinguishes user-facing and reference-facing wording. This table is a prose style aid, not an owner route or product contract.

| English concept | Korean reference prose | Korean user-facing or style note |
|---|---|---|
| close readiness | 닫기 준비 상태 | 닫기 가능 여부 in user-facing explanations |
| close readiness evaluation | 닫기 준비 상태 평가 | Avoid mixed close/가능성 wording. |
| close task | Task 닫기 | Use `harness.close_task` only for the exact method identifier. |
| close-readiness blocker | 닫기 차단 사유 | Use `CloseReadinessBlocker` only for the schema identifier. |
| blocker category | 차단 사유 범주 | Use `CloseReadinessBlocker.category` only for the exact field. |
| full or entire evaluation order | 전체 평가 순서 | Use 전체 닫기 준비 상태 평가 순서 in close-readiness context. |
| artifact | 아티팩트 | Preserve exact artifact schema identifiers. |
| evidence | 증거 | Preserve exact evidence schema identifiers. |
| surface | 접점 | Preserve exact surface identifiers. |
| active surface context | 현재 적용 접점 맥락 | Preserve `VerifiedSurfaceContext` when naming the identifier. |
| lifecycle | 생명주기 | Leave lifecycle in English only when it is an identifier. |
| projection | 상태 보기 | Use `Projection` when the exact Harness label matters. |
| user-owned judgment | 사용자 소유 판단 | 사용자 판단 may fit user-facing prose. |
| sensitive-action approval | 민감 동작 승인 | Keep `Write Authorization` as the exact label when that label is named. |
| cooperative guarantee | 협력형 보장 | Use the same guarantee-strength wording as the source. |
| detective guarantee | 탐지형 보장 | Use the same guarantee-strength wording as the source. |
| baseline scope | 기준 범위 | Keep distinct from 현재 적용 범위. |
| supported scope | 지원 범위 | 지원되는 범위 may be more natural as a modifier. |
| supported behavior | 지원 동작 | Keep distinct from currently applied runtime state. |
| active scope | 현재 적용 범위 | Preserve exact identifiers when present. |
| active Change Unit | 현재 적용 Change Unit | Keep Change Unit as the product term. |
| owner document | 담당 문서 | Use for documentation ownership prose. |
| owner contract | 담당 계약 | 담당 문서가 정의한 계약 may be clearer in prose. |
| applicable owner path | 적용되는 담당 경로 | Use for documentation-routing prose. |
| applicable reference | 적용되는 참조 문서 | Use when the sentence points to a reference document. |
| error routing | 오류 처리 경로 | Use exact owner paths only when naming paths. |
| blocker routing | 차단 사유 처리 경로 | Avoid blocker 라우팅 in Korean prose. |
| out-of-scope capability | 지원 범위 밖 기능 | Keep the Korean term concept-first. |
| migration | 마이그레이션 | Use for technical migration contexts. |

When a term is missing, add it to the terminology map before spreading a new variant across the docs.

## 5. General prose translation

Translate ordinary English nouns and noun phrases into Korean prose. Do not preserve English just because the English source used a compact noun phrase.

Use English unchanged only when it is:

- an exact identifier
- a file path or anchor
- a code literal, schema value, enum value, table/field name, or API method
- an intentional Harness product label that must remain searchable
- an industry term that is more natural in Korean as borrowed technical vocabulary, such as API, SDK, MCP, YAML, JSON, QA, or CLI

Avoid "English noun + Korean particle" when the English noun is not an identifier. Prefer a Korean concept first, then add the exact English value only if the reader needs contract precision.

Examples:

| Avoid | Use |
|---|---|
| artifact를 저장한다 | 아티팩트를 저장한다 |
| evidence를 기록한다 | 증거를 기록한다 |
| surface에서 보인다 | 접점에서 보인다 |
| lifecycle 의미 | 생명주기의 뜻 |
| staged handle을 전달한다 | 스테이징된 아티팩트 핸들을 전달한다 |
| surface 정보 | 접점 정보 |

## 6. Korean technical writing style

Korean documents should read as native Korean technical documentation, not as mirrored English.

Write Korean with:

- natural Korean headings
- short explanatory sentences
- Korean concept-first phrasing in user-facing prose
- consistent terms from the terminology map
- enough context that the Korean reader does not need to mentally reconstruct the English
- exact identifiers preserved in backticks where needed

Do not mirror English sentence order when it produces stiff translationese. It is acceptable to reorder clauses, split long English sentences, combine short repetitive sentences, and replace English abstract nouns with Korean verbs when the meaning stays aligned.

Visible Korean headings should be natural Korean. Do not keep an English heading visible only to preserve an existing link. Use the hidden anchor policy instead.

## 7. Hidden anchor policy

Stable English anchors may be needed for existing links, old references, or external bookmarks. Preserve those anchors with hidden HTML anchors before a natural Korean heading.

Use this pattern:

```markdown
<a id="close-readiness"></a>
## 닫기 준비 상태
```

Do not make the visible Korean heading unnatural to match the English anchor. The anchor preserves link stability; the heading serves the reader.

Link text must match the visible heading's meaning. If the visible heading is `## 닫기 준비 상태`, use link text such as "닫기 준비 상태", not "close readiness" or "close 가능성".

When changing headings in one language, check paired-language links and anchors in the same batch.

## 8. User-facing vs reference-facing terminology

User-facing docs explain what the reader can decide, expect, or do. Reference-facing docs define contracts, schemas, owner boundaries, and exact behavior. Korean terminology may differ by audience while preserving the same product meaning.

Use user-facing Korean when the reader needs a plain operational meaning:

- 닫기 가능 여부
- 확인한 것
- 다음 안전한 행동
- 에이전트가 스스로 판단해도 되는 범위
- 하네스가 확인할 수 있는 수준

Use reference-facing Korean when the page defines a product concept or contract:

- 닫기 준비 상태
- 닫기 준비 상태 평가
- 닫기 차단 사유
- 사용자 소유 판단
- 증거, 증거 요약
- 협력형 보장, 탐지형 보장, 예방형 보장
- `Projection`(읽기 전용 상태 보기) on first reference when the exact label matters

Do not expose raw enum names or schema fields as user-facing labels unless the exact raw value is the subject. A Korean display label is localized text, not a replacement for the canonical value.

## 9. Forbidden mixed-language patterns

The following patterns are forbidden in Korean prose unless they appear inside a code block or are being cited as a bad example in this guide.

| Forbidden | Use instead |
|---|---|
| close 가능성 평가 | 닫기 준비 상태 평가 |
| 닫기 가능성 평가 | 닫기 준비 상태 평가 |
| `complete` 평가 순서 | 전체 평가 순서 |
| complete 평가 순서 | 전체 평가 순서 |
| `complete` 닫기 준비 상태 순서 | 전체 닫기 준비 상태 평가 순서 |
| complete 닫기 준비 상태 순서 | 전체 닫기 준비 상태 평가 순서 |
| artifact 저장 | 아티팩트 저장, or 아티팩트를 저장 |
| artifact bytes | 아티팩트 본문 바이트 |
| evidence 기록 | 증거 기록 |
| evidence summary | 증거 요약, or `EvidenceSummary` when naming the schema |
| evidence collection workflow | 증거 수집 흐름 |
| staged handle | 스테이징된 아티팩트 핸들, or `StagedArtifactHandle` when naming the identifier |
| checksum, size 검증 | 체크섬과 크기 검증 |
| ToolEnvelope 봉투 | `ToolEnvelope` 요청 래퍼 |
| lifecycle 의미 | 생명주기 의미, or 생명주기의 뜻 |
| surface 정보 | 접점 정보, or `surface_id` when naming the field |
| close blocker를 확인한다 | 닫기 차단 사유를 확인한다 |
| blocker 처리 경로 | 차단 사유 처리 경로 |
| blocker 라우팅 | 차단 사유 처리 경로 |
| migration = 이전 선택 | technical migration context: 마이그레이션 |

Mixed English/Korean may be correct when the English part is an identifier, for example `ArtifactRef`를 보존한다 or `surface_id` 필드를 보존한다. When the English part is ordinary prose, translate it.

## 10. Review

After translation edits, run the focused Maintain checks instead of using this guide as a checklist:

- [Language parity checks](checks/language-parity.md) for meaning-unit parity, natural Korean structure, headings, tables, lists, and identifier preservation.
- [Terminology checks](checks/terminology.md) for terminology-map alignment, mixed-language Korean, glossary card focus, `active` wording, `complete` ambiguity, and related term controls.
- [Links and indexes checks](checks/links-and-indexes.md) when headings, anchors, relative links, terminology targets, or route metadata changed.
