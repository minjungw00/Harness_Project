# 용어집

## 공식 용어

### Acceptance

결과와 남은 trade-off가 acceptable하다는 사용자의 판단. Acceptance는 approval, assurance, verification, Manual QA와 구분된다.

### Acceptance Gate

Required user acceptance가 not required, required, pending, accepted, rejected 중 어떤 상태인지 기록하는 kernel gate. Acceptance는 QA나 verification을 대신할 수 없다.

### Approval

정의된 scope 안에서 sensitive change를 진행할 수 있도록 허용하는 사전 user decision. Approval은 path, tool, command 또는 command class, network target, secret scope, baseline, sensitive category, expiry condition에 묶인다.

### Approval Gate

Sensitive-change approval을 위한 kernel gate. Sensitive category가 있을 때만 required다. Granted approval은 correctness를 prove하거나 acceptance를 imply하지 않는다.

### Artifact

Evidence, recovery, audit에 사용되는 recorded output. Canonical evidence-file boundary는 Raw Artifact를 참고한다.

### Artifact Reference

Artifact store에 registered된 raw artifact file을 가리키는 structured pointer. Identity, kind, URI 또는 path, hash, size, content type, redaction state, task/run relationship을 포함한다.

### Assurance

Recorded check와 verification independence가 뒷받침하는 technical confidence level.

```text
none | self_checked | detached_verified
```

Eval verdict alone does not upgrade assurance. `detached_verified`에는 valid independence가 있는 passed verification과 same-session self-review violation 없음이 필요하다.

### Baseline

Scope, approval drift, evidence freshness, verification validity를 판단하는 데 사용하는 captured repository state.

### Capability Profile

Connected agent surface가 실제로 무엇을 할 수 있는지에 대한 declared and verified description. Target profile, support tier, guarantee level, supported feature, risk, fallback, last verification time을 기록한다. 하네스는 product name만으로 capability를 infer하지 않는다.

### Capability Tier

Connected surface를 위한 coarse integration level.

```text
T0 Context | T1 Skill | T2 MCP | T3 Capture |
T4 Guard | T5 Isolation | T6 QA Capture
```

Capability tier는 available integration support를 설명하며 kernel gate가 아니다.

### Change Unit

Product write를 위한 scoped implementation unit. Product write에는 intended path, tool, command, network target, sensitive category를 cover하는 active Change Unit이 필요하다.

### Close Reason

Task가 terminal close state에 도달한 canonical reason.

```text
none | completed_verified | completed_self_checked |
completed_with_risk_accepted | cancelled | superseded
```

### Common Tool Envelope

Public MCP tool call이 가진 shared field: `request_id`, `idempotency_key`, `expected_state_version`, `project_id`, optional `task_id`, `surface_id`, optional `run_id`, `actor_kind`, `dry_run`.

### Cooperative Guarantee

Agent surface가 harness instruction과 MCP decision을 따를 것으로 기대되는 guarantee level. Harness는 behavior를 guide할 수 있지만 surface가 hard pre-execution enforcement를 제공하지 않을 수 있다.

### Connector Manifest

Connector-managed file, managed block hash, capability profile, surface target profile, drift status를 기록하는 generated manifest. Generated surface file이 조용히 overwrite되는 것을 막는다.

### Context Hygiene

Current state, evidence, relevant reference는 context에 유지하고, stale chat, old PRD, closed issue, oversized raw artifact는 명시적으로 필요할 때만 가져오는 policy.

### Design Gate

Shared design, domain language, TDD trace, module/interface review 또는 기타 policy-pack requirement 같은 required design-quality precondition을 위한 kernel gate.

### Design-Quality Policy Pack

Shared design, domain language, vertical slice, TDD trace, module/interface review, Manual QA, context hygiene를 위한 policy contract 모음. Design, QA, evidence, close blocker에 영향을 주지만 kernel state machine을 재정의하지 않는다.

### Detached Verification

Fresh session, fresh worktree, sandbox, manual evaluator bundle 같은 meaningful independence boundary를 가로질러 수행되는 verification. Same-session self-review는 detached verification이 아니며, subagent context도 기본적으로 detached가 아니다.

### Detective Guarantee

Harness가 violation을 detect하고 observation 후 state를 blocked, stale, partial, failed로 mark할 수 있는 guarantee level.

### Direct

Scope와 result가 명확한 작고 low-risk인 change를 위한 work mode. Direct product write에도 active scoped Change Unit이 필요하다.

### Domain Language

Product의 canonical vocabulary와 meaning. Canonical source는 `domain_terms`이고 Markdown domain-language document는 projection이자 proposal surface다.

### Domain Term

Product term, meaning, code representation, related term, source, status, "not this" 같은 boundary를 저장하는 `domain_terms`의 canonical structured record.

### Evidence

Diff, log, test, run summary, screenshot, Eval record, Manual QA record처럼 work에 대한 claim을 뒷받침하는 recorded support.

### Evidence Gate

Required evidence coverage를 위한 kernel gate.

```text
not_required | none | partial | sufficient | stale | blocked
```

