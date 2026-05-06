# 05. User Guide

## 1. 문서 역할

이 문서는 사용자가 일상 작업에서 하네스를 어떻게 쓰는지 설명한다.

다룬다:

- 처음 연결
- 사용자가 어디에서 작업하는지
- 에이전트와 대화로 하네스를 시작하는 방법
- 상태 카드 읽기
- shared design 질문에 답하는 법
- domain language, module impact, Change Unit 읽기
- approval, assurance, manual QA, acceptance 구분
- direct와 work 흐름
- TDD trace와 evidence 읽기
- 멈춘 작업 이어가기
- projection과 reconcile 상황 처리

설치 내부, MCP server, adapter, hook, validator, DB schema는 구현 문서와 통합 문서가 소유한다.

## 2. 하네스를 한 문장으로 이해하기

하네스는 AI와 함께 하는 개발 작업에서 다음이 계속 보이게 만드는 구조다.

```text
지금 무엇을 하는가
무엇을 아직 결정해야 하는가
어떤 설계 개념을 공유했는가
어떤 근거가 있는가
기술적으로 어디까지 검증했는가
사람이 무엇을 수용해야 하는가
```

사용자는 하네스 내부 명령을 조합하지 않고 에이전트와 대화하면서 이 구조를 사용한다.

## 3. 처음 한 번 연결하기

프로젝트와 agent surface를 처음 한 번 연결한다.

```bash
harness connect agents --auto
```

연결 상태는 다음 명령으로 확인한다.

```bash
harness doctor agents
```

이 명령은 설정과 진단용이다. 일상 작업은 대화로 시작한다.

## 4. 연결되면 프로젝트에 생기는 것

Product Repository에는 사람이 읽는 표면이 생긴다.

```text
repo/
  AGENTS.md
  docs/
    tasks/
    approvals/
    decisions/
    reports/
    evidence/
    design/
      domain-language.md
      module-map.md
      interface-contracts.md
  .harness/
    agent/
    reconcile/
```

운영 상태와 raw artifact는 기본적으로 `~/.harness`에 저장된다.

```text
~/.harness/projects/PRJ-0001/state.sqlite
~/.harness/projects/PRJ-0001/artifacts/
```

사용자는 보통 Product Repository만 열고 일한다. Harness Server / Installation은 MCP server와 Core를 실행하는 설치물이다.

## 5. 기본 사용 방식

하네스가 연결되어 있으면 이렇게 말하면 된다.

```text
이메일 로그인 플로우 추가해줘. 하네스 기준으로 진행해.
```

또는 더 짧게 말할 수 있다.

```text
이 작업 하네스로 진행해.
```

에이전트는 다음을 수행한다.

1. 하네스 상태를 확인한다.
2. 새 Task를 만들지 기존 Task를 이어갈지 판단한다.
3. 요청을 `advisor`, `direct`, `work` 중 하나로 분류한다.
4. `work`면 요구사항 정렬 질문을 통해 shared design concept을 만든다.
5. 필요한 domain language와 module/interface 영향을 확인한다.
6. Change Unit을 만들고 가능한 한 vertical slice로 나눈다.
7. 제품 파일 쓰기 전 approval과 scope를 확인한다.
8. 구현 중 가능한 경우 TDD red → green → refactor를 사용한다.
9. 변경 파일, diff, logs, command output, checkpoint, TDD trace를 evidence로 기록한다.
10. `work` 작업이면 detached verify를 분리한다.
11. manual QA가 필요한 작업이면 QA 상태를 표시한다.
12. 사용자 수용 판단이 남으면 acceptance를 묻는다.
13. 현재 상태와 다음 행동을 요약한다.

## 6. 자주 쓰는 말

```text
상태 보여줘.
```

```text
이 작업 이어서 해.
```

```text
하네스 기준으로 범위와 질문부터 잡아줘.
```

```text
내가 답해야 할 모호성만 하나씩 물어봐.
```

```text
domain language와 module impact도 같이 확인해.
```

