# 로컬 MCP 빠른 시작

하나의 `Product Repository`에 대해 일반적인 로컬 MCP 설정 경로가 필요할 때 이 가이드를 사용합니다.

`harness`가 로컬 설정을 수행합니다. `harness-mcp`는 설정 뒤 MCP 호스트가 자식 프로세스로 시작하는 실행 파일입니다. 로컬 MCP 프로세스는 네트워크 포트, URL, 소켓, 리스너가 아니라 stdio로 통신합니다. 설정 명령은 알맞은 호스트별 설정에 복사할 수 있는 호스트 중립 설정을 생성합니다. 알 수 없는 외부 호스트를 설치하거나 찾아내거나 편집하지 않습니다.

이 문서는 작업 중심 온보딩 순서만 담당합니다. 정확한 `harness` 명령 동작은 [관리 CLI](../reference/admin-cli.md#local-mcp-setup-orchestration)가 담당합니다. 정확한 `harness-mcp` 프로세스 동작, stdio 프레이밍, 응답 래핑, 사전 점검, 종료, 재연결은 [MCP 전송](../reference/mcp-transport.md)이 담당합니다. 런타임 위치 경계는 [런타임 경계](../reference/runtime-boundaries.md)가 담당합니다. 접점 역할과 행위자 출처 경계는 [에이전트 통합](../reference/agent-integration.md)이 담당합니다.

## 1. 실행 파일 빌드

하네스 저장소 루트에서 실행합니다.

```sh
cargo build --release -p harness-cli -p harness-mcp
```

예상되는 release 실행 파일:

- `target/release/harness`
- `target/release/harness-mcp`

옵션 없는 설정은 아래 조건에서 `harness-mcp`를 찾을 수 있습니다.

- release 빌드 출력처럼 `harness-mcp`가 `harness` 옆에 있음
- `harness-mcp`가 `PATH`에 있음

특정 MCP 실행 파일을 지정해야 할 때만 `--mcp-command`를 사용합니다.

## 2. 일반 설정 경로 실행

바인딩하려는 제품 저장소에서 실행합니다.

```sh
cd /absolute/path/to/product-repository
/absolute/path/to/harness setup local-mcp
```

옵션 없는 경로는 가이드 수준에서 아래 기본값을 사용합니다.

- 현재 디렉터리를 `Product Repository`로 사용합니다.
- Runtime Home은 일반 공유 선택 규칙을 사용합니다.
- 정확히 같은 저장소 경로를 가진 기존 `active` 프로젝트가 있으면 재사용합니다.
- 그렇지 않으면 최종 디렉터리 이름에서 프로젝트 ID를 파생합니다.
- 에이전트 MCP 접점은 기준 워크플로 프로필을 사용합니다.
- 사용자 상호작용은 만들지 않습니다.
- 등록 뒤 사전 점검을 자동으로 실행합니다.
- 호스트 중립 에이전트 설정을 출력합니다.

정확한 기본값, 충돌 처리, 종료 코드, 스트림 동작은 [관리 CLI](../reference/admin-cli.md#local-mcp-setup-orchestration)를 봅니다.

## 3. 필요할 때 명시 경로 사용

고정 위치가 필요한 운영자는 직접 값을 지정할 수 있습니다.

```sh
/absolute/path/to/harness setup local-mcp \
  --runtime-home /absolute/path/to/harness-runtime-home \
  --repo-root /absolute/path/to/product-repository \
  --project-id demo \
  --mcp-command /absolute/path/to/harness-mcp
```

자세한 옵션 동작은 [관리 CLI](../reference/admin-cli.md#local-mcp-setup-orchestration)로 보냅니다.

## 4. 설정 결과 읽기

성공한 text 결과에는 아래와 같은 중요한 줄이 포함됩니다. 이는 사람이 읽는 명령 출력이지 공개 API 스키마가 아닙니다.

```text
setup: complete
runtime_home: ...
project_id: ...
agent_surface_id: agent_mcp
agent_surface_instance_id: agent_mcp_local
mcp_command: ...
preflight: passed
```

actions 절은 설정이 수행한 일을 식별합니다.

- `created`는 설정이 빠진 Runtime Home, 프로젝트, 접점 기록을 추가했다는 뜻입니다.
- `reused`는 호환되는 기존 기록을 유지했다는 뜻입니다.
- `updated`는 관리 CLI가 담당하는 명시적 충돌 처리 경로를 통해서만 대상 접점을 교체했다는 뜻입니다.

출력된 `agent_config_json`은 일반 에이전트 프로세스를 위한 호스트 중립 조각입니다.

```json
{
  "mcpServers": {
    "harness-agent": {
      "command": "/absolute/path/to/harness-mcp",
      "env": {
        "HARNESS_HOME": "/absolute/path/to/runtime-home",
        "HARNESS_PROJECT_ID": "project-id",
        "HARNESS_SURFACE_ID": "agent_mcp",
        "HARNESS_SURFACE_INSTANCE_ID": "agent_mcp_local"
      }
    }
  }
}
```

이 조각을 운영하는 MCP 호스트가 사용하는 설정 위치와 래퍼 형태에 맞게 넣습니다. 기준 로컬 MCP 프로세스에는 URL, TCP 포트, HTTP 엔드포인트, 소켓 경로를 설정하지 않습니다.

## 5. 선택적 사용자 상호작용 커넥터 설정

실제 사용자 대상 UI나 커넥터가 사용자 행동을 제출할 때만 사용자 상호작용 바인딩을 추가합니다.

```sh
harness setup local-mcp --with-user-interaction
```

이 명령은 별도의 `user_interaction` 접점을 만들고 사전 점검한 뒤, 별도의 `harness-user-interaction` 설정을 출력합니다. 이 바인딩을 일반 에이전트 설정에 합치지 않습니다.

사용자 상호작용 설정은 알맞은 UI나 커넥터를 위한 것이며, 일반 에이전트 호스트를 위한 것이 아닙니다.

```json
{
  "mcpServers": {
    "harness-user-interaction": {
      "command": "/absolute/path/to/harness-mcp",
      "env": {
        "HARNESS_HOME": "/absolute/path/to/runtime-home",
        "HARNESS_PROJECT_ID": "project-id",
        "HARNESS_SURFACE_ID": "user_ui",
        "HARNESS_SURFACE_INSTANCE_ID": "user_ui_local"
      }
    }
  }
}
```

`actor_kind=user`만으로는 충분하지 않습니다. 권한을 지니는 사용자 행동에는 적절한 사용자 대상 UI나 커넥터가 별도 `user_interaction` 바인딩을 사용해야 합니다. 정확한 행위자 출처 규칙은 [에이전트 통합](../reference/agent-integration.md#current-surface-context)에 있습니다.

## 6. 설정 파일 쓰기

호스트 중립 조각을 디렉터리에 쓰려면 아래처럼 실행합니다.

```sh
harness setup local-mcp --config-dir /absolute/path/to/generated-mcp-config
```

예상 파일:

```text
harness-agent.mcp.json
harness-user-interaction.mcp.json
```

`harness-user-interaction.mcp.json`은 `--with-user-interaction`을 요청했을 때만 존재합니다.

기존 파일은 기본적으로 덮어쓰지 않습니다. 선택한 설정 디렉터리의 생성 파일을 의도적으로 교체할 때만 `--overwrite-config`를 사용합니다. 이 파일들은 호스트 중립 조각입니다. 설정은 외부 호스트의 설정 위치를 추측하지 않습니다.

## 7. 설정 미리보기 또는 자동화

상태 변경 없이 미리 보려면 아래처럼 실행합니다.

```sh
harness setup local-mcp --dry-run
```

`dry-run`은 경로 해석, 계획, 실행 파일 탐색, 설정 렌더링, 충돌 분석을 수행합니다. 등록, 사전 점검, Runtime Home 생성, SQLite 쓰기, 설정 파일 쓰기는 수행하지 않습니다.

자동화에서는 JSON 출력을 요청합니다.

```sh
harness setup local-mcp --output json
```

JSON 모드는 stdout에 기계가 읽을 수 있는 성공 객체 하나를 출력합니다. 자동화는 사람이 읽는 텍스트를 파싱하지 말고 JSON을 사용해야 합니다. 오류는 계속 stderr와 프로세스 상태를 사용합니다. JSON 출력은 관리 CLI 출력이지 공개 하네스 API 응답 스키마가 아닙니다.

## 8. 선택적 대화형 마법사

같은 입력을 프롬프트로 확인하려면 마법사를 사용합니다.

```sh
harness setup local-mcp --interactive
```

마법사는 선택 사항이며 터미널이 필요합니다. 에이전트 바인딩과 접근 등급을 보여 주고, 사용자 상호작용 커넥터 기본값은 no로 둡니다. 파괴적 교체와 설정 덮어쓰기는 명시적 확인을 요구하고, 최종 확인에서 취소하면 아무것도 쓰지 않습니다. 비대화식 명령과 같은 설정 엔진을 사용합니다. 정확한 프롬프트 동작은 [관리 CLI](../reference/admin-cli.md#local-mcp-setup-orchestration)에 있습니다.

## 9. 연결과 도구 탐색 확인

호스트가 에이전트 프로세스를 시작한 뒤 아래 MCP 순서를 확인합니다.

1. `initialize`를 보냅니다.
2. `notifications/initialized`를 보냅니다.
3. `tools/list`를 보냅니다.
4. `harness.status`를 호출합니다.

예상 관찰 결과:

- `initialize`가 `serverInfo.name`을 `harness-mcp`로 반환합니다.
- `tools/list`가 공개 하네스 도구 정확히 아홉 개를 노출합니다.
- `harness.status`는 `result.content[0].text`에 직렬화된 하네스 JSON을 담은 MCP text content를 반환합니다.
- 클라이언트는 `result.content[0].text`를 파싱하고 `base.response_kind`와 `errors`를 확인합니다.
- `isError: false`는 MCP 전송 성공을 뜻합니다. 하네스 도메인 수락을 증명하지 않습니다.

정확한 공개 메서드 목록은 [API 메서드](../reference/api/methods.md)가 담당합니다. 정확한 MCP 와이어 동작과 응답 래핑은 [MCP 전송](../reference/mcp-transport.md)이 담당합니다.

원시 stdio 점검은 한 줄에 JSON 값 하나를 사용합니다.

```text
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"harness-quickstart","version":"0.0.0"}}}
{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"harness.status","arguments":{"envelope":{"project_id":"demo","task_id":null,"actor_kind":"agent","surface_id":"agent_mcp","request_id":"req_quickstart_status","idempotency_key":null,"expected_state_version":null,"dry_run":false,"locale":"en-US"},"include":{"task":true,"pending_user_judgments":true,"write_authority":false,"evidence":false,"close":true,"guarantees":true}}}}
```

## 10. 중지와 재연결

MCP 호스트는 stdin을 닫거나 자식 프로세스를 종료해서 로컬 세션을 끝냅니다. stdin EOF는 stdout을 플러시한 뒤 stdio 세션을 끝냅니다.

SQLite 상태는 Runtime Home에 남습니다. 같은 `HARNESS_HOME`, `HARNESS_PROJECT_ID`, `HARNESS_SURFACE_ID`, `HARNESS_SURFACE_INSTANCE_ID`로 새 `harness-mcp` 자식 프로세스를 시작하면 같은 저장된 프로젝트 상태에 다시 연결합니다. 프로젝트, 접점, 접점 인스턴스를 바꾸려면 다른 프로세스가 필요합니다.

## 고급 수동 설정과 복구

사용자 지정 ID가 필요하거나, 충돌을 진단하거나, 부분 설정을 복구하거나, 등록 상태를 살펴보거나, `harness setup local-mcp`를 사용할 수 없는 자동화를 지원해야 할 때 낮은 수준의 명령을 사용합니다.

Runtime Home을 초기화합니다.

```sh
HARNESS_HOME=/absolute/path/to/harness-runtime-home \
/absolute/path/to/harness init
```

제품 저장소를 등록합니다.

```sh
HARNESS_HOME=/absolute/path/to/harness-runtime-home \
/absolute/path/to/harness project register \
  --project-id demo \
  --repo-root /absolute/path/to/product-repository
```

에이전트 MCP 접점을 등록합니다.

```sh
HARNESS_HOME=/absolute/path/to/harness-runtime-home \
/absolute/path/to/harness surface register \
  --project-id demo \
  --surface-id agent_mcp \
  --surface-instance-id agent_mcp_local \
  --kind mcp \
  --interaction-role agent \
  --profile baseline-workflow
```

선택적으로 별도의 사용자 상호작용 접점을 등록합니다.

```sh
HARNESS_HOME=/absolute/path/to/harness-runtime-home \
/absolute/path/to/harness surface register \
  --project-id demo \
  --surface-id user_ui \
  --surface-instance-id user_ui_local \
  --kind mcp \
  --interaction-role user_interaction \
  --access-class read_status \
  --access-class core_mutation
```

등록 상태를 확인합니다.

```sh
HARNESS_HOME=/absolute/path/to/harness-runtime-home \
/absolute/path/to/harness surface list --project-id demo
```

직접 MCP 사전 점검을 실행합니다.

```sh
HARNESS_HOME=/absolute/path/to/harness-runtime-home \
HARNESS_PROJECT_ID=demo \
HARNESS_SURFACE_ID=agent_mcp \
HARNESS_SURFACE_INSTANCE_ID=agent_mcp_local \
/absolute/path/to/harness-mcp --check
```

에이전트 바인딩에서는 `configuration: valid`, `transport: stdio`, `interaction_role: agent`, `baseline_workflow_access: full`을 예상합니다. 자세한 시작 검증과 실패 조건은 [MCP 전송](../reference/mcp-transport.md#configuration-preflight)에 둡니다.

## 문제 해결

| 증상 | 가능한 원인 | 다음 행동 |
|---|---|---|
| `harness-mcp`를 찾을 수 없습니다. | `harness` 옆에 없고 `PATH`에도 없습니다. | 두 release 실행 파일을 빌드하거나, `harness-mcp`를 `PATH`에 추가하거나, `--mcp-command /absolute/path/to/harness-mcp`를 전달합니다. |
| 저장소 디렉터리에서 쓸 수 있는 ID를 파생할 수 없습니다. | 설정이 최종 디렉터리 이름에서 유효한 프로젝트 ID를 파생할 수 없습니다. | `--project-id`를 지정해 다시 실행합니다. |
| 같은 저장소에 여러 프로젝트가 일치합니다. | 등록된 프로젝트 둘 이상이 정규화된 저장소 경로를 가리킵니다. | 의도한 `--project-id`로 다시 실행하거나 관리 CLI 명령으로 등록 상태를 확인합니다. |
| 프로젝트 ID가 다른 저장소를 가리킵니다. | 선택한 `--project-id`가 이미 다른 `repo_root`에 등록되어 있습니다. | 올바른 프로젝트 ID나 저장소를 선택합니다. 설정은 프로젝트 ID를 재바인딩하지 않습니다. |
| 기존 에이전트 접점이 호환되지 않습니다. | 대상 접점이 다른 역할, 종류, 접근 집합, 또는 MCP 시작 메타데이터로 존재합니다. | `harness surface list`로 확인합니다. 해당 대상 접점을 교체하려는 의도가 분명할 때만 관리 CLI 충돌 처리 경로를 사용합니다. |
| 생성 파일이 이미 있습니다. | `--config-dir`가 기존 생성 조각을 가리킵니다. | 다른 디렉터리를 선택하거나 교체가 의도된 경우 `--overwrite-config`로 다시 실행합니다. |
| 등록 뒤 사전 점검이 실패합니다. | 등록은 성공했지만 `harness-mcp --check`가 바인딩이나 환경을 거절했습니다. | 사전 점검 진단을 읽고 바인딩이나 실행 파일 경로를 고친 뒤 설정을 다시 실행합니다. 설정은 안전하게 다시 실행할 수 있도록 설계되어 있습니다. |
| 에이전트가 예상과 달리 읽기 전용입니다. | 에이전트 접점을 수동으로 만들면서 기준 워크플로 프로필이나 동등한 접근 집합을 주지 않았습니다. | 설정을 다시 실행하거나 `--profile baseline-workflow`로 에이전트 접점을 등록합니다. |
| 사용자 상호작용 커넥터가 설정되지 않았습니다. | 일반 설정 경로는 에이전트 바인딩만 만듭니다. | `--with-user-interaction`으로 다시 실행하고, 별도 `harness-user-interaction` 조각을 실제 UI나 커넥터에 연결합니다. |
| JSON-RPC 성공을 하네스 수락과 혼동했습니다. | 클라이언트가 `isError`만 확인했습니다. | `result.content[0].text`를 파싱하고 `base.response_kind`와 `errors`를 확인합니다. |

정확한 설정 동작은 [관리 CLI](../reference/admin-cli.md#local-mcp-setup-orchestration)로, 정확한 MCP 프로세스 동작은 [MCP 전송](../reference/mcp-transport.md)으로 보냅니다.
