# Harness Documentation / 하네스 문서

This is the bilingual routing page for the Harness documentation set.

This repository is in documentation review. This page does not authorize Harness server/runtime implementation, generated operational files, executable fixtures, or runtime data. The first implementation/proof path remains Kernel Smoke first, Agency-Hardened MVP later, and post-MVP automation in the roadmap unless owner docs promote it.

Harness is an agency-preserving local authority kernel for AI-assisted product work. It keeps a local operating record of the work facts that need to remain followable after chat moves on.

Harness is not a chat script, prompt bundle, test harness, evaluation harness, dashboard, or replacement for the user's product repository, version control, tests, code review, or product and technical judgment.

이 문서는 Harness 문서 세트의 이중 언어 길잡이입니다.

이 저장소는 문서 검토 단계입니다. 이 페이지는 Harness server/runtime 구현, 생성된 운영 파일, 실행 가능한 fixture 파일, runtime data를 승인하지 않습니다. 첫 구현/증명 경로는 Kernel Smoke 먼저, Agency-Hardened MVP 나중, post-MVP automation은 owner 문서가 승격하기 전까지 roadmap에 둡니다.

Harness는 AI 지원 제품 작업을 위한, 사용자 판단권을 보존하는 로컬 권한 커널입니다. 대화가 지나간 뒤에도 따라갈 수 있어야 하는 작업 사실을 로컬 운영 기록으로 유지합니다.

Harness는 대화 스크립트, prompt 묶음, test harness, evaluation harness, dashboard, 사용자의 제품 저장소, 버전 관리, 테스트, 코드 리뷰, 제품 판단과 기술 판단을 대체하지 않습니다.

## Choose A Language / 언어 선택

| Language | Entry point |
|---|---|
| English | [en/README.md](en/README.md) |
| Korean | [ko/README.md](ko/README.md) |

## Reader Routes / 독자별 경로

| Reader / 독자 | English start / 영어 시작 | Korean start / 한국어 시작 |
|---|---|---|
| New reader or user / 처음 읽는 사람 또는 사용자 | [Overview](en/learn/overview.md), then [User Guide](en/use/user-guide.md) | [개요](ko/learn/overview.md), then [사용자 가이드](ko/use/user-guide.md) |
| Implementer / 구현자 | [Implementation Overview](en/build/implementation-overview.md), then [First Runnable Slice](en/build/first-runnable-slice.md) | [구현 개요](ko/build/implementation-overview.md), then [첫 실행 가능한 조각](ko/build/first-runnable-slice.md) |
| Operator or conformance author / 운영자 또는 Conformance 작성자 | [Operations And Conformance Reference](en/reference/operations-and-conformance.md#contract-map) | [운영과 Conformance 참조](ko/reference/operations-and-conformance.md#계약-위치-지도) |
| Documentation maintainer / 문서 유지보수 담당자 | [Authoring Guide](en/maintain/authoring-guide.md), then [Translation Guide](en/maintain/translation-guide.md) | [문서 작성 가이드](ko/maintain/authoring-guide.md), then [번역 가이드](ko/maintain/translation-guide.md) |

## Contract Owners / 계약 Owner

Reference docs own exact contracts. Learn, Use, and Build docs summarize for their reader and link to the owning Reference page instead of copying schemas, DDL, transition tables, fixture bodies, template bodies, enum tables, or other normative contract blocks.

엄격한 계약은 Reference 문서가 담당합니다. Learn, Use, Build 문서는 독자에게 필요한 만큼 요약하고, schema, DDL, transition table, fixture body, template body, enum table, 기타 규범적 계약 블록을 복사하지 않고 owner Reference 문서로 연결합니다.

| Need / 필요 | English owner / 영어 owner | Korean owner / 한국어 owner |
|---|---|---|
| Kernel state, gates, write authority, close / Kernel 상태, gate, 쓰기 권한, 닫기 | [Kernel Reference](en/reference/kernel.md) | [커널 참조](ko/reference/kernel.md) |
| Public MCP schemas, tools, errors / Public MCP schema, tool, error | [MCP API And Schemas](en/reference/mcp-api-and-schemas.md) | [MCP API와 스키마](ko/reference/mcp-api-and-schemas.md) |
| Runtime architecture, Core flow, guarantee levels / Runtime architecture, Core flow, guarantee level | [Runtime Architecture Reference](en/reference/runtime-architecture.md) | [런타임 아키텍처 참조](ko/reference/runtime-architecture.md) |
| Storage layout, DDL, migrations, artifacts / Storage layout, DDL, migration, artifact | [Storage And DDL](en/reference/storage-and-ddl.md) | [Storage와 DDL](ko/reference/storage-and-ddl.md) |
| Projections and rendered templates / Projection과 렌더링 template | [Document Projection Reference](en/reference/document-projection.md), [Template Reference](en/reference/templates/README.md) | [문서 Projection 참조](ko/reference/document-projection.md), [Template 참조](ko/reference/templates/README.md) |
| Operators, conformance, docs-maintenance reporting / 운영자, Conformance, docs-maintenance 보고 | [Operations And Conformance Reference](en/reference/operations-and-conformance.md) | [운영과 Conformance 참조](ko/reference/operations-and-conformance.md) |
| Agent integration and surface recipes / Agent 통합과 접점 recipe | [Agent Integration Reference](en/reference/agent-integration.md), [Surface Cookbook](en/reference/surface-cookbook.md) | [Agent 통합 참조](ko/reference/agent-integration.md), [Surface Cookbook](ko/reference/surface-cookbook.md) |
| Official terms / 공식 용어 | [Glossary Reference](en/reference/glossary.md) | [용어집 참조](ko/reference/glossary.md) |

## Roadmap / 로드맵

Post-MVP items live in each language tree's roadmap: [English](en/roadmap.md), [Korean](ko/roadmap.md). The roadmap is not part of the MVP implementation contract unless a future owner explicitly promotes an item with scope, fixtures, and fallback behavior.

Post-MVP 항목은 각 언어 트리의 Roadmap에 둡니다. [English](en/roadmap.md), [Korean](ko/roadmap.md)를 봅니다. 향후 담당자가 범위, fixture, fallback 동작을 정해 항목을 명시적으로 승격하기 전까지 Roadmap 항목은 MVP 구현 계약에 포함되지 않습니다.

## Parity / 문서 Parity

English and Korean docs keep the same file map and semantic content. Semantic parity must be maintained across `docs/en` and `docs/ko`, while Korean headings and prose may be natural Korean.

영어 문서와 한국어 문서는 같은 파일 지도와 의미상 같은 내용을 유지합니다. `docs/en`과 `docs/ko`의 의미 일치를 유지하되, 한국어 문서는 자연스러운 한국어 제목과 문장 흐름을 사용할 수 있습니다.
