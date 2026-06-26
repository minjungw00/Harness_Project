# 시작하기 개요

이 문서는 Volicord(볼리코드)를 처음 읽는 독자를 위한 개요입니다. 제품의 핵심 생각을 평이하게 설명하고, 정확한 계약 질문은 참조 담당 문서로 안내합니다.

<a id="what-volicord-is"></a>
## Volicord란 무엇인가

Volicord는 AI 지원 제품 작업을 위한 로컬 작업 권한 제품이자 시스템입니다. 사용자, AI 호스트, 에이전트를 위한 로컬 권한 제어 평면입니다. 핵심 생각은 단순합니다. AI 지원 작업이 빠르게 진행되더라도 사용자의 권한 근거는 계속 보여야 합니다.

Volicord 자체는 로컬 기준 기록이 아닙니다. Core가 Volicord 상태를 위한 로컬 기준 기록입니다. Volicord는 그 기록을 둘러싼 더 넓은 제품과 시스템이며, 로컬 런타임 구성 요소, 지원되는 접점, 호스트 통합 기록, 문서 경로를 포함합니다.

## 평소에 생기는 문제

사용자는 에이전트에게 제품 동작 변경, 실패 조사, 릴리스 노트 준비를 요청할 수 있습니다. 에이전트는 파일을 살피고, 계획을 제안하고, 코드를 쓰고, 테스트를 실행하고, 결과를 요약할 수 있습니다. 그 속도는 유용하지만 아래와 같은 대체를 숨길 수 있습니다.

- 작은 요청이 더 넓은 제품 변경으로 커집니다.
- 제품 결정이 구현 속에 묻힙니다.
- 한 주장에 대한 증거가 모든 것의 증거처럼 들립니다.
- 테스트 통과가 최종 수락처럼 취급됩니다.
- 사용자의 가벼운 승인이 남은 모든 판단까지 해결한 것으로 취급됩니다.

Volicord는 이런 대체를 보이게 하기 위해 존재합니다. 에이전트와 사용자가 범위, 판단, 증거, 검증 기준, 수락, 잔여 위험, 닫기 준비 상태를 서로 구분해 둘 로컬 자리를 제공합니다.

## 로컬 구성 요소

아래 이름들은 서로 연결되어 있지만 같은 뜻으로 바꿔 쓸 수 없습니다.

