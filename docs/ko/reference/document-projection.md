# 문서 Projection 참조

## 이 문서로 할 수 있는 일

이 참조 문서는 Harness가 기준 상태 기록과 artifact reference를 바탕으로 사람이 읽을 수 있는 Markdown projection을 어떻게 생성하는지 확인할 때 사용합니다.

Projection의 권한 경계, managed block의 동작, 사람이 편집할 수 있는 영역, artifact reference 표시 방식, template tier, projection freshness rule을 정의합니다. Canonical kernel state, MCP request/response schema, SQLite DDL, design-quality policy requirement, full template body text는 정의하지 않습니다. MVP-required projection에서는 full template body를 [Template 참조](templates/README.md)에서 찾습니다. Optional 및 extension full template body의 현재 위치는 이동 전까지 [Appendix A](../appendix/A-template-library.md)입니다.

## 읽는 시점

- Markdown projection 동작을 구현하거나 리뷰할 때
- report, status card, Journey Card가 canonical state인지 확인할 때
- projected Markdown에 남긴 사람이 쓴 내용이 state가 되는 경로를 판단할 때
- `TASK`, `APR`, `RUN-SUMMARY`, `EVIDENCE-MANIFEST`, `EVAL`, `DIRECT-RESULT`가 참조하는 source record 목록을 확인할 때
- stale, failed, drifted projection을 진단할 때

## Projection을 쉽게 말하면

Harness projection은 이미 canonical state 또는 artifact storage에 있는 작업을 사람이 읽기 좋게 보여주는 view입니다. Projector는 `state.sqlite` record, `state.sqlite.task_events`, 등록된 artifact reference를 읽어 `TASK`, `APR`, `RUN-SUMMARY`, `EVIDENCE-MANIFEST`, `EVAL`, `DIRECT-RESULT` 같은 Markdown을 생성합니다.

Markdown은 사람이 작업을 이해하고, context를 다시 잡고, evidence를 살펴보고, correction을 제안하는 데 도움을 줍니다. 하지만 Markdown이 작업을 소유하지는 않습니다. Report는 gate를 요약하거나, evidence를 link하거나, Write Authorization ref를 표시하거나, Decision Packet을 보여줄 수 있지만, report text 자체가 gate, evidence, authorization, decision은 아닙니다.

엄격한 경계는 다음과 같습니다.

| Item | What it is | Authority |
|---|---|---|
| Raw artifact | diff, log, screenshot, checkpoint, bundle, manifest file 같은 durable evidence file | artifact store |
| State record | Task, Change Unit, Decision Packet, Journey Spine Entry, Residual Risk, Run, Approval, Write Authorization, Eval, Manual QA record, Evidence Manifest, Artifact record, Reconcile Item 같은 canonical structured record | `state.sqlite` |
| Markdown report | record와 artifact ref에서 만든 human-readable projection | projector output |

Markdown report는 evidence를 link하고 state를 요약할 수 있지만 raw artifact나 state record가 아닙니다.

## 담당하는 참조 범위

이 문서는 다음을 담당합니다.

- projection principles
- document authority matrix
- managed block rules
- 사람이 편집할 수 있는 영역의 rules
- artifact reference 표시 rules
- template tiers
- projection source-record rules
- projection freshness and failure rules
- projection rule 수준의 `source_state_version`과 `managed_hash` 해석

## 여기서 다루지 않는 것

이 문서는 다음을 담당하지 않습니다.

- canonical kernel state와 transition rules. [Kernel Reference](kernel.md)를 봅니다.
- public MCP request/response schemas. [MCP API And Schemas](mcp-api-and-schemas.md)를 봅니다.
- SQLite DDL과 storage layout. [Storage And DDL](storage-and-ddl.md)를 봅니다.
- design-quality policy contracts. [설계 품질 정책](design-quality-policies.md)을 봅니다.
- operator command semantics. [Operations And Conformance](../11-operations-and-conformance.md)를 보고, 이후 경로는 `reference/operations-and-conformance.md`입니다.
- conformance fixture assertion semantics. [Operations And Conformance](../11-operations-and-conformance.md)를 보고, 이후 경로는 `reference/operations-and-conformance.md`입니다.
- optional 또는 extension projection의 full template body. 이동하기 전까지 [Appendix A](../appendix/A-template-library.md)를 봅니다.
- connector capability profiles 또는 surface recipes. [Agent Integration](../09-agent-integration.md)을 보고, 이후 경로는 `reference/agent-integration.md`입니다.
- MVP-required full template body. [Template 참조](templates/README.md)를 봅니다.

