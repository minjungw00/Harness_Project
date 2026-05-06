# Preserve / Rewrite / Move-to-Appendix / Later / Delete Matrix

이 문서는 기존 하네스 문서 세트의 내용을 새 문서 구조로 어떻게 처리할지 결정한다.

## 1. Categories

```text
PRESERVE:
  핵심 의미를 유지한다. 문장은 다시 쓸 수 있다.

REWRITE:
  의도는 유지하되 구조, 층위, 표현을 바꾼다.

MOVE_TO_APPENDIX:
  본문에서 appendix로 이동한다.

LATER:
  설계 재료로 보존하되 MVP 본문 요구에서 제거한다.

DELETE:
  현재 기준 문서에는 남기지 않는다. 필요한 경우 migration note에만 둔다.
```

Every content item must use exactly one of these disposition labels. Canonical ownership is controlled by `DOC-OWNERSHIP-MAP.md`; this file records treatment during rewrite.

## 2. Preserve

다음은 새 문서 세트에서도 반드시 보존한다.

| Content Item | Disposition | Canonical Owner | Notes |
|---|---|---|---|
| 세 공간 모델 | PRESERVE | `04-runtime-architecture.md` | README, introduction, user guide는 요약만 둔다. |
| Product Repository의 projection 개념 | PRESERVE | `07-document-projection.md` | Source-of-truth와 분리한다. |
| Harness Runtime Home 개념 | PRESERVE | `04-runtime-architecture.md` | Exact `~/.harness` layout은 `06-reference-mvp.md`에서 구체화한다. |
| Chat보다 state/evidence를 우선하는 원칙 | PRESERVE | `02-strategy.md` | Core invariant 1과 연결한다. |
| Source-of-truth/projection 분리 | PRESERVE | `02-strategy.md` | Document authority matrix는 `07-document-projection.md`가 소유한다. |
| Approval / assurance / manual QA / acceptance 분리 | PRESERVE | `02-strategy.md` | Gate mechanics는 `03-kernel-spec.md`에서 구현한다. |
| Task와 Current Summary | PRESERVE | `03-kernel-spec.md` | `TASK` projection shape는 `07-document-projection.md`가 소유한다. |
| Change Unit 개념 | PRESERVE | `03-kernel-spec.md` | Vertical slice는 policy default로 이동한다. |
| Evidence Manifest 개념 | PRESERVE | `03-kernel-spec.md` | AC와 evidence 연결은 core에 유지한다. |
| Detached verification 개념 | PRESERVE | `03-kernel-spec.md` | Waiver는 `detached_verified`가 아님을 명시한다. |
| Public MCP tool 축소 | PRESERVE | `05-mcp-api-and-schemas.md` | Tool별 schema를 추가한다. |
| Capability profile | PRESERVE | `09-agent-integration.md` | 제품명 대신 profile로 판단한다. |
| Managed/human-editable 영역 | PRESERVE | `07-document-projection.md` | User Notes authority를 수정한다. |
| Reconcile flow | PRESERVE | `04-runtime-architecture.md` | Proposal과 accepted state를 분리한다. |
| Shared design policy | PRESERVE | `08-design-quality-policy-pack.md` | Work policy default로 유지한다. |
| Domain language policy | PRESERVE | `08-design-quality-policy-pack.md` | Canonical source는 `domain_terms`. |
| Module/interface review policy | PRESERVE | `08-design-quality-policy-pack.md` | Policy + validator mapping으로 유지한다. |
| TDD trace policy | PRESERVE | `08-design-quality-policy-pack.md` | Required/waived semantics를 정의한다. |
| Manual QA policy | PRESERVE | `08-design-quality-policy-pack.md` | QA gate와 policy를 분리한다. |
| Context hygiene policy | PRESERVE | `08-design-quality-policy-pack.md` | Old doc push 금지 원칙 유지. |

## 3. Rewrite

다음은 의도를 유지하되 전면 재작성한다.