```text
첫 Change Unit은 vertical slice로 잡아줘.
```

```text
이건 작은 수정이면 direct로 처리하고, 커지면 work로 전환해.
```

```text
승인해. 범위는 방금 설명한 내용까지만.
```

```text
거절해. 대안 다시 보여줘.
```

```text
TDD trace와 evidence manifest 기준으로 보여줘.
```

```text
detached verify 시작해.
```

```text
manual QA가 필요한지 판단해줘.
```

```text
검증 결과와 남은 판단을 요약해.
```

```text
수용해. 이 작업 닫아.
```

## 7. 세 가지 작업 모드

### 7.1 advisor

`advisor`는 설명, 비교, 리뷰, 초안 작성에 쓰는 모드다.

적합한 요청:

- “이 모듈이 하는 일을 설명해 줘”
- “두 구현안의 trade-off를 정리해 줘”
- “이 설계 선택이 맞는지 검토해 줘”
- “결정 문서 초안을 만들어 줘”
- “domain language 초안을 만들어 줘”
- “이 코드베이스의 deep module 후보를 찾아줘”

### 7.2 direct

`direct`는 작고 저위험이며 결과 확인이 분명한 변경을 빠르게 처리하는 모드다.

적합한 요청:

- 버튼 오타 수정
- 명백한 import 경로 수정
- 작은 테스트 보정
- 간단한 문서 링크 수정
- 국소적이고 리스크가 낮은 버그 수정

`direct`는 빠르게 끝내는 모드지만, 범위가 커지면 같은 Task를 `work`로 전환한다.

### 7.3 work

`work`는 기능 추가, 구조 변경, 비국소 버그 수정처럼 범위 정리, 구현, 검증이 필요한 일반 작업 모드다.

적합한 요청:

- 새 로그인 플로우 추가
- 인증 구조 변경
- 여러 파일에 걸친 회귀 수정
- 리팩터링
- 테스트 전략 강화
- public interface 변경
- UI/UX 결과를 사람이 봐야 하는 작업

`work`는 구현 후 별도 verify가 필요하다.

## 8. 첫 응답에서 확인할 것

좋은 하네스 연결 세션은 먼저 짧은 상태 카드를 보여준다.

```text
하네스로 작업을 시작합니다.

TASK-0042 이메일 로그인 플로우 추가
상태: work / shaping
다음 행동: shared design 질문 해결
사용자 판단: 로그인 실패 UX와 세션 저장 방식 결정 필요
리스크: medium
증거: none
설계: domain language 확인 필요, module impact pending
QA: none
최신 보고: none

초안 범위:
- 로그인 폼
- 로그인 API 호출
- 세션 저장
- 기본 오류 처리

비범위:
- 소셜 로그인
- 비밀번호 재설정
- 권한 체계 개편

먼저 한 가지 질문입니다.
잘못된 비밀번호일 때 같은 화면에 오류 메시지를 표시할까요, 별도 알림을 사용할까요?
권장: 같은 화면의 inline 오류. 이유: 접근성과 테스트가 쉽고 로그인 흐름을 벗어나지 않습니다.
```

사용자는 다음을 확인한다.

- 요청이 맞게 이해되었는가
- `direct`로 충분한지, `work`가 필요한지
- 질문이 실제 모호성을 줄이는가
- 범위와 비범위가 맞는가
- acceptance 기준이 충분한가
- domain language나 module impact가 빠지지 않았는가
- approval이 필요한 범주가 있는가
- 다음 행동이 명확한가

## 9. shared design 질문에 답하는 법

Shaping 질문은 긴 계획서가 아니라 필요한 결정을 하나씩 정리하는 절차다.

```text
질문 2/7: 기존 사용자에게도 새 포인트를 소급 적용할까요?

권장: 첫 릴리스에서는 소급 적용하지 않음.
이유:
- migration risk가 낮습니다.
- 새 기능의 동작을 먼저 검증할 수 있습니다.
- 필요하면 별도 backfill Change Unit으로 분리할 수 있습니다.

대안:
A. 전체 소급 적용
B. 최근 30일만 소급 적용
C. 소급 적용 없음

선택하시겠습니까?
```