## 작은 generated TASK 예시

일부러 아주 작게 보여주는 예시입니다. 전체 render 형태는 [TASK template](templates/task.md)에 있습니다.

```md
---
doc_type: task
task_id: TASK-0001
display_state: executing
projection_version: 7
source_state_version: 42
updated_at: 2026-05-06T09:30:15+09:00
---

# TASK-0001 Add Import Preview

<!-- HARNESS:BEGIN managed -->
## Current Summary
- mode: work
- lifecycle phase: executing
- next action: record evidence for CU-01
- evidence gate: pending
- verification gate: pending
- Manual QA: pending
- active change unit: CU-01
- projection freshness: current

## Evidence And Reports
- Run Summary: RUN-20260506-093015-LEAD-01
- Diff: DIFF-0001 (`artifact_id=ART-0001`, sha256:abc123..., redaction:none)
<!-- HARNESS:END managed -->

## User Notes and Proposals
-
```

## 사람이 편집할 수 있는 것

사람은 다음과 같이 명시적으로 편집 가능하다고 표시된 영역을 편집할 수 있습니다.

```md
## User Notes and Proposals
-
```

사람이 편집할 수 있는 text는 입력입니다. Note, question, correction, proposal을 담을 수 있습니다. Reconcile은 편집 내용을 읽고 `reconcile_items` candidate를 만들 수 있습니다. Accepted proposal은 Core state-changing action과 appended `state.sqlite.task_events` row를 통해서만 state가 됩니다. Rejected proposal은 note 또는 rejected reconcile item으로 남습니다.

사람이 편집한 proposal은 Task summary, acceptance criteria, Domain Language, Module Map, Interface Contract, Manual QA note, 기타 state-backed record를 target할 수 있지만 proposal 자체가 target record는 아닙니다.

## 사람이 state에 직접 반영할 수 없는 것

사람은 다음 projection text를 기준 상태에 직접 반영할 수 없습니다.

- managed block content
- `source_state_version` 같은 front matter field
- current gate value, lifecycle phase, result, close reason, assurance level
- approval, verification, Manual QA, acceptance, residual-risk status
- Decision Packet, Journey Card, Journey Spine, Autonomy Boundary, Write Authority Summary, Implementation Micro-Plan, Change Unit DAG, Residual Risk, Stewardship Impact, Review Stage, Write Authorization display text
- artifact reference identity, hash, redaction state, artifact availability
- status card, Journey Card, 기타 generated display surface
- template body

Managed block 안의 direct edit는 accepted state가 아니라 drift입니다. 표시된 authority text를 직접 편집해도 write를 authorize하거나, decision을 resolve하거나, evidence를 satisfy하거나, verification 또는 Manual QA를 replace하거나, residual risk를 accept하거나, assurance를 upgrade하거나, work를 close하거나, owner record를 변경하지 않습니다.

## Projection principles

1. Projection은 source-of-truth가 아닙니다.
2. Canonical operational state는 `state.sqlite` current record와 `state.sqlite.task_events`입니다.
3. Raw evidence는 artifact store에서 canonical합니다.
4. Markdown report는 state record와 artifact reference를 바탕으로 생성됩니다.
5. Markdown report는 기본적으로 raw artifact가 아닙니다.
6. Front matter는 identity, projection version 또는 status, `source_state_version`, timestamp/freshness metadata만 가집니다.
7. Managed block은 projector가 생성하며 필요하면 다시 생성될 수 있습니다.
8. 사람이 편집할 수 있는 영역은 note와 proposal을 위한 입력 영역입니다.
9. 수용된 human edit만 reconcile 또는 Core state-changing action을 통해 state가 됩니다.
10. Large log, diff, trace, screenshot, bundle, checkpoint는 embed하지 않고 artifact ref로 연결합니다.
11. Projection failure 또는 staleness는 underlying task result를 절대 바꾸지 않습니다.
12. User-facing card는 friendly label을 사용할 수 있지만 canonical gate name은 kernel field로 남습니다.
13. Decision Packet, Journey Card, Journey Spine, Autonomy Boundary, Write Authority Summary, Implementation Micro-Plan, Change Unit DAG, Residual Risk, Stewardship Impact 표시는 owner record와 artifact ref에서 만든 non-canonical projection입니다.

