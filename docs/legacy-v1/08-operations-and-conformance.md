# 08. Operations and Conformance

## 1. 문서 역할

이 문서는 하네스의 운영 절차와 conformance 기준을 정의한다. Setup, connect, MCP server 운영, doctor, projection refresh, reconcile, recover, export, artifact integrity check, CI, conformance, 운영 지표, 장애 대응을 소유한다.

일반 사용자의 일상 대화 흐름은 `05-user-guide.md`가 소유한다. MCP tool schema는 `04-reference-implementation.md`가 소유한다. Connector 세부 템플릿은 `06-agent-integration.md`가 소유한다.

## 2. 운영 명령의 위치

일상 작업은 대화가 기본이다. CLI는 setup, doctor, recovery, reconcile, export, CI, 운영자용 디버그 표면이다.

대표 명령:

```bash
harness connect agents --auto
harness doctor agents
harness serve mcp
harness reconcile
harness recover TASK-0001
harness export TASK-0001
harness conformance run
```

## 3. Setup / connect

Connect는 Product Repository, Harness Runtime Home, agent surface를 연결한다.

기본 명령:

```bash
harness connect agents --auto
```

명시적 표면 선택:

```bash
harness connect agents --surfaces codex,claude,gemini,copilot,cursor
```

수행 항목:

1. 저장소 루트를 확인한다.
2. 프로젝트를 registry에 등록한다.
3. `project.yaml`을 생성하거나 연결한다.
4. `state.sqlite`와 artifact directory를 준비한다.
5. `AGENTS.md`와 `docs/` 구조를 준비한다.
6. `docs/design/domain-language.md`, `module-map.md`, `interface-contracts.md`를 준비한다.
7. surface별 rule/context 파일을 생성하거나 갱신한다.
8. surface별 Skill 또는 playbook을 생성하거나 갱신한다.
9. Harness MCP server 실행 방법을 등록한다.
10. 가능한 hook, adapter, sidecar 연결을 설정한다.
11. capability profile을 기록한다.
12. design-quality validator availability를 확인한다.
13. conformance smoke test를 실행한다.
14. 사용자가 말할 수 있는 첫 대화 예시를 출력한다.

결과물:

```text
Product Repository
  AGENTS.md
  docs/design/domain-language.md
  docs/design/module-map.md
  docs/design/interface-contracts.md
  .harness/agent/generated/
  .harness/reconcile/pending/

Runtime Home
  ~/.harness/registry.sqlite
  ~/.harness/projects/PRJ-0001/project.yaml
  ~/.harness/projects/PRJ-0001/state.sqlite
  ~/.harness/projects/PRJ-0001/artifacts/
```

## 4. MCP server 운영

MCP server는 agent surface의 기본 조작 API다.

운영자는 다음을 확인한다.

- server process가 실행 중인가
- local stdio 또는 local HTTP 연결이 유효한가
- surface별 MCP config가 현재 server entrypoint를 가리키는가
- tool 목록과 resource 목록이 기대와 일치하는가
- state-changing tool이 idempotency와 state_version을 요구하는가
- resource가 read-only로 동작하는가
- design-quality resource가 read-only로 제공되는가
- design-quality 상태 변경이 tool을 통해 수행되는가

대표 명령:

```bash
harness serve mcp
harness doctor mcp
```

MCP 연결이 없으면 agent surface는 product file write를 보류해야 한다.

## 5. Doctor

Doctor는 하네스 연결과 상태를 진단한다.

```bash
harness doctor agents
harness doctor project
harness doctor mcp
harness doctor artifacts
harness doctor projections
harness doctor design
```

검사 항목:

| 범주 | 검사 |
|---|---|
| project | registry 등록, repo_root fingerprint, project.yaml 유효성 |
| state | state.sqlite 접근, lock 상태, active Task 상태 |
| MCP | server 실행, resource/tool 목록, write tool envelope |
| agent | rule/context, Skill, MCP config, capability profile |
| connector | generated manifest, managed hash, drift |
| artifact | registry와 파일 존재성, hash 검증 |
| projection | projection_jobs, projected_version, stale/failed 상태 |
| design | domain-language, module-map, interface-contracts 존재와 drift |
| validator | 기본 validator와 design-quality validator availability |
| security | network/secret policy 기본값 |