| 이름 | 처음 읽는 독자를 위한 의미 | 정확한 담당 문서 |
|---|---|---|
| Volicord | AI 지원 제품 작업을 위한 로컬 작업 권한 제품이자 시스템이며 권한 제어 평면입니다. | [Volicord란 무엇인가](#what-volicord-is) |
| Core | Volicord 상태를 위한 로컬 기준 기록입니다. | [Core 모델](../reference/core-model.md) |
| Volicord 구현 | 이 저장소가 유지하는 구현 집합입니다. Core, 저장소, 타입, `volicord` CLI, `volicord-mcp`, 테스트, 문서, 검증 도구를 포함하며 Volicord 전체와 같은 말은 아닙니다. | [런타임 경계](../reference/runtime-boundaries.md) |
| `volicord` | 설정, 프로젝트, 접점, 통합, 호스트, 지침 기록을 만드는 로컬 관리 CLI입니다. | [관리 CLI](../reference/admin-cli.md) |
| `volicord-mcp` | MCP 호스트가 자식 프로세스로 시작하는 stdio MCP 어댑터 프로세스입니다. | [MCP 전송](../reference/mcp-transport.md) |
| `Volicord Runtime Home` | 저장소/런타임 담당 문서가 정의하는 Volicord 운영 데이터의 로컬 런타임 데이터 공간입니다. | [런타임 경계](../reference/runtime-boundaries.md) |
| `Product Repository` | 사용자의 프로젝트 작업 공간과 제품 파일입니다. 명시적으로 선택한 통합 파일을 담을 수 있습니다. | [런타임 경계](../reference/runtime-boundaries.md) |
| 에이전트 호스트 설정 | `volicord-mcp --integration <integration_id>`를 시작하는 Codex, Claude Code, 또는 내보낸 MCP 설정입니다. | [관리 CLI](../reference/admin-cli.md) |

현재 기준 에이전트 통합은 고정 프로젝트 방식이 아니라 통합 바인딩 방식입니다. 하나의 `volicord-mcp` 프로세스는 하나의 Agent Integration Profile에 묶입니다. 각 공개 도구 호출은 그때마다 명시적으로 허용된 프로젝트 하나를 선택하고 검증합니다.

## 설정이 하는 일

에이전트 설정은 아래 일을 할 수 있습니다.

- Runtime Home 기록을 만들거나 재사용합니다.
- `Product Repository`를 등록하거나 재사용합니다.
- Agent Integration Profile과 명시적 프로젝트 허용 목록을 만듭니다.
- Codex 또는 Claude Code 호스트 설정을 설치하거나 generic 설정을 내보냅니다.
- 설정 검증을 실행하고 `complete`, `action_required`, `partial_failure`, `failed`를 보고합니다.
- 명시적으로 선택하고 승인한 경우 저장소 지침을 쓸 수 있습니다.

에이전트 설정은 아래 일을 하면 안 됩니다.

- Runtime Home의 모든 프로젝트에 접근을 부여하면 안 됩니다.
- Volicord 런타임 데이터베이스나 런타임 기록을 `Product Repository`에 저장하면 안 됩니다.
- Codex 프로젝트 신뢰, Claude Code 프로젝트 MCP 승인, OAuth, reload, restart, 또는 그 밖의 호스트 소유 동작을 우회한다고 주장하면 안 됩니다.
- 모델이 Volicord 도구를 자동으로 선택한다고 약속하면 안 됩니다.

## 처음 알아둘 권한 개념

처음 읽는 수준에서 Volicord 문서는 아래 권한 개념을 서로 구분하고, 정확한 의미는 [Core 모델](../reference/core-model.md)로 안내합니다.

- 사용자 소유 판단은 사용자에게 남습니다. 에이전트는 선택지를 설명할 수 있지만 판단을 만들어 낼 수는 없습니다.
- 증거는 기록된 특정 주장을 뒷받침합니다. 최종 수락이나 잔여 위험 수락이 아닙니다.
- 검증 기준은 무엇을 확인해야 하는지 안내합니다. 그 자체가 증거나 수락은 아닙니다.
- `Write Authorization`은 일반 쓰기 승인, 민감 동작 승인, 최종 수락, 잔여 위험 수락과 구분됩니다.
- 닫기 준비 상태는 Core 권한 개념이며 제품 정확성의 증명이 아닙니다.

정확한 권한 규칙과 대체 금지 경계는 [Core 모델](../reference/core-model.md)을 봅니다.

## Volicord가 아닌 것

이 개요는 처음 읽는 제품 정체성을 설명합니다. 정확한 기준 범위와 지원 범위 밖 경계는 [범위](../reference/scope.md#product-role-exclusions)를 사용합니다.

또한 Volicord는 잘 쓴 대화 답변, 생성된 요약, 읽기 쉬운 상태 카드, 복사한 식별자, 선택적 저장소 지침, `Projection`을 기준 기록으로 바꾸지 않습니다. 정확한 표시 경계는 [상태 보기와 템플릿](../reference/projection-and-templates.md)이, 런타임과 위치 경계는 [런타임 경계](../reference/runtime-boundaries.md)가, 보안 표현은 [보안](../reference/security.md)이 담당합니다.

## 다음 읽기 경로

| 독자 | 다음 경로 |
|---|---|
| 처음 읽는 제품 독자 | [사용자 가이드](../guides/user-workflow.md) |
| 환경 확인 | [시스템 요구사항](../reference/system-requirements.md) |
| 첫 설정 | [설치](installation.md) -> [빠른 시작](quickstart.md) |
| 에이전트 호스트 운영자 | [빠른 시작](quickstart.md) -> [에이전트 호스트 설정](../guides/agent-host-setup.md) -> [에이전트 호스트 문제 해결](../guides/agent-host-troubleshooting.md) |
| 여러 저장소 운영자 | [다중 저장소 에이전트 설정](../guides/multi-repository-agent-setup.md) |
| 에이전트 작성자 | [에이전트 가이드](../guides/agent-workflow.md) -> [에이전트 통합](../reference/agent-integration.md) |
| 소스 코드 학습자 | [구현 가이드](../development/change-guide.md) -> [아키텍처](../development/architecture.md) |
| 참조 독자 | [참조 색인](../reference/README.md), [관리 CLI](../reference/admin-cli.md), [API 메서드](../reference/api/methods.md) |

처음 읽는 독자가 Volicord를 이해하기 위해 API 스키마나 담당 문서 메타데이터부터 읽을 필요는 없습니다. 정확한 계약 담당 문서가 필요할 때는 [참조 색인](../reference/README.md)을 사용합니다.
