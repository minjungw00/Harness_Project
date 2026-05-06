# 문서 Projection

## 문서 역할

이 문서는 Product Repository 안의 human-readable Markdown projection rule을 담당한다. Projection principle, document authority boundary, managed block rule, human-editable rule, artifact reference rendering, template tier, required MVP template summary, optional design-quality template summary, projection freshness rule을 정의한다.

Canonical kernel state, MCP request/response schema, SQLite DDL, design-quality policy requirement, full template text는 정의하지 않는다. Full template은 [Appendix A](appendix/A-template-library.md)에 있다.

## Projection Principles

1. Projection은 source-of-truth가 아니다.
2. Canonical operational state는 `state.sqlite` current record와 `state.sqlite.task_events`다.
3. Raw evidence는 artifact store에서 canonical하다.
4. Markdown report는 state record와 artifact reference에서 render된다.
5. Markdown report는 기본적으로 raw artifact가 아니다.
6. Front matter는 identity, projection version, status summary, freshness metadata만 가진다.
7. Managed block은 projector가 생성하며 regenerate될 수 있다.
8. Human-editable section은 note와 proposal을 위한 input surface다.
9. Human edit는 reconcile 또는 MCP tool이 accepted state event나 record를 기록한 뒤에만 state를 바꾼다.
10. Large log, diff, trace, screenshot, bundle, checkpoint는 embed하지 않고 artifact ref로 link한다.
11. Projection failure 또는 staleness는 underlying task result를 절대 바꾸지 않는다.
12. User-facing card는 friendly label을 사용할 수 있지만 canonical gate name은 kernel field로 남는다.

## Document Authority Matrix

| Fact or surface | Canonical source | Projection or display surface | Update path |
|---|---|---|---|
| Current Task state | `state.sqlite.tasks`, `task_gates`, `state.sqlite.task_events` | `TASK` Current Summary와 status card | Core transition, then projector |
| Task continuity | task record, run record, evidence record, decision, event | `TASK` Rolling Spine | Core transition 또는 reconcile, then projector |
| User Notes | human-editable input -> `reconcile_items` -> accepted state event/record | `TASK` User Notes and Proposals | human edit, reconcile decision, Core event |
| Shared Design | shared design record와 event | `TASK` summary, `DESIGN`, `DEC` | Core transition 또는 reconcile, then projector |
| Domain Language | `domain_terms` table | `DOMAIN-LANGUAGE` projection | Core transition 또는 reconcile, then projector |
| Module Map | `module_map_items` table | `MODULE-MAP` projection | Core transition 또는 reconcile, then projector |
| Interface Contract | `interface_contracts` table | `INTERFACE-CONTRACT` projection | Core transition 또는 reconcile, then projector |
| Approval | `approvals`, decision record, event | `APR` projection과 approval card | `request_user_decision` / `record_user_decision`, then projector |
| Run summary | `runs` table plus artifact refs | `RUN-SUMMARY` projection | `record_run`, then projector |
| Direct result | direct run record plus artifact refs | `DIRECT-RESULT` projection | `record_run` / `close_task`, then projector |
| Evidence coverage | `evidence_manifests` plus artifact refs | `EVIDENCE-MANIFEST` projection | evidence module update, then projector |
| Verification verdict | `evals` plus artifact refs | `EVAL` projection과 verification card | `record_eval`, then projector |
| TDD trace | `tdd_traces` plus artifact refs | `TDD-TRACE` projection | `record_run` 또는 reconcile, then projector |
| Manual QA | `manual_qa_records` plus artifact refs; `qa_gate` is canonical gate | `MANUAL-QA` projection과 QA card | `record_manual_qa`, then projector |
| Raw evidence | artifact store plus `artifacts` records | report 안의 artifact reference | artifact registry |
| Projection freshness | `projection_jobs`, projected versions, managed hashes | front matter, status card, operations output | projector and recovery tools |

Required authority statement:

- User Notes: human-editable input -> `reconcile_items` -> accepted state event/record
- Domain Language: `domain_terms` table -> `DOMAIN-LANGUAGE` projection
- Module Map: `module_map_items` table -> `MODULE-MAP` projection
- Interface Contract: `interface_contracts` table -> `INTERFACE-CONTRACT` projection

## Markdown Report Boundary

경계는 의도적으로 엄격하다.

| Item | What it is | Authority |
|---|---|---|
| Raw artifact | diff, log, screenshot, checkpoint, bundle, manifest file 같은 durable evidence file | artifact store |
| State record | Task, Run, Approval, Eval, Manual QA record, Evidence Manifest, Artifact record, Reconcile Item 같은 canonical structured record | `state.sqlite` |
| Markdown report | record와 artifact ref에서 만든 human-readable projection | projector output |

