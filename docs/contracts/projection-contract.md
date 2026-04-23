# 하네스 Projection과 Reconcile 계약 v04

## 1. 문서 목적

이 문서는 Markdown projection, managed block, human-editable section, projection outbox, freshness, reconcile 알고리즘을 정의한다.

이 문서는 authoritative state schema, MCP tool schema, 문서 템플릿 전문을 직접 정의하지 않는다.

## 2. 기본 원칙

1. 문서는 사람이 읽는 표면이다.
2. authoritative 운영 상태는 상태 저장소가 가진다.
3. front matter는 projection이다.
4. managed 영역은 projector가 갱신한다.
5. human-editable 영역은 사용자 메모와 제안을 보존한다.
6. 사람이 managed 영역을 수정해도 운영 상태가 자동 변경되지 않는다.
7. projection stale과 state current는 분리해서 표시한다.
8. projection 실패는 state rollback이 아니라 projection failure다.

## 3. Projection 대상

기본 projection 대상은 다음이다.

| target_kind | 문서 |
|---|---|
| `task` | `TASK` |
| `approval` | `APR` |
| `decision` | `DEC` |
| `design` | `DESIGN` |
| `run_summary` | `RUN-SUMMARY` |
| `eval` | `EVAL` |
| `direct_result` | `DIRECT-RESULT` |
| `evidence_manifest` | `EVIDENCE-MANIFEST` |
| `agents_rule` | `AGENTS.md` |
| `skill` | surface Skill/playbook |
| `surface_status` | `.harness/agent/surface-status.json` |

## 4. Front matter

Front matter는 최소 식별과 projection version만 가진다.

공통 필드는 다음이다.

```yaml
---
doc_type: task | approval | decision | design | run_summary | eval | direct_result | evidence_manifest
task_id: TASK-0001
projection_version: 12
updated_at: 2026-04-23T10:00:00+09:00
---
```

상세 상태 축은 front matter에 넣지 않고 본문 `Current Summary`에 둔다.

## 5. Managed block

Managed block marker는 다음이다.

```md
<!-- HARNESS:BEGIN managed -->
...
<!-- HARNESS:END managed -->
```

Projector는 이 영역만 자동 갱신한다.

Marker가 없는 문서는 connector manifest의 content hash와 managed range metadata로 관리한다.

## 6. Human-editable section

기본 human-editable section은 다음이다.

```md
## User Notes and Proposals
- 
```

Projector는 이 section을 보존한다.

Reconcile service는 새 항목을 다음 중 하나로 분류할 수 있다.

- user observation
- proposal
- pending decision candidate
- reconcile request
- no-op note

상태 반영은 MCP tool, reconcile decision, operator action으로만 수행한다.

## 7. Projection outbox

상태 변경은 transaction 안에서 projection job을 enqueue한다.

Projection job 필드는 다음이다.

```yaml
job_id: string
task_id: string
target_kind: string
target_path: string
target_version: integer
status: queued | running | succeeded | failed | superseded
attempt_count: integer
last_error: optional string
enqueued_at: timestamp
updated_at: timestamp
```

## 8. Projection update algorithm

Projector는 다음 절차를 따른다.

1. queued job을 가져온다.
2. target task state와 관련 refs를 읽는다.
3. job target version이 현재 task projection version보다 오래되었으면 job을 superseded로 표시한다.
4. 기존 파일을 읽는다.
5. front matter를 parse한다.
6. managed block과 human-editable section을 분리한다.
7. human-editable section을 보존한다.
8. template renderer로 새 managed content를 만든다.
9. 새 managed content의 hash를 계산한다.
10. 기존 managed block이 projector 기록과 다르면 reconcile item을 생성하고 자동 덮어쓰기를 중단한다.
11. front matter를 새 projection version으로 갱신한다.
12. atomic write를 수행한다.
13. `doc_refs`와 `projected_version`을 갱신한다.
14. job을 succeeded로 표시한다.

## 9. Atomic write

