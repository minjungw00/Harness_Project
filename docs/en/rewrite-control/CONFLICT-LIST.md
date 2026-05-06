# Conflict List and Resolutions

This document records contradictions, ambiguities, and overloaded areas found in the existing documentation set, and fixes the resolution policy for the new documentation set.

## Path Convention

```text
docs/README.md:
  harness documentation entrypoint

root README.md:
  repository landing page

All target documentation paths are interpreted under docs/
unless explicitly stated otherwise.
```

## Status Labels

```text
RESOLVED:
  Resolved in KERNEL-DECISIONS.md.

REWRITE:
  Preserve the intent of the existing explanation, but rewrite the structure.

MOVE_TO_OWNER:
  Keep the content, but move it to the canonical owner document.

MOVE_TO_APPENDIX:
  Keep the content, but move it to appendix.

LATER:
  Preserve as design material, but remove from MVP body requirements.

DELETE:
  Remove from canonical document body text.

TODO_DECISION:
  Not decided yet. Do not reflect arbitrarily in body text.
```

## Conflict Type Labels

```text
CONTRADICTION:
  Two documents state criteria that cannot both be true.

AMBIGUITY:
  Implementers or authors can interpret it in more than one way.

OVERLOAD:
  One document or concept carries responsibilities across multiple layers.

SCOPE_LEAK:
  Later or appendix content looks like an MVP body requirement.
```

## Conflict Index

| ID | Type | Short Name | Resolution Status |
|---|---|---|---|
| C-01 | AMBIGUITY | `state.sqlite + event log` | RESOLVED |
| C-02 | CONTRADICTION | User Notes authority | RESOLVED |
| C-03 | CONTRADICTION | Domain Language authority | RESOLVED |
| C-04 | CONTRADICTION | Module/Interface authority | RESOLVED |
| C-05 | CONTRADICTION | Verification invariant vs accepted exception | RESOLVED |
| C-06 | AMBIGUITY | Scope and approval conflation | RESOLVED |
| C-07 | OVERLOAD | Core invariant overload | RESOLVED |
| C-08 | OVERLOAD | State axis density | REWRITE |
| C-09 | OVERLOAD | Strategy owning state machine | MOVE_TO_OWNER |
| C-10 | OVERLOAD | Reference implementation too broad | REWRITE |
| C-11 | SCOPE_LEAK | Full template overload | MOVE_TO_APPENDIX |
| C-12 | OVERLOAD | Surface details in integration core | MOVE_TO_APPENDIX |
| C-13 | AMBIGUITY | Guarantee level naming and placement | REWRITE |
| C-14 | AMBIGUITY | MCP unavailable write hold | RESOLVED |
| C-15 | AMBIGUITY | `record_run` payload | RESOLVED |
| C-16 | AMBIGUITY | `request_user_decision` payload | RESOLVED |
| C-17 | AMBIGUITY | Direct detached verification | RESOLVED |
| C-18 | AMBIGUITY | Projection stale close behavior | RESOLVED |
| C-19 | OVERLOAD | Design validators as core | REWRITE |
| C-20 | SCOPE_LEAK | Long-term metrics | LATER |
| C-21 | SCOPE_LEAK | Browser QA/dashboard/cross-surface automation | LATER |
| C-22 | OVERLOAD | User Guide length | REWRITE |
| C-23 | CONTRADICTION | Authoring ownership table vs target tree | REWRITE |
| C-24 | AMBIGUITY | Missing gate/guarantee glossary terms | REWRITE |
| C-25 | OVERLOAD | Schema ownership | RESOLVED |
| C-26 | AMBIGUITY | Root README vs docs README path | RESOLVED |
| C-27 | CONTRADICTION | Runtime/artifact layout drift | REWRITE |
| C-28 | CONTRADICTION | MVP template tier vs optional design projections | RESOLVED |
| C-29 | AMBIGUITY | Sensitive category prose vs enum | RESOLVED |
| C-30 | AMBIGUITY | Artifact/report/projection boundary | RESOLVED |
| C-31 | OVERLOAD | Conformance scenario tables vs fixtures | REWRITE |
| C-32 | AMBIGUITY | Security boundary vs guarantee level | REWRITE |
| C-33 | SCOPE_LEAK | User guide setup/install detail | MOVE_TO_OWNER |
| C-34 | AMBIGUITY | Capability gate vs validator boundary | RESOLVED |
| C-35 | AMBIGUITY | EVAL verdict vs verification gate vs assurance | RESOLVED |
| C-36 | AMBIGUITY | Manual QA state vs QA gate | RESOLVED |
| C-37 | AMBIGUITY | Evidence gate applicability | RESOLVED |
| C-38 | SCOPE_LEAK | Legacy docs remaining canonical | RESOLVED |