| Existing Area | Disposition | Rewrite Target | Required Change |
|---|---|---|---|
| Strategy 불변식 17개 | REWRITE | `02-strategy.md` | Core invariant 7개 + policy defaults로 분리 |
| 기존 상태 축 | REWRITE | `03-kernel-spec.md` | lifecycle + gates + compatibility matrix |
| Reference Implementation | REWRITE | `03`, `05`, `06` split | state/API/DDL/implementation order 분리 |
| Source-of-truth matrix | REWRITE | `07-document-projection.md` | User Notes, Domain Language, Module/Interface authority 수정 |
| Artifact/report/projection boundary | REWRITE | `07-document-projection.md` | raw artifacts, state records, Markdown reports를 분리 |
| User Guide | REWRITE | `10-user-guide.md` | Quick start + 대화 문구 중심으로 축소 |
| Long user examples | REWRITE | `10-user-guide.md` | 대표 예시만 남기고 반복 예시는 제거 |
| Operations conformance | REWRITE | `11-operations-and-conformance.md` | fixture-based format으로 재작성 |
| Agent Integration | REWRITE | `09-agent-integration.md` | capability profile 중심으로 축소 |
| Design Quality Playbooks | REWRITE | `08-design-quality-policy-pack.md` | playbook prose → policy contract format |
| Runtime/artifact layout variants | REWRITE | `06-reference-mvp.md` | exact layout 하나로 통일 |
| Security boundary prose | REWRITE | `04-runtime-architecture.md` | guarantee level과 분리해서 표현 |
| Glossary | REWRITE | `glossary.md` | gate/waiver/guarantee/source terms 갱신 |
| Authoring Guide | REWRITE | `99-authoring-guide.md` | 새 owner map과 appendix 구조 반영 |

## 4. Move to Appendix

다음은 버리지 말고 appendix로 이동한다.

| Content | Disposition | Destination | Reason |
|---|---|---|---|
| Full DEC template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | MVP required template 아님 |
| Full DESIGN template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | 고급 설계 문서 |
| Full DOMAIN-LANGUAGE template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | optional design-quality projection |
| Full MODULE-MAP template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | optional design-quality projection |
| Full INTERFACE-CONTRACT template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | optional design-quality projection |
| Full TDD-TRACE template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | optional/full variant |
| Full MANUAL-QA template | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | optional/full variant |
| Expanded card templates | MOVE_TO_APPENDIX | `appendix/A-template-library.md` | 07에는 compact MVP card만 둔다. |
| AGENTS.md full template | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Integration doc에는 principle만 둔다. |
| Harness Skill full template | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Surface cookbook에서 관리한다. |
| Codex addendum | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Surface-specific detail |
| Claude Code addendum | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Surface-specific detail |
| Gemini addendum | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Surface-specific detail |
| GitHub Copilot addendum | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Surface-specific detail |
| Cursor addendum | MOVE_TO_APPENDIX | `appendix/B-surface-cookbook.md` | Surface-specific detail |
| Version comparison notes | MOVE_TO_APPENDIX | `appendix/D-migration-notes.md` | 기준 문서 본문에서 제거 |
| `REWRITE-MANIFEST.md` simplification history | MOVE_TO_APPENDIX | `appendix/D-migration-notes.md` | final main tree 기준 문서가 아니라 migration input |

## 5. Later Roadmap

다음은 `appendix/C-later-roadmap.md`에 보존한다. MVP 본문에는 구현 요구처럼 쓰지 않는다.

| Later Item | Disposition | Reason |
|---|---|---|
| dashboard | LATER | state/evidence/projection이 안정된 뒤 UI화 |
| browser QA artifact automatic capture | LATER | T6 capability 이후 |
| cross-surface verify | LATER | reference surface와 bundle이 안정된 뒤 |
| native hook expansion | LATER | surface별 변동성이 큼 |
| advanced sidecar file watcher | LATER | MVP는 detective 최소로 시작 |
| worktree-based fresh verify automation | LATER | v1 stable 후보 |
| parallel Change Unit orchestration | LATER | DAG, baseline, approval scope 안정화 이후 |
| long-term analytics | LATER | event model 안정화 이후 derived metrics로 계산 |
| team profile export/import | LATER | 개인/소규모 local kernel 이후 |
| artifact dashboard | LATER | artifact schema와 retention 안정화 이후 |
| advanced architecture drift validator | LATER | baseline validators 이후 |
| public interface advanced validator | LATER | v1 stable 후보 |
| domain language semantic consistency advanced check | LATER | v1/later 후보 |

