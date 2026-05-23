# 문서 작성 가이드

## 이 가이드로 할 수 있는 일

Harness 문서를 새로 쓰거나, 나누거나, 이름을 바꾸거나, 리뷰할 때 이 가이드를 사용합니다.

목표는 재설계된 문서가 독자에게 읽기 쉽고, 세부 계약의 위치가 분명하며, 영어와 한국어 문서가 같은 의미를 유지하도록 돕는 것입니다.

이 가이드는 문서 유지보수만 다룹니다. Runtime behavior, server implementation, product state change, generated operational file, evidence record, QA result, acceptance decision, task close를 승인하거나 대체하지 않습니다.

## 문서 작성 원칙

문서는 독자의 다음 행동에서 출발합니다. 독자가 무엇을 이해하고, 결정하고, 사용하고, 구현하고, 검증하고, 유지해야 하는지 분명해야 합니다.

내부 구조를 빠짐없이 나열하기보다 핵심 아이디어를 적게, 선명하게 설명합니다. 엄격한 계약은 Reference 문서로 보내고, 다른 문서에서는 필요한 만큼만 요약한 뒤 링크합니다.

낯선 개념은 정의부터 던지지 않습니다. 먼저 실제 상황이나 짧은 예시로 왜 필요한지 보여주고, 그다음 이름과 정의를 붙입니다.

각 문서의 시작은 예측 가능해야 합니다. 독자가 이 문서로 무엇을 할 수 있는지, 언제 읽어야 하는지, 무엇을 먼저 알아야 하는지, 전체를 묶는 핵심 생각이 무엇인지 빨리 알 수 있어야 합니다.

현재 문서는 현재의 사실처럼 씁니다. 마이그레이션 과정, 제거된 구조, 예전 이름은 본문 설명에 넣지 않습니다. 마이그레이션 중 별도 migration note가 있는 경우에만 그곳에 두고, 그렇지 않으면 Git history나 명확히 표시된 비활성 migration 기록에 남깁니다.

## 문서 유형

### Learn

Learn 문서는 독자의 mental model을 만듭니다.

목적, 개념, 예시, trade-off를 구현 세부사항보다 먼저 설명합니다. 독자에게 command, schema, checklist보다 방향 감각이 필요할 때 사용합니다.

### Use

Use 문서는 사용자가 AI 지원 개발 세션에서 Harness를 따라가도록 돕습니다.

사용자에게 보이는 흐름, 상태 해석, 결정 지점, 복구 경로를 중심에 둡니다. Internal gate는 사용자가 보는 block이나 next action을 설명할 때만 이름 붙입니다.

### Build

Build 문서는 문서 재설계가 승인된 뒤 reference system을 구현하는 사람을 돕습니다.

구현 순서, module boundary, runnable slice, verification strategy를 설명합니다. 정확한 schema, DDL, invariant는 Reference 문서로 연결합니다.

### Reference

Reference 문서는 정확한 계약을 담당합니다.

Strict schema, gate, DDL, enum value, state transition, invariant, API shape, storage rule, projection rule, fixture semantics, official definition은 Reference 문서에 둡니다.

### Maintain

Maintain 문서는 문서 시스템 자체를 관리합니다.

Authoring rule, translation policy, review checklist, link hygiene, ownership map, documentation-maintenance expectation을 정의합니다. Maintain 문서가 runtime conformance spec이나 product implementation plan이 되면 안 됩니다.

## 문서 시작 방식

재설계된 문서는 짧고 예측 가능한 시작부를 둡니다. 길게 설명하지 않더라도 독자의 경로가 보여야 합니다.

### 이 문서로 할 수 있는 일

문서가 독자에게 주는 결과를 평범한 말로 씁니다. 무엇을 "다룬다"는 설명만으로 끝내지 않습니다.

### 이런 때 읽기

이 문서를 읽어야 하는 상황을 적습니다. 짧은 목록이어도 됩니다.

### 읽기 전에

