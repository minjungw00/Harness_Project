# 하네스 구현 가이드 v04

## 1. 문서 목적

이 문서는 하네스 전략을 실제 시스템으로 집행하기 위한 참조 구현 구조를 정의한다.

이 문서는 다음을 다룬다.

- project-independent core 인터페이스
- 로컬 참조 아키텍처
- 상태 변경 처리 방식
- 서비스 경계
- approval, evidence, verification 흐름
- artifact registry와 projection outbox 연동
- adapter, sidecar, connector와의 관계
- 복구와 reconcile 흐름
- 보안 경계와 운영 보고

이 문서는 다음을 직접 다루지 않는다.

- 상위 목표 재정의
- DB DDL 전문
- MCP tool JSON Schema 전문
- 문서 템플릿 전문
- 표면별 실제 제품 버전 기능 보장

## 2. 구현 원칙

참조 구현은 다음 원칙을 따른다.

1. 코어 불변식을 먼저 구현한다.
2. agent surface 수보다 상태 전이, evidence, approval, verification 집행을 우선한다.
3. 일상 조작 API는 Harness MCP server로 제공한다.
4. 상태 변경은 idempotent하고 감사 가능해야 한다.
5. 저장소 문서는 사람이 읽는 projection으로 유지한다.
6. generated file과 사람이 편집한 내용은 reconcile 규칙으로 분리한다.
7. detached verification은 실행자 자기 보고와 구분한다.
8. surface capability는 제품명으로 가정하지 않고 profile로 검증한다.

## 3. project-independent core

하네스의 코어는 다음 인터페이스로 정의한다.

- Task state service
- append-only event log
- artifact registry
- document projector
- document reconcile service
- approval service
- evidence manifest service
- validator runner
- verification launcher
- MCP service
- agent connector service
- adapter interface
- hook and sidecar policy interface
- recovery service

이 코어는 특정 IDE, 특정 CLI, 특정 모델, 특정 파일 레이아웃에 종속되지 않는다.

agent surface는 코어 위에 연결되는 조작 표면이다.

## 4. 참조 아키텍처

하네스는 여섯 계층으로 구현한다.

```text
사용자 대화 표면
  ↓
에이전트 표면
  ↓
Harness Skill / rules / custom agent
  ↓
Harness MCP server
  ↓
Harness Core
  ↓
hook / adapter / sidecar / validator / artifact store
```

### 4.1 사용자 대화 표면

사용자는 에이전트 표면 안에서 자연어로 요청한다.

책임은 다음이다.

- 요청 입력
- 상태 확인
- 승인과 거절
- 결과 수용 또는 거부
- 검증 결과 확인

사용자 대화 표면은 하네스의 source-of-truth가 아니다.

### 4.2 에이전트 표면

에이전트 표면은 작업을 실제로 수행하는 LLM 개발 도구다.

책임은 다음이다.

- 항상 읽히는 rule/context 확인
- Harness Skill 사용
- Harness MCP tool 호출
- 코드 읽기와 수정
- 명령 실행
- 하네스에 evidence 전달
- 사용자에게 상태 카드 보고

### 4.3 Skill / rule / custom agent 계층

이 계층은 에이전트에게 절차를 알려준다.

이 계층은 context와 playbook이다. 정책 집행과 authoritative state update는 이 계층의 책임이 아니다.

### 4.4 Harness MCP server

MCP server는 에이전트가 하네스를 조작하는 기본 API다.

MCP server는 resources, prompts, tools를 제공한다.

이 계층은 CLI 명령의 1:1 remote API가 아니라 상태기계 기반 high-level intent API다.

### 4.5 Harness Core

Core는 다음을 수행한다.

- mode 분류
- Task와 Change Unit 생성
- 상태 전이
- approval 생성과 집행
- evidence manifest 생성과 갱신
- artifact 등록
- validator 실행
- verification run 준비
- projection job 생성
- recovery와 reconcile

### 4.6 집행 계층

집행 계층은 다음을 담당한다.

- allowed path guard
- shell command guard
- network write guard
- secret access guard
- approval scope guard
- product file write guard for evaluator
- same-session verify guard
- artifact capture
- baseline capture
- worktree isolation
- projection freshness monitoring

표면이 native hook을 제공하지 않으면 sidecar와 validator가 보완한다.

## 5. 서비스 경계

### 5.1 Task State Service

Task State Service는 canonical state를 읽고 변경한다.

책임은 다음이다.

- state version 관리
- mode, phase, result, assurance, approval, acceptance, risk, evidence field 갱신
- expected state version conflict 처리
- state transition guard 실행
- task event append
- projection job enqueue

