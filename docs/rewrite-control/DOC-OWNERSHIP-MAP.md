# Document Ownership Map

이 문서는 하네스 문서 세트 v2의 canonical ownership을 고정한다. 같은 개념의 기준 설명은 하나의 문서만 소유한다. 다른 문서는 한 문장 요약과 링크만 둔다.

## 1. Ownership Rules

```text
1. 개념 하나에는 owner 문서 하나만 둔다.
2. Owner가 아닌 문서는 짧은 요약만 둔다.
3. Schema는 지정된 schema 문서 밖에 반복하지 않는다.
4. State transition은 kernel spec 밖에 반복하지 않는다.
5. Full template은 appendix 밖에 반복하지 않는다.
6. Surface별 상세는 core integration 문서 밖에 반복하지 않는다.
7. Later 기능은 MVP 본문에서 구현 요구처럼 쓰지 않는다.
```

Owner names in this file refer to the v2 target tree in `TARGET-DOC-TREE.md`. Current source files may have different names until the rewrite is executed.

Path convention:

```text
docs/README.md:
  harness documentation entrypoint

root README.md:
  repository landing page

All target documentation paths are interpreted under docs/
unless explicitly stated otherwise.
```

## 2. Canonical Ownership Table

| Concept | Canonical Owner | Allowed Summary Elsewhere | Must Not Be Owned By |
|---|---|---|---|
| Harness one-sentence definition | `README.md` | all docs may quote once | strategy/kernel/API docs |
| Repository landing page | root `README.md` | `docs/README.md` may link to it | harness canonical docs |
| Reader paths | `README.md` | user guide may repeat quick path | strategy/kernel/API docs |
| Document list and target tree summary | `README.md` | authoring guide may reference | implementation docs |
| Project purpose | `01-project-charter.md` | README, introduction | implementation docs |
| Target users and non-goals | `01-project-charter.md` | README | strategy/API docs |
| Automation philosophy | `01-project-charter.md` | strategy summary | reference MVP/API |
| MVP/v1/later boundary summary | `02-strategy.md` | README orientation | reference MVP only implements MVP |
| Strategic thesis | `02-strategy.md` | README, introduction | kernel/API/user guide |
| Failure model | `02-strategy.md` | introduction | operations/API |
| Core invariants | `02-strategy.md` | authoring guide checklist | kernel as implementation mapping only |
| Policy defaults list | `02-strategy.md` | policy pack expands | kernel as gates only |
| Human judgment model | `02-strategy.md` | user guide examples | API schemas |
| Advisor/direct/work concept | `03-kernel-spec.md` | introduction/user guide | strategy as rationale only |
| Mode enum | `03-kernel-spec.md` | status card examples | API docs except schema reference |
| Task entity model | `03-kernel-spec.md` | introduction/user guide | projection templates |
| Change Unit entity model | `03-kernel-spec.md` | user guide examples | design playbook as policy only |
| Run entity model | `03-kernel-spec.md` | API doc references | projection docs |
| Approval entity model | `03-kernel-spec.md` | user guide/API doc | projection APR template |
| Evidence Manifest entity model | `03-kernel-spec.md` | projection/API doc | user guide |
| Eval entity model | `03-kernel-spec.md` | projection/API doc | user guide |
| EVAL verdict to assurance upgrade rule | `03-kernel-spec.md` | API/projection docs summarize | user guide |
| Manual QA entity model | `03-kernel-spec.md` | policy pack/user guide | projection template |
| QA gate vs manual QA result rule | `03-kernel-spec.md` | projection/user guide summarize | policy docs |
| Artifact entity semantics | `03-kernel-spec.md` | architecture/API | projection docs |
| State record entity model | `03-kernel-spec.md` | API/projection docs | artifact docs |
| Reconcile Item entity model | `03-kernel-spec.md` | projection/operations | user guide as procedure only |
| Human-editable User Notes proposal flow | `07-document-projection.md` | user guide/operations | kernel state |
| Lifecycle phase enum | `03-kernel-spec.md` | status card examples | strategy/user guide |
| Result enum | `03-kernel-spec.md` | user guide summaries | API docs except schema reference |
| Close reason enum | `03-kernel-spec.md` | user guide summaries | strategy/API |
| Gate model | `03-kernel-spec.md` | strategy summary/API transitions | projection docs |
| Scope gate | `03-kernel-spec.md` | user guide summary | approval docs |
| Approval gate | `03-kernel-spec.md` | user guide summary | projection APR template |
| Design gate | `03-kernel-spec.md` | policy pack explains requirements | strategy |
| Evidence gate | `03-kernel-spec.md` | projection/API references | user guide |
| Verification gate | `03-kernel-spec.md` | user guide summary | strategy |
| QA gate | `03-kernel-spec.md` | policy pack/user guide | projection template |
| Acceptance gate | `03-kernel-spec.md` | user guide summary | projection template |
| Capability gate exclusion | `03-kernel-spec.md` | integration/architecture explain capability handling | API docs |
| Assurance level | `03-kernel-spec.md` | glossary and user guide explain meaning | verification policy |
| Verification independence concept | `03-kernel-spec.md` | architecture flow, API schema | user guide |
| State compatibility matrix | `03-kernel-spec.md` | none except tests | API/reference MVP |
| State transition table | `03-kernel-spec.md` | operations fixture expected states | strategy/reference MVP |
| Waiver semantics | `03-kernel-spec.md` | user guide summary | strategy/API |
| `close_task` decision algorithm | `03-kernel-spec.md` | API doc tool summary | operations/user guide |
| `prepare_write` state logic | `03-kernel-spec.md` | API doc wire schema | architecture only flow summary |
| Three spaces model | `04-runtime-architecture.md` | README/introduction/user guide | reference MVP only layout detail |
| Product Repository role | `04-runtime-architecture.md` | README/introduction | projection docs as file placement only |
| Harness Server / Installation role | `04-runtime-architecture.md` | README/introduction | user guide |
| Harness Runtime Home role | `04-runtime-architecture.md` | README/introduction | reference MVP exact layout |
| Runtime layers | `04-runtime-architecture.md` | README summary | strategy |
| Core process model | `04-runtime-architecture.md` | reference MVP implementation details | agent integration |
| State transaction model | `04-runtime-architecture.md` | reference MVP DDL uses | API doc |
| Artifact store architecture | `04-runtime-architecture.md` | projection/API details | user guide |
| Raw artifact boundary | `04-runtime-architecture.md` | kernel/API/projection summaries | report projection docs |
| Projection outbox architecture | `04-runtime-architecture.md` | reference MVP DDL | projection doc rules |
| Reconcile flow architecture | `04-runtime-architecture.md` | projection/operations procedures | user guide as usage only |
| Guarantee levels | `04-runtime-architecture.md` | strategy/user guide/integration summary | operations metrics |
| Security boundary architecture | `04-runtime-architecture.md` | API/reference MVP/integration implement aspects | user guide |
| MCP resources | `05-mcp-api-and-schemas.md` | integration mentions use | user guide/reference MVP |
| MCP public tools | `05-mcp-api-and-schemas.md` | integration lists names | strategy/user guide |
| Common tool envelope | `05-mcp-api-and-schemas.md` | reference MVP implementation follows | operations |
| Tool request schemas | `05-mcp-api-and-schemas.md` | none | all other docs |
| Tool response schemas | `05-mcp-api-and-schemas.md` | none | all other docs |
| Error code taxonomy | `05-mcp-api-and-schemas.md` | operations fixtures use codes | kernel/reference MVP |
| Validator result schema | `05-mcp-api-and-schemas.md` | reference MVP runner implements | design policy |
| Artifact ref payload shape | `05-mcp-api-and-schemas.md` | projection doc renders refs | architecture |
| Sensitive category enum | `05-mcp-api-and-schemas.md` | strategy/user guide summarize | projection APR prose |
| Approval request/decision wire contract | `05-mcp-api-and-schemas.md` | kernel entity, APR projection | user guide |
| `surface_capability_check` validator schema | `05-mcp-api-and-schemas.md` | integration/architecture mention meaning | kernel gates |
| `prepare_write` blocked reasons | `05-mcp-api-and-schemas.md` | kernel state logic, user guide summary | projection docs |
| SQLite DDL | `06-reference-mvp.md` | API doc refers to records | kernel/architecture |
| `project.yaml` schema | `06-reference-mvp.md` | architecture/user guide mention static config | API docs |
| Migration/versioning | `06-reference-mvp.md` | operations mentions doctor | API doc |
| Lock policy | `06-reference-mvp.md` | API conflict behavior summary | architecture |
| Artifact table DDL | `06-reference-mvp.md` | kernel/API concept summary | architecture |
| Artifact directory layout | `06-reference-mvp.md` | architecture high-level layout | projection docs |
| Baseline capture format | `06-reference-mvp.md` | API request fields | operations recovery |
| Reference MVP implementation sequence | `06-reference-mvp.md` | README/strategy summary | architecture |
| MVP validator runner skeleton | `06-reference-mvp.md` | API validator result schema | policy pack |
| Minimal CLI implementation plan | `06-reference-mvp.md` | operations command semantics | user guide |
| Reference surface behavior | `06-reference-mvp.md` | integration summarizes | surface cookbook details |
| Projection principles | `07-document-projection.md` | README/introduction | reference MVP |
| Document authority matrix | `07-document-projection.md` | introduction summary | source DDL |
| Markdown report projection boundary | `07-document-projection.md` | API/operations reference | artifact store docs |
| Markdown rendering of artifact refs | `07-document-projection.md` | user guide examples | artifact ref schema |
| Managed block rules | `07-document-projection.md` | operations reconcile | user guide short mention |
| Human-editable section rules | `07-document-projection.md` | user guide/operations | strategy |
| Projection freshness rules | `07-document-projection.md` | operations refresh | kernel state |
| Template tiers | `07-document-projection.md` | appendix contains full text | user guide |
| Required MVP templates | `07-document-projection.md` | appendix has full variants | API docs |
| RUN-SUMMARY projection | `07-document-projection.md` | appendix full variant | raw artifact docs |
| EVAL projection | `07-document-projection.md` | appendix full variant | kernel assurance rule |
| TDD-TRACE projection | `07-document-projection.md` | appendix full variant | raw artifact docs |
| MANUAL-QA projection | `07-document-projection.md` | appendix full variant | QA gate |
| EVIDENCE-MANIFEST projection | `07-document-projection.md` | appendix full variant | raw artifact docs |
| DIRECT-RESULT projection | `07-document-projection.md` | appendix full variant | raw artifact docs |
| Full template library | `appendix/A-template-library.md` | projection doc references | main docs |
| Shared design entity model | `03-kernel-spec.md` | policy/projection docs | DDL/policy docs |
| Shared design policy | `08-design-quality-policy-pack.md` | strategy lists default | kernel as design_gate only |
| Shared design projection | `07-document-projection.md` | appendix full variants | policy docs |
| Domain term entity model | `03-kernel-spec.md` | policy/projection docs | DDL/policy docs |
| Domain language policy | `08-design-quality-policy-pack.md` | projection/API records | strategy summary |
| DOMAIN-LANGUAGE projection | `07-document-projection.md` | appendix full variant | policy docs |
| Module map item entity model | `03-kernel-spec.md` | policy/projection docs | DDL/policy docs |
| Module map policy | `08-design-quality-policy-pack.md` | strategy summary | architecture |
| MODULE-MAP projection | `07-document-projection.md` | appendix full variant | policy docs |
| Interface contract entity model | `03-kernel-spec.md` | policy/projection docs | DDL/policy docs |
| Interface contract policy | `08-design-quality-policy-pack.md` | strategy summary | architecture |
| INTERFACE-CONTRACT projection | `07-document-projection.md` | appendix full variant | policy docs |
| TDD trace entity model | `03-kernel-spec.md` | policy/projection docs | DDL/policy docs |
| Vertical slice policy | `08-design-quality-policy-pack.md` | user guide examples | strategy as invariant |
| TDD policy | `08-design-quality-policy-pack.md` | user guide examples | kernel only gate impact |
| Deep module/interface policy | `08-design-quality-policy-pack.md` | strategy summary | architecture |
| Manual QA projection wording | `07-document-projection.md` | user guide card examples | kernel QA gate |
| Manual QA policy | `08-design-quality-policy-pack.md` | user guide | kernel only gate state |
| Context hygiene policy | `08-design-quality-policy-pack.md` | user guide/authoring guide | API docs |
| Policy-to-validator mapping | `08-design-quality-policy-pack.md` | reference MVP implements hooks | API validator schema |
| Agent capability tier | `09-agent-integration.md` | architecture guarantee summary | user guide |
| Capability profile schema | `09-agent-integration.md` | reference MVP stores profile | surface cookbook examples |
| Capability profile ownership | `09-agent-integration.md` | architecture/API summary | kernel gates |
| Connector generated manifest | `09-agent-integration.md` | operations drift check | projection docs |
| Push/pull context principle | `09-agent-integration.md` | policy pack context hygiene | user guide |
| MCP unavailable fallback semantics | `09-agent-integration.md` | user guide short warning | operations |
| Connector conformance overview | `09-agent-integration.md` | operations fixtures own exact tests | surface cookbook |
| Surface-specific details | `appendix/B-surface-cookbook.md` | integration references | core integration doc |
| User quick start | `10-user-guide.md` | README path | operations |
| Status card reading | `10-user-guide.md` | projection card template | API docs |
| User phrases | `10-user-guide.md` | README short examples | strategy |
| Approval/assurance/QA/acceptance user explanation | `10-user-guide.md` | strategy/kernel summaries | API/projection |
| Resume user procedure | `10-user-guide.md` | operations recover command | architecture |
| Evidence-shortage user procedure | `10-user-guide.md` | operations fixtures | API schema |
| Setup/connect command semantics | `11-operations-and-conformance.md` | user guide mentions minimal | integration wizard summary |
| Doctor/recover/export | `11-operations-and-conformance.md` | user guide short mention | reference MVP implementation details only |
| Projection refresh command semantics | `11-operations-and-conformance.md` | projection doc rules | user guide |
| Reconcile command/procedure | `11-operations-and-conformance.md` | projection architecture | user guide |
| Artifact integrity check | `11-operations-and-conformance.md` | reference MVP implements | user guide |
| Conformance fixture format | `11-operations-and-conformance.md` | review checklist | strategy |
| Core conformance fixtures | `11-operations-and-conformance.md` | kernel references expected states | strategy |
| Connector conformance fixtures | `11-operations-and-conformance.md` | integration overview | surface cookbook |
| Design-quality conformance fixtures | `11-operations-and-conformance.md` | policy pack validator mapping | strategy |
| Operational metrics | `appendix/C-later-roadmap.md` unless MVP-critical | operations may list as later | strategy/reference MVP |
| Later roadmap | `appendix/C-later-roadmap.md` | README orientation only | MVP docs |
| Migration notes | `appendix/D-migration-notes.md` | authoring guide may reference | main docs |
| Authoring rules | `99-authoring-guide.md` | README references | all docs |
| Contradiction review checklist | `99-authoring-guide.md` | rewrite-control tracks current conflicts | runtime docs |
| Official term definitions | `glossary.md` | all docs use terms | individual docs should not redefine extensively |