## 6. Delete or De-emphasize

다음은 기준 문서 본문에서 제거하거나 강하게 축소한다.

| Content | Disposition | Reason |
|---|---|---|
| 작성 경위 중심 표현 | DELETE | 현재 상태 직접 서술 원칙 |
| 초안 대비/버전 비교 | MOVE_TO_APPENDIX | `appendix/D-migration-notes.md`로 이동 |
| 모든 connector를 동시에 완성한다는 암시 | DELETE | MVP boundary와 충돌 |
| 모든 surface에서 preventive write block 보장 암시 | REWRITE | guarantee level로 정확화 |
| 모든 작업에 무거운 TDD/설계문서 강제 암시 | REWRITE | policy applies_when/waiver로 표현 |
| Projection이 canonical state처럼 보이는 표현 | REWRITE | source-of-truth 원칙 위반 |
| event log가 별도 store처럼 보이는 표현 | REWRITE | KD-01 |
| `domain language record + reconciled doc` | REWRITE | KD-08 |
| `사용자 메모 = human-editable source-of-truth` | REWRITE | KD-07 |
| 중복 report-reading table | DELETE | 대표 사용자 절차만 `10-user-guide.md`에 남긴다. |

## 7. Legacy File Cleanup

After content migration, legacy docs replaced by v2 docs must not remain as canonical docs.

| Legacy File | Disposition | Replacement / Notes |
|---|---|---|
| `docs/legacy-v1/00-overview.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/00-introduction.md`; optional migration note in `docs/appendix/D-migration-notes.md`. |
| `docs/legacy-v1/01-project-charter.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/01-project-charter.md`. |
| `docs/legacy-v1/02-strategy.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/02-strategy.md`, `docs/03-kernel-spec.md`, and `docs/08-design-quality-policy-pack.md`. |
| `docs/legacy-v1/03-architecture.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/04-runtime-architecture.md`; optional migration note in `docs/appendix/D-migration-notes.md`. |
| `docs/legacy-v1/04-reference-implementation.md` | DELETE_AFTER_MIGRATION | Content split into `docs/03-kernel-spec.md`, `docs/05-mcp-api-and-schemas.md`, `docs/06-reference-mvp.md`, and `docs/appendix/C-later-roadmap.md`. |
| `docs/legacy-v1/05-user-guide.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/10-user-guide.md`; long examples may be deleted or summarized in migration notes. |
| `docs/legacy-v1/06-agent-integration.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/09-agent-integration.md`; surface notes move to `docs/appendix/B-surface-cookbook.md`. |
| `docs/legacy-v1/07-document-and-artifact-contracts.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/07-document-projection.md`; full templates move to `docs/appendix/A-template-library.md`. |
| `docs/legacy-v1/08-operations-and-conformance.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/11-operations-and-conformance.md`; metrics move to `docs/appendix/C-later-roadmap.md` if not MVP. |
| `docs/legacy-v1/09-design-quality-playbooks.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/08-design-quality-policy-pack.md`; examples retained only selectively. |
| `docs/legacy-v1/99-authoring-guide.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/99-authoring-guide.md`. |
| `docs/legacy-v1/glossary.md` | DELETE_AFTER_MIGRATION | Content rewritten into `docs/glossary.md`. |
| `docs/legacy-v1/REWRITE-MANIFEST.md` | MOVE_TO_MIGRATION_NOTES | Historical rewrite summary moves to `docs/appendix/D-migration-notes.md`. |

If repository history or user preference requires a visible file at a legacy path, replace the legacy file with a short migration stub. The stub is not canonical content and must point to the v2 owner doc or `docs/appendix/D-migration-notes.md`.

## 8. Source Document Treatment

### `REWRITE-MANIFEST.md`

Treatment: `MOVE_TO_APPENDIX`

Destination: `appendix/D-migration-notes.md`

Not part of final main tree. Use as control/migration input while writing migration notes.

Use it to confirm original simplification goals:

```text
- 세 공간 실행 모델
- source-of-truth/projection
- public MCP tool 축소
- SQLite runtime 중심
- MVP/later 분리
- four judgment separation
- detached verification/design-quality principles
```

### `README.md`

Treatment: `REWRITE`

