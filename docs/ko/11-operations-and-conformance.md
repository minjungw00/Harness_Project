# 운영과 Conformance

## 문서 역할

이 문서는 하네스의 operator procedure와 fixture-based conformance를 담당한다. Connect, doctor, serve MCP, projection refresh, reconcile, recover, export, artifact integrity, conformance suite를 포함한다.

Daily user workflow, MCP request/response schema, SQLite DDL, long-term analytics를 MVP requirement로 다루는 것은 담당하지 않는다.

## Operations Scope

모든 operator entrypoint는 agent가 사용하는 같은 Core rule 위의 surface다. Operator tool은 diagnose, repair, export, fixture 실행을 할 수 있지만, 두 번째 state model을 만들면 안 된다.

Required MVP operator entrypoint:

```text
harness connect
harness doctor
harness serve mcp
harness projection refresh
harness reconcile
harness recover
harness export
harness artifacts check
harness conformance run
```

정확한 command flag는 구현마다 달라질 수 있지만, reference MVP에는 아래 semantics가 필요하다.

## Connect

`connect`는 Product Repository, Harness Runtime Home, 하나의 reference agent surface를 연결한다.

Required behavior:

- repository root identify
- local project register 또는 reuse
- static project configuration create 또는 validate
- per-project state와 artifact storage initialize
- reference surface와 capability profile register
- manifest를 통해 connector-managed file create 또는 refresh
- MCP configuration이 harness server에 reach할 수 있는지 confirm
- conformance smoke check를 run하거나 실행 command print

Connect는 human edit를 조용히 overwrite하지 않고 generated-file drift를 report해야 한다. Surface-specific generated file name은 surface cookbook에 둔다.

## Doctor

`doctor`는 readiness, drift, repair option을 report한다.

Required category:

| Category | Checks |
|---|---|
| project | registered project, repo root, static config validity |
| state | current state readability, locks, active Task consistency |
| MCP | server reachability, read resource availability, public tool availability |
| surface | capability profile, generated manifest, MCP config freshness |
| artifacts | file existence, hash, size, redaction state, task/run relation |
| projections | queued jobs, freshness, managed hash drift, failed renders |
| reconcile | pending human edits, managed block drift, generated-file drift |
| validators | required core, artifact, projection, connector, and policy validators |

Output level:

```text
OK
WARN
FAIL
REPAIRABLE
MANUAL
```

Doctor는 current state failure와 projection stale 또는 projection failed status를 구분해야 한다.

## Serve MCP

`serve mcp`는 local MCP server를 시작하거나 connection information을 print한다.

Required behavior:

- mutation 없이 read resource expose
- shell shortcut이 아니라 Core를 통해 public tool expose
- state-changing call에 Core conflict와 idempotency behavior 요구
- active project와 connected surface profile report
- server가 runtime state 또는 artifact storage에 reach할 수 없으면 명확히 fail

MCP가 unavailable이면 cooperative surface는 product write를 hold해야 한다. Stronger profile은 hold를 preventively 또는 isolation으로 enforce할 수 있지만, operations는 실제 guarantee level을 report해야 한다.

## Projection Refresh

Projection refresh는 committed state record와 artifact ref에서 Product Repository Markdown을 regenerate한다.

Required behavior:

- target의 latest projection version만 render
- human-editable section preserve
- overwrite 전에 managed block hash compare
- managed-block drift에는 reconcile item 생성
- projection job을 `completed`, `failed`, `pending`, `skipped`로 mark
- projection failure를 Task result와 분리

Supported target:

```text
one Task
all active Tasks
approval/run/evidence/eval/direct reports for a Task
design-quality projections when enabled
```

## Reconcile

Reconcile은 human-editable input 또는 generated/managed drift를 explicit decision으로 바꾼다.

Target:

- Task user notes and proposals
- managed block edits
- Domain Language proposals
- Module Map proposals
- Interface Contract proposals
- connector generated-file drift
- stale projection references that affect current work

Decision outcome:

| Outcome | Meaning |
|---|---|
| merge | Core를 통해 proposal을 apply하고 state history append |
| reject | canonical state를 unchanged로 두고 필요하면 projection refresh |
| convert_to_note | content를 state가 아닌 human note로 keep |
| create_decision | proposal을 pending user decision으로 전환 |
| defer | reconcile item을 open 상태로 유지 |

Reconcile은 edited Markdown 자체를 canonical state로 취급하면 안 된다.

## Recover

Recover는 history를 rewrite하지 않고 interrupted 또는 inconsistent operational state를 repair한다.

Required scenario:

| Scenario | Recovery behavior |
|---|---|
| agent crash during write | run을 interrupted로 mark하고 가능하면 diff/log artifact capture |
| stale approval baseline | scope가 affected되면 approval expire 또는 re-request |
| evaluator observes drift | verification blocked 또는 evidence stale로 mark |
| artifact registry mismatch | file rescan, missing artifact를 stale로 mark, hash preserve |
| projection job failed | retry 또는 failed로 mark하고 reconcile guidance 생성 |
| managed Markdown edited | reconcile item 생성 |
| lock expired | recovery event append 후 lock policy에 따라 release 또는 reacquire |
| MCP unavailable | write hold와 next diagnosis step report |

Recovery는 compensating event를 append할 수 있다. Evidence를 조용히 delete하거나, event history를 rewrite하거나, projection을 authoritative하게 만들면 안 된다.

## Export

Export는 Task에 대한 review 또는 archival bundle을 만든다.

Required contents:

- created time, task id, projection freshness, redaction summary가 있는 export manifest
- Task와 related record의 state snapshot
- relevant report의 projection snapshot
- artifact reference와 허용되는 경우 included raw artifact file
- artifact integrity manifest
- secret, sensitive log, PII에 대한 redaction 및 omission note

Exported projection snapshot은 hash를 가질 수 있지만, 그렇다고 Markdown projection이 canonical evidence가 되지는 않는다. Raw evidence는 artifact file과 registered ref로 남는다.

## Artifact Integrity

Artifact integrity check는 artifact record와 stored file을 비교한다.

Required check:

- file exists
- hash matches
- size matches
- content type이 known이거나 명시적으로 `other`
- redaction state가 valid
- task/run relation이 valid
- retention class가 valid
- projection 또는 evidence ref가 resolve됨

Failure는 Core rule에 따라 related evidence, projection freshness, close readiness를 stale/blocked로 mark해야 한다. Missing artifact는 Markdown report를 edit해서 고치는 것이 아니다.

## Conformance Fixture Format

Conformance는 fixture-based다. Scenario table만으로는 충분하지 않다. 각 test fixture는 action을 drive하고 state, event, artifact, projection, error를 assert해야 한다.

각 fixture는 이 shape를 포함해야 한다.

```yaml
scenario_id: string
initial_state: object
input: object
action: string
expected_state: object
expected_events: list
expected_artifacts: list
expected_projection: object
expected_error: object | null
```

`name`, `suite`, `tags`, `notes` 같은 optional metadata는 허용되지만, 위 required field는 반드시 있어야 한다.

## Conformance Execution

`harness conformance run`은 MCP tool과 operator command가 사용하는 같은 Core entrypoint를 통해 fixture를 실행한다. Prose output만 inspect해서 behavior를 assert하면 안 된다.

MVP execution semantic:

1. Fixture YAML file을 load하고 required fixture shape를 validate한다.
2. Fixture가 existing read-only sample을 명시적으로 target하지 않는 한 isolated runtime home과 temporary Product Repository를 만든다.
3. `initial_state`에서 `registry.sqlite`, `project.yaml`, `state.sqlite`, artifact file, projection file, connector manifest를 seed한다.
4. Core를 통해 `action`을 execute한다. MCP tool action은 public request schema를 사용한다. `projection_refresh`, `doctor_surface`, `recover`, `artifacts_check` 같은 operator action은 이 문서의 operator semantics를 사용한다.
5. Resulting state summary, appended `task_events`, validator result, artifact registry/file integrity, projection job status, reconcile item, returned error code를 capture한다.
6. Captured result를 `expected_state`, `expected_events`, `expected_artifacts`, `expected_projection`, `expected_error`와 compare한다.
7. Fixture id, pass/fail, observed state summary, observed event, artifact integrity result, projection freshness, error comparison을 report한다.

