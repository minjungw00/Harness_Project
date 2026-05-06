# 설계 품질 정책 팩

## 문서 역할

이 문서는 design-quality policy를 policy contract로 담당한다. 이 정책은 AI 지원 작업이 product design, domain language, module boundary, testing discipline, human QA, context hygiene와 정렬된 상태를 유지하도록 안내한다.

Design-quality policy는 추가 kernel invariant가 아니다. Kernel은 lifecycle, gate, close semantic, state transition을 담당한다. 이 문서는 policy evaluator가 언제 `design_gate`, `qa_gate`, evidence, close blocker에 영향을 주어야 하는지 말한다.

이 문서는 MCP schema, SQLite DDL, state transition table, full template을 정의하지 않는다.

## Policy Contract 형태

각 policy는 동일한 field를 사용한다.

| Field | Meaning |
|---|---|
| `name` | Stable policy name. |
| `applies_when` | Policy가 relevant해지는 condition. |
| `default_requirement` | 적용될 때 기본적으로 일어나야 하는 것. |
| `allowed_waiver` | 누가 waive할 수 있고 무엇을 기록해야 하는지. |
| `required_record` | Result를 저장하는 canonical state record 또는 record family. |
| `validator` | Compliance, warning, failure, blocker를 report하는 validator. |
| `evidence` | Policy가 기대하는 evidence 또는 projection ref. |
| `close_impact` | Unmet requirement가 close 또는 gate에 미치는 영향. |

Policy validator는 MCP API document가 담당하는 validator result schema를 반환한다.

## Policy Contract

### Shared Design

| Field | Contract |
|---|---|
| `name` | `shared_design` |
| `applies_when` | Work request가 ambiguous하거나, scope/non-scope가 unclear하거나, user value alignment가 필요하거나, public interface/schema/auth/UX/workflow가 affected되거나, `work` task가 shaping을 필요로 할 때. |
| `default_requirement` | Goal, scope, non-goal, acceptance criteria, blocking decision, assumption, rejected option, domain-language impact, module/interface impact, first Change Unit shape를 기록한다. 가장 blocking한 question을 한 번에 하나씩 묻고, 첫 safe Change Unit을 propose할 수 있으면 멈춘다. |
| `allowed_waiver` | User/operator가 reason과 design risk가 남을 때 follow-up을 기록하면 small obvious `direct` work, docs-only edit, emergency fix에 허용된다. |
| `required_record` | Shared Design record, Task shaping field, decision record, optional `DESIGN` 또는 `DEC` projection. |
| `validator` | `shared_design_alignment` |
| `evidence` | Task summary, acceptance criteria, decision ref, rejected option ref, domain/module/interface impact ref. |
| `close_impact` | Required인데 없으면 `design_gate=pending` 또는 `partial`로 set/keep한다. Risk가 high이고 waiver가 없으면 close를 block한다. Valid waiver는 `design_gate=waived`를 허용할 수 있다. |

### Domain Language

| Field | Contract |
|---|---|
| `name` | `domain_language` |
| `applies_when` | New product term이 나타나거나, existing term이 new meaning으로 쓰이거나, code와 product language가 diverge하거나, multiple name이 하나의 concept를 가리키거나, reviewer/evaluator가 term mismatch를 발견할 때. |
| `default_requirement` | Affected term의 meaning, code representation, "not this" boundary, related term, source, status를 record/update한다. Implementation agent는 task-relevant term만 pull하고, reviewer/evaluator는 relevant term을 받는다. |
| `allowed_waiver` | Work에 domain term impact가 없거나 term이 intentionally local/temporary일 때 허용된다. Waiver는 canonical term update가 필요 없는 이유를 기록해야 한다. |
| `required_record` | `domain_terms` record; `DOMAIN-LANGUAGE`는 projection only. |
| `validator` | `domain_language_consistency` |
| `evidence` | Domain term ref, code ref, test naming ref, proposal용 reconcile item ref. |
| `close_impact` | Required term이 missing 또는 conflicting이면 `design_gate=partial` 또는 `stale`로 mark한다. Mismatch가 acceptance criteria, public behavior, verification confidence에 영향을 주면 close를 block한다. |