## Document authority matrix

| Fact or surface | Canonical source | Projection 또는 표시되는 뷰 | Update path |
|---|---|---|---|
| Current Task state | `state.sqlite.tasks`, `task_gates`, `state.sqlite.task_events` | `TASK` Current Summary와 status card | Core transition, then projector |
| Task continuity | `state.sqlite` Task, Change Unit, Run, Evidence Manifest, Eval, Manual QA, Decision Packet, Approval, Residual Risk, `task_gates.acceptance_gate`, acceptance Decision Packet user-decision state, close events, artifact ref, 필요할 때 `journey_spine_entries`, `state.sqlite.task_events` | `TASK` Journey Spine | Core transition 또는 reconcile, Journey reconstruction, then projector |
| Decision Packet | `state.sqlite.decision_packets`, 관련 `decision_gate` state, decision event, 관련 approval 또는 reconcile record, artifact ref, 필요할 때 연결된 `state.sqlite.residual_risks` | `TASK` Pending Decisions, Journey Card decision line, status/next responses, judgment-context resources, decision-packet resources; standalone projection이 enabled일 때 optional `DEC` | `request_user_decision` / `record_user_decision`, then projector |
| Journey Spine | `state.sqlite` Task, Change Unit, Run, Decision Packet, Approval, Evidence Manifest, Eval, Manual QA, Residual Risk, `task_gates.acceptance_gate`, acceptance Decision Packet user-decision state, close events, artifact ref, 필요할 때 `journey_spine_entries`, `state.sqlite.task_events` | `TASK` Journey Spine section, resume view, Journey Spine-oriented card | Core transition 또는 reconcile, Journey reconstruction, then projector |
| Journey Card | current `state.sqlite` Task state, gate, active Change Unit, Autonomy Boundary summary, active Decision Packet ref, residual-risk summary, latest evidence/eval/QA/report ref, projection freshness | `JOURNEY-CARD`, status card, `harness.status` card text, `harness.next` current-position text, significant resume output | current state에서 read 또는 projection refresh; card를 직접 edit하지 않음 |
| Autonomy Boundary | active `state.sqlite.change_units` Autonomy Boundary field와 관련 Decision Packet resolution/event | `TASK` Autonomy Boundary, Change Unit block, Journey Card autonomy line, standalone projection이 enabled일 때 optional related `DEC` | shaping update 또는 user Decision Packet resolution, then projector |
| Write Authorization | `state.sqlite.write_authorizations`와 관련 Task, Change Unit, approval, Decision Packet, baseline, consumed Run ref | `TASK` Write Authority Summary, Journey Card Write Authority Summary line, `RUN-SUMMARY` relation | `prepare_write`가 create함; idempotent replay는 already committed response를 반환함; `record_run`이 authorization을 consume한 뒤 projector |
| Implementation Micro-Plan | current `state.sqlite` Task state와 gate, active Change Unit scope와 Autonomy Boundary, Change Unit dependency summary, selected feedback-loop records, TDD가 selected된 경우 TDD traces, expected evidence needs, Decision Packet blockers, latest report refs | `TASK` Implementation Micro-Plan managed section | Accepted reconcile outcome 또는 Core state-changing action이 owner record를 update한 뒤 projector |
| Change Unit DAG | `state.sqlite.change_units`, `state.sqlite.change_unit_dependencies`, dependency 관련 event, active Task state | `TASK` Change Unit Dependencies / DAG summary | shaping update 또는 reconcile, then projector |
| Residual Risk | `state.sqlite.residual_risks`, accepted-risk metadata와 residual-risk refs, related Decision Packet, evidence/QA/eval ref, artifact ref | `TASK` Residual Risk, standalone projection이 enabled일 때 optional `DEC` accepted-risk context, Journey Card residual-risk line | decision, evidence, QA, Eval, reconcile 또는 close flow에서 Core transition, then projector |
| Stewardship Impact Summary | `domain_terms`, `module_map_items`, `interface_contracts`, `feedback_loops`, TDD가 selected된 경우 TDD records, `state.sqlite.residual_risks`, `state.sqlite.decision_packets`, policy validator results, related refs | `TASK` Stewardship Impact와 status/resume stewardship display | Owner record update, validator result, reconcile, close flow, then projector |
| User Notes | human-editable input -> `reconcile_items` -> accepted state event/record | `TASK` User Notes and Proposals | human edit, reconcile decision, Core event |
| Shared Design | shared design record와 event | `TASK` summary, `DESIGN`, standalone projection이 enabled일 때 optional `DEC` | Core transition 또는 reconcile, then projector |
| Domain Language | `domain_terms` table | `DOMAIN-LANGUAGE` projection | Core transition 또는 reconcile, then projector |
| Module Map | `module_map_items` table | `MODULE-MAP` projection | Core transition 또는 reconcile, then projector |
| Interface Contract | `interface_contracts` table | `INTERFACE-CONTRACT` projection | Core transition 또는 reconcile, then projector |
| Feedback Loop | `feedback_loops` table plus runs, artifacts, TDD traces, Manual QA, evidence manifests refs | `TASK` Stewardship Impact와 Evidence Manifest design-quality coverage; MVP에는 standalone Feedback Loop projection이 없음 | `record_run` shaping 또는 evidence update의 `FeedbackLoopUpdate`, `record_manual_qa`의 `feedback_loop_ref`, 또는 reconcile, then projector |
| Approval | `approvals`, approval-shaped Decision Packet, 구현이 유지하는 경우 optional decision request routing/replay record, event; `approval_request_candidate` alone은 제외 | `APR` projection과 approval card | `request_user_decision(decision_kind=approval)`이 pending Approval record를 create하고, `record_user_decision`이 approval decision을 update한 뒤 projector |
| Run summary | `runs` table plus artifact refs | `RUN-SUMMARY` projection | `record_run`, then projector |
| Direct result | direct run record plus artifact refs | `DIRECT-RESULT` projection | `record_run` / `close_task`, then projector |
| Evidence coverage | `evidence_manifests` plus artifact refs | `EVIDENCE-MANIFEST` projection | evidence module update, then projector |
| Verification verdict | `evals` plus artifact refs | `EVAL` projection과 verification card | `record_eval`, then projector |
| TDD trace | `tdd_traces` plus artifact refs | `TDD-TRACE` projection | `record_run` 또는 reconcile, then projector |
| Manual QA | Aggregate QA requirement state에는 `qa_gate`; record가 있을 때 `manual_qa_records` plus artifact refs | `MANUAL-QA` projection과 QA card | `record_manual_qa`, then projector |
| Raw evidence | artifact store plus `artifacts` records | report 안의 artifact reference | artifact registry |
| Projection freshness | `projection_jobs.source_state_version`, `projection_jobs.projection_version`, job status, managed hashes, artifact records | front matter mirror, status card, operations output | projector and recovery tools |

