# 하네스 문서 세트

하네스는 AI와 함께 하는 개발 작업을 사용자가 읽고, 제한하고, 검증하고, 다시 이어받을 수 있게 만드는 로컬 운영 구조다.

핵심 문장은 다음이다.

```text
사용자는 방향과 판단을 가진다.
에이전트는 실행을 돕는다.
하네스는 상태, 범위, 승인, 증거, 검증, QA, 수용 판단을 운영 구조로 만든다.
```

하네스는 AI가 좋은 소프트웨어 설계 경계 안에서 일하게 한다. 요구사항을 먼저 정렬하고, 제품 도메인 언어를 안정화하고, 구현 단위를 가능한 한 vertical slice로 나누고, TDD와 짧은 피드백 루프를 evidence에 연결한다. 기술 검증, manual QA, 사용자 acceptance는 서로 다른 판단으로 관리한다.

## 기본 실행 모델

```text
Product Repository
  사용자가 여는 제품 저장소다.
  제품 코드와 사람이 읽는 Markdown projection이 있다.

Harness Server / Installation
  MCP server와 Core를 실행하는 설치물이다.
  일반 사용자는 패키지나 바이너리로 접한다.

Harness Runtime Home
  운영 상태와 raw evidence가 저장되는 로컬 홈이다.
  기본 위치는 ~/.harness다.
```

사용자는 Product Repository에서 agent surface와 대화한다. Agent surface는 Harness MCP server를 호출한다. Harness Core는 `state.sqlite`와 event log를 갱신한다. Artifact store는 diff, logs, bundle, TDD trace, QA note 같은 raw evidence를 저장한다. Product Repository의 문서는 사람이 읽는 projection이다.

## 문서 목록

| 문서 | 소유 역할 |
|---|---|
| `00-overview.md` | 전체 개요, 세 공간, 기본 흐름, 핵심 개념 |
| `01-project-charter.md` | 프로젝트 목적, 대상, 가치, 비목표 |
| `02-strategy.md` | 불변식, 작업 모델, 상태 축, source-of-truth, 품질 원칙 |
| `03-architecture.md` | 로컬 배치, 런타임 계층, 데이터 흐름, 주요 시퀀스 |
| `04-reference-implementation.md` | MVP 구현 계약, state, MCP, validator, security, recovery |
| `05-user-guide.md` | 사용자가 실제로 말하고 읽는 법 |
| `06-agent-integration.md` | agent surface 공통 통합 계약과 surface별 차이 |
| `07-document-and-artifact-contracts.md` | Markdown projection, artifact ref, card, 템플릿 |
| `08-operations-and-conformance.md` | setup, doctor, projection, reconcile, recover, export, conformance |
| `09-design-quality-playbooks.md` | shared design, domain language, vertical slice, TDD, module, QA playbook |
| `99-authoring-guide.md` | 문서 작성과 개정 기준 |
| `glossary.md` | 공통 용어 정의 |

## 권장 읽기 경로

### 일반 사용자

```text
00-overview.md
→ 05-user-guide.md
→ 필요한 경우 09-design-quality-playbooks.md
```

### 구현자

```text
00-overview.md
→ 01-project-charter.md
→ 02-strategy.md
→ 03-architecture.md
→ 04-reference-implementation.md
→ 07-document-and-artifact-contracts.md
→ 08-operations-and-conformance.md
```

### connector 작성자

```text
00-overview.md
→ 03-architecture.md
→ 06-agent-integration.md
→ 08-operations-and-conformance.md
```

### projection 관리자

```text
00-overview.md
→ 07-document-and-artifact-contracts.md
→ 08-operations-and-conformance.md
```

## 핵심 원칙

```text
대화는 조작 표면이다.
상태 전이는 state.sqlite와 event log가 기준이다.
문서는 사람이 읽는 projection이다.
Raw evidence는 artifact store가 기준이다.
Work는 detached verification 없이 닫지 않는다.
Approval, assurance, manual QA, acceptance는 서로 다른 판단이다.
```

