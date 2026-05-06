# 커널 명세

## 문서 역할

이 문서는 하네스의 운영 커널 명세를 담당한다. Entity model, lifecycle model, gate, state compatibility rule, transition table, close semantics, waiver semantics, `prepare_write` state logic, `close_task` state logic, invariant enforcement mapping을 정의한다.

이 문서는 MCP wire schema, SQLite DDL, projection template text, design-quality playbook procedure, connector capability schema, capability를 first-class kernel gate로 다루는 방식을 정의하지 않는다.

## 커널 범위

Kernel은 로컬 AI 지원 product work를 위한 canonical state machine이다. Kernel은 다음을 결정한다.

- 어떤 Task가 active인지
- 어떤 Change Unit이 product write 범위를 지정하는지
- write를 진행할 수 있는지
- 어떤 approval, evidence, verification, QA, acceptance gate가 적용되는지
- task를 close할 수 있는지
- 어떤 state event를 append할지
- 어떤 projection을 refresh해야 하는지

Operational state는 current record와 append-only event로 저장된다. MVP에서 event history는 `state.sqlite.task_events`이며, 별도의 event store가 아니다.

Markdown document는 projection이자 proposal surface다. Raw evidence file은 artifact store에 있다. Kernel은 둘 모두에 대한 reference를 기록하지만, chat text도 generated Markdown도 canonical state를 대체하지 않는다.

## 작업 모드

`advisor`는 read-only explanation, comparison, review, decision support에 사용한다. Product write를 허가하지 않는다. Advisor task는 보통 `result=advice_only`로 close되며, policy나 user가 명시적으로 요구하지 않는 한 evidence, verification, QA, acceptance gate는 필요하지 않다.

`direct`는 scope와 result가 명확한 작고 low-risk인 product change에 사용한다. Write-capable이므로 product write에는 여전히 active scoped Change Unit이 필요하다. Direct work는 기본적으로 `self_checked`로 close될 수 있다. Optional detached verification이 수행되고 valid independence qualifier와 함께 통과하면 direct work를 `detached_verified`로 표시할 수 있다.

`work`는 structured implementation, non-local change, riskier change, independent verification이 필요한 작업에 사용한다. Write-capable이며 product write 전에 active scoped Change Unit이 필요하고, same-session self-review로 `detached_verified`를 받을 수 없다.

## Entity Model

### Task

Task는 사용자 가치 단위다. Current mode, lifecycle phase, result, close reason, assurance level, gate state, current summary, acceptance criteria, pending decision, active Change Unit, active Run, latest record reference, projection freshness를 가진다. Task는 status, resume, close decision에 사용되는 primary state record다.

### Change Unit

Change Unit은 product write를 위한 scoped implementation unit이다. Purpose, non-goal, slice type, intended end-to-end path, allowed path, allowed tool, validator profile, sensitive category, approval need, evidence expectation, QA expectation, dependency, merge risk, completion condition, evaluator focus를 기록한다.

모든 product write에는 intended write를 포함하는 active Change Unit이 필요하다. Task에는 하나 이상의 Change Unit이 있을 수 있지만, current write를 허가하는 것은 active Change Unit뿐이다.

### Run

Run은 lead agent, evaluator, operator, 기타 actor의 execution attempt다. Actor identity, surface identity, mode, Change Unit, baseline, intended operation, observed change, command result, artifact reference, summary를 기록한다. Lead Run은 shape하거나 implement할 수 있다. Evaluator Run은 별도 verification boundary에서 verify하며, independence qualifier가 valid하지 않으면 detached verification이 될 수 없다.

### Approval

Approval은 sensitive change를 위한 scope-bound prior decision이다. Approved path, tool, command 또는 command class, network target, secret scope, baseline, sensitive category, expiry condition, user decision을 기록한다. Approval은 correctness를 증명하지 않고, evidence를 대체하지 않으며, QA를 만족시키거나 acceptance를 뜻하지 않는다.

### Evidence Manifest

Evidence Manifest는 acceptance criteria 또는 completion condition을 evidence reference에 매핑한다. 각 criterion이 supported, unsupported, not applicable인지 기록하고 durable artifact, run summary, Eval record, TDD trace, Manual QA record, 기타 recorded evidence를 참조한다. Evidence sufficiency는 이 manifest와 관련 record를 기준으로 판단한다.

### Eval