Atomic write는 다음 절차를 따른다.

1. 같은 디렉터리에 temp file 생성
2. UTF-8 content write
3. fsync 가능하면 수행
4. rename으로 교체
5. 실패하면 temp file 제거 또는 recovery 대상으로 기록

## 10. Projection freshness

Projection freshness는 다음 값 중 하나다.

| freshness | 의미 |
|---|---|
| `current` | projected_version == projection_version이고 마지막 job 성공 |
| `stale` | state가 최신이지만 문서 반영이 늦음 |
| `failed` | projection job이 실패함 |
| `unknown` | projection 상태를 확인할 수 없음 |

에이전트는 사용자에게 state와 projection freshness를 분리해서 보고한다.

## 11. Managed drift detection

Managed drift는 다음 조건에서 발생한다.

- managed block hash가 projector가 기록한 hash와 다름
- front matter의 task id 또는 doc id가 DB와 다름
- generated connector file의 managed block hash가 manifest와 다름
- projection path가 DB의 doc_refs와 다름

Managed drift 처리 절차는 다음이다.

1. reconcile item 생성
2. projection job은 failed 또는 blocked로 표시
3. 상태는 자동 변경하지 않음
4. 사용자 또는 operator에게 선택지 표시
5. merge, reject, convert-to-note 중 하나로 resolve

## 12. Human note reconciliation

Human-editable section에 새 항목이 있으면 다음 절차를 따른다.

1. 이전 snapshot과 비교해 새 항목을 찾는다.
2. 항목을 observation/proposal/question으로 분류한다.
3. 상태 변경이 필요한 문장은 pending reconcile item을 만든다.
4. 단순 메모는 보존하고 no-op으로 둔다.
5. 명시적 user decision이 필요한 경우 `pending_decision_summary`를 갱신할 수 있다.

Human note는 자동 approval이나 acceptance로 처리하지 않는다.

## 13. Reconcile decision

Reconcile decision은 `harness.user_decision`의 `decision_kind=reconcile_decision`으로 기록한다.

Resolution은 다음이다.

| resolution | 처리 |
|---|---|
| `merge` | 변경을 상태 변경 tool 또는 operator action으로 반영한 뒤 projection 재생성 |
| `reject` | 사람 수정 내용을 폐기하고 projection 재생성 |
| `convert_to_note` | managed 변경을 human-editable note로 이동 |

모든 reconcile decision은 event log에 남긴다.

## 14. Template versioning

Template 변경은 projection contract 변경으로 본다.

Template renderer는 다음 metadata를 남긴다.

```yaml
template_id: task-v04
template_version: 4
renderer_version: string
managed_block_hash: sha256
```

Template version이 바뀌면 관련 문서의 projection job을 enqueue한다.

## 15. Connector generated files

Agent connector가 생성하는 rule, Skill, MCP config도 projection이다.

원칙은 다음이다.

- 가능한 경우 managed marker를 사용한다.
- marker가 불가능한 파일은 connector manifest hash로 관리한다.
- 수동 수정은 덮어쓰기 전에 drift로 표시한다.
- connector drift가 guard나 MCP 연결을 약화하면 작업을 blocked 또는 warning으로 표시한다.

## 16. Projection failure 처리

Projection 실패는 다음을 포함한다.

- 파일 쓰기 실패
- template render 실패
- managed drift로 인한 자동 갱신 중단
- front matter parse 실패
- path permission 문제

실패 시 처리:

1. projection job status를 failed로 설정
2. last_error 기록
3. task projection freshness를 failed로 설정
4. status card에 표시
5. retry 또는 reconcile 필요 여부를 next action으로 제공

## 17. Close와 projection

`work` task close는 projection이 current이면 바로 진행할 수 있다.

Projection이 stale이지만 state가 current이고 사용자에게 stale 상태가 표시되었으면 close를 허용할 수 있다.

Projection이 failed이고 latest report나 EVAL을 읽을 수 없으면 close를 차단한다.