## 3. Source-of-Truth Ownership

This table assigns exactly one concept owner. Storage DDL owner is listed separately and does not take conceptual ownership.

| Fact Type | Canonical Source in System | Concept Owner | Storage DDL Owner |
|---|---|---|---|
| Project registration | `registry.sqlite.projects` | `04-runtime-architecture.md` | `06-reference-mvp.md` |
| Surface connection | `registry.sqlite.project_surfaces` | `09-agent-integration.md` | `06-reference-mvp.md` |
| Static project config | `project.yaml` | `06-reference-mvp.md` | `06-reference-mvp.md` |
| Current task state | `state.sqlite.tasks` and related current tables | `03-kernel-spec.md` | `06-reference-mvp.md` |
| State event history | `state.sqlite.task_events` | `03-kernel-spec.md` | `06-reference-mvp.md` |
| Raw evidence | artifact store + `artifacts` table | `04-runtime-architecture.md` | `06-reference-mvp.md` |
| Artifact entity semantics | artifact records and refs | `03-kernel-spec.md` | `06-reference-mvp.md` |
| Artifact reference shape | artifact ref payloads | `05-mcp-api-and-schemas.md` | n/a |
| Markdown reports | projections generated from state records + artifact refs | `07-document-projection.md` | n/a |
| Run summary | `state.sqlite.runs` + artifact refs | `03-kernel-spec.md` | `06-reference-mvp.md` |
| Direct result | `state.sqlite.runs` + artifact refs | `03-kernel-spec.md` | `06-reference-mvp.md` |
| Projection freshness | `projection_jobs` | `07-document-projection.md` | `06-reference-mvp.md` |
| User notes proposal flow | human-editable section → `reconcile_items` → accepted state | `07-document-projection.md` | `06-reference-mvp.md` |
| Reconcile item entity | `reconcile_items` | `03-kernel-spec.md` | `06-reference-mvp.md` |
| Domain terms | `domain_terms` | `03-kernel-spec.md` | `06-reference-mvp.md` |
| Module map items | `module_map_items` | `03-kernel-spec.md` | `06-reference-mvp.md` |
| Interface contracts | `interface_contracts` | `03-kernel-spec.md` | `06-reference-mvp.md` |
| TDD trace | `tdd_traces` + artifact refs | `03-kernel-spec.md` | `06-reference-mvp.md` |
| Evidence manifest | `evidence_manifests` + artifact refs | `03-kernel-spec.md` | `06-reference-mvp.md` |
| Approval | `approvals` + `task_events` | `03-kernel-spec.md` | `06-reference-mvp.md` |
| Manual QA | `manual_qa_records` + artifact refs | `03-kernel-spec.md` | `06-reference-mvp.md` |
| Verification verdict | `evals` + artifact refs | `03-kernel-spec.md` | `06-reference-mvp.md` |
| Validator result | validator result payloads | `05-mcp-api-and-schemas.md` | n/a |