### Vertical Slice

| Field | Contract |
|---|---|
| `name` | `vertical_slice` |
| `applies_when` | Feature work, user-visible behavior, workflow change, integration behavior, medium/large `work` task. |
| `default_requirement` | Trigger/input, domain logic, persistence 또는 state, API/caller boundary, observable output, test evidence, optional Manual QA를 연결하는 thin end-to-end Change Unit을 선호한다. |
| `allowed_waiver` | Scaffold, test harness, deep module boundary, migration safety, public interface decision이 먼저 필요할 때 horizontal/enabling Change Unit을 허용한다. Change Unit은 applicable할 때 `horizontal_exception_reason`과 follow-up vertical Change Unit을 기록해야 한다. |
| `required_record` | Change Unit field: `slice_type`, end-to-end path, completion condition, follow-up vertical Change Unit, validator result. |
| `validator` | `vertical_slice_shape` |
| `evidence` | Change Unit record, run summary, evidence manifest, test, user-visible인 경우 Manual QA ref. |
| `close_impact` | Vertical slice가 required인데 satisfied 또는 waived가 아니면 `design_gate=partial` 또는 `blocked`를 set한다. Justified horizontal exception은 follow-up risk가 recorded된 경우에만 close를 허용할 수 있다. |

### TDD Trace

| Field | Contract |
|---|---|
| `name` | `tdd_trace` |
| `applies_when` | Domain logic, service module, bug fix, parser/validator, state transition, deep module internal, edge-case-heavy behavior. API/caller boundary와 integration behavior에는 권장된다. |
| `default_requirement` | 적어도 하나의 acceptance criterion 또는 behavior slice에 대해 red, green, refactor evidence를 기록한다. Trace를 evidence manifest에 link한다. |
| `allowed_waiver` | Docs, typo, throwaway prototype, exploratory UI prototype, initial scaffold, 또는 user/operator가 non-TDD justification과 alternate feedback loop를 기록한 경우 허용된다. |
| `required_record` | `tdd_traces` record와 rendered될 때 `TDD-TRACE` projection. |
| `validator` | `tdd_trace_required` |
| `evidence` | Failing test log, passing test log, refactor check log, diff ref, waived 시 non-TDD justification. |
| `close_impact` | Required TDD trace가 missing이면 `design_gate=partial`이 되고 evidence가 insufficient해질 수 있다. Valid non-TDD justification은 design policy를 satisfy할 수 있지만 그 자체로 behavior를 증명하지는 않는다. |

### Deep Module / Interface

| Field | Contract |
|---|---|
| `name` | `deep_module_interface` |
| `applies_when` | Public interface change, module boundary change, schema/data model change, auth/security boundary, compatibility impact, deep module internal, shallow-module risk. |
| `default_requirement` | Affected module, current role, proposed public interface, interface 뒤에 숨겨진 internal complexity, impacted caller, compatibility impact, test boundary를 identify한다. 충분한 internal capability를 뒤에 둔 작고 simple한 public interface를 선호한다. |
| `allowed_waiver` | Public boundary impact, dependency direction change, compatibility risk가 없고 localized internal change일 때 허용된다. Module/interface review가 불필요한 이유를 기록해야 한다. |
| `required_record` | `module_map_items`, `interface_contracts`, decision record, optional `MODULE-MAP` / `INTERFACE-CONTRACT` projection. |
| `validator` | `module_interface_review` |
| `evidence` | Module map ref, interface contract ref, caller impact list, boundary test, design decision, compatibility note. |
| `close_impact` | Required review가 missing이면 `design_gate=pending` 또는 `partial`로 남는다. Public interface 또는 compatibility risk가 있는데 review가 없으면 close를 block하거나 residual risk에 대한 user acceptance가 필요할 수 있다. |

### Manual QA