`not_required`는 evidence gate가 적용되지 않음을 뜻한다. `none`은 evidence가 required이지만 evidence가 기록되지 않았음을 뜻한다.

### Evidence Manifest

Acceptance criteria 또는 completion condition을 supporting evidence reference에 mapping하는 state record.

### Evidence Profile

`advisor`, `direct docs-only`, `direct code`, `work feature`, `UI/UX/copy work`, `sensitive work`, `verification-required work` 같은 named evidence sufficiency profile. Task shape에 충분한 evidence가 무엇인지 validator에 알려준다.

### Evidence Sufficiency

Required acceptance criteria 또는 completion condition이 Evidence Manifest와 관련 state record 및 artifact ref로 support되는지에 대한 close-relevant judgment. Chat text나 Markdown report prose만으로 판단하지 않는다.

### Eval

Verdict, performed check, reviewed evidence, independence qualifier, blocker, artifact reference가 있는 verification result record.

### Fresh Session

Evaluator가 lead chat context를 이어받지 않고 task/evidence bundle에서 시작해 Evidence Manifest와 changed files를 검토하고 Eval을 기록하는 verification independence profile.

### Fresh Worktree

Evaluator가 별도 worktree 또는 동등하게 isolated repository state에서 baseline, changed paths, artifacts, Evidence Manifest를 확인하는 verification independence profile.

### Gate

Task가 write, proceed, close할 수 있는지 control하는 canonical kernel field. Gate는 state이며 display text가 아니다.

### Generated File

Connector, projector, operator tool이 produced한 repository file 또는 managed block. Generated file이 canonical state에서 drift될 수 있으면 manifest 또는 projection job으로 track해야 한다.

### Guarantee Display

Status 또는 write decision에 대한 actual guarantee level의 user-facing 및 connector-facing display. Enforcement가 cooperative 또는 detective인 경우 limitation note를 포함한다.

### Guarantee Level

Connected surface 또는 runtime path에서 available한 enforcement strength.

```text
cooperative | detective | preventive | isolated
```

Capability는 validator result, blocked reason, display에 영향을 주지만 kernel gate는 아니다.

### Harness Core

State transition, gate update, validator interpretation, artifact registration, projection job enqueue, close decision을 담당하는 runtime component.

### Harness Runtime Home

`registry.sqlite`, per-project `project.yaml`, per-project `state.sqlite`, artifact directory를 포함하는 local runtime storage area.

### Human-editable 영역

사람이 note, proposal, question, correction을 쓸 수 있는 Markdown area. Input surface이지 canonical state가 아니다. Authority path는 `human-editable input -> reconcile_items -> accepted state event/record`다.

### Isolated Guarantee

Risky work가 worktree, sandbox, process boundary 또는 동등한 isolation mechanism으로 분리되는 guarantee level.

### Interface Contract

Module 또는 external boundary의 public interface, input, output, error, compatibility impact, caller, boundary test에 대한 canonical record. Canonical source는 `interface_contracts`다.

### Manual QA

UX, workflow, copy, visual output, accessibility, product fit 같은 experiential product quality에 대한 human inspection.

### Manual Bundle

Human 또는 separate evaluator에게 verification을 handoff하는 package. Task summary, acceptance criteria, Change Unit scope, approval scope, diff/log/test artifacts, Evidence Manifest, known risks, Eval verdict를 기록하기에 충분한 context를 포함한다.

### Manual QA Record

Performer, profile, result, artifact, finding, applicable한 경우 waiver reason, next action을 포함하는 record-level Manual QA result. `qa_gate`에 feed되지만 그 자체가 canonical gate는 아니다.

### Managed Block

Harness marker로 delimit되고 projector가 state record와 artifact ref에서 regenerate하는 Markdown block. Managed block에 대한 direct edit는 drift 또는 reconcile candidate를 만들며, 그 자체로 state가 되지 않는다.

### MCP Resource

Current project, task, design, policy, status, bundle information을 위한 read-only MCP surface. Resource는 state를 mutate하지 않는다.

### MCP Tool

Core에 state를 validate, record, transition, close하도록 요청하는 public MCP operation. State change는 resource read가 아니라 tool 또는 reconcile action을 통해야 한다.

### Markdown Report

State record와 artifact reference에서 generated된 human-readable document. Markdown report는 기본적으로 raw artifact가 아니며 canonical state가 되지 않는다.

### Module Map

Product의 module, responsibility, public interface, dependency direction, test boundary map. Canonical source는 `module_map_items`다.

### Module Map Item

Module role, public interface, dependency, internal complexity, test boundary, owner decision, watchpoint를 저장하는 `module_map_items`의 canonical structured record.

### Policy Contract

Design-quality policy가 사용하는 standard form: `name`, `applies_when`, `default_requirement`, `allowed_waiver`, `required_record`, `validator`, `evidence`, `close_impact`.

### Preventive Guarantee

Harness 또는 connector가 violating action을 execution 전에 block할 수 있는 guarantee level.

### Projection

Canonical state record와 artifact reference를 사람이 읽을 수 있게 rendering한 것. Projection은 reading과 decision-making에 유용하지만 canonical state를 override할 수 없다.