필요한 사전 맥락, 먼저 읽을 문서, 전제 조건을 적습니다. 전제 조건이 없다면 간단히 없다고 말합니다.

### 핵심 생각

나머지 내용을 이해하기 쉽게 만드는 중심 모델이나 주장을 먼저 알려줍니다.

### Reference 범위

Reference 문서에만 둡니다. 이 문서가 어떤 정확한 계약을 담당하고, 무엇을 담당하지 않는지 밝힙니다. 이렇게 해야 Learn, Use, Build 문서로 엄격한 세부사항이 퍼지지 않습니다.

## 개념 소개 규칙

개념은 엄격한 정의보다 예시로 먼저 소개합니다.

구체적인 상황에서 시작해 어떤 문제를 해결하는지 보여준 뒤 개념 이름을 붙입니다. 독자가 왜 중요한지 본 다음에 엄격한 정의를 둡니다.

좋은 흐름:

```text
Agent가 제품 상태를 변경하려면 Harness는 먼저 그 write가 어떤 범위의 구현 단위에 속하는지 알아야 합니다. 그 단위가 Change Unit입니다. 사용자가 끝내거나 답을 얻고 싶은 더 큰 가치 단위가 Task입니다.
```

Learn 문서를 조밀한 정의 목록으로 시작하지 않습니다. Glossary나 reference table이라면 예외입니다.

## Reference 계약 규칙

Strict schema, gate, DDL, enum value, state transition, invariant, API shape, storage rule, projection contract detail, fixture semantics는 Reference 문서에 둡니다.

Learn, Use, Build, Maintain 문서는 필요할 때 계약을 한두 문장으로 요약하고 owner Reference 문서에 링크합니다. Full table, schema body, transition matrix, DDL block, fixture mini-language를 중복하지 않습니다.

## 반복 규칙

긴 source-of-truth 문단을 여러 문서에 반복하지 않습니다.

다른 문서에 같은 생각이 필요하면 짧게 요약하고 owner로 링크합니다. 원문이 바뀌면 owner를 먼저 고친 뒤 요약문이 drift하지 않았는지 확인합니다.

독자가 다른 예시를 필요로 한다면 설명용 예시는 반복할 수 있습니다. 하지만 규범적인 계약 문구를 반복하면 drift 위험이 큽니다.

## 다이어그램 규칙

Diagram은 cognitive load를 줄일 때만 사용합니다.

관계, 순서, 경계, lifecycle이 prose보다 그림으로 더 분명할 때 diagram이 유용합니다. 장식으로 넣거나, 이미 명확한 목록을 한 번 더 보여주거나, 아직 정리되지 않은 구조를 감추기 위해 쓰지 않습니다.

모든 diagram 근처에는 무엇을 봐야 하는지 알려주는 prose가 있어야 합니다. Diagram과 prose가 다르면 owner prose나 reference contract를 먼저 고칩니다.

## 영어/한국어 의미 일치 규칙

영어와 한국어 문서는 같은 active file map, 의미상 같은 section coverage, 같은 계약 세부사항을 유지해야 합니다.

영어/한국어 대응 문서는 같은 active file map과 의미상 같은 section coverage를 유지합니다. 다만 owner link, stable identifier, reviewability가 분명하다면 한국어 heading과 소단락 구성은 자연스러운 한국어가 되도록 조정할 수 있습니다. 의미상 같은 한국어 heading 차이는 docs-maintenance에서 자동 `FAIL`로 보지 않습니다. Official identifier, API name, schema name, enum value, DDL name, file name, error code, validator ID, code identifier, translation guide에 있는 product term은 정확히 유지합니다.

`docs/en`의 의미가 바뀌면 같은 batch에서 `docs/ko`도 반영합니다. 반대 방향도 같습니다.

## 링크와 이름 변경 규칙

문서 이름을 바꾸거나, 옮기거나, 나누거나, 합칠 때는 양쪽 언어의 링크를 같은 batch에서 고칩니다.

Secondary summary보다 owner document나 owner section으로 링크합니다. Active owner link가 제거된 migration context를 가리키면 안 됩니다.