Task State Service는 artifact 파일을 직접 쓰지 않는다. artifact 등록은 Artifact Registry를 통해 수행한다.

### 5.2 Event Log Service

Event Log Service는 append-only 이벤트를 기록한다.

이벤트는 다음 정보를 포함한다.

- event id
- task id
- event type
- actor
- request id
- idempotency key
- state version before/after
- payload
- created at

이벤트는 운영 감사와 conformance 판정의 기준이다.

### 5.3 Approval Service

Approval Service는 approval request와 decision을 관리한다.

책임은 다음이다.

- 민감 범주 판정
- APR 생성
- approval scope 저장
- 사용자 승인, 거절, scope confirmation 기록
- scope drift 감지
- approval 만료 처리

approval은 verification이나 acceptance를 대신하지 않는다.

### 5.4 Evidence Manifest Service

Evidence Manifest Service는 acceptance criteria와 evidence ref의 대응 관계를 관리한다.

책임은 다음이다.

- criteria snapshot 저장
- supporting evidence 연결
- changed file coverage 기록
- stale condition 기록
- evidence_state 산정
- EVIDENCE-MANIFEST projection enqueue

### 5.5 Artifact Registry

Artifact Registry는 durable output을 등록한다.

책임은 다음이다.

- artifact id 발급
- 파일 저장
- SHA-256 hash 계산
- content type과 retention class 기록
- artifact table 갱신
- rescan과 missing artifact 처리

Artifact Registry는 raw secret 값을 저장하지 않는다.

### 5.6 Validator Runner

Validator Runner는 validator를 실행하고 결과를 기록한다.

책임은 다음이다.

- validator profile 해석
- 실행 순서 결정
- validator input bundle 구성
- verdict 저장
- output artifact 등록
- hard block 여부 반환

### 5.7 Verification Launcher

Verification Launcher는 detached verification을 준비한다.

책임은 다음이다.

- verification bundle 생성
- evaluator run 생성
- independence target 기록
- fresh session, fresh worktree, sandbox, manual bundle 중 가능한 방식 선택
- same-session verify guard에 필요한 metadata 기록

### 5.8 Document Projector

Document Projector는 projection outbox job을 처리한다.

책임은 다음이다.

- state snapshot 읽기
- template render
- managed block 갱신
- human-editable section 보존
- front matter 갱신
- atomic write
- projected_version 갱신
- 실패 시 projection_failed 표시

### 5.9 Reconcile Service

Reconcile Service는 사람이 projection 문서 또는 generated file을 수정했을 때 처리한다.

책임은 다음이다.

- managed block drift 감지
- human-editable proposal 감지
- reconcile item 생성
- merge/reject/convert-to-note decision 처리
- 상태 반영이 필요한 경우 MCP tool 또는 operator action으로 연결

### 5.10 Agent Connector Service

Agent Connector Service는 agent surface 설정을 생성하고 유지한다.

책임은 다음이다.

- surface detection
- capability profile 생성
- generated file install/update
- MCP configuration 생성
- Skill/playbook 생성
- connection health check
- generated drift repair
- conformance smoke test 실행

## 6. 상태 변경 트랜잭션

모든 state-changing tool은 하나의 SQLite transaction에서 다음을 수행한다.

1. idempotency key 조회
2. lock 획득
3. expected state version 검증
4. guard 실행
5. snapshot state 갱신
6. event append
7. projection version 증가
8. projection job enqueue
9. idempotency result 저장
10. commit

파일 시스템 쓰기는 commit 이후 수행한다.

projection 실패는 state rollback 사유가 아니다. 대신 projection freshness를 `failed`로 표시하고 retry 또는 reconcile 대상으로 둔다.

## 7. idempotency와 conflict

### 7.1 idempotency

같은 `idempotency_key`와 같은 actor scope로 재시도된 요청은 같은 결과를 반환한다.

저장된 요청 hash와 새 요청 hash가 다르면 `STATE_CONFLICT` 또는 `INVALID_TRANSITION`을 반환한다.

### 7.2 expected state version

`expected_state_version`이 현재 state와 다르면 state-changing tool은 `STATE_CONFLICT`를 반환한다.

에이전트는 최신 `harness.status` 또는 `harness.next`를 읽고 다시 시도해야 한다.

### 7.3 lock

프로젝트 단위 lock을 기본으로 사용한다.

lock timeout은 짧게 유지하고, 획득 실패 시 `LOCK_UNAVAILABLE`을 반환한다.

## 8. baseline 모델

