# 프로젝트 헌장

## 문서 역할

프로젝트 목적, 대상, 가치, 범위, 비목표.

## 담당 범위

- project purpose
- target users
- core values
- current non-goals
- automation philosophy

## 담당하지 않는 범위

- 전략 불변식 세부
- 상태 모델
- API contract
- 운영 절차

## 섹션

### 목적

이 프로젝트의 목적은 AI 지원 개발을 위한 로컬 하네스를 만드는 것이다. 하네스는 작업을 보이게 하고, 범위를 묶고, 근거를 남기고, 검증 가능하게 하며, 명시적인 사람의 판단으로 운영하게 하는 작은 운영 커널이다.

하네스는 대화를 대체하기 위한 것이 아니다. 사용자는 평범한 언어로 시작할 수 있고, 오래 남아야 하는 작업 사실은 로컬 state, artifact, readable projection에 보관된다.

### 대상 사용자

주요 사용자는 다음과 같다.

- AI agent로 product code를 수정, 검증, 설명하는 개발자
- session을 넘나들며 신뢰할 수 있는 resume, evidence, close behavior가 필요한 개인 maintainer
- approval, verification, QA, acceptance의 로컬 기록을 원하는 operator 또는 technical lead
- 하나의 agent surface를 harness contract와 통합하는 connector 작성자
- v2 ownership model을 유지 관리하는 문서 작성자

### 핵심 가치

프로젝트가 중시하는 가치는 다음과 같다.

- Local authority: operational state와 evidence는 remote chat transcript가 아니라 local harness runtime에 보관한다.
- Explicit boundaries: scope, approval, evidence, verification, Manual QA, acceptance를 별도 관심사로 보이게 한다.
- Honest assurance: 무엇을 어떻게 확인했는지, 그 확인이 얼마나 독립적인지 시스템이 말해야 한다.
- Human agency: user는 goal, sensitive approval, product trade-off, QA judgment, acceptance를 계속 통제한다.
- Small implementability: MVP 선택은 fixture로 만들고 test할 수 있을 만큼 구체적이어야 한다.
- Projection humility: Markdown은 사람이 읽고 변경을 제안하는 데 도움을 주지만, 조용히 operational truth가 되지는 않는다.
- Surface neutrality: capability는 product name을 보고 가정하지 않고 profile과 guarantee level로 설명한다.

### 범위

현재 범위는 v2 documentation set과 그것이 설명하는 reference MVP다.

Reference MVP는 하나의 project, 하나의 reference agent surface, local runtime state, durable artifact, public MCP tool, write gating, evidence, detached verification support, Manual QA, acceptance, projection, reconcile, recovery, export, fixture 기반 conformance로 local kernel을 입증해야 한다.

이 헌장은 세부 ownership을 나머지 문서 세트에 맡긴다.

- strategy와 policy boundary: [02-strategy.md](02-strategy.md)
- kernel behavior: [03-kernel-spec.md](03-kernel-spec.md)
- runtime architecture: [04-runtime-architecture.md](04-runtime-architecture.md)
- API와 schema: [05-mcp-api-and-schemas.md](05-mcp-api-and-schemas.md)
- reference implementation plan: [06-reference-mvp.md](06-reference-mvp.md)

### 비목표

현재 비목표는 다음과 같다.

- 사용자의 product repository, VCS, test runner, review process를 대체하기
- chat history를 durable state로 취급하기
- 생성된 Markdown report를 canonical operational record로 취급하기
- MVP에서 모든 agent surface 지원하기
- 연결된 surface가 cooperative 또는 detective behavior만 지원하는데도 preventive enforcement를 약속하기
- dashboard, team workflow platform, long-term analytics layer, broad connector ecosystem을 MVP 범위로 만들기
- approval, QA, verification, acceptance, remaining risk를 하나의 "done" label 뒤에 숨기기

이후 자동화는 future version이 ownership, fixture, fallback behavior, implementation scope를 부여하기 전까지 [appendix/C-later-roadmap.md](appendix/C-later-roadmap.md)에 둔다.

### 자동화 철학

자동화는 작업을 더 이해하기 어렵게 만드는 것이 아니라 더 신뢰하기 쉽게 만들어야 한다.

하네스는 state recording, write check, artifact registration, projection refresh, validator execution, recovery, export, conformance처럼 test하기 충분히 deterministic한 action을 자동화해야 한다. 질문이 intent, sensitive permission, product taste, trade-off acceptance, risk에 관한 것이라면 사람의 판단을 요청해야 한다.

자동화가 어떤 rule을 preventively enforce할 수 없을 때는 실제 guarantee level을 보고하고, enforcement가 더 강한 척하지 않고 cooperative 또는 detective behavior로 fallback해야 한다.

프로젝트는 authority boundary가 불명확한 큰 시스템보다 작고 inspectable한 MVP를 선호한다.