## 4. Prohibited Duplicate Ownership Patterns

### 4.1 Strategy Owning Implementation Detail

Do not put these in `02-strategy.md`:

```text
- full gate enum
- transition matrix
- SQLite table list
- request/response schema
- full template text
```

### 4.2 User Guide Owning Contracts

Do not put these in `10-user-guide.md`:

```text
- tool payload schema
- DB fields
- validator result schema
- artifact layout
```

### 4.3 Integration Owning Surface Cookbooks

Do not put repeated surface addenda in `09-agent-integration.md`. Use `appendix/B-surface-cookbook.md`.

### 4.4 Projection Owning State

Do not let `TASK`, `DOMAIN-LANGUAGE`, or `MODULE-MAP` documents become authoritative state. They are projection and proposal surfaces.

### 4.5 Reference MVP Owning Strategy

`06-reference-mvp.md` implements decisions. It does not re-decide core invariants, policy defaults, or UX philosophy.

## 5. Minimal Cross-References Required

| From | Must Reference |
|---|---|
| `02-strategy.md` | `03-kernel-spec.md`, `08-design-quality-policy-pack.md` |
| `03-kernel-spec.md` | `05-mcp-api-and-schemas.md`, `06-reference-mvp.md` |
| `04-runtime-architecture.md` | `03-kernel-spec.md`, `07-document-projection.md`, `09-agent-integration.md` |
| `05-mcp-api-and-schemas.md` | `03-kernel-spec.md`, `06-reference-mvp.md` |
| `06-reference-mvp.md` | `03-kernel-spec.md`, `05-mcp-api-and-schemas.md`, `11-operations-and-conformance.md` |
| `07-document-projection.md` | `03-kernel-spec.md`, `appendix/A-template-library.md` |
| `08-design-quality-policy-pack.md` | `03-kernel-spec.md`, `07-document-projection.md` |
| `09-agent-integration.md` | `05-mcp-api-and-schemas.md`, `appendix/B-surface-cookbook.md` |
| `11-operations-and-conformance.md` | `05-mcp-api-and-schemas.md`, `06-reference-mvp.md` |