모든 write-capable run은 시작 전에 baseline을 기록한다.

baseline 최소 내용은 다음이다.

- HEAD commit 또는 worktree snapshot
- uncommitted changes 존재 여부
- branch 또는 worktree 식별자
- 시작 시점 파일 해시 요약
- 시작 시각
- dependency lockfile hash
- relevant config hash

baseline은 approval scope, evidence freshness, verification bundle의 기준이다.

## 9. approval 흐름

1. 에이전트가 `harness.prepare_write`를 호출한다.
2. Core가 intended path, tool, sensitive category, baseline을 검사한다.
3. approval이 필요하면 `APPROVAL_REQUIRED`와 approval request guidance를 반환한다.
4. 에이전트가 `harness.request_approval`을 호출해 APR을 생성한다.
5. 사용자가 승인 또는 거절한다.
6. 에이전트가 `harness.user_decision`으로 decision을 기록한다.
7. 승인된 scope 안에서만 write가 허용된다.
8. scope drift가 발생하면 approval은 expired 후보가 되거나 새 approval을 요구한다.

## 10. direct 흐름

1. `harness.intake`가 요청을 direct로 분류한다.
2. `harness.prepare_write`가 scope와 approval을 확인한다.
3. 에이전트가 변경한다.
4. `harness.record_change`가 changed files, diff, logs, command results를 등록한다.
5. 필요한 validator가 실행된다.
6. `harness.finish_direct`가 DIRECT-RESULT를 생성한다.
7. Task는 completed 또는 work escalation 상태가 된다.

`direct`는 detached verification을 요구하지 않을 수 있지만, assurance는 self_checked를 넘지 않는다.

## 11. work 흐름

1. `harness.intake`가 요청을 work로 분류한다.
2. shaping에서 scope, acceptance criteria, Change Unit을 만든다.
3. `harness.prepare_write`가 active Change Unit과 approval을 확인한다.
4. lead run이 구현한다.
5. `harness.record_change`가 evidence 후보를 등록한다.
6. `harness.update_evidence_manifest`가 criteria coverage를 기록한다.
7. `harness.finish_implementation`이 RUN-SUMMARY를 생성하고 phase를 verifying으로 전환한다.
8. `harness.launch_verify`가 fresh evaluator 또는 manual verify bundle을 만든다.
9. evaluator가 evidence를 검토하고 `harness.record_eval`을 호출한다.
10. 통과 후 acceptance가 필요하면 waiting_user로 둔다.
11. acceptance가 resolved되면 `harness.close_task`가 completion 조건을 확인한다.

## 12. verification 흐름

Verification은 lead run의 자기 보고가 아니라 독립된 판정이다.

Evaluator run은 다음 원칙을 따른다.

- fresh run 사용
- 가능하면 fresh environment 또는 분리된 worktree 사용
- source bundle 명시
- lead의 긴 대화 이력은 primary evidence로 사용하지 않음
- 자체적으로 changed files와 baseline 상태를 다시 확인
- 필요한 validator 재실행
- 제품 코드 쓰기 권한 없음

same-session self-review는 detached verification으로 기록하지 않는다.

## 13. artifact와 projection 연동

상태 변경과 artifact 등록은 다음 순서로 처리한다.

1. artifact 파일을 임시 경로에 쓴다.
2. hash와 size를 계산한다.
3. artifact record를 transaction 안에서 등록한다.
4. 관련 task state와 event payload에 artifact id를 연결한다.
5. projection job이 artifact ref를 Markdown에 표시한다.

artifact 파일이 누락되면 registry rescan이 missing 상태를 감지하고 evidence_state를 stale 후보로 만든다.

## 14. document authority와 reconcile

Markdown 문서는 사람이 읽는 projection이다.

managed 영역은 projector가 관리한다.

human-editable 영역은 사용자 메모와 제안을 받는다.

사람이 managed 영역을 수정하면 하네스는 자동으로 운영 상태를 바꾸지 않는다. reconcile item을 만들고 사용자 또는 operator decision을 요구한다.

## 15. security 경계

하네스는 파일 범위만이 아니라 여러 경계를 집행한다.

### 15.1 filesystem scope

- allowed paths
- read-only versus write-capable run 구분
- evaluator의 제품 코드 쓰기 금지

### 15.2 process scope

- shell 명령 실행 정책
- 허용된 도구와 스크립트 범위
- worktree 또는 프로세스 격리

### 15.3 network scope

- 기본은 no network write
- 외부 변경은 approval과 allowlist 필요
- verify run은 기본적으로 network write 금지

### 15.4 credential scope

