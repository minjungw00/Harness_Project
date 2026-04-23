# 하네스 사용 가이드 v04

## 1. 문서 목적

이 가이드는 사용자가 일상 작업에서 하네스를 어떻게 쓰는지 설명한다.

이 가이드는 다음에 집중한다.

- 에이전트와 대화만으로 하네스를 시작하는 방법
- 현재 상태를 읽는 방법
- approval, assurance, acceptance를 구분하는 방법
- 작은 작업과 큰 작업의 흐름 차이
- 구현과 검증을 분리하는 방법
- 멈춘 작업을 이어서 하는 방법
- 여러 agent surface에서 공통으로 쓸 수 있는 말

설치, MCP server, adapter, hook, validator, 상태 저장 구조는 구현 문서와 통합 문서에서 다룬다.

## 2. 하네스를 한 문장으로 이해하기

하네스는 AI와 함께 하는 개발 작업에서 다음 네 가지가 계속 보이게 만드는 구조다.

- 지금 무엇을 하는가
- 무엇을 아직 결정해야 하는가
- 어떤 근거가 있는가
- 다음에 무엇을 해야 하는가

사용자는 하네스 내부 명령을 조합하지 않고, 에이전트와 대화하면서 이 구조를 사용할 수 있어야 한다.

## 3. 기본 사용 방식

하네스가 프로젝트와 에이전트 표면에 연결되어 있으면, 보통 이렇게 말하면 된다.

```text
이메일 로그인 플로우 추가해줘. 하네스 기준으로 진행해.
```

또는 더 짧게 말할 수 있다.

```text
이 작업 하네스로 진행해.
```

에이전트는 다음을 자동으로 수행한다.

1. 하네스 상태를 확인한다.
2. 새 Task를 만들지 기존 Task를 이어갈지 판단한다.
3. 요청을 `advisor`, `direct`, `work` 중 하나로 분류한다.
4. 필요한 경우 작업 범위와 acceptance 기준을 사용자에게 확인한다.
5. 구현 중 approval이 필요하면 사용자에게 승인 요청을 보여준다.
6. 변경 파일, diff, logs, command output, checkpoint를 증거로 기록한다.
7. `work` 작업이면 detached verify를 분리한다.
8. 사용자 수용 판단이 남으면 acceptance를 묻는다.
9. 현재 상태와 다음 행동을 요약한다.

사용자가 직접 `shape`, `implement`, `verify`, `change unit`, `surface`를 조합하는 흐름은 기본 사용 방식이 아니다.

## 4. 처음 한 번만 하는 연결

일상 사용 전에 한 번은 프로젝트와 에이전트 표면을 연결해야 한다.

기본 연결은 설치 wizard로 처리한다.

```bash
harness connect agents --auto
```

연결 상태는 다음 명령으로 확인한다.

```bash
harness doctor agents
```

이 명령들은 일상 작업 명령이 아니라 설정과 진단용이다.

일상 작업은 대화로 시작한다.

## 5. 자주 쓰는 대화 명령

```text
상태 보여줘.
```

```text
이 작업 이어서 해.
```

```text
하네스 기준으로 범위 먼저 잡아줘.
```

```text
작은 수정이면 바로 처리하고, 커지면 work로 전환해.
```

```text
승인해. 범위는 방금 설명한 내용까지만.
```

```text
거절해. 대안 다시 보여줘.
```

```text
detached verify 시작해.
```

```text
검증 결과와 남은 판단을 요약해.
```

```text
수용해. 이 작업 닫아.
```

```text
수용하지 않아. 대안으로 다시 잡아줘.
```

## 6. 세 가지 작업 모드

### 6.1 advisor

`advisor`는 설명, 비교, 리뷰, 초안 작성에 쓰는 모드다.

예시:

```text
이 인증 구조가 어떻게 동작하는지 설명해줘. 하네스 기준으로 결정 메모가 필요하면 남겨줘.
```

적합한 요청은 다음이다.

