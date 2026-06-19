# 로컬 MCP 빠른 시작

이 가이드는 운영자가 로컬 실행 파일을 빌드하고, `Harness Runtime Home`을 초기화하고, `Product Repository` 하나를 등록하고, MCP 접점 두 개를 바인딩하고, 바인딩을 사전 점검하고, MCP 호스트를 연결하고, 도구 탐색을 확인하고, `tools/call` 응답을 해석하고, 프로세스를 다시 연결하는 흐름을 안내합니다.

이 문서는 실행 가능한 로컬 설정 순서만 담당합니다. 정확한 `harness` 명령 동작은 [관리 CLI](../reference/admin-cli.md)가 담당합니다. 정확한 `harness-mcp` 프로세스 동작, stdio 프레이밍, 응답 래핑, 사전 점검, 종료, 재연결은 [MCP 전송](../reference/mcp-transport.md)이 담당합니다. 런타임 위치 경계는 [런타임 경계](../reference/runtime-boundaries.md)가 담당합니다. 접점 역할과 행위자 출처 경계는 [에이전트 통합](../reference/agent-integration.md)이 담당합니다.

## 시작 전 준비

하나의 예시를 기준으로 삼고 자리표시자를 절대 경로로 바꿉니다.

```text
HARNESS_HOME=/absolute/path/to/harness-runtime-home
PRODUCT_REPO=/absolute/path/to/product-repository
HARNESS_BIN=/absolute/path/to/Harness_Project/target/release/harness
HARNESS_MCP_BIN=/absolute/path/to/Harness_Project/target/release/harness-mcp
```

아래 명령 예시는 흐름을 읽기 쉽게 하려고 POSIX 계열 셸 변수와 환경 구문을 사용합니다. 다른 셸은 다른 구문을 쓰며, 일부 플랫폼에서는 `.exe` 같은 실행 파일 접미사가 붙습니다. 같은 명령 이름, 인자, 환경 의미를 유지하되 사용하는 셸에 맞게 바꿉니다.

