# 작성 가이드

## 문서 역할

이 문서는 하네스 문서 세트를 작고, 구현 가능하고, 올바르게 계층화된 상태로 유지하는 규칙을 담당한다.

Runtime behavior, user procedure, conformance fixture content, MCP schema, SQLite DDL, projection template은 담당하지 않는다.

## Ownership Boundaries

각 concept에는 정확히 하나의 canonical owner를 사용한다. 다른 문서는 한 문장 요약과 link를 포함할 수 있다.

| 계층 | Canonical owner |
|---|---|
| one-sentence definition, reader paths, document list, target tree summary | `README.md` |
| shared reader mental model, three-space summary, core concepts introduction | `00-introduction.md` |
| project purpose, target users, values, scope, non-goals, automation philosophy | `01-project-charter.md` |
| why, failure model, MVP boundary, 7 core invariants, policy defaults list | `02-strategy.md` |
| entity meanings, lifecycle, gates, state transitions, close semantics, `prepare_write` and `close_task` logic | `03-kernel-spec.md` |
| three spaces, runtime authority flow, artifact architecture, projection/reconcile architecture, guarantee levels | `04-runtime-architecture.md` |
| MCP resources/tools, request/response schemas, error taxonomy, validator result schema, artifact ref shape | `05-mcp-api-and-schemas.md` |
| reference MVP implementation order, SQLite DDL, migrations, storage layout, validator runner skeleton | `06-reference-mvp.md` |
| Markdown projection principles, managed blocks, human-editable sections, template tiers, template summaries | `07-document-projection.md` |
| shared design, domain language, vertical slice, TDD, module/interface, Manual QA, context hygiene policies | `08-design-quality-policy-pack.md` |
| agent surface capability profile, common connector contract, fallback semantics | `09-agent-integration.md` |
| user-facing conversation, status reading, resume procedure, approval/assurance/QA/acceptance explanation | `10-user-guide.md` |
| connect, doctor, serve MCP, projection refresh, reconcile, recover, export, artifact integrity, conformance | `11-operations-and-conformance.md` |
| full templates and expanded variants | `appendix/A-template-library.md` |
| surface-specific cookbooks | `appendix/B-surface-cookbook.md` |
| later automation and derived analytics | `appendix/C-later-roadmap.md` |
| old-to-new mapping and migration notes | `appendix/D-migration-notes.md` |
| official term definitions | `glossary.md` |

## Core Invariant vs Policy Default

Core invariant는 `02-strategy.md`가 담당하는 일곱 항목으로 제한된다. Rewrite-control decision이 업데이트되지 않은 상태에서 helpful practice를 kernel invariant로 승격하지 않는다.

Core invariant 문구는 mandatory하고 structural하게 들려야 한다.

```text
Product write requires an active scoped Change Unit.
Projection cannot override canonical state.
```

Policy default 문구는 applicability, waiver, record, validator, close impact를 밝혀야 한다.

```text
Vertical slice is the default for feature work when it applies.
A horizontal exception may be recorded with a reason and follow-up.
```

현재 policy default는 shared design, domain language consistency, vertical slice default, suitable work에 대한 TDD trace, module/interface review, Manual QA, context hygiene다.

## MVP, v1, Later Label

이 label을 일관되게 사용한다.

| Label | 의미 |
|---|---|
| MVP | reference implementation이 core invariant를 검증하는 데 required |
| v1 | MVP 이후의 plausible next version이며, 여전히 fixture와 ownership 필요 |
| later | MVP requirement처럼 읽히면 안 되는 유용한 future automation |

규칙:

- Main doc은 later work를 non-MVP context로만 언급할 수 있으며 `appendix/C-later-roadmap.md`를 가리켜야 한다.
- Appendix C의 later-automation item이나 team workflow expansion을 MVP requirement에 넣지 않는다.
- later item이 v1이 되면 main doc을 바꾸기 전에 conformance expectation과 owner를 추가한다.
- Derived metric은 MVP-critical conformance signal로 명시적으로 승격되지 않는 한 analytics다.