Required authority statements:

- User Notes: human-editable input -> `reconcile_items` -> accepted state event/record
- Domain Language: `domain_terms` table -> `DOMAIN-LANGUAGE` projection; canonical term row에 대한 public ref는 `StateRecordRef.record_kind=domain_term`을 사용합니다.
- Module Map: `module_map_items` table -> `MODULE-MAP` projection; canonical module row에 대한 public ref는 `StateRecordRef.record_kind=module_map_item`을 사용합니다.
- Interface Contract: `interface_contracts` table -> `INTERFACE-CONTRACT` projection; canonical contract row에 대한 public ref는 `StateRecordRef.record_kind=interface_contract`를 사용합니다.
- Feedback Loop: `feedback_loops` table -> `TASK`와 Evidence Manifest display; canonical feedback-loop row에 대한 public ref는 `StateRecordRef.record_kind=feedback_loop`를 사용합니다. TDD Trace refs는 separate execution evidence refs로 남습니다.
- Decision Packet: `state.sqlite.decision_packets`와 관련 ref -> `TASK` Pending Decisions, status/next responses, judgment-context resources, decision-packet resources; standalone projection이 켜져 있을 때 optional `DEC` projection
- Journey Spine: owner record, artifact ref, `journey_spine_entries` supplement, `state.sqlite.task_events`에서 재구성합니다. 자체 authority record가 아닙니다.
- Journey Card: current state와 ref에서 만든 derived display입니다. 절대 canonical state가 아닙니다.
- Autonomy Boundary: active `state.sqlite.change_units` boundary field -> projection surface. 판단 재량이지 scope authority가 아닙니다.
- Write Authority Summary: active scope, approval, Write Authorization, baseline, guarantee ref에서 만든 derived display입니다. 절대 canonical state가 아니며 work를 authorize할 수 없습니다.
- Write Authorization: `state.sqlite.write_authorizations`는 specific allowed write attempt를 기록합니다. Scope, approval, evidence, verification, QA, acceptance, residual-risk acceptance가 아닙니다.
- Implementation Micro-Plan: current Task와 Change Unit owner record plus related refs -> `TASK` managed execution-aid section. Canonical state가 아니고, 새 `ProjectionKind`도 아니며, scope authority, approval, Write Authorization이 아닙니다.
- Approval: `approvals`와 approval-shaped Decision Packet -> Approval record가 존재하거나 변경된 뒤에만 `APR` projection을 만듭니다. `prepare_write`가 반환한 `approval_request_candidate`는 candidate display로 표시할 수 있지만 `APR` source가 아닙니다.
- Change Unit DAG: `state.sqlite.change_unit_dependencies`와 Change Unit ref -> dependency projection. scheduler 또는 authorization surface가 아닙니다.
- Residual Risk: accepted-risk metadata/refs를 포함한 `state.sqlite.residual_risks` -> residual-risk display
- Stewardship Impact Summary: owner record, validator result, ref에서 derive됨 -> `StewardshipImpactSummary` display. canonical record가 아닙니다.
- Review Stages: Task, Change Unit, gates, evidence, validator results, residual-risk refs, stewardship owner refs -> Spec Compliance Review와 Code Quality / Stewardship Review라는 managed `TASK` 또는 `RUN-SUMMARY` display sections. Canonical records가 아니고, 새 `ProjectionKind` values가 아니며, detached verification이 아닙니다.

