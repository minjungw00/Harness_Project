# Harness Documentation Rewrite Brief

## 1. Mission

하네스 문서 세트를 **구현 가능한 로컬 운영 커널 사양**으로 재구성한다.

이번 작업은 기존 문장의 교정이나 단순 이동이 아니다. 기존 문서는 설계 재료로 사용하되, 문서 구조와 canonical ownership을 새로 고정한다. 필요한 경우 본문은 처음부터 다시 쓴다.

목표 산출물은 다음 조합이다.

```text
small implementable harness kernel
+ closed state/gate model
+ explicit MCP API and schemas
+ reference MVP implementation plan
+ design-quality policy pack
+ lightweight user and integration docs
+ fixture-based conformance
```

## 2. Problem Being Fixed

현재 문서 세트는 하네스의 핵심 방향을 잘 담고 있다. 세 공간 실행 모델, source-of-truth/projection 분리, public MCP tool 표면 축소, SQLite runtime 중심화, MVP/later 분리, approval/assurance/manual QA/acceptance 분리는 보존해야 한다.

하지만 현재 문서들은 다음 문제를 남긴다.

```text
- 전략, 상태 모델, API/schema, 구현 계약, 템플릿, connector, 운영 기준이 한 문서 안에 과밀하게 섞인다.
- 여러 좋은 설계 원칙이 모두 core invariant처럼 보인다.
- MVP에서 실제로 구현해야 하는 kernel과 later automation의 경계가 흐리다.
- 상태 전이, gate, waiver, close semantics가 구현자가 임의 해석할 여지를 남긴다.
- event log, User Notes, Domain Language authority 표현이 문서 간 흔들린다.
- MCP tool 이름은 정리되어 있으나 tool별 request/response schema가 충분히 닫혀 있지 않다.
- conformance가 fixture 기반 구현 테스트로 충분히 구체화되어 있지 않다.
```

## 3. Primary Goal

새 문서 세트는 다음 질문에 정확히 답해야 한다.

```text
사용자:
  하네스를 어떻게 쓰는가?

구현자:
  이 문서만 보고 MVP를 만들 수 있는가?

connector 작성자:
  특정 agent surface를 어떻게 붙이는가?

운영자:
  설치, 진단, 복구, conformance를 어떻게 검증하는가?

설계 책임자:
  AI가 좋은 설계 경계 안에서 일하는지 어떻게 통제하는가?
```

## 4. Non-negotiable Principles

새 문서 세트는 다음 원칙을 약화하지 않는다.

```text
1. Chat is not state.
2. Product write requires an active scoped Change Unit.
3. Sensitive change requires explicit approval.
4. Completion requires evidence coverage where evidence is required.
5. Work cannot self-certify detached verification.
6. Required QA and acceptance are separate gates.
7. Projection cannot override canonical state.
```

## 5. Core Rewrite Thesis

현재 문서의 좋은 원칙을 모두 버리지 않는다. 대신 층위를 분리한다.

```text
Kernel:
  Task, Change Unit, lifecycle, gates, state transition, evidence, verification, close semantics

Policy Pack:
  shared design, domain language, vertical slice, TDD, module/interface review, manual QA, context hygiene

Projection:
  사람이 읽는 Markdown, managed block, human-editable area, artifact refs

Integration:
  agent surface capability profile, connector contract, fallback semantics

Operations:
  doctor, recover, reconcile, export, fixture-based conformance
```

## 6. MVP Boundary

MVP는 agent surface 통합 프로젝트가 아니라 **core invariant 검증 프로젝트**다.

MVP는 다음을 목표로 한다.

```text
- 단일 로컬 프로젝트 등록
- 하나의 reference agent surface 연결
- state.sqlite 기반 상태 저장
- state.sqlite.task_events append-only event table
- artifact registry
- public MCP tools
- prepare_write gate
- approval/evidence/verification/manual QA/acceptance gate 최소 구현
- TASK, APR, RUN-SUMMARY, EVIDENCE-MANIFEST, EVAL, DIRECT-RESULT projection
- detached verification bundle 또는 manual evaluator instruction bundle
- doctor, recover, reconcile, export, conformance smoke
```

MVP는 다음을 목표로 하지 않는다.

```text
- 모든 agent surface connector 동시 완성
- dashboard
- browser QA 자동 캡처
- cross-surface orchestration
- native hook coverage for every surface
- 완전 자동 병렬 실행
- 장기 analytics
- team workflow system
```