## Source-Of-Truth 표현

다음 표현 계열을 사용한다.

```text
Operational state is canonical in state.sqlite current records plus state.sqlite.task_events.
Raw evidence is canonical in the artifact store.
Markdown reports are projections generated from state records and artifact refs.
Human-editable sections are input surfaces.
Accepted human edits become state only through reconcile or a Core state-changing action.
```

별도의 MVP event store가 있음을 암시하는 표현은 피한다.

```text
phrases that put state.sqlite beside a separate event log
```

Historical comparison에 이 개념이 필요하면 MVP event history가 `state.sqlite.task_events`임을 즉시 명확히 한다.

다음처럼 쓰지 않는다.

```text
TASK is canonical state.
Projection updates state.
User Notes are the source-of-truth.
Domain Language is canonical in the Markdown document.
Report projections are raw artifacts by default.
```

선호하는 authority path:

```text
User Notes: human-editable input -> reconcile_items -> accepted state event/record
Domain Language: domain_terms -> DOMAIN-LANGUAGE projection
Module Map: module_map_items -> MODULE-MAP projection
Interface Contract: interface_contracts -> INTERFACE-CONTRACT projection
```

## Schema And Template Ownership

MCP tool request/response schema, common envelope, error taxonomy, validator result schema, artifact ref shape는 `05-mcp-api-and-schemas.md`에만 둔다.

SQLite DDL, migration/versioning, lock policy, artifact directory layout, reference implementation storage detail은 `06-reference-mvp.md`에만 둔다.

Projection rule과 template tier는 `07-document-projection.md`에 둔다. Full template body와 expanded report variant는 `appendix/A-template-library.md`에 둔다.

User-facing example은 status card나 짧은 report snippet을 보여줄 수 있지만, schema definition이 되면 안 된다.

## Current-State Writing

Canonical doc은 rewrite history가 아니라 current truth로 쓴다.

선호:

```text
The harness uses lifecycle fields plus gates.
```

Main doc에서 피할 표현:

```text
Unlike the old version, the harness now uses lifecycle fields plus gates.
```

Version comparison, removed section, old file name은 `appendix/D-migration-notes.md`에 둔다.

## Cross-Reference Rules

Contract를 중복하지 말고 link로 owner를 가리킨다.

최소 reference:

- Strategy는 kernel과 policy pack을 참조한다.
- Kernel은 API와 reference MVP를 참조한다.
- Runtime architecture는 kernel, projection, integration을 참조한다.
- API는 kernel과 reference MVP를 참조한다.
- Reference MVP는 kernel, API, operations를 참조한다.
- Projection은 kernel과 Appendix A를 참조한다.
- Policy pack은 kernel과 projection을 참조한다.
- Integration은 API와 Appendix B를 참조한다.
- Operations는 API와 reference MVP를 참조한다.

## TODO Rules

실제 product 또는 architecture decision이 미해결일 때만 `TODO_DECISION`을 사용한다. 필요한 decision, 영향받는 doc, likely owner를 포함한다.

Decision은 이미 내려졌지만 implementation detail, DDL, fixture coverage, CLI behavior가 아직 채워지지 않았을 때만 `TODO_IMPLEMENT`를 사용한다.

완성된 v2 canonical section에는 `TODO_REWRITE`를 사용하지 않는다. 남아 있는 `TODO_REWRITE`는 해당 section이 아직 migration stub이라는 뜻이다.

## Authoring Checklist

```text
[ ] Does this concept have exactly one canonical owner?
[ ] Are schema and DDL kept in their owner docs?
[ ] Are core invariants still exactly the approved seven?
[ ] Are policy defaults written with applicability and waiver boundaries?
[ ] Are MVP, v1, and later labels clear?
[ ] Are long-term analytics kept out of MVP requirements?
[ ] Does source-of-truth phrasing preserve state/artifact/projection boundaries?
[ ] Does the user guide avoid DB/API/connector internals?
[ ] Does operations use fixture-based conformance?
[ ] Are legacy names confined to migration notes?
[ ] Are official terms aligned with glossary?
```
