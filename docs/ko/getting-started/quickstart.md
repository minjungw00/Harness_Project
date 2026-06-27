# Quickstart

이 튜토리얼은 실제 로컬 에이전트 호스트를 위한 가장 짧은 지원 첫 setup 경로입니다. [설치](installation.md) 뒤에 시작하며, 하나의 `Product Repository`를 사용하고 개인 Codex 사용자 범위 connection과 프로젝트 범위 Claude Code `.mcp.json` connection 중 하나를 선택하게 합니다.

전체 호스트 setup 옵션, dry-run 미리보기, 여러 프로젝트 운영, 제거, 문제 해결은 [에이전트 호스트 Setup](../guides/agent-host-setup.md)을 봅니다.

## 대상, 목표, 완료

대상: 로컬 `volicord`와 `volicord-mcp` 실행 파일을 이미 확인했고 setup을 확장하기 전에 하나의 에이전트 호스트 경로를 동작시키려는 첫 사용자나 운영자.

목표: Agent Connection 하나를 만들고, 첫 결과가 `complete`인지 `action_required`인지 인식하며, 선택한 경로에 대한 독립 검증 명령을 실행합니다.

완료 상태: 선택한 경로는 `volicord agent verify --connection-id <connection_id>`가 `status: complete`를 보고할 때 완료입니다. 명령이 `action_required`를 보고하면 이름 붙은 호스트 소유 trust, approval, reload, restart 동작을 완료한 뒤 검증을 다시 실행합니다.

## 시작 상태와 예시 값

명령을 실행하기 전에:

- POSIX 스타일 셸에서 [설치](installation.md)를 완료합니다.
- `VOLICORD_BIN`이 두 실행 파일을 포함하는 확인된 절대 디렉터리로 설정되어 있어야 합니다.
- `Volicord Runtime Home`이 아니고 그 안이나 위에 있지 않은 `Product Repository`를 선택합니다.
- 아래 모든 예시 경로와 ID를 실제 값으로 바꿉니다.

적용 전에 집중 명령 help를 확인합니다.

```sh
"$VOLICORD_BIN/volicord" agent connect --help
```

예시는 아래 값을 사용합니다.

| 값 | 종류 | 이 walkthrough에서 쓰는 방식 |
|---|---|---|
| `VOLICORD_BIN="/absolute/path/to/selected/bin"` | 튜토리얼 셸 변수 | `volicord`와 `volicord-mcp`가 모두 들어 있는 선택된 절대 디렉터리. |
| `/Users/alex/.volicord` | 예시 경로 | `Volicord Runtime Home`; `Product Repository`와 구분합니다. |
| `/work/acme-api` | 예시 경로 | Product Repository A. |
| `acme-api` | 예시 식별자 | Product Repository A의 안정적인 논리 프로젝트 ID. |
| `conn-codex-team`, `conn-claude-acme` | 예시 식별자 | 이후 verify, status, 설정, 관련 명령에서 쓰는 예측 가능한 `connection_id` 값. |
| `volicord-main` | 예시 서버 이름 | 사람이 읽기 쉬운 호스트 MCP 서버 키. |

## 호스트 경로 하나 선택

| 경로 | 선택할 때 | 결과 |
|---|---|---|
| 경로 A: Codex `user` 범위 | 개인 Codex MCP 항목 하나가 지금 이 저장소를 쓰고 나중에 명시적으로 연결된 저장소를 더 섬길 수 있어야 할 때. | 호스트 설정은 Codex 사용자 설정에 있고 절대 `volicord-mcp` 명령 경로와 `VOLICORD_HOME`을 저장합니다. |
| 경로 B: Claude Code `project` 범위 | Product Repository A가 팀 공유 Claude Code `.mcp.json` 항목을 가져야 할 때. | 프로젝트 파일은 이식 가능한 `volicord-mcp`를 사용하고 개인 `VOLICORD_HOME`을 생략하며, 실제 적용 명령에는 `--allow-repository-write`가 필요하고 Claude Code 승인이 끝날 때까지 `action_required`일 수 있습니다. |

에이전트 호스트가 workflow 도구를 노출해야 하면 `--mode workflow`를 사용합니다. 읽기 전용 connection을 의도할 때만 생략합니다.

## 경로 A: Codex 사용자 범위 Setup

개인 Codex MCP 항목 하나가 명시적으로 연결된 하나 이상의 `Product Repository` 등록을 섬겨야 할 때 사용합니다.

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

변경될 수 있는 위치:

| 위치 | 변경될 수 있는 것 |
|---|---|
| `/Users/alex/.volicord` | Runtime Home registry, Agent Connection, 연결된 project, project state 기록. |
| Codex 사용자 설정, 보통 `~/.codex/config.toml` 또는 `CODEX_HOME/config.toml` | `[mcp_servers.volicord-main]` 테이블. |
| `/work/acme-api` | 이 명령은 파일을 변경하지 않습니다. |

예상 Codex 생성 형태:

```toml
[mcp_servers.volicord-main]
command = "/absolute/path/to/selected/bin/volicord-mcp"
args = ["--connection", "conn-codex-team"]

[mcp_servers.volicord-main.env]
VOLICORD_HOME = "/Users/alex/.volicord"
```

독립 완료 확인:

```sh
"$VOLICORD_BIN/volicord" agent verify \
  --connection-id conn-codex-team \
  --runtime-home /Users/alex/.volicord
```

경로 A는 검증이 `status: complete`를 보고할 때 완료입니다. 검증이 `action_required`를 보고하면 이름 붙은 동작을 읽습니다. 흔한 Codex 사용자 범위 원인은 관리 명령 `PATH`에 `codex`가 없거나 `codex --version`을 실행할 수 없는 경우입니다.

## 경로 B: Claude Code 프로젝트 범위 Setup

Product Repository A가 팀 공유 Claude Code `.mcp.json` 항목을 가져야 할 때 사용합니다.

프로젝트 파일을 쓰기 전 선택적 dry-run:

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
  --dry-run \
  --output json
```

Setup 적용:

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

예상 `.mcp.json` 형태:

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

프로젝트 파일은 Core 권한이 아닙니다. `Product Repository` 안에 저장된 선택된 호스트 설정일 뿐입니다. 런타임 기록은 Runtime Home에 남습니다.

독립 완료 확인:

```sh
VOLICORD_HOME=/Users/alex/.volicord \
PATH="$VOLICORD_BIN:$PATH" \
"$VOLICORD_BIN/volicord" agent verify \
  --connection-id conn-claude-acme
```

프로젝트 범위 Claude Code setup은 Claude Code가 프로젝트 MCP 서버를 승인하거나 reload 또는 restart하기 전까지 `action_required`를 보고할 수 있습니다. 이름 붙은 호스트 소유 동작만 완료한 뒤 검증을 다시 실행합니다.

## MCP 시작 확인

두 경로 모두에서 `volicord-mcp --check`는 Agent Connection 하나에 대한 로컬 어댑터 시작만 검증합니다.

```sh
VOLICORD_HOME=/Users/alex/.volicord \
"$VOLICORD_BIN/volicord-mcp" --check --connection conn-codex-team
```

성공한 시작 확인은 Codex나 Claude Code가 서버를 로드, 신뢰, 승인, 노출했다는 증명이 아닙니다.

## 에이전트가 할 수 있는 일

Agent Connection은 명시적으로 연결된 Project에만 접근할 수 있습니다. 이 quickstart에서는 연결된 Project가 정확히 하나라서 MCP 호출에서 `project_id`를 생략할 수 있습니다. 같은 connection에 Project를 더 연결한 뒤에는 에이전트가 `volicord.list_projects`를 호출하는 경우를 제외하고 MCP 호출에 명시적 `project_id`가 필요합니다.

`read_only` 모드는 읽기와 프로젝트 발견 작업을 노출합니다. `workflow` 모드는 에이전트 워크플로 작업을 노출하지만 `volicord.record_user_judgment`는 노출하지 않습니다. 사용자 판단 기록은 User Channel이 담당합니다.

`Write Check`은 제품 파일 쓰기 시도 하나에 대한 Core 상태 호환성입니다. OS 권한, OS 샌드박싱, 파일시스템 ACL, 네트워크 정책, 비밀 격리가 아닙니다.

## 흔한 다음 단계

| 필요 | 경로 |
|---|---|
| 사용자 범위 connection에 Project 추가. | [여러 저장소 Agent Setup](../guides/multi-repository-agent-setup.md) |
| connection 상태 확인. | `volicord agent status --connection-id <connection_id>` |
| MCP에서 사용 가능한 Project 확인. | `volicord.list_projects` |
| 대기 중인 사용자 판단 기록. | [User Channel 명령](../reference/admin-cli.md#user-channel-commands) |
| Setup이 `volicord-mcp`를 해석하지 못합니다. | [실행 파일 누락 문제 해결](../guides/agent-host-troubleshooting.md#missing-volicord-mcp) |
| 결과가 `action_required`입니다. | [action_required 문제 해결](../guides/agent-host-troubleshooting.md#status-action_required) |
| 결과가 `failed`입니다. | [failed 문제 해결](../guides/agent-host-troubleshooting.md#status-failed) |