짧게 답해도 된다.

```text
C로 가자. 대신 나중에 backfill 이슈를 남겨줘.
```

모를 때는 이렇게 답한다.

```text
모르겠어. 리스크가 가장 낮은 선택으로 추천해줘.
```

도메인 확인이 필요하면 이렇게 답한다.

```text
이건 도메인 담당자 확인이 필요해. pending decision으로 남겨줘.
```

## 10. domain language와 module impact 보기

작업이 도메인 용어를 포함하면 에이전트는 용어를 확인한다.

```text
Domain Language impact:
- Login Session: 사용자가 인증 후 유지하는 client/server session 상태
- Auth Token: API 요청 인증에 쓰는 token. Login Session과 같은 말로 쓰지 않음
- Account: 로그인 가능한 사용자 identity

Module impact:
- AuthService: 로그인 검증과 세션 생성. public interface 변경 있음
- LoginForm: UI 입력과 오류 표시
- SessionStore: 세션 유지. 기존 interface 재사용
```

사용자는 다음을 본다.

- 용어가 제품에서 쓰는 말과 맞는가
- 기존 용어와 혼동되지 않는가
- public interface 변경이 있는가
- deep module 내부 구현으로 위임해도 되는가
- 사람이 반드시 interface를 검토해야 하는가

## 11. Change Unit 읽는 법

좋은 Change Unit은 구현 단위이면서 검증 단위다.

```text
CU-01 로그인 성공/실패 vertical slice
slice type: vertical
end-to-end path:
- input: 로그인 폼 submit
- domain logic: AuthService.validateCredentials
- persistence: session 저장
- boundary: POST /login
- output: dashboard 이동 또는 inline 오류
TDD: required
Manual QA: browser_smoke + copy
Blocked by: none
Unblocks: CU-02 session expiry polish
Merge risk: medium
```

기능 작업의 첫 Change Unit이 다음처럼 보이면 조정한다.

```text
CU-01 DB schema 전체 작성
CU-02 API 전체 작성
CU-03 UI 전체 작성
```

이렇게 말한다.

```text
이건 너무 horizontal해. 첫 Change Unit을 실제 로그인 흐름이 끝까지 얇게 통과하는 vertical slice로 다시 나눠줘.
```

## 12. approval, assurance, manual QA, acceptance

네 질문은 서로 다르다.

```text
approval: 진행해도 되는가
assurance: 기술적으로 어느 수준까지 확인되었는가
manual QA: 사람이 봐야 하는 품질을 확인했는가
acceptance: 사용자가 결과와 남은 trade-off를 받아들이는가
```

Approval은 민감 변경을 시작하기 전 진행 허가다. 대표 범주는 auth, permission, schema, dependency, public API, destructive write, network write, secret access, production config, CI/CD, infra, privacy, telemetry, license, billing이다. Approval은 검증 통과나 사용자 수용을 대신하지 않는다.

Assurance는 기술적으로 어느 수준까지 확인되었는지 나타낸다.

```text
none
self_checked
detached_verified
```

Manual QA는 사람이 실제 결과를 보고 UX, copy, workflow, accessibility, visual quality를 확인하는 절차다.

```text
verdict: passed
assurance: detached_verified
manual QA: pending
acceptance: pending
```

의미:

```text
기술 검증은 통과했지만, 사람이 실제 UX를 확인하고 최종 수용해야 한다.
```

Acceptance는 사용자가 이 결과와 남은 trade-off를 받아들이는 판단이다. 검증이 통과해도 acceptance가 남을 수 있다.

## 13. approval 대화 예시

```text
승인이 필요합니다.

APR-0012 dependency_change
요청: zod 추가
목적: 로그인 폼 입력 검증
허용 경로:
- package.json
- pnpm-lock.yaml
- src/auth/**
- tests/auth/**

리스크:
- dependency 추가
- lockfile 변경

대안:
1. 기존 validation utility 사용
2. 직접 validation 함수 작성
3. zod 추가

권장: zod 추가

승인하시겠습니까?
```