이 report kind는 기본적으로 projection 또는 state-backed record다. Artifact store의 evidence file에 link할 수 있고 export가 snapshot을 포함할 수 있지만, 그렇다고 Markdown report가 canonical evidence가 되지는 않는다.

## Managed Blocks

Managed block은 projector가 overwrite할 수 있는 유일한 Markdown area다.

```md
<!-- HARNESS:BEGIN managed -->
...
<!-- HARNESS:END managed -->
```

규칙:

- Managed block content는 committed state record와 artifact ref에서 생성된다.
- Projector는 state version, projection version, rendered timestamp, managed hash를 기록한다.
- Rendering 전에 managed block hash가 last projected hash와 다르면 projector는 reconcile item을 create/update한다.
- Projector는 managed block 내부의 direct edit를 accepted state로 조용히 취급하지 않는다.
- Managed block을 re-render할 때 unrelated human-editable section은 preserve해야 한다.
- Failed render는 projection freshness를 `failed` 또는 `stale`로 mark하며 state를 rollback하지 않는다.

## Human-Editable Section

Human-editable section은 사용자가 note, question, correction, proposal을 남기는 공간이다.

```md
## User Notes and Proposals
-
```

규칙:

- Human-editable text는 input이지 canonical state가 아니다.
- Reconcile은 edit를 읽고 state가 바뀌어야 할 수 있으면 `reconcile_items` candidate를 만든다.
- Accepted proposal은 Core transition과 appended `state.sqlite.task_events` row를 통해서만 state가 된다.
- Rejected proposal은 note 또는 rejected reconcile item으로 남는다.
- Projector는 refresh 중 human-editable content를 preserve해야 한다.
- Human-editable proposal은 Task summary, acceptance criteria, Domain Language, Module Map, Interface Contract, Manual QA note, 기타 state-backed record를 target할 수 있지만 proposal 자체가 target record는 아니다.

## Artifact References In Markdown

Markdown report는 artifact reference를 compact하고 consistent하게 render한다. Payload shape는 MCP API document가 담당하며, projection은 presentation rule만 담당한다.

권장 display:

```text
- Diff: DIFF-0001 (`artifact_id=ART-0001`, sha256:abc123..., redaction:none)
- Test log: LOG-0002 (`artifact_id=ART-0002`, sha256:def456..., redaction:redacted)
- Bundle: BUNDLE-0001 (`artifact_id=ART-0003`, sha256:789abc..., redaction:secret_omitted)
```

규칙:

- 모든 artifact ref는 artifact record로 resolve되어야 한다.
- 모든 raw artifact ref는 integrity metadata와 redaction state를 가져야 한다.
- Large 또는 sensitive evidence는 Markdown에 paste하지 않고 link한다.
- Missing 또는 hash-mismatched artifact는 related evidence 또는 projection freshness를 stale로 mark한다.
- State record ref는 record identity와 optional projection path를 사용한다. Raw artifact ref로 render하지 않는다.

## Template Tiers

Projection template에는 세 tier가 있다.

| Tier | Templates | Rule |
|---|---|---|
| Required MVP | `TASK`, `APR`, `RUN-SUMMARY`, `EVIDENCE-MANIFEST`, `EVAL`, `DIRECT-RESULT` | MVP projector는 이를 render해야 한다. |
| Optional design-quality | `DOMAIN-LANGUAGE`, `MODULE-MAP`, `INTERFACE-CONTRACT`, `TDD-TRACE`, `MANUAL-QA` | Policy가 적용되거나, record가 있거나, user/operator가 projection을 enable할 때 render한다. |
| Appendix variants | `DEC`, `DESIGN`, `EXPORT`, expanded cards, connector context templates | Full text는 Appendix A 또는 surface cookbook에 있다. |

Main doc은 각 template의 purpose와 source record만 정의한다. Full template body는 [Appendix A](appendix/A-template-library.md)에 있다.

## Required MVP Templates

### TASK

목적: active work를 위한 continuity projection이다. Mode, lifecycle phase, next action, current gate, active Change Unit, pending decision, evidence, report ref, projection freshness를 요약한다.

Source: Task, task gate, active Change Unit, pending decision request, latest Run, latest Evidence Manifest, latest Eval, latest Manual QA record, approval record, projection freshness.

Human-editable area: User Notes and Proposals.

### APR

목적: sensitive change를 위한 readable approval request와 decision record다.

Source: approval record, decision request, Change Unit scope, sensitive category, allowed path/tool/command/network/secret, baseline, expiry, alternative, decision note.

Boundary: approval은 correctness를 prove하지 않고, evidence를 satisfy하지 않으며, verification이나 Manual QA를 replace하지 않고, acceptance를 imply하지 않는다.

### RUN-SUMMARY

목적: execution run의 readable summary다.

Source: run record, actor/surface identity, baseline, Change Unit, changed path, command result, validator result, artifact ref, evidence update, follow-up.

Boundary: raw log와 diff는 artifact로 남고 report는 link한다.

