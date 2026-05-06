# 사용자 가이드

## 문서 역할

이 문서는 사용자가 에이전트에게 어떻게 말하고, 상태를 어떻게 읽고, 어떤 판단을 언제 내려야 하는지 설명한다.

구현 내부나 설치 세부는 다루지 않는다.

## 시작 문장

일상 작업은 명령어가 아니라 대화로 시작한다.

```text
이 작업 하네스 기준으로 진행해.
```

이 말은 "상태를 확인하고, 범위를 잡고, 쓰기 전에 허용 범위를 확인하고, 근거와 검증과 사용자 판단을 남기면서 진행해"라는 뜻이다.

자주 쓰는 말:

```text
상태 보여줘.
이 작업 이어서 해. 하네스 상태부터 확인해.
범위와 질문부터 잡아줘.
작은 수정이면 direct로 처리하고, 커지면 work로 전환해.
승인해. 범위는 방금 설명한 내용까지만.
detached verify 시작해.
Manual QA가 필요한지 판단해줘.
수용해. 이 작업 닫아.
```

## 기본 진행 흐름

기본 흐름은 work-management system이 아니라 짧은 대화처럼 느껴져야 한다. 사용자는 보통 모든 internal record가 아니라 compact status card와 다음 safe action을 본다.

1. 상태 확인 또는 intake.
2. `advisor`, `direct`, `work`로 분류.
3. 범위와 Change Unit 확인.
4. 쓰기 전 `prepare_write`.
5. 변경 후 run/evidence 기록.
6. 필요한 경우 verify, Manual QA, acceptance.
7. Close.

Gate는 "왜 지금 task가 안전하게 proceed 또는 close될 수 없는지"로 설명한다. Evidence insufficiency는 abstract database condition이 아니라 acceptance criterion별로 보여준다. Cooperative guarantee가 표시되면 surface가 Harness decision을 따를 것으로 기대되지만 모든 violating write를 실행 전에 물리적으로 막지는 못할 수 있다고 평이하게 설명한다.

```text
Close blocked:
- AC-02 evidence missing
- Manual QA pending for UI copy
- Verification waived would close as risk accepted, not detached verified
```

## 상태 카드 읽기

좋은 하네스 세션은 먼저 짧은 상태 카드를 보여준다.

```text
TASK-0044 이메일 로그인 플로우 추가
모드: work
상태: shaping
다음 행동: 로그인 실패 UX 결정
범위: 로그인 폼, 로그인 API 호출, 세션 저장
Approval: dependency_change 승인 필요
Evidence: none
Verification: not started
Manual QA: pending
Acceptance: pending
Projection: current
```

볼 것은 네 가지다.

- 요청과 범위가 맞는가.
- 내가 답해야 할 결정이 무엇인가.
- approval, evidence, verification, Manual QA, acceptance 중 무엇이 남았는가.
- 다음 행동이 안전하게 진행 가능한가.

상태가 이상해 보이면 이렇게 말한다.

```text
state 기준으로 현재 상태와 다음 행동을 다시 보여줘.
```

## advisor, direct, work

`advisor`는 읽고 설명하고 비교하고 검토하는 모드다. 제품 파일을 쓰지 않는다.

```text
이 모듈 역할을 설명해줘.
이 설계 선택의 trade-off를 정리해줘.
```

`direct`는 작고 저위험인 변경을 빠르게 처리하는 모드다. Direct도 제품 파일을 쓰려면 범위가 잡혀 있어야 하며, 기본 assurance는 `self_checked`다.

```text
프로필 저장 버튼 오타 고쳐줘. 작으면 direct로 처리해.
```

`work`는 기능 추가, 구조 변경, 위험한 수정, 여러 파일에 걸친 작업처럼 범위 정리와 근거와 독립 검증이 필요한 모드다.

```text
이메일 로그인 플로우 추가해줘. 하네스 기준으로 진행해.
```

작게 시작했지만 범위가 커지면 에이전트는 같은 Task를 `work`로 전환한다고 알려야 한다.

