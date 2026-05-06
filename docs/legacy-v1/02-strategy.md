# 02. Harness Strategy

## 1. 문서 역할

이 문서는 하네스의 전략 층위를 정의한다. 해결 문제, 설계 원칙, 핵심 불변식, 사용자 작업 모델, 상태 축, source-of-truth, approval/assurance/manual QA/acceptance 분리, 설계 품질 기준을 소유한다.

구현 schema와 MCP tool 입출력은 `04-reference-implementation.md`가 소유한다. 문서 템플릿은 `07-document-and-artifact-contracts.md`가 소유한다. 실행 playbook은 `09-design-quality-playbooks.md`가 소유한다.

## 2. 전략 문제

하네스가 다루는 전략 문제는 네 묶음이다.

### 2.1 사용자가 흐름을 잃는 문제

- 현재 상태가 보이지 않는다.
- 다음 행동과 남은 판단이 불분명하다.
- 완료 보고가 검증, QA, acceptance를 섞는다.
- 채팅 기록이 사라지면 재개가 어렵다.

### 2.2 범위와 승인 문제

- 작업 범위가 대화 중 넓어진다.
- 민감 변경이 사전 approval 없이 진행된다.
- approval 범위와 실제 변경이 어긋난다.
- agent surface capability가 과대평가된다.

### 2.3 설계 품질 문제

- 사람과 AI가 shared design concept 없이 구현을 시작한다.
- 제품 도메인 용어가 흔들린다.
- 기능이 horizontal phase로 나뉘어 실제 피드백이 늦어진다.
- 테스트가 구현 뒤에 맞춰진다.
- shallow module과 public interface drift가 누적된다.
- UX, copy, workflow 품질이 manual QA 없이 통과된다.

### 2.4 source-of-truth 문제

- 에이전트 대화와 문서가 사실처럼 사용된다.
- 오래된 PRD, DESIGN, issue가 현재 context를 오염시킨다.
- raw logs와 diff가 문서에 흩어진다.
- 운영 상태, projection, evidence의 기준 저장소가 섞인다.

## 3. 설계 원칙

### 3.1 사용자는 방향과 판단을 가진다

사용자는 목표, 우선순위, 승인, 수용 판단을 가진다. 에이전트는 선택지, 근거, 리스크, 현재 상태를 드러내고 승인된 경계 안에서 실행한다.

### 3.2 기본 경험은 대화 중심이다

사용자는 Product Repository에서 자연어로 요청한다. 에이전트는 Harness Skill과 MCP를 사용해 Task, mode, next action, approval, evidence, verification, QA를 처리한다. CLI는 setup, doctor, recovery, reconcile, export, conformance에 사용한다.

### 3.3 가장 작은 유효 워크플로를 사용한다

```text
advisor: 설명, 비교, 리뷰, 결정 초안
direct: 작고 저위험이며 결과 확인이 분명한 변경
 work: 구조화, 구현, 독립 검증이 필요한 작업
```

절차는 작업 리스크에 맞게 증가한다. 작은 작업은 작게 끝나고, 커지는 작업은 같은 Task를 `work`로 전환한다.

### 3.4 구현 전에 shared design을 맞춘다

Work는 바로 구현으로 들어가지 않는다. 다음을 정리한다.

- Goal
- Scope / Out of scope
- Acceptance Criteria
- Open Decisions
- Assumptions with expiry
- Rejected Options
- Domain Language impact
- Module / Interface impact
- Change Unit draft

### 3.5 제품 도메인 언어를 기준 입력으로 둔다

제품별 `DOMAIN-LANGUAGE`는 구현과 검증에서 사용하는 공통 어휘다. 구현 에이전트는 필요한 용어를 pull한다. Reviewer와 evaluator는 관련 domain language를 push받아 일관성을 확인한다.

### 3.6 기능 Change Unit은 vertical slice를 기본값으로 둔다

기능 작업의 첫 구현 Change Unit은 가능한 한 vertical slice다.

```text
trigger / input
→ domain logic
→ persistence
→ API or caller boundary
→ UI or system-observable output
→ test evidence
→ optional manual QA
```

Horizontal/enabling Change Unit은 예외 사유와 후속 vertical Change Unit을 가진다.

### 3.7 피드백 루프가 AI 산출물의 상한이다

TDD, lint, typecheck, unit test, integration test, browser check, runtime validation, manual QA는 AI가 방향을 잃지 않게 하는 피드백 루프다. 가능한 작업은 red → green → refactor evidence를 남긴다.

### 3.8 사람은 interface와 boundary를 설계한다

사람은 public interface, module boundary, test boundary, domain language, data model 의미, 권한/보안 boundary를 검토한다. AI는 승인된 경계 뒤의 내부 구현을 맡을 수 있다.

### 3.9 실행과 검증을 분리한다

