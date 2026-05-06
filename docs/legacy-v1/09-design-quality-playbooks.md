# 09. Design Quality Playbooks

## 1. 문서 역할

이 문서는 하네스에서 AI가 좋은 소프트웨어 설계 경계 안에서 일하도록 돕는 실행 playbook을 정의한다. Shared Design, Domain Language, Vertical Slice, TDD, Deep Module, Manual QA, Change Unit DAG, context hygiene, coding standards push/pull을 소유한다.

State DB schema와 MCP tool schema는 `04-reference-implementation.md`가 소유한다. 문서 템플릿은 `07-document-and-artifact-contracts.md`가 소유한다.

## 2. 공통 원칙

AI 개발 실패는 보통 다음 조건에서 커진다.

- 무엇을 만들지 충분히 정렬하지 않았다.
- 같은 용어를 같은 의미로 쓰지 않았다.
- 실제 사용자 흐름을 끝까지 검증하지 않았다.
- 테스트를 구현 뒤에 맞췄다.
- 모듈 경계가 얕고 복잡하다.
- 사람의 UX/taste 판단이 빠졌다.
- 오래된 문서가 현재 context를 오염시켰다.

이 playbook의 목표는 AI가 작은 단위로 피드백을 받으며 좋은 설계 경계 안에서 구현하게 하는 것이다.

## 3. Shared Design / Grill Protocol

### 3.1 목적

Shared Design Concept은 사람과 AI가 만들려는 것에 대해 공유하는 설계 이해다. 질문, 결정, 가정, rejected option, acceptance criteria, domain language, module impact, Change Unit으로 투영된다.

### 3.2 사용 조건

다음이면 Grill Protocol을 사용한다.

- 요청이 여러 의미로 해석된다.
- 범위와 비범위가 명확하지 않다.
- 사용자 가치와 구현 방식 사이에 결정이 필요하다.
- 도메인 판단이 필요하다.
- public interface, schema, auth, UX, workflow가 영향을 받는다.

작고 명백한 direct 작업은 생략할 수 있다.

### 3.3 절차

```text
1. 사용자 요청을 요약한다.
2. 모호성을 스캔한다.
3. 가장 blocking한 질문부터 하나씩 묻는다.
4. 각 질문마다 AI의 추천안과 trade-off를 제시한다.
5. 사용자 답변을 decision, assumption, pending decision으로 기록한다.
6. rejected option과 이유를 기록한다.
7. domain language와 module impact를 확인한다.
8. acceptance criteria와 out-of-scope를 확정한다.
9. Change Unit으로 전환한다.
```

### 3.4 좋은 질문 형식

```text
질문: 기존 수강 기록에도 포인트를 소급 적용할까요?

권장: 첫 릴리스에서는 소급 적용하지 않음.
이유:
- migration risk가 낮습니다.
- 첫 vertical slice를 빨리 검증할 수 있습니다.
- backfill은 별도 Change Unit으로 분리할 수 있습니다.

선택지:
A. 전체 소급 적용
B. 최근 30일만 소급 적용
C. 소급 적용 없음
```

### 3.5 Stop condition

다음 조건을 만족하면 질문을 멈춘다.

- Goal과 user value가 명확하다.
- Scope와 out-of-scope가 명확하다.
- Acceptance criteria가 검증 가능하다.
- Blocking decision이 없거나 pending으로 명시되었다.
- Domain language impact가 기록되었다.
- Module/interface impact가 기록되었다.
- 첫 Change Unit이 제안 가능하다.

## 4. Domain Language Playbook

### 4.1 목적

Domain Language는 사람, AI, 코드, 테스트가 같은 용어를 같은 의미로 사용하게 한다.

### 4.2 갱신 조건

- 새 도메인 개념이 등장한다.
- 기존 용어를 새 의미로 쓰려 한다.
- 코드와 제품 대화의 용어가 다르다.
- 같은 개념이 여러 이름으로 나타난다.
- reviewer가 domain term mismatch를 발견한다.

### 4.3 작성 기준

각 용어는 다음을 가진다.

- Term
- Meaning
- Code Representation
- Not This
- Related Terms
- Source
- Status

예시:

```text
Term: Enrollment
Meaning: 사용자가 특정 Course에 등록되어 학습 진행 상태를 가질 수 있는 관계
Code Representation: src/courses/enrollment.ts
Not This: Subscription. 결제 상태가 아니라 학습 접근 관계다.
Related Terms: Course, LessonProgress, Student
Source: TASK-0044
Status: active
```

### 4.4 Agent 운영

구현 에이전트는 작업과 관련된 용어만 pull한다. 새 용어를 발견하면 proposal로 남긴다.

Reviewer/evaluator는 관련 domain language를 push받고 코드와 테스트가 같은 용어를 같은 의미로 쓰는지 확인한다.

