# Review Checklist

This document is the review standard for the rewritten harness documentation set. After each Codex batch, use this checklist for semantic QA.

## 0. Path Convention

```text
docs/README.md:
  harness documentation entrypoint

root README.md:
  repository landing page

All target documentation paths are interpreted under docs/
unless explicitly stated otherwise.
```

## 1. Global Completion Checklist

```text
[ ] core invariants remain at 7.
[ ] policy defaults and core invariants are not mixed.
[ ] event log location is clearly state.sqlite.task_events.
[ ] lifecycle + gates state model exists.
[ ] state transition table and impossible combinations exist.
[ ] close_task algorithm exists.
[ ] prepare_write algorithm exists.
[ ] approval gate and scope gate are separated.
[ ] detached verification waiver is not displayed as detached_verified.
[ ] User Notes authority is organized into three stages.
[ ] Domain Language canonical source is organized as the domain_terms table.
[ ] Module Map canonical source is organized as the module_map_items table.
[ ] Interface Contract canonical source is organized as the interface_contracts table.
[ ] guarantee level is displayed as cooperative/detective/preventive/isolated.
[ ] per-MCP-tool request/response schemas exist.
[ ] error code taxonomy exists.
[ ] validator result schema exists.
[ ] SQLite DDL draft exists.
[ ] artifact schema and redaction rules exist.
[ ] projection job schema exists.
[ ] required/optional/appendix templates are separated.
[ ] connector documents are centered on capability profiles.
[ ] surface-specific details are in docs/appendix.
[ ] User Guide is short and conversation-oriented.
[ ] Operations conformance is fixture-based.
[ ] docs/README.md reader paths match the new document structure.
[ ] all official terms in Glossary match the new model.
[ ] later features do not blur MVP body text.
[ ] legacy docs replaced by v2 docs no longer remain as canonical docs.
```

## 2. Source-of-Truth Checks

### Required Statements

The following meanings must be consistent across all documents.

```text
The canonical source for operational state is state.sqlite.
state.sqlite has current state tables and an append-only task_events table.
Documents are human-readable projections.
The canonical source for raw evidence is the artifact store.
Raw artifacts are durable evidence files in the artifact store.
State records are canonical structured records in state.sqlite.
Markdown reports are projections generated from records and artifact refs.
Human-editable areas are input surfaces.
State reflection happens through an MCP tool or reconcile action.
```

### Prohibited or Suspicious Phrases

Review if the following expression appears.

```text
state.sqlite + event log
```

Allowed condition:

```text
The same sentence or the immediately following sentence must clarify that the event log is state.sqlite.task_events.
```

Prohibited phrases:

```text
domain language record + reconciled doc
module map records + reconciled doc
user notes canonical source = human-editable document area
projection updates state
TASK is canonical state
RUN-SUMMARY/EVAL/TDD-TRACE/MANUAL-QA/EVIDENCE-MANIFEST/DIRECT-RESULT are raw artifacts
exported projection hash makes a projection canonical raw evidence
```

## 3. State and Gate Checks

```text
[ ] `03-kernel-spec.md` owns the lifecycle_phase enum.
[ ] `03-kernel-spec.md` owns the gate enum.
[ ] `evidence_gate` enum is `not_required | none | partial | sufficient | stale | blocked`.
[ ] `evidence_gate=not_required` means evidence gate does not apply.
[ ] `evidence_gate=none` means evidence is required but absent.
[ ] `02-strategy.md` does not include gate enum detail.
[ ] `10-user-guide.md` only explains gates at a user-understandable level.
[ ] `close_reason` is defined.
[ ] `completed_verified` and `completed_with_risk_accepted` are distinguished.
[ ] The combination `verification_gate=waived_by_user` and `assurance_level=detached_verified` is prohibited.
[ ] `direct` optional detached verification rules are clear.
[ ] Projection stale/failed is distinguished from state failure.
[ ] `03-kernel-spec.md` canonical gate list does not include `capability_gate`.
[ ] Capability is represented through the `surface_capability_check` validator, `prepare_write` blocked reason, and guarantee display.
```

## 4. Scope / Approval Checks

```text
[ ] Scope gate is described as applying to every write-capable run.
[ ] Approval gate is described as required when a sensitive category exists.
[ ] `prepare_write` request receives intended paths/tools/commands/network/secrets/sensitive categories.
[ ] Approval request has allowed paths/tools/network/secrets/baseline/expiry.
[ ] approval scope drift is represented as approval expired or re-approval required.
[ ] User Guide explains that approval does not replace verification or acceptance.
```