## Managed block rules

Managed block은 projector가 overwrite할 수 있는 유일한 Markdown 영역입니다.

```md
<!-- HARNESS:BEGIN managed -->
...
<!-- HARNESS:END managed -->
```

규칙:

- Managed block content는 committed state record와 artifact ref에서 생성됩니다.
- Projector는 `projection_jobs.source_state_version`, projection version, rendered timestamp, managed hash를 기록합니다. Front matter는 operator를 위해 recorded source state version을 mirror합니다.
- Managed hash는 `HARNESS:BEGIN`과 `HARNESS:END` marker lines를 제외한 projector-owned managed block body에서 계산하며, line endings를 LF로 normalize하고 projector rules가 요구하는 meaningful whitespace를 보존합니다.
- Rendering 전에 managed block hash가 last projected hash와 다르면 projector는 reconcile item을 create/update합니다.
- Managed hash는 drift detection에만 사용하며 render된 Markdown을 기준 상태로 만들지 않습니다.
- Projector는 managed block 내부의 direct edit를 accepted state로 조용히 취급하지 않습니다.
- Managed block을 다시 render할 때 관련 없는 사람이 편집할 수 있는 영역은 보존해야 합니다.
- Failed render는 projection freshness를 `failed` 또는 `stale`로 mark하며 state를 rollback하지 않습니다.

Front matter는 diagnostic 용도로 compact하게 유지합니다. Rendered object를 identify하고, projection version 또는 status를 표시하며, `source_state_version`을 mirror하고, rendered timestamp를 포함할 수 있습니다. Large state summary, evidence body, gate rollup, artifact inventory는 포함하면 안 됩니다.

`projection_version`은 projection/template/job version입니다. State clock이 아니며 source-state freshness basis로 사용하면 안 됩니다. `source_state_version`은 render source로 사용한 affected-scope state clock 값입니다. Projection이 task-scoped이면 Task State Version이고, 그렇지 않으면 Project State Version 또는 extension-defined owner state clock입니다.

Canonical per-projection value는 successful render job의 `projection_jobs.source_state_version`입니다. Front matter `source_state_version`은 operator diagnosis를 위해 그 값을 mirror할 뿐입니다.

## Human-editable section rules

- 사람이 편집할 수 있는 text는 입력이지 canonical state가 아닙니다.
- Reconcile은 edit를 읽고 state가 바뀌어야 할 수 있으면 `reconcile_items` candidate를 만듭니다.
- Accepted proposal은 Core transition과 appended `state.sqlite.task_events` row를 통해서만 state가 됩니다.
- Rejected proposal은 note 또는 rejected reconcile item으로 남습니다.
- Projector는 refresh 중 사람이 편집할 수 있는 content를 보존해야 합니다.
- 사람이 편집한 proposal은 Task summary, acceptance criteria, Domain Language, Module Map, Interface Contract, Manual QA note, 기타 state-backed record를 target할 수 있지만 proposal 자체가 target record는 아닙니다.