- “이 모듈이 하는 일을 설명해 줘”
- “두 구현안의 trade-off를 정리해 줘”
- “이 설계 선택이 맞는지 검토해 줘”
- “결정 문서 초안을 만들어 줘”

기대 결과는 답변, 결정 메모, 설계 초안이다.

제품 코드 변경은 전제하지 않는다.

### 6.2 direct

`direct`는 작고 저위험이며 결과 확인이 분명한 변경을 빠르게 처리하는 모드다.

예시:

```text
프로필 저장 버튼 오타 고쳐줘. 하네스 기준으로 작은 수정이면 direct로 처리해.
```

적합한 요청은 다음이다.

- 버튼 오타 수정
- 명백한 import 경로 수정
- 작은 테스트 보정
- 간단한 문서 링크 수정
- 국소적이고 리스크가 낮은 버그 수정

`direct`는 빠르게 끝내는 모드지만, 범위가 커지면 같은 Task를 `work`로 전환한다.

### 6.3 work

`work`는 기능 추가, 구조 변경, 비국소 버그 수정처럼 범위 정리, 구현, 검증이 필요한 일반 작업 모드다.

예시:

```text
이메일 로그인 플로우 추가해줘. 하네스 기준으로 범위와 acceptance 기준부터 잡고 진행해.
```

적합한 요청은 다음이다.

- 새 로그인 플로우 추가
- 인증 구조 변경
- 여러 파일에 걸친 회귀 수정
- 리팩터링
- 테스트 전략 강화

`work`는 구현 후 별도 verify가 필요하다.

## 7. 에이전트의 첫 응답에서 확인할 것

좋은 하네스 연결 세션은 작업을 바로 밀어붙이지 않고, 먼저 짧은 상태 카드를 보여준다.

```text
하네스로 작업을 시작합니다.

TASK-0042 이메일 로그인 플로우 추가
상태: work / shaping
다음 행동: 범위와 acceptance 기준 확정
사용자 판단: 범위 확인 필요
리스크: medium
증거: none
승인: 아직 필요 없음

초안 범위:
- 로그인 폼
- 로그인 API 호출
- 세션 저장
- 기본 오류 처리

비범위:
- 소셜 로그인
- 비밀번호 재설정
- 권한 체계 개편

이 범위로 진행할까요?
```

사용자는 다음을 확인한다.

- 요청이 맞게 이해되었는가
- `direct`로 충분한지, `work`가 필요한지
- 범위와 비범위가 맞는가
- acceptance 기준이 충분한가
- approval이 필요한 범주가 있는가
- 다음 행동이 명확한가

## 8. 상태 카드 읽는 법

기본 상태 카드는 짧아야 한다.

```text
TASK-0044 이메일 로그인 플로우 추가
상태: work / verifying
다음 행동: detached verify 결과 확인
사용자 판단: 세션 만료 UX 수용 여부
리스크: medium
증거: sufficient
최신 보고: EVAL 대기
```

자세한 상태가 필요하면 이렇게 말한다.

```text
자세한 상태 보여줘.
```

상세 상태는 다음처럼 보일 수 있다.

```text
TASK-0044 이메일 로그인 플로우 추가
mode: work
phase: verifying
result: none
assurance: self_checked
approval: granted
acceptance: pending
risk: medium
evidence: sufficient
active change unit: CU-01
next action: detached verify 결과 확인
pending decision: 세션 만료 UX 수용 여부
latest report: RUN-SUMMARY-...
latest approval: APR-0012
projection: current
```

먼저 볼 것은 다음이다.

- `next action`: 지금 무엇을 해야 하는가
- `pending decision`: 사용자가 결정해야 하는 것이 있는가
- `approval`: 진행 허가가 필요한가
- `assurance`: 기술 검증 수준은 어디까지인가
- `acceptance`: 결과 수용 판단이 남았는가
- `evidence`: 검증과 재개에 충분한 근거가 있는가
- `latest report`: 최신 판정 또는 실행 요약은 무엇인가

핵심은 “누가 일하고 있는가”가 아니라 “무엇이 남았는가”를 먼저 읽는 것이다.