Eval은 verification result record다. Verification target, verdict, performed check, reviewed evidence, independence qualifier, baseline relationship, blocker, artifact reference를 기록한다. Eval verdict만으로 assurance를 올리지는 않는다. `assurance_level=detached_verified`에는 passed verification result, valid independence qualifier, same-session self-review violation 부재가 필요하다.

### Manual QA

Manual QA는 UX, workflow, copy, accessibility, visual output, product taste 또는 사람의 judgment가 필요한 기타 result에 대한 human inspection record다. `qa_gate`는 canonical kernel gate다. `manual_qa_record.result`는 gate에 반영될 수 있는 individual record의 result다.

### Artifact

Artifact는 diff, log, bundle, manifest, screenshot, checkpoint, exported bundle component처럼 artifact store에 보관되는 durable evidence file이다. Artifact record는 reference와 integrity metadata로 이 file을 identify하고 verify한다. Raw artifact는 Markdown report와 state record와 구분된다.

### Reconcile Item

Reconcile Item은 human-editable content나 generated projection drift가 state에 영향을 줄 필요가 있을 때 생성되는 canonical candidate record다. Reconcile decision은 item을 merge, reject, note로 convert, decision 생성, defer할 수 있다. Human-editable text는 input이다. Accepted state change는 reconcile action과 state event를 통해서만 발생한다.

### Design Support Records

Kernel은 design support record의 entity meaning도 담당한다.

- Shared Design record는 goal, scope, assumption, rejected option, acceptance criteria, decision을 capture한다.
- Domain Term record는 Domain Language의 canonical source다.
- Module Map Item record는 Module Map의 canonical source다.
- Interface Contract record는 Interface Contract의 canonical source다.
- TDD Trace record는 red, green, refactor evidence 또는 recorded non-TDD justification을 capture한다.

이들의 policy requirement는 design-quality policy pack이 담당한다. Storage DDL은 reference MVP document가 담당한다.

## Authority Rules

User Notes authority는 다음과 같다.

```text
human-editable input -> reconcile_items -> accepted state event/record
```

Domain Language canonical source는 `domain_terms`다.

Module Map canonical source는 `module_map_items`다.

Interface Contract canonical source는 `interface_contracts`다.

`DOMAIN-LANGUAGE`, `MODULE-MAP`, `INTERFACE-CONTRACT` Markdown document는 projection이자 proposal surface다. 이 문서들은 canonical record를 override하지 않는다.

## Lifecycle Model

Kernel은 lifecycle field와 gate를 함께 사용한다. Compact display state는 이 canonical field에서 파생된다.

### Mode

```text
advisor | direct | work
```

### Lifecycle Phase

```text
intake | shaping | ready | executing | verifying | qa |
waiting_user | blocked | completed | cancelled
```

### Result

```text
none | advice_only | passed | failed | cancelled
```

### Close Reason

```text
none | completed_verified | completed_self_checked |
completed_with_risk_accepted | cancelled | superseded
```

### Assurance Level

```text
none | self_checked | detached_verified
```

Assurance는 approval, QA, acceptance가 아니다. Run, evidence, Eval record, verification independence가 뒷받침하는 technical checking level을 요약한다.

## Gate Model

Gate는 `prepare_write`, `close_task`, status display, conformance fixture가 사용하는 canonical kernel field다.

### Scope Gate

```text
not_required | required | pending | passed | failed | blocked
```

`scope_gate`는 모든 write-capable product work에 적용된다. Advisor-only task는 보통 `not_required`를 사용한다. Direct와 work product write는 writing 전에 scoped Change Unit과 passed scope gate가 필요하다.

### Approval Gate

```text
not_required | required | pending | granted | denied | expired
```

`approval_gate`는 sensitive category가 있을 때만 필요하다. Display layer는 approval drift가 없을 때 `granted`의 alias로 `passed`를 보여줄 수 있지만, canonical value는 `granted`다.

### Design Gate

```text
not_required | required | pending | passed | partial | waived | stale | blocked
```

`design_gate`는 필요한 design-quality precondition을 반영한다. 적용 시점과 waiver 허용 시점은 policy가 결정한다.

### Evidence Gate

```text
not_required | none | partial | sufficient | stale | blocked
```

`evidence_gate=not_required`는 evidence gate가 적용되지 않음을 뜻한다.

