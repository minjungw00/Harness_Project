# Kernel Decisions

This document records design decisions fixed before rewriting the harness documentation set.

Authors of body documents do not reopen these decisions. If a new decision is needed, mark it as `TODO_DECISION` instead of writing it directly into canonical document body text.

## KD-01. Event Log Physical Location

### Decision

In MVP, the event log is the `state.sqlite.task_events` append-only table, not a separate canonical store.

Recommended wording:

```text
The canonical source for operational state is state.sqlite.
state.sqlite has current state tables and an append-only task_events table.
```

Wording to avoid:

```text
state.sqlite + event log
```

The wording above can read as if a separate event store exists. When needed, write:

```text
state.sqlite current tables + state.sqlite.task_events
```

### Rationale

Keeping operational state and event history inside the same SQLite transaction boundary keeps the MVP implementation small. External event streams remain v1/later extensions.

### Impact

- `03-kernel-spec.md`: define event schema and state transition events
- `04-runtime-architecture.md`: specify event location in the authority flow
- `06-reference-mvp.md`: include `task_events` in SQLite DDL
- `07-document-projection.md`: explain projection freshness and event relation

## KD-02. Three Spaces Stay Fixed

### Decision

The harness continues to separate three spaces.

```text
Product Repository
Harness Server / Installation
Harness Runtime Home
```

### Meaning

```text
Product Repository:
  product code, tests, human-readable projections, human-editable notes

Harness Server / Installation:
  MCP server, Core, validator, connector, projector, CLI

Harness Runtime Home:
  registry.sqlite, project.yaml, state.sqlite, artifacts
```

### Rationale

If product code, harness runtime code, and operational state are mixed, source-of-truth and projection become confused again.

### Impact

- `04-runtime-architecture.md` owns the canonical explanation.
- `00-introduction.md`, `README.md`, and `10-user-guide.md` provide only short summaries.

## KD-03. State Model: Lifecycle + Gates

### Decision

Restructure the state model as `lifecycle + gates`, not a single long list of axes.

### Lifecycle

```yaml
mode: advisor | direct | work
lifecycle_phase:
  intake | shaping | ready | executing | verifying | qa |
  waiting_user | blocked | completed | cancelled
result:
  none | advice_only | passed | failed | cancelled
close_reason:
  none | completed_verified | completed_self_checked |
  completed_with_risk_accepted | cancelled | superseded
```

### Gates

```yaml
scope_gate:
  not_required | required | pending | passed | failed | blocked
approval_gate:
  not_required | required | pending | granted | denied | expired
  # display alias allowed: passed = granted when no drift exists
design_gate:
  not_required | required | pending | passed | partial | waived | stale | blocked
evidence_gate:
  not_required | none | partial | sufficient | stale | blocked
verification_gate:
  not_required | required | pending | passed | failed | waived_by_user | blocked
qa_gate:
  not_required | required | pending | passed | failed | waived
acceptance_gate:
  not_required | required | pending | accepted | rejected
```

### Derived Display

Compact status cards are derived from canonical fields. Display state is not a canonical source.

### Evidence Gate Applicability

```text
not_required:
  evidence gate does not apply, for example advisor-only work

none:
  evidence is required but no evidence has been recorded yet
```

### Rationale

The existing state axes are expressive, but their combination rules are not closed. The gate model makes completion judgment implementable.

## KD-04. Scope Gate and Approval Gate Are Separate

### Decision

Separate the scope gate from the approval gate.

```text
scope_gate:
  applies to every write-capable run.

approval_gate:
  becomes required only when a sensitive change category exists.

capability:
  is not a first-class kernel gate in MVP.
  surface_capability_check validator, prepare_write blocked_reasons,
  and guarantee level display express it.
```

### Rule

Always check the scope gate before writing product files. If a sensitive category exists, also check the approval gate.

Do not add `capability_gate` to the canonical gate list in `03-kernel-spec.md`.

### Examples

```text
Typo fix:
  scope_gate=passed
  approval_gate=not_required

dependency addition:
  scope_gate=passed or pending
  approval_gate=required/pending/granted
```

### Rationale

Existing documents often said "check scope and approval" together, which could mix the two concepts. Every write needs scope, but not every write needs approval.

## KD-05. Verification Waiver Is Not Detached Verification

### Decision

The user may accept a verification exception and close the task. However, a waiver is not displayed as `detached_verified`.

### Required Representation

```yaml
verification_gate: waived_by_user
assurance_level: none | self_checked
close_reason: completed_with_risk_accepted
```

Prohibited representation:

```yaml
verification_gate: waived_by_user
assurance_level: detached_verified
```

### Rationale

Keep the principle that work does not close on the implementer's self-report alone, while honestly representing risk-accepted close in real operations.

### Close Semantics

- `completed_verified`: close where detached verification actually passed
- `completed_with_risk_accepted`: close where the user accepted remaining verification risk
- distinguish these states in user cards and exports.

## KD-06. Direct Work May Be Optionally Verified

