# Target Document Tree

이 문서는 하네스 문서 세트 v2의 목표 파일 구조와 각 문서의 소유 책임을 정의한다.

## 0. Path Convention

```text
docs/README.md:
  harness documentation entrypoint

root README.md:
  repository landing page

All target documentation paths in this file are relative to docs/
unless explicitly stated otherwise.
```

## 1. Target Tree

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

  rewrite-control/
    REWRITE-BRIEF.md
    KERNEL-DECISIONS.md
    TARGET-DOC-TREE.md
    DOC-OWNERSHIP-MAP.md
    CONFLICT-LIST.md
    PRESERVE-MOVE-LATER.md
    CODEX-BATCHES.md
    REVIEW-CHECKLIST.md

README.md
  repository landing page, not the harness documentation entrypoint
```

## 2. Main Documents

### README.md

Owner role:

```text
문서 세트의 현관.
하네스의 한 문장 정의, 핵심 원칙, 읽기 경로, target tree를 안내한다.
```

Path:

```text
docs/README.md
```

Owns:

```text
- one-sentence definition
- reader paths
- document list
- MVP/v1/later orientation summary
```

Does not own:

```text
- state machine
- MCP schema
- projection template
- connector details
- conformance fixtures
```

### 00-introduction.md

Owner role:

```text
사용자와 구현자가 공유하는 정신 모델.
```

Owns:

```text
- 하네스가 줄이는 문제 요약
- 세 공간 모델 요약
- Task / Change Unit / Evidence / Projection 기본 개념
- advisor/direct/work 소개
- status card 예시
- source-of-truth 요약
```

Does not own:

```text
- 구현 schema
- 상태 전이표
- tool schema
- template 전문
```

### 01-project-charter.md

Owner role:

```text
프로젝트 목적, 대상, 가치, 범위, 비목표.
```

Owns:

```text
- project purpose
- target users
- core values
- current non-goals
- automation philosophy
```

Does not own:

```text
- 전략 불변식 세부
- 상태 모델
- API contract
- 운영 절차
```

### 02-strategy.md

Owner role:

```text
전략적 thesis, failure model, core invariants, policy defaults.
```

Owns:

```text
- strategic thesis
- failure model
- minimal harness kernel concept
- 7 core invariants
- policy default 목록
- human judgment model 요약
- source-of-truth principle 요약
- guarantee level 전략적 의미
- MVP boundary summary
```

Does not own:

```text
- lifecycle transition table
- gate enum 상세
- MCP request/response schema
- SQLite DDL
- template 전문
- surface별 connector addendum
```

### 03-kernel-spec.md

Owner role:

```text
하네스 운영 커널 사양.
```

Owns:

```text
- entity model
- Task / Change Unit / Run / Approval / Evidence / Eval / QA / Artifact / Reconcile Item 개념 계약
- lifecycle model
- gate model
- state compatibility matrix
- transition table
- close semantics
- waiver semantics
- prepare_write decision algorithm의 state-level logic
- close_task decision algorithm의 state-level logic
- invariant enforcement mapping
```

Does not own:

```text
- MCP wire schema
- SQLite DDL column list
- projection template
- design-quality playbook procedure
- connector capability details
- first-class capability_gate
```

### 04-runtime-architecture.md

Owner role:

```text
세 공간, runtime home, Core 흐름, authority와 projection/reconcile 아키텍처.
```

Owns:

```text
- Product Repository / Harness Server / Runtime Home canonical explanation
- runtime layers
- Core process model
- state transaction model
- artifact store architecture
- raw artifact vs state record vs Markdown report boundary summary
- projection outbox architecture
- reconcile flow
- validator runner placement
- adapter/sidecar boundary
- guarantee levels architecture
- failure and recovery flow overview
```

Does not own:

```text
- tool별 schema
- DB DDL
- full CLI commands
- conformance fixtures
- surface별 addendum
```

### 05-mcp-api-and-schemas.md

Owner role:

```text
MCP resource/tool, schema, errors, idempotency, validator result schema의 소유 문서.
```

Owns:

```text
- MCP resources
- public MCP tools
- common envelope
- tool별 request schema
- tool별 response schema
- state transition summary per tool
- events emitted per tool
- projection jobs enqueued per tool
- error code taxonomy
- idempotency and retry
- state conflict behavior
- validator result schema
- artifact ref schema
```

Does not own:

```text
- why strategy
- full state transition table
- SQLite DDL implementation details
- user-facing conversation examples
```

### 06-reference-mvp.md

Owner role:

```text
MVP 구현 순서와 참조 구현 세부.
```

Owns:

```text
- MVP-0 to MVP-5 implementation sequence
- SQLite DDL draft
- migration/versioning
- lock policy
- artifact directory layout
- baseline capture format
- projection job table
- reference surface behavior
- validator runner skeleton
- minimal CLI implementation plan
```

Does not own:

```text
- strategic rationale
- public MCP schema source of truth
- full projection template library
- surface cookbook
```

### 07-document-projection.md

Owner role:

```text
Markdown projection, managed/human-editable 영역, artifact refs, template tiers.
```

Owns:

```text
- projection principles
- authority matrix for documents
- managed block rules
- human-editable section rules
- artifact ref rules
- Markdown report projection boundary
- required MVP templates
- optional template summary
- projection freshness rules
```

Does not own:

```text
- DB schema
- full template library for all report types
- API schema
- design-quality policy applies_when rules
```

### 08-design-quality-policy-pack.md

Owner role:

```text
설계 품질 원칙을 policy contract로 정의한다.
```

Owns:

```text
- shared design policy
- domain language policy
- vertical slice policy
- TDD trace policy
- deep module/interface policy
- manual QA policy
- context hygiene policy
- waiver rules
- policy-to-validator mapping
```

Does not own:

```text
- core state machine
- tool schema
- template 전문
- user guide examples beyond short policy examples
```

### 09-agent-integration.md

Owner role:

```text
agent surface 공통 통합 계약과 capability profile.
```

Owns:

```text
- common integration structure
- capability tier
- capability profile schema
- generated file manifest concept
- push/pull context principle
- MCP unavailable fallback
- cooperative/detective/preventive/isolated guarantee expression
- reference surface contract
- connector conformance overview
```

Does not own:

```text
- surface별 detailed cookbook
- Core state machine
- kernel gate list
- MCP schema details
- operational fixtures
```

### 10-user-guide.md

Owner role:

```text
사용자가 실제로 말하고 읽는 법.
```

Owns:

```text
- quick start
- 자주 쓰는 말
- status card 읽기
- direct/work/advisor 차이
- approval/assurance/manual QA/acceptance 차이
- 멈춘 작업 이어가기
- evidence 부족, verification, QA, acceptance 상황 처리
```

Does not own:

```text
- DB schema
- MCP schema
- connector installation details beyond user-level summary
- full playbook
```

### 11-operations-and-conformance.md

Owner role:

```text
설치, 진단, 복구, export, fixture-based conformance.
```

Owns:

```text
- connect
- doctor
- serve mcp
- projection refresh
- reconcile
- recover
- export
- artifact integrity
- conformance fixture format
- core conformance fixtures
- connector conformance fixtures
- design-quality conformance fixtures
```

Does not own:

```text
- daily user workflow
- MCP schema
- long-term analytics as MVP requirement
```

### 99-authoring-guide.md

Owner role:

```text
앞으로 문서가 다시 비대해지지 않도록 하는 작성 규칙.
```

Owns:

```text
- document layer rules
- canonical ownership rules
- source-of-truth phrasing rules
- MVP/v1/later label rules
- schema ownership rules
- template ownership rules
- contradiction review checklist
```

Does not own:

```text
- runtime contract itself
- user procedure itself
- conformance fixture content itself
```

### glossary.md

Owner role:

```text
공식 용어 정의.
```

Owns:

```text
- official definitions
- enum meaning summaries
- differences between confusing terms
```

Does not own:

```text
- full policy or implementation contract
```

## 3. Appendices

### appendix/A-template-library.md

Owns:

```text
- full DEC template
- full DESIGN template
- full DOMAIN-LANGUAGE template
- full MODULE-MAP template
- full INTERFACE-CONTRACT template
- full TDD-TRACE template
- full MANUAL-QA template
- full EXPORT manifest template
- expanded report variants
```

### appendix/B-surface-cookbook.md

Owns:

```text
- Codex-specific notes
- Claude Code-specific notes
- Gemini-specific notes
- GitHub Copilot-specific notes
- Cursor-specific notes
- surface별 generated file details
- profile examples
```

### appendix/C-later-roadmap.md

Owns:

```text
- dashboard
- browser QA capture
- cross-surface verify
- native hook expansion
- sidecar advanced watcher
- parallel Change Unit orchestration
- long-term analytics
- team profile export/import
```

### appendix/D-migration-notes.md

Owns:

```text
- old file to new file migration notes
- removed/renamed sections
- compatibility guidance for existing docs
- version comparison and change rationale
```

## 4. Old-to-New Mapping

| Current File | New Destination |
|---|---|
| root `README.md` | repository landing page, outside canonical harness docs |
| `docs/README.md` | `docs/README.md` rewritten as documentation entrypoint |
| `docs/00-overview.md` | `docs/00-introduction.md` |
| `docs/01-project-charter.md` | `docs/01-project-charter.md` retained and tightened |
| `docs/02-strategy.md` | Split into `docs/02-strategy.md`, `docs/03-kernel-spec.md`, `docs/08-design-quality-policy-pack.md` |
| `docs/03-architecture.md` | `docs/04-runtime-architecture.md` |
| `docs/04-reference-implementation.md` | Split into `docs/03-kernel-spec.md`, `docs/05-mcp-api-and-schemas.md`, `docs/06-reference-mvp.md`, `docs/appendix/C-later-roadmap.md` |
| `docs/05-user-guide.md` | `docs/10-user-guide.md`, with long examples reduced or deleted |
| `docs/06-agent-integration.md` | `docs/09-agent-integration.md`, surface details to `docs/appendix/B-surface-cookbook.md` |
| `docs/07-document-and-artifact-contracts.md` | `docs/07-document-projection.md`, full templates to `docs/appendix/A-template-library.md` |
| `docs/08-operations-and-conformance.md` | `docs/11-operations-and-conformance.md`, metrics to `docs/appendix/C-later-roadmap.md` if not MVP |
| `docs/09-design-quality-playbooks.md` | `docs/08-design-quality-policy-pack.md`, detailed examples retained selectively |
| `docs/99-authoring-guide.md` | `docs/99-authoring-guide.md` updated |
| `docs/glossary.md` | `docs/glossary.md` updated |

## 5. Legacy Cleanup Requirement

Legacy docs replaced by v2 docs must not remain as canonical docs after content migration.

Legacy docs:

```text
docs/00-overview.md
docs/03-architecture.md
docs/04-reference-implementation.md
docs/05-user-guide.md
docs/06-agent-integration.md
docs/07-document-and-artifact-contracts.md
docs/08-operations-and-conformance.md
docs/09-design-quality-playbooks.md
```

Allowed treatments after migration:

```text
delete the legacy file
replace it with a short migration stub
move historical notes to docs/appendix/D-migration-notes.md
```

`docs/README.md` must not link to legacy docs except migration notes.

## 6. Ownership Constraints

```text
- State transition belongs only to 03-kernel-spec.md.
- MCP request/response schema belongs only to 05-mcp-api-and-schemas.md.
- SQLite DDL belongs only to 06-reference-mvp.md.
- Projection template tiers belong to 07-document-projection.md.
- Full template text belongs to appendix/A-template-library.md.
- Surface-specific connector details belong to appendix/B-surface-cookbook.md.
- Later automation belongs to appendix/C-later-roadmap.md.
- Historical/version comparison belongs to appendix/D-migration-notes.md.
```