사용자 답변:

```text
승인해. zod 추가까지만.
```

또는:

```text
거절해. 기존 validation utility를 쓰는 대안으로 다시 잡아줘.
```

## 14. direct 작업 흐름

사용자:

```text
프로필 저장 버튼 오타 고쳐줘.
```

에이전트:

```text
하네스가 direct 작업으로 분류했습니다.

TASK-0043 프로필 저장 버튼 오타 수정
변경 범위: UI label 1개
승인 필요: 없음
검증: changed_paths + 관련 테스트
Manual QA: 없음

수정 후 결과를 보고하겠습니다.
```

완료 후:

```text
완료했습니다.

TASK-0043
결과: passed
assurance: self_checked
변경 파일:
- src/profile/ProfileForm.tsx

검증:
- changed_paths: passed
- pnpm test src/profile: passed

다음 행동: 없음
```

Direct 범위가 커지면 에이전트는 같은 Task를 `work`로 전환한다고 보고한다.

## 15. work 작업 흐름

사용자:

```text
이메일 로그인 플로우 추가해줘. 하네스 기준으로 진행해.
```

Shaping 결과:

```text
TASK-0044 이메일 로그인 플로우 추가
분류: work
현재 단계: shaping

Goal:
- 이메일과 비밀번호로 로그인할 수 있게 한다.

Scope:
- 로그인 폼
- 로그인 API 호출
- 세션 저장
- 기본 실패 메시지

Out of scope:
- 소셜 로그인
- 비밀번호 재설정
- 관리자 권한 체계 변경

Acceptance Criteria:
- AC-01 유효한 계정으로 로그인 성공
- AC-02 잘못된 비밀번호에서 inline 오류 표시
- AC-03 로그인 성공 후 세션 유지
- AC-04 기존 회원가입 플로우 회귀 없음

Domain Language:
- Account
- Login Session
- Auth Token

Module Impact:
- AuthService public interface 변경
- LoginForm UI 변경
- SessionStore 기존 interface 재사용

Change Units:
- CU-01 로그인 성공/실패 vertical slice
- CU-02 세션 만료 UX 보강
- CU-03 회귀 테스트 보강

다음 행동: CU-01 구현
```

구현 후:

```text
CU-01 구현이 끝났습니다.

RUN-SUMMARY, TDD-TRACE, EVIDENCE-MANIFEST가 기록되었습니다.
변경 파일:
- src/auth/LoginForm.tsx
- src/auth/AuthService.ts
- tests/auth/login.test.ts

TDD Trace:
- red: invalid password test failed as expected
- green: login tests passed
- refactor: AuthService validation helper 정리

검증:
- changed_paths: passed
- approval_scope: passed
- vertical_slice_shape: passed
- tdd_trace: passed
- pnpm test tests/auth/login.test.ts: passed
- evidence_sufficiency: sufficient

work 작업이므로 detached verify가 필요합니다.
```

Verify 후:

```text
검증 완료.

EVAL-0020
verdict: passed
assurance: detached_verified
manual QA: pending
acceptance: pending

남은 사용자 판단:
- 세션 만료 시 로그인 화면으로 이동하는 UX를 현재 방식으로 수용할지 확인 필요
- 오류 copy가 제품 톤과 맞는지 manual QA 필요
```

닫을 때:

```text
manual QA 통과. 수용해. 이 작업 닫아.
```

## 16. TDD trace 읽는 법

TDD trace는 테스트가 구현에 맞춰 나중에 만들어진 것이 아니라, 구현 전에 실패했고 구현 후 통과했는지를 보여준다.

```text
TDD-0007 CU-01 로그인 실패 오류
required: true
red:
- test: tests/auth/login.test.ts::invalid_password
- command: pnpm test tests/auth/login.test.ts
- result: failed_as_expected

green:
- command: pnpm test tests/auth/login.test.ts
- result: passed

refactor:
- AuthService validation helper 추출
- tests still passed
```