### Decision

`direct` work can close as `self_checked` by default. If the user wants it or policy requires it, optional detached verification can be added.

### Rule

```text
direct:
  verification_gate=not_required by default
  assurance_level=self_checked by default

optional direct verify passed:
  verification_gate=passed
  assurance_level=detached_verified allowed
```

### Rationale

Requiring detached verification for every small direct task makes the default experience heavy. But if a direct result was independently verified, there is no reason to hide that fact.

## KD-07. User Notes Authority

### Decision

Represent User Notes as three authority stages.

```text
Input source:
  human-editable document section

Canonical record for reflection candidates:
  state.sqlite.reconcile_items

Operational fact after reflection:
  state.sqlite event + target record
```

### Rule

Human-editable sections are user input surfaces. They do not change Task state by themselves.

### Rationale

Existing documents wavered between `human-editable document area` and `reconcile item` as the canonical source for user notes. The new model separates input surface from operational reflection.

## KD-08. Domain Language Authority

### Decision

The canonical source for Domain Language is `state.sqlite.domain_terms`.

```text
canonical source:
  state.sqlite.domain_terms

Human-readable projection:
  DOMAIN-LANGUAGE

User proposal:
  human-editable section → reconcile_items → domain_terms
```

### Rule

The `DOMAIN-LANGUAGE` document is a human-facing projection. Accepted human edits are reflected into `domain_terms` through reconcile.

### Rationale

The phrase `domain language record + reconciled doc` weakens the source-of-truth principle.

## KD-09. Module and Interface Authority

### Decision

For Module Map and Interface Contract, operational records are also the canonical source.

```text
module map canonical source:
  state.sqlite.module_map_items

interface contract canonical source:
  state.sqlite.interface_contracts

projection:
  MODULE-MAP
  INTERFACE-CONTRACT
```

### Rule

Human-editable proposals in design projections are promoted to records through reconcile.

## KD-10. Core Invariants

### Decision

Limit core invariants to these 7.

```text
1. Chat is not state.
2. Product write requires an active scoped Change Unit.
3. Sensitive change requires explicit approval.
4. Completion requires evidence coverage where evidence is required.
5. Work cannot self-certify detached verification.
6. Required QA and acceptance are separate gates.
7. Projection cannot override canonical state.
```

### Rule

Do not present any other principle as a core invariant in document body text.

### Rationale

The 17 invariants in the existing documents are valuable, but not all are kernel invariants. Kernel invariants include only what would make the harness stop being the harness if broken.

## KD-11. Policy Defaults

### Decision

The following items are design-quality policy defaults, not core invariants.

```text
- shared design for work
- domain language consistency
- vertical slice default
- TDD trace for suitable work
- module/interface review
- manual QA for UI/UX/copy
- context hygiene
```

### Rule

Define policy defaults with applies_when, default_requirement, allowed_waiver, required_record, validator, and close_impact.

### Rationale

These principles matter for design quality, but they need waivers and exceptions depending on task type.

## KD-12. Guarantee Levels

### Decision

Unify guarantee levels into these four stages.

```text
cooperative:
  guarantee that assumes the agent surface follows the procedure

detective:
  guarantee that detects violations and changes state to blocked/stale/partial

preventive:
  guarantee that blocks violations before execution

isolated:
  guarantee that isolates risk in a separate worktree/sandbox/process
```

### Rule

The MVP reference surface is cooperative/detective by default. Claim partial preventive guarantees only for profiles with T4 guards. Claim isolated guarantees only for profiles with T5 isolation.

### Rationale

Remove any implication that product writes can be fully blocked in advance on every surface.

## KD-13. MCP Public Surface

### Decision

Keep public MCP tools, but tighten their schemas.

```text
harness.status
harness.intake
harness.next
harness.prepare_write
harness.record_run
harness.request_user_decision
harness.record_user_decision
harness.launch_verify
harness.record_eval
harness.record_manual_qa
harness.close_task
```

### Rule

Keep tool names as high-level intent. Per-tool schemas, errors, events, validators, and projection jobs are owned by `05-mcp-api-and-schemas.md`.

### Required Disambiguation

```yaml
harness.record_run:
  kind: shaping_update | implementation | direct | verification_input

harness.request_user_decision:
  decision_kind:
    approval | scope_confirmation | design_choice |
    qa_waiver | acceptance | reconcile
```

### Rationale

Prevent over-broad payloads without increasing the number of public tools.

## KD-14. Common Tool Envelope

### Decision

State-changing MCP tools have a common envelope.

```yaml
request_id: string
idempotency_key: string
expected_state_version: integer
project_id: string
task_id: optional string
surface_id: string
run_id: optional string
actor_kind: user | lead_agent | evaluator | operator
dry_run: boolean
```

### Rule

If `expected_state_version` does not match, return `STATE_CONFLICT`. Determine retries by idempotency key.

## KD-15. Projection Template Tiers

### Decision

Divide projection templates into three tiers.

