# 레거시 로컬 MCP 설정

호환 명령인 `harness setup local-mcp`를 이해하거나 복구해야 할 때만 이 페이지를 사용합니다.

새 Codex 및 Claude Code 설정 예시는 `harness agent install`을 사용해야 합니다. [빠른 시작](../getting-started/quickstart.md)에서 시작한 뒤 [에이전트 호스트 설정](agent-host-setup.md)을 봅니다. 다중 저장소 사용자 범위 토폴로지는 [다중 저장소 에이전트 설정](multi-repository-agent-setup.md)에 있습니다.

`harness setup local-mcp`는 레거시 고정 프로젝트 MCP 설정을 위한 기준 범위 밖 호환 명령입니다. Agent Integration Profile 이전의 오래된 로컬 설정이나 스크립트를 진단할 때는 여전히 유용할 수 있습니다.

## 호환성 모양

레거시 설정은 고정 프로젝트와 접점 환경 변수를 사용하는 호스트 중립 MCP 조각을 생성했습니다.

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

이 모양은 직접 Codex 또는 Claude Code 설치의 현재 기준이 아닙니다. 현재 호스트 설정은 다음을 사용합니다.

```text
harness-mcp --integration <integration_id>
```

그리고 공개 도구 호출마다 프로젝트를 선택합니다.

## 현재 경로를 사용해야 하는 경우

아래가 필요하면 `harness agent install`을 사용합니다.

- 직접 Codex 설치
- 직접 Claude Code 설치
- 하나의 사용자 범위 통합이 여러 명시적으로 허용된 프로젝트를 처리하는 구성
- Host Installation 상태와 검증
- 선택적 저장소 guidance
- 관리되는 호스트 설정과 guidance의 안전한 uninstall
- 지원하지 않는 호스트를 위한 generic export

정확한 호환 동작은 [관리 CLI](../reference/admin-cli.md#local-mcp-setup-orchestration)에 남아 있습니다. 정확한 현재 프로세스 동작은 [MCP 전송](../reference/mcp-transport.md)에 남아 있습니다.
