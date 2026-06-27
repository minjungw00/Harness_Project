# 에이전트 호스트 Setup

Codex, Claude Code, 또는 일반 MCP 설정 export를 위한 Volicord MCP 호스트 connection을 연결, 검증, 조회, 제거해야 할 때 이 가이드를 사용합니다.

먼저 [설치](../getting-started/installation.md)에서 `volicord`와 `volicord-mcp`를 빌드하거나 찾고, 가장 짧은 첫 setup은 [Quickstart](../getting-started/quickstart.md)를 봅니다. 정확한 명령 동작은 [관리 CLI](../reference/admin-cli.md)가 담당합니다. 정확한 Agent Connection 동작은 [Agent Connection Reference](../reference/agent-integration.md)가 담당합니다. 정확한 프로세스 동작은 [MCP 전송](../reference/mcp-transport.md)이 담당합니다.

Volicord는 OS 보안 제품이 아닙니다. Agent Connection, Write Check, 호스트 설정은 OS 샌드박싱, 파일시스템 ACL, 네트워크 정책, 비밀 격리를 제공하지 않습니다.

## 실행 파일 관례

명령 예시는 하나의 절대 디렉터리에 두 실행 파일이 모두 있다고 가정합니다.

```sh
export VOLICORD_BIN="/absolute/path/to/selected/bin"
```

이 저장소 루트에서 디버그 빌드를 사용할 때:

```sh
export VOLICORD_BIN="$(pwd)/target/debug"
```

`VOLICORD_BIN`은 이 예시를 위한 셸 편의 변수일 뿐입니다. Volicord는 이를 런타임이나 호스트 설정으로 읽지 않습니다.

## 책임

| 부분 | 담당 | 참고 |
|---|---|---|
| Volicord 설치 | `volicord`와 `volicord-mcp` 실행 파일. | 소스 빌드는 `target/` 아래에 쓰고 설치된 실행 파일은 다른 곳에 있을 수 있습니다. |
| `Volicord Runtime Home` | Project registry, Agent Connection, 연결된 Project, Volicord 런타임 데이터. | 모든 `Product Repository`와 분리합니다. |
| `Product Repository` | 제품 파일과 명시적으로 선택된 프로젝트 범위 호스트 설정. | Core 권한이 아니며 Runtime Home 데이터베이스를 포함하면 안 됩니다. |
| Codex 또는 Claude Code | 호스트 설정 로드, 프로젝트 trust, 프로젝트 MCP 승인, reload/restart 동작, MCP 서버 실행 환경, 모델 도구 선택. | Volicord는 호스트 소유 결정을 우회할 수 없습니다. |
| `volicord-mcp` 프로세스 | `--connection <connection_id>`로 시작되는 connection-bound stdio 서버 하나. | 프로젝트 라우팅은 공개 도구 호출마다 수행됩니다. |

## Setup 순서

`volicord agent connect`는 운영자에게 보이는 순서로 아래를 수행합니다.

1. host, scope, project, connection, mode, Runtime Home, 실행 파일, output, approval 옵션을 해석합니다.
2. 선택된 Project 하나를 해석하거나 등록합니다.
3. `read_only` 또는 `workflow` 모드의 Agent Connection 하나를 만들거나 재사용합니다.
4. 선택된 Project를 `connection_projects`에 추가합니다.
5. 선택된 Runtime Home으로 `volicord-mcp --check --connection <connection_id>`를 실행합니다.
6. `volicord-mcp --connection <connection_id>`를 시작하는 호스트 설정을 설치하거나 내보냅니다.
7. 선택한 호스트가 충분한 상태를 노출하면 호스트 준비 상태를 검증합니다.

`--dry-run`은 계획을 미리 보여 주고 아무것도 쓰지 않습니다.

## 인자 선택