`evidence_gate=none`은 evidence가 required이지만 아무 evidence도 기록되지 않았음을 뜻한다.

Evidence가 required인 곳에서 successful completion에는 `evidence_gate=sufficient`가 필요하다.

### Verification Gate

```text
not_required | required | pending | passed | failed | waived_by_user | blocked
```

`verification_gate=waived_by_user`는 user가 remaining verification risk를 accepted했음을 기록한다. 이것이 `assurance_level=detached_verified`가 되어서는 안 된다.

### QA Gate

```text
not_required | required | pending | passed | failed | waived
```

`qa_gate`는 required human QA를 위한 canonical kernel gate다. Individual Manual QA record에는 record-level result가 있고, gate는 close와 관련된 aggregate state다.

### Acceptance Gate

```text
not_required | required | pending | accepted | rejected
```

`acceptance_gate`는 acceptance가 required인 곳에서 user's final acceptance judgment를 기록한다. QA나 verification을 대체하지 않는다.

### Capability Boundary

Capability는 의도적으로 kernel gate enum에서 제외한다.

Surface capability는 다음에 속한다.

- `surface_capability_check` validator
- `prepare_write` blocked reason
- guarantee level display

Capability는 kernel이 write를 허용하는지, rule을 얼마나 강하게 enforce할 수 있는지, 어떤 warning을 보여줄지에 영향을 줄 수 있지만 first-class lifecycle gate가 아니다.

## Compatibility Matrix

### Mode Compatibility

| Mode | Product write eligible | Change Unit required for write | Default close assurance | Detached verification |
|---|---:|---:|---|---|
| `advisor` | no | no | `none` | not required |
| `direct` | yes | yes | `self_checked` | optional |
| `work` | yes | yes | `none` until checked | required unless user accepts verification risk |

### Completion Compatibility

| Close path | Required compatible state |
|---|---|
| Advisor completed | no active Run; no product write pending; `result=advice_only`; `close_reason=completed_self_checked` |
| Direct self-checked | no active Run; active Change Unit completed or not needed for non-write direct; scope passed for writes; required approval granted; required evidence sufficient; `assurance_level=self_checked`; `close_reason=completed_self_checked` |
| Direct verified | direct self-checked requirement와 valid passed detached verification 추가; `assurance_level=detached_verified`; `close_reason=completed_verified` |
| Work verified | no active Run; Change Unit complete or explicitly deferred; scope passed; approval not required or granted; design passed or waived; evidence sufficient; verification passed with valid independence; QA passed or waived if required; acceptance accepted if required; `close_reason=completed_verified` |
| Work risk accepted | verification may be `waived_by_user`라는 점을 제외하면 all work verified requirement; assurance는 `none` 또는 `self_checked`; `close_reason=completed_with_risk_accepted` |
| Cancelled | no active write in progress; `result=cancelled`; `close_reason=cancelled` or `superseded` |

### Invalid State Combinations

다음 조합은 invalid이며 kernel이 reject하거나 repair해야 한다.

| Invalid combination | Required handling |
|---|---|
| `lifecycle_phase=completed` with `active_run_id` present | Run이 recorded, interrupted, cancelled될 때까지 close block |
| `lifecycle_phase=completed` with `result=none` | state transition reject |
| `lifecycle_phase=completed` with `close_reason=none` | state transition reject |
| `lifecycle_phase=cancelled` with `result` other than `cancelled` | state transition reject |
| Product write attempted with no active Task | `prepare_write` block |
| Product write attempted in `advisor` mode | `prepare_write` block |
| Product write attempted with no active Change Unit | `prepare_write` block |
| Product write attempted when `scope_gate` is not `passed` | block 또는 scope confirmation 요청 |
| Sensitive change with `approval_gate=not_required` | approval required로 mark하고 block 또는 approval 요청 |
| Sensitive change with approval denied, expired, or outside approved scope | `prepare_write` block |
| Required evidence with `evidence_gate=not_required` | `none`, `partial`, `sufficient`, `stale`, `blocked`로 repair |
| `evidence_gate=none` while evidence records support required criteria | evidence gate recompute |
| Completed passed result where required evidence is `none`, `partial`, `stale`, or `blocked` | close block |
| `verification_gate=waived_by_user` with `assurance_level=detached_verified` | state transition reject |
| Same-session review producing `assurance_level=detached_verified` | assurance upgrade reject |
| Eval verdict passed without valid independence producing `detached_verified` | assurance upgrade reject |
| `qa_gate=waived` without waiver reason | waiver reject |
| Completed passed result with required `qa_gate=pending` or `failed` | close block |
| Completed passed result with required `acceptance_gate=pending` or `rejected` | close block |
| Projection stale or failed recorded as state failure by itself | display/projection status repair; projection freshness만으로 result change 금지 |
| A Markdown projection used as canonical state | reconcile item 생성 또는 state mutation reject |
| A capability field introduced as a canonical lifecycle gate | schema/state mutation reject |

