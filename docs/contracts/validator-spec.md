# 하네스 Validator 계약 v04

## 1. 문서 목적

이 문서는 하네스 validator의 입력, 출력, 판정 기준, 상태 전이에 미치는 영향을 정의한다.

이 문서는 다음을 다룬다.

- 공통 validator input/output
- validator verdict 의미
- hard block 기준
- 기본 validator별 알고리즘
- validator 결과 artifact 등록 방식
- 상태 전이와 projection에 미치는 영향

이 문서는 상태 저장 DDL, MCP tool schema, 사용자 문구를 직접 정의하지 않는다.

## 2. 공통 원칙

Validator는 에이전트가 작성한 설명을 신뢰하지 않고, 상태 저장소, artifact registry, baseline, diff, logs, document projection을 검토한다.

Validator는 다음 중 하나의 verdict를 반환한다.

| verdict | 의미 |
|---|---|
| `passed` | 검토 기준을 충족한다. |
| `failed` | 검토 기준을 충족하지 못한다. |
| `blocked` | 필요한 입력이나 전제가 없어 판정할 수 없고 다음 전이가 위험하다. |
| `warning` | 진행은 가능하지만 사용자 또는 evaluator에게 알려야 한다. |
| `skipped` | 현재 task/mode/profile에는 적용되지 않는다. |

`failed`와 `blocked`는 hard block일 수 있다. hard block 여부는 validator와 호출 맥락에 따라 결정한다.

## 3. 공통 입력

Validator runner는 각 validator에 다음 공통 입력을 제공한다.

```yaml
validator_name: string
trigger:
  tool_name: string
  transition_target: optional string
task:
  task_id: string
  mode: advisor | direct | work
  phase: string
  state_version: integer
  canonical_state: object
change_unit: optional object
run: optional object
baseline: optional object
approvals: object[]
evidence_manifest: optional object
artifacts: object[]
changed_files: string[]
commands: object[]
projection:
  projection_version: integer
  projected_version: integer
  freshness: current | stale | failed | unknown
surface_profile: optional object
reconcile_items: object[]
```

Validator는 DB에 직접 상태 변경을 하지 않는다. Validator runner가 결과를 `validator_runs`에 기록하고, 상태 전이 handler가 그 결과를 반영한다.

## 4. 공통 출력

```yaml
validator_name: string
status: passed | failed | blocked | warning | skipped
hard_block: boolean
summary: string
findings:
  - code: string
    severity: info | warning | error | block
    message: string
    refs: string[]
recommended_state_updates: object
output_artifact: optional artifact_ref
```

`recommended_state_updates`는 직접 적용되지 않는다. 상태 전이 handler가 현재 tool과 state-machine 계약에 따라 적용한다.

## 5. 실행 순서

기본 실행 순서는 다음이다.

1. `surface_capability_check`
2. `baseline_freshness`
3. `changed_paths`
4. `approval_scope`
5. `required_docs_present`
6. `docs_consistency`
7. `bundle_integrity`
8. `evidence_sufficiency`
9. `same_session_verify_guard`
10. `acceptance_review`
11. `reconcile_required`
12. `lint`
13. `test`
14. `build`

특정 tool은 필요한 validator subset만 실행할 수 있다. 그러나 `close_task`는 completion 관련 validator를 반드시 실행한다.

## 6. Validator별 계약

### 6.1 `changed_paths`

#### 목적

변경 파일이 active Change Unit의 허용 경로와 approval scope 안에 있는지 확인한다.

#### 주요 입력

- `changed_files`
- active Change Unit `allowed_paths`
- granted approval `allowed_paths`
- run profile과 evaluator 여부

#### 알고리즘

1. changed file 목록을 artifact, git diff, command capture에서 수집한다.
2. active Change Unit이 있으면 모든 changed file이 `allowed_paths`에 포함되는지 확인한다.
3. approval-required category가 있는 변경은 granted approval의 allowed paths에도 포함되는지 확인한다.
4. evaluator run이면 제품 파일 변경이 없는지 확인한다.
5. generated projection 파일 변경은 제품 파일 변경과 분리해서 판정한다.

#### hard block

다음 경우 hard block이다.

- 제품 파일이 active Change Unit allowed paths 밖에서 변경됨
- evaluator run이 제품 파일을 변경함
- changed file 목록을 얻을 수 없고 write-capable run이 이미 수행됨

#### recommended updates

- hard block이면 `phase=blocked`
- evaluator 제품 파일 변경이면 EVAL verdict는 `blocked`

### 6.2 `required_docs_present`

#### 목적

현재 상태에 필요한 문서 projection 또는 artifact가 있는지 확인한다.