주의 신호:

```text
TDD required: true
red evidence: missing
green evidence: passed
```

이때 이렇게 말할 수 있다.

```text
TDD trace가 partial이야. 이 Change Unit에서 red evidence가 왜 없는지 설명하고, reviewer focus에 남겨줘.
```

## 17. manual QA 읽는 법

```text
MANUAL-QA-0003
profile: browser_smoke + copy
result: failed
finding:
- 로그인 실패 메시지가 너무 기술적임: "Invalid credentials" 대신 사용자 친화적 문구 필요
next action:
- CU-04 오류 copy 조정
```

기술 검증이 통과했어도 QA가 실패하면 작업은 닫지 않는다.

```text
기술 검증은 통과했지만 manual QA가 실패했어. finding을 새 Change Unit으로 잡아줘.
```

Manual QA 생략은 명시적 waiver를 남긴다.

```text
이번 내부 CLI 작업은 manual QA를 waived 처리해. 이유: 사용자 UI가 없고 test/log로 충분히 확인 가능.
```

## 18. `TASK` 문서 읽는 법

`TASK`는 현재 continuity를 이해하는 중심 문서다.

먼저 `Current Summary`를 본다.

- mode와 phase
- 다음 행동
- 사용자 판단 대기
- active Change Unit
- 최신 report
- design alignment
- manual QA
- projection freshness

다음으로 `Rolling Spine`을 본다.

- 현재 유효한 사실
- 현재 유효한 가정
- 결정
- rejected option
- domain terms in force
- module/interface impacts
- watchpoint
- resume note

`TASK`를 모든 역사 기록으로 읽지 않는다. 지금 이어서 일하기 위한 문서로 읽는다.

## 19. report 문서 읽는 법

| 문서 | 먼저 볼 것 |
|---|---|
| `DIRECT-RESULT` | 요청, 범위, 변경 파일, checks, assurance, work 전환 여부 |
| `RUN-SUMMARY` | 변경 파일, 명령, 결과, 남은 이슈, evidence refs, TDD trace |
| `EVIDENCE-MANIFEST` | 각 AC의 supported 여부, supporting evidence, stale 조건 |
| `EVAL` | verdict, assurance impact, independence, blockers, manual QA, user follow-up |
| `TDD-TRACE` | red, green, refactor, non-TDD justification |
| `MANUAL-QA` | profile, result, findings, waiver reason |

## 20. 멈춘 작업 이어서 하기

오래된 채팅을 찾기보다 이렇게 말한다.

```text
이 프로젝트의 active task 상태 보여줘.
```

또는:

```text
TASK-0044 이어서 해. 하네스 상태부터 확인해.
```

재개할 때 확인할 질문은 두 가지다.

```text
지금 다음 행동은 무엇인가?
지금 멈춘 이유는 무엇인가?
```

## 21. projection이나 문서 수정이 어긋났을 때

문서는 사람이 읽는 projection이다. 상태 저장소는 최신인데 문서가 늦게 갱신될 수 있다.

```text
projection 최신성 확인하고, state 기준으로 현재 상태 보여줘.
```

`TASK` 문서에 직접 메모를 남겼다면 이렇게 말한다.

```text
TASK 문서의 사용자 메모를 확인하고, 상태에 반영해야 할 항목을 reconcile해줘.
```

하네스는 사람이 쓴 메모를 자동으로 운영 상태로 바꾸지 않고, 반영할지 확인한다.

## 22. 오래된 문서와 doc rot 다루기

기본 원칙:

- 현재 active `TASK`와 최신 report를 먼저 본다.
- 완료된 PRD, DESIGN, 이슈 문서는 기본 context에 계속 넣지 않는다.
- 필요할 때 artifact ref로 찾아본다.
- 오래된 문서가 현재 코드와 다르면 stale로 표시하거나 DEC로 정리한다.

