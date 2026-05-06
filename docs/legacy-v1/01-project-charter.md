# 01. Project Charter

## 1. 문서 역할

이 문서는 하네스 프로젝트의 상위 기준을 정의한다. 프로젝트 목적, 대상 사용자, 핵심 가치, 범위, 비목표를 소유한다.

전략 불변식은 `02-strategy.md`가 소유한다. 로컬 배치와 런타임 구조는 `03-architecture.md`가 소유한다. 구현 계약은 `04-reference-implementation.md`가 소유한다.

## 2. 프로젝트 목적

하네스는 AI와 함께 하는 개발 작업을 가시적이고 제한 가능하며 검증 가능한 운영 단위로 만든다.

하네스의 상위 목적은 다음이다.

```text
사용자는 작업 방향과 최종 판단을 가진다.
에이전트는 실행을 돕는다.
하네스는 상태, 범위, shared design, domain language, approval, evidence, verification, manual QA, acceptance를 운영 구조로 만든다.
```

AI가 코드를 빠르게 만들수록 설계 부채도 빠르게 쌓일 수 있다. 하네스는 AI가 좋은 소프트웨어 설계 경계 안에서 일하게 한다. 요구사항을 먼저 정렬하고, 도메인 언어를 안정화하고, 모듈 경계와 public interface를 사람이 검토하게 하며, 작은 피드백 루프와 evidence를 작업 단위에 연결한다.

## 3. 기본 관점

하네스의 우선순위는 다음이다.

```text
목적 > 편의
이해 가능한 흐름 > 숨겨진 자동화
공유된 설계 개념 > 성급한 구현
작은 검증 가능한 단위 > 큰 일괄 생성
도메인 언어와 모듈 경계 > 임기응변 설명
상태와 evidence > 채팅 기억
사용자 판단 > 에이전트 단독 완료
좋은 코드베이스 > 많은 코드 생성량
```

사용자는 목표, 우선순위, 승인, 수용 판단을 가진다. 에이전트는 선택지, 근거, 리스크, 현재 상태를 드러내고 승인된 경계 안에서 실행한다.

## 4. 해결해야 하는 문제

하네스는 다음 문제를 줄인다.

- 작업 상태와 다음 행동이 사용자에게 충분히 보이지 않는다.
- 작업 범위와 리스크가 대화 흐름에 따라 흔들린다.
- 중요한 선택의 이유와 trade-off가 채팅 안에만 남는다.
- 사람과 AI가 같은 설계 개념 없이 구현을 시작한다.
- 제품 도메인 용어가 대화, 코드, 테스트, 문서에서 다르게 쓰인다.
- DB, API, UI를 수평적으로 나누어 통합 피드백이 늦어진다.
- 구현 뒤에 테스트를 맞춰 쓰면서 얕은 assurance가 생긴다.
- shallow module이 늘어나 코드 탐색과 테스트 경계가 흐려진다.
- public interface 변경이 사람의 설계 검토 없이 누적된다.
- 프론트엔드, UX, copy, workflow 품질이 manual QA 없이 통과된다.
- approval, assurance, manual QA, acceptance가 하나의 완료 보고로 섞인다.
- 재개와 복구가 채팅 기록, 모델 기억, 사람의 기억에 의존한다.
- 오래된 문서가 현재 코드와 어긋난 채 context를 오염시킨다.

## 5. 사용자가 얻어야 하는 경험

사용자는 에이전트와 대화하면서 다음을 계속 볼 수 있어야 한다.

- 현재 무엇이 진행 중인지
- 다음 행동이 무엇인지
- 어떤 질문에 답해야 하는지
- 무엇이 scope이고 무엇이 out-of-scope인지
- 어떤 도메인 용어와 모듈 경계가 영향을 받는지
- Change Unit이 vertical slice인지, 예외가 있는지
- 어떤 approval이 필요한지
- 어떤 변경, diff, log, test, TDD trace가 evidence로 남았는지
- work 작업이 detached verification을 거쳤는지
- manual QA가 필요한지, 통과했는지, waive 되었는지
- 기술 검증과 최종 acceptance가 어떻게 분리되는지
- 새 세션에서 어디서부터 이어가야 하는지