Fixture execution은 deterministic해야 한다. Network access, wall-clock-sensitive expiry, external tool output은 suite가 integration smoke라고 명시적으로 선언하지 않는 한 stub하거나 seeded fixture input으로 표현해야 한다.

## Hardened MVP Fixture Coverage

Hardened evidence, verification, connector rule은 required shape를 가진 fixture로 cover해야 한다. 각 fixture는 해당 behavior가 구현되어야 하는 가장 이른 MVP stage에 map한다.

```yaml
scenario_id: CORE-evidence-direct-docs-only-sufficient
mvp_stage: MVP-4
initial_state:
  active_task:
    mode: direct
    lifecycle_phase: executing
    acceptance_criteria: ["AC-01 typo corrected"]
    gates:
      scope_gate: passed
      evidence_gate: partial
      verification_gate: not_required
input:
  evidence_profile: direct docs-only
  changed_paths: ["docs/help.md"]
  diff_artifact: ART-DIFF-001
  self_check_summary: "Rendered Markdown heading and checked typo fix."
action: close_task
expected_state:
  lifecycle_phase: completed
  result: passed
  close_reason: completed_self_checked
  assurance_level: self_checked
  gates:
    evidence_gate: sufficient
expected_events:
  - evidence_manifest_updated
  - close_requested
  - task_closed
expected_artifacts:
  - artifact_id: ART-DIFF-001
    kind: diff
expected_projection:
  TASK: enqueued
  EVIDENCE-MANIFEST: enqueued
expected_error: null
```

```yaml
scenario_id: CORE-evidence-work-ac-missing-blocks-close
mvp_stage: MVP-4
initial_state:
  active_task:
    mode: work
    lifecycle_phase: verifying
    acceptance_criteria: ["AC-01 saves profile", "AC-02 shows validation error"]
    gates:
      scope_gate: passed
      approval_gate: not_required
      evidence_gate: partial
      verification_gate: pending
input:
  evidence_profile: work feature
  criteria:
    AC-01:
      status: supported
      refs: [ART-TEST-001]
    AC-02:
      status: unsupported
      refs: []
action: close_task
expected_state:
  lifecycle_phase: blocked
  gates:
    evidence_gate: partial
expected_events:
  - close_requested
  - close_blocked
expected_artifacts:
  - artifact_id: ART-TEST-001
    kind: log
expected_projection:
  TASK: enqueued
  EVIDENCE-MANIFEST: enqueued
expected_error:
  code: EVIDENCE_INSUFFICIENT
```

```yaml
scenario_id: CORE-evidence-ui-manual-qa-pending-blocks-close
mvp_stage: MVP-4
initial_state:
  active_task:
    mode: work
    lifecycle_phase: qa
    acceptance_criteria: ["AC-01 button copy updated"]
    gates:
      scope_gate: passed
      evidence_gate: sufficient
      verification_gate: passed
      qa_gate: pending
input:
  evidence_profile: UI/UX/copy work
  manual_qa_record: null
action: close_task
expected_state:
  lifecycle_phase: qa
  gates:
    qa_gate: pending
expected_events:
  - close_requested
  - close_blocked
expected_artifacts: []
expected_projection:
  TASK: enqueued
expected_error:
  code: QA_REQUIRED
```