요청 예시:

```text
이 DESIGN이 현재 코드와 맞는지 stale 여부를 확인해줘.
```

## 23. CLI를 쓰는 경우

CLI는 setup, doctor, recovery, reconcile, export, CI, 운영자 디버그에 사용한다.

```bash
harness connect agents --auto
harness doctor agents
harness serve mcp
harness reconcile
harness recover TASK-0001
harness export TASK-0001
```

사용자는 복잡한 작업 흐름을 CLI로 조합하지 않는다.

## 24. 자주 있는 상황

| 상황 | 말하기 |
|---|---|
| 에이전트가 바로 수정하려 함 | `잠깐. 하네스 상태부터 확인하고 Task를 만든 뒤 진행해.` |
| plan만 길게 씀 | `계획서보다 shared design 질문부터 해줘. 한 번에 하나씩 묻고 추천안과 trade-off를 같이 보여줘.` |
| approval 범위가 넓음 | `승인 범위가 넓어. 허용 경로와 도구를 줄여서 다시 요청해.` |
| Change Unit이 horizontal함 | `사용자 흐름이 끝까지 보이는 vertical slice로 다시 나눠줘.` |
| TDD trace가 없음 | `red/green/refactor trace를 남겨줘. 이미 구현했다면 non-TDD justification을 기록해.` |
| self-review가 detached verify처럼 보고됨 | `그건 detached verify가 아니야. fresh evaluator나 별도 bundle 기반 검증으로 다시 확인해.` |
| evidence 부족 | `어떤 acceptance 기준에 증거가 부족한지 evidence manifest 기준으로 보여줘.` |
| domain language가 어긋남 | `domain language 기준으로 정리하고 필요한 결정만 남겨줘.` |

## 25. 빠른 참조

| 상황 | 말하기 |
|---|---|
| 시작 | `이 작업 하네스 기준으로 진행해.` |
| 상태 확인 | `상태 보여줘.` |
| 요구사항 정렬 | `범위와 질문부터 잡아줘.` |
| 도메인 언어 | `domain language impact를 보여줘.` |
| Change Unit | `첫 Change Unit은 vertical slice로 잡아줘.` |
| 승인 | `승인해. 범위는 방금 설명한 내용까지만.` |
| 거절 | `거절해. 대안 다시 보여줘.` |
| TDD | `TDD trace 기준으로 진행해.` |
| 검증 | `detached verify 시작해.` |
| QA | `manual QA가 필요한지 판단해줘.` |
| 수용 | `수용해. 이 작업 닫아.` |
| 재개 | `이 작업 이어서 해. 하네스 상태부터 확인해.` |

## 26. 운영 습관

1. 새 작업은 “하네스 기준으로 진행해”라고 시작한다.
2. 첫 상태 카드에서 범위, 리스크, 다음 행동을 확인한다.
3. 큰 작업은 plan보다 shared design 질문을 먼저 진행한다.
4. 작은 작업은 direct로 처리하되, 커지면 work로 전환한다.
5. 기능 작업의 첫 Change Unit은 가능한 한 vertical slice로 만든다.
6. 민감한 변경은 approval 없이 진행하지 않는다.
7. 가능한 작업은 TDD trace를 남긴다.
8. work는 detached verify 없이 완료로 닫지 않는다.
9. UI/UX/copy 작업은 manual QA 상태를 확인한다.
10. 완료 판단 전에 approval, assurance, manual QA, acceptance를 각각 확인한다.
11. 재개할 때는 채팅보다 `TASK`와 최신 report를 먼저 본다.
12. evidence가 부족하면 어떤 기준이 증거 없이 남았는지 확인한다.
13. 문서와 상태가 다르면 projection freshness와 reconcile 상태를 확인한다.
14. 오래된 PRD/DESIGN은 기본 context에 넣지 말고 필요할 때 pull한다.

하네스는 명령어를 외우기 위한 도구가 아니라, AI 작업을 다시 읽고 통제할 수 있게 만드는 구조다.