Rename 뒤에는 old path, old anchor, old heading, old title text를 검색합니다. README path, 주변 cross-reference, appendix link, paired-language link를 함께 업데이트합니다.

## Docs-maintenance checks

Docs-maintenance checks는 read-only documentation maintenance입니다. Core fixture conformance, runtime validator, canonical state transition, projection refresh, generated operational report, QA result, acceptance record, evidence artifact, residual-risk acceptance, close readiness, implementation readiness가 아닙니다.

Docs-maintenance review 또는 future checker는 category, file path, 가능한 경우 heading 또는 anchor, owner document, observed drift, suggested fix, canonical state transition이 수행되지 않았다는 statement를 report해야 합니다. Drift는 먼저 owner를 update해서 해결하고, 그다음 non-owner duplicate를 짧은 summary plus owner link로 바꿉니다.

Result 의미:

| Result | Meaning |
|---|---|
| `FAIL` | Broken owner links, schema/DDL/enum/stable event/`ValidatorResult`/`ProjectionKind` mismatch, paired active file 누락, semantic section coverage 누락, owner contract를 다시 정의하는 non-owner text처럼 active docs를 contradictory하거나 non-actionable하게 만들 수 있는 drift입니다. Owner link, stable identifier, reviewability가 분명하다면 idiomatic heading text나 minor grouping difference는 실패가 아닙니다. |
| `WARN` | Minor glossary phrasing drift, normative하지 않은 duplicate explanatory prose, stale하지만 non-blocking인 cross-reference wording, incomplete하지만 still understandable한 TODO metadata처럼 정리해야 하지만 아직 owner contract와 모순되지는 않는 drift입니다. |
| `PASS` | 해당 category에서 relevant drift가 발견되지 않았습니다. |

필수 check categories:

| Category | Required check |
|---|---|
| English/Korean file structure parity | 명시적인 예외가 문서화되지 않는 한 `docs/en`과 `docs/ko`는 같은 active document paths를 유지합니다. |
| English/Korean semantic section parity | Paired files는 같은 active file map과 의미상 같은 section coverage를 유지합니다. Owner link, stable identifier, schema name, enum value, DDL name, validator ID, code identifier, reviewability가 분명하다면 heading text와 minor grouping은 자연스럽게 조정할 수 있습니다. |
| Broken cross-reference detection | Markdown links, heading anchors, appendix links, paired-language entry links가 active docs로 resolve됩니다. |
| Owner-boundary drift | Exact contract는 active owner에 머뭅니다. 여기에는 `reference/kernel.md`, `reference/mcp-api-and-schemas.md`, `reference/storage-and-ddl.md`, `reference/document-projection.md`, `reference/templates/*.md`, `reference/design-quality-policies.md`, `reference/operations-and-conformance.md`, `reference/glossary.md`가 포함됩니다. |
| Fixture/action schema drift | Operations fixture examples의 `action`과 executable `input`은 `reference/mcp-api-and-schemas.md`의 public MCP request schemas 및 `reference/operations-and-conformance.md`의 `ToolEnvelope` expansion convention과 aligned되어야 합니다. Fixture semantics는 여기서 restate하지 않고 link합니다. |
| Enum, event, validator, projection drift | State/gate/result values와 Kernel Stable Event Catalog names는 `reference/kernel.md`, error와 stable `ValidatorResult` IDs는 `reference/mcp-api-and-schemas.md`, storage values는 `reference/storage-and-ddl.md`, `ProjectionKind` tiers는 `reference/document-projection.md` 및 `reference/templates/*.md`와 일치해야 합니다. |
| Glossary and source-of-truth phrasing drift | Official terms, capitalization, record ID prefixes, source-of-truth boundaries는 `reference/glossary.md`와 일치하고 추가 state authority를 암시하지 않아야 합니다. |
| TODO compliance | `TODO_DECISION`과 `TODO_IMPLEMENT`는 allowed meaning으로 쓰고 gap을 명확히 naming하며, finished canonical sections에 `TODO_REWRITE` marker를 남기지 않습니다. |
| Non-owner duplicate full contracts | Owner doc 밖의 full schema, DDL, transition table, fixture mini-language, template body, glossary definition은 짧은 summary plus owner link로 바꿉니다. |