### Close Eligibility

`close_ready`는 `lifecycle_phase`가 아니다. Task에 open Run이 없고 close와 관련된 모든 required gate가 requested close intent와 compatible하다는 derived condition이다. `close_task`만 Task를 `lifecycle_phase=completed`로 옮긴다.

## Transition Table

State transition은 current state change와 같은 transaction 안에서 `state.sqlite.task_events`에 event를 append한다.

| Trigger | From | To | Gate or record effect |
|---|---|---|---|
| User request is accepted | no active Task | `lifecycle_phase=intake`, `result=none` | Task 생성 |
| Request classified as advisor | `intake` | `mode=advisor`, `lifecycle_phase=executing` | product write disabled |
| Request classified as direct | `intake` | `mode=direct`, `lifecycle_phase=ready` | write가 예상되면 scoped Change Unit 생성 또는 선택 |
| Request classified as work | `intake` | `mode=work`, `lifecycle_phase=shaping` | design 및 scope shaping 시작 |
| Shaping finds blocking user decision | `shaping` | `waiting_user` | decision request 기록 |
| User decision resolves shaping blocker | `waiting_user` | `shaping` 또는 `ready` | decision event 기록 |
| Change Unit scope is confirmed | `shaping` 또는 `waiting_user` | `ready` | `scope_gate=passed` |
| Scope is missing for intended write | any non-terminal phase | `waiting_user` 또는 `blocked` | `scope_gate=pending` 또는 `blocked` |
| Sensitive approval requested | any non-terminal phase | `waiting_user` | `approval_gate=pending` |
| Sensitive approval granted | `waiting_user` | previous runnable phase | `approval_gate=granted` |
| Sensitive approval denied | `waiting_user` | `blocked` | `approval_gate=denied` |
| Approval scope drifts or expires | any non-terminal phase | `waiting_user` 또는 `blocked` | `approval_gate=expired` |
| `prepare_write` allows write | `ready` 또는 `executing` | `executing` | active Run may proceed |
| `prepare_write` blocks write | any non-terminal phase | `waiting_user` 또는 `blocked` | blocked reason 기록 |
| Direct implementation and self-check recorded | `executing` | same phase with close eligibility 또는 `waiting_user` | Run, artifact, evidence 기록 |
| Work implementation recorded | `executing` | `verifying` | evidence manifest updated |
| Evidence required but absent | `executing` 또는 `verifying` | `blocked` | `evidence_gate=none` 또는 `partial` |
| Evidence becomes stale | any non-terminal phase | `blocked` 또는 current phase with stale gate | `evidence_gate=stale` |
| Verification launched | `verifying` | `verifying` | evaluator Run 또는 bundle 기록 |
| Eval passed with valid independence | `verifying` | `qa`, `waiting_user`, 또는 same phase with close eligibility | `verification_gate=passed`; assurance may become `detached_verified` |
| Eval passed without valid independence | `verifying` | `verifying` 또는 `blocked` | detached assurance upgrade 없음 |
| Eval failed | `verifying` | `executing`, `shaping`, 또는 `blocked` | `verification_gate=failed` |
| User accepts verification risk | `waiting_user` 또는 `verifying` | same phase with close eligibility | `verification_gate=waived_by_user`; detached assurance 없음 |
| Manual QA requested | any non-terminal phase | `qa` 또는 `waiting_user` | `qa_gate=pending` |
| Manual QA passed | `qa` 또는 `waiting_user` | same phase with close eligibility 또는 `waiting_user` | `qa_gate=passed` |
| Manual QA failed | `qa` 또는 `waiting_user` | `executing`, `shaping`, 또는 `blocked` | `qa_gate=failed` |
| QA waiver accepted | `waiting_user` | same phase with close eligibility | `qa_gate=waived`; waiver reason required |
| Acceptance requested | any non-terminal phase with close eligibility | `waiting_user` | `acceptance_gate=pending` |
| Acceptance accepted | `waiting_user` | same phase with close eligibility | `acceptance_gate=accepted` |
| Acceptance rejected | `waiting_user` | `shaping`, `executing`, 또는 `cancelled` | `acceptance_gate=rejected` |
| `close_task` succeeds | any non-terminal phase with close eligibility | `completed` | result와 close reason assigned |
| User cancels Task | any non-terminal phase | `cancelled` | `result=cancelled`; `close_reason=cancelled` |
| Task is superseded | any non-terminal phase | `cancelled` | `result=cancelled`; `close_reason=superseded` |
| Projection refresh fails | any phase | same lifecycle phase | projection status marked stale or failed; state result unchanged |

