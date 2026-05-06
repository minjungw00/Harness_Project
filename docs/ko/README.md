# 하네스 문서 세트

하네스는 AI와 함께 하는 개발 작업을 명시적 상태, 범위, 근거, 검증, 사람의 판단으로 운영하게 하는 로컬 커널이다.

이 파일은 `docs/README.md`이며 하네스 문서의 시작점이다. 저장소 루트의 `README.md`는 저장소 랜딩 페이지다.

## 핵심 불변식

승인된 일곱 가지 커널 불변식의 정식 소유 문서는 [02-strategy.md](02-strategy.md#core-invariants)다. 이 불변식은 설계 품질 정책 기본값과 구분된다.

## 독자 경로

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

MVP는 여러 agent surface를 동시에 지원하는 플랫폼이 아니라 core invariant를 검증하는 작은 로컬 운영 커널이다.

MVP는 하나의 reference surface, 로컬 상태, artifact, public MCP tool, write gating, evidence, verification, Manual QA, acceptance, projection, reconcile, recovery, export, fixture 기반 conformance에 집중한다.

이후 자동화는 [appendix/C-later-roadmap.md](appendix/C-later-roadmap.md)에 정리하며, MVP 범위의 일부처럼 읽히면 안 된다.

## 대상 트리

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

## 주요 문서

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
| [08-design-quality-policy-pack.md](08-design-quality-policy-pack.md) | policy contract로서의 설계 품질 정책 |
| [09-agent-integration.md](09-agent-integration.md) | agent surface integration과 capability profile |
| [10-user-guide.md](10-user-guide.md) | user conversation phrases, status reading, judgments, resume |
| [11-operations-and-conformance.md](11-operations-and-conformance.md) | operator procedures와 fixture 기반 conformance |
| [99-authoring-guide.md](99-authoring-guide.md) | document ownership과 authoring rules |
| [glossary.md](glossary.md) | 공식 용어 |

## 부록

| 문서 | 소유 역할 |
|---|---|
| [appendix/A-template-library.md](appendix/A-template-library.md) | 전체 template library와 확장 report 변형 |
| [appendix/B-surface-cookbook.md](appendix/B-surface-cookbook.md) | surface별 connector note와 profile 예시 |
| [appendix/C-later-roadmap.md](appendix/C-later-roadmap.md) | 이후 자동화와 post-MVP roadmap |
| [appendix/D-migration-notes.md](appendix/D-migration-notes.md) | migration context 전용; 활성 canonical owner가 아님 |

## 재작성 제어

`docs/rewrite-control/`은 이번 재작성을 조율하는 영역이다. 결정 사항과 review checklist를 기록하지만 runtime product documentation은 아니다. Migration history와 legacy mapping은 [appendix/D-migration-notes.md](appendix/D-migration-notes.md)에 요약한다.