## 리뷰 체크리스트

```text
[ ] 이 문서는 분명한 독자 상황을 돕는가?
[ ] 시작부가 표준 패턴을 따르는가?
[ ] 개념을 엄격한 정의보다 예시로 먼저 소개하는가?
[ ] strict schema, gate, DDL, enum, invariant가 Reference 문서에 머무는가?
[ ] 긴 source-of-truth 문단을 반복하지 않고 요약과 링크로 처리했는가?
[ ] diagram이 cognitive load를 줄이는가?
[ ] 영어와 한국어 파일이 의미상 일치하는가?
[ ] official identifier가 정확히 보존되었는가?
[ ] renamed path, anchor, README link가 양쪽 언어에서 업데이트되었는가?
[ ] 현재 사실과 migration history가 분리되어 있는가?
[ ] Maintain 문서가 runtime behavior가 아니라 documentation governance에 머무는가?
```

## Reference ownership map

정확한 세부사항을 어디에 둘지 판단할 때 이 map을 사용합니다. 이 map은 재설계된 문서 구조의 active owner를 식별하며, retired paths가 authoring workflow에 남지 않게 합니다.

| Subject | Active owner |
|---|---|
| Entrypoint, reader paths, document list, target tree summary | `README.md` |
| Shared reader mental model and three-space overview | `learn/overview.md` |
| Small core concept introduction | `learn/concepts.md` |
| Project purpose, target users, values, scope, non-goals, automation philosophy | `learn/purpose-and-principles.md` |
| Strategic thesis, failure model, MVP boundary, principle groups | 독자 설명은 `learn/purpose-and-principles.md`; exact contract impact는 `reference/design-quality-policies.md`와 `reference/kernel.md` |
| Kernel entities, lifecycle, gates, state transitions, close semantics, `prepare_write`, `close_task` | `reference/kernel.md` |
| Runtime architecture, three spaces in implementation detail, Core process model, artifact architecture, projection/reconcile architecture, guarantee levels | `reference/runtime-architecture.md` |
| MCP resources/tools, request/response schemas, error taxonomy, validator result schema, artifact ref shape | `reference/mcp-api-and-schemas.md` |
| SQLite DDL, migrations, storage layout, lock policy, artifact directory layout, baseline capture format, projection job table | `reference/storage-and-ddl.md` |
| MVP implementation order and stage exit criteria | `build/mvp-plan.md` |
| First runnable implementation slice | `build/first-runnable-slice.md` |
| Markdown projection principles, authority matrix, managed blocks, human-editable sections, artifact reference rendering, template tiers, projection freshness/failure rules | `reference/document-projection.md` |
| 모든 projection template 본문과 표시 카드 형태 | `reference/templates/*.md` |
| 설계 품질 정책 계약, validator ID, severity composition 규칙, 정책 waiver 의미, 근거 기대사항, close 영향 | `reference/design-quality-policies.md` |
| User-facing conversation, status reading, user judgments, close checklist | `use/user-guide.md` |
| User/agent session procedure | `use/agent-session-flow.md` |
| Agent surface capability profiles, common connector contract, fallback semantics, Role Lens, connector conformance overview | `reference/agent-integration.md` |
| Surface-specific recipes | `reference/surface-cookbook.md` |
| Generic capability profile examples | `reference/agent-integration.md` |
| Operator procedures, conformance fixture bodies, fixture assertion semantics, doctor/recover/reconcile/export/artifact integrity, docs-maintenance reporting | `reference/operations-and-conformance.md` |
| Official term definitions and capitalization | `reference/glossary.md` |
| Post-MVP roadmap | `roadmap.md` |
| Documentation authoring rules | `maintain/authoring-guide.md` |
| Translation and bilingual prose rules | `maintain/translation-guide.md` |
