# 하네스 Artifact와 Bundle 계약 v04

## 1. 문서 목적

이 문서는 하네스가 남기는 durable artifact의 종류, ID, 저장 위치, hash, redaction, retention, bundle manifest, registry rescan 방식을 정의한다.

이 문서는 상태 전이 자체나 MCP tool schema를 직접 정의하지 않는다.

## 2. Artifact 원칙

Artifact는 나중에 재개, 판정, 추적, 감사를 위해 남기는 durable output이다.

하네스는 다음 원칙을 따른다.

1. artifact는 registry record와 파일을 함께 가진다.
2. artifact 파일은 SHA-256 hash로 식별 가능해야 한다.
3. raw logs, patch 전문, command output은 Markdown 문서에 직접 넣지 않고 artifact로 참조한다.
4. raw secret 값은 artifact, DB, 문서, trace에 저장하지 않는다.
5. artifact가 누락되면 evidence는 stale 후보가 된다.
6. bundle은 evaluator와 후속 run의 기본 입력이다.

## 3. Artifact ID 규칙

Artifact ID는 다음 형식을 사용한다.

```text
{PREFIX}-{YYYYMMDD}-{HHMMSS}-{RAND6}
```

예시:

```text
DIFF-20260423-103015-A1B2C3
LOG-20260423-103016-D4E5F6
BND-20260423-103018-G7H8I9
```

권장 prefix는 다음이다.

| Prefix | 의미 |
|---|---|
| `BASE` | baseline snapshot |
| `DIFF` | diff artifact |
| `LOG` | command or run log |
| `CHK` | checkpoint |
| `BND` | verification bundle |
| `RUN` | run summary document artifact |
| `EVAL` | eval document artifact |
| `DIR` | direct result artifact |
| `EM` | evidence manifest artifact |
| `APR` | approval document artifact |
| `DEC` | decision document artifact |
| `VAL` | validator output |
| `EXP` | export |

Task, approval, decision, run, eval의 logical id는 별도 prefix를 가질 수 있다. Artifact ID는 registry에서 유일해야 한다.

## 4. 저장 위치

참조 구현은 다음 layout을 사용한다.

```text
~/.harness/projects/{project_id}/artifacts/
  baselines/{task_id}/{artifact_id}.json
  bundles/{task_id}/{artifact_id}/bundle.json
  bundles/{task_id}/{artifact_id}/files/...
  checkpoints/{task_id}/{artifact_id}.json
  diffs/{task_id}/{artifact_id}.patch
  logs/{task_id}/{artifact_id}.log
  manifests/{task_id}/{artifact_id}.json
  reports/{task_id}/{artifact_id}.md
  validator-output/{task_id}/{artifact_id}.json
  exports/{task_id}/{artifact_id}.zip
```

저장소 문서 projection은 artifact store 밖에 있을 수 있다. 그러나 projection 파일도 필요하면 artifact registry에 `task_doc`, `run_summary_doc`, `eval_doc` 같은 kind로 등록할 수 있다.

## 5. Artifact record

Artifact registry record는 다음 필드를 가진다.

```yaml
artifact_id: string
task_id: optional string
run_id: optional string
kind: string
path: string
sha256: string
size_bytes: integer
content_type: string
retention_class: short | standard | long | permanent
redaction_state: not_needed | redacted | blocked_sensitive
metadata: object
created_at: timestamp
```

`path`는 하네스 홈 기준 상대 경로로 저장하는 것을 권장한다.

## 6. Hash 규칙

### 6.1 파일 hash

파일 artifact의 `sha256`은 파일 bytes를 그대로 사용해 계산한다.

### 6.2 Canonical JSON hash

JSON manifest의 hash는 다음 canonicalization을 적용한다.

1. UTF-8 encoding
2. object key lexical sort
3. insignificant whitespace 제거
4. timestamp precision 보존
5. array order 보존

### 6.3 Bundle hash

Bundle hash는 `bundle.json` canonical hash와 포함 파일 hash 목록을 결합해 계산한다.

```text
sha256(canonical(bundle.json) + "\n" + sorted(file_hash_lines))
```

## 7. Artifact kind

| kind | content type | retention | 설명 |
|---|---|---|---|
| `baseline` | `application/json` | standard | run 시작 상태 |
| `diff` | `text/x-patch` | standard | 변경 diff |
| `log` | `text/plain` | short | command 또는 run output |
| `checkpoint` | `application/json` | standard | 중단/재개 snapshot |
| `bundle` | `application/json` 또는 directory | long | verification input |
| `command_result` | `application/json` | short | command, exit code, log ref |
| `validator_output` | `application/json` | standard | validator findings |
| `task_doc` | `text/markdown` | long | TASK projection snapshot |
| `approval_doc` | `text/markdown` | long | APR projection snapshot |
| `decision_doc` | `text/markdown` | long | DEC projection snapshot |
| `run_summary_doc` | `text/markdown` | long | RUN-SUMMARY |
| `eval_doc` | `text/markdown` | long | EVAL |
| `direct_result_doc` | `text/markdown` | standard | DIRECT-RESULT |
| `evidence_manifest_doc` | `text/markdown` | long | EVIDENCE-MANIFEST |
| `export` | `application/zip` | long | task export |

