# 빠른 시작

이 문서는 가장 짧게 지원되는 로컬 MCP 성공 경로를 담당합니다. 이 저장소의 로컬 복제본에서 시작하고, 바인딩할 `Product Repository` 디렉터리가 있다고 가정합니다.

빌드 세부사항, 릴리스 실행 파일 위치, 실행 파일 탐색 규칙은 [설치](installation.md)를 봅니다. 모든 설정 옵션과 문제 해결 경로는 [로컬 MCP 설정](../guides/local-mcp-setup.md)을 봅니다.

## 1. 두 실행 파일 빌드

하네스 저장소 루트에서 실행합니다.

```sh
cargo build -p harness-cli -p harness-mcp
```

아래 실행 파일을 사용할 수 있습니다.

- `target/debug/harness`
- `target/debug/harness-mcp`

## 2. Product Repository 선택

기존 프로젝트 디렉터리를 고르거나 하네스 문서 트리 밖에 새 디렉터리를 만듭니다. 그 절대 경로를 `Product Repository`로 사용합니다.

아래 명령에서 값을 바꿉니다.

- `/absolute/path/to/product-repository`: 제품 저장소 경로
- `/absolute/path/to/harness-runtime-home`: 이 로컬 설정에 사용할 런타임 홈 경로

## 3. 로컬 MCP 설정 실행

하네스 저장소 루트에서 실행합니다.

```sh
target/debug/harness setup local-mcp \
  --repo-root /absolute/path/to/product-repository \
  --runtime-home /absolute/path/to/harness-runtime-home \
  --project-id demo \
  --mcp-command "$(pwd)/target/debug/harness-mcp"
```

이 명령은 선택한 `Product Repository`를 등록하고, 로컬 MCP 에이전트 접점을 만들거나 재사용하고, MCP 사전 점검을 실행하고, 호스트 중립 MCP 설정을 출력합니다. 외부 MCP 호스트를 설치하거나 찾아내거나 편집하지 않습니다.

## 4. 생성된 설정 읽기

텍스트 출력에는 아래와 같은 호스트 중립 조각을 담은 `agent_config_json`이 포함됩니다.

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

이 조각을 운영하는 MCP 호스트가 사용하는 래퍼 형태와 설정 위치에 넣습니다. 기준 로컬 MCP 프로세스는 stdio를 사용합니다. URL, TCP 포트, HTTP 엔드포인트, 소켓 경로를 설정하지 않습니다.

## 5. 성공 확인

성공한 설정에는 아래와 같은 줄이 포함됩니다.

```text
setup: complete
project_id: demo
agent_surface_id: agent_mcp
agent_surface_instance_id: agent_mcp_local
preflight: passed
agent_preflight: passed
```

이는 사람이 읽는 명령 출력이지 공개 API 스키마가 아닙니다. `preflight: passed`는 로컬 MCP 프로세스 바인딩이 검증되었다는 뜻입니다. 이후 MCP 전송 성공은 여전히 하네스 도메인 수락과 구분됩니다. 클라이언트는 파싱한 하네스 응답에서 도메인 결과나 거절을 확인해야 합니다.

## 6. 계속 읽기

- 전체 설정 운영, dry-run 미리보기, JSON 출력, 설정 파일, 대화형 설정, 복구, 문제 해결: [로컬 MCP 설정](../guides/local-mcp-setup.md)
- 에이전트 작업 흐름: [에이전트 가이드](../guides/agent-workflow.md)
- 정확한 `harness` 설정 동작: [관리 CLI](../reference/admin-cli.md#local-mcp-setup-orchestration)
- 정확한 `harness-mcp` 프로세스 동작: [MCP 전송](../reference/mcp-transport.md)
