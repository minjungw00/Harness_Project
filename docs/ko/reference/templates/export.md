# EXPORT Template

## 사용 시점

리뷰, 보관, 마이그레이션, Release Handoff를 위한 선택적 export/보고서 projection을 만들 때 `EXPORT`를 사용합니다.

## 기준 기록

- 포함된 Task와 gate 기록
- Change Unit
- Run
- approval
- Evidence Manifest
- Eval 기록
- Manual QA 기록
- reconcile item
- projection snapshot과 projection 최신성
- artifact 참조, redaction state, retention, integrity metadata

## 렌더링 섹션

- Scope
- State Snapshots
- Projection Snapshots
- Artifact Refs
- Redaction Summary
- Integrity
- Release Handoff

## 전체 템플릿

````md
---
doc_type: export_manifest
export_id: EXPORT-0001
project_id: PRJ-0001
profile: standard | release_handoff
status: complete
source_state_version: 50
updated_at: 2026-05-06T10:30:00+09:00
---

# EXPORT-0001 Harness Export

## Scope
- project_id:
- task_ids:
- included state version range:
- created by:
- created at:

## State Snapshots
- tasks:
- task gates:
- change units:
- runs:
- approvals:
- evidence manifests:
- Eval records:
- Manual QA records:
- reconcile items:

## Projection Snapshots
- TASK:
- APR:
- RUN-SUMMARY:
- EVIDENCE-MANIFEST:
- EVAL:
- DIRECT-RESULT:
- optional design projections:

## Artifact Refs
| Artifact ID | Kind | Owner Record | URI | SHA256 | Redaction State | Retention |
|---|---|---|---|---|---|---|

## Redaction Summary
- secrets omitted:
- redacted artifacts:
- blocked artifacts:

## Integrity
- export hash:
- manifest hash:
- generated at:

## Release Handoff
- close readiness:
- close blockers:
- evidence refs:
- verification refs:
- Manual QA refs:
- residual-risk refs:
- changed files:
- projection freshness:
- redaction notes:
- suggested PR checklist:
- suggested deploy checklist:
- suggested rollback 또는 monitoring notes:
- external authority reminder: Deployment, merge, approval, production monitoring, QA 또는 verification waiver, gate satisfaction, final acceptance, residual-risk acceptance, assurance upgrade, Task close는 이 보고서 밖에 남는다.
````

## 메모

이 template은 렌더링 결과일 뿐 기준 상태가 아닙니다. `EXPORT`는 `ProjectionKind`일 뿐이며, export snapshot과 component는 owner 기록 또는 projection ref에 연결된 artifact로 남습니다.