Shaping과 implementation은 하나의 lead run에서 이어질 수 있다. Verification은 가능한 한 fresh session, fresh worktree, sandbox, manual bundle 같은 분리된 검증 경계에서 수행한다. Same-session self-review는 detached verification으로 인정하지 않는다.

### 3.10 Manual QA와 acceptance를 분리한다

Manual QA는 사람이 실제 결과를 보고 품질을 확인하는 절차다. Acceptance는 사용자가 결과와 남은 trade-off를 받아들이는 판단이다. 두 상태는 서로를 대체하지 않는다.

### 3.11 대화보다 상태와 evidence를 신뢰한다

재개, 판정, 추적의 기준은 `state.sqlite`, event log, artifact store, projection이다. 에이전트 대화는 조작 표면과 설명 표면이다.

## 4. 핵심 불변식

1. 사용자는 자연어로 작업을 시작할 수 있다.
2. Work shaping은 shared design concept을 확인한다.
3. 제품 파일 쓰기 전 scope와 approval을 확인한다.
4. 도메인 용어는 작업 안에서 일관되게 사용한다.
5. 기능 Change Unit은 vertical slice를 기본값으로 둔다.
6. TDD trace는 적합한 작업에서 evidence로 남긴다.
7. Public interface 변경은 설계 검토 대상이다.
8. 변경 후 evidence를 남긴다.
9. Work는 실행자의 자기 보고만으로 닫지 않는다.
10. Manual QA가 필요한 작업은 QA 상태 없이 닫지 않는다.
11. Approval, assurance, manual QA, acceptance는 서로 다른 질문이다.
12. 재개와 복구는 대화 없이 가능해야 한다.
13. 하나의 사실에는 하나의 authoritative source가 있다.
14. 사람이 읽는 문서는 현재 요약과 판단 가능성에 집중한다.
15. Human-editable 문서 입력은 reconcile을 거쳐 상태에 반영한다.
16. 민감 범주는 명시적 approval 없이 진행하지 않는다.
17. Agent surface capability는 선언되고 검증된다.

## 5. 사용자 작업 모델

### 5.1 advisor

`advisor`는 읽기, 비교, 설명, 리뷰, 결정 초안에 사용한다. 제품 코드 변경을 전제하지 않는다.

예시:

- 구조 설명
- 코드 리뷰
- 설계 선택지 비교
- 테스트 전략 검토
- DEC 또는 DESIGN 초안
- domain language 또는 module map 검토

### 5.2 direct

`direct`는 작고 저위험이며 결과 확인이 분명한 변경을 처리한다.

예시:

- 오타 수정
- 명백한 import 경로 보정
- 문서 링크 정리
- 범위가 좁은 테스트 보정
- 국소 버그 수정

Direct는 보통 `self_checked` assurance를 가진다. 범위가 커지면 같은 Task를 `work`로 전환한다.

### 5.3 work

`work`는 구조화, 구현, 독립 검증이 필요한 일반 개발 모드다.

예시:

- 기능 추가
- 구조 변경
- 비국소 버그 수정
- 리팩터링
- 테스트 전략 강화
- public interface 변경
- UI/UX 결과를 사람이 봐야 하는 작업

Work는 shaping, implementation, verification, manual QA, acceptance를 가질 수 있다.

## 6. 상태 축

하네스는 여러 축의 상태를 사용한다. 사용자 표면은 compact card로 필요한 축만 먼저 보여준다.

| 축 | 의미 |
|---|---|
| `mode` | advisor, direct, work |
| `phase` | intake, shaping, ready, executing, verifying, qa, waiting_user, blocked, completed, cancelled |
| `result` | none, advice_only, passed, failed, cancelled |
| `assurance` | none, self_checked, detached_verified |
| `verification_independence` | same_session, fresh_session, fresh_worktree, sandbox 등 qualifier |
| `approval` | not_required, pending, granted, denied, expired |
| `manual_qa` | none, pending, passed, failed, waived |
| `acceptance` | not_requested, pending, accepted, rejected |
| `risk` | low, medium, high |
| `evidence` | none, partial, sufficient, stale |
| `design_alignment` | none, partial, aligned, stale |
| `architecture` | none, review_required, reviewed, drift_detected |

## 7. Task와 Task Spine

Task는 사용자 가치 단위다. Task의 continuity는 세 층으로 유지한다.

```text
Current Summary
  지금 바로 알아야 할 상태, 다음 행동, pending decision

Rolling Spine
  현재 유효한 사실, 가정, 결정, rejected option, domain term, module impact, watchpoint

Snapshot References
  RUN-SUMMARY, EVAL, APR, DEC, DESIGN, EVIDENCE-MANIFEST, TDD-TRACE, MANUAL-QA
```

`TASK` 문서는 현재를 이해하는 중심 projection이다. Raw trace와 과거 상세 이력은 report와 artifact로 분리한다.

## 8. Change Unit

Change Unit은 실제 구현 단위다.

기본 필드는 다음이다.

