# 개발자 문서

이 문서는 현재 Rust 구현을 이해하려는 개발자를 위한 소스 코드 학습
진입점입니다. 어디서 시작할지, 다음에 무엇을 읽을지, 정확한 제품 계약이
어디에 있는지를 안내합니다.

이 문서 묶음은 구현 구조를 가르치기 위한 자료입니다. 공개 API 동작,
요청이나 응답 스키마, 저장 효과, 보안 보장, 런타임 경계, Core 권한
의미, 제품 계약을 정의하거나 덮어쓰지 않습니다. 정확한 동작은 연결된
집중 참조 담당 문서를 따릅니다.

하네스는 AI 지원 제품 작업을 위한 로컬 작업 권한 제품이자 시스템입니다.
Core는 하네스 상태를 위한 로컬 기준 기록입니다.

## 읽는 순서

1. 워크스페이스와 크레이트 책임: [코드베이스 둘러보기](codebase-tour.md)에서
   시작합니다. 모든 Cargo 워크스페이스 멤버, 처음 열 소스 파일, 중요한
   심볼, 관련 테스트, 다음에 읽을 컴포넌트를 확인합니다.
2. 대표 요청 흐름: [요청 생명주기](request-lifecycle.md)를 읽습니다.
   `harness.status`, `harness.intake`, `harness.prepare_write`가 MCP
   `tools/call`에서 Core와 Store 동작을 거쳐 MCP 응답 래퍼로 돌아오는
   경로를 따라갑니다.
3. 아키텍처와 경계: [구현 아키텍처](architecture.md)에서 오래 유지되는
   워크스페이스 형태, 의존 방향, 실행 흐름 지도, 효과 경로, 커밋 경계,
   관리 CLI 설정 흐름, 테스트 구조를 봅니다.
4. 저장소와 트랜잭션 개념: 아키텍처의
   [Core 파이프라인과 Store 경계](architecture.md#core-파이프라인과-store-경계)
   및 [효과와 커밋 경계](architecture.md#효과와-커밋-경계)를 읽은 뒤,
   정확한 저장소 질문은 [저장소](../reference/storage.md),
   [저장 효과](../reference/storage-effects.md),
   [저장소 기록](../reference/storage-records.md),
   [저장소 DDL](../reference/storage-ddl.md),
   [저장소 버전 관리](../reference/storage-versioning.md)로 보냅니다.
5. 테스트 구조: 둘러보기 문서의 멤버별 테스트 안내와 아키텍처의
   [테스트 구조](architecture.md#테스트-구조)를 사용합니다. 주요 경로는
   `crates/harness-core/src/methods/tests.rs`,
   `crates/harness-mcp/tests/binary_transport.rs`,
   `crates/harness-cli/tests/binary_admin.rs`,
   `tests/integration/mcp_surface.rs`,
   `tests/conformance/baseline.rs`입니다.
6. 변경 작업 흐름: 변경을 분류하고, 담당 문서를 찾고, 구현 경계를 확인하고,
   검증을 고를 준비가 되면 [구현 가이드](change-guide.md)를 사용합니다.
7. 정확한 참조 계약: [참조 색인](../reference/README.md)과
   [API 메서드](../reference/api/methods.md)를 사용합니다. 학습 문서는
   현재 코드 배치를 설명할 수 있지만, 정확한 메서드 동작, 스키마, 저장
   효과, 보안 표현, 런타임 경계, 오류 의미, Core 권한 의미는 참조 문서가
   담당합니다.

## 학습 문서와 담당 문서

| 질문 | 여기서 시작 | 정확한 담당 경로 |
|---|---|---|
| 어떤 크레이트를 먼저 열어야 하나? | [코드베이스 둘러보기](codebase-tour.md) | [구현 아키텍처](architecture.md)가 가이드 수준 구현 구조를 담당합니다. |
| 메서드 호출이 MCP, Core, Store를 지나 응답까지 어떻게 흐르나? | [요청 생명주기](request-lifecycle.md) | 메서드 동작은 [API 메서드](../reference/api/methods.md)와 연결된 메서드 담당 문서가 담당합니다. |
| 왜 Core는 CLI나 MCP에 의존하지 않나? | [구현 아키텍처](architecture.md) | 의존 경계 안내는 아키텍처에 남고, 공개 동작은 참조 담당 문서로 돌아갑니다. |
| 어떤 저장소 변이가 커밋되나? | [요청 생명주기](request-lifecycle.md)와 [구현 아키텍처](architecture.md) | 정확한 저장 효과는 [저장 효과](../reference/storage-effects.md)와 인접 저장소 담당 문서로 보냅니다. |
| 변경할 때 무엇을 고쳐야 하나? | [구현 가이드](change-guide.md) | [참조 색인](../reference/README.md) 또는 `docs/doc-index.yaml`에서 고른 집중 참조 담당 문서입니다. |

## 소스 읽기 지름길

공개 메서드 작업에서 가장 짧게 유용한 소스 경로는 아래와 같습니다.

1. [`crates/harness-types/src/methods.rs`](../../../crates/harness-types/src/methods.rs)
2. [`crates/harness-mcp/src/lib.rs`](../../../crates/harness-mcp/src/lib.rs)
3. [`crates/harness-core/src/pipeline.rs`](../../../crates/harness-core/src/pipeline.rs)
4. [`crates/harness-core/src/methods/`](../../../crates/harness-core/src/methods/)
5. [`crates/harness-store/src/core_pipeline.rs`](../../../crates/harness-store/src/core_pipeline.rs)
6. [`tests/integration/mcp_surface.rs`](../../../tests/integration/mcp_surface.rs)
7. [`tests/conformance/baseline.rs`](../../../tests/conformance/baseline.rs)

로컬 설정과 운영자 동작을 읽을 때는
[`crates/harness-cli/src/main.rs`](../../../crates/harness-cli/src/main.rs)에서
시작한 뒤
[`crates/harness-cli/src/local_mcp_command.rs`](../../../crates/harness-cli/src/local_mcp_command.rs)와
[`crates/harness-cli/src/setup.rs`](../../../crates/harness-cli/src/setup.rs)를
읽습니다.

## 경계 기억하기

- Core 쪽 코드는 CLI와 MCP 어댑터 크레이트에 의존하지 않습니다.
- `harness-mcp`는 시작과 세션 검증을 위해 Store를 직접 사용할 수 있습니다.
  이 직접 Store 사용은 공개 메서드 의미를 구현하는 다른 경로가 아닙니다.
- `Harness Runtime Home`과 `Product Repository`는 서로 다른 위치입니다.
- 테스트는 담당 문서가 정의한 사실을 검증하지만, 테스트와 픽스처는 제품
  계약 담당 문서가 아닙니다.
- 학습 문서는 소스 파일과 심볼을 이름으로 가리키며, 불안정한 줄 번호를
  사용하지 않습니다.