출력 등급:

```text
OK: 정상
WARN: 작업은 가능하지만 보장 수준 낮음
FAIL: 작업 차단 또는 복구 필요
REPAIRABLE: 자동 repair 가능
MANUAL: 사용자 또는 operator 판단 필요
```

## 6. Projection refresh

Projection refresh는 state와 artifact ref를 기준으로 Markdown projection을 다시 생성한다.

```bash
harness projection refresh TASK-0001
harness projection refresh --all-active
harness projection refresh --design
```

원칙:

- 최신 `projection_version`보다 오래된 job은 적용하지 않는다.
- 같은 version의 재실행은 idempotent하다.
- Human-editable 영역은 보존한다.
- Managed 영역 drift가 있으면 reconcile item을 만든다.
- Projection 실패는 state 실패로 간주하지 않는다.
- 완료된 PRD/DESIGN/issue를 기본 context에 다시 push하지 않는다.

## 7. Reconcile

Reconcile은 문서 또는 generated file의 수동 수정과 canonical state의 관계를 정리한다.

```bash
harness reconcile
harness reconcile TASK-0001
harness reconcile --design
```

대상:

- `TASK` managed 영역 직접 수정
- `TASK` human-editable proposal
- `DOMAIN-LANGUAGE` human proposal 또는 managed drift
- `MODULE-MAP` human proposal 또는 managed drift
- `INTERFACE-CONTRACT` proposal
- connector generated file 수동 수정
- projection failed 상태
- stale doc ref
- active/completed 폴더 위치 불일치

처리 선택지:

| 선택지 | 의미 |
|---|---|
| merge | 내용을 상태 변경 또는 문서 projection에 반영 |
| reject | 수동 수정 내용을 폐기하고 projection 재생성 |
| convert-to-note | managed 수정 내용을 human-editable note로 이동 |
| create-decision | proposal을 `DEC` 또는 pending decision으로 승격 |
| create-interface-contract | public interface proposal을 contract로 승격 |
| defer | 나중에 처리하도록 보류 |

상태 반영은 명시적 reconcile action 또는 MCP tool을 통해 수행한다.

## 8. Recover

Recover는 중단되거나 손상된 작업 상태를 복구한다.

```bash
harness recover TASK-0001
harness recover --active
```

복구 시나리오:

| 시나리오 | 처리 |
|---|---|
| write 중 agent crash | active run interrupted, diff/log snapshot artifact 등록 |
| approval 후 baseline drift | approval expired 후보 표시, 재확인 요구 |
| evaluator 중 repo drift | evidence stale, EVAL blocked |
| state와 artifact directory 불일치 | artifact registry rescan, missing artifact stale 표시 |
| TDD trace log 누락 | tdd_trace partial, missing log ref 표시 |
| manual QA artifact 누락 | manual_qa partial 또는 stale 표시 |
| domain-language drift | reconcile item 생성 |
| module-map drift | architecture_state review_required 또는 drift_detected |
| worktree 삭제 또는 branch 변경 | baseline stale, active run blocked |
| MCP 연결 손실 | write 중단, last known state 기준 조회 제한 |
| projection job 실패 | projection failed 표시, refresh/reconcile 안내 |

Recover 출력:

- 복구 대상 Task와 run
- 발견한 drift 또는 손상
- 자동 복구한 항목
- 사용자 판단이 필요한 항목
- 다음 MCP action 또는 운영자 명령

## 9. Export

Export는 Task와 관련 artifact를 묶어 검토, 보관, 외부 전달에 사용할 수 있게 한다.

```bash
harness export TASK-0001
harness export TASK-0001 --include-bundles --include-logs
```

Bundle 포함 후보:

- `TASK` snapshot
- relevant `DOMAIN-LANGUAGE` snapshot
- relevant `MODULE-MAP` snapshot
- `INTERFACE-CONTRACT`
- `RUN-SUMMARY`
- `TDD-TRACE`
- `MANUAL-QA`
- `EVAL`
- `DIRECT-RESULT`
- `APR`
- `DEC`
- `DESIGN`
- `EVIDENCE-MANIFEST`
- diff refs
- logs refs
- checkpoint refs
- bundle manifest
- artifact hash manifest

Raw secret, 민감 로그, PII는 redaction policy를 통과해야 한다.