`PRODUCT_REPO`는 이미 접근 가능한 디렉터리여야 합니다. `HARNESS_HOME`은 절대 Runtime Home 경로로 두어 관리 CLI와 MCP 자식 프로세스가 같은 로컬 런타임 데이터 위치를 쓰게 합니다. 정확한 경로 선택 규칙은 [관리 CLI](../reference/admin-cli.md#runtime-home-selection)에 있습니다.

## 1. 실행 파일 빌드

저장소 루트에서 실행합니다.

```sh
cargo build --release -p harness-cli -p harness-mcp
```

예상되는 release 실행 파일:

- `target/release/harness`
- `target/release/harness-mcp`

`harness`는 로컬 관리 설정을 수행합니다. `harness-mcp`는 MCP 호스트가 자식 프로세스로 시작하는 실행 파일입니다. `harness-mcp`는 stdio를 사용하며 네트워크 포트나 URL은 관련되지 않습니다.

## 2. Runtime Home 초기화

관리 실행 파일을 확인한 뒤 선택한 Runtime Home을 초기화합니다.

```sh
"$HARNESS_BIN" --version
HARNESS_HOME="$HARNESS_HOME" "$HARNESS_BIN" init
```

`harness init`은 선택한 Runtime Home registry를 만들거나 검증합니다. 높은 수준에서 명령 출력에는 `registry.sqlite`로 끝나는 `registry_db` 경로가 포함됩니다.

## 3. Product Repository 등록

로컬 프로젝트를 등록하고 확인합니다.

```sh
HARNESS_HOME="$HARNESS_HOME" "$HARNESS_BIN" project register --project-id demo --repo-root "$PRODUCT_REPO"
HARNESS_HOME="$HARNESS_HOME" "$HARNESS_BIN" project list
```

`--repo-root`는 이미 존재하고 접근 가능한 디렉터리여야 합니다. `Product Repository`와 `Harness Runtime Home`은 서로 다른 위치입니다. 정확한 경계는 [런타임 경계](../reference/runtime-boundaries.md)를 봅니다.

## 4. MCP 접점 등록

### 에이전트 MCP 접점

기준 워크플로 프로필을 써서 에이전트 역할 MCP 접점을 등록합니다.

```sh
HARNESS_HOME="$HARNESS_HOME" "$HARNESS_BIN" surface register \
  --project-id demo \
  --surface-id agent_mcp \
  --surface-instance-id agent_mcp_local \
  --kind mcp \
  --interaction-role agent \
  --profile baseline-workflow
```

주의: `--profile baseline-workflow`를 생략하고 명시적인 `--access-class` 옵션도 생략하면 `read_status`만 가진 접점이 만들어집니다. 이 접점은 상태 읽기에는 응답할 수 있지만, `core_mutation`, `write_authorization`, `artifact_registration`, `run_recording`이 필요한 기준 워크플로 호출은 지원할 수 없습니다.

### 사용자 상호작용 MCP 접점

별도의 `user_interaction` 접점을 등록합니다.

```sh
HARNESS_HOME="$HARNESS_HOME" "$HARNESS_BIN" surface register \
  --project-id demo \
  --surface-id user_ui \
  --surface-instance-id user_ui_local \
  --kind mcp \
  --interaction-role user_interaction \
  --access-class read_status \
  --access-class core_mutation
```

이 접점은 에이전트 접점과 분리해서 유지합니다.

- `actor_kind=user`만으로는 사용자 권한이 성립하지 않습니다.
- `agent` 역할 접점은 요청 텍스트를 바꾼다고 `user_interaction` 접점이 되지 않습니다.
- 권한을 지니는 사용자 판단에는 등록된 `user_interaction` 접점에 묶인 프로세스가 필요합니다.
- 실제 사용자용 UI나 커넥터는 그 사용자 행동에 대해 이 바인딩을 호출해야 합니다.

정확한 행위자 출처 규칙은 [에이전트 통합](../reference/agent-integration.md#current-surface-context)에 있습니다.

## 5. 등록 상태 확인

등록된 접점을 나열합니다.

```sh
HARNESS_HOME="$HARNESS_HOME" "$HARNESS_BIN" surface list --project-id demo
```

`agent_mcp`의 인스턴스가 `agent_mcp_local`이고, `user_ui`의 인스턴스가 `user_ui_local`인지 확인합니다. 명시적 인스턴스 ID를 쓰면 각 MCP 프로세스가 암묵적 인스턴스 선택에 기대지 않고 알려진 `surface_instance_id` 하나에 바인딩되므로 호스트 시작이 결정적입니다.

## 6. MCP 사전 점검 실행

호스트를 연결하기 전에 에이전트 바인딩을 사전 점검합니다.

```sh
HARNESS_HOME="$HARNESS_HOME" \
HARNESS_PROJECT_ID=demo \
HARNESS_SURFACE_ID=agent_mcp \
HARNESS_SURFACE_INSTANCE_ID=agent_mcp_local \
"$HARNESS_MCP_BIN" --check
```

보고서에는 아래 항목이 포함되어야 합니다.

- `configuration: valid`
- `transport: stdio`
- 절대 `runtime_home`
- `project_id: demo`
- `surface_id: agent_mcp`
- `surface_instance_id: agent_mcp_local`
- `interaction_role: agent`
- 등록된 `access_classes`
- `baseline_workflow_access: full`

`user_interaction` 바인딩도 사전 점검합니다.

```sh
HARNESS_HOME="$HARNESS_HOME" \
HARNESS_PROJECT_ID=demo \
HARNESS_SURFACE_ID=user_ui \
HARNESS_SURFACE_INSTANCE_ID=user_ui_local \
"$HARNESS_MCP_BIN" --check
```

`user_interaction` 접점에서는 `interaction_role: user_interaction`, `access_classes: read_status,core_mutation`, `baseline_workflow_access: not_applicable`을 예상합니다.

자세한 시작 검증과 실패 조건은 [MCP 전송](../reference/mcp-transport.md#configuration-preflight)에 둡니다.

## 7. MCP 호스트 설정

호스트가 로컬 자식 프로세스 두 개를 시작하도록 설정합니다. 이 JSON 형태 예시는 호스트 중립입니다. 실제 호스트 설정 파일 이름, 자식 프로세스 항목을 감싸는 키, 래퍼 구문은 호스트마다 다릅니다. 명령 경로와 환경 의미는 하네스가 정의합니다.

```json
{
  "mcpServers": {
    "harness-agent": {
      "command": "/absolute/path/to/Harness_Project/target/release/harness-mcp",
      "env": {
        "HARNESS_HOME": "/absolute/path/to/harness-runtime-home",
        "HARNESS_PROJECT_ID": "demo",
        "HARNESS_SURFACE_ID": "agent_mcp",
        "HARNESS_SURFACE_INSTANCE_ID": "agent_mcp_local"
      }
    },
    "harness-user-interaction": {
      "command": "/absolute/path/to/Harness_Project/target/release/harness-mcp",
      "env": {
        "HARNESS_HOME": "/absolute/path/to/harness-runtime-home",
        "HARNESS_PROJECT_ID": "demo",
        "HARNESS_SURFACE_ID": "user_ui",
        "HARNESS_SURFACE_INSTANCE_ID": "user_ui_local"
      }
    }
  }
}
```

기준 로컬 MCP 프로세스에는 URL, TCP 포트, HTTP 엔드포인트, 소켓 경로를 설정하지 않습니다.

## 8. 연결과 도구 탐색 확인

호스트가 에이전트 프로세스를 시작한 뒤 아래 MCP 순서를 확인합니다.

1. `initialize`를 보냅니다.
2. initialized notification을 보냅니다.
3. `tools/list`를 보냅니다.
4. `harness.status`를 호출합니다.

예상 관찰 결과:

- `initialize`가 `serverInfo.name`을 `harness-mcp`로 반환합니다.
- `tools/list`가 공개 하네스 도구 정확히 아홉 개를 노출합니다.
- 정확한 공개 메서드 목록은 [API 메서드](../reference/api/methods.md)가 담당합니다.
- `harness.status`는 직렬화된 하네스 JSON을 담은 MCP text content를 반환합니다.

원시 stdio 점검은 한 줄에 JSON 값 하나를 사용합니다.

```text
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"harness-quickstart","version":"0.0.0"}}}
{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"harness.status","arguments":{"envelope":{"project_id":"demo","task_id":null,"actor_kind":"agent","surface_id":"agent_mcp","request_id":"req_quickstart_status","idempotency_key":null,"expected_state_version":null,"dry_run":false,"locale":"en-US"},"include":{"task":true,"pending_user_judgments":true,"write_authority":false,"evidence":false,"close":true,"guarantees":true}}}}
```

## 9. `tools/call` 응답 해석

파싱 계층은 두 개입니다.

1. MCP `tools/call` 결과에서 `result.content[0].text`를 읽습니다.
2. 그 문자열을 하네스 JSON으로 파싱합니다.

그 뒤 파싱한 하네스 응답에서 아래를 확인합니다.

- `base.response_kind`
- `errors`

`isError: false`는 MCP 전송 호출이 성공했다는 뜻입니다. 하네스 요청이 수락되었다는 뜻은 아닙니다. 하네스 도메인 수준 `rejected` 응답도 성공한 MCP 전송을 사용하며 `isError: false`와 함께 나타날 수 있습니다.

JSON-RPC `error`는 다릅니다. 이는 프로토콜, 잘못된 파라미터, 어댑터/내부 실패에 사용합니다. 정확한 응답 스키마와 오류 의미는 [API 코어 스키마](../reference/api/schema-core.md#common-response), [API 오류 처리 경로](../reference/api/error-routing.md), [API 오류 코드](../reference/api/error-codes.md), [API 오류 세부사항](../reference/api/error-details.md)에 있습니다.

## 10. 중지와 재연결

MCP 호스트는 stdin을 닫거나 자식 프로세스를 종료해서 로컬 세션을 끝냅니다. stdin EOF는 stdout을 플러시한 뒤 stdio 세션을 끝냅니다.

SQLite 상태는 Runtime Home에 남습니다. 같은 `HARNESS_HOME`, `HARNESS_PROJECT_ID`, `HARNESS_SURFACE_ID`, `HARNESS_SURFACE_INSTANCE_ID`로 새 `harness-mcp` 자식 프로세스를 시작하면 같은 저장된 프로젝트 상태에 다시 연결합니다. 프로젝트, 접점, 접점 인스턴스를 바꾸려면 다른 프로세스가 필요합니다.

## 문제 해결

| 증상 | 가능한 원인 | 다음 행동 |
|---|---|---|
| Runtime Home이 초기화되지 않았습니다. | MCP 프로세스가 유효한 registry가 없는 `HARNESS_HOME`을 사용합니다. | 같은 절대 `HARNESS_HOME`으로 `harness init`을 실행합니다. [관리 CLI](../reference/admin-cli.md#runtime-home-selection)를 봅니다. |
| 프로젝트가 등록되지 않았습니다. | `HARNESS_PROJECT_ID=demo`에 해당하는 프로젝트 기록이 그 Runtime Home에 없습니다. | `harness project register`를 실행한 뒤 `harness project list`를 확인합니다. |
| 프로젝트가 `active`가 아닙니다. | 프로젝트 기록은 있지만 시작에 사용할 수 없습니다. | 관리 프로젝트 등록 계약과 MCP 시작 검증 담당 문서에 따라 해결합니다. |
| 접점이 등록되지 않았습니다. | `HARNESS_SURFACE_ID`가 프로젝트에 등록된 접점과 일치하지 않습니다. | 접점을 등록하거나 환경 값을 고칩니다. |
| 명시적 인스턴스를 찾을 수 없습니다. | `HARNESS_SURFACE_INSTANCE_ID`가 등록된 인스턴스와 일치하지 않습니다. | `harness surface list --project-id demo`를 확인하고 바인딩을 고칩니다. |
| 암묵적 인스턴스 선택이 모호합니다. | 명시적 인스턴스가 없고 후보가 둘 이상입니다. | `HARNESS_SURFACE_INSTANCE_ID`를 명시합니다. |
| 에이전트 접점이 읽기 전용처럼 동작합니다. | 접점을 `--profile baseline-workflow` 또는 명시적 접근 등급 없이 등록했습니다. | 기준 워크플로 프로필로 에이전트 MCP 접점을 등록합니다. |
| 로컬 접근 메타데이터가 유효하지 않습니다. | 저장된 접점 접근 메타데이터가 없거나, 형식이 잘못되었거나, 접근 등급을 하나도 부여하지 않습니다. | 관리 CLI로 다시 등록하거나 세부사항을 [에이전트 통합](../reference/agent-integration.md)과 [MCP 전송](../reference/mcp-transport.md)으로 보냅니다. |
| JSON-RPC 성공을 하네스 수락과 혼동했습니다. | 클라이언트가 `isError`만 확인했습니다. | `result.content[0].text`를 파싱하고 `base.response_kind`와 `errors`를 확인합니다. |