## C-01. `state.sqlite + event log` Ambiguity

### Existing Problem

Several documents describe the basis of operational state as `state.sqlite` and event log. This wording does not clarify whether the event log is an internal SQLite table, a separate file, or an external stream.

### Resolution

Status: `RESOLVED`

The MVP event log is the `state.sqlite.task_events` append-only table.

Canonical phrasing:

```text
The canonical source for operational state is state.sqlite.
state.sqlite has current state tables and an append-only task_events table.
```

### Apply To

```text
README.md
00-introduction.md
02-strategy.md
03-kernel-spec.md
04-runtime-architecture.md
06-reference-mvp.md
07-document-projection.md
99-authoring-guide.md
glossary.md
```

## C-02. User Notes Authority Conflict

### Existing Problem

Some documents describe the canonical source for user notes as the human-editable area, while others describe it as the reconcile item.

### Resolution

Status: `RESOLVED`

```text
Input source:
  human-editable document section

Canonical record for reflection candidates:
  state.sqlite.reconcile_items

Operational fact after reflection:
  state.sqlite event + target record
```

### Apply To

```text
03-kernel-spec.md
07-document-projection.md
11-operations-and-conformance.md
glossary.md
```

## C-03. Domain Language Authority Conflict

### Existing Problem

The phrase `domain language record + reconciled doc` mixes source-of-truth and projection.

### Resolution

Status: `RESOLVED`

```text
canonical source:
  state.sqlite.domain_terms

projection:
  DOMAIN-LANGUAGE

proposal path:
  human-editable section → reconcile_items → domain_terms
```

### Apply To

```text
03-kernel-spec.md
07-document-projection.md
08-design-quality-policy-pack.md
glossary.md
```

## C-04. Module Map / Interface Contract Authority Conflict

### Existing Problem

Module Map and Interface Contract are also described as `record + reconciled doc` or `design records + reconciled docs`, which blurs the boundary between projection and record.

### Resolution

Status: `RESOLVED`

```text
module map canonical source:
  state.sqlite.module_map_items

interface contract canonical source:
  state.sqlite.interface_contracts

projection:
  MODULE-MAP, INTERFACE-CONTRACT
```

## C-05. Detached Verification Invariant vs Accepted Exception

### Existing Problem

Strategy says work does not close on the implementer's self-report alone. Implementation docs say close is possible with detached verification passed or accepted exception. These appear to conflict.

### Resolution

Status: `RESOLVED`

Exception close is possible. But it is not displayed as detached verification.

```yaml
verification_gate: waived_by_user
assurance_level: self_checked
close_reason: completed_with_risk_accepted
```

### Required Wording

```text
The user may accept remaining verification risk and close the task.
In that case, the harness does not display the task as detached_verified.
```

## C-06. Approval and Scope Confirmation Conflation

### Existing Problem

The phrase "check scope and approval" before writing product files can make every scope confirmation look like approval.

### Resolution

Status: `RESOLVED`

```text
scope_gate:
  applies to every write-capable run

approval_gate:
  required only when a sensitive category exists
```

### Apply To

```text
02-strategy.md
03-kernel-spec.md
05-mcp-api-and-schemas.md
10-user-guide.md
```

## C-07. Core Invariant Overload

### Existing Problem

The 17 invariants in the existing strategy document contain many good principles, but they mix MVP kernel invariants with design-quality policy defaults.

### Resolution

Status: `RESOLVED`

Reduce core invariants to 7. Move shared design, domain language, vertical slice, TDD, module/interface review, manual QA, and context hygiene to policy defaults.

### Apply To

```text
02-strategy.md
08-design-quality-policy-pack.md
99-authoring-guide.md
```

## C-08. State Axis Density

### Existing Problem

The existing state axes include many fields such as `mode`, `phase`, `result`, `assurance`, `verification_independence`, `approval`, `manual_qa`, `acceptance`, `risk`, `evidence`, `design_alignment`, and `architecture`. Impossible combinations are not closed enough.

