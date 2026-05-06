# 하네스 문서 세트

하네스는 AI와 함께 하는 개발 작업을 명시적 상태, 범위, 근거, 검증, 사람의 판단으로 운영하게 하는 로컬 커널이다.

이 파일은 `docs/README.md`이며 하네스 문서의 entrypoint다. 저장소 루트의 `README.md`는 repository landing page다.

## Core Invariants

The approved seven kernel invariants are owned by [02-strategy.md](02-strategy.md#core-invariants). They are distinct from design-quality policy defaults.

## Reader Paths

일반 사용자:

```text
00-introduction.md
-> 10-user-guide.md
```

구현자:

```text
00-introduction.md
-> 02-strategy.md
-> 03-kernel-spec.md
-> 04-runtime-architecture.md
-> 05-mcp-api-and-schemas.md
-> 06-reference-mvp.md
-> 11-operations-and-conformance.md
```

Connector 작성자:

```text
09-agent-integration.md
-> appendix/B-surface-cookbook.md
-> 11-operations-and-conformance.md
```

Projection 관리자:

```text
07-document-projection.md
-> appendix/A-template-library.md
-> 11-operations-and-conformance.md
```

설계 품질 책임자:

```text
02-strategy.md
-> 08-design-quality-policy-pack.md
-> 11-operations-and-conformance.md
```

문서 작성자:

```text
99-authoring-guide.md
-> glossary.md
```

## MVP / v1 / Later

MVP는 많은 agent surface를 동시에 지원하는 플랫폼이 아니라 core invariant를 검증하는 작은 로컬 운영 커널이다.

MVP focuses on one reference surface, local state, artifacts, public MCP tools, write gating, evidence, verification, Manual QA, acceptance, projections, reconcile, recovery, export, and fixture-based conformance.

Later automation is cataloged in [appendix/C-later-roadmap.md](appendix/C-later-roadmap.md) and must not read as part of MVP scope.

## Target Tree

```text
docs/
  README.md
  00-introduction.md
  01-project-charter.md
  02-strategy.md
  03-kernel-spec.md
  04-runtime-architecture.md
  05-mcp-api-and-schemas.md
  06-reference-mvp.md
  07-document-projection.md
  08-design-quality-policy-pack.md
  09-agent-integration.md
  10-user-guide.md
  11-operations-and-conformance.md
  99-authoring-guide.md
  glossary.md

  appendix/
    A-template-library.md
    B-surface-cookbook.md
    C-later-roadmap.md
    D-migration-notes.md
```

## Main Documents

| 문서 | 소유 역할 |
|---|---|
| [00-introduction.md](00-introduction.md) | 사용자와 구현자가 공유하는 정신 모델 |
| [01-project-charter.md](01-project-charter.md) | 프로젝트 목적, 대상, 가치, 범위, 비목표 |
| [02-strategy.md](02-strategy.md) | 전략적 thesis, failure model, core invariants, policy defaults |
| [03-kernel-spec.md](03-kernel-spec.md) | 운영 커널, entity, lifecycle, gates, transitions, close semantics |
| [04-runtime-architecture.md](04-runtime-architecture.md) | 세 공간, runtime home, Core, artifact, projection/reconcile architecture |
| [05-mcp-api-and-schemas.md](05-mcp-api-and-schemas.md) | MCP resources/tools, schemas, errors, validators, artifact refs |
| [06-reference-mvp.md](06-reference-mvp.md) | MVP implementation sequence, DDL, storage layout, validator skeleton |
| [07-document-projection.md](07-document-projection.md) | Markdown projection, managed/human-editable areas, template tiers |
| [08-design-quality-policy-pack.md](08-design-quality-policy-pack.md) | design-quality policies as policy contracts |
| [09-agent-integration.md](09-agent-integration.md) | agent surface integration and capability profile |
| [10-user-guide.md](10-user-guide.md) | user conversation phrases, status reading, judgments, resume |
| [11-operations-and-conformance.md](11-operations-and-conformance.md) | operator procedures and fixture-based conformance |
| [99-authoring-guide.md](99-authoring-guide.md) | document ownership and authoring rules |
| [glossary.md](glossary.md) | official terms |

## Appendices

| 문서 | 소유 역할 |
|---|---|
| [appendix/A-template-library.md](appendix/A-template-library.md) | full template library and expanded report variants |
| [appendix/B-surface-cookbook.md](appendix/B-surface-cookbook.md) | surface-specific connector notes and profile examples |
| [appendix/C-later-roadmap.md](appendix/C-later-roadmap.md) | later automation and post-MVP roadmap |
| [appendix/D-migration-notes.md](appendix/D-migration-notes.md) | migration context only; not an active canonical owner |

## Rewrite Control

`docs/rewrite-control/` is the coordination area for this rewrite. It records decisions and review checklists, but it is not runtime product documentation. Migration history and legacy mapping are summarized in [appendix/D-migration-notes.md](appendix/D-migration-notes.md).
