# Kernel Spec

## Document Role

This document owns the operational kernel specification for the harness. It defines the entity model, lifecycle model, gates, state compatibility rules, transition table, close semantics, waiver semantics, `prepare_write` state logic, `close_task` state logic, and invariant enforcement mapping.

This document does not define MCP wire schemas, SQLite DDL, projection template text, design-quality playbook procedures, connector capability schemas, or capability as a first-class kernel gate.

## Kernel Scope

The kernel is the canonical state machine for local AI-assisted product work. It decides:

- which Task is active
- which Change Unit scopes product writes
- whether a write may proceed
- which approvals, evidence, verification, QA, and acceptance gates apply
- whether a task may close
- which state events are appended
- which projections need refresh

Operational state is canonical in `state.sqlite` current records plus `state.sqlite.task_events`.
Raw evidence is canonical in the artifact store. Markdown reports are projections generated from state records and artifact refs. Human-editable sections are input surfaces.

The kernel records references to raw evidence and projections, but neither chat text nor generated Markdown replaces canonical state.

## Work Modes

`advisor` is for read-only explanation, comparison, review, and decision support. It does not authorize product writes. Advisor tasks usually close with `result=advice_only`; evidence, verification, QA, and acceptance gates are normally not required unless policy or the user explicitly requires them.

`direct` is for small, low-risk product changes whose scope and result are obvious. It is write-capable, so product writes still require an active scoped Change Unit. Direct work may close as `self_checked` by default. If optional detached verification is performed and passes with a valid independence qualifier, direct work may be marked `detached_verified`.

`work` is for structured implementation, non-local change, riskier change, or work that needs independent verification. It is write-capable, requires an active scoped Change Unit before product writes, and cannot be marked `detached_verified` by same-session self-review.

## Entity Model

### Task

A Task is the user value unit. It carries the current mode, lifecycle phase, result, close reason, assurance level, gate states, current summary, acceptance criteria, pending decisions, active Change Unit, active Run, latest record references, and projection freshness. A Task is the primary state record used by status, resume, and close decisions.

### Change Unit

A Change Unit is the scoped implementation unit for product writes. It records purpose, non-goals, slice type, intended end-to-end path, allowed paths, allowed tools, validator profile, sensitive categories, approval needs, evidence expectations, QA expectations, dependencies, merge risk, completion conditions, and evaluator focus.

Every product write requires an active Change Unit whose scope covers the intended write. A Task may have one or many Change Units, but only the active Change Unit authorizes the current write.

### Run

A Run is an execution attempt by a lead agent, evaluator, operator, or other actor. It records actor identity, surface identity, mode, Change Unit, baseline, intended operation, observed changes, command results, artifact references, and summary. A lead Run may shape or implement. An evaluator Run verifies from a separate verification boundary and is not allowed to become detached verification unless its independence qualifier is valid.

### Approval

An Approval is a scope-bound prior decision for sensitive change. It records what was approved: paths, tools, commands or command classes, network targets, secret scope, baseline, sensitive categories, expiry conditions, and user decision. Approval does not prove correctness, replace evidence, satisfy QA, or imply acceptance.

### Evidence Manifest

An Evidence Manifest maps acceptance criteria or completion conditions to evidence references. It records whether each criterion is supported, unsupported, or not applicable, and it references durable artifacts, run summaries, Eval records, TDD traces, Manual QA records, or other recorded evidence. Evidence sufficiency is judged from this manifest and related records.

### Eval

An Eval is a verification result record. It records the verification target, verdict, checks performed, evidence reviewed, independence qualifier, baseline relationship, blockers, and artifact references. An Eval verdict alone does not upgrade assurance. `assurance_level=detached_verified` requires a passed verification result, a valid independence qualifier, and no same-session self-review violation.

### Manual QA

Manual QA is a human inspection record for UX, workflow, copy, accessibility, visual output, product taste, or any other result that needs human judgment. `qa_gate` is the canonical kernel gate. `manual_qa_record.result` is the result of an individual record that can feed the gate.

### Artifact