### Resolution

Status: `REWRITE`

Rewrite as lifecycle + gates in `03-kernel-spec.md`.

```text
Lifecycle:
  mode, lifecycle_phase, result, close_reason

Gates:
  scope, approval, design, evidence, verification, QA, acceptance
```

## C-09. Strategy Owning State Machine

### Existing Problem

The existing strategy document owns many state axes and work-model details. In the new structure, the strategy document becomes overloaded if it owns the implementation state machine.

### Resolution

Status: `MOVE_TO_OWNER`

```text
02-strategy.md:
  why, failure model, core invariants, policy defaults

03-kernel-spec.md:
  state machine, gates, transitions, close semantics
```

## C-10. Reference Implementation Too Broad

### Existing Problem

The existing reference implementation contains MVP, storage, Core, state contract, MCP, approval, evidence, artifact, verification, design-quality, validators, security, adapter, and recovery all together.

### Resolution

Status: `REWRITE`

Split into:

```text
03-kernel-spec.md:
  entity/state/gate/transition

05-mcp-api-and-schemas.md:
  MCP resources/tools/schema/error/validator result

06-reference-mvp.md:
  implementation sequence, SQLite DDL, artifact layout, reference surface

appendix/C-later-roadmap.md:
  later automation
```

## C-11. Full Template Overload

### Existing Problem

The document contracts doc contains many full templates at the same weight. MVP implementers may misunderstand all projections as required.

### Resolution

Status: `MOVE_TO_APPENDIX`

```text
07-document-projection.md:
  projection principles, authority, managed/human-editable, required MVP templates

appendix/A-template-library.md:
  full templates and expanded variants
```

## C-12. Surface-Specific Connector Details in Core Integration

### Existing Problem

The agent integration doc includes Codex, Claude Code, Gemini, Copilot, and Cursor addenda, mixing the core connector contract with the cookbook.

### Resolution

Status: `MOVE_TO_APPENDIX`

```text
09-agent-integration.md:
  capability profile, connector contract, fallback, reference surface

appendix/B-surface-cookbook.md:
  surface-specific notes
```

## C-13. Guarantee Level Hidden Too Late

### Existing Problem

Architecture includes advisory/detective/preventive/isolated guarantee levels, but they are not surfaced early enough for users/implementers.

### Resolution

Status: `REWRITE`

Unify the new terms as follows.

```text
cooperative
detective
preventive
isolated
```

Explain them at the appropriate layer in `02-strategy.md`, `04-runtime-architecture.md`, `09-agent-integration.md`, and `10-user-guide.md`.

## C-14. MCP Unavailable Write Hold Ambiguity

### Existing Problem

"If MCP is unavailable, hold product file writes" is correct, but depending on surface capability, it may not be preventively enforceable.

### Resolution

Status: `RESOLVED`

```text
T2 MCP + no guard:
  cooperative guarantee

T3/T4 sidecar/guard:
  detective/preventive guarantee possible

T5 isolation:
  isolated guarantee possible
```

Documents distinguish "should hold" from "can enforce."

## C-15. `record_run` Payload Too Broad

### Existing Problem

`harness.record_run` may receive design updates, changed files, commands, logs, TDD trace, evidence mapping, and run summary, creating a risk of an over-broad JSON blob.

### Resolution

Status: `RESOLVED`

Make the `kind` field required.

```yaml
kind: shaping_update | implementation | direct | verification_input
```

Define per-tool schemas in `05-mcp-api-and-schemas.md` as discriminated unions by kind.

## C-16. `request_user_decision` Payload Too Broad

### Existing Problem

Approval, scope, unresolved decision, QA, acceptance, and reconcile judgments are all inside one tool, so their meanings can mix.

### Resolution

Status: `RESOLVED`

Make the `decision_kind` field required.

```yaml
decision_kind:
  approval | scope_confirmation | design_choice |
  qa_waiver | acceptance | reconcile
```

## C-17. Direct Detached Verification Ambiguity

### Existing Problem

Direct can close as self_checked, but conformance includes the possibility of detached verify after direct. It is unclear whether Direct can have `detached_verified`.

### Resolution

Status: `RESOLVED`

Direct is not verification-required by default. However, if optional fresh verification is performed, it may have `assurance_level=detached_verified`.