## 네 가지 판단

Approval, assurance, Manual QA, acceptance는 서로 다른 질문이다.

| 판단 | 대답하는 질문 | 대신할 수 없는 것 |
|---|---|---|
| Approval | 이 민감 변경을 진행해도 되는가? | 검증, QA, acceptance |
| Assurance | 기술적으로 어디까지 확인되었는가? | approval, QA, acceptance |
| Manual QA | 사람이 실제 경험 품질을 봤는가? | verification, acceptance |
| Acceptance | 결과와 남은 trade-off를 수용하는가? | approval, verification, QA |

Approval이 필요한 예시는 dependency 추가, auth/permission 변경, schema 변경, public API 변경, destructive write, secret access, production config 변경이다. Approval은 correctness나 acceptance를 뜻하지 않는다.

Assurance는 보통 `none`, `self_checked`, `detached_verified`로 보인다. `detached_verified`는 같은 세션의 자기 검토가 아니라 별도 검증 경계에서 통과한 결과다.

사용자가 verification risk를 수용해서 닫을 수는 있지만, 그 경우는 `detached_verified`가 아니라 risk accepted close다.

## Evidence 부족

Evidence는 "했음"이라는 말이 아니라 acceptance criteria를 뒷받침하는 기록이다.

```text
Evidence: partial
Close blocked: AC-02 supporting evidence missing
```

이렇게 말한다.

```text
어떤 acceptance 기준에 증거가 부족한지 보여주고, 추가로 무엇을 확인하면 충분해지는지 제안해줘.
```

근거가 stale이면 새 실행, 새 로그, 새 diff, 새 검증 bundle, 또는 범위 재확인이 필요할 수 있다.

## Verify

Work는 구현자의 자기 보고만으로 `detached_verified`가 되지 않는다.

```text
detached verify 시작해.
```

검증이 통과하면 에이전트는 무엇을 확인했고, 어떤 기준으로 독립성이 인정되는지, 남은 blocker가 있는지 요약해야 한다.

검증을 지금 하지 않고 닫아야 한다면 이렇게 말한다.

```text
검증 risk를 수용하고 닫아. 남은 위험을 기록해줘.
```

이 경우 성공으로 닫을 수 있지만, assurance는 `detached_verified`로 표시되지 않는다.

## Manual QA

Manual QA는 UX, workflow, copy, accessibility, visual result처럼 사람이 봐야 하는 품질을 확인하는 절차다.

```text
Manual QA가 필요한지 판단해줘.
```

QA가 실패하면 작업은 닫지 않고 rework나 block으로 돌아간다. QA를 생략하려면 이유를 남긴 waiver가 필요하다.

```text
이번 내부 CLI 작업은 Manual QA waived 처리해. 이유: 사용자 UI가 없고 test/log로 충분히 확인 가능.
```

## Acceptance

Acceptance는 "이 결과를 받아들인다"는 마지막 사용자 판단이다. 기술 검증이 통과하고 Manual QA가 끝나도, 남은 trade-off를 사용자가 받아들이지 않으면 닫지 않는다.

```text
수용해. 이 작업 닫아.
```

거절할 수도 있다.

```text
수용하지 않아. 세션 만료 UX를 다시 잡아줘.
```

Acceptance는 approval이나 Manual QA가 아니다.

## 이어서 하기

오래된 채팅을 찾지 말고 하네스 상태에서 재개한다.

```text
이 프로젝트의 active task 상태 보여줘.
TASK-0044 이어서 해. 하네스 상태부터 확인해.
```

재개할 때 확인할 질문은 두 가지다.

```text
지금 다음 행동은 무엇인가?
지금 멈춘 이유는 무엇인가?
```

문서에 메모를 남겼다면 이렇게 말한다.

```text
TASK 문서의 사용자 메모를 확인하고, 상태에 반영해야 할 항목을 reconcile해줘.
```

문서는 사람이 읽는 projection이다. 상태와 문서가 어긋난 것 같으면 projection freshness를 확인하고 state 기준으로 다시 요약하게 한다.