- env allowlist
- secret handle만 기록
- raw secret 값은 문서, DB, trace, log에 저장하지 않음
- secret access는 별도 approval과 격리된 실행 정책을 가짐

### 15.5 data and privacy scope

- PII 처리 변경은 approval 필요
- data export는 approval 필요
- telemetry/logging 변경은 approval 필요
- evidence artifact에 민감 데이터가 들어가지 않도록 redaction 적용

### 15.6 untrusted input 원칙

다음은 모두 검증 대상 입력이다.

- 사용자 자유 입력
- 저장소 내 임의 문서
- 외부 검색 결과
- MCP 또는 connector 응답
- 모델 초안
- local LLM 응답
- 도구 출력
- agent surface memory
- generated rule/skill files

prompt injection 대응은 prompt 문구가 아니라 policy engine과 tool boundary에서 집행한다.

## 16. adapter, hook, sidecar

### 16.1 adapter 책임

adapter는 agent surface와 제어 계층 사이의 번역기다.

책임은 다음이다.

- instruction injection
- run launch
- profile mapping
- MCP session association
- stdout and stderr capture
- structured output capture
- artifact registration support
- approval UX 연결
- fresh verify entrypoint 제공
- generated file install/update
- capability declaration
- connector drift repair

### 16.2 hook 책임

hook은 surface lifecycle에 붙어 사전/사후 정책을 집행한다.

권장 hook 지점은 다음이다.

- session start
- user prompt submit
- pre tool use
- post tool use
- stop
- pre compact 또는 context reset

hook이 없는 표면은 sidecar watcher가 보완한다.

### 16.3 sidecar 책임

sidecar는 표면 독립적인 집행 보완 계층이다.

책임은 다음이다.

- filesystem watch
- git diff sampling
- command wrapper
- artifact capture
- MCP server process supervision
- stale projection detection
- worktree setup
- evaluator isolation
- generated file drift detection

sidecar는 사용자 조작면이 아니라 운영자/adapter 조작면이다.

## 17. 복구와 reconcile 시나리오

참조 구현은 다음 시나리오를 처리한다.

| 시나리오 | 처리 |
|---|---|
| write 중 agent crash | active run을 interrupted로 표시하고 diff/log snapshot을 artifact로 등록 |
| approval granted 후 baseline drift | approval_state를 expired 후보로 표시하고 재확인 요구 |
| evaluator 검증 중 repo drift 발견 | evidence_state를 stale로 바꾸고 EVAL verdict를 blocked로 기록 |
| 사용자가 TASK managed 영역 직접 수정 | reconcile item 생성, 자동 state 변경 금지 |
| 사용자가 User Notes에 제안 추가 | pending decision 또는 user observation으로 승격 가능 |
| state와 artifact directory 불일치 | artifact registry 재스캔, missing artifact를 stale로 표시 |
| connector generated file 수동 수정 | generated drift 표시 후 merge/reinstall 선택 요구 |
| worktree 삭제 또는 branch 변경 | baseline stale, active run blocked |
| projection job 실패 | state current / projection failed를 분리 표시 |
| MCP 연결 손실 | write 중단, 상태 조회는 last known state로 제한 |

## 18. 운영 보고와 메트릭

운영 지표는 raw event에서 계산한다.

핵심 질문은 다음이다.

- direct가 work로 얼마나 자주 전환되는가
- verify가 missing evidence 때문에 얼마나 자주 막히는가
- approval turnaround는 얼마나 걸리는가
- detached verify latency는 어느 수준인가
- passed 뒤 일정 기간 내 reopen이 얼마나 발생하는가
- 표면별 capability 부족으로 fallback이 얼마나 발생하는가
- MCP 연결 실패가 얼마나 자주 발생하는가
- projection stale 상태가 얼마나 오래 지속되는가
- reconcile item이 얼마나 자주 발생하는가
- same-session verify guard가 얼마나 자주 동작하는가

지표는 운영 개선용 파생값이지, 또 하나의 source-of-truth가 아니다.

## 19. 요약

참조 구현의 핵심은 다음이다.

1. 일상 조작 API는 Harness MCP server다.
2. authoritative state는 `state.sqlite`와 event log에 있다.
3. 저장소 문서는 사람이 읽는 projection이며 reconcile 규칙을 가진다.
4. approval은 scope-bound contract로 집행한다.
5. evidence manifest는 acceptance criteria와 evidence를 연결한다.
6. verification은 fresh run과 bundle을 기반으로 수행한다.
7. conformance suite는 core invariant가 실제로 동작하는지 검증한다.