An Artifact is a durable evidence file in the artifact store, such as a diff, log, bundle, manifest, screenshot, checkpoint, or exported bundle component. Artifact records identify and verify these files by reference and integrity metadata. Raw artifacts are distinct from Markdown reports and state records.

### Reconcile Item

A Reconcile Item is the canonical candidate record created when human-editable content or generated projection drift may need to affect state. Reconcile decisions may merge, reject, convert to note, create a decision, or defer the item. Human-editable text is input; accepted state changes occur only through reconcile action and state events.

### Design Support Records

The kernel also owns the entity meaning for design support records:

- Shared Design records capture goals, scope, assumptions, rejected options, acceptance criteria, and decisions.
- Domain Term records are the canonical source for Domain Language.
- Module Map Item records are the canonical source for Module Map.
- Interface Contract records are the canonical source for Interface Contract.
- TDD Trace records capture red, green, refactor evidence or a recorded non-TDD justification.

Their policy requirements are owned by the design-quality policy pack. Their storage DDL is owned by the reference MVP document.

## Authority Rules

User Notes authority is:

```text
human-editable input -> reconcile_items -> accepted state event/record
```

Domain Language canonical source is `domain_terms`.

Module Map canonical source is `module_map_items`.

Interface Contract canonical source is `interface_contracts`.

The `DOMAIN-LANGUAGE`, `MODULE-MAP`, and `INTERFACE-CONTRACT` Markdown documents are projections and proposal surfaces. They do not override their canonical records.

## Lifecycle Model

The kernel uses lifecycle fields plus gates. Compact display states are derived from these canonical fields.

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

Assurance is not approval, QA, or acceptance. It summarizes the technical checking level supported by runs, evidence, Eval records, and verification independence.

## Gate Model

Gates are canonical kernel fields used by `prepare_write`, `close_task`, status display, and conformance fixtures.

### Scope Gate

```text
not_required | required | pending | passed | failed | blocked
```

`scope_gate` applies to all write-capable product work. Advisor-only tasks normally use `not_required`. Direct and work product writes require a scoped Change Unit and a passed scope gate before writing.

### Approval Gate

```text
not_required | required | pending | granted | denied | expired
```

`approval_gate` is required only when sensitive categories are present. A display layer may show `passed` as an alias for `granted` when no approval drift exists, but the canonical value remains `granted`.

### Design Gate

```text
not_required | required | pending | passed | partial | waived | stale | blocked
```

`design_gate` reflects required design-quality preconditions. Policy determines when it applies and when a waiver is allowed.

### Evidence Gate

```text
not_required | none | partial | sufficient | stale | blocked
```

`evidence_gate=not_required` means evidence gate does not apply.

`evidence_gate=none` means evidence is required but no evidence has been recorded.

Where evidence is required, a successful completion requires `evidence_gate=sufficient`.

### Evidence Sufficiency Profiles

Evidence sufficiency is judged from the Evidence Manifest plus related state records and artifact refs. It must not be judged from chat text or report prose alone. A status card or Markdown report may summarize why evidence is missing, but the close decision uses the manifest, Task, gates, Change Units, Runs, approvals, Evals, Manual QA records, baseline relation, and registered artifacts.

| Evidence Profile | Minimum sufficiency guidance |
|---|---|
| `advisor` | `evidence_gate` is usually `not_required` unless the user or policy asks for a recorded decision, review bundle, or exportable artifact. |
| `direct docs-only` | Sufficient evidence may be changed path list, diff artifact or recorded patch summary, and self-check summary. |
| `direct code` | Sufficient evidence may be changed path list, diff artifact, relevant command/test/log artifact or explicit reason no automated check applies, and self-check summary. |
| `work feature` | Sufficient evidence requires acceptance-criteria-to-evidence mapping, changed file coverage, run summary, diff/log/test/build artifacts as applicable, and `evidence_manifest.status=sufficient`. |
| `UI/UX/copy work` | Requires `work feature` evidence plus Manual QA record or valid QA waiver when QA is required. |
| `sensitive work` | Requires normal task evidence plus approval ref, approval scope compatibility, baseline relation, and no approval drift. |
| `verification-required work` | Requires Evidence Manifest plus Eval record with reviewed evidence and valid independence if the task is to close as `completed_verified`. |