```yaml
scenario_id: CORE-verify-manual-bundle-detached-passed
mvp_stage: MVP-4
initial_state:
  active_task:
    mode: work
    lifecycle_phase: verifying
    gates:
      evidence_gate: sufficient
      verification_gate: pending
input:
  eval:
    verdict: passed
    independence_context:
      profile: manual_bundle
      reviewed_bundle_ref: ART-BUNDLE-001
      received_task_summary: true
      received_acceptance_criteria: true
      received_change_unit_scope: true
      received_approval_scope: true
      received_diff_log_test_artifacts: true
      received_evidence_manifest: true
      received_known_risks: true
    evidence_reviewed: [ART-DIFF-001, ART-TEST-001, EVIDENCE-MANIFEST-001]
action: record_eval
expected_state:
  lifecycle_phase: verifying
  assurance_level: detached_verified
  gates:
    verification_gate: passed
expected_events:
  - eval_recorded
  - verification_passed
expected_artifacts:
  - artifact_id: ART-BUNDLE-001
    kind: bundle
expected_projection:
  EVAL: enqueued
  TASK: enqueued
expected_error: null
```

```yaml
scenario_id: CORE-verify-subagent-context-not-detached-by-default
mvp_stage: MVP-4
initial_state:
  active_task:
    mode: work
    lifecycle_phase: verifying
    gates:
      verification_gate: pending
input:
  eval:
    verdict: passed
    independence_context:
      profile: subagent_context
      stricter_profile_satisfied: false
    evidence_reviewed: [EVIDENCE-MANIFEST-001]
action: record_eval
expected_state:
  lifecycle_phase: verifying
  assurance_level: none
  gates:
    verification_gate: pending
expected_events:
  - eval_recorded
  - verify_not_detached_detected
expected_artifacts: []
expected_projection:
  EVAL: enqueued
  TASK: enqueued
expected_error:
  code: VERIFY_NOT_DETACHED
```

```yaml
scenario_id: CORE-verify-waiver-risk-accepted-not-detached
mvp_stage: MVP-4
initial_state:
  active_task:
    mode: work
    lifecycle_phase: waiting_user
    assurance_level: self_checked
    gates:
      scope_gate: passed
      evidence_gate: sufficient
      verification_gate: waived_by_user
      qa_gate: not_required
      acceptance_gate: accepted
input:
  close_intent: accept_verification_risk
  waiver_reason: "User accepts remaining verification risk for urgent local-only fix."
action: close_task
expected_state:
  lifecycle_phase: completed
  result: passed
  close_reason: completed_with_risk_accepted
  assurance_level: self_checked
expected_events:
  - close_requested
  - risk_accepted_close_recorded
  - task_closed
expected_artifacts: []
expected_projection:
  TASK: enqueued
expected_error: null
```

```yaml
scenario_id: CONN-cooperative-guarantee-display
mvp_stage: MVP-2
initial_state:
  surface:
    surface_id: SURF-0001
    guarantee_level: cooperative
    changed_path_detection: validator
  active_task:
    mode: direct
    lifecycle_phase: ready
input:
  include:
    guarantees: true
action: status
expected_state:
  guarantee_display:
    level: cooperative
    notes:
      - "This surface is expected to follow Harness decisions, but Harness may not physically block an out-of-scope write before it happens. Changed-path validation can detect violations afterward."
expected_events: []
expected_artifacts: []
expected_projection: {}
expected_error: null
```

```yaml
scenario_id: CONN-mcp-unavailable-write-hold
mvp_stage: MVP-5
initial_state:
  surface:
    guarantee_level: cooperative
    mcp_available: false
  active_task:
    mode: direct
    lifecycle_phase: ready
input:
  intended_paths: ["src/profile/ProfileForm.tsx"]
action: connector_prepare_write_attempt
expected_state:
  lifecycle_phase: blocked
  write_held: true
expected_events:
  - mcp_unavailable_detected
expected_artifacts: []
expected_projection:
  TASK: enqueued
expected_error:
  code: MCP_UNAVAILABLE
```

## Core Fixture Examples

