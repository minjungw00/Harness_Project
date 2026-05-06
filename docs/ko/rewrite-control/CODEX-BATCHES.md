# Codex Batches

이 문서는 Codex에게 맡길 batch별 작업 지시를 정의한다. Codex는 저장소 파일을 실제로 편집하는 repo editor다. 새로운 아키텍처 결정은 하지 않는다.

## 0. 경로 규칙

```text
docs/README.md:
  harness documentation entrypoint

root README.md:
  repository landing page

All target documentation paths are interpreted under docs/
unless explicitly stated otherwise.
```

## 1. Global Codex Rules

모든 Codex task는 다음 rule을 따라야 한다.

```text
- Read docs/rewrite-control/REWRITE-BRIEF.md first.
- Read docs/rewrite-control/KERNEL-DECISIONS.md before changing main docs.
- Do not invent new architecture decisions.
- If a decision is needed, mark TODO_DECISION.
- Do not preserve old text merely because it exists.
- Keep canonical ownership defined in docs/rewrite-control/DOC-OWNERSHIP-MAP.md.
- Keep MVP content in main docs.
- Move later automation to docs/appendix/C-later-roadmap.md.
- Move surface-specific details to docs/appendix/B-surface-cookbook.md.
- Move full templates to docs/appendix/A-template-library.md.
- Keep state transitions in docs/03-kernel-spec.md only.
- Keep MCP schemas in docs/05-mcp-api-and-schemas.md only.
- Keep SQLite DDL in docs/06-reference-mvp.md only.
- Keep user-facing docs short and conversation-oriented.
- Do not edit root README.md unless a batch explicitly says repository landing page.
```

## 2. Branch Setup

권장 branch:

```bash
git checkout -b docs/harness-rewrite-v2
```

아직 없으면 control directory를 만든다.

```text
docs/rewrite-control/
  REWRITE-BRIEF.md
  KERNEL-DECISIONS.md
  TARGET-DOC-TREE.md
  DOC-OWNERSHIP-MAP.md
  CONFLICT-LIST.md
  PRESERVE-MOVE-LATER.md
  CODEX-BATCHES.md
  REVIEW-CHECKLIST.md
```

Legacy v1 source document는 다음 아래 archived될 수 있다.

```text
docs/legacy-v1/
```

Batch가 archived된 pre-rewrite source file을 요구하면 `docs/legacy-v1/`의 matching file을 읽고 `docs/appendix/D-migration-notes.md`를 path map으로 사용한다.

## 3. Batch 0 — Inventory and Conflict Confirmation

### 목표

Canonical doc을 편집하기 전에 existing content inventory를 확인한다.

### 읽을 파일

```text
README.md
docs/README.md
docs/legacy-v1/00-overview.md
docs/01-project-charter.md
docs/legacy-v1/01-project-charter.md
docs/02-strategy.md
docs/legacy-v1/02-strategy.md
docs/legacy-v1/03-architecture.md
docs/legacy-v1/04-reference-implementation.md
docs/legacy-v1/05-user-guide.md
docs/legacy-v1/06-agent-integration.md
docs/legacy-v1/07-document-and-artifact-contracts.md
docs/legacy-v1/08-operations-and-conformance.md
docs/legacy-v1/09-design-quality-playbooks.md
docs/99-authoring-guide.md
docs/legacy-v1/99-authoring-guide.md
docs/glossary.md
docs/legacy-v1/glossary.md
docs/rewrite-control/*.md
```

### 편집할 파일

```text
docs/rewrite-control/CONFLICT-LIST.md
docs/rewrite-control/DOC-OWNERSHIP-MAP.md
docs/rewrite-control/PRESERVE-MOVE-LATER.md
```

### Codex Prompt

```text
You are editing the harness documentation repository.

Do not rewrite canonical docs yet.

Read all current documentation and the rewrite-control files.

Update only:
- docs/rewrite-control/CONFLICT-LIST.md
- docs/rewrite-control/DOC-OWNERSHIP-MAP.md
- docs/rewrite-control/PRESERVE-MOVE-LATER.md

Rules:
- Do not invent new architecture decisions.
- Mark unresolved decisions as TODO_DECISION.
- For each concept, assign exactly one owner document.
- Mark content as preserve, rewrite, move-to-appendix, later, or delete.
- Do not edit root README.md, docs/README.md, or main docs in this batch.

Output a summary:
1. conflicts confirmed
2. new conflicts found
3. ownership changes proposed
4. questions marked TODO_DECISION
```