#### 알고리즘

- work task에는 `TASK` projection이 있어야 한다.
- direct 완료에는 `DIRECT-RESULT`가 있어야 한다.
- completed work task에는 `EVAL`이 있어야 한다.
- evidence-based verification에는 `EVIDENCE-MANIFEST`가 있어야 한다.
- approval pending 상태에는 `APR`이 있어야 한다.

#### hard block

- `close_task`에서 required docs가 누락되면 hard block
- 중간 단계에서는 `warning` 또는 `blocked`

### 6.3 `docs_consistency`

#### 목적

문서 projection이 current state와 모순되지 않는지 확인한다.

#### 알고리즘

1. `doc_refs`의 path가 존재하는지 확인한다.
2. front matter의 doc id와 task id가 DB와 일치하는지 확인한다.
3. `projection_version`과 `projected_version`을 비교한다.
4. latest report ref가 artifact registry와 일치하는지 확인한다.
5. active/completed 폴더 위치가 task phase와 맞는지 확인한다.
6. managed block hash가 projector 기록과 맞는지 확인한다.

#### hard block

- `close_task`에서 projection이 failed이고 사용자에게 표시되지 않은 경우
- managed block drift가 unresolved reconcile item으로 존재하는 경우

### 6.4 `bundle_integrity`

#### 목적

verification bundle이 artifact registry와 일치하는지 확인한다.

#### 알고리즘

1. bundle manifest의 artifact refs를 읽는다.
2. 각 artifact id가 registry에 존재하는지 확인한다.
3. 파일의 SHA-256을 다시 계산한다.
4. bundle manifest hash를 canonical JSON으로 계산한다.
5. task id, change unit id, baseline ref가 현재 state와 일치하는지 확인한다.
6. source bundle이 lead long chat history만으로 구성되지 않았는지 확인한다.

#### hard block

- missing artifact
- hash mismatch
- bundle task id mismatch
- bundle 없이 detached verification을 주장하는 경우

### 6.5 `evidence_sufficiency`

#### 목적

acceptance criteria를 검증하고 재개하기에 충분한 evidence가 있는지 확인한다.

#### 알고리즘

1. acceptance criteria snapshot이 존재하는지 확인한다.
2. 각 criterion이 `supported`, `unsupported`, `not_applicable` 중 하나인지 확인한다.
3. `supported` criterion에 하나 이상의 evidence ref가 있는지 확인한다.
4. evidence ref가 artifact registry에 존재하는지 확인한다.
5. changed files가 Change Unit과 연결되어 있는지 확인한다.
6. required approval ref가 evidence manifest에 연결되어 있는지 확인한다.
7. diff, command logs, run summary가 artifact registry에 등록되어 있는지 확인한다.
8. stale conditions를 평가한다.

#### 판정

| 조건 | verdict | evidence_state |
|---|---|---|
| criteria snapshot 없음 | blocked | none |
| 일부 criterion이 unsupported | warning 또는 blocked | partial |
| evidence ref 누락 | blocked | partial |
| baseline drift | blocked | stale |
| 모든 조건 충족 | passed | sufficient |

#### hard block

- work close 시 evidence_state가 `sufficient`가 아님
- record_eval passed 요청에서 supported criterion evidence가 누락됨

### 6.6 `acceptance_review`

#### 목적

검증 결과와 사용자 수용 판단이 분리되어 있는지 확인한다.

#### 알고리즘

1. EVAL verdict를 읽는다.
2. evidence manifest의 unsupported criterion을 확인한다.
3. EVAL `user_followup`과 `blockers_or_rework`를 확인한다.
4. trade-off가 남아 있으면 `acceptance_state=pending`을 권고한다.
5. 사용자가 acceptance를 기록했는지 확인한다.

#### hard block

- high risk work에서 explicit user trade-off가 남았는데 close_task가 요청됨
- acceptance rejected 상태에서 close_task가 요청됨

### 6.7 `approval_scope`

#### 목적

민감 변경이 granted approval의 scope 안에 있는지 확인한다.

#### 알고리즘

1. intended sensitive category를 확인한다.
2. category별 granted approval을 찾는다.
3. approval status가 `granted`인지 확인한다.
4. allowed paths, allowed tools, network targets, secret scopes를 비교한다.
5. baseline ref가 approval 당시와 일치하는지 확인한다.
6. diff envelope 확장 여부를 평가한다.

#### hard block

- approval-required category에 granted approval 없음
- approval denied 또는 expired
- allowed path/tool/network/secret scope 초과
- baseline drift가 approval scope에 영향을 줌

### 6.8 `same_session_verify_guard`

#### 목적