```yaml
scenario_id: CORE-prepare-write-no-change-unit
initial_state:
  active_task:
    mode: work
    lifecycle_phase: ready
    active_change_unit: null
input:
  intended_paths: ["src/auth/login.ts"]
  sensitive_categories: []
action: prepare_write
expected_state:
  lifecycle_phase: blocked
  gates:
    scope_gate: blocked
expected_events:
  - prepare_write_blocked
expected_artifacts: []
expected_projection:
  TASK: stale_or_enqueued
expected_error:
  code: NO_ACTIVE_CHANGE_UNIT
```

```yaml
scenario_id: CORE-same-session-verify-not-detached
initial_state:
  active_task:
    mode: work
    lifecycle_phase: verifying
    verification_gate: pending
input:
  eval:
    verdict: passed
    independence_context: same_session
action: record_eval
expected_state:
  assurance_level: none
  gates:
    verification_gate: pending
expected_events:
  - eval_recorded
  - verify_not_detached_detected
expected_artifacts: []
expected_projection:
  EVAL: enqueued
  TASK: enqueued
expected_error:
  code: VERIFY_NOT_DETACHED
```

```yaml
scenario_id: CORE-projection-failure-state-current
initial_state:
  active_task:
    mode: direct
    lifecycle_phase: completed
    result: passed
    projection_status: current
input:
  projection_kind: TASK
  render_error: permission_denied
action: projection_refresh
expected_state:
  lifecycle_phase: completed
  result: passed
  projection_status: failed
expected_events:
  - projection_refresh_failed
expected_artifacts: []
expected_projection:
  TASK: failed
expected_error:
  code: PROJECTION_STALE
```

## Connector Fixture Examples

```yaml
scenario_id: CONN-generated-file-drift-reconcile
initial_state:
  connector_manifest:
    status: current
input:
  changed_generated_path: ".harness/agent/generated/rules.md"
action: doctor_surface
expected_state:
  reconcile_required: true
expected_events:
  - generated_file_drift_detected
  - reconcile_item_created
expected_artifacts: []
expected_projection: {}
expected_error:
  code: RECONCILE_REQUIRED
```

## Design-Quality Fixture Examples

```yaml
scenario_id: DESIGN-horizontal-feature-without-exception
initial_state:
  active_task:
    mode: work
    lifecycle_phase: shaping
input:
  change_unit:
    slice_type: horizontal-exception
    horizontal_exception_reason: null
action: validate_design_policy
expected_state:
  gates:
    design_gate: partial
expected_events:
  - design_validator_failed
expected_artifacts: []
expected_projection:
  TASK: enqueued
expected_error:
  code: VALIDATOR_FAILED
```

```yaml
scenario_id: DESIGN-manual-qa-required-missing
initial_state:
  active_task:
    mode: work
    lifecycle_phase: qa
    qa_gate: pending
input:
  changed_surface: ui
  manual_qa_record: null
action: close_task
expected_state:
  lifecycle_phase: qa
  gates:
    qa_gate: pending
expected_events:
  - close_requested
  - close_blocked
expected_artifacts: []
expected_projection:
  TASK: enqueued
expected_error:
  code: QA_REQUIRED
```

## Fixture Suites

Minimum MVP suite:

- core: active status, advisor close, direct close, write gate, approval required, evidence insufficient, same-session verification guard, QA required, acceptance required, projection failure separation
- connector: capability profile, MCP unavailable hold, generated manifest drift, changed-path detection, artifact capture, fallback guarantee display
- design-quality: shared design required, vertical slice or exception, TDD trace required or waived, module/interface review, Manual QA policy, context hygiene stale projection

Conformance output은 fixture id, pass/fail, observed state summary, observed event, artifact integrity result, projection freshness, error code comparison을 포함해야 한다.

## Metrics Boundary

Long-term operational metric은 derived analytics이지 MVP-critical state나 conformance requirement가 아니다. Approval turnaround, verification latency, projection stale duration, same-session guard frequency, surface fallback rate 같은 metric은 future version이 fixture와 implementation ownership으로 승격하기 전까지 [Appendix C](appendix/C-later-roadmap.md)에 둔다.