Close impact:

- Required evidence absent means `evidence_gate=none`.
- Required evidence incomplete means `evidence_gate=partial`.
- Evidence invalidated by baseline, changed files, approval drift, missing artifact, or relevant design record change means `evidence_gate=stale` or `blocked`.
- Successful close where evidence is required needs `evidence_gate=sufficient`.
- `evidence_gate=not_required` must not be used when evidence is required but missing.

Examples:

- Direct typo fix: changed path `docs/help.md`, diff artifact or patch summary, and self-check summary can support `direct docs-only` evidence.
- Work feature: AC-01 maps to passing test log and changed path coverage; AC-02 maps to build log plus run summary; the Evidence Manifest records both as supported.
- UI copy change: changed copy path, diff artifact, self-check, and required Manual QA record support close; until Manual QA is recorded or validly waived, close remains blocked.

### Verification Gate

```text
not_required | required | pending | passed | failed | waived_by_user | blocked
```

`verification_gate=waived_by_user` records that the user accepted remaining verification risk. It must not become `assurance_level=detached_verified`.

### Verification Independence Profiles

Verification independence profiles describe the minimum qualification needed before an Eval can support detached assurance.

| Profile | Minimum qualification |
|---|---|
| `same_session` | Not detached. May record self-check or review notes. Must not produce `detached_verified`. |
| `subagent_context` | Not detached by default. May qualify only if the implementation context, write authority, and reviewed bundle satisfy a stricter profile; otherwise treat as not detached. |
| `fresh_session` | Detached candidate if the evaluator receives a task/evidence bundle rather than continuing lead chat context, reviews the Evidence Manifest and changed files, and records an Eval. |
| `fresh_worktree` | Detached candidate if the evaluator checks baseline, changed paths, artifacts, and Evidence Manifest in a separate worktree or equivalent isolated repository state. |
| `sandbox` | Detached or isolated candidate if execution and verification happen across a meaningful process/filesystem boundary and artifacts are captured. |
| `manual_bundle` | Detached candidate if the evaluator receives task summary, acceptance criteria, Change Unit scope, approval scope, diff/log/test artifacts, Evidence Manifest, known risks, and records a verdict. |

Rules:

- Eval verdict alone does not upgrade assurance.
- Valid independence plus passed verification plus absence of a same-session self-review violation is required for `assurance_level=detached_verified`.
- User verification waiver must close as `completed_with_risk_accepted`, not `completed_verified`.
- A verifier that can write product files must disclose that in Eval independence context; write capability may reduce confidence and may require an additional guard or bundle review.

### QA Gate

```text
not_required | required | pending | passed | failed | waived
```

`qa_gate` is the canonical kernel gate for required human QA. Individual Manual QA records have record-level results; the gate is the aggregate close-relevant state.

### Acceptance Gate

```text
not_required | required | pending | accepted | rejected
```

`acceptance_gate` records the user's final acceptance judgment where acceptance is required. It does not replace QA or verification.

### Capability Boundary

Capability is deliberately excluded from the kernel gate enum.

Surface capability belongs to:

- the `surface_capability_check` validator
- `prepare_write` blocked reasons
- guarantee level display

Capability can affect whether the kernel allows a write, how strongly it can enforce the rule, and what warning is shown, but it is not a first-class lifecycle gate.

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
| Direct verified | direct self-checked requirements plus valid passed detached verification; `assurance_level=detached_verified`; `close_reason=completed_verified` |
| Work verified | no active Run; Change Unit complete or explicitly deferred; scope passed; approval not required or granted; design passed or waived; evidence sufficient; verification passed with valid independence; QA passed or waived if required; acceptance accepted if required; `close_reason=completed_verified` |
| Work risk accepted | all work verified requirements except verification may be `waived_by_user`; assurance must be `none` or `self_checked`; `close_reason=completed_with_risk_accepted` |
| Cancelled | no active write in progress; `result=cancelled`; `close_reason=cancelled` or `superseded` |

### Invalid State Combinations

The following combinations are invalid and must be rejected or repaired by the kernel:

| Invalid combination | Required handling |
|---|---|
| `lifecycle_phase=completed` with `active_run_id` present | block close until the Run is recorded, interrupted, or cancelled |
| `lifecycle_phase=completed` with `result=none` | reject state transition |
| `lifecycle_phase=completed` with `close_reason=none` | reject state transition |
| `lifecycle_phase=cancelled` with `result` other than `cancelled` | reject state transition |
| Product write attempted with no active Task | block `prepare_write` |
| Product write attempted in `advisor` mode | block `prepare_write` |
| Product write attempted with no active Change Unit | block `prepare_write` |
| Product write attempted when `scope_gate` is not `passed` | block or request scope confirmation |
| Sensitive change with `approval_gate=not_required` | mark approval required and block or request approval |
| Sensitive change with approval denied, expired, or outside approved scope | block `prepare_write` |
| Required evidence with `evidence_gate=not_required` | repair to `none`, `partial`, `sufficient`, `stale`, or `blocked` |
| `evidence_gate=none` while evidence records support required criteria | recompute evidence gate |
| Completed passed result where required evidence is `none`, `partial`, `stale`, or `blocked` | block close |
| `verification_gate=waived_by_user` with `assurance_level=detached_verified` | reject state transition |
| Same-session review producing `assurance_level=detached_verified` | reject assurance upgrade |
| Eval verdict passed without valid independence producing `detached_verified` | reject assurance upgrade |
| `qa_gate=waived` without waiver reason | reject waiver |
| Completed passed result with required `qa_gate=pending` or `failed` | block close |
| Completed passed result with required `acceptance_gate=pending` or `rejected` | block close |
| Projection stale or failed recorded as state failure by itself | repair display/projection status; do not change result solely for projection freshness |
| A Markdown projection used as canonical state | create reconcile item or reject as state mutation |
| A capability field introduced as a canonical lifecycle gate | reject schema/state mutation |

### Close Eligibility

`close_ready` is not a `lifecycle_phase`. It is a derived condition meaning that the Task has no open Run and all close-relevant required gates are compatible with the requested close intent. Only `close_task` moves a Task to `lifecycle_phase=completed`.

## Transition Table

State transitions append an event to `state.sqlite.task_events` in the same transaction as current state changes.

