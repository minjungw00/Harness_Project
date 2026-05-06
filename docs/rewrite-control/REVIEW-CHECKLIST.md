# Review Checklist

이 문서는 재작성된 하네스 문서 세트를 검토하기 위한 기준이다. 각 Codex batch 후 이 체크리스트로 semantic QA를 수행한다.

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
[ ] core invariant가 7개로 유지된다.
[ ] policy default와 core invariant가 섞이지 않는다.
[ ] event log 위치가 state.sqlite.task_events로 명확하다.
[ ] lifecycle + gates 상태 모델이 있다.
[ ] 상태 전이표와 불가능 조합이 있다.
[ ] close_task 알고리즘이 있다.
[ ] prepare_write 알고리즘이 있다.
[ ] approval gate와 scope gate가 분리되어 있다.
[ ] detached verification waiver가 detached_verified로 표시되지 않는다.
[ ] User Notes authority가 3단계로 정리되어 있다.
[ ] Domain Language canonical source가 domain_terms table로 정리되어 있다.
[ ] Module Map canonical source가 module_map_items table로 정리되어 있다.
[ ] Interface Contract canonical source가 interface_contracts table로 정리되어 있다.
[ ] guarantee level이 cooperative/detective/preventive/isolated로 표시되어 있다.
[ ] MCP tool별 request/response schema가 있다.
[ ] error code taxonomy가 있다.
[ ] validator result schema가 있다.
[ ] SQLite DDL 초안이 있다.
[ ] artifact schema와 redaction rule이 있다.
[ ] projection job schema가 있다.
[ ] required/optional/appendix template가 분리되어 있다.
[ ] connector 문서는 capability profile 중심이다.
[ ] surface별 세부는 docs/appendix에 있다.
[ ] User Guide는 짧고 대화 중심이다.
[ ] Operations conformance는 fixture 기반이다.
[ ] docs/README.md의 읽기 경로가 새 문서 구조와 일치한다.
[ ] Glossary의 모든 공식 용어가 새 모델과 일치한다.
[ ] later 기능이 MVP 본문을 흐리지 않는다.
[ ] legacy docs replaced by v2 docs no longer remain as canonical docs.
```

## 2. Source-of-Truth Checks

### Required Statements

다음 의미가 모든 문서에서 일관되어야 한다.

```text
운영 상태의 canonical source는 state.sqlite이다.
state.sqlite는 current state table과 append-only task_events table을 가진다.
문서는 사람이 읽는 projection이다.
Raw evidence의 canonical source는 artifact store다.
Raw artifacts are durable evidence files in the artifact store.
State records are canonical structured records in state.sqlite.
Markdown reports are projections generated from records and artifact refs.
Human-editable 영역은 입력 표면이다.
상태 반영은 MCP tool 또는 reconcile action을 통해 수행한다.
```

### Prohibited or Suspicious Phrases

다음 표현이 있으면 검토한다.

```text
state.sqlite + event log
```

허용 조건:

```text
같은 문장 또는 바로 다음 문장에서 event log가 state.sqlite.task_events임을 명확히 해야 한다.
```

금지 표현:

```text
domain language record + reconciled doc
module map records + reconciled doc
사용자 메모 canonical source = human-editable 문서 영역
projection이 상태를 갱신한다
TASK가 canonical state다
RUN-SUMMARY/EVAL/TDD-TRACE/MANUAL-QA/EVIDENCE-MANIFEST/DIRECT-RESULT are raw artifacts
exported projection hash makes a projection canonical raw evidence
```

## 3. State and Gate Checks

```text
[ ] `03-kernel-spec.md`가 lifecycle_phase enum을 소유한다.
[ ] `03-kernel-spec.md`가 gate enum을 소유한다.
[ ] `evidence_gate` enum is `not_required | none | partial | sufficient | stale | blocked`.
[ ] `evidence_gate=not_required` means evidence gate does not apply.
[ ] `evidence_gate=none` means evidence is required but absent.
[ ] `02-strategy.md`에는 gate enum 상세가 없다.
[ ] `10-user-guide.md`에는 사용자가 이해할 수준의 gate 설명만 있다.
[ ] `close_reason`이 정의되어 있다.
[ ] `completed_verified`와 `completed_with_risk_accepted`가 구분된다.
[ ] `verification_gate=waived_by_user`와 `assurance_level=detached_verified` 조합이 금지되어 있다.
[ ] `direct` optional detached verification 규칙이 명확하다.
[ ] Projection stale/failed가 state failure와 구분된다.
[ ] `03-kernel-spec.md`의 canonical gate list에 `capability_gate`가 없다.
[ ] Capability는 `surface_capability_check` validator, `prepare_write` blocked reason, guarantee display로 표현된다.
```

## 4. Scope / Approval Checks

```text
[ ] Scope gate는 모든 write-capable run에 적용된다고 되어 있다.
[ ] Approval gate는 sensitive category가 있을 때 required라고 되어 있다.
[ ] `prepare_write` request는 intended paths/tools/commands/network/secrets/sensitive categories를 받는다.
[ ] Approval request는 allowed paths/tools/network/secrets/baseline/expiry를 가진다.
[ ] approval scope drift는 approval expired 또는 재승인 필요 상태로 표현된다.
[ ] User Guide에서 approval이 검증이나 acceptance를 대체하지 않는다고 설명한다.
```

## 5. Verification / QA / Acceptance Checks

```text
[ ] Work는 same-session self-review를 detached verification으로 인정하지 않는다.
[ ] Verification independence qualifier가 정의되어 있다.
[ ] EVAL verdict alone does not upgrade assurance.
[ ] `assurance_level=detached_verified` requires passed verification and valid independence qualifier.
[ ] Same-session review cannot produce `detached_verified`.
[ ] Manual QA는 verification과 다르다.
[ ] Acceptance는 manual QA와 다르다.
[ ] `qa_gate` is the canonical kernel gate.
[ ] `manual_qa_record.result` is record-level result.
[ ] User-facing cards may say Manual QA: pending/passed/failed/waived.
[ ] QA waived에는 waiver reason이 필요하다.
[ ] Acceptance accepted는 남은 trade-off 수용 판단이다.
[ ] QA failed는 rework 또는 blocked 상태로 이어진다.
```

## 6. MCP API Checks

`05-mcp-api-and-schemas.md`는 모든 public tool에 대해 다음을 제공해야 한다.

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
[ ] It explains “이 작업 하네스 기준으로 진행해.”
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
[ ] Human-editable 영역
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
grep -R "detached verification passed 또는 accepted exception" .
grep -R "capability_gate" docs/03-kernel-spec.md docs/02-strategy.md docs/05-mcp-api-and-schemas.md
grep -R "RUN-SUMMARY.*raw artifact\\|EVAL.*raw artifact\\|TDD-TRACE.*raw artifact\\|MANUAL-QA.*raw artifact" docs
grep -R "all agent surface" .
grep -R "모든 agent surface" .
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
