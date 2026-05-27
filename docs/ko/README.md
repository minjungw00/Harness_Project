# Harness 문서

이 문서는 Harness 한국어 문서 세트의 길잡이입니다.

이 저장소는 문서 검토 단계입니다. 이 페이지는 Harness server/runtime 구현, 생성된 운영 파일, 실행 가능한 fixture 파일, runtime data를 승인하지 않습니다. 첫 구현/증명 경로는 Kernel Smoke 먼저, Agency-Hardened MVP 나중, post-MVP automation은 owner 문서가 승격하기 전까지 roadmap에 둡니다.

Harness는 AI 지원 제품 작업을 위한, 사용자 판단권을 보존하는 로컬 권한 커널입니다. 범위, 사용자 소유 판단, 쓰기 권한, 근거, 검증, QA, 수락, 남은 위험, 닫기 상태를 로컬 운영 기록으로 유지합니다.

Harness는 대화 스크립트, prompt 묶음, test harness, evaluation harness, dashboard, 사용자의 제품 저장소, 버전 관리, 테스트, 코드 리뷰, 제품 판단과 기술 판단을 대체하지 않습니다.

## 독자별 경로

| 독자 역할 | 먼저 읽을 문서 | 이어서 볼 문서 |
|---|---|---|
| 처음 읽는 사람 | [개요](learn/overview.md) | [하나의 작업으로 보는 Harness](learn/harness-in-one-task.md), 그다음 [핵심 개념](learn/concepts.md) |
| 사용자 | [사용자 가이드](use/user-guide.md) | Agent-facing 흐름이 필요하면 [Agent 세션 흐름](use/agent-session-flow.md) |
| 구현자 | [구현 개요](build/implementation-overview.md) | [첫 실행 가능한 조각](build/first-runnable-slice.md), [MVP 계획](build/mvp-plan.md), 그다음 관련 Reference owner |
| 운영자 또는 Conformance 작성자 | [운영과 Conformance 참조](reference/operations-and-conformance.md#계약-위치-지도) | [런타임 아키텍처](reference/runtime-architecture.md), [MCP API와 스키마](reference/mcp-api-and-schemas.md), [Storage와 DDL](reference/storage-and-ddl.md) |
| 문서 유지보수 담당자 | [문서 작성 가이드](maintain/authoring-guide.md) | [번역 가이드](maintain/translation-guide.md) |

## 소유권 규칙

정확한 계약은 Reference 문서가 담당합니다. Schema, DDL, gate, state transition, enum value, fixture 의미, template 본문, 공식 정의가 여기에 속합니다. Learn, Use, Build 문서는 독자에게 필요한 생각을 설명하고 Reference로 연결하며, 엄격한 계약 블록을 복사하지 않습니다.

Docs-maintenance check는 읽기 전용 리뷰 지침이며 runtime conformance나 implementation readiness가 아닙니다. Drift category와 owner-first resolution은 [문서 작성 가이드](maintain/authoring-guide.md#docs-maintenance-checks)를 사용하고, docs-maintenance profile reporting boundary는 [운영과 Conformance](reference/operations-and-conformance.md#docs-maintenance-프로필)를 사용합니다.

## Learn

정확한 계약에 들어가기 전에 전체 그림을 잡는 경로입니다.

- [개요](learn/overview.md)
- [하나의 작업으로 보는 Harness](learn/harness-in-one-task.md)
- [핵심 개념](learn/concepts.md)
- [목적과 원칙](learn/purpose-and-principles.md)

## Use

AI 지원 개발 세션을 Harness 기준으로 진행할 때 보는 경로입니다. 이 문서들은 사용자에게 보이는 흐름, 상태 해석, 결정 지점, 복구 경로를 우선합니다.

- [사용자 가이드](use/user-guide.md)
- [Agent 세션 흐름](use/agent-session-flow.md)

## Build

구현 방향을 파악하고 계획을 리뷰하기 위한 경로입니다. 첫 경로는 좁게 유지합니다. Kernel Smoke를 먼저, Agency-Hardened MVP를 나중에 두며, roadmap automation은 owner 문서가 승격하기 전까지 MVP 밖에 둡니다.

먼저 [문서 승인 상태](build/implementation-overview.md#문서-승인-상태)를 확인합니다. Maintainer가 그곳에서 첫 runtime batch 계획을 명시적으로 승인하기 전까지 Build 문서는 계획 지침이며 runtime/server 구현을 승인하지 않습니다.

- [구현 개요](build/implementation-overview.md)
- [첫 실행 가능한 조각](build/first-runnable-slice.md)
- [MVP 계획](build/mvp-plan.md)

## Reference

엄격한 계약을 찾아보는 경로입니다. 다른 경로에서 엄격한 규칙을 요약했다면 먼저 고쳐야 할 기준 문서는 해당 Reference owner입니다.

- [커널 참조](reference/kernel.md)
- [런타임 아키텍처 참조](reference/runtime-architecture.md)
- [MCP API와 스키마](reference/mcp-api-and-schemas.md)
- [Storage와 DDL](reference/storage-and-ddl.md)
- [문서 Projection 참조](reference/document-projection.md)
- [설계 품질 정책](reference/design-quality-policies.md)
- [Agent 통합 참조](reference/agent-integration.md)
- [Surface Cookbook](reference/surface-cookbook.md)
- [운영과 Conformance 참조](reference/operations-and-conformance.md)
- [용어집 참조](reference/glossary.md)
- [Template 참조](reference/templates/README.md)

## Maintain

문서와 이후 Harness 시스템의 일관성을 유지하기 위한 경로입니다. Maintain 문서는 런타임 동작이 아니라 문서 유지보수를 관리합니다.

- [문서 작성 가이드](maintain/authoring-guide.md)
- [번역 가이드](maintain/translation-guide.md)

## Roadmap

- [로드맵](roadmap.md)

Post-MVP 항목은 Roadmap에 둡니다. 향후 담당자가 범위, fixture, fallback 동작을 정해 항목을 명시적으로 승격하기 전까지 Roadmap 항목은 MVP 구현 계약에 포함되지 않습니다.

## 언어 의미 일치

영어 문서와 한국어 문서는 같은 파일 지도와 의미상 같은 내용을 유지합니다. 한국어 문서는 영어 문장을 한 줄씩 옮기기보다 자연스러운 한국어 제목과 흐름을 사용할 수 있습니다.