| Trigger | From | To | Gate or record effect |
|---|---|---|---|
| User request is accepted | no active Task | `lifecycle_phase=intake`, `result=none` | create Task |
| Request classified as advisor | `intake` | `mode=advisor`, `lifecycle_phase=executing` | product write disabled |
| Request classified as direct | `intake` | `mode=direct`, `lifecycle_phase=ready` | create or select scoped Change Unit if write is expected |
| Request classified as work | `intake` | `mode=work`, `lifecycle_phase=shaping` | design and scope shaping begins |
| Shaping finds blocking user decision | `shaping` | `waiting_user` | decision request recorded |
| User decision resolves shaping blocker | `waiting_user` | `shaping` or `ready` | decision event recorded |
| Change Unit scope is confirmed | `shaping` or `waiting_user` | `ready` | `scope_gate=passed` |
| Scope is missing for intended write | any non-terminal phase | `waiting_user` or `blocked` | `scope_gate=pending` or `blocked` |
| Sensitive approval requested | any non-terminal phase | `waiting_user` | `approval_gate=pending` |
| Sensitive approval granted | `waiting_user` | previous runnable phase | `approval_gate=granted` |
| Sensitive approval denied | `waiting_user` | `blocked` | `approval_gate=denied` |
| Approval scope drifts or expires | any non-terminal phase | `waiting_user` or `blocked` | `approval_gate=expired` |
| `prepare_write` allows write | `ready` or `executing` | `executing` | active Run may proceed |
| `prepare_write` blocks write | any non-terminal phase | `waiting_user` or `blocked` | blocked reason recorded |
| Direct implementation and self-check recorded | `executing` | same phase with close eligibility or `waiting_user` | Run, artifacts, and evidence recorded |
| Work implementation recorded | `executing` | `verifying` | evidence manifest updated |
| Evidence required but absent | `executing` or `verifying` | `blocked` | `evidence_gate=none` or `partial` |
| Evidence becomes stale | any non-terminal phase | `blocked` or current phase with stale gate | `evidence_gate=stale` |
| Verification launched | `verifying` | `verifying` | evaluator Run or bundle recorded |
| Eval passed with valid independence | `verifying` | `qa`, `waiting_user`, or same phase with close eligibility | `verification_gate=passed`; assurance may become `detached_verified` |
| Eval passed without valid independence | `verifying` | `verifying` or `blocked` | no detached assurance upgrade |
| Eval failed | `verifying` | `executing`, `shaping`, or `blocked` | `verification_gate=failed` |
| User accepts verification risk | `waiting_user` or `verifying` | same phase with close eligibility | `verification_gate=waived_by_user`; no detached assurance |
| Manual QA requested | any non-terminal phase | `qa` or `waiting_user` | `qa_gate=pending` |
| Manual QA passed | `qa` or `waiting_user` | same phase with close eligibility or `waiting_user` | `qa_gate=passed` |
| Manual QA failed | `qa` or `waiting_user` | `executing`, `shaping`, or `blocked` | `qa_gate=failed` |
| QA waiver accepted | `waiting_user` | same phase with close eligibility | `qa_gate=waived`; waiver reason required |
| Acceptance requested | any non-terminal phase with close eligibility | `waiting_user` | `acceptance_gate=pending` |
| Acceptance accepted | `waiting_user` | same phase with close eligibility | `acceptance_gate=accepted` |
| Acceptance rejected | `waiting_user` | `shaping`, `executing`, or `cancelled` | `acceptance_gate=rejected` |
| `close_task` succeeds | any non-terminal phase with close eligibility | `completed` | result and close reason assigned |
| User cancels Task | any non-terminal phase | `cancelled` | `result=cancelled`; `close_reason=cancelled` |
| Task is superseded | any non-terminal phase | `cancelled` | `result=cancelled`; `close_reason=superseded` |
| Projection refresh fails | any phase | same lifecycle phase | projection status marked stale or failed; state result unchanged |

## Waiver Semantics

Waivers are explicit user or policy decisions that must be recorded with reason, actor, time, and affected gate.

Allowed waivers:

- `design_gate=waived` when policy allows design-quality waiver.
- `verification_gate=waived_by_user` when the user accepts remaining verification risk.
- `qa_gate=waived` when required QA is waived with reason.

Not allowed:

- Scope waiver for product writes.
- Approval waiver for sensitive changes.
- Evidence waiver where evidence is required for completion.
- Acceptance waiver where acceptance is required.

Verification waiver is not detached verification. A task closed through verification waiver uses `close_reason=completed_with_risk_accepted` and `assurance_level=none` or `self_checked`.

## `prepare_write` State Logic

`prepare_write` is the product-write decision point. It returns one of these state-level decisions:

```text
allowed | blocked | approval_required | state_conflict
```

The decision algorithm is:

1. Check state version expectations. If the caller is acting on stale state, return `state_conflict`.
2. Resolve the active Task. If none exists, return `blocked`.
3. Confirm the Task mode is write-eligible. `advisor` mode blocks product writes.
4. Resolve the active Change Unit. If no active Change Unit scopes the intended write, return `blocked`.
5. Check intended paths, tools, commands, network targets, and secret access against the Change Unit. Scope gaps return `blocked` or require scope confirmation.
6. Check baseline freshness. If the baseline is stale, return `blocked` and mark dependent approvals or evidence stale where applicable.
7. Determine sensitive categories. If sensitive categories exist and no matching approval is granted, return `approval_required`.
8. Validate approval scope. Denied, expired, drifted, or insufficient approval returns `blocked` or `approval_required` depending on whether a new approval can resolve it.
9. Run design-policy precondition checks that apply before writing. Required unmet design preconditions return `blocked` or request a decision according to policy.
10. Run surface capability checks. Capability failures are recorded as validator results, blocked reasons, and guarantee display changes; they do not create capability as a first-class kernel gate.
11. If all required checks pass, record the decision and return `allowed`.