## 10. Artifact integrity check

Artifact integrity check는 DB record와 파일 시스템의 artifact가 일치하는지 확인한다.

```bash
harness artifacts check TASK-0001
harness artifacts check --all
```

검사 항목:

- artifact file 존재 여부
- sha256 일치 여부
- size_bytes 일치 여부
- task_id/run_id relation 유효성
- retention class 유효성
- redaction 상태
- document ref와 artifact registry 일치 여부
- TDD red/green log ref 존재 여부
- manual QA screenshot/log/note ref 존재 여부

문제가 발견되면 evidence_state를 stale 또는 partial로 바꿀 수 있다.

## 11. CI와 conformance

Conformance suite는 구현과 connector가 핵심 불변식을 만족하는지 확인한다.

```bash
harness conformance run
harness conformance run --surface reference
harness conformance run --suite core
harness conformance run --suite connector
harness conformance run --suite design
```

## 12. Core conformance suite

| 시나리오 | 기대 결과 |
|---|---|
| active Task 없음 상태 조회 | compact status card 반환 |
| advisor 종료 | code write 없이 advice result 기록 |
| direct 성공 | changed_paths, DIRECT-RESULT, self_checked 기록 |
| direct 후 detached verify | fresh evaluator와 EVAL 기록 가능 |
| direct에서 work 전환 | 같은 Task 유지, phase=shaping |
| dependency 추가 요청 | approval 없이는 prepare_write blocked |
| approval scope 밖 수정 시도 | approval_scope fail, approval expired 또는 재승인 필요 |
| work 구현 완료 | RUN-SUMMARY와 evidence manifest 생성, completed 차단 |
| 같은 세션 자기 검토 | detached_verified 불가 |
| fresh evaluator + bundle 검증 | EVAL 생성, assurance 승격 가능 |
| 검증 통과 but trade-off 남음 | acceptance pending |
| projection job 실패 | state current / projection failed 분리 표시 |
| 사용자 TASK 직접 수정 | reconcile pending 감지 |
| repo drift after baseline | evidence stale 또는 verify blocked |
| MCP 없이 Skill만 있음 | product file write 보류 |
| MCP resource read와 tool write 구분 | resource는 read-only, state change는 tool만 가능 |

## 13. Design-quality conformance suite

| 시나리오 | 기대 결과 |
|---|---|
| work 요청 후 즉시 구현 시도 | shaping_incomplete 또는 design_alignment_required |
| unresolved decision 존재 | phase=shaping 또는 waiting_user 유지 |
| domain term 새로 발견 | DOMAIN-LANGUAGE proposal 또는 reconcile item 생성 |
| domain term mismatch | domain_language_consistency warning |
| 첫 기능 CU가 horizontal | vertical_slice_required 또는 horizontal_exception_reason 요구 |
| horizontal exception만 있고 후속 vertical CU 없음 | Change Unit blocked |
| TDD required인데 red evidence 없음 | tdd_trace partial, evidence partial |
| TDD waived | non-TDD justification과 alternate feedback loop 요구 |
| public interface 변경인데 contract 없음 | interface_review_required |
| module boundary 변경인데 DESIGN 없음 | module_boundary_review fail |
| shallow module 증가 위험 감지 | architecture_drift warning |
| UI work인데 manual QA 없음 | manual_qa_required, acceptance pending |
| manual QA failed | rework Change Unit 또는 blocked |
| 완료된 DESIGN을 기본 context에 push | doc_rot_context warning |
| reviewer run에 coding standards 미제공 | reviewer_context_incomplete warning |

## 14. Connector conformance suite

각 surface connector는 다음을 통과한다.

1. rule/context 파일 생성 또는 갱신
2. Harness Skill 또는 playbook 설치
3. MCP 설정 연결
4. capability profile 기록
5. generated manifest 기록
6. generated drift 감지
7. status card 표시
8. shaping prompt 표시
9. prepare_write 호출 유도
10. record_run 호출 유도
11. TDD trace 기록 유도
12. manual QA card 표시
13. approval request와 user decision 처리
14. work 후 verify 요구 표시
15. same-session verify guard 동작
16. projection stale 보고
17. MCP unavailable 시 product write 보류
18. push/pull context 정책 준수

## 15. 운영 지표