## Artifact reference rendering

Markdown report는 artifact reference를 compact하고 consistent하게 표시합니다. Payload shape는 MCP API document가 담당하며, projection은 presentation rule만 담당합니다.

권장 display:

```text
- Diff: DIFF-0001 (`artifact_id=ART-0001`, sha256:abc123..., redaction:none)
- Test log: LOG-0002 (`artifact_id=ART-0002`, sha256:def456..., redaction:redacted)
- Bundle: BUNDLE-0001 (`artifact_id=ART-0003`, sha256:789abc..., redaction:secret_omitted)
```

규칙:

- 모든 artifact ref는 artifact record로 resolve되어야 합니다.
- 모든 raw artifact ref는 integrity metadata와 redaction state를 가져야 합니다.
- Large 또는 sensitive evidence는 Markdown에 paste하지 않고 link만 둡니다.
- Missing 또는 hash-mismatched artifact는 related evidence 또는 projection freshness를 stale로 mark합니다.
- State record ref는 record identity와 optional projection path를 사용합니다. `record_kind=projection`에서 identity는 `projection_jobs.projection_job_id`이고 path는 locator일 뿐입니다. Raw artifact ref로 표시하지 않습니다.
- `artifact_links.record_kind`는 existing same-Task state owner 또는 projection ref로 resolve되어야 합니다. MVP artifact links는 Task-scoped입니다. `record_kind=projection`은 같은 `task_id`를 가진 completed `projection_jobs` row로 resolve되며, link의 `record_id`는 `projection_jobs.projection_job_id`이고 path display는 `StateRecordRef.projection_path` 또는 `projection_jobs.output_path`를 사용합니다. Project-level owner rows와 project-level projection jobs는 future extension이 project-scoped artifact linking을 추가하지 않는 한 state 또는 projection job freshness/output metadata를 사용합니다. `EXPORT`는 `ProjectionKind`일 뿐입니다. Export snapshot과 component는 owner record 또는 `record_kind=projection`에 link되는 artifact로 남으며 `record_kind=export`에 link하지 않습니다.

## Template tiers

Projection template은 API `ProjectionKind` tier와 일치합니다.

| Tier | Templates | Rule | Template reference |
|---|---|---|---|
| MVP-required | `TASK`, `APR`, `RUN-SUMMARY`, `EVIDENCE-MANIFEST`, `EVAL`, `DIRECT-RESULT` | MVP projector는 이를 render해야 합니다. | [TASK](templates/task.md), [APR](templates/approval.md), [RUN-SUMMARY](templates/run-summary.md), [EVIDENCE-MANIFEST](templates/evidence-manifest.md), [EVAL](templates/eval.md), [DIRECT-RESULT](templates/direct-result.md) |
| MVP-optional | `MANUAL-QA`, `TDD-TRACE`, `DOMAIN-LANGUAGE`, `MODULE-MAP`, `INTERFACE-CONTRACT` | Policy가 적용되거나, record가 있거나, user/operator가 projection을 켰을 때 render합니다. | Optional template이 이동하기 전까지 consolidated legacy body는 [Appendix A](../appendix/A-template-library.md)에 남아 있습니다. |
| Extension / appendix | `DEC`, `DESIGN`, `EXPORT`, `JOURNEY-CARD` | 해당 extension 또는 appendix projection이 켜져 있을 때만 render합니다. | Extension template이 이동하기 전까지 consolidated legacy body는 [Appendix A](../appendix/A-template-library.md)에 남아 있습니다. |

`ProjectionKind` tiering은 renderer support expectations를 정하지만 projection을 기준 상태로 만들지 않습니다.

`EXPORT` template은 optional projection output입니다. Artifact links를 위한 `export` state record를 도입하지 않습니다.

Persisted `JOURNEY-CARD` Markdown은 optional입니다. `harness.status`, `harness.next`, significant resume flow의 current-position Journey Card output은 agency conformance에 required입니다.

MVP Decision Packet visibility는 `TASK` projections, status/next responses, judgment-context resources, decision-packet resources를 통해 required입니다. Standalone `DEC` Markdown은 standalone Decision Packet projection feature가 enabled인 경우가 아니면 optional입니다.