## 5. Verification / QA / Acceptance Checks

```text
[ ] Work does not accept same-session self-review as detached verification.
[ ] Verification independence qualifier is defined.
[ ] EVAL verdict alone does not upgrade assurance.
[ ] `assurance_level=detached_verified` requires passed verification and valid independence qualifier.
[ ] Same-session review cannot produce `detached_verified`.
[ ] Manual QA is distinct from verification.
[ ] Acceptance is distinct from manual QA.
[ ] `qa_gate` is the canonical kernel gate.
[ ] `manual_qa_record.result` is record-level result.
[ ] User-facing cards may say Manual QA: pending/passed/failed/waived.
[ ] QA waived requires a waiver reason.
[ ] Acceptance accepted is the judgment that remaining trade-offs are accepted.
[ ] QA failed leads to rework or blocked state.
```

## 6. MCP API Checks

`05-mcp-api-and-schemas.md` must provide the following for every public tool.

```text
[ ] purpose
[ ] allowed actor
[ ] request schema
[ ] response schema
[ ] state transition summary
[ ] events emitted
[ ] projection jobs enqueued
[ ] validators run
[ ] possible errors
[ ] idempotency behavior
```

Public tools:

```text
[ ] harness.status
[ ] harness.intake
[ ] harness.next
[ ] harness.prepare_write
[ ] harness.record_run
[ ] harness.request_user_decision
[ ] harness.record_user_decision
[ ] harness.launch_verify
[ ] harness.record_eval
[ ] harness.record_manual_qa
[ ] harness.close_task
```

Required discriminators:

```text
[ ] record_run.kind
[ ] request_user_decision.decision_kind
```

Common envelope:

```text
[ ] request_id
[ ] idempotency_key
[ ] expected_state_version
[ ] project_id
[ ] task_id optional
[ ] surface_id
[ ] run_id optional
[ ] actor_kind
[ ] dry_run
```

Required error codes:

```text
[ ] STATE_CONFLICT
[ ] NO_ACTIVE_TASK
[ ] NO_ACTIVE_CHANGE_UNIT
[ ] SCOPE_REQUIRED
[ ] SCOPE_VIOLATION
[ ] APPROVAL_REQUIRED
[ ] APPROVAL_DENIED
[ ] APPROVAL_EXPIRED
[ ] CAPABILITY_INSUFFICIENT
[ ] MCP_UNAVAILABLE
[ ] EVIDENCE_INSUFFICIENT
[ ] VERIFY_NOT_DETACHED
[ ] QA_REQUIRED
[ ] ACCEPTANCE_REQUIRED
[ ] PROJECTION_STALE
[ ] RECONCILE_REQUIRED
[ ] ARTIFACT_MISSING
[ ] BASELINE_STALE
[ ] VALIDATOR_FAILED
```

## 7. SQLite / Reference MVP Checks

`06-reference-mvp.md` must include DDL drafts or explicit TODO_IMPLEMENT for:

```text
registry.sqlite:
[ ] projects
[ ] project_surfaces
[ ] connector_manifests

state.sqlite:
[ ] tasks
[ ] task_gates
[ ] change_units
[ ] runs
[ ] approvals
[ ] evidence_manifests
[ ] evals
[ ] manual_qa_records
[ ] artifacts
[ ] task_events
[ ] projection_jobs
[ ] reconcile_items
[ ] domain_terms
[ ] module_map_items
[ ] interface_contracts
[ ] tdd_traces
[ ] validator_runs
[ ] locks
```

Implementation sequence:

```text
[ ] MVP-0 Harness Kernel
[ ] MVP-1 Scope, Approval, Evidence
[ ] MVP-2 Verification
[ ] MVP-3 Design-quality minimum
[ ] MVP-4 Projection and Reconcile
[ ] MVP-5 Reference Surface
```

## 8. Projection Checks

