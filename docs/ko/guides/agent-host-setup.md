# 에이전트 호스트 설정

Codex, Claude Code, 또는 아직 직접 지원하지 않는 호스트를 위한 하네스 MCP 통합을 설치, 확인, 점검, 안내, 제거해야 할 때 이 가이드를 사용합니다.

먼저 `harness`와 `harness-mcp`를 빌드하거나 찾으려면 [설치](../getting-started/installation.md)를 보고, 가장 짧은 첫 설정은 [빠른 시작](../getting-started/quickstart.md)을 봅니다. 이 가이드는 그 뒤의 운영 경로를 다룹니다.

정확한 명령 동작은 [관리 CLI](../reference/admin-cli.md)가 담당합니다. 정확한 Agent Integration Profile, Host Installation, 프로젝트 선택, guidance 경계는 [에이전트 통합](../reference/agent-integration.md)이 담당합니다. 정확한 프로세스 동작은 [MCP 전송](../reference/mcp-transport.md)이 담당합니다. Runtime Home과 Product Repository 쓰기 경계는 [런타임 경계](../reference/runtime-boundaries.md)가 담당합니다.

## 책임

| 부분 | 담당 | 참고 |
|---|---|---|
| 하네스 설치 | `harness`와 `harness-mcp` 실행 파일. | 소스 빌드는 `target/` 아래에 쓰고, 설치된 실행 파일은 다른 위치에 있을 수 있습니다. |
| `Harness Runtime Home` | 프로젝트 registry, Agent Integration Profile, integration project membership, Host Installation inventory, 하네스 런타임 데이터. | 모든 `Product Repository`와 분리해 둡니다. |
| `Product Repository` | 제품 파일과 명시적으로 선택한 프로젝트 범위 통합 파일. | 하네스 런타임 데이터베이스와 런타임 기록은 여기에 저장하지 않습니다. |
| Codex 또는 Claude Code | 호스트 설정, 프로젝트 신뢰, 프로젝트 MCP 승인, 재로드/재시작 동작, 모델의 도구 선택. | 하네스는 호스트가 소유한 결정을 우회할 수 없습니다. |
| `harness-mcp` 프로세스 | `--integration <integration_id>`로 시작되는 하나의 통합 바인딩 stdio 서버. | 프로젝트 선택은 공개 도구 호출마다 일어납니다. |

## 설정 상태 의미

| 상태 | 의미 |
|---|---|
| `complete` | 지속되는 통합 상태가 있고, 호스트 설정이 설치됐고, MCP 초기화와 도구 발견이 성공했습니다. |
| `action_required` | 지속되는 통합 상태와 호스트 설정은 있지만, 호스트 신뢰, 프로젝트 승인, OAuth, 재로드, 재시작, 또는 비슷한 사용자 제어 호스트 행동이 남았습니다. |
| `partial_failure` | 일부 지속 관리 동작은 성공했지만 뒤따르는 설치, 확인, 호스트 대상, 정리 단계가 실패했습니다. 보고된 문제를 고친 뒤 다시 실행합니다. |
| `failed` | 요청한 설치나 확인이 사용할 수 있는 지속 통합 상태 또는 호스트 설정을 만들지 못했습니다. |

`harness-mcp --check --integration <integration_id>`는 MCP 시작 검증일 뿐입니다. 호스트 설정이 있다는 사실은 도구 발견이 아닙니다. 도구 발견이 성공해도 이후 모델이 매번 하네스 도구를 선택한다는 보장은 아닙니다. 저장소 guidance는 발견 가능성을 높이지만, 강제 장치가 아니라 조언 맥락입니다.

## 쓰기 전 dry-run

호스트 설정이나 `Product Repository` guidance를 쓸 수 있는 명령에는 dry-run을 사용합니다.

```sh
/opt/harness/bin/harness agent install \
  --host codex \
  --scope user \
  --server-name harness-main \
  --integration-id int-codex-team \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --runtime-home /Users/alex/.harness \
  --mcp-command /opt/harness/bin/harness-mcp \
  --dry-run \
  --output json
```