## 8. Baseline artifact

Baseline artifact는 다음 정보를 가진다.

```yaml
baseline_ref: BASE-...
task_id: TASK-...
run_id: RUN-...
head_ref: string
branch_ref: string
worktree_ref: string
has_uncommitted_changes: boolean
file_hash_summary:
  path: sha256
lockfile_hash: optional string
config_hash: optional string
captured_at: timestamp
```

Baseline은 approval scope, evidence freshness, verification bundle의 기준이다.

## 9. Command result artifact

Command result는 다음 구조를 가진다.

```yaml
command_id: CMD-...
run_id: RUN-...
command: string
cwd: string
started_at: timestamp
finished_at: timestamp
exit_code: integer
stdout_log_ref: optional artifact_id
stderr_log_ref: optional artifact_id
redaction_applied: boolean
```

Command string에는 secret 값이 포함되지 않아야 한다.

## 10. Verification bundle

### 10.1 Bundle 목적

Bundle은 evaluator가 lead run의 긴 대화 이력 없이 검증할 수 있게 하는 구조화된 입력이다.

### 10.2 Bundle 최소 내용

```yaml
bundle_id: BND-...
bundle_version: harness-verify-bundle-v04
task_id: TASK-...
mode: work
phase_at_creation: verifying
active_change_unit_id: CU-01
acceptance_criteria_snapshot:
  - id: AC-01
    statement: string
    status: pending
current_summary_snapshot: object
rolling_spine_snapshot: object
allowed_paths: []
changed_files: []
baseline_ref: BASE-...
run_summary_ref: RUN-...
evidence_manifest_ref: EM-...
diff_ref: DIFF-...
command_results:
  - command_id: CMD-...
    log_refs: []
related_decisions: []
related_approvals: []
known_issues: []
next_checks: []
surface_capability_metadata: object
artifact_hashes:
  artifact_id: sha256
created_at: timestamp
created_by_run: RUN-...
```

### 10.3 Bundle 생성 규칙

1. current task state를 snapshot한다.
2. active Change Unit을 snapshot한다.
3. acceptance criteria를 snapshot한다.
4. evidence manifest와 artifact refs를 수집한다.
5. diff, logs, run summary, approval, decision을 연결한다.
6. artifact hash를 계산한다.
7. `bundle_integrity` validator를 실행한다.
8. bundle artifact를 registry에 등록한다.

### 10.4 Bundle 금지 사항

Bundle은 lead run의 긴 chat history를 primary evidence로 삼지 않는다.

Bundle에는 raw secret, access token, private key, password, session cookie를 포함하지 않는다.

## 11. Redaction 정책

Artifact 저장 전 다음 값을 redaction 대상으로 본다.

- access token
- API key
- password
- private key
- session cookie
- raw secret env value
- 주민등록번호, 여권번호, 계좌번호 같은 고위험 PII
- production credential

Redaction 결과는 다음 중 하나다.

| state | 의미 |
|---|---|
| `not_needed` | 민감 값이 감지되지 않음 |
| `redacted` | 민감 값이 치환되어 저장됨 |
| `blocked_sensitive` | 안전하게 저장할 수 없어 artifact 등록 차단 |

Redaction으로 인해 검증이 불가능하면 evidence는 partial 또는 blocked로 남긴다.

## 12. Retention 정책

기본 retention class는 다음이다.

| class | 기본 보존 |
|---|---|
| `short` | 30일 |
| `standard` | 90일 |
| `long` | 180일 |
| `permanent` | 명시적 삭제 전까지 |

보존 기간 만료 후 삭제된 artifact는 registry record를 제거하지 않고 `missing` 또는 metadata status로 표시한다. 관련 evidence는 stale 후보가 된다.

## 13. Registry rescan

Artifact registry rescan은 다음 절차를 따른다.

1. registry의 artifact path를 순회한다.
2. 파일 존재 여부를 확인한다.
3. 존재하는 파일의 hash를 다시 계산한다.
4. hash mismatch 또는 missing을 기록한다.
5. task별 evidence manifest를 stale 후보로 표시한다.
6. 필요한 경우 recovery event를 append한다.

Rescan은 artifact를 임의로 복구하지 않는다.

## 14. Export

Task export는 다음을 포함한다.

- TASK projection snapshot
- related APR, DEC, DESIGN
- RUN-SUMMARY
- EVAL
- DIRECT-RESULT
- EVIDENCE-MANIFEST
- bundle manifest
- diff/log/checkpoint refs
- artifact registry subset
- conformance-relevant event log subset

Export는 사용자가 작업 흐름을 다시 읽거나 다른 환경에서 검증할 수 있게 하는 보조 산출물이다.
