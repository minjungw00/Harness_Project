# Operations And Conformance

## Document Role

This document owns operator procedures and fixture-based conformance for the harness: connect, doctor, serve MCP, projection refresh, reconcile, recover, export, artifact integrity, and conformance suites.

It does not own daily user workflow, MCP request/response schemas, SQLite DDL, or long-term analytics as MVP requirements.

## Operations Scope

Every operator entrypoint is a surface over the same Core rules used by the agent. Operator tools may diagnose, repair, export, or run fixtures, but they must not create a second state model.

Required MVP operator entrypoints:

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

Exact command flags may vary by implementation, but the semantics below are required for the reference MVP.

## Connect

`connect` links a Product Repository, Harness Runtime Home, and one reference agent surface.

Required behavior:

- identify the repository root
- register or reuse the local project
- create or validate static project configuration
- initialize per-project state and artifact storage
- register the reference surface and capability profile
- create or refresh connector-managed files through a manifest
- confirm MCP configuration can reach the harness server
- run a conformance smoke check or print the command to run it

Connect must report generated-file drift instead of overwriting human edits silently. Surface-specific generated file names belong in the surface cookbook.

## Doctor

`doctor` reports readiness, drift, and repair options.

Required categories:

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

Output levels:

```text
OK
WARN
FAIL
REPAIRABLE
MANUAL
```

Doctor must distinguish current state failures from projection stale or projection failed status.

## Serve MCP

`serve mcp` starts or prints connection information for the local MCP server.

Required behavior:

- expose read resources without mutation
- expose public tools through Core, not shell shortcuts
- require state-changing calls to use Core conflict and idempotency behavior
- report the active project and connected surface profile
- fail clearly when the server cannot reach runtime state or artifact storage

If MCP is unavailable, cooperative surfaces must hold product writes. Stronger profiles may enforce the hold preventively or through isolation, but operations must still report the actual guarantee level.

## Projection Refresh

Projection refresh regenerates Product Repository Markdown from committed state records and artifact refs.

Required behavior:

- render only the latest projection version for a target
- preserve human-editable sections
- compare managed block hashes before overwrite
- create reconcile items for managed-block drift
- mark projection jobs `completed`, `failed`, `pending`, or `skipped`
- keep projection failure separate from Task result

Supported targets:

```text
one Task
all active Tasks
approval/run/evidence/eval/direct reports for a Task
design-quality projections when enabled
```

## Reconcile

Reconcile turns human-editable input or generated/managed drift into an explicit decision.

Targets:

- Task user notes and proposals
- managed block edits
- Domain Language proposals
- Module Map proposals
- Interface Contract proposals
- connector generated-file drift
- stale projection references that affect current work

Decision outcomes:

| Outcome | Meaning |
|---|---|
| merge | apply the proposal through Core and append state history |
| reject | leave canonical state unchanged and refresh projection if needed |
| convert_to_note | keep the content as a human note, not state |
| create_decision | turn the proposal into a pending user decision |
| defer | keep the reconcile item open |

Reconcile must not treat edited Markdown as canonical state by itself.

## Recover

Recover repairs interrupted or inconsistent operational state without rewriting history.

Required scenarios:

| Scenario | Recovery behavior |
|---|---|
| agent crash during write | mark the run interrupted and capture diff/log artifacts when possible |
| stale approval baseline | expire or re-request approval when scope is affected |
| evaluator observes drift | mark verification blocked or evidence stale |
| artifact registry mismatch | rescan files, mark missing artifacts stale, preserve hashes |
| projection job failed | retry or mark failed and create reconcile guidance |
| managed Markdown edited | create reconcile item |
| lock expired | append recovery event and release or reacquire according to lock policy |
| MCP unavailable | report write hold and next diagnosis step |

Recovery may append compensating events. It must not silently delete evidence, rewrite event history, or make projections authoritative.

## Export

Export creates a review or archival bundle for a Task.

Required contents:

- export manifest with created time, task id, projection freshness, and redaction summary
- state snapshots for the Task and related records
- projection snapshots for relevant reports
- artifact references and included raw artifact files when allowed
- artifact integrity manifest
- redaction and omission notes for secrets, sensitive logs, and PII

Exported projection snapshots may have hashes, but that does not make the Markdown projection the canonical evidence. Raw evidence remains the artifact files and their registered refs.

## Artifact Integrity

Artifact integrity check compares artifact records with stored files.

Required checks:

- file exists
- hash matches
- size matches
- content type is known or explicitly `other`
- redaction state is valid
- task/run relation is valid
- retention class is valid
- projection or evidence refs resolve

Failures should mark related evidence, projection freshness, or close readiness stale/blocked according to Core rules. Missing artifacts are not fixed by editing Markdown reports.

## Conformance Fixture Format

Conformance is fixture-based. A scenario table is not enough; each test fixture must drive an action and assert state, events, artifacts, projections, and errors.

Each fixture must include this shape:

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

Optional metadata such as `name`, `suite`, `tags`, and `notes` is allowed, but the required fields above must be present.

## Conformance Execution

`harness conformance run` executes fixtures through the same Core entrypoints used by MCP tools and operator commands. It must not assert behavior by inspecting prose output alone.

MVP execution semantics:

1. Load fixture YAML files and validate the required fixture shape.
2. Create an isolated runtime home and temporary Product Repository for the fixture, unless the fixture explicitly targets an existing read-only sample.
3. Seed `registry.sqlite`, `project.yaml`, `state.sqlite`, artifact files, projection files, and connector manifests from `initial_state`.
4. Execute `action` through Core. MCP tool actions use the public request schema; operator actions such as `projection_refresh`, `doctor_surface`, `recover`, and `artifacts_check` use the operator semantics in this document.
5. Capture resulting state summaries, appended `task_events`, validator results, artifact registry/file integrity, projection job status, reconcile items, and returned error code.
6. Compare the captured results with `expected_state`, `expected_events`, `expected_artifacts`, `expected_projection`, and `expected_error`.
7. Report fixture id, pass/fail, observed state summary, observed events, artifact integrity result, projection freshness, and error comparison.

Fixture execution should be deterministic. Network access, wall-clock-sensitive expiry, and external tool output must be stubbed or represented as seeded fixture inputs unless a suite explicitly declares itself an integration smoke.

## Hardened MVP Fixture Coverage

The hardened evidence, verification, and connector rules should be covered by fixtures with the required shape. Each fixture maps to the earliest MVP stage where the behavior must be implemented.

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

Minimum MVP suites:

- core: active status, advisor close, direct close, write gate, approval required, evidence insufficient, same-session verification guard, QA required, acceptance required, projection failure separation
- connector: capability profile, MCP unavailable hold, generated manifest drift, changed-path detection, artifact capture, fallback guarantee display
- design-quality: shared design required, vertical slice or exception, TDD trace required or waived, module/interface review, Manual QA policy, context hygiene stale projection

Conformance output must include fixture id, pass/fail, observed state summary, observed events, artifact integrity result, projection freshness, and error code comparison.

## Metrics Boundary

Long-term operational metrics are derived analytics, not MVP-critical state or conformance requirements. Keep metrics such as approval turnaround, verification latency, projection stale duration, same-session guard frequency, and surface fallback rate in [Appendix C](appendix/C-later-roadmap.md) until a future version promotes them with fixtures and implementation ownership.