### Acceptance Criteria

```text
[ ] No main docs edited.
[ ] Every major concept has one owner.
[ ] Known conflicts C-01 through C-25 are present or intentionally resolved.
[ ] No new architecture decision is made silently.
```

## 4. Batch A — Target Structure Skeleton

### 목표

Full rewrite 없이 새 target document structure를 만든다.

### 만들거나 이름을 바꿀 파일

```text
docs/00-introduction.md
docs/03-kernel-spec.md
docs/04-runtime-architecture.md
docs/05-mcp-api-and-schemas.md
docs/06-reference-mvp.md
docs/07-document-projection.md
docs/08-design-quality-policy-pack.md
docs/09-agent-integration.md
docs/10-user-guide.md
docs/11-operations-and-conformance.md
docs/appendix/A-template-library.md
docs/appendix/B-surface-cookbook.md
docs/appendix/C-later-roadmap.md
docs/appendix/D-migration-notes.md
```

### 편집할 파일

```text
docs/README.md
```

Skeleton file이 존재할 때만 doc list를 update한다.

### Codex Prompt

```text
Create the new target document structure from docs/rewrite-control/TARGET-DOC-TREE.md.

Do not fully rewrite content yet.
Create skeleton files with:
- title
- document role
- owns
- does not own
- section headings
- TODO_REWRITE or TODO_CONTENT sections where source content will be migrated later

Do not use TODO_IMPLEMENT for generic skeleton placeholders.
Reserve TODO_IMPLEMENT for implementation-critical gaps found in Batch G.
Reserve TODO_DECISION for unresolved architecture decisions.

Update docs/README.md document list to reference the new tree only after skeleton files exist.

Do not edit root README.md. Do not delete old files unless asked. If both old and new files coexist, add migration notes in docs/appendix/D-migration-notes.md.
```

### Acceptance Criteria

```text
[ ] New files exist.
[ ] Each file states its owner role.
[ ] docs/README.md references new tree.
[ ] No full content rewrite yet.
[ ] Generic skeleton placeholders use TODO_REWRITE or TODO_CONTENT, not TODO_IMPLEMENT.
[ ] Appendix directory exists.
```

## 5. Batch B — Strategy and Kernel

### 목표

Strategy를 rewrite하고 implementable kernel spec을 만든다.

### 편집할 파일

```text
docs/02-strategy.md
docs/03-kernel-spec.md
docs/glossary.md
```

### 읽을 파일

```text
docs/rewrite-control/KERNEL-DECISIONS.md
docs/rewrite-control/DOC-OWNERSHIP-MAP.md
docs/rewrite-control/CONFLICT-LIST.md
docs/01-project-charter.md
```

편집 전에 listed source file에서 current pre-rewrite content를 읽는다.

```text
docs/legacy-v1/02-strategy.md
docs/legacy-v1/04-reference-implementation.md
docs/legacy-v1/glossary.md
```

해당 file이 이미 replaced 또는 archived되었으면 `docs/legacy-v1/`, git history, `docs/appendix/D-migration-notes.md`를 사용한다.

### Codex Prompt

```text
Rewrite 02-strategy.md and 03-kernel-spec.md according to KERNEL-DECISIONS.md.

Rules:
- 02-strategy.md owns why, failure model, 7 core invariants, policy defaults, human judgment model, source-of-truth principle summary, guarantee levels summary, MVP boundary.
- 03-kernel-spec.md owns entity model, lifecycle, gates, state compatibility matrix, transition table, close semantics, waiver semantics, prepare_write decision algorithm, close_task decision algorithm.
- Move design-quality details to references to 08-design-quality-policy-pack.md.
- Do not include MCP request/response schemas.
- Do not include SQLite DDL.
- Do not include full templates.
- Update glossary terms for Gate, Scope Gate, Approval Gate, Evidence Gate, Verification Gate, QA Gate, Acceptance Gate, Close Reason, Waiver, Guarantee Level.

Use exact decisions:
- event log = state.sqlite.task_events
- scope gate != approval gate
- evidence_gate = not_required | none | partial | sufficient | stale | blocked
- evidence_gate=not_required means evidence does not apply
- evidence_gate=none means evidence is required but absent
- verification waiver != detached_verified
- EVAL verdict alone does not upgrade assurance
- same-session review cannot produce detached_verified
- qa_gate is canonical; manual_qa_record.result is record-level
- MVP has no first-class capability_gate
- User Notes authority is three-step
- Domain Language canonical source = domain_terms
- core invariants = 7 approved items
```