## 5. Vertical Slice / Tracer Bullet Playbook

### 5.1 목적

Vertical slice는 실제 흐름을 얇게 끝까지 연결해 빠른 피드백을 얻는 방식이다.

### 5.2 좋은 vertical slice

```text
trigger / input
→ domain logic
→ persistence
→ API or caller boundary
→ UI or system-observable output
→ test evidence
→ optional manual QA
```

예시:

```text
수업 완료 클릭
→ LessonProgress가 완료됨
→ GamificationService가 포인트를 부여함
→ point_events에 기록됨
→ dashboard summary API가 포인트를 반환함
→ dashboard에 포인트 표시
→ service test + route test + browser smoke QA
```

### 5.3 피해야 할 분해

```text
CU-01 모든 DB schema 작성
CU-02 모든 service 작성
CU-03 모든 UI 작성
CU-04 테스트 작성
```

이 구조는 feedback loop를 뒤로 미룬다.

### 5.4 Horizontal/enabling 예외

다음 경우 horizontal/enabling Change Unit을 허용한다.

- migration scaffold가 없으면 vertical slice가 위험하다.
- test harness가 없으면 red/green 루프가 불가능하다.
- deep module boundary를 먼저 잡지 않으면 구현이 흩어진다.
- public interface 결정이 먼저 필요하다.

예외 기록:

```text
slice type: horizontal-exception
reason: 기존 schema migration 도구가 없어 안전한 scaffold가 먼저 필요함
follow-up vertical CU: CU-02 lesson completion → points visible on dashboard
```

## 6. TDD Trace Playbook

### 6.1 목적

TDD는 AI가 많은 코드를 한 번에 만들지 않게 하는 피드백 장치다. 테스트를 나중에 쓰면 구현을 정당화하는 얕은 테스트가 생길 수 있다.

### 6.2 적용 기준

TDD required:

- domain logic
- service module
- bug fix
- parser/validator
- state transition
- deep module internals

TDD recommended:

- API/caller boundary
- integration behavior
- edge case가 명확한 기능

TDD optional:

- 문서
- 오타
- throwaway prototype
- 탐색적 UI prototype
- 초기 scaffold

생략 시 non-TDD justification을 남긴다.

### 6.3 절차

```text
1. acceptance criterion 하나를 선택한다.
2. 실패하는 테스트를 작성한다.
3. red command를 실행하고 실패 로그를 남긴다.
4. 최소 구현으로 green을 만든다.
5. green command와 통과 로그를 남긴다.
6. 구조 개선이 필요하면 refactor한다.
7. refactor 후 다시 check한다.
8. TDD-TRACE와 EVIDENCE-MANIFEST를 연결한다.
```

### 6.4 Reviewer focus

- 테스트가 실제 behavior를 검증하는가
- mock이 과도하지 않은가
- public interface 경계에서 테스트하는가
- 구현 세부에 너무 강하게 결합되어 있지 않은가
- red 실패가 의미 있는 실패였는가

## 7. Deep Module / Interface-first Playbook

### 7.1 목적

사람은 코드베이스의 큰 모양, module boundary, public interface, test boundary를 소유해야 한다.

Deep module은 다음 구조다.

```text
작고 단순한 public interface
+ 내부에 충분한 기능과 복잡성
+ 명확한 test boundary
+ 호출자가 내부 구현을 몰라도 됨
```

### 7.2 Shallow module 위험

- AI가 의존성 그래프를 추적하다 길을 잃는다.
- 테스트 경계를 잡기 어렵다.
- 작은 함수마다 얕은 테스트가 생긴다.
- 변경 하나가 많은 파일로 퍼진다.
- public interface가 내부 구현을 노출한다.

### 7.3 Module/interface review 절차

```text
1. 영향을 받는 모듈을 찾는다.
2. 현재 역할과 public interface를 적는다.
3. 새 기능이 interface를 바꾸는지 확인한다.
4. 내부 구현으로 숨길 수 있는 복잡성을 구분한다.
5. test boundary를 정한다.
6. compatibility impact를 확인한다.
7. 필요한 경우 INTERFACE-CONTRACT 또는 DEC를 만든다.
```

### 7.4 사람과 AI의 책임

사람이 검토하는 것:

- public interface
- data model 의미
- 권한/보안 boundary
- migration/compatibility impact
- test boundary
- user-facing behavior

AI에게 위임할 수 있는 것:

- interface 뒤 내부 구현
- 반복적인 테스트 보강
- local refactor
- fixture 작성
- low-risk adapter 구현

## 8. Manual QA / Taste Review Playbook

### 8.1 목적

자동 테스트와 detached verification이 잡지 못하는 품질이 있다.