Dry-run은 계획된 Runtime Home 동작, 호스트 대상 경로, guidance 대상 경로를 보고합니다. SQLite 데이터베이스, 호스트 설정, 저장소 guidance, MCP 호스트 상태를 만들거나 수정하지 않습니다.

## Codex 사용자 범위 설치

하나의 개인 Codex 설정이 여러 Codex 프로젝트에서 같은 하네스 통합을 로드해야 할 때 사용자 범위를 사용합니다.

```sh
/opt/harness/bin/harness agent install \
  --host codex \
  --scope user \
  --server-name harness-main \
  --integration-id int-codex-team \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --default-project-id acme-api \
  --runtime-home /Users/alex/.harness \
  --mcp-command /opt/harness/bin/harness-mcp
```

이 명령은 아래 항목을 쓸 수 있습니다.

- `/Users/alex/.harness` 아래 Runtime Home 기록
- `[mcp_servers.harness-main]` 같은 Codex 사용자 `config.toml` 항목

`--guidance codex`, `--guidance both`, 또는 별도 guidance 명령을 `--allow-repository-write`와 함께 선택하지 않으면 `/work/acme-api`에는 쓰지 않습니다.

예상되는 Codex 생성 모양은 다음과 같습니다.

```toml
[mcp_servers.harness-main]
command = "/opt/harness/bin/harness-mcp"
args = ["--integration", "int-codex-team"]

[mcp_servers.harness-main.env]
HARNESS_HOME = "/Users/alex/.harness"
```

Codex 프로젝트 범위도 지원되지만 `/work/acme-api/.codex/config.toml`에 쓰고, 비대화형 실행에서는 `--allow-repository-write`가 필요하며, `PATH`의 `harness-mcp`를 사용합니다. Codex가 프로젝트를 신뢰할 때까지 `action_required`를 보고할 수 있습니다.

## Claude Code 프로젝트 또는 로컬 설치

프로젝트 범위는 `Product Repository` 안의 팀 공유 `.mcp.json` 파일에 씁니다.

```sh
HARNESS_HOME=/Users/alex/.harness \
PATH="/opt/harness/bin:$PATH" \
/opt/harness/bin/harness agent install \
  --host claude-code \
  --scope project \
  --server-name harness-main \
  --integration-id int-claude-acme \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --mcp-command harness-mcp \
  --allow-repository-write
```

예상되는 `.mcp.json` 모양은 다음과 같습니다.

```json
{
  "mcpServers": {
    "harness-main": {
      "command": "harness-mcp",
      "args": ["--integration", "int-claude-acme"]
    }
  }
}
```

Claude Code는 보통 프로젝트 범위 `.mcp.json` 서버를 로드하기 전에 프로젝트 MCP 승인을 요구합니다. 이 결과는 `action_required`입니다.

로컬 범위는 MCP 서버를 현재 Claude Code 프로젝트에 비공개로 유지하고, CLI 어댑터를 통해 Claude Code의 `claude mcp add --scope local` 경로를 사용합니다.

```sh
HARNESS_HOME=/Users/alex/.harness \
/opt/harness/bin/harness agent install \
  --host claude-code \
  --scope local \
  --server-name harness-main \
  --integration-id int-claude-acme-local \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --mcp-command /opt/harness/bin/harness-mcp
```

로컬 범위와 프로젝트 범위는 단일 저장소 범위입니다. 하나의 명시적으로 허용된 통합이 여러 저장소를 처리해야 하면 사용자 범위를 사용합니다.

## 선택적 저장소 guidance

저장소 guidance는 선택 사항이며 명시적으로 승인해야 합니다.

Codex guidance는 `AGENTS.md`에 하네스 관리 블록을 씁니다.

```sh
/opt/harness/bin/harness agent guidance apply \
  --integration-id int-codex-team \
  --project-id acme-api \
  --host codex \
  --runtime-home /Users/alex/.harness \
  --dry-run \
  --allow-repository-write \
  --output json
```

Claude Code guidance는 `.claude/rules/harness.md`에 씁니다.

```sh
/opt/harness/bin/harness agent guidance apply \
  --integration-id int-codex-team \
  --project-id acme-api \
  --host claude-code \
  --runtime-home /Users/alex/.harness \
  --allow-repository-write
```

