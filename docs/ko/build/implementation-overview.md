# Build: 구현 개요

## 이 문서가 도와주는 일

이 문서는 구현자가 전체 reference 명세에 들어가기 전에 무엇을 먼저 계획해야 하는지 알려 줍니다. 독자 중심 문서가 kernel, runtime, MCP, storage, projection, conformance reference와 어떻게 이어지는지 보여 주는 Build 계층입니다.

이 문서는 구현 계획 문서입니다. 재설계 문서가 승인되기 전에는 runtime/server 구현을 시작하라는 뜻이 아닙니다.

이 문서로 다음을 확인합니다.

- 먼저 필요한 runtime 구성 요소는 무엇인가?
- 첫 실행 가능한 조각은 어떤 증명을 보여야 하는가?
- MVP를 완료했다고 말하려면 어떤 증명이 필요한가?

이 문서는 SQLite DDL, public MCP 스키마, projection 템플릿 본문, 명령 문법을 정의하지 않습니다. 그런 세부 계약은 reference 문서에 둡니다.

## 이런 때 읽기

- 재설계 문서가 승인된 뒤 첫 구현 형태를 계획할 때.
- 제안된 MVP 구현 계획이 올바른 범위를 유지하는지 리뷰할 때.
- 엄밀한 reference 명세를 읽기 전에 짧은 지도가 필요할 때.

## 읽기 전에

Learn 경로에서 Harness의 기본 개념을 먼저 이해해 두는 것이 좋습니다. 정확한 동작은 이 문서 끝에 연결된 현재 번호 문서들을 봅니다.

## 핵심 생각

가장 작은 로컬 Core 권한 경로를 먼저 증명하고, 그다음 근거, projection, conformance, 운영자 복구 경로를 붙여 단단하게 만듭니다.

## 무엇을 만드는가

Harness MVP는 AI 지원 제품 작업을 위한 로컬 권한 커널입니다. 첫 구현 계획은 명확한 내부 모듈을 가진 하나의 로컬 시스템을 기준으로 하며, 분산 플랫폼으로 시작하지 않습니다.

### Local Server / Process

MCP 경계를 제공하고, Core 전이를 소유하며, runtime home을 읽고 쓰는 로컬 Harness server 또는 프로세스 하나를 계획합니다. 검증기 실행, projection enqueue, reconcile, 복구, export, conformance 진입점은 모두 같은 Core 규칙 위에서 실행되어야 합니다.

MVP는 모듈을 가진 단일 프로세스로 충분합니다. Core, projection, validation, 운영자 도구를 별도 서비스로 나눌 필요는 없습니다.

### Core

Core는 운영 상태의 기준 기록을 변경하는 유일한 경로입니다. Core는 다음을 해야 합니다.

- tool envelope, idempotency key, expected state version을 검증한다
- 필요한 project 또는 task lock을 획득한다
- 현재 기록을 읽는다
- Core check와 validator를 실행한다
- 하나의 transaction에서 현재 기록을 갱신하고 task_events를 append한다
- 상태 변경 뒤 projection 작업을 enqueue한다
- 결과를 설명하는 막힘과 참조를 반환한다

Agent, 운영자 명령, projector, recovery flow는 Core를 통하거나 같은 Core compatibility rule을 보존해야 합니다.

### State Store

State store는 운영 상태의 기준 기록을 보관합니다. 여기에는 project state, Task, gate, Change Unit, Decision Packet, approval, Write Authorization, Run, Evidence Manifest, Eval record, Manual QA record, Residual Risk, projection job, reconcile item, task_events가 포함됩니다.

Build 계층에서 이를 새로 설계하지 않습니다. 재설계 중 storage와 DDL의 현재 담당 문서: [Reference MVP](../06-reference-mvp.md). 이후 reference 경로: `reference/storage-and-ddl.md`.

### Artifact Store

Artifact store는 오래 보존해야 하는 근거 파일과 integrity metadata를 보관합니다. Raw artifact는 diff, log, screenshot, bundle, manifest, checkpoint, export component, 그 밖의 근거 파일이 될 수 있습니다.

Artifact store는 느슨한 파일 덤프가 아닙니다. Harness state를 뒷받침하는 모든 artifact에는 등록된 artifact ref, hash, size, redaction state, 그리고 이를 사용하는 Task 또는 owner record와의 relation이 필요합니다.

### MCP API

MCP server는 read resource와 public tool을 제공합니다. MCP resource는 read-only입니다. 상태를 변경하는 작업은 public tool과 Core를 거칩니다.

첫 Build 경로에서는 다음을 우선합니다.

- 상태와 active Task read
- intake 또는 Task creation
- next-action guidance
- `prepare_write`
- `record_run`
- 필요한 tool flow를 통한 artifact 등록
- Evidence Manifest 갱신
- `close_task` 차단 조건 동작

Public request와 response 규칙은 [MCP API와 스키마](../05-mcp-api-and-schemas.md)가 담당합니다.

### Projections

Projection은 state record와 artifact ref에서 나온 사람이 읽기 쉬운 view입니다. 기준 상태가 아닙니다.