### Acceptance Criteria

```text
[ ] Strategy no longer owns detailed state machine.
[ ] Kernel spec has lifecycle + gates.
[ ] Kernel spec evidence_gate includes not_required and distinguishes it from none.
[ ] Kernel spec has impossible/invalid state combinations.
[ ] Kernel spec has close_task algorithm.
[ ] Kernel spec has prepare_write algorithm.
[ ] Glossary contains new gate and guarantee terms.
[ ] No MCP wire schema appears in these files.
[ ] No SQLite DDL appears in these files.
```

## 6. Batch C — Runtime, API, Reference MVP

### 목표

Architecture, MCP schema, reference implementation을 닫힌 implementation layer로 분리한다.

### 편집할 파일

```text
docs/04-runtime-architecture.md
docs/05-mcp-api-and-schemas.md
docs/06-reference-mvp.md
docs/appendix/C-later-roadmap.md
docs/glossary.md
```

### Codex Prompt

```text
Rewrite runtime architecture, MCP API, and reference MVP docs.

Rules:
- 04-runtime-architecture.md owns three spaces, runtime layers, state transaction flow, artifact store architecture, projection/reconcile flow, guarantee levels, failure/recovery flow overview.
- 05-mcp-api-and-schemas.md owns MCP resources/tools, common envelope, tool request/response schemas, error codes, idempotency, validator result schema, artifact ref schema.
- 06-reference-mvp.md owns MVP-0 to MVP-5 implementation sequence, SQLite DDL draft, artifact directory layout, baseline capture, projection job table, validator runner skeleton, reference surface behavior.
- Move dashboard, browser QA capture, cross-surface verify, native hook expansion, analytics, parallel orchestration to docs/appendix/C-later-roadmap.md.
- Capability is not a first-class kernel gate in MVP; represent it through surface_capability_check validator, prepare_write blocked_reasons, and guarantee level display.
- Raw artifacts are durable evidence files; RUN-SUMMARY, EVAL, TDD-TRACE, MANUAL-QA, EVIDENCE-MANIFEST, and DIRECT-RESULT are projections/records, not raw artifacts by default.

Required API details:
- tool purpose
- allowed actor
- request schema
- response schema
- state transition summary
- events emitted
- projection jobs enqueued
- validators run
- possible errors
- idempotency behavior

Required error taxonomy includes:
STATE_CONFLICT, NO_ACTIVE_TASK, NO_ACTIVE_CHANGE_UNIT, SCOPE_REQUIRED, SCOPE_VIOLATION, APPROVAL_REQUIRED, APPROVAL_DENIED, APPROVAL_EXPIRED, CAPABILITY_INSUFFICIENT, MCP_UNAVAILABLE, EVIDENCE_INSUFFICIENT, VERIFY_NOT_DETACHED, QA_REQUIRED, ACCEPTANCE_REQUIRED, PROJECTION_STALE, RECONCILE_REQUIRED, ARTIFACT_MISSING, BASELINE_STALE, VALIDATOR_FAILED.
```

### Acceptance Criteria

```text
[ ] Architecture uses state.sqlite.task_events phrasing.
[ ] API doc has schemas for every public MCP tool.
[ ] API doc has error taxonomy.
[ ] API doc has validator result schema.
[ ] Reference MVP has SQLite DDL draft.
[ ] Reference MVP has MVP-0 to MVP-5 sequence.
[ ] Later automation moved to docs/appendix/C.
```

## 7. Batch D — Projection, Policy, Integration

### 목표

Projection contract, design-quality policy, connector contract를 분리한다.

### 편집할 파일

