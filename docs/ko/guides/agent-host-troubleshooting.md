# 에이전트 호스트 문제 해결

Codex, Claude Code, 일반 MCP 호스트 connection이 `volicord agent connect`, `volicord agent verify`, `volicord agent status`, project membership 변경, uninstall 뒤 기대한 상태에 도달하지 않을 때 이 가이드를 사용합니다.

일반 setup 경로는 [에이전트 호스트 Setup](agent-host-setup.md)을 사용합니다. 하나의 사용자 범위 connection이 여러 저장소를 섬기는 경우 [여러 저장소 Agent Setup](multi-repository-agent-setup.md)을 사용합니다.

이 가이드는 관찰된 상태를 식별하고, 가능한 경우 추가 변경 없이 원인을 확인하고, 제한된 복구 동작을 수행한 뒤 결과를 검증하도록 돕습니다. 정확한 동작은 [관리 CLI](../reference/admin-cli.md), [MCP 전송](../reference/mcp-transport.md), [런타임 경계](../reference/runtime-boundaries.md), [Agent Connection Reference](../reference/agent-connection.md), [저장소](../reference/storage.md)가 안내하는 저장소 담당 문서에 남습니다.

## 변경 전에

setup 때 사용한 같은 값을 유지합니다.

- `VOLICORD_BIN`은 `volicord`와 `volicord-mcp`가 들어 있는 선택된 디렉터리입니다.
- `VOLICORD_HOME` 또는 `--runtime-home`은 선택된 `Volicord Runtime Home`입니다.
- `<connection_id>`, `<project_id>`, `<repo_root>`, `<server_name>`은 setup 출력의 실제 값입니다.

먼저 읽기 전용 또는 비변경 확인을 실행합니다.

```sh
"$VOLICORD_BIN/volicord" agent status \
  --connection-id <connection_id> \
  --runtime-home <runtime_home>

VOLICORD_HOME=<runtime_home> \
"$VOLICORD_BIN/volicord-mcp" --check --connection <connection_id>
```

`volicord agent status`는 저장된 Agent Connection과 연결된 Project 상태를 보고합니다. Codex나 Claude Code가 MCP 서버를 로드했다는 증명은 아닙니다. `volicord-mcp --check`는 MCP 프로세스의 로컬 시작만 검증합니다.

<a id="missing-volicord-mcp"></a>
## `volicord-mcp`가 없거나 실행 불가이거나 해석되지 않음

관찰 증상: setup, 검증, 호스트 시작이 `volicord-mcp`가 없거나 unavailable, not executable, 또는 `PATH`에서 찾을 수 없다고 보고합니다.

가능성이 큰 원인: 선택한 실행 파일 디렉터리에 `volicord`와 `volicord-mcp`가 모두 없거나, 파일이 선택한 사용자에게 실행 가능하지 않거나, 프로젝트 범위 호스트 설정은 이식 가능한 `volicord-mcp`를 저장했지만 이후 호스트 프로세스가 이를 찾을 수 있는 `PATH`를 받지 못했습니다.

제한된 복구:

```sh
test -x "$VOLICORD_BIN/volicord-mcp"
"$VOLICORD_BIN/volicord-mcp" --version
```

사용자, 로컬, generic export 범위에서는 같은 `--connection-id`에 대해 유효한 절대 `--mcp-command`로 `volicord agent connect`를 다시 실행합니다. 프로젝트 범위에서는 생성된 호스트 항목을 이식 가능한 형태로 유지하고 호스트 시작 `PATH`를 고칩니다.

검증:

```sh
VOLICORD_HOME=<runtime_home> \
"$VOLICORD_BIN/volicord-mcp" --check --connection <connection_id>
```

실행 파일을 해석할 수 없었다는 이유만으로 Runtime Home, project state, Product Repository 파일, 관련 없는 호스트 설정을 삭제하지 않습니다.

<a id="wrong-absolute-mcp-command"></a>
## 절대 `--mcp-command`가 잘못됨

관찰 증상: CLI가 `--mcp-command`를 거부하거나, 검증이 설정된 명령이 missing, changed, unavailable, 또는 launch 불가라고 보고합니다.

가능성이 큰 원인: 경로가 절대 경로가 아니거나, 오래된 빌드 출력을 가리키거나, `volicord-mcp`가 아니라 `volicord`를 가리키거나, rebuild 또는 이동 뒤 더 이상 존재하지 않습니다.

제한된 복구: `test -x /absolute/path/to/volicord-mcp`와 `/absolute/path/to/volicord-mcp --help`를 실행한 뒤, 같은 `--connection-id`, host, scope, server name에 대해 고친 절대 명령으로 `volicord agent connect`를 다시 실행합니다.

