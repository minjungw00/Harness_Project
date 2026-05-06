# 하네스 문서 세트

하네스는 AI와 함께 하는 개발 작업을 사용자가 읽고, 제한하고, 검증하고, 다시 이어받을 수 있게 만드는 로컬 운영 커널이다.

이 문서 세트는 v2 target tree로 재구성되는 중이다. Batch A에서는 새 구조의 skeleton을 만들었고, v1 원본 문서는 [legacy-v1/](legacy-v1/)에 보관한다. 마이그레이션 상태는 [appendix/D-migration-notes.md](appendix/D-migration-notes.md)를 기준으로 확인한다.

## 읽기 경로

### 일반 사용자

```text
00-introduction.md
→ 10-user-guide.md
```

### 구현자

```text
00-introduction.md
→ 02-strategy.md
→ 03-kernel-spec.md
→ 04-runtime-architecture.md
→ 05-mcp-api-and-schemas.md
→ 06-reference-mvp.md
→ 11-operations-and-conformance.md
```

### Connector 작성자

```text
00-introduction.md
→ 09-agent-integration.md
→ appendix/B-surface-cookbook.md
→ 11-operations-and-conformance.md
```

### Projection 관리자

```text
00-introduction.md
→ 07-document-projection.md
→ appendix/A-template-library.md
→ 11-operations-and-conformance.md
```

### 문서 작성자

```text
99-authoring-guide.md
→ rewrite-control/DOC-OWNERSHIP-MAP.md
→ appendix/D-migration-notes.md
```

## MVP / v1 / Later 방향

MVP는 많은 agent surface를 동시에 지원하는 프로젝트가 아니라 작은 로컬 운영 커널과 core invariant를 검증하는 프로젝트다. v1은 reference surface 이후의 connector와 watcher를 넓히고, later 항목은 dashboard, browser QA capture, cross-surface verification, native hook expansion 같은 확장으로 분리한다.

Later 항목은 구현 요구처럼 main docs에 섞지 않고 [appendix/C-later-roadmap.md](appendix/C-later-roadmap.md)에 둔다.

## v2 Target Tree

| 문서 | 소유 역할 |
|---|---|
| [00-introduction.md](00-introduction.md) | 사용자와 구현자가 공유하는 정신 모델 |
| [01-project-charter.md](01-project-charter.md) | 프로젝트 목적, 대상, 가치, 범위, 비목표 |
| [02-strategy.md](02-strategy.md) | 전략적 thesis, failure model, core invariants, policy defaults |
| [03-kernel-spec.md](03-kernel-spec.md) | 하네스 운영 커널 사양 |
| [04-runtime-architecture.md](04-runtime-architecture.md) | 세 공간, runtime home, Core 흐름, authority와 projection/reconcile 아키텍처 |
| [05-mcp-api-and-schemas.md](05-mcp-api-and-schemas.md) | MCP resource/tool, schema, errors, idempotency, validator result schema |
| [06-reference-mvp.md](06-reference-mvp.md) | MVP 구현 순서와 참조 구현 세부 |
| [07-document-projection.md](07-document-projection.md) | Markdown projection, managed/human-editable 영역, artifact refs, template tiers |
| [08-design-quality-policy-pack.md](08-design-quality-policy-pack.md) | 설계 품질 원칙을 policy contract로 정의 |
| [09-agent-integration.md](09-agent-integration.md) | agent surface 공통 통합 계약과 capability profile |
| [10-user-guide.md](10-user-guide.md) | 사용자가 실제로 말하고 읽는 법 |
| [11-operations-and-conformance.md](11-operations-and-conformance.md) | 설치, 진단, 복구, export, fixture-based conformance |
| [99-authoring-guide.md](99-authoring-guide.md) | 문서 작성과 개정 기준 |
| [glossary.md](glossary.md) | 공식 용어 정의 |

## Appendices

| 문서 | 소유 역할 |
|---|---|
| [appendix/A-template-library.md](appendix/A-template-library.md) | full template library and expanded report variants |
| [appendix/B-surface-cookbook.md](appendix/B-surface-cookbook.md) | surface-specific connector notes and profile examples |
| [appendix/C-later-roadmap.md](appendix/C-later-roadmap.md) | later automation and post-MVP roadmap |
| [appendix/D-migration-notes.md](appendix/D-migration-notes.md) | old-to-new migration notes and compatibility guidance |

## Rewrite Control

The rewrite-control directory is the coordination layer for this documentation rewrite. Its ownership decisions and batch instructions are not runtime product documentation.

| 문서 | 역할 |
|---|---|
| [rewrite-control/REWRITE-BRIEF.md](rewrite-control/REWRITE-BRIEF.md) | rewrite mission and quality bar |
| [rewrite-control/KERNEL-DECISIONS.md](rewrite-control/KERNEL-DECISIONS.md) | fixed kernel decisions for the rewrite |
| [rewrite-control/TARGET-DOC-TREE.md](rewrite-control/TARGET-DOC-TREE.md) | target v2 tree and ownership summary |
| [rewrite-control/DOC-OWNERSHIP-MAP.md](rewrite-control/DOC-OWNERSHIP-MAP.md) | canonical ownership map |
| [rewrite-control/CONFLICT-LIST.md](rewrite-control/CONFLICT-LIST.md) | confirmed conflicts and rewrite risks |
| [rewrite-control/PRESERVE-MOVE-LATER.md](rewrite-control/PRESERVE-MOVE-LATER.md) | source content treatment and migration disposition |
| [rewrite-control/CODEX-BATCHES.md](rewrite-control/CODEX-BATCHES.md) | batch instructions |
| [rewrite-control/REVIEW-CHECKLIST.md](rewrite-control/REVIEW-CHECKLIST.md) | rewrite review checklist |