실행자의 자기 검토가 detached verification으로 기록되는 것을 막는다.

#### 알고리즘

1. lead run id와 verify run id를 비교한다.
2. evaluator context가 `fresh_session`, `fresh_worktree`, `sandbox` 중 하나인지 확인한다.
3. evaluator가 write-capable인지 확인한다.
4. evaluator가 product file write 권한을 갖는지 확인한다.
5. source input이 bundle 또는 raw artifacts인지 확인한다.
6. baseline이 evaluator에서 재확인되었는지 확인한다.
7. acceptance criteria와 evidence manifest를 독립 검토했는지 확인한다.

#### hard block

다음 중 하나면 `detached_verified` 승격을 hard block한다.

- 같은 run id 재사용
- 같은 write-capable session 재사용
- bundle 없이 chat history만 전달
- baseline 재확인 없음
- evaluator가 제품 파일 쓰기 권한 유지
- evidence manifest 독립 검토 없음

### 6.9 `surface_capability_check`

#### 목적

현재 surface가 요청된 작업에 필요한 capability tier를 충족하는지 확인한다.

#### 알고리즘

1. surface profile을 읽는다.
2. 작업 risk와 write kind에 필요한 tier를 계산한다.
3. MCP tools/resources/prompts 지원 여부를 확인한다.
4. capture, guard, isolation capability를 확인한다.
5. 부족한 기능에 대한 fallback을 제안한다.

#### hard block

- state-changing 작업에서 MCP tool 호출 불가
- high risk work에서 guard 또는 sidecar fallback 없음
- detached verification에서 fresh run 또는 manual bundle 생성 불가

### 6.10 `reconcile_required`

#### 목적

사람 수정 또는 generated file drift가 현재 상태와 충돌하는지 확인한다.

#### 알고리즘

1. pending reconcile item을 조회한다.
2. item이 active task의 managed state에 영향을 주는지 확인한다.
3. human-editable proposal이 pending decision으로 승격되어야 하는지 확인한다.
4. generated connector drift가 현재 MCP/Skill 동작을 위험하게 만드는지 확인한다.

#### hard block

- active task close에 영향을 주는 managed drift가 unresolved
- connector generated file drift가 MCP 연결 또는 guard 규칙을 무력화함

### 6.11 `baseline_freshness`

#### 목적

baseline 이후 repo state가 바뀌어 evidence나 approval이 stale해졌는지 확인한다.

#### 알고리즘

1. current HEAD/worktree fingerprint를 계산한다.
2. baseline ref의 fingerprint와 비교한다.
3. changed files modified after eval 조건을 확인한다.
4. dependency lockfile hash와 relevant config hash를 비교한다.
5. drift가 있으면 drift가 active Change Unit에 영향을 주는지 평가한다.

#### hard block

- verify 또는 close에서 baseline drift가 active changed files에 영향을 줌
- approval baseline이 변경되었고 scope 재확인이 없음

### 6.12 `lint`, `test`, `build`

#### 목적

프로젝트의 기본 check command를 실행하거나 실행 결과 artifact를 검토한다.

#### 알고리즘

1. `project.yaml`의 default command를 읽는다.
2. command가 실행되었으면 exit code와 logs를 검토한다.
3. command가 실행되지 않았으면 skipped 또는 warning으로 기록한다.
4. failed command는 validator output artifact로 남긴다.

#### hard block

- Change Unit의 done definition이 해당 command 통과를 요구하고 command가 실패함
- command output artifact가 누락되어 판정 불가함

## 7. 상태 전이 영향 요약

| validator | prepare_write | record_change | finish_implementation | record_eval | close_task |
|---|---|---|---|---|---|
| changed_paths | warning | hard block | hard block | hard block for evaluator writes | hard block |
| approval_scope | hard block | hard block | hard block | hard block | hard block |
| evidence_sufficiency | warning | warning | warning/block | hard block if passed | hard block |
| same_session_verify_guard | skipped | skipped | skipped | hard block | hard block |
| docs_consistency | warning | warning | warning | warning | hard block if unresolved |
| reconcile_required | warning/block | warning/block | warning/block | warning/block | hard block |
| baseline_freshness | hard block when stale | warning/block | warning/block | hard block | hard block |

## 8. Validator output artifact

각 validator run은 필요한 경우 다음 artifact를 생성한다.

```yaml
artifact_kind: validator_output
content_type: application/json
retention_class: standard
fields:
  validator_name: string
  status: string
  findings: array
  input_refs: array
  output_refs: array
  recommended_state_updates: object
```

Validator output은 사람이 읽는 report에 요약되고, 원본 JSON은 artifact store에 보관된다.