## 9. approval, assurance, acceptance

하네스를 읽을 때 가장 자주 헷갈리는 세 질문은 서로 다르다.

### 9.1 approval

“이 변경을 진행해도 되는가?”

민감한 범주의 변경을 시작하기 전에 확인한다.

대표 범주는 다음이다.

- auth / permission 변경
- schema / migration 변경
- dependency 추가 또는 제거
- public API 변경
- destructive write
- network write 또는 외부 서비스 write
- secret access
- production config, deployment, CI/CD, infra 변경
- privacy, PII, data export, telemetry 변경
- license, compliance, billing 영향 변경
- policy override

approval은 진행 허가다.

검증 통과나 사용자 수용을 대신하지 않는다.

### 9.2 assurance

“기술적으로 어느 수준까지 확인되었는가?”

예를 들어 self-check만 했는지, detached verify까지 했는지를 본다.

### 9.3 acceptance

“사용자가 이 결과와 남은 trade-off를 받아들이는가?”

검증이 통과해도 acceptance가 별도로 남을 수 있다.

```text
verdict: passed
assurance: detached_verified
acceptance: pending
```

의미는 다음이다.

```text
기술 검증은 통과했지만, 사용자가 남은 trade-off를 수용할지 아직 결정하지 않았다.
```

## 10. approval 대화 예시

에이전트가 dependency 추가가 필요하다고 판단하면 바로 설치하지 않고 승인 요청을 보여준다.

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

사용자는 이렇게 답할 수 있다.

```text
승인해. zod 추가까지만.
```

또는 이렇게 답할 수 있다.

```text
거절해. 기존 validation utility를 쓰는 대안으로 다시 잡아줘.
```

승인 범위가 넓으면 이렇게 답한다.

```text
승인 범위가 넓어. 허용 경로와 도구를 줄여서 다시 요청해.
```

## 11. direct 작업 흐름

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

`direct`가 커지면 에이전트는 같은 Task를 `work`로 전환한다고 보고해야 한다.

```text
이 수정은 direct 범위를 넘었습니다.

이유:
- 저장 API 응답 구조 변경이 필요합니다.
- 테스트 범위가 profile UI를 넘어갑니다.

같은 Task를 work로 전환하고 범위와 acceptance 기준을 다시 잡겠습니다.
```

## 12. work 작업 흐름

사용자:

```text
이메일 로그인 플로우 추가해줘. 하네스 기준으로 진행해.
```

에이전트는 먼저 범위와 기준을 잡는다.

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
- 유효한 계정으로 로그인 성공
- 잘못된 비밀번호에서 오류 표시
- 로그인 성공 후 세션 유지
- 기존 회원가입 플로우 회귀 없음

Change Units:
- CU-01 로그인 UI/API 연결
- CU-02 테스트 보강

다음 행동: CU-01 구현
```

구현 후에는 다음을 보고한다.

```text
CU-01 구현이 끝났습니다.

RUN-SUMMARY와 EVIDENCE-MANIFEST가 기록되었습니다.
변경 파일:
- src/auth/LoginForm.tsx
- src/auth/api.ts
- tests/auth/login.test.ts

검증:
- changed_paths: passed
- approval_scope: passed
- pnpm test tests/auth/login.test.ts: passed
- evidence_sufficiency: sufficient

work 작업이므로 detached verify가 필요합니다.
지금 별도 verify run을 시작하겠습니다.
```

verify 후에는 다음을 보고한다.

```text
검증 완료.

EVAL-0020
verdict: passed
assurance: detached_verified
acceptance: pending

남은 사용자 판단:
- 세션 만료 시 로그인 화면으로 이동하는 UX를 현재 방식으로 수용할지 확인 필요