```text
[ ] `07-document-projection.md` says projection is not source-of-truth.
[ ] Managed block behavior is defined.
[ ] Human-editable section behavior is defined.
[ ] Projection freshness is defined.
[ ] Projection failure is separate from state failure.
[ ] Artifact refs require registry, sha256, task/run relation, retention, redaction state.
[ ] Raw artifacts are durable evidence files, not Markdown report projections by default.
[ ] RUN-SUMMARY/EVAL/TDD-TRACE/MANUAL-QA/EVIDENCE-MANIFEST/DIRECT-RESULT are projections from records and artifact refs.
[ ] Export bundles may include projections with hashes without making them canonical raw evidence artifacts.
[ ] Required MVP templates are limited to TASK/APR/RUN-SUMMARY/EVIDENCE-MANIFEST/EVAL/DIRECT-RESULT.
[ ] Optional design-quality templates are summarized only.
[ ] Full templates are in docs/appendix/A.
```

## 9. Design-Quality Policy Checks

Each policy in `08-design-quality-policy-pack.md` should have:

```text
[ ] name
[ ] applies_when
[ ] default_requirement
[ ] allowed_waiver
[ ] required_record
[ ] validator
[ ] evidence
[ ] close_impact
```

Policies required:

```text
[ ] Shared Design
[ ] Domain Language
[ ] Vertical Slice
[ ] TDD Trace
[ ] Deep Module / Interface
[ ] Manual QA
[ ] Context Hygiene
```

## 10. Agent Integration Checks

```text
[ ] `09-agent-integration.md` is product-name-neutral.
[ ] It uses capability profile rather than assuming capabilities by surface name.
[ ] It defines cooperative/detective/preventive/isolated fallback.
[ ] It owns capability profile.
[ ] It does not define a kernel `capability_gate`.
[ ] It states that MCP unavailable means write should be held, but whether this is enforceable depends on capability.
[ ] Surface-specific details are not in the main integration doc.
[ ] Surface cookbook exists in docs/appendix/B.
```

## 11. User Guide Checks

```text
[ ] The user guide starts from conversation, not CLI.
[ ] It explains "Run this work under the harness."
[ ] It explains status card reading.
[ ] It explains advisor/direct/work.
[ ] It explains approval/assurance/manual QA/acceptance with examples.
[ ] It explains resume by state, not chat history.
[ ] It avoids DB schema, tool payload, and implementation detail.
[ ] It is shorter than the old guide.
```

## 12. Operations and Conformance Checks

```text
[ ] Operations owns connect/doctor/serve/reconcile/recover/export/artifact check/conformance.
[ ] Conformance is fixture-based.
[ ] Each fixture includes scenario_id, initial_state, input, action, expected_state, expected_events, expected_artifacts, expected_projection, expected_error.
[ ] Long-term metrics are not presented as MVP requirements.
[ ] Failure handling reports state current vs projection failed/stale separately.
```

## 13. Later Boundary Checks

Main docs must not treat these as MVP requirements:

```text
[ ] dashboard
[ ] browser QA automatic capture
[ ] cross-surface verify
[ ] native hooks for every surface
[ ] advanced sidecar watcher
[ ] parallel Change Unit orchestration
[ ] long-term analytics
[ ] team profile export/import
```

If mentioned in main docs, they must be clearly labeled `later` and point to `docs/appendix/C-later-roadmap.md`.

## 14. Legacy Cleanup Checks

Legacy docs replaced by v2 docs:

```text
[ ] docs/legacy-v1/00-overview.md
[ ] docs/legacy-v1/01-project-charter.md
[ ] docs/legacy-v1/02-strategy.md
[ ] docs/legacy-v1/03-architecture.md
[ ] docs/legacy-v1/04-reference-implementation.md
[ ] docs/legacy-v1/05-user-guide.md
[ ] docs/legacy-v1/06-agent-integration.md
[ ] docs/legacy-v1/07-document-and-artifact-contracts.md
[ ] docs/legacy-v1/08-operations-and-conformance.md
[ ] docs/legacy-v1/09-design-quality-playbooks.md
[ ] docs/legacy-v1/99-authoring-guide.md
[ ] docs/legacy-v1/glossary.md
[ ] docs/legacy-v1/REWRITE-MANIFEST.md
```

Required cleanup behavior:

```text
[ ] Each archived legacy doc is deleted, replaced by a short migration stub, or represented in docs/appendix/D-migration-notes.md.
[ ] docs/README.md does not link to legacy docs except migration notes.
[ ] Active canonical docs are the v2 target docs.
[ ] Archived migration notes are not treated as active canonical docs.
[ ] Final consistency grep scans active canonical docs separately from docs/appendix/D-migration-notes.md.
```

## 15. Glossary Checks

New or updated terms:

```text
[ ] Gate
[ ] Scope Gate
[ ] Approval Gate
[ ] Design Gate
[ ] Evidence Gate
[ ] Verification Gate
[ ] QA Gate
[ ] Acceptance Gate
[ ] Close Reason
[ ] Waiver
[ ] Guarantee Level
[ ] Cooperative Guarantee
[ ] Detective Guarantee
[ ] Preventive Guarantee
[ ] Isolated Guarantee
[ ] Reference Surface
[ ] Source-of-truth
[ ] Projection
[ ] Raw Artifact
[ ] State Record
[ ] Markdown Report
[ ] Report Projection
[ ] Domain Language
[ ] Human-editable Area
[ ] Reconcile
[ ] Detached Verification
[ ] Assurance
```

## 16. Suggested Grep Checks

Run these checks manually or through Codex.

```bash
grep -R "state.sqlite + event log" .
grep -R "domain language record + reconciled doc" .
grep -R "module map records + reconciled doc" .
grep -R "detached verification passed or accepted exception" .
grep -R "capability_gate" docs/03-kernel-spec.md docs/02-strategy.md docs/05-mcp-api-and-schemas.md
grep -R "RUN-SUMMARY.*raw artifact\\|EVAL.*raw artifact\\|TDD-TRACE.*raw artifact\\|MANUAL-QA.*raw artifact" docs
grep -R "all agent surface" .
grep -R "all agent surfaces" .
grep -R "dashboard" docs/README.md docs/00-introduction.md docs/01-project-charter.md docs/02-strategy.md docs/03-kernel-spec.md docs/04-runtime-architecture.md docs/05-mcp-api-and-schemas.md docs/06-reference-mvp.md docs/07-document-projection.md docs/08-design-quality-policy-pack.md docs/09-agent-integration.md docs/10-user-guide.md docs/11-operations-and-conformance.md
grep -R "browser QA" docs/README.md docs/00-introduction.md docs/01-project-charter.md docs/02-strategy.md docs/03-kernel-spec.md docs/04-runtime-architecture.md docs/05-mcp-api-and-schemas.md docs/06-reference-mvp.md docs/07-document-projection.md docs/08-design-quality-policy-pack.md docs/09-agent-integration.md docs/10-user-guide.md docs/11-operations-and-conformance.md
grep -R "cross-surface" docs/README.md docs/00-introduction.md docs/01-project-charter.md docs/02-strategy.md docs/03-kernel-spec.md docs/04-runtime-architecture.md docs/05-mcp-api-and-schemas.md docs/06-reference-mvp.md docs/07-document-projection.md docs/08-design-quality-policy-pack.md docs/09-agent-integration.md docs/10-user-guide.md docs/11-operations-and-conformance.md
grep -R "docs/00-overview.md\\|docs/03-architecture.md\\|docs/04-reference-implementation.md\\|docs/05-user-guide.md\\|docs/06-agent-integration.md\\|docs/07-document-and-artifact-contracts.md\\|docs/08-operations-and-conformance.md\\|docs/09-design-quality-playbooks.md\\|docs/legacy-v1/" docs/README.md docs/00-introduction.md docs/01-project-charter.md docs/02-strategy.md docs/03-kernel-spec.md docs/04-runtime-architecture.md docs/05-mcp-api-and-schemas.md docs/06-reference-mvp.md docs/07-document-projection.md docs/08-design-quality-policy-pack.md docs/09-agent-integration.md docs/10-user-guide.md docs/11-operations-and-conformance.md docs/99-authoring-guide.md docs/glossary.md
```

Interpretation:

```text
- Some hits may be allowed in migration notes or later roadmap.
- Hits in main docs require review.
- Do not treat docs/appendix/D-migration-notes.md as an active canonical doc.
- Do not blindly delete; fix ownership and phrasing.
```

## 17. Review Output Format

When reviewing a Codex batch, use this format.

```md
# Batch Review

## Must Fix
- item / file / reason / required change

## Should Fix
- item / file / reason / suggested change

## Acceptable
- item / note

## Possible TODO_DECISION
- decision needed / affected files / options

## Codex Follow-up Prompt
```text
...
```
```

## 18. Final Acceptance

Final acceptance requires:

```text
[ ] Semantic review passed.
[ ] Implementability review passed.
[ ] User readability review passed.
[ ] Ownership review passed.
[ ] Link/file tree review passed.
[ ] Legacy file cleanup review passed.
[ ] Later boundary review passed.
[ ] Human accepts remaining trade-offs.
```