```text
Required MVP:
  TASK
  APR
  RUN-SUMMARY
  EVIDENCE-MANIFEST
  EVAL
  DIRECT-RESULT

Optional design-quality:
  DOMAIN-LANGUAGE
  MODULE-MAP
  INTERFACE-CONTRACT
  TDD-TRACE
  MANUAL-QA

Appendix:
  DEC
  DESIGN
  EXPORT
  full report variants
```

### Rule

The body of `07-document-projection.md` keeps only required MVP templates and operating rules. The full template library is owned by `appendix/A-template-library.md`.

## KD-16. Reference Surface Scope

### Decision

MVP targets only one reference agent surface.

```text
MVP:
  one reference surface
  MCP T2
  minimal artifact capture
  manual verify bundle
  cooperative prepare_write

v1:
  sidecar file watcher
  worktree verify
  second connector

later:
  native hooks
  browser QA capture
  cross-surface verify
```

### Rationale

The success criterion for MVP is kernel invariant validation, not broad surface support.

## KD-17. Conformance Fixture Format

### Decision

Conformance is written fixture-first, not as scenario description tables.

```yaml
scenario_id:
name:
initial_state:
input:
action:
expected_state:
expected_events:
expected_artifacts:
expected_projection:
expected_error:
```

### Rule

`11-operations-and-conformance.md` owns fixture format. Each suite provides fixture examples.

## KD-18. Projection Staleness Does Not Rewrite State

### Decision

Projection stale/failed is not state failure. It does not block close by default, but it must be shown in user cards and exports.

### Rule

```text
state current / projection stale
state current / projection failed
```

Distinguish the states above clearly.

### Exception

If the projection itself is needed as the user decision surface, the task may become `waiting_user` or `blocked`. Even then, record the reason as required human decision delivery failure, not projection failure.

## KD-19. Prepare Write Authority

### Decision

`harness.prepare_write` is the gatekeeper before product writes.

### Required Decision Values

```yaml
decision: allowed | blocked | approval_required | state_conflict
```

### Required Checks

```text
- active Task
- active Change Unit
- mode write eligibility
- baseline freshness
- intended paths
- intended tools
- intended commands
- network targets
- secret access
- sensitive categories
- approval scope
- surface capability profile
- design policy preconditions
```

### Rule

The agent does not decide in natural language whether product files may be written.

## KD-20. Close Task Authority

### Decision

`harness.close_task` is the single judgment point for completion conditions.

### Rule

`close_task` judges the following.

```text
- no active run
- active Change Unit state
- scope gate
- approval gate
- design gate
- evidence gate
- verification gate
- QA gate
- acceptance gate
- close_reason
- result
- projection freshness reporting
```

### Rationale

Prevent the agent's completion report from replacing close.

## KD-21. Sensitive Categories

### Decision

Keep sensitive change categories, but define them as canonical enums in the API schema and approval contract.

Minimum categories:

```text
auth_change
permission_model_change
schema_change
dependency_change
public_api_change
destructive_write
network_write
external_service_write
secret_access
production_config_change
ci_cd_change
infra_or_deployment_change
privacy_or_pii_change
data_export
telemetry_or_logging_change
license_or_compliance_change
billing_or_cost_change
model_or_prompt_policy_change
policy_override
```

## KD-22. Later Automation Location

### Decision

Do not write the following items as implementation scope in core document body text.

```text
- dashboard
- browser QA automatic capture
- cross-surface verify
- native hook coverage for every surface
- parallel Change Unit execution
- long-term analytics
- team profile export/import
```

These items are owned by `appendix/C-later-roadmap.md`.

## KD-23. Documentation Ownership Rule

### Decision

Each concept has one canonical owner document. Other documents provide only a one-sentence summary and a reference.

Ownership details are owned by `docs/rewrite-control/DOC-OWNERSHIP-MAP.md`.

## KD-24. Artifact / Report / Projection Boundary

### Decision

Raw artifacts, state records, and Markdown reports are distinct.

```text
Raw artifacts:
  durable evidence files in the artifact store

State records:
  canonical structured records in state.sqlite

Markdown reports:
  projections generated from records and artifact refs
```

`RUN-SUMMARY`, `EVAL`, `TDD-TRACE`, `MANUAL-QA`, `EVIDENCE-MANIFEST`, and `DIRECT-RESULT` are not raw artifacts by default.

Export bundles may include projections with hashes, but that does not make them canonical raw evidence artifacts.

## KD-25. EVAL Verdict, Verification Gate, and Assurance Level

### Decision

An `EVAL` verdict alone does not upgrade assurance.

```text
assurance_level=detached_verified requires:
  - a passed verification result
  - a valid independence qualifier
  - same-session self-review guard not violated
```

Same-session review cannot produce `detached_verified`.

## KD-26. QA Gate and Manual QA Record Result

### Decision

`qa_gate` is the canonical kernel gate.

`manual_qa_record.result` is the record-level result.

User-facing cards may say:

```text
Manual QA: pending/passed/failed/waived
```