| Field | Contract |
|---|---|
| `name` | `manual_qa` |
| `applies_when` | UI change, UX flow change, copy/error message change, onboarding/checkout/auth/billing 또는 other critical flow, accessibility impact, visual output, browser-only behavior, product taste judgment가 필요한 result. |
| `default_requirement` | Manual QA profile, setup, checklist, result, finding, evidence ref, performer, next action을 기록한다. Profile에는 `ui_quality`, `workflow`, `copy`, `accessibility`, `browser_smoke`, `performance_smoke`가 포함된다. |
| `allowed_waiver` | User/operator가 명시적으로 QA를 waive하고 waiver reason을 기록할 때 허용된다. Legal, safety, privacy, high-impact user harm이 inspection을 요구하는 경우에는 적절하지 않다. |
| `required_record` | `manual_qa_records`; `qa_gate`가 canonical aggregate gate. |
| `validator` | `manual_qa_required` |
| `evidence` | Manual QA record, screenshot, note, browser log, walkthrough ref, finding ref. |
| `close_impact` | Manual QA가 required이면 `qa_gate=pending` 또는 `failed`가 successful close를 block한다. `qa_gate=waived`에는 waiver reason이 필요하다. QA failed는 rework를 만들거나 close를 block하거나 explicit follow-up path를 요구해야 한다. |

### Context Hygiene

| Field | Contract |
|---|---|
| `name` | `context_hygiene` |
| `applies_when` | Work가 interruption 후 resume되거나, old PRD/design doc/issue가 있거나, code path가 moved되었거나, acceptance criteria가 changed되었거나, module/interface/domain record가 changed되었거나, evaluator/reviewer가 focused bundle을 필요로 할 때. |
| `default_requirement` | Current Task summary, rolling spine, latest run/eval/evidence ref, relevant policy ref, current acceptance criteria를 push한다. Older PRD, closed issue, coding standard, long log는 필요할 때만 pull한다. Stale doc을 mark하고 chat을 state로 취급하지 않는다. |
| `allowed_waiver` | Product state, design state, evidence state가 바뀌지 않는 short advisor-only work에 허용된다. |
| `required_record` | Task summary, projection freshness, drift에 대한 reconcile item, evidence manifest, validator result. |
| `validator` | `context_hygiene_check` |
| `evidence` | Current projection ref, freshness state, stale ref, reconcile item ref, evaluator용 bundle contents. |
| `close_impact` | Stale critical context는 `design_gate=stale`, evidence stale, projection stale로 mark될 수 있다. Agent가 scope, evidence, current acceptance criteria를 safe하게 determine할 수 없으면 write 또는 close를 block할 수 있다. |

## Waiver 규칙

Waiver는 explicit, scoped, recorded여야 한다. Waiver에는 다음을 포함해야 한다.

- policy name
- task와 Change Unit
- reason
- accepted risk
- waived한 actor
- 필요할 때 expiry 또는 follow-up
- affected gate 또는 close impact

Policy waiver는 policy contract가 허용하는 곳에서만 design-quality requirement를 satisfy할 수 있다. Product write scope, sensitive-change approval, required evidence coverage, required acceptance를 waive하지 않는다. Verification waiver는 kernel close semantics가 담당하며 `assurance_level=detached_verified`를 만들면 안 된다.

## Policy-To-Validator Mapping

| Policy | Validator | Primary gate or state impact |
|---|---|---|
| `shared_design` | `shared_design_alignment` | `design_gate` pending/partial/passed/waived |
| `domain_language` | `domain_language_consistency` | `design_gate` partial/stale/passed |
| `vertical_slice` | `vertical_slice_shape` | `design_gate` partial/blocked/passed |
| `tdd_trace` | `tdd_trace_required` | `design_gate` and evidence sufficiency |
| `deep_module_interface` | `module_interface_review` | `design_gate` partial/blocked/passed |
| `manual_qa` | `manual_qa_required` | `qa_gate` pending/passed/failed/waived |
| `context_hygiene` | `context_hygiene_check` | projection freshness, reconcile, evidence/design stale |

Reference MVP는 minimal validator를 먼저 구현할 수 있지만, conformance fixture가 policy name을 바꾸지 않고 커질 수 있도록 validator ID는 stable하게 유지해야 한다.