```text
docs/07-document-projection.md
docs/08-design-quality-policy-pack.md
docs/09-agent-integration.md
docs/appendix/A-template-library.md
docs/appendix/B-surface-cookbook.md
docs/glossary.md
```

### Codex Prompt

```text
Rewrite projection, design-quality policy pack, and agent integration docs.

Rules:
- 07-document-projection.md owns projection principles, authority matrix, managed block rules, human-editable rules, artifact reference rules, required MVP templates, optional template summary, stale/freshness rules.
- Correct authority matrix:
  - User Notes: human-editable input → reconcile_items → accepted state event/record
  - Domain Language: domain_terms table → DOMAIN-LANGUAGE projection
  - Module Map: module_map_items table → MODULE-MAP projection
  - Interface Contract: interface_contracts table → INTERFACE-CONTRACT projection
- Markdown reports are projections from state records and artifact refs, not raw artifacts by default.
- User-facing cards may say Manual QA: pending/passed/failed/waived, but qa_gate remains the canonical kernel gate.
- Move full templates to docs/appendix/A-template-library.md.
- 08-design-quality-policy-pack.md must use policy contract format: applies_when, default_requirement, allowed_waiver, required_record, validator, evidence, close_impact.
- 09-agent-integration.md keeps common contract, capability tier/profile, fallback, reference surface, connector conformance overview.
- Move Codex/Claude/Gemini/Copilot/Cursor addenda to docs/appendix/B-surface-cookbook.md.
```

### Acceptance Criteria

```text
[ ] 07 has required/optional/appendix template tiers.
[ ] Full template text is not in 07.
[ ] 08 uses policy contract format consistently.
[ ] 09 is capability-profile centered.
[ ] Surface-specific details are in docs/appendix/B.
[ ] Authority matrix no longer says domain language record + reconciled doc.
```

## 8. Batch E — User Guide, Operations, Authoring, README

### 목표

새 kernel을 기준으로 user-facing 및 operations-facing doc을 finalize한다.

### 편집할 파일

```text
docs/10-user-guide.md
docs/11-operations-and-conformance.md
docs/README.md
docs/99-authoring-guide.md
docs/appendix/D-migration-notes.md
```

### Codex Prompt

```text
Rewrite user guide, operations/conformance, docs/README.md, and authoring guide.

Rules:
- 10-user-guide.md must be short, conversation-oriented, and not expose DB/API internals.
- User Guide must explain status card, advisor/direct/work, approval/assurance/manual QA/acceptance, resume, evidence insufficiency, verify, QA, acceptance.
- 11-operations-and-conformance.md must use fixture-based conformance.
- Operations owns connect, doctor, projection refresh, reconcile, recover, export, artifact integrity, conformance fixture format.
- Move long-term metrics to docs/appendix/C unless framed clearly as later derived analytics.
- docs/README.md must reflect the new target tree and reader paths.
- Root README.md remains the repository landing page unless explicitly assigned.
- 99-authoring-guide.md must reflect new ownership boundaries, schema ownership, template ownership, core invariant vs policy default separation, MVP/v1/later labels.
```

### Acceptance Criteria

```text
[ ] User Guide is not a schema document.
[ ] Operations has fixture examples.
[ ] docs/README.md tree matches actual files.
[ ] Authoring Guide owner table matches DOC-OWNERSHIP-MAP.md.
[ ] Long-term metrics are not MVP requirements.
```

## 9. Batch H — Legacy File Cleanup

### 목표

v2 doc으로 replaced된 legacy doc이 canonical doc으로 남지 않도록 한다.

### Legacy Files

```text
docs/legacy-v1/00-overview.md
docs/legacy-v1/01-project-charter.md
docs/legacy-v1/02-strategy.md
docs/legacy-v1/03-architecture.md
docs/legacy-v1/04-reference-implementation.md
docs/legacy-v1/05-user-guide.md
docs/legacy-v1/06-agent-integration.md
docs/legacy-v1/07-document-and-artifact-contracts.md
docs/legacy-v1/08-operations-and-conformance.md
docs/legacy-v1/09-design-quality-playbooks.md
docs/legacy-v1/99-authoring-guide.md
docs/legacy-v1/glossary.md
docs/legacy-v1/REWRITE-MANIFEST.md
```