첫 실행 가능한 조각은 최소 `TASK` projection job을 enqueue하거나 최소 `TASK` projection을 render할 수 있으면 됩니다. 최종 MVP는 원천 기록이 있을 때 MVP-required `ProjectionKind`인 `TASK`, `APR`, `RUN-SUMMARY`, `EVIDENCE-MANIFEST`, `EVAL`, `DIRECT-RESULT`를 지원해야 합니다.

Projection failure는 committed Core 상태를 rollback하면 안 됩니다. Projection이 최신인지 또는 job 상태가 어떤지 표시하고, repair나 reconcile은 이후 action에 맡깁니다.

### Operator Commands

Operator 진입점은 Core 동작 위에 놓이는 경로이지 두 번째 상태 모델이 아닙니다. 먼저 command-independent 기능으로 계획합니다.

- project connect 또는 등록
- doctor/readiness 상태 표시
- MCP 경계 제공
- projection refresh
- human edit 또는 managed-block drift reconcile
- interrupted 또는 stale operational state 복구
- state, projection, artifact ref export
- artifact 무결성 확인
- conformance fixture 실행

정확한 command name과 flag는 나중에 정해도 됩니다. 중요한 것은 operator 동작이 MCP tool과 같은 Core 상태, event, artifact, projection, error를 사용한다는 점입니다.

## 아직 만들지 않는 것

첫 구현 계획은 좁게 유지합니다. 다음은 MVP 선행 조건으로 만들지 않습니다.

- dashboard 또는 rich hosted UI
- 넓은 connector ecosystem
- team workflow, shared workspace, permission, profile import/export
- parallel orchestration automation
- 기준 agent 접점이 구체적인 pre-tool blocking 경로를 증명하지 않은 preventive guard expansion

MVP는 cooperative 또는 detective guard/freeze 상태를 표시할 수 있고, existing Change Unit, Autonomy Boundary, `prepare_write` 동작을 통해 작업을 보류하거나 범위를 좁힐 수 있습니다. 접점 label만으로 저장된 guarantee level이 올라가지는 않습니다.

## 첫 증명

첫 증명은 Kernel Smoke입니다. Harness가 하나의 권한 결정을 만들고 적용할 수 있음을 보여 주는 가장 작은 실행 가능한 경로입니다.

다음을 보여야 합니다.

- 등록된 프로젝트 하나와 기준 agent 접점 하나
- 현재 상태와 gate를 가진 Task 하나
- active scoped Change Unit 하나
- `prepare_write`가 권한 없는 쓰기를 차단하고 compatible scoped 쓰기를 허용함
- 허용된 `prepare_write`가 durable Write Authorization을 만듦
- `record_run`이 `direct` 또는 구현 Run에서 그 Write Authorization을 한 번 사용한 것으로 기록함
- artifact를 등록하고 Run 또는 근거에 연결할 수 있음
- 최소 Evidence Manifest가 support 또는 insufficiency를 기록함
- 최소 `TASK` projection이 최신이거나 적어도 durably enqueued됨
- 근거 또는 decision requirement가 없으면 `close_task`가 차단함
- 같은 동작이 basic Core fixture로 실행 가능함

Kernel Smoke는 최종 MVP가 아닙니다. 쓰기 권한 경로가 살아 있음을 증명하는 단계입니다.

## 최종 MVP 증명

최종 증명은 Agency-Hardened MVP입니다. Agent가 정직한 경계 안에서 행동하기 위해 필요한 나머지 conformance를 추가합니다.

- Decision Packet 품질과 사용자 판단 라우팅
- approval, Decision Packet, Write Authorization의 분리
- acceptance와 close 전에 남은 위험을 표시하는 규칙
- detached verification 독립성
- Manual QA 기록과 QA 차단 조건
- feedback-loop, TDD, stewardship, context-hygiene validators
- projection과 reconcile 완전성
- recovery, export, artifact integrity 동작
- broad automation을 MVP 밖에 두는 later 경계 확인
- 필수 agency conformance fixture 적용 범위

Agency-Hardened MVP는 rendered prose뿐 아니라 Core 상태, events, artifacts, projections, errors로 동작을 증명할 때 완료됩니다.

## Build 읽기 경로

Build 계층은 다음 순서로 읽습니다.

1. [구현 개요](implementation-overview.md): 무엇을 만드는지 확인합니다.
2. [첫 실행 가능한 조각](first-runnable-slice.md): 가장 먼저 계획할 최소 증명을 확인합니다.
3. [MVP 계획](mvp-plan.md): MVP-0부터 MVP-5까지 단계별 구현을 확인합니다.

그다음 정확한 동작은 현재 담당 문서를 봅니다.

- [커널 명세](../03-kernel-spec.md): entity, gate, state logic, `prepare_write`, `close_task`.
- [런타임 아키텍처](../04-runtime-architecture.md): runtime space, Core flow, artifact, projection/reconcile, guarantee level.
- [MCP API와 스키마](../05-mcp-api-and-schemas.md): public resource, tool, schema, error, artifact ref.
- [운영과 Conformance](../11-operations-and-conformance.md): operator semantics와 fixture expectation.
- 재설계 중 storage와 DDL의 현재 담당 문서: [Reference MVP](../06-reference-mvp.md). 이후 reference 경로: `reference/storage-and-ddl.md`.