## C-18. Projection Stale as Close Blocker

### Existing Problem

It may be unclear whether projection failure is state failure or blocks close.

### Resolution

Status: `RESOLVED`

Projection stale/failed is not state failure and does not block close by default. However, it must be shown to the user.

```text
state current / projection stale
state current / projection failed
```

## C-19. Design-Quality Validators as MVP Core

### Existing Problem

Vertical slice, TDD trace, module boundary, and manual QA validators look like core invariants.

### Resolution

Status: `REWRITE`

MVP keeps only minimal validator hooks. Application conditions and waivers are owned by `08-design-quality-policy-pack.md`.

## C-20. Long-Term Metrics in Operations

### Existing Problem

The operating metrics list may look like an MVP operations requirement.

### Resolution

Status: `LATER`

MVP operations stay centered on conformance and doctor/recover. Long-term analytics and metrics move to `appendix/C-later-roadmap.md`.

## C-21. Browser QA Capture and Dashboard in Main Docs

### Existing Problem

If automatic browser QA capture, dashboard, and cross-surface verify appear in early documents, they look like MVP requirements.

### Resolution

Status: `LATER`

All later automation goes to `appendix/C-later-roadmap.md`.

## C-22. User Guide Too Long

### Existing Problem

The existing User Guide has rich practical phrasing, but it is too long to read as a quick-start document.

### Resolution

Status: `REWRITE`

Rewrite `10-user-guide.md` to be short and conversation-centered. Keep only representative examples and remove long repetitive examples.

## C-23. Authoring Guide Ownership Table Outdated

### Existing Problem

The existing Authoring Guide explains ownership based on the old document tree.

### Resolution

Status: `REWRITE`

`99-authoring-guide.md` reflects the new tree, new owner map, schema ownership, template ownership, and appendix ownership.

## C-24. Glossary Missing Gate and Guarantee Terms

### Existing Problem

The model lacks terms needed by the new model: Gate, Scope Gate, Approval Gate, Evidence Gate, Verification Gate, QA Gate, Acceptance Gate, Close Reason, Waiver, and Guarantee Level.

### Resolution

Status: `REWRITE`

Add the new terms to `glossary.md` and revise existing definitions for Source-of-truth, Projection, Domain Language, Reconcile, Assurance, and Detached Verification.

## C-25. Schema Ownership Ambiguity

### Existing Problem

MCP schema, validator result schema, artifact schema, and SQLite DDL may be scattered across several documents.

### Resolution

Status: `RESOLVED`

```text
MCP wire schema:
  05-mcp-api-and-schemas.md

validator result schema:
  05-mcp-api-and-schemas.md

artifact ref schema:
  05-mcp-api-and-schemas.md

SQLite DDL:
  06-reference-mvp.md

state transition:
  03-kernel-spec.md
```

## C-26. Root README vs Docs README Path Ambiguity

### Existing Problem

The current repository has both root `README.md` and `docs/README.md`. Root `README.md` is a short project description, while `docs/README.md` is the front door for the harness documentation set. Because user requests and `TARGET-DOC-TREE.md` may say only `README.md`, the rewrite target path is ambiguous.

### Resolution

Status: `RESOLVED`

Use Option A.

```text
docs/README.md:
  harness documentation entrypoint

root README.md:
  repository landing page

Path convention:
  All target documentation paths are interpreted under docs/
  unless explicitly stated otherwise.
```

### Apply To

```text
TARGET-DOC-TREE.md
DOC-OWNERSHIP-MAP.md
CODEX-BATCHES.md
REVIEW-CHECKLIST.md
PRESERVE-MOVE-LATER.md
```

## C-27. Runtime Home and Artifact Directory Layout Drift

### Existing Problem

`00-overview.md`, `03-architecture.md`, `04-reference-implementation.md`, `05-user-guide.md`, and `08-operations-and-conformance.md` show `~/.harness` and artifact subdirectories slightly differently. Examples differ by whether they include `config.yaml`, `traces`, `checkpoints`, `exports`, or `manifests`.

### Resolution

Status: `REWRITE`

The exact runtime/artifact directory layout is owned by `06-reference-mvp.md`. `04-runtime-architecture.md` explains only the three spaces and the high-level structure of the artifact store. README, introduction, and user guide summarize only the default `~/.harness` location and the existence of `state.sqlite` and `artifacts/`.

