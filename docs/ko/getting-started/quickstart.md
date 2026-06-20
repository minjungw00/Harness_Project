# 빠른 시작

이 문서는 가장 짧게 지원되는 로컬 MCP 성공 경로를 담당합니다. `Harness
Server` 실행 파일을 빌드했거나 찾을 수 있고, 바인딩할 `Product Repository`가
있다고 가정합니다.

빌드 세부사항, 릴리스 실행 파일 위치, 실행 파일 탐색 규칙은
[설치](installation.md)를 봅니다. 모든 설정 옵션과 문제 해결 경로는
[로컬 MCP 설정](../guides/local-mcp-setup.md)을 봅니다.

## 1단계: Harness Server 준비

작업 디렉터리: `Harness Server` 소스 저장소 루트.

```sh
cargo build -p harness-cli -p harness-mcp
```

아래 실행 파일을 사용할 수 있습니다.

- `target/debug/harness`
- `target/debug/harness-mcp`

다음 단계에서는 이 파일들을 절대 경로로 사용하거나, 같은 `harness`와
`harness-mcp` 명령을 제공하는 설치된 실행 파일을 사용합니다.

## 2단계: Product Repository 바인딩

하네스에 등록하려는 프로젝트 작업 공간에서 시작합니다. 현재 디렉터리는 명령이
`--repo-root .`을 전달하기 때문에 선택됩니다.

작업 디렉터리: `Product Repository` 루트.

```sh
/absolute/path/to/harness setup local-mcp \
  --repo-root . \
  --runtime-home /absolute/path/to/harness-runtime-home \
  --project-id demo \
  --mcp-command /absolute/path/to/harness-mcp
```

`Product Repository` 밖에 있는 `Harness Runtime Home`을 사용합니다.
`--runtime-home`을 생략하면 설정은 문서화된 `HARNESS_HOME` 또는 사용자 홈 대체
경로를 사용하지만, 선택된 Runtime Home은 여전히 `Product Repository`와
분리되어 있어야 합니다.

설정은 `Product Repository` 경로를 Runtime Home에 등록하고, 로컬 MCP 에이전트
접점을 만들거나 재사용하며, MCP 사전 점검을 실행하고, 호스트 중립 MCP 설정을
출력합니다. 외부 MCP 호스트를 설치하거나 찾아내거나 편집하지 않습니다.

설정은 하네스 데이터베이스나 런타임 아티팩트를 `Product Repository` 안에 두지
않으며, 저장소를 선택했다는 이유만으로 제품 파일을 편집하지 않습니다.
`--config-dir`를 전달하면 명시적으로 선택한 그 디렉터리에 생성된 호스트 중립
설정 조각을 쓸 수 있습니다.

## 3단계: 외부 MCP 호스트 설정

텍스트 출력에는 아래와 같은 호스트 중립 조각을 담은 `agent_config_json`이
포함됩니다.

```json
{
  "mcpServers": {
    "harness-agent": {
      "command": "/absolute/path/to/harness-mcp",
      "env": {
        "HARNESS_HOME": "/absolute/path/to/harness-runtime-home",
        "HARNESS_PROJECT_ID": "demo",
        "HARNESS_SURFACE_ID": "agent_mcp",
        "HARNESS_SURFACE_INSTANCE_ID": "agent_mcp_local"
      }
    }
  }
}
```

이 조각을 MCP 호스트가 지원하는 설정 방식에 따라 적용합니다. 실제 설정 파일,
디렉터리, 래퍼 형태는 외부 호스트가 소유합니다. 기준 로컬 MCP 프로세스는
stdio를 사용하므로 URL, TCP 포트, HTTP 엔드포인트, 소켓 경로를 설정하지
않습니다.

## 위치와 소유권

| 위치 | 소유자 | 일반적인 내용 | 설정이 자동으로 쓰는가 |
|---|---|---|---|
| `Harness Server` 소스 또는 설치 | `Harness Server` 유지보수자 또는 설치자 | `harness`, `harness-mcp`, 소스 파일 또는 설치된 실행 파일 리소스. | 소스 빌드는 Cargo 출력을 `target/` 아래에 씁니다. 로컬 MCP 설정은 실행 파일을 읽거나 호출할 뿐입니다. |
| `Harness Runtime Home` | 로컬 하네스 운영자 | 하네스 registry, 프로젝트 상태, 접점 등록, 런타임 데이터. | 예. 설정은 그곳에 로컬 기록을 만들거나 재사용합니다. |
| `Product Repository` | 제품 프로젝트 소유자 | 제품 소스, 테스트, 문서, 프로젝트 설정. | 아니요. 설정은 Runtime Home에 그 경로를 기록합니다. 선택했다는 이유만으로 하네스 데이터베이스나 런타임 아티팩트를 그곳에 두지 않습니다. |
| MCP 호스트 설정 위치 | 외부 MCP 호스트 운영자 | 생성된 환경 값으로 `harness-mcp`를 시작하는 호스트별 설정. | 아니요. 하네스는 호스트 중립 조각을 출력하거나 씁니다. 호스트 자체 설정은 계속 호스트가 소유합니다. |

`--config-dir`는 `harness-agent.mcp.json` 같은 생성된 호스트 중립 조각을 위한
명시적 출력 위치입니다. 호스트 운영자가 그 조각을 의도적으로 복사하거나
맞추어 넣지 않는 한 외부 호스트의 설정 위치 자체가 아닙니다.

## 성공 확인

성공한 설정에는 아래와 같은 줄이 포함됩니다.

```text
setup: complete
project_id: demo
repo_root: /absolute/path/to/product-repository
agent_surface_id: agent_mcp
agent_surface_instance_id: agent_mcp_local
preflight: passed
agent_preflight: passed
```

이는 사람이 읽는 명령 출력이지 공개 API 스키마가 아닙니다. `preflight: passed`는
로컬 MCP 프로세스 바인딩이 검증되었다는 뜻입니다. 이후 MCP 전송 성공은 여전히
하네스 도메인 수락과 구분됩니다. 클라이언트는 파싱한 하네스 응답에서 도메인
결과나 거절을 확인해야 합니다.

## 의도적인 자체 적용

`Harness Server` 소스 저장소 자체를 dogfooding용 `Product Repository`로
의도적으로 선택할 수 있습니다. 이때도 그 체크아웃에서 `--repo-root .`을 쓰거나
다른 디렉터리에서 그 경로를 전달해 명시적으로 선택해야 합니다. 이는 일반 설치
흐름이 아닙니다.

## 계속 읽기

- 전체 설정 운영, dry-run 미리보기, JSON 출력, 설정 파일, 대화형 설정, 복구, 문제 해결: [로컬 MCP 설정](../guides/local-mcp-setup.md)
- 에이전트 작업 흐름: [에이전트 가이드](../guides/agent-workflow.md)
- 정확한 `harness` 설정 동작: [관리 CLI](../reference/admin-cli.md#local-mcp-setup-orchestration)
- 정확한 `harness-mcp` 프로세스 동작: [MCP 전송](../reference/mcp-transport.md)
- 정확한 런타임 위치 경계: [런타임 경계](../reference/runtime-boundaries.md)