## Waiver Semantics

Waiver는 reason, actor, time, affected gate와 함께 기록해야 하는 explicit user 또는 policy decision이다.

허용되는 waiver:

- Policy가 design-quality waiver를 허용할 때 `design_gate=waived`.
- User가 remaining verification risk를 accepted할 때 `verification_gate=waived_by_user`.
- Required QA가 reason과 함께 waived될 때 `qa_gate=waived`.

허용되지 않는 waiver:

- Product write에 대한 scope waiver.
- Sensitive change에 대한 approval waiver.
- Completion에 evidence가 required인 경우의 evidence waiver.
- Acceptance가 required인 경우의 acceptance waiver.

Verification waiver는 detached verification이 아니다. Verification waiver로 close된 task는 `close_reason=completed_with_risk_accepted`와 `assurance_level=none` 또는 `self_checked`를 사용한다.

## `prepare_write` State Logic

`prepare_write`는 product-write decision point다. 다음 state-level decision 중 하나를 반환한다.

```text
allowed | blocked | approval_required | state_conflict
```

Decision algorithm:

1. State version expectation을 확인한다. Caller가 stale state에 대해 act하면 `state_conflict`를 반환한다.
2. Active Task를 resolve한다. 없으면 `blocked`를 반환한다.
3. Task mode가 write-eligible인지 확인한다. `advisor` mode는 product write를 block한다.
4. Active Change Unit을 resolve한다. Intended write 범위를 지정하는 active Change Unit이 없으면 `blocked`를 반환한다.
5. Intended path, tool, command, network target, secret access를 Change Unit과 대조한다. Scope gap은 `blocked`를 반환하거나 scope confirmation을 요구한다.
6. Baseline freshness를 확인한다. Baseline이 stale이면 `blocked`를 반환하고 applicable한 dependent approval 또는 evidence를 stale로 mark한다.
7. Sensitive category를 결정한다. Sensitive category가 있고 matching approval이 granted가 아니면 `approval_required`를 반환한다.
8. Approval scope를 validate한다. Denied, expired, drifted, insufficient approval은 새 approval로 해결 가능한지에 따라 `blocked` 또는 `approval_required`를 반환한다.
9. Writing 전에 적용되는 design-policy precondition check를 실행한다. Required unmet design precondition은 policy에 따라 `blocked`를 반환하거나 decision을 요청한다.
10. Surface capability check를 실행한다. Capability failure는 validator result, blocked reason, guarantee display change로 기록되며 capability를 first-class kernel gate로 만들지 않는다.
11. 모든 required check가 통과하면 decision을 기록하고 `allowed`를 반환한다.

Required check에는 active Task, active Change Unit, mode write eligibility, baseline freshness, intended path, intended tool, intended command, network target, secret access, sensitive category, approval scope, surface capability profile, design policy precondition이 포함된다.

MCP가 cooperative-only surface에서 unavailable이면 product write는 instruction으로 held되어야 한다. 더 강한 guard나 isolation layer가 있다면 같은 decision을 preventively 또는 isolation으로 enforce할 수 있다.

## `close_task` State Logic

`close_task`는 단일 completion decision point다. Agent report, Eval report, QA note, acceptance message는 input을 제공할 수 있지만 그 자체로 Task를 close하지 않는다.

Decision algorithm:

1. Active Task와 requested close intent를 resolve한다.
2. Intent가 cancellation 또는 supersession이면 unsafe in-progress write state가 없는지 확인한 뒤 `lifecycle_phase=cancelled`, `result=cancelled`, matching close reason을 설정한다.
3. Active Run이 아직 open이면 completion을 reject한다.
4. Active Change Unit을 확인한다. Write-capable Task에는 policy에 따라 active Change Unit이 completed, explicitly deferred, superseded되어야 한다.
5. `scope_gate`를 확인한다. Product write에는 passed scope가 필요하다.
6. `approval_gate`를 확인한다. Sensitive change에는 drift나 expiry가 없는 granted approval이 필요하다.
7. `design_gate`를 확인한다. Required design gate는 passed이거나 validly waived되어야 한다. Stale, blocked, pending, partial required design gate는 policy가 recorded waiver로 convert하지 않는 한 close를 block한다.
8. `evidence_gate`를 확인한다. Evidence가 required인 곳에서는 `sufficient`만 successful close가 가능하다.
9. `verification_gate`를 확인한다. Work에는 passed detached verification 또는 explicit user verification waiver가 필요하다. Direct work는 기본적으로 not required지만 optional passed detached verification은 assurance를 upgrade할 수 있다. Same-session review는 detached assurance를 만들 수 없다.
10. `qa_gate`를 확인한다. Required QA는 passed이거나 validly waived되어야 한다. Manual QA record result만으로는 kernel이 `qa_gate`에 aggregate하지 않는 한 gate가 close되지 않는다.
11. `acceptance_gate`를 확인한다. Required acceptance는 accepted여야 한다. Rejection은 Task를 shaping, execution, cancellation로 되돌린다.
12. `assurance_level`, `result`, `close_reason`을 할당한다.
    - advisor completion: `result=advice_only`, `assurance_level=none`, `close_reason=completed_self_checked`
    - direct self-check: `result=passed`, `assurance_level=self_checked`, `close_reason=completed_self_checked`
    - detached verified completion: `result=passed`, `assurance_level=detached_verified`, `close_reason=completed_verified`
    - verification risk accepted: `result=passed`, `assurance_level=none` 또는 `self_checked`, `close_reason=completed_with_risk_accepted`
13. Projection freshness를 report한다. Projection stale 또는 failed status는 user와 export에 표시되지만 그 자체로 Task를 failed로 만들지 않는다.
14. Current record를 update하고, close event를 append하고, projection refresh를 enqueue한다.

## Close Semantics

`completed_verified`는 detached verification이 실제로 passed했고 independence qualifier가 valid하다는 뜻이다.

`completed_self_checked`는 result가 implementing path에서 checked되었거나 detached verification이 필요하지 않았음을 뜻한다.

`completed_with_risk_accepted`는 user가 remaining verification risk를 accepted했음을 뜻한다. 이는 explicit risk가 있는 successful close이지 detached verification이 아니다.

`cancelled`는 Task가 passed result 없이 중단되었음을 뜻한다.

`superseded`는 다른 Task나 Change Unit이 이것을 대체한다는 뜻이다. Supersession이 success를 의미하지 않는다.

## Invariant Enforcement Mapping

| Core invariant | Kernel enforcement points |
|---|---|
| Chat is not state. | State-changing action은 state record와 `task_events`를 생성한다. Projection과 chat text는 MCP action 또는 reconcile 없이 state를 mutate할 수 없다. |
| Product write requires an active scoped Change Unit. | `prepare_write`는 active Task, active Change Unit, passed scope gate가 없는 write-capable action을 block한다. |
| Sensitive change requires explicit approval. | `prepare_write`는 sensitive category를 detect하고 approval gate와 approval scope를 확인하며 denied, expired, missing, drifted approval을 block한다. |
| Completion requires evidence coverage where evidence is required. | `close_task`는 evidence가 적용될 때 `evidence_gate=sufficient`를 요구한다. Required evidence는 passed completion을 위해 waived될 수 없다. |
| Work cannot self-certify detached verification. | `detached_verified`에는 Eval과 valid independence가 필요하다. Same-session review와 verification waiver는 assurance를 upgrade할 수 없다. |
| Required QA and acceptance are separate gates. | `qa_gate`와 `acceptance_gate`는 독립적으로 checked된다. Manual QA record는 acceptance를 imply하지 않고, acceptance도 QA를 imply하지 않는다. |
| Projection cannot override canonical state. | Projection edit는 reconcile item을 생성한다. Projection freshness는 display와 delivery에 영향을 줄 뿐 canonical result 자체에는 영향을 주지 않는다. |