## C-28. MVP Template Tier vs Optional Design-Quality Projections

### Existing Problem

`04-reference-implementation.md` includes minimal projections for `DOMAIN-LANGUAGE`, `MODULE-MAP`, `INTERFACE-CONTRACT`, `TDD-TRACE`, and `MANUAL-QA` in MVP scope. `KERNEL-DECISIONS.md` splits template tiers into Required MVP and Optional design-quality. Keeping both as-is could make implementers misunderstand the whole design-quality projection set as required MVP.

### Resolution

Status: `RESOLVED`

Follow `KD-15`.

```text
Required MVP:
  TASK, APR, RUN-SUMMARY, EVIDENCE-MANIFEST, EVAL, DIRECT-RESULT

Optional design-quality:
  DOMAIN-LANGUAGE, MODULE-MAP, INTERFACE-CONTRACT, TDD-TRACE, MANUAL-QA
```

Describe design-quality projections as optional projections that support policy/gates. The required/optional/full variant distinction is owned by `07-document-projection.md` and `appendix/A-template-library.md`.

## C-29. Sensitive Category Prose vs Canonical Enum

### Existing Problem

`02-strategy.md`, `05-user-guide.md`, and `04-reference-implementation.md` repeat sensitive change categories. Some are human-readable descriptions and some are enums. They look like the same list, but their granularity differs.

### Resolution

Status: `RESOLVED`

The canonical enum is owned by `05-mcp-api-and-schemas.md`. `10-user-guide.md` keeps only a user-facing summary. `02-strategy.md` keeps only the principle "sensitive change requires explicit approval." Use the minimum categories in `KD-21` as the basis for writing API schemas.

## C-30. Artifact / Report / Projection Boundary Ambiguity

### Existing Problem

Some documents include run summary, eval report, TDD trace, and manual QA record as Artifact examples. Other documents treat them as state records and Markdown projections. The boundary between raw artifact, state record, and projection document can blur.

### Resolution

Status: `RESOLVED`

The boundary is fixed as follows.

```text
Raw artifacts:
  durable evidence files in the artifact store

State records:
  canonical structured records in state.sqlite

Markdown reports:
  projections generated from records and artifact refs

RUN-SUMMARY, EVAL, TDD-TRACE, MANUAL-QA,
EVIDENCE-MANIFEST, DIRECT-RESULT:
  not raw artifacts by default

Export bundles:
  may include projections with hashes, but that does not make
  those projections canonical raw evidence artifacts
```

### Apply To

```text
03-kernel-spec.md
04-runtime-architecture.md
05-mcp-api-and-schemas.md
07-document-projection.md
11-operations-and-conformance.md
glossary.md
```

## C-31. Conformance Scenario Tables vs Fixture-Based Conformance

### Existing Problem

`04-reference-implementation.md`, `06-agent-integration.md`, and `08-operations-and-conformance.md` often explain conformance as scenario lists or checklists. The rewrite brief and `KD-17` require fixture-based conformance.

### Resolution

Status: `REWRITE`

`11-operations-and-conformance.md` owns fixture format.

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

Preserve existing scenario/checklist content as source material for fixture examples, but rewrite the body standard around fixture shape.

## C-32. Security Boundary vs Guarantee Level Ambiguity

### Existing Problem

`04-reference-implementation.md` says it enforces filesystem/process/network/credential/data boundaries. `06-agent-integration.md` explains surface capability and fallback. This can read as if preventive security boundaries are always enforced even in environments with low surface capability.

### Resolution

Status: `REWRITE`

Preserve the desired policy for security boundaries. But display guarantees according to capability.

```text
cooperative:
  assumes the surface follows the procedure

detective:
  validator/sidecar detects violations

preventive:
  hook/policy engine blocks before execution

isolated:
  worktree/sandbox/process isolation
```

`04-runtime-architecture.md` owns boundary placement, `09-agent-integration.md` owns capability expression, and `06-reference-mvp.md` owns the actual guarantee of the MVP reference surface.

## C-33. User Guide Setup/Install Detail Overload

### Existing Problem

`05-user-guide.md` combines a conversation-centered quick start with setup/connect/CLI explanations. The user guide may appear to own operator procedures.

### Resolution

Status: `MOVE_TO_OWNER`