Required checks include active Task, active Change Unit, mode write eligibility, baseline freshness, intended paths, intended tools, intended commands, network targets, secret access, sensitive categories, approval scope, surface capability profile, and design policy preconditions.

If MCP is unavailable on a cooperative-only surface, product writes must be held by instruction. If a stronger guard or isolation layer exists, the same decision may be enforced preventively or by isolation.

## `close_task` State Logic

`close_task` is the single completion decision point. Agent reports, Eval reports, QA notes, and acceptance messages may provide inputs, but they do not close the Task by themselves.

The decision algorithm is:

1. Resolve the active Task and requested close intent.
2. If the intent is cancellation or supersession, ensure no write is in an unsafe in-progress state, then set `lifecycle_phase=cancelled`, `result=cancelled`, and the matching close reason.
3. Reject completion if an active Run is still open.
4. Check the active Change Unit. Write-capable Tasks need the active Change Unit completed, explicitly deferred, or superseded according to policy.
5. Check `scope_gate`. Product writes require passed scope.
6. Check `approval_gate`. Sensitive changes require granted approval with no drift or expiry.
7. Check `design_gate`. Required design gates must be passed or validly waived; stale, blocked, pending, or partial required design gates block close unless policy converts them to a recorded waiver.
8. Check `evidence_gate`. Where evidence is required, only `sufficient` can close successfully.
9. Check `verification_gate`. Work requires passed detached verification or explicit user verification waiver. Direct work defaults to not required, but optional passed detached verification may upgrade assurance. Same-session review cannot produce detached assurance.
10. Check `qa_gate`. Required QA must be passed or validly waived. A Manual QA record result alone does not close the gate unless the kernel aggregates it into `qa_gate`.
11. Check `acceptance_gate`. Required acceptance must be accepted. Rejection routes the Task back to shaping, execution, or cancellation.
12. Assign `assurance_level`, `result`, and `close_reason`:
    - advisor completion: `result=advice_only`, `assurance_level=none`, `close_reason=completed_self_checked`
    - direct self-check: `result=passed`, `assurance_level=self_checked`, `close_reason=completed_self_checked`
    - detached verified completion: `result=passed`, `assurance_level=detached_verified`, `close_reason=completed_verified`
    - verification risk accepted: `result=passed`, `assurance_level=none` or `self_checked`, `close_reason=completed_with_risk_accepted`
13. Report projection freshness. Projection stale or failed status is shown to the user and export, but it does not by itself make the Task failed.
14. Update current records, append a close event, and enqueue projection refresh.

## Close Semantics

`completed_verified` means detached verification actually passed and the independence qualifier is valid.

`completed_self_checked` means the result was checked by the implementing path or did not require detached verification.

`completed_with_risk_accepted` means the user accepted remaining verification risk. This is a successful close with explicit risk, not detached verification.

`cancelled` means the Task stopped without a passed result.

`superseded` means another Task or Change Unit replaces this one. Supersession does not imply success.

## Invariant Enforcement Mapping

| Core invariant | Kernel enforcement points |
|---|---|
| Chat is not state. | State-changing actions create state records and `task_events`; projections and chat text cannot mutate state without MCP action or reconcile. |
| Product write requires an active scoped Change Unit. | `prepare_write` blocks write-capable actions without active Task, active Change Unit, and passed scope gate. |
| Sensitive change requires explicit approval. | `prepare_write` detects sensitive categories, checks approval gate and approval scope, and blocks denied, expired, missing, or drifted approval. |
| Completion requires evidence coverage where evidence is required. | `close_task` requires `evidence_gate=sufficient` when evidence applies; required evidence cannot be waived for passed completion. |
| Work cannot self-certify detached verification. | Eval plus valid independence is required for `detached_verified`; same-session review and verification waiver cannot upgrade assurance. |
| Required QA and acceptance are separate gates. | `qa_gate` and `acceptance_gate` are checked independently; Manual QA records do not imply acceptance, and acceptance does not imply QA. |
| Projection cannot override canonical state. | Projection edits create reconcile items; projection freshness affects display and delivery, not canonical result by itself. |