수용하시겠습니까, 아니면 UX를 조정할까요?
```

사용자는 이렇게 닫을 수 있다.

```text
수용해. 이 작업 닫아.
```

## 13. `TASK` 문서를 읽는 법

`TASK`는 계획 문서가 아니라 현재 continuity를 이해하는 중심 문서다.

`TASK`에서 가장 먼저 보는 블록은 `Current Summary`다.

여기서 다음을 본다.

- 지금 어떤 mode와 phase인가
- 무엇이 다음 행동인가
- 무엇이 아직 사용자 판단 대기인가
- active change unit이 무엇인가
- 최신 report가 무엇인가
- projection이 최신인가

그 다음 `Rolling Spine`을 읽는다.

여기에는 다음이 들어 있다.

- 현재 유효한 사실
- 현재 유효한 가정
- 이미 내려진 결정
- 버린 선택지
- watchpoint
- 다음 세션이 알아야 할 메모

핵심은 `TASK`를 모든 역사 기록으로 읽지 않고, 지금 이어서 일하기 위한 문서로 읽는 것이다.

## 14. report 문서 읽는 법

### 14.1 DIRECT-RESULT

`direct` 작업 뒤에 남는다.

먼저 볼 것:

- 요청
- 처리 범위
- 변경 파일
- 실행한 검증
- 결과 요약
- assurance level
- work로 전환되었는지 여부

### 14.2 RUN-SUMMARY

`work`의 구현 run 뒤에 남는다.

먼저 볼 것:

- 변경 파일
- 실행한 명령과 검증
- 결과
- 남은 이슈
- Task Spine에 반영할 업데이트
- diff/log/bundle/checkpoint 참조

### 14.3 EVIDENCE-MANIFEST

acceptance criteria와 증거의 대응 관계를 보여준다.

먼저 볼 것:

- 각 acceptance criterion이 supported인지
- 어떤 test, log, diff, run summary가 근거인지
- 어떤 changed file이 어떤 기준을 뒷받침하는지
- stale 조건이 있는지

### 14.4 EVAL

Detached verify 뒤에 남는다.

먼저 볼 것:

- verdict
- assurance impact
- verification independence
- acceptance impact
- next action
- 수행한 검증
- evidence reviewed
- blocker 또는 rework
- 사용자 후속 확인 항목

## 15. 멈춘 작업 이어서 하기

오래된 채팅을 찾기보다 이렇게 말한다.

```text
이 프로젝트의 active task 상태 보여줘.
```

또는:

```text
TASK-0044 이어서 해. 하네스 상태부터 확인해.
```

에이전트는 다음을 읽고 요약해야 한다.

- `TASK`
- 최신 `EVAL` 또는 `DIRECT-RESULT`
- 최신 `RUN-SUMMARY`
- `EVIDENCE-MANIFEST`
- 관련 `APR`
- 필요한 경우 `DEC` 또는 `DESIGN`

재개할 때 사용자가 확인할 질문은 두 가지면 충분하다.

```text
지금 다음 행동은 무엇인가?
지금 멈춘 이유는 무엇인가?
```

## 16. projection이나 문서 수정이 어긋났을 때

문서는 사람이 읽는 projection이다.

가끔 상태 저장소는 최신인데 문서가 늦게 갱신될 수 있다.

이럴 때는 이렇게 말한다.

```text
projection 최신성 확인하고, state 기준으로 현재 상태 보여줘.
```

사용자가 `TASK` 문서에 직접 메모를 남겼다면 에이전트에게 이렇게 말할 수 있다.

```text
TASK 문서의 사용자 메모를 확인하고, 상태에 반영해야 할 항목을 reconcile해줘.
```

하네스는 사람이 쓴 메모를 자동으로 운영 상태로 바꾸지 않고, 반영할지 확인한다.

## 17. 여러 agent surface에서 쓰는 법

각 도구의 세부 설정은 다르지만 사용자의 말은 거의 같다.

### 17.1 Codex

```text
이 작업 하네스 기준으로 진행해. 먼저 Harness 상태를 확인하고 필요한 Skill/MCP를 사용해.
```

### 17.2 Claude Code

```text
하네스 상태부터 확인하고, approval이 필요한 변경은 진행 전에 물어봐. work면 detached verify를 분리해.
```

### 17.3 Gemini

```text
Harness extension을 사용해서 이 작업을 진행해. 범위, 증거, 검증 상태를 계속 보여줘.
```

### 17.4 GitHub Copilot

```text
Harness agent로 이 작업 진행해. 상태 카드부터 보여줘.
```

### 17.5 Cursor

```text
Cursor에서 하네스 기준으로 진행해. rules와 MCP 상태를 먼저 확인해.
```

## 18. CLI를 쓰는 경우

일상 작업은 대화가 기본이다.

CLI는 다음 경우에 쓴다.

- 프로젝트 연결
- 에이전트 표면 연결
- 연결 상태 진단
- MCP server 실행 또는 재시작
- projection reconcile
- adapter debug
- 손상된 Task 복구
- artifact export
- CI/conformance 실행

대표 운영자 명령은 다음이다.

```bash
harness connect agents --auto
harness doctor agents
harness serve mcp
harness reconcile
harness recover TASK-0001
harness export TASK-0001
```

사용자는 복잡한 작업 흐름을 CLI로 조합하지 않는다.

## 19. 자주 있는 상황

### 19.1 에이전트가 하네스를 쓰지 않고 바로 수정하려고 한다

```text
잠깐. 하네스 상태부터 확인하고 Task를 만든 뒤 진행해.
```

### 19.2 direct로 시작했는데 커졌다

같은 Task를 `work`로 전환한다.

새 Task를 만들기보다 continuity를 유지하는 편이 좋다.

### 19.3 approval 요청이 너무 넓다

```text
승인 범위가 넓어. 허용 경로와 도구를 줄여서 다시 요청해.
```

### 19.4 verify가 막혔다

`EVAL`의 blocker와 `TASK`의 next action을 본다.

대부분의 경우 다음 둘 중 하나다.

- 증거가 부족하다
- 사용자 판단이나 approval이 남아 있다

### 19.5 검증은 통과했지만 마음에 들지 않는다

검증은 기술 판정이고 acceptance는 사용자 판단이다.

```text
기술 검증은 수용하지만 결과 UX는 수용하지 않아. 대안을 다시 잡아줘.
```

### 19.6 same-session self-review가 detached verify처럼 보고된다

```text
그건 detached verify가 아니야. fresh evaluator나 별도 bundle 기반 검증으로 다시 확인해.
```

### 19.7 evidence가 부족하다고 나온다

```text
어떤 acceptance 기준에 증거가 부족한지 evidence manifest 기준으로 보여줘.
```

## 20. 빠른 참조

| 목적 | 말하기 |
|---|---|
| 시작 | 이 작업 하네스 기준으로 진행해. |
| 상태 확인 | 상태 보여줘. |
| 범위 확인 | 범위와 acceptance 기준부터 잡아줘. |
| 승인 | 승인해. 범위는 방금 설명한 내용까지만. |
| 거절 | 거절해. 대안 다시 보여줘. |
| 검증 | detached verify 시작해. |
| 수용 | 수용해. 이 작업 닫아. |
| 재개 | 이 작업 이어서 해. 하네스 상태부터 확인해. |

## 21. 운영 습관

1. 새 작업은 “하네스 기준으로 진행해”라고 시작한다.
2. 에이전트의 첫 상태 카드에서 범위, 리스크, 다음 행동을 확인한다.
3. 작은 작업은 direct로 처리하되, 커지면 work로 전환한다.
4. 민감한 변경은 approval 없이 진행하지 않는다.
5. work는 detached verify 없이 완료로 닫지 않는다.
6. 완료 판단 전에 approval, assurance, acceptance를 각각 확인한다.
7. 재개할 때는 채팅보다 `TASK`와 최신 report를 먼저 본다.
8. evidence가 부족하면 어떤 기준이 증거 없이 남았는지 확인한다.
9. 문서와 상태가 다르면 projection freshness와 reconcile 상태를 확인한다.

하네스는 명령어를 외우기 위한 도구가 아니라, AI 작업을 다시 읽고 통제할 수 있게 만드는 구조다.