- 목적과 비목표
- slice type: vertical, enabling, cleanup, horizontal_exception
- end-to-end path
- allowed paths
- allowed tools
- validator profile
- approval categories
- TDD requirement
- manual QA requirement
- dependencies
- merge risk
- completion conditions
- evaluator focus

Change Unit이 1개일 때는 DAG를 사용자 카드에 노출하지 않는다. Change Unit이 2개 이상이면 blocked_by, unblocks, parallelizable_with, merge risk를 표시한다.

## 9. Evidence와 Artifact

Evidence는 검증과 재개에 사용할 수 있는 근거다. Artifact는 durable output이다.

예시는 다음이다.

- diff
- logs
- checkpoint
- bundle
- run summary
- TDD trace
- manual QA note 또는 screenshot
- eval report
- evidence manifest

Evidence Manifest는 acceptance criteria와 evidence를 연결한다. 검증자는 criterion이 어떤 evidence로 supported인지 확인한다.

## 10. source-of-truth 원칙

| 정보 | canonical source | 사람이 보는 표면 |
|---|---|---|
| 프로젝트 등록과 surface 연결 | `registry.sqlite` | status, connector report |
| 정적 프로젝트 설정 | `project.yaml` | project summary |
| 운영 상태 전이 | `state.sqlite` + event log | status card, `TASK` |
| raw evidence | artifact store | artifact ref |
| approval | approval record + event log | `APR` |
| evidence coverage | evidence manifest record | `EVIDENCE-MANIFEST` |
| verification verdict | eval record + artifact refs | `EVAL` |
| TDD trace | tdd_trace record + artifact refs | `TDD-TRACE`, `RUN-SUMMARY` |
| manual QA | QA record + artifact refs | `MANUAL-QA`, `EVAL` |
| domain language | domain language record + reconciled doc | `DOMAIN-LANGUAGE` |
| module/interface | design records + reconciled docs | `MODULE-MAP`, `INTERFACE-CONTRACT` |
| 사용자 메모 | human-editable 문서 영역 | reconcile item |

운영 상태 전이의 canonical source는 `state.sqlite`와 event log다. 문서는 사람이 읽는 projection이다. Artifact store는 raw evidence의 canonical source다.

## 11. 네 가지 판단의 분리

```text
approval
  진행해도 되는가

assurance
  기술적으로 어느 수준까지 확인되었는가

manual QA
  사람이 봐야 하는 품질을 확인했는가

acceptance
  사용자가 결과와 남은 trade-off를 받아들이는가
```

Approval은 사전 허가다. Assurance는 기술 검증 수준이다. Manual QA는 사람이 직접 보는 품질 확인이다. Acceptance는 사후 수용 판단이다.

## 12. 민감 변경 범주

다음 범주는 explicit approval을 요구한다.

- auth, permission, security policy
- schema, migration, persistence
- dependency 추가, 제거, 업그레이드
- public API 또는 외부 contract
- destructive write
- network write 또는 external service write
- secret access
- production config, deployment, CI/CD, infra
- privacy, PII, data export, telemetry
- license, compliance, billing 또는 비용 영향
- model, prompt, policy override

Approval은 허용 경로, 허용 도구, network target, secret scope, baseline, 만료 조건을 함께 가진다.

## 13. 설계 품질 기준

### Shared Design

큰 작업은 질문과 결정으로 shared design concept을 만든다. 질문은 blocking ambiguity를 줄이는 데 집중한다.

### Domain Language

제품 용어는 한 의미로 사용한다. 새 용어와 충돌 가능성은 proposal 또는 reconcile item으로 남긴다.

### Deep Module과 Public Interface

하네스가 선호하는 구조는 단순한 public interface, 충분한 내부 기능, 명확한 test boundary, 제한된 dependency direction이다.

### Vertical Slice

기능 작업은 사용자 가치 또는 system-observable 결과를 얇게 끝까지 연결한다.

### TDD

TDD는 AI가 너무 많은 코드를 한 번에 생성하는 것을 제한하는 피드백 장치다. Red, green, refactor 근거를 남긴다.

### Manual QA

UI, UX, copy, accessibility, visual output, product taste는 사람이 확인한다.

## 14. 통합 원칙

- 항상 읽히는 rule/context는 짧다.
- 반복 절차는 Skill이 가진다.
- 상태 변경은 MCP tool로 수행한다.
- 정책 집행은 Core, validator, hook, adapter, sidecar가 맡는다.
- 구현 에이전트는 긴 기준을 필요할 때 pull한다.
- Reviewer/evaluator는 관련 기준을 push받는다.
- Capability는 제품명 대신 profile로 선언한다.
- 부족한 capability는 sidecar, validator, fresh evaluator, 격리 실행으로 보완한다.

## 15. 전략 요약

```text
연속 맥락
공유된 설계 개념
명시적 경계
짧은 피드백 루프
독립 검증
durable evidence
```