좋은 사용자 경험은 CLI 조합이 아니라 대화 중심이다. CLI는 설치, 진단, 복구, export, conformance 같은 운영 작업에 사용한다.

## 6. 적용 대상

하네스는 개인 또는 소규모 팀의 로컬 개발 환경을 기본 대상으로 한다.

일차 대상은 다음이다.

- AI coding agent를 사용하는 제품 저장소
- 요구사항 정렬과 설계 품질을 사람이 유지해야 하는 코드베이스
- 여러 agent surface를 연결할 수 있는 로컬 프로젝트
- 재개, 검증, approval, QA 기록이 필요한 개인 또는 소규모 팀 작업

하네스 코어는 특정 프로젝트 구조, 기술 스택, IDE, CLI, 모델에 강하게 종속되지 않는다. 프로젝트별 특수성은 `project.yaml`, domain language, module map, interface contract, 로컬 규칙으로 분리한다.

## 7. 핵심 가치

### 7.1 가시성

현재 상태, 다음 행동, 남은 사용자 판단, 최신 evidence가 짧은 카드와 `TASK` projection으로 보인다.

### 7.2 통제

제품 파일 쓰기 전 scope, approval, sensitive category, allowed paths, surface capability를 확인한다.

### 7.3 설계 정렬

큰 작업은 shared design 질문을 통해 scope, acceptance criteria, rejected option, domain language, module/interface impact를 먼저 정리한다.

### 7.4 짧은 피드백 루프

기능 로직과 deep module 구현은 가능한 한 TDD red → green → refactor를 사용한다. 실행 명령과 결과는 artifact로 남긴다.

### 7.5 독립 검증

Work 작업은 실행자의 자기 보고만으로 닫지 않는다. Fresh session, fresh worktree, sandbox, manual bundle 같은 독립 검증 경계를 기록한다.

### 7.6 사람의 QA와 수용 판단

자동 테스트와 verification이 다루기 어려운 UX, workflow, copy, visual quality, product taste는 manual QA 상태로 관리한다. 사용자의 acceptance는 검증 이후의 별도 판단이다.

## 8. 자동화 원칙

자동화는 가시성, 통제, evidence, 검증 구조가 안정된 뒤 확장한다.

자동화가 확장되기 위한 조건은 다음이다.

- 현재 상태가 읽힌다.
- 다음 행동이 명확하다.
- 남은 사용자 판단이 표시된다.
- 승인 범위가 확인된다.
- evidence가 acceptance criteria에 연결된다.
- detached verification이 실행자 맥락과 분리된다.
- manual QA 필요 여부가 표시된다.
- acceptance가 별도 상태로 남는다.
- 재개와 복구가 대화 없이 가능하다.

Parallel agent orchestration은 Change Unit DAG, baseline, approval scope, merge risk, detached verification이 안정적으로 표현된 뒤 확장한다.

## 9. 현재 범위의 비목표

현재 범위는 다음을 목표로 삼지 않는다.

- 모든 agent surface connector를 동시에 완성하는 것
- 특정 IDE, CLI, 모델에 코어 계약을 고정하는 것
- 사용자가 내부 구조를 익혀야만 쓸 수 있는 흐름
- 규칙 파일과 프롬프트만으로 보안, 범위, 검증을 집행하는 방식
- agent surface memory를 하네스 상태 저장소로 사용하는 방식
- 문서만 고치고 코드를 보지 않는 specs-to-code 흐름
- 모든 작업에 무거운 설계 문서나 TDD를 강제하는 방식
- 자동 병렬 실행을 core 품질보다 먼저 완성하는 것
- 사람 조직 직함을 기본 아키텍처로 삼는 multi-agent 구조

## 10. 목표 관리 원칙

새 목표는 기존 목표를 조용히 대체하지 않는다. 중요한 변경은 어떤 가치가 강화되고 어떤 가치가 약화될 수 있는지 검토한다.

하네스 목표는 다음 균형을 유지한다.

```text
통제와 속도
자동화와 사용자 판단
문서와 source-of-truth
AI 위임과 사람의 설계 소유권
절차 준수와 실제 코드 품질
```