Decision Packet record ID는 `DEC-*`를 사용합니다. `projection_kind`의 `DEC`는 projection kind label일 뿐입니다. Standalone projection에 별도 identity가 필요하면 `DEC-PROJ-0001` 같은 별도 `projection_id`를 사용합니다.

## Freshness and failure rules

Projection freshness는 current owner 또는 affected-scope state clock, canonical `projection_jobs.source_state_version`, projection job state, managed hash, artifact availability, known stale trigger에서 계산됩니다. Front matter `source_state_version`은 마지막 successful render의 canonical value를 mirror하여, operator가 Markdown을 canonical로 취급하지 않고도 stale projection을 diagnose할 수 있게 합니다.

| Projection | Generated when | Stale when |
|---|---|---|
| `TASK` | Task가 created, resumed, changed, refreshed될 때 | current `tasks.state_version > projection_jobs.source_state_version` for the `TASK` projection, managed block drift, unresolved reconcile required, stewardship owner ref 또는 design-quality validator result changed |
| `APR` | `request_user_decision(decision_kind=approval)`이 committed approval request를 create하거나, `record_user_decision`을 통해 approval decision이 changed될 때 | approval-shaped Decision Packet, linked Approval record status, scope, baseline, expiry, decision note가 changed |
| `RUN-SUMMARY` | `record_run`이 `runs.status=completed`, `interrupted`, `blocked`, `violation`을 포함한 Run을 commit할 때 | run relation changed, artifact ref missing, artifact integrity fails |
| `EVIDENCE-MANIFEST` | evidence coverage가 changed될 때 | baseline drift, changed files modified, required evidence missing/stale, approval expired |
| `EVAL` | verification result가 recorded될 때 | Eval 후 baseline changes, evidence becomes stale, independence relation invalidated |
| `DIRECT-RESULT` | direct run이 closes 또는 escalates될 때 | changed file drift, escalation state changes, artifact ref missing |
| `EXPORT` | export/report projection이 generated될 때. Enabled된 경우 Release Handoff profile 포함 | 포함된 Task/gate/Change Unit/Decision Packet/Residual Risk/evidence/verification/Manual QA/artifact/projection/redaction/checklist source가 changed 또는 unavailable일 때 |
| `DEC` | standalone Decision Packet projection이 enabled되어 있고 Decision Packet이 created, requested, resolved, deferred, rejected, blocked, superseded될 때 | packet status, affected scope, current-state context, related approval/reconcile state, residual-risk ref, evidence ref가 바뀔 때 |
| `JOURNEY-CARD` | card가 rendered 또는 projection으로 persisted될 때. `harness.status`와 `harness.next`가 projection job 없이 ephemeral하게 반환할 수도 있음 | 표시된 Task/gate/Change Unit/Autonomy Boundary/Write Authorization/approval/baseline/guarantee/Decision Packet/Residual Risk/evidence/report/freshness source가 rendered card보다 앞서 이동할 때 |
| `DOMAIN-LANGUAGE` | domain terms change | term conflict, accepted term record changes, related code representation moves |
| `MODULE-MAP` | module map records change | module path, public interface, dependency direction, internal complexity, test boundary, owner decision, watchpoint changes |
| `INTERFACE-CONTRACT` | interface contract records change | linked interface, caller, compatibility impact, boundary tests change |
| `TDD-TRACE` | trace recorded 또는 updated | red/green log missing, baseline drift, linked test file changes |
| `MANUAL-QA` | QA record created 또는 updated | linked UI/code changes, required capture missing, finding unresolved |

Freshness state:

| State | Meaning |
|---|---|
| `current` | projected content가 canonical `projection_jobs.source_state_version`에 기록된 committed state version 및 managed hash에 match함 |
| `stale` | state 또는 referenced evidence가 projection보다 앞서 이동함 |
| `failed` | projector가 refresh를 attempted했고 failed함 |
| `unknown` | freshness를 compute할 수 없음. 보통 recovery 또는 migration 중 |

Projection staleness는 current readable context가 필요한 action을 block할 수 있지만, 그 자체로 lifecycle result, gate value, assurance를 바꾸지는 않습니다. Projection failure 또는 staleness는 underlying task result를 절대 바꾸지 않습니다.