Path note: `docs/README.md` is the harness documentation entrypoint. Root `README.md` is the repository landing page.

Keep:

```text
- 한 문장 정의
- 세 공간 요약
- 문서 목록
- 핵심 원칙
```

Change:

```text
- 새 target doc tree 반영
- state.sqlite.task_events 표현 반영
- reader paths 갱신
```

### `00-overview.md`

Treatment: `REWRITE`

Target: `00-introduction.md`

Keep:

```text
- 하네스가 줄이는 문제
- 세 공간
- 기본 흐름
- 핵심 개념
- status card 예시
```

REWRITE targets:

```text
- source-of-truth matrix 상세 → 07
- 구현 흐름 상세 → 03/04
- template/schema 언급 → owner docs
```

### `01-project-charter.md`

Treatment: `PRESERVE`

Keep most content. Add:

```text
- MVP는 cooperative/detective local kernel임
- 모든 connector 동시 완성이 아님
- 완전 자동 차단 시스템이 아님
```

### `02-strategy.md`

Treatment: `REWRITE`

Split:

```text
- why/failure/core invariants → 02-strategy
- state axes/gates/transitions → 03-kernel-spec
- design quality details → 08-design-quality-policy-pack
```

### `03-architecture.md`

Treatment: `REWRITE`

Target: `04-runtime-architecture.md`

Keep:

```text
- 세 공간
- runtime layers
- transaction flow
- projection/reconcile
- detached verification flow
- guarantee level
```

Change:

```text
- event log location 명확화
- guarantee level 앞쪽 배치
- sidecar/native hook later 구분
```

### `04-reference-implementation.md`

Treatment: `REWRITE`

Split into:

```text
03-kernel-spec.md
05-mcp-api-and-schemas.md
06-reference-mvp.md
appendix/C-later-roadmap.md
```

### `05-user-guide.md`

Treatment: `REWRITE`

Target: `10-user-guide.md`

Keep:

```text
- 자주 쓰는 말
- status card reading
- four judgment separation
- resume guidance
```

REWRITE/remove:

```text
- 긴 work examples
- detailed report reading tables
- verbose operational habits
```

### `06-agent-integration.md`

Treatment: `REWRITE`

Keep in main:

```text
- common integration structure
- capability tiers
- capability profile
- fallback principles
- connector conformance overview
```

MOVE_TO_APPENDIX:

```text
- Codex/Claude/Gemini/Copilot/Cursor addenda
```

### `07-document-and-artifact-contracts.md`

Treatment: `REWRITE`

Keep in main:

```text
- projection principles
- authority matrix corrected
- managed/human-editable rules
- artifact refs
- required MVP templates
```

MOVE_TO_APPENDIX:

```text
- full templates
```

### `08-operations-and-conformance.md`

Treatment: `REWRITE`

Keep:

```text
- setup/connect
- doctor
- projection refresh
- reconcile
- recover
- export
- conformance
```

Change:

```text
- conformance → fixture-based
- long-term metrics → later roadmap
```

### `09-design-quality-playbooks.md`

Treatment: `REWRITE`

Keep concepts:

```text
- Shared Design
- Domain Language
- Vertical Slice
- TDD
- Deep Module
- Manual QA
- Context Hygiene
```

Change structure:

```yaml
policy:
  applies_when:
  default_requirement:
  allowed_waiver:
  required_record:
  validator:
  close_impact:
```

### `99-authoring-guide.md`

Treatment: `REWRITE`

Update:

```text
- new doc tree
- new ownership map
- core invariant vs policy default rule
- schema/template/appendix ownership
```

### `glossary.md`

Treatment: `REWRITE`

Add:

```text
Gate
Scope Gate
Approval Gate
Evidence Gate
Verification Gate
QA Gate
Acceptance Gate
Close Reason
Waiver
Guarantee Level
Cooperative Guarantee
Detective Guarantee
Preventive Guarantee
Reference Surface
Raw Artifact
State Record
Markdown Report
Report Projection
```

Modify:

```text
Source-of-truth
Projection
Artifact
Raw Artifact
State Record
Markdown Report
Report Projection
Evidence Manifest
Domain Language
Human-editable 영역
Reconcile
Detached Verification
Assurance
```