| 결정 | 선택 규칙 |
|---|---|
| Host와 scope | 지원 매트릭스에서 `--host`와 `--scope`를 함께 선택합니다. Codex는 `user` 또는 `project`, Claude Code는 `local`, `project`, `user`, generic setup은 `export`를 사용합니다. |
| Project 선택 | 새 Project 등록에는 `--project-id`와 `--repo-root`를 모두 제공합니다. 이미 등록된 Project는 `--project-id`만으로 등록된 경로를 재사용할 수 있습니다. |
| Connection identity | `--connection-id`는 선택 사항입니다. 이후 status, verify, uninstall, 스크립트, 예시에 안정적인 이름이 필요하면 제공합니다. |
| Connection mode | 읽기 전용 connection을 의도할 때만 `--mode`를 생략합니다. 에이전트 호스트가 workflow 도구를 써야 하면 `--mode workflow`를 사용합니다. |
| Runtime Home | 기본 Runtime Home이 아닌 경로를 선택하거나 예시를 반복 가능하게 하려면 `--runtime-home` 또는 `VOLICORD_HOME`을 사용합니다. 프로젝트 범위 호스트 파일은 개인 Runtime Home 경로를 저장하지 않습니다. |
| MCP 실행 파일 | 사용자, 로컬, export 범위에서는 CLI가 `volicord-mcp`를 찾게 하거나 명시적 절대 `--mcp-command`를 제공합니다. 프로젝트 범위에서는 `--mcp-command`를 생략합니다. 생성된 공유 설정은 이식 가능한 `volicord-mcp`를 사용하고 호스트 시작 `PATH`에 의존합니다. |
| 저장소 쓰기 승인 | 실제 프로젝트 범위 connection에는 `--allow-repository-write`를 포함합니다. dry-run은 아무것도 쓰지 않으므로 대칭을 위해 추가하지 않습니다. |
| Export 대상 | `generic` `export`에서는 정확한 파일 하나가 필요하면 `--export-path`, 선택한 디렉터리의 생성 파일 이름이면 `--export-dir`를 사용합니다. |

## 결과 상태

| 상태 | 의미 |
|---|---|
| `complete` | 지속 Agent Connection 상태가 있고, 관리 호스트 설정이 예상 fingerprint와 일치하며, 필요한 호스트 gate가 만족되고, connection preflight, MCP initialize, tool discovery가 필요한 도구를 노출합니다. |
| `action_required` | 지속 Agent Connection 상태와 호스트 설정은 있지만 host trust, project approval, OAuth, reload, restart, 또는 그에 준하는 사용자 제어 호스트 동작이 남았습니다. |
| `failed` | 요청한 connection 또는 검증이 사용할 수 있는 지속 connection 상태나 호스트 설정을 만들지 못했습니다. |

성공한 `volicord-mcp --check --connection <connection_id>`는 시작 검증일 뿐입니다. Codex, Claude Code, 일반 호스트가 서버를 로드, 신뢰, 승인, 노출했다는 증명이 아닙니다.

## 쓰기 전 Dry-Run

호스트 설정이나 프로젝트 범위 설정을 쓸 수 있는 명령에는 dry-run을 사용합니다.

```sh
"$VOLICORD_BIN/volicord" agent connect \
  --host codex \
  --scope user \
  --server-name volicord-main \
  --connection-id conn-codex-team \
  --mode workflow \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --runtime-home /Users/alex/.volicord \
  --mcp-command "$VOLICORD_BIN/volicord-mcp" \
  --dry-run \
  --output json
```

Dry-run은 계획된 Runtime Home 동작, 호스트 대상 경로, connection 세부사항을 보고합니다. 아무것도 만들거나 수정하지 않습니다.

## Codex 사용자 범위 Connection

하나의 개인 Codex 설정이 명시적으로 연결된 하나 이상의 Project에서 같은 Volicord connection을 로드해야 할 때 user scope를 사용합니다.

```sh
"$VOLICORD_BIN/volicord" agent connect \
  --host codex \
  --scope user \
  --server-name volicord-main \
  --connection-id conn-codex-team \
  --mode workflow \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --runtime-home /Users/alex/.volicord \
  --mcp-command "$VOLICORD_BIN/volicord-mcp"
```

예상 Codex 생성 형태:

```toml
[mcp_servers.volicord-main]
command = "/absolute/path/to/selected/bin/volicord-mcp"
args = ["--connection", "conn-codex-team"]

[mcp_servers.volicord-main.env]
VOLICORD_HOME = "/Users/alex/.volicord"
```

## Codex 또는 Claude Code 프로젝트 범위

프로젝트 범위는 선택된 `Product Repository` 안에 호스트 설정을 씁니다. 해당 저장소가 공유 호스트 항목을 가져야 할 때만 사용합니다.

```sh
VOLICORD_HOME=/Users/alex/.volicord \
PATH="$VOLICORD_BIN:$PATH" \
"$VOLICORD_BIN/volicord" agent connect \
  --host claude-code \
  --scope project \
  --server-name volicord-main \
  --connection-id conn-claude-acme \
  --mode workflow \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --allow-repository-write
```

예상 Claude Code `.mcp.json` 형태:

```json
{
  "mcpServers": {
    "volicord-main": {
      "command": "volicord-mcp",
      "args": ["--connection", "conn-claude-acme"]
    }
  }
}
```

프로젝트 범위 파일은 선택된 호스트 설정을 위한 제품 파일 경계 예외입니다. Core 권한이 아니며 Runtime Home 기록을 저장하지 않습니다.

## Generic Export

사용자가 직접 관리하는 호스트를 위해 Volicord가 MCP 설정 객체를 써야 할 때 `generic` `export`를 사용합니다.

```sh
"$VOLICORD_BIN/volicord" agent connect \
  --host generic \
  --scope export \
  --server-name volicord-main \
  --connection-id conn-generic-acme \
  --mode workflow \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --runtime-home /Users/alex/.volicord \
  --mcp-command "$VOLICORD_BIN/volicord-mcp" \
  --export-path /tmp/volicord-main.mcp.json
```

Generic export는 사용자 관리 호스트가 내보낸 설정을 로드하고 검증할 때까지 `action_required`로 남습니다.

## 조회, 검증, 활성화, 비활성화

```sh
"$VOLICORD_BIN/volicord" agent list \
  --runtime-home /Users/alex/.volicord

"$VOLICORD_BIN/volicord" agent status \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord

"$VOLICORD_BIN/volicord" agent verify \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord

"$VOLICORD_BIN/volicord" agent disable \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord

"$VOLICORD_BIN/volicord" agent enable \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord
```

disable과 enable은 저장된 Agent Connection eligibility를 바꿉니다. 호스트 설정을 다시 쓰지 않고 사용자 소유 판단을 만들지 않습니다.

## 연결된 Project

사용자 범위 connection은 둘 이상의 Project를 연결할 수 있습니다.

```sh
"$VOLICORD_BIN/volicord" agent project add \
  --connection-id conn-codex-team \
  --project-id billing-api \
  --repo-root /work/billing-api \
  --runtime-home /Users/alex/.volicord
```

정확히 하나의 Project만 연결되어 있으면 MCP 호출에서 `project_id`를 생략할 수 있습니다. 여러 Project가 연결되어 있으면 `volicord.list_projects`를 사용하는 경우가 아닌 MCP 호출은 명시적 `project_id`가 필요합니다.

연결된 Project 하나를 제거합니다.

```sh
"$VOLICORD_BIN/volicord" agent project remove \
  --connection-id conn-codex-team \
  --project-id billing-api \
  --runtime-home /Users/alex/.volicord
```

project와 local 범위는 단일 Project 범위입니다. 여러 저장소 운영에는 user scope를 사용합니다.

## 제거

```sh
"$VOLICORD_BIN/volicord" agent uninstall \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord \
  --dry-run

"$VOLICORD_BIN/volicord" agent uninstall \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord
```

저장소 파일을 제거하는 프로젝트 범위 uninstall의 실제 명령에는 `--allow-repository-write`가 필요합니다. Uninstall은 소유권과 안전 점검이 허용할 때 선택된 관리 호스트 설정을 제거하고, 관련 Connection Project 기록을 제거하며, 더 쓰이지 않는 Agent Connection 기록을 제거합니다. Product Repository 내용, project registration과 project state, Core task/evidence/decision/run/artifact 기록, artifact storage, 관련 없는 호스트 항목은 담당 문서에 따라 보존합니다.

## 도구 노출

`read_only` 모드는 읽기와 프로젝트 발견 작업을 노출합니다. `volicord.status`, 닫기 준비 상태 확인용 `volicord.close_task`, `volicord.list_projects`입니다.

`workflow` 모드는 읽기 작업과 에이전트 워크플로 작업을 노출합니다. `volicord.intake`, `volicord.update_scope`, `volicord.status`, `volicord.prepare_write`, `volicord.stage_artifact`, `volicord.record_run`, `volicord.request_user_judgment`, `volicord.close_task`, `volicord.list_projects`입니다.

`workflow` 모드는 `volicord.record_user_judgment`를 노출하지 않습니다. 사용자 판단 기록은 User Channel이 담당합니다.

## 문제 해결 경로

- `action_required`: [`status: action_required`](agent-host-troubleshooting.md#status-action_required)
- `failed`: [`status: failed`](agent-host-troubleshooting.md#status-failed)
- 누락된 `volicord-mcp`: [`volicord-mcp` 누락](agent-host-troubleshooting.md#missing-volicord-mcp)
- 여러 연결된 Project와 selector 없음: [모호한 프로젝트 선택](agent-host-troubleshooting.md#ambiguous-project-selection)