### 편집할 파일

```text
legacy files listed above, only for cleanup
docs/README.md
docs/appendix/D-migration-notes.md
```

### Codex Prompt

```text
Clean up archived legacy docs replaced by the v2 target docs.

Rules:
- Legacy docs replaced by v2 docs must not remain as canonical docs.
- After content migration, remove them, replace them with short migration stubs, or move historical notes to docs/appendix/D-migration-notes.md.
- docs/README.md must not link to legacy docs except migration notes.
- Final consistency grep must scan active canonical docs separately from docs/appendix/D-migration-notes.md.
- Archived migration notes are not active canonical docs.
- Do not edit root README.md unless explicitly asked.
```

### Acceptance Criteria

```text
[ ] Replaced legacy docs are deleted, stubbed, or represented in docs/appendix/D-migration-notes.md.
[ ] docs/README.md does not link to legacy docs except migration notes.
[ ] Active canonical docs are the v2 target docs.
[ ] Final grep excludes archived migration notes from active-doc checks.
[ ] No migrated content remains only in a legacy doc.
```

## 10. Batch F — Cross-Document Consistency QA

### 목표

Rewrite와 legacy cleanup 후 contradiction을 찾아 수정한다.

### 편집할 파일

모든 main doc 가능. 단 contradiction과 reference를 수정하는 경우에만.

### Codex Prompt

```text
Perform cross-document consistency QA using docs/rewrite-control/REVIEW-CHECKLIST.md.

Run grep-like checks for prohibited or suspicious phrases against v2 active canonical docs only:
- "state.sqlite + event log"
- "domain language record + reconciled doc"
- "human-editable 문서 영역 | reconcile item" as a source-of-truth conflict
- "detached verification passed 또는 accepted exception"
- "scope와 approval" where the distinction is not explained
- "all surfaces" near MVP claims
- "dashboard" in main docs
- "browser QA capture" in main docs
- "cross-surface" in main docs

Do not scan replaced legacy docs as active docs.
Legacy docs are checked only for whether they were deleted, stubbed, or represented in docs/appendix/D-migration-notes.md.
Fix only consistency issues.
Do not add new decisions.
Do not treat docs/appendix/D-migration-notes.md as an active canonical doc during final consistency grep.
Summarize all edits.
```

### Acceptance Criteria

```text
[ ] No prohibited source-of-truth phrasing remains in v2 active canonical docs.
[ ] All internal links point to existing files.
[ ] docs/README.md, Authoring Guide, and Ownership Map agree.
[ ] MVP/later boundaries are clear.
[ ] Glossary terms match usage.
[ ] Replaced legacy docs are not treated as active canonical docs.
```

## 11. Batch G — Implementability Review Patch

### 목표

Docs만으로 implementation을 시작할 수 있게 한다.

### Codex Prompt

```text
Review docs from an implementer's perspective.

Verify that the docs define enough to implement:
- state.sqlite creation
- task_events append-only table
- Task creation
- Change Unit creation
- prepare_write allowed/blocked/approval_required/state_conflict
- approval request/decision record
- record_run artifact and evidence record
- launch_verify bundle
- record_eval assurance update
- record_manual_qa QA gate update
- close_task completion decision
- TASK/APR/RUN-SUMMARY/EVAL projection
- projection stale reporting
- reconcile item creation
- conformance fixture execution

If something is missing, add TODO_IMPLEMENT in the correct owner doc.
Do not invent unresolved details unless already decided in KERNEL-DECISIONS.md.
```

### Acceptance Criteria

```text
[ ] Missing implementation details are either specified or marked TODO_IMPLEMENT.
[ ] TODO_DECISION appears only where a true decision is still needed.
[ ] No implementation-critical concept lacks an owner doc.
```

## 12. Commit Strategy

권장 commit:

```text
1. docs: add rewrite control documents
2. docs: create v2 document skeleton
3. docs: rewrite strategy and kernel spec
4. docs: split runtime api and reference mvp
5. docs: split projection policy and integration
6. docs: rewrite user operations and authoring guides
7. docs: clean up legacy documentation files
8. docs: run cross-document consistency QA
9. docs: patch implementability gaps
```