Guidance 적용 전 대상 파일은 없거나 하네스 관리 블록이 없습니다.

```text
# Existing repository instructions
```

Codex guidance 적용 뒤 `AGENTS.md`에는 관리 블록이 들어갑니다.

```md
# Existing repository instructions

<!-- BEGIN HARNESS MANAGED GUIDANCE v1 -->
## Harness MCP guidance for Codex

...
<!-- END HARNESS MANAGED GUIDANCE v1 -->
```

Claude Code guidance 적용 뒤 `.claude/rules/harness.md`에는 `## Harness MCP guidance for Claude Code`를 포함한 같은 관리 marker 모양이 들어갑니다.

관리되는 내용은 호스트에게 범위, 상태, 쓰기 준비, 실행 증거, 사용자 판단, 닫기 준비 상태 추적에 하네스를 사용하라고 안내합니다. 대상 저장소가 불분명하면 `harness.list_projects`를 호출하고, prose로 하네스 상태를 만들어 내지 말라고 안내합니다. 또한 MCP 서버 instructions와 저장소 guidance가 모델 동작을 보장할 수 없다는 점도 말합니다.

Guidance 파일은 호스트 설정 또는 조언 맥락입니다. 하네스 런타임 상태, Core 권한, 증거, 수락, 닫기 준비 상태, 잔여 위험 수락, 보안 보장이 아닙니다.

## 상태와 검증

Registry와 host inventory를 점검합니다.

```sh
/opt/harness/bin/harness agent status \
  --integration-id int-codex-team \
  --runtime-home /Users/alex/.harness
```

검증을 새로고침합니다.

```sh
/opt/harness/bin/harness agent verify \
  --integration-id int-codex-team \
  --runtime-home /Users/alex/.harness
```

직접 MCP 시작을 점검합니다.

```sh
HARNESS_HOME=/Users/alex/.harness \
/opt/harness/bin/harness-mcp --check --integration int-codex-team
```

`--check`는 `configuration: valid`, `transport: stdio`, `integration_id`, 허용 프로젝트 수, `verification_scope: startup_check_only`를 보고해야 합니다. 호스트가 도구를 로드하거나 노출했다는 증명은 아닙니다.

## 안전한 제거

호스트 설정을 다시 쓰지 않고 다중 프로젝트 통합에서 프로젝트 하나를 제거합니다.

```sh
/opt/harness/bin/harness agent project remove \
  --integration-id int-codex-team \
  --project-id billing-api \
  --runtime-home /Users/alex/.harness
```

예상 결과에는 아래 내용이 포함됩니다.

```text
verification_detail: project membership removed; host configuration was not rewritten
```

관리되는 호스트 설정과 관리되는 guidance를 완전히 제거합니다.

```sh
/opt/harness/bin/harness agent uninstall \
  --integration-id int-codex-team \
  --runtime-home /Users/alex/.harness \
  --allow-repository-write \
  --remove-managed
```

Uninstall은 하네스가 관리하는 호스트 항목, 블록, 파일, fingerprint만 제거합니다. `Product Repository`, Runtime Home, 프로젝트 상태, Core 기록, 아티팩트 저장소, 관련 없는 호스트 설정은 삭제하지 않습니다.

## Generic export fallback

하네스가 직접 설치하지 않는 호스트에만 generic export를 사용합니다.

```sh
/opt/harness/bin/harness agent install \
  --host generic \
  --scope export \
  --server-name harness-main \
  --integration-id int-generic-acme \
  --project-id acme-api \
  --repo-root /work/acme-api \
  --runtime-home /Users/alex/.harness \
  --mcp-command /opt/harness/bin/harness-mcp \
  --export-dir /tmp/harness-mcp-export
```

Export된 JSON에는 `command`, `args = ["--integration", "int-generic-acme"]`, 적용될 때 `HARNESS_HOME`을 가진 하나의 `mcpServers.harness-main` 항목이 들어갑니다. Generic export는 호스트가 서버를 로드했다고 주장하지 않습니다. 검증은 사용자가 관리합니다.