`10-user-guide.md` owns only the flow of how users speak and read. Setup/connect command semantics and doctor/recover/export are owned by `11-operations-and-conformance.md`. The user guide keeps only short guidance at the level of "connect once at the beginning."

## C-34. Capability Gate vs Validator Boundary

### Existing Problem

`KD-04` and some architecture/API wording may read as if capability could be a separate gate. That risks expanding the canonical gate list in `03-kernel-spec.md` beyond scope, approval, design, evidence, verification, QA, and acceptance.

### Resolution

Status: `RESOLVED`

MVP does not have a first-class `capability_gate`.

Capability is represented by:

```text
surface_capability_check validator
prepare_write blocked_reasons
guarantee level display
```

Ownership:

```text
09-agent-integration.md:
  capability profile

04-runtime-architecture.md:
  guarantee level architecture

03-kernel-spec.md:
  canonical gate list, excluding capability_gate
```

Required wording:

```text
Capability can block or downgrade a run through validator results,
prepare_write blocked reasons, and guarantee display. It is not a
kernel gate in MVP.
```

## C-35. EVAL Verdict vs Verification Gate vs Assurance Level

### Existing Problem

An `EVAL` projection can say `verdict: passed`, while the kernel also tracks `verification_gate` and `assurance_level`. Without an explicit rule, writers may treat any passed EVAL as enough to set `assurance_level=detached_verified`.

### Resolution

Status: `RESOLVED`

`EVAL` verdict alone does not upgrade assurance.

```text
assurance_level=detached_verified requires:
  - a passed verification result
  - a valid independence qualifier
  - same-session self-review guard not violated
```

Same-session review cannot produce `detached_verified`.

Owner:

```text
03-kernel-spec.md:
  assurance update rule

05-mcp-api-and-schemas.md:
  record_eval request/response fields and errors

07-document-projection.md:
  EVAL projection wording
```

## C-36. Manual QA State vs QA Gate

### Existing Problem

Existing docs use `manual_qa_state`, `MANUAL-QA.result`, and user card text interchangeably. This can make the record-level QA result look like the canonical kernel state.

### Resolution

Status: `RESOLVED`

```text
qa_gate:
  canonical kernel gate

manual_qa_record.result:
  record-level result

User-facing cards:
  may say Manual QA: pending/passed/failed/waived
```

Owner:

```text
03-kernel-spec.md:
  qa_gate and close semantics

07-document-projection.md:
  MANUAL-QA projection and card wording

10-user-guide.md:
  user-facing explanation
```

## C-37. Evidence Gate Applicability

### Existing Problem

`evidence_gate` previously lacked a value for tasks where evidence coverage does not apply, such as advisor-only work. This made `none` ambiguous: it could mean evidence is irrelevant or evidence is required but absent.

### Resolution

Status: `RESOLVED`

Use this enum:

```yaml
evidence_gate:
  not_required | none | partial | sufficient | stale | blocked
```

Meaning:

```text
not_required:
  evidence gate does not apply, for example advisor-only work

none:
  evidence is required but no evidence has been recorded yet
```

Owner:

```text
03-kernel-spec.md:
  evidence_gate enum and close semantics

07-document-projection.md:
  user-facing projection and card wording
```

## C-38. Legacy Docs Remaining Canonical

### Existing Problem

After v2 target docs are created, replaced legacy docs can remain in `docs/` and still look canonical. This risks stale links from `docs/README.md`, duplicate ownership, and consistency grep treating archived migration material as active main documentation.

Legacy docs include:

```text
docs/00-overview.md
docs/03-architecture.md
docs/04-reference-implementation.md
docs/05-user-guide.md
docs/06-agent-integration.md
docs/07-document-and-artifact-contracts.md
docs/08-operations-and-conformance.md
docs/09-design-quality-playbooks.md
```

### Resolution

Status: `RESOLVED`

After content migration, legacy docs replaced by v2 docs must not remain as canonical docs.

Allowed treatments:

```text
DELETE:
  remove the legacy doc after migration

MIGRATION_STUB:
  replace with a short stub pointing to the v2 owner and migration notes

MOVE_TO_APPENDIX:
  move historical notes to docs/appendix/D-migration-notes.md
```

Rules:

```text
docs/README.md must not link to legacy docs except migration notes.
Final consistency grep must scan active canonical docs separately from
docs/appendix/D-migration-notes.md.
Archived migration notes are not active canonical docs.
```