<a id="portable-project-command-not-on-path"></a>
## 이식 가능한 프로젝트 범위 명령이 호스트 `PATH`에 없음

관찰 증상: 프로젝트 범위 Codex 또는 Claude Code 설정에 `command = "volicord-mcp"` 또는 `"command": "volicord-mcp"`가 있지만 이후 호스트 세션이 Volicord를 시작할 수 없습니다.

가능성이 큰 원인: 프로젝트 범위 설정은 개인 빌드 경로와 개인 `VOLICORD_HOME`을 의도적으로 생략합니다. 이후 호스트 프로세스가 `volicord-mcp`를 해석할 수 없는 환경에서 시작되었습니다.

제한된 복구: 호스트 시작 환경, 셸 시작 설정, 서비스 설정, 또는 이에 해당하는 호스트 소유 경로를 바꿔 `volicord-mcp`를 해석할 수 있게 합니다. 프로젝트 범위 호스트 파일은 이식 가능한 형태로 유지합니다.

<a id="status-action_required"></a>
## `status: action_required`

의미: 지속 Agent Connection 상태와 호스트 설정은 있지만 host trust, project approval, OAuth, reload, restart, 또는 그에 준하는 사용자 제어 호스트 동작이 남았습니다.

제한된 복구:

1. 보고된 동작과 호스트 세부사항을 읽습니다.
2. 그 호스트 소유 동작만 완료합니다.
3. 검증을 다시 실행합니다.

```sh
"$VOLICORD_BIN/volicord" agent verify \
  --connection-id <connection_id> \
  --runtime-home <runtime_home>
```

`action_required`를 Core 실패나 제품 수락 결과로 취급하지 않습니다.

<a id="status-failed"></a>
## `status: failed`

의미: 요청한 connection 또는 검증이 사용할 수 있는 지속 Agent Connection 상태나 호스트 설정을 만들지 못했습니다. 실패 전에 일부 지속 호스트 효과가 이미 발생했을 수 있습니다.

제한된 복구: stderr 또는 JSON `warnings`, `verification`, `effects`, `residual_effects`, host detail 필드를 확인합니다. 이름 붙은 실행 파일, Runtime Home, Product Repository 경로, host target, host gate 문제를 고친 뒤 같은 명령을 다시 실행합니다. 첫 대응으로 관련 없는 상태를 삭제하지 않습니다.

<a id="ambiguous-project-selection"></a>
## 여러 연결된 Project가 있지만 사용할 selector가 없음

관찰 증상: 둘 이상의 연결된 Project가 있고 요청이 `project_id`를 제공하지 않아 MCP workflow 호출이 거부됩니다.

가능성이 큰 원인: Agent Connection은 올바르게 여러 Project에 연결되어 있지만 에이전트가 명시적 `project_id` 없이 Project 라우팅 도구 호출을 보냈습니다.

제한된 복구: `volicord.list_projects`를 호출하고 의도한 Project를 선택한 뒤 Project 라우팅 도구 호출을 `project_id`와 함께 다시 시도합니다.

<a id="host-config-remains-zero-projects"></a>
## 현재 연결된 Project가 없지만 호스트 설정이 남아 있음

관찰 증상: 호스트 설정은 있지만 Project 라우팅 MCP 도구가 진행할 수 없거나 `volicord.list_projects`가 빈 Project 목록을 반환합니다.

의미: Agent Connection은 연결된 Project 없이 존재할 수 있지만 그 상태는 project-tool eligibility가 아닙니다.

제한된 복구:

```sh
"$VOLICORD_BIN/volicord" agent project add \
  --connection-id <connection_id> \
  --project-id <project_id> \
  --repo-root <repo_root> \
  --runtime-home <runtime_home>
```

그런 다음 `volicord-mcp --check --connection <connection_id>`와 호스트 검증 명령을 다시 실행합니다.

<a id="partial-removal"></a>
## 제거가 일부만 완료됨

관찰 증상: uninstall이 일부 정리는 성공했지만 다른 선택 호스트 대상이나 저장된 connection 기록을 제거할 수 없다고 보고합니다.

제한된 복구: `volicord agent status --connection-id <connection_id>`를 실행하고, 이름 붙은 host target, path permission, ownership mismatch를 고친 뒤 uninstall을 다시 실행합니다. project state, artifact storage, 관련 없는 호스트 항목을 직접 제거하지 않습니다.

## 보안 경계

Agent Connection은 로컬 MCP 호스트 connection 맥락을 식별합니다. OS 계정, 샌드박스, 파일시스템 ACL, 네트워크 정책, 비밀 격리가 아닙니다. `Write Check`은 제품 파일 쓰기 시도 하나에 대한 Core 상태 호환성이지 OS 권한이 아닙니다.