- 화면이 어색하다.
- copy가 제품 톤과 맞지 않는다.
- 오류 메시지가 사용자에게 도움이 되지 않는다.
- 흐름이 실제 사용에서 막힌다.
- 접근성이나 반응형이 깨진다.
- 기능은 맞지만 제품 감각이 부족하다.

Manual QA는 이런 문제를 사람이 확인하는 절차다.

### 8.2 필요 조건

Manual QA required:

- UI 변경
- UX flow 변경
- copy/error message 변경
- onboarding, checkout, auth, billing 같은 critical flow
- accessibility 영향
- visual output
- browser-only behavior

Manual QA usually none:

- 내부 service logic만 변경
- 문서 변경
- CLI 내부 동작
- test-only 변경

### 8.3 QA profile

| Profile | 확인 대상 |
|---|---|
| `ui_quality` | 시각적 레이아웃, 상태, 반응형 |
| `workflow` | 실제 사용자 흐름 |
| `copy` | 문구, 오류 메시지, tone |
| `accessibility` | 키보드, label, contrast smoke |
| `browser_smoke` | 주요 브라우저 실행 |
| `performance_smoke` | 명백한 지연 또는 렌더링 문제 |

### 8.4 QA 결과 처리

QA passed:

- `manual_qa_state=passed`
- acceptance로 이동 가능

QA failed:

- finding 기록
- severity 표시
- rework Change Unit 생성

QA waived:

- waiver reason 기록
- 사용자 또는 operator decision으로 남김

## 9. Change Unit DAG Playbook

### 9.1 목적

큰 작업은 dependency가 있는 Change Unit DAG로 표현한다. DAG는 병렬 실행을 기본값으로 만들기보다 blocking, merge risk, rework 위치를 읽게 한다.

### 9.2 표시 규칙

Change Unit이 1개면 dependencies를 사용자 카드에 표시하지 않는다. 2개 이상이면 다음을 표시한다.

```yaml
blocked_by: []
unblocks: []
parallelizable_with: []
merge_conflict_risk: low | medium | high
integration_owner: optional CU id
```

### 9.3 병렬화 후보 조건

- 서로 다른 allowed paths 또는 낮은 merge risk
- approval scope 충돌 없음
- shared baseline 또는 worktree isolation
- independent acceptance criteria
- merge 후 통합 verify 계획 있음

## 10. Context hygiene와 doc rot

### 10.1 원칙

대화와 문서는 context를 오염시킬 수 있다. 오래된 PRD나 DESIGN이 현재 코드와 맞지 않으면 AI가 잘못된 방향으로 작업한다.

### 10.2 기본 정책

- 긴 과거 대화를 기본 context에 넣지 않는다.
- 완료된 PRD/DESIGN/issue를 항상 push하지 않는다.
- 현재 `TASK Current Summary`와 `Rolling Spine`을 우선한다.
- 최신 `RUN-SUMMARY`, `EVAL`, `EVIDENCE-MANIFEST`를 우선한다.
- 오래된 문서는 필요할 때 pull한다.
- stale 조건을 명시한다.

### 10.3 Stale 판단

문서는 다음 경우 stale 후보가 된다.

- linked Task scope가 바뀌었다.
- module map이 바뀌었다.
- public interface가 바뀌었다.
- acceptance criteria가 바뀌었다.
- code path가 rename/move되었다.
- EVAL 이후 관련 파일이 변경되었다.

## 11. Coding Standards Push/Pull

### 구현 에이전트

Coding standards는 pull 가능하게 둔다. 항상 push하면 context가 커지고 모든 작업에 모든 규칙이 들어간다.

### Reviewer/evaluator

관련 coding standards를 push한다. Reviewer는 작성된 코드와 기준을 비교해야 한다.

### 문서 배치

```text
docs/design/domain-language.md
docs/design/module-map.md
docs/design/interface-contracts.md
docs/design/coding-standards.md
```

## 12. Playbook 선택 기준

| 상황 | 적용 playbook |
|---|---|
| 요청이 모호함 | Shared Design / Grill |
| 용어가 흔들림 | Domain Language |
| 기능 추가 | Vertical Slice |
| 로직/버그 수정 | TDD Trace |
| 모듈 경계가 복잡함 | Deep Module / Interface-first |
| UI/UX 변경 | Manual QA |
| 큰 작업 | Change Unit DAG |
| 오래된 문서 있음 | Context Hygiene |
| 리뷰 품질 필요 | Push/Pull Coding Standards |

## 13. 요약

```text
구현 전에 정렬한다.
같은 언어를 쓴다.
작게 끝까지 연결한다.
테스트를 먼저 둔다.
인터페이스를 사람이 설계한다.
사람이 봐야 하는 품질은 사람이 본다.
오래된 문서로 현재 context를 오염시키지 않는다.
```