운영 지표는 raw event에서 계산한다. 지표는 운영 개선용 파생값이다.

핵심 질문:

- direct가 work로 얼마나 자주 전환되는가
- verify가 missing evidence 때문에 얼마나 자주 막히는가
- approval turnaround는 얼마나 걸리는가
- detached verify latency는 어느 수준인가
- passed 뒤 reopen이 얼마나 발생하는가
- 표면별 capability 부족으로 fallback이 얼마나 발생하는가
- MCP 연결 실패가 얼마나 자주 발생하는가
- projection stale 상태가 얼마나 오래 지속되는가
- reconcile item이 얼마나 자주 발생하는가
- same-session verify guard가 얼마나 자주 동작하는가
- shaping에서 unresolved decision이 얼마나 자주 남는가
- horizontal exception이 얼마나 자주 발생하는가
- TDD required 작업에서 red evidence가 얼마나 자주 누락되는가
- manual QA pending이 얼마나 오래 지속되는가
- architecture drift warning이 얼마나 자주 발생하는가

기본 지표:

```text
direct_to_work_escalation_rate
approval_turnaround_time
verify_latency
reopen_within_7d
evaluator_blocked_due_to_missing_evidence
same_session_verify_guard_triggered
surface_fallback_rate
mcp_connection_failure_rate
projection_stale_duration
reconcile_pending_count
shaping_unresolved_decision_count
horizontal_exception_rate
tdd_red_missing_rate
manual_qa_pending_duration
architecture_drift_warning_count
domain_language_mismatch_count
interface_review_required_count
```

## 16. 장애 시나리오 대응표

| 장애 | 사용자에게 보이는 상태 | 운영 처리 |
|---|---|---|
| MCP server down | MCP 연결 실패, write 보류 | server restart, doctor mcp |
| projection stale | state current / projection stale | projection refresh |
| projection failed | state current / projection failed | failed job inspect, retry, reconcile |
| approval scope drift | approval expired 후보 | 새 APR 또는 scope confirmation |
| evidence insufficient | verify blocked 또는 evidence partial | missing AC/evidence 표시, 추가 check |
| baseline stale | evidence stale 또는 write blocked | baseline recapture, approval 재확인 |
| same-session verify attempt | VERIFY_NOT_DETACHED | fresh evaluator 또는 bundle verify |
| generated file drift | connector drift | diff 표시, merge/reinstall |
| managed doc edited | reconcile required | merge/reject/convert-to-note |
| artifact missing | evidence stale | rescan, restore, mark missing |
| worktree deleted | run blocked | worktree restore 또는 new baseline |
| vertical slice missing | Change Unit blocked | slice 재분해 또는 exception 기록 |
| TDD trace missing | evidence partial | red/green evidence 추가 또는 waiver |
| manual QA missing | acceptance pending 또는 close blocked | QA 수행 또는 waiver |
| interface contract missing | write blocked 또는 verify blocked | IFACE 생성/검토 |
| domain language conflict | design alignment partial | reconcile 또는 DEC 생성 |

## 17. 운영자 체크리스트

새 프로젝트 연결 후 확인:

```text
[ ] registry.sqlite에 project 등록됨
[ ] project.yaml 생성됨
[ ] state.sqlite 생성됨
[ ] artifact directory 준비됨
[ ] AGENTS.md 생성 또는 갱신됨
[ ] docs/ 구조 생성됨
[ ] docs/design/domain-language.md 준비됨
[ ] docs/design/module-map.md 준비됨
[ ] docs/design/interface-contracts.md 준비됨
[ ] 최소 하나의 surface T2 이상 연결됨
[ ] Harness Skill 설치됨
[ ] MCP resource/tool 호출 가능
[ ] compact status card 반환됨
[ ] prepare_write blocked/allowed 테스트 통과
[ ] projection refresh 통과
[ ] design-quality smoke test 통과
[ ] conformance smoke test 통과
```

## 18. 요약

```text
일상 작업은 대화가 기본이다.
CLI는 설치, 진단, 복구, reconcile, export, conformance에 쓴다.
State와 projection의 최신성을 분리해서 보고한다.
장애는 상태와 evidence에 반영하고 숨기지 않는다.
Conformance suite는 core invariant, connector 보장, design-quality 보장을 검증한다.
```