## 7. Fixed Architectural Direction

다음 방향은 고정한다.

```text
Product Repository:
  제품 코드와 사람이 읽는 projection이 있는 저장소

Harness Server / Installation:
  MCP server와 Core를 실행하는 설치물

Harness Runtime Home:
  registry.sqlite, state.sqlite, artifacts가 있는 로컬 운영 홈
```

`Product Repository` 문서는 사람이 읽는 projection이다. 운영 상태의 canonical source는 Runtime Home의 `state.sqlite`다. Raw evidence의 canonical source는 artifact store다.

## 8. Fixed Rewrite Decisions

상세 결정은 `docs/rewrite-control/KERNEL-DECISIONS.md`가 소유한다. 요약은 다음이다.

```text
- event log는 MVP에서 state.sqlite.task_events다.
- 상태 모델은 lifecycle + gates로 재구성한다.
- scope gate와 approval gate를 분리한다.
- verification waiver는 detached_verified가 아니다.
- User Notes authority는 input surface / reconcile_items / accepted state로 분리한다.
- Domain Language canonical source는 state.sqlite.domain_terms다.
- design-quality는 kernel invariant가 아니라 policy pack이다.
- public MCP tools는 유지하되 schema를 엄격화한다.
- template은 required / optional / appendix로 분리한다.
- conformance는 fixture 기반이다.
```

## 9. New Document Layers

새 문서 세트는 다음 층위를 따른다.

```text
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

## 10. Editing Rules

문서 작성자는 다음 규칙을 따른다.

```text
- 기존 문장을 보존하기 위해 보존하지 않는다.
- 새로운 아키텍처 결정을 임의로 만들지 않는다.
- 결정이 필요하면 TODO_DECISION으로 남긴다.
- MVP 본문에 later automation을 섞지 않는다.
- surface별 세부는 core 문서가 아니라 appendix로 이동한다.
- 같은 개념의 canonical 설명은 한 문서에만 둔다.
- 다른 문서는 한 문장 요약과 참조만 둔다.
- schema는 05-mcp-api-and-schemas.md 또는 06-reference-mvp.md의 지정된 영역에만 둔다.
- state transition은 03-kernel-spec.md만 소유한다.
- full template 전문은 appendix/A-template-library.md로 보낸다.
```

## 11. Output Quality Bar

재작성된 문서 세트는 다음을 만족해야 한다.

```text
- 처음 읽는 사용자는 하네스를 대화 중심 도구로 이해한다.
- 구현자는 state.sqlite, MCP server, projection, validator skeleton을 만들 수 있다.
- connector 작성자는 capability profile 기준으로 surface를 연결할 수 있다.
- 운영자는 doctor/recover/export/conformance를 fixture로 검증할 수 있다.
- 설계 책임자는 core invariant와 design-quality policy default를 구분할 수 있다.
```

## 12. Definition of Done

```text
[ ] core invariant가 승인된 7개로 유지된다.
[ ] policy default와 core invariant가 분리되어 있다.
[ ] event log 위치가 state.sqlite.task_events로 명확하다.
[ ] lifecycle + gates 상태 모델이 있다.
[ ] 상태 전이표와 불가능 조합이 있다.
[ ] close_task 알고리즘이 있다.
[ ] prepare_write 알고리즘이 있다.
[ ] approval gate와 scope gate가 분리되어 있다.
[ ] detached verification waiver가 detached_verified로 표시되지 않는다.
[ ] User Notes authority가 3단계로 정리되어 있다.
[ ] Domain Language canonical source가 domain_terms table로 정리되어 있다.
[ ] guarantee level이 cooperative/detective/preventive/isolated로 표시되어 있다.
[ ] MCP tool별 request/response schema가 있다.
[ ] error code taxonomy가 있다.
[ ] validator result schema가 있다.
[ ] SQLite DDL 초안이 있다.
[ ] artifact schema와 redaction rule이 있다.
[ ] projection job schema가 있다.
[ ] required/optional/appendix template가 분리되어 있다.
[ ] connector 문서는 capability profile 중심이다.
[ ] surface별 세부는 docs/appendix에 있다.
[ ] User Guide는 짧고 대화 중심이다.
[ ] Operations conformance는 fixture 기반이다.
[ ] `docs/README.md`의 읽기 경로가 새 문서 구조와 일치한다.
[ ] Glossary의 모든 공식 용어가 새 모델과 일치한다.
[ ] later 기능이 MVP 본문을 흐리지 않는다.
```