### Projection Freshness

Projection과 source record, managed hash, artifact ref, projection job state 사이의 관계. Freshness는 `current`, `stale`, `failed`, `unknown`일 수 있다.

### Projection Job

Committed state record와 artifact ref에서 Markdown projection을 render하도록 projector에 요청하는 durable outbox record.

### QA Gate

Required Manual QA를 위한 canonical kernel gate. `manual_qa_record.result`는 record-level이고, `qa_gate`는 close-relevant aggregate state다.

### Raw Artifact

Diff, log, bundle, screenshot, checkpoint, manifest file처럼 artifact store에 있는 durable evidence file. Raw artifact는 state record와 Markdown report와 구분된다.

### Reconcile

Human-editable input 또는 projection drift를 accepted state change, rejected proposal, note, decision, deferred item으로 바꾸는 process.

### Reconcile Item

Reconcile decision이 accept, reject, convert, defer하기 전에 human-editable input 또는 projection drift에서 생성되는 canonical candidate record.

### Reference Surface

MVP implementation이 target하는 단일 agent surface. Broad MVP surface support를 imply하지 않고 kernel과 connector contract를 demonstrate한다.

### Report Projection

Task report, approval report, run summary, evidence manifest report, Eval report, direct-result report처럼 state record와 artifact reference에서 generated되는 Markdown report.

Named report projection kind는 기본적으로 projection 또는 record다. Evidence-file authority는 registered artifact file에 남는다.

### Risk Accepted Close

사용자가 남은 verification risk를 수용한 successful close. `close_reason=completed_with_risk_accepted`를 사용하며 `assurance_level=detached_verified`로 표시하면 안 된다.

### Run

Agent, evaluator, operator, 기타 actor가 Task와 optional Change Unit에 대해 수행하는 execution attempt. Run은 baseline, surface, observed change, command, artifact, summary를 기록한다.

### Scope Gate

Product write가 active scoped Change Unit으로 covered되어야 함을 요구하는 kernel gate. Approval이 required가 아니어도 write-capable direct와 work mode에는 scope가 required다.

### Shared Design

Task에 대한 shared understanding을 담은 design-quality record 또는 projection: goal, scope, non-goal, acceptance criteria, assumption, decision, rejected option, domain impact, module/interface impact, first Change Unit shape.

### Source-of-truth

어떤 fact에 대한 authoritative source. 하네스에서 operational state는 `state.sqlite` current record와 `state.sqlite.task_events`에서 canonical하고, raw evidence는 artifact store에서 canonical하며, Markdown document는 projection이다.

### `state.sqlite.task_events`

`state.sqlite` 안의 append-only event history table. MVP는 별도의 event store를 사용하지 않는다.

### State Record

Task, Change Unit, Run, Approval, Evidence Manifest, Eval, Manual QA record, Artifact record, Reconcile Item 같은 kernel state의 canonical structured record.

### Surface Capability Check

Connected agent surface가 required harness behavior를 satisfy할 수 있는지 report하는 validator. Blocked reason과 guarantee display에 영향을 주지만 kernel gate는 아니다.

### Surface Cookbook

Surface-specific connector note, generated file detail, profile example을 담은 appendix. Common integration rule은 cookbook이 아니라 agent integration document에 둔다.

### Subagent Context

Subagent 또는 helper가 일부 inherited implementation context를 가지고 work를 review하는 verification independence profile. 기본적으로 detached가 아니며, stricter profile metadata가 real independence boundary를 prove할 때만 qualify될 수 있다.

### Task

Kernel이 track하는 user value unit. Mode, lifecycle phase, gate, result, close reason, assurance, current summary, decision, evidence, projection status를 가진다.

### TDD Trace

Change Unit에 대한 red, green, refactor evidence record 또는 policy가 허용하는 recorded non-TDD justification.

### Verification

Result가 relevant criteria를 satisfy하는지 check하는 process. Verification은 approval, Manual QA, acceptance와 구분된다.

### Verification Gate

Required verification을 위한 kernel gate. User waiver는 `verification_gate=waived_by_user`를 set하며 `detached_verified` assurance를 만들지 않는다.

### Verification Independence Profile

`same_session`, `subagent_context`, `fresh_session`, `fresh_worktree`, `sandbox`, `manual_bundle` 같은 Eval independence context의 named minimum qualification. Passed Eval은 `detached_verified`를 뒷받침하기 전에 valid profile을 만족해야 한다.

### Validator Result

Status, guarantee level, target, finding, blocked reason, suggested next action을 포함하는 validator의 structured result.

### Vertical Slice

Trigger/input에서 domain logic, persistence 또는 state, caller/API boundary, observable output, test, optional Manual QA까지 얇은 경로를 연결하는 Change Unit shape.

### Waiver

Policy가 허용하는 곳에서 gate requirement에 대한 explicit recorded exception. Verification waiver, design waiver, QA waiver는 정의된 rule 아래 허용된다. Scope, sensitive approval, required evidence, required acceptance는 successful completion을 위해 waived되지 않는다.