### EVIDENCE-MANIFEST

목적: acceptance criteria와 completion condition에서 supporting evidence로 가는 readable map이다.

Source: evidence manifest record, acceptance criteria, changed file coverage, design-quality coverage, approval ref, artifact ref, related Run, Eval, Manual QA, TDD trace ref.

Boundary: evidence가 required인 곳에서 close는 report text alone이 아니라 canonical `evidence_gate`에 의존한다.

### EVAL

목적: independence context를 포함한 readable verification result다.

Source: Eval record, verification target, verdict, independence qualifier, baseline relationship, performed check, reviewed evidence, blocker, artifact ref.

Boundary: Eval verdict alone은 assurance를 upgrade하지 않는다. `detached_verified`에는 valid independence와 passed verification, same-session self-review violation 부재가 필요하다.

### DIRECT-RESULT

목적: 작은 direct work를 위한 compact result report다.

Source: direct run record, changed path, performed check, artifact ref, escalation flag, close assurance.

Boundary: policy 또는 user가 detached verification이나 다른 gate를 요구하지 않는 한 direct work는 기본적으로 self-checked로 close될 수 있다.

## Optional Template Summary

### DOMAIN-LANGUAGE

목적: canonical product vocabulary의 readable projection이다.

Source: `domain_terms` table. Human edit는 `domain_terms`로 reconcile되는 proposal이다.

### MODULE-MAP

목적: module, responsibility, public interface, dependency, test boundary, watchpoint의 readable projection이다.

Source: `module_map_items` table. Human edit는 module map record로 reconcile되는 proposal이다.

### INTERFACE-CONTRACT

목적: public interface expectation, compatibility impact, caller, boundary test의 readable projection이다.

Source: `interface_contracts` table. Human edit는 interface contract record로 reconcile되는 proposal이다.

### TDD-TRACE

목적: readable red/green/refactor evidence trail 또는 recorded non-TDD justification이다.

Source: `tdd_traces` plus artifact refs. Trace가 required인지 waivable인지는 policy가 결정한다.

### MANUAL-QA

목적: UX, workflow, copy, accessibility, visual output, 기타 experiential quality에 대한 readable human inspection record다.

Source: `manual_qa_records` plus artifact refs. User-facing card는 `Manual QA: pending/passed/failed/waived`라고 말할 수 있지만 `qa_gate`가 canonical close-relevant gate로 남는다.

## Status Cards

Status card는 state가 아니라 derived display다. 다음과 같은 friendly label을 사용할 수 있다.

```text
Manual QA: pending
Manual QA: passed
Manual QA: failed
Manual QA: waived
```

Card는 card text가 canonical임을 imply하면 안 된다. Canonical field는 `qa_gate`다.

## Projection Freshness

Projection freshness는 state version, projection job state, managed hash, artifact availability, known stale trigger에서 compute된다.

| Projection | Generated when | Stale when |
|---|---|---|
| `TASK` | Task가 created, resumed, changed, refreshed될 때 | `state_version > projected_version`, managed block drift, unresolved reconcile required |
| `APR` | approval request 또는 decision이 changed될 때 | approval status, scope, baseline, expiry, decision note가 changed |
| `RUN-SUMMARY` | run이 completes 또는 interrupted될 때 | run relation changed, artifact ref missing, artifact integrity fails |
| `EVIDENCE-MANIFEST` | evidence coverage가 changed될 때 | baseline drift, changed files modified, required evidence missing/stale, approval expired |
| `EVAL` | verification result가 recorded될 때 | Eval 후 baseline changes, evidence becomes stale, independence relation invalidated |
| `DIRECT-RESULT` | direct run이 closes 또는 escalates될 때 | changed file drift, escalation state changes, artifact ref missing |
| `DOMAIN-LANGUAGE` | domain terms change | term conflict, accepted term record changes, related code representation moves |
| `MODULE-MAP` | module map records change | module path, public interface, dependency direction, test boundary changes |
| `INTERFACE-CONTRACT` | interface contract records change | linked interface, caller, compatibility impact, boundary tests change |
| `TDD-TRACE` | trace recorded 또는 updated | red/green log missing, baseline drift, linked test file changes |
| `MANUAL-QA` | QA record created 또는 updated | linked UI/code changes, required capture missing, finding unresolved |

Freshness state:

| State | Meaning |
|---|---|
| `current` | projected content가 committed state version과 managed hash에 match함 |
| `stale` | state 또는 referenced evidence가 projection보다 앞서 이동함 |
| `failed` | projector가 refresh를 attempted했고 failed함 |
| `unknown` | freshness를 compute할 수 없음. 보통 recovery 또는 migration 중 |

Projection staleness는 current readable context가 필요한 action을 block할 수 있지만, 그 자체로 lifecycle result, gate value, assurance를 바꾸지는 않는다.
