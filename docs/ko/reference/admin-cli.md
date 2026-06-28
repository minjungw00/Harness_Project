# 관리 CLI 참조

이 문서는 로컬 `volicord` 관리/부트스트랩 CLI 계약을 담당합니다. 이 CLI는 `Volicord Runtime Home`을 초기화하고, 로컬 프로젝트를 등록하며, 선택된 사용자 대상 Core 동작을 위한 로컬 `User Channel` 경로를 제공하고, Agent Connection을 관리하고, 지원되는 코딩 에이전트 호스트의 호스트 설정을 설치하고, 호스트 연결 상태를 검증합니다. 이 명령들은 공개 Volicord API 메서드가 아닙니다.

이 문서는 공개 API 메서드 동작, API 스키마, 저장소 기록 배치, 보안 보장, Core 권한 의미, MCP stdio 전송 동작을 정의하지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- `volicord` 명령 이름, 명령줄 인자, 기본값, stdout/stderr 처리, 프로세스 종료 코드
- `volicord` 관리 명령의 Runtime Home 경로 선택
- 관리용 프로젝트 등록 기본값
- 로컬 `User Channel` 명령 이름과 명령 출력
- Agent Connection 명령 동작
- Connection Project 멤버십 명령 동작
- Codex, Claude Code, generic export를 위한 호스트 연결, 상태, 검증, 제거 명령 동작
- 설정 결과 상태, dry-run 동작, 기계 판독 출력, 비대화식 승인 동작
- 관리 명령, 로컬 `User Channel` 명령, 공개 Volicord API 메서드 사이의 경계

이 문서는 담당하지 않습니다.

- 공개 Volicord API 메서드: [API 메서드](api/methods.md)
- Agent Connection, Connection Project, 연결 모드, 행위자 출처 의미: [Agent Connection](agent-connection.md)
- 런타임 데이터 경계 의미와 `Product Repository` 파일 경계 예외: [런타임 경계](runtime-boundaries.md)
- MCP 프로세스 시작, stdio 프레이밍, 와이어 동작, 응답 래핑, 사전 점검 내부 동작, 종료: [MCP 전송](mcp-transport.md)
- 저장소 기록 배치, SQLite DDL, 일반 저장소 마이그레이션 정의, Core 권한 의미, 보안 보장 의미

## 명령 모델

`volicord`는 로컬 관리/부트스트랩 실행 파일입니다. 장기 실행 서버가 아닙니다. `volicord user` 명령군은 선택된 Core 메서드 위에 있는 로컬 `User Channel` CLI 어댑터입니다. 이 명령 이름은 공개 Volicord API 메서드가 아니라 관리 CLI 명령으로 남습니다.

지원되는 기준 명령은 아래와 같습니다.

```text
volicord --help
volicord --version
volicord init [--runtime-home-id ID]
volicord project register --project-id ID --repo-root PATH [--status active]
volicord project list
volicord user status --project-id ID [--task-id ID] [--runtime-home PATH] [--output text|json]
volicord user judgment list --project-id ID [--task-id ID] [--runtime-home PATH] [--output text|json]
volicord user judgment show --project-id ID --judgment-id ID [--runtime-home PATH] [--output text|json]
volicord user judgment record --project-id ID --judgment-id ID --option-id ID [--request-id ID] [--idempotency-key KEY] [--expected-state-version VERSION] [--note TEXT] [--runtime-home PATH] [--output text|json]
volicord agent connect --host codex|claude-code|claude_code|generic --scope user|project|local|export [--project-id ID] [--repo-root PATH] [--connection-id ID] [--mode read_only|workflow] [--server-name NAME] [--mcp-command PATH] [--runtime-home PATH] [--export-path PATH|--export-dir PATH] [--output text|json] [--dry-run] [--allow-repository-write] [--replace-managed]
volicord agent list [--runtime-home PATH] [--output text|json]
volicord agent status --connection-id ID [--runtime-home PATH] [--output text|json]
volicord agent enable --connection-id ID [--runtime-home PATH] [--output text|json]
volicord agent disable --connection-id ID [--runtime-home PATH] [--output text|json]
volicord agent project add --connection-id ID --project-id ID [--repo-root PATH] [--runtime-home PATH] [--output text|json] [--dry-run]
volicord agent project remove --connection-id ID --project-id ID [--runtime-home PATH] [--output text|json] [--dry-run]
volicord agent verify --connection-id ID [--runtime-home PATH] [--output text|json]
volicord agent uninstall --connection-id ID [--runtime-home PATH] [--output text|json] [--dry-run] [--allow-repository-write]
```

종료 코드와 스트림 동작:

- 성공한 명령은 성공 출력을 stdout에 쓰고 종료 코드 `0`으로 끝납니다.
- `action_required`는 성공한 관리 결과이며 종료 코드 `0`으로 끝납니다.
- `failed`, 런타임 오류, 저장소 오류, 사전 점검 실패, 검증 실패, 충돌은 종료 코드 `1`로 끝납니다.
- 사용법 오류는 진단을 stderr에 쓰고 종료 코드 `2`로 끝납니다.
- `volicord --version`은 stdout에 `volicord <version>`을 쓰며 Runtime Home 해석을 요구하지 않습니다.
- `--output json`은 stdout에 JSON 문서 정확히 하나를 쓰며 사람용 설명을 섞지 않습니다.
- 오류는 기존 CLI 종료 코드 모델에 따라 stderr 진단으로 남습니다.

지원하지 않는 것:

- `volicord setup`과 `volicord setup local-mcp`는 지원되는 명령이 아닙니다.
- CLI에는 `serve`, `server`, `connect` 명령이 없습니다.
- 공개 `volicord agent` 계약에는 포괄적 자동 확인 플래그가 없습니다. 이 계약이 요구하는 명시적 승인 및 교체 플래그를 사용해야 합니다.
- 관리 명령은 공개 Volicord API 메서드가 아니며 공개 메서드 목록에 추가되면 안 됩니다.

<a id="runtime-home-selection"></a>
## Runtime Home 선택

`volicord` 관리 CLI는 아래 Runtime Home 경로 해석 규칙을 사용합니다. `volicord-mcp` 프로세스 환경과 현재 MCP Runtime Home 경로 해석은 [MCP 전송](mcp-transport.md#process-environment)이 담당합니다.

해석 순서:

1. 명령이 정의한 경우 명령별 `--runtime-home`
2. `VOLICORD_HOME`
3. `HOME`, `USERPROFILE`, `HOMEDRIVE`와 `HOMEPATH` 결합 순서의 첫 번째 비어 있지 않은 홈 소스에 `.volicord`를 붙인 경로

규칙:

- `VOLICORD_HOME`이 존재하지만 비어 있으면 오류입니다.
- 설정, 설치, 검증, 마이그레이션 계획을 수행하는 명령에서 명령별 `--runtime-home` 값은 절대 경로여야 합니다.
- 상대 경로 `VOLICORD_HOME`은 그 경로가 존재하지 않아도 프로세스의 현재 작업 디렉터리를 기준으로 해석합니다.
- `volicord init`은 선택된 Runtime Home 레지스트리를 만들거나 검증할 수 있습니다.
- 다른 관리 명령은 선택된 Runtime Home에 요청 작업에 필요한 기록이 있어야 합니다.

<a id="user-channel-commands"></a>
<a id="user-interaction-commands"></a>
## User Channel 명령

`volicord user` 명령은 사람이 로컬 CLI에서 `User Channel`을 통해 작업 상태를 확인하고 대기 중인 사용자 판단에 답할 수 있는 경로를 제공합니다. 이 명령은 설정 명령, 사용자 대상 어댑터 등록, 어댑터 식별자를 요구하지 않습니다. Agent Connection을 만들거나, MCP 호스트 설정을 설치하거나, Agent Connection이 사용자처럼 동작할 수 있게 하지 않습니다.

`volicord user status`는 `actor_source=local_user`, `operation_category=read`, `verification_basis=cli_direct_user_channel`로 `volicord.status`를 통해 사용자 중심 작업 상태를 보여 줍니다.

`volicord user judgment list`는 활성 작업 또는 선택된 작업의 대기 판단을 읽고 작업, 판단 종류, 상태, 질문, 선택지를 나열합니다.

`volicord user judgment show`는 대기 중이거나 과거의 판단 하나를 읽고 저장된 판단 요청, 맥락 요약, Core가 생성한 선택지를 표시합니다.

`volicord user judgment record`는 대기 중인 판단과 그 판단에 저장된 Core 생성 선택지 중 하나를 가리키는 `--option-id`를 요구합니다. 이 명령은 `actor_source=local_user`, `operation_category=user_only`, `verification_basis=cli_direct_user_channel`, `assurance_level=local_user_channel`로 `volicord.record_user_judgment`를 통해 선택을 기록합니다. 기록되는 답은 선택된 선택지의 `machine_action`과 `resolution_outcome`으로 결정됩니다. `--note`는 메모로만 저장됩니다. `--request-id`, `--idempotency-key`, `--expected-state-version`이 생략되면 명령은 로컬 요청 ID, 로컬 idempotency key, 현재 프로젝트 상태 버전을 제공합니다. Agent Connection은 `volicord user judgment record`에 사용할 수 없습니다.

판단 하나를 기록하는 것은 그 판단만 기록합니다. 최종 수락과 잔여 위험 수락은 별개의 판단 종류와 동작으로 남아야 하며, 이 명령이 둘을 하나로 합치면 안 됩니다.

안정적인 판단 작업 흐름:

1. `volicord user status --project-id ID [--task-id ID]`로 현재 작업 상태, 대기
   판단 수, 닫기 상태, 다음 행동을 확인합니다.
2. `volicord user judgment list --project-id ID [--task-id ID]`로 활성 작업 또는
   선택된 작업의 대기 판단을 봅니다.
3. `volicord user judgment show --project-id ID --judgment-id ID`로 저장된 요청,
   맥락 요약, Core 생성 선택지를 확인합니다.
4. `volicord user judgment record --project-id ID --judgment-id ID --option-id ID`로 그
   판단에 대해 선택한 Core 생성 선택지 하나를 기록합니다.

`status`, `list`, `show` 출력은 사용자의 다음 행동을 위해 선택된 담당 상태를
보여 줍니다. 이 출력은 증거, 최종 수락, 잔여 위험 수락, 닫기 준비 상태를 만들지
않습니다. `volicord user judgment record`만 대기 중인 해당 판단을 변경하며, 그것도
선택된 Core 생성 선택지를 통해서만 변경합니다.

## 호스트와 범위 지원

지원되는 호스트와 범위 값:

| `--host` | 지원되는 `--scope` 값 | 기준 대상 |
|---|---|---|
| `codex` | `user`, `project` | 사용자 설정은 Codex 사용자 `config.toml`입니다. 프로젝트 설정은 연결된 `Product Repository` 안의 `.codex/config.toml`입니다. |
| `claude_code` | `local`, `project`, `user` | local과 user 설정은 Claude Code 사용자 소유 설정 대상입니다. 프로젝트 설정은 연결된 `Product Repository` 안의 `.mcp.json`입니다. CLI는 `claude-code`를 별칭으로 받을 수 있지만 저장 기록은 `claude_code`를 사용합니다. |
| `generic` | `export` | 직접 설치를 주장하지 않고 명시적 MCP 설정 객체를 내보냅니다. |

범위 규칙:

- `project`와 `local` 범위는 연결된 `Product Repository` 하나만 허용합니다.
- `user` 범위는 명시적으로 추가된 여러 프로젝트를 허용할 수 있지만, 각 `volicord agent connect` 호출은 연결 프로젝트 선택 규칙에 따라 정확히 하나의 프로젝트를 선택합니다.
- `generic export`는 명시적 설정 내보내기만 쓰거나 출력하며, 호스트 로드를 주장하지 않습니다.
- 지원하지 않는 호스트/범위 조합은 사용법 오류입니다.

호스트 설정 형태:

- Codex 연결은 선택한 호스트 범위가 선택된 Runtime Home 경로 저장을 허용할 때 선택적 `env.VOLICORD_HOME`과 함께, `command`, `args = ["--connection", "<connection_id>"]`를 가진 `[mcp_servers.<server_name>]`와 동등한 MCP 서버 테이블을 씁니다.
- Claude Code 연결은 선택한 호스트 범위가 선택된 Runtime Home 경로 저장을 허용할 때 선택적 `env.VOLICORD_HOME`과 함께, `command`, `args`를 가진 `mcpServers.<server_name>` MCP 서버 항목을 씁니다.
- Generic export는 같은 command, args, environment 값을 호스트 중립 JSON 객체로 출력합니다.
- Generic export가 명시적 `--export-path` 대신 내보내기 디렉터리에 쓸 때 생성 파일 이름은 `volicord-<connection>.mcp.json`입니다.
- user와 local 범위는 정식으로 확인된 `volicord-mcp` 실행 파일 경로나 명시적이고 유효한 절대 경로를 사용할 수 있습니다.
- 프로젝트 범위 공유 설정은 호스트 환경의 `PATH`에서 찾을 수 있는 이식 가능한 `volicord-mcp` 명령을 사용해야 합니다. 개인 빌드 경로, 홈 디렉터리 경로, 개인 `VOLICORD_HOME`을 넣으면 안 됩니다.
- Generic export는 명시적으로 선택한 절대 명령 경로를 내보낼 수 있지만, 내보낸 설정은 사용자가 관리하는 호스트가 로드하고 검증하기 전까지 계속 `action_required`입니다.
- 새 기준 호스트 설정은 오래된 라우팅 환경 변수를 요구하면 안 됩니다.

호스트 신뢰 경계:

- 설정 쓰기와 호스트가 MCP 서버를 로드하고 노출하는 것은 구분됩니다.
- Codex 프로젝트 범위 설정은 로드되기 전에 Codex 프로젝트 신뢰가 필요할 수 있습니다.
- Claude Code 프로젝트 범위 MCP 설정은 로드되기 전에 프로젝트 MCP 승인이 필요할 수 있습니다.
- Volicord는 호스트 신뢰, 프로젝트 신뢰, 프로젝트 MCP 승인, OAuth, 또는 그와 비슷한 사용자 통제 호스트 동작을 우회할 수 있다고 주장하면 안 됩니다.

<a id="agent-connection-command-effects"></a>
## Agent Connection 명령 효과

Agent Connection 명령은 서로 다른 상태 영역에 작용합니다. 정확한 Agent
Connection 의미는 [Agent Connection](agent-connection.md#lifecycle-and-state-boundaries)이
담당합니다.

| 명령 | Runtime Home 레지스트리 효과 | 호스트 설정 효과 | 검증 효과 |
|---|---|---|---|
| `volicord agent connect` | `agent_connections` 기록 하나를 만들거나 재사용하고, `enabled=true`를 설정하며, 선택한 `connection.mode`를 저장하고, 선택된 프로젝트를 `connection_projects`에 추가합니다. | 선택된 호스트와 범위에 맞는 관리 호스트 설정을 설치하거나 내보냅니다. | 가능한 곳에서 구현된 호스트/사전 점검/MCP 점검을 실행하고 결과 `last_verified_status`를 저장합니다. |
| `volicord agent list`와 `volicord agent status` | 저장된 Agent Connection 필드와 연결 프로젝트를 읽습니다. | 호스트를 시작하지 않고 호스트 설정을 다시 쓰지 않습니다. | 저장된 검증 상태를 보고하며 갱신하지 않습니다. |
| `volicord agent verify` | 기존 Agent Connection을 읽고 검증 결과에 따라 `last_verified_status`와 관리 지문을 갱신합니다. | 호스트 통합이 관찰 가능한 대상을 소유하면 그 관리 대상을 검사합니다. | 구현된 호스트 경로에 따라 Volicord 쪽 상태와 호스트/MCP 준비 상태를 점검합니다. |
| `volicord agent enable`과 `volicord agent disable` | Agent Connection 하나의 저장된 `enabled` 필드를 전환합니다. | 호스트 설정을 다시 쓰지 않습니다. | 검증을 갱신하지 않고 사용자 소유 판단을 만들지 않습니다. |
| `volicord agent project add`와 `volicord agent project remove` | `connection_projects` 멤버십 하나를 추가하거나 제거합니다. `project add`는 필요한 저장소 정보가 제공되면 프로젝트를 등록할 수 있습니다. | 호스트 설정을 다시 쓰지 않습니다. | 검증을 갱신하지 않습니다. |
| `volicord agent uninstall` | 관련 `connection_projects` 기록을 제거하고 멤버십이 남아 있지 않으면 Agent Connection 기록을 제거합니다. | 소유권과 안전 점검이 허용할 때 일치하는 관리 호스트 설정만 제거합니다. | `Product Repository` 내용, 프로젝트 등록 또는 프로젝트 상태, Core 기록, Runtime Home 자체, 아티팩트 저장소, 관련 없는 호스트 설정을 삭제하지 않습니다. |

<a id="agent-connection-result-states"></a>
<a id="agent-setup-result-states"></a>
## Agent Connection 결과 상태

에이전트 명령군은 아래 연결 결과 상태를 사용합니다.

| 상태 | 의미 |
|---|---|
| `not_verified` | Agent Connection에 현재 기록된 검증 결과가 없습니다. 호스트가 실패했다는 증거가 아닙니다. |
| `complete` | 오래 유지되는 Agent Connection 상태가 있고, 관리되는 호스트 설정이 존재하며 예상 관리 지문과 일치하고, 호스트별 로드 가능성 게이트가 충족되고, 필요한 신뢰나 승인 동작이 남아 있지 않고, 연결 사전 점검이 성공하고, MCP 초기화가 성공하고, `tools/list`가 필요한 도구를 노출합니다. |
| `action_required` | 오래 유지되는 Agent Connection 상태와 호스트 설정은 있지만 호스트 신뢰, 프로젝트 승인, OAuth, reload, restart, 또는 그와 비슷한 사용자 통제 호스트 동작이 남아 있습니다. |
| `failed` | 요청한 연결이나 검증이 사용할 수 있는 오래 유지되는 Agent Connection 상태 또는 호스트 설정을 만들지 못했습니다. |

`dry_run`은 출력 상태이며 설정 결과 상태가 아닙니다.

성공한 `volicord-mcp --check --connection <connection_id>`만으로는 Agent Connection을 `complete`로 설명하면 안 됩니다. 이는 MCP 프로세스의 시작 검증일 뿐입니다.

호스트별 상태 규칙:

- Codex project 범위는 Codex 프로젝트 신뢰를 확인할 수 없는 동안 `action_required`로 남습니다.
- Claude Code project 범위는 프로젝트 MCP 승인이 대기 중인 동안 `action_required`로 남습니다.
- 거절됨, 없음, 변경됨, 사용할 수 없음, 알 수 없음 호스트 상태는 `complete`가 되면 안 됩니다.
- Generic export는 Volicord가 외부 호스트가 내보낸 설정을 로드했다는 사실을 증명할 수 없으므로 `action_required`로 남습니다.

<a id="volicord-agent-connect"></a>
<a id="volicord-agent-install"></a>
## `volicord agent connect`

`volicord agent connect`는 Agent Connection을 만들거나 재사용하고, 선택된 프로젝트 하나를 명시적으로 연결하며, 호스트 설정을 설치하거나 내보내고, 호스트를 확인할 수 있을 때 결과를 검증합니다.

인자의 필수성 및 생략 시 동작:

| 인자 | 필수성 | 의미, 적용 범위, 생략 시 동작 |
|---|---|---|
| `--host codex|claude-code|claude_code|generic` | 항상 필수 | 호스트 어댑터를 선택합니다. 값은 선택한 `--scope`와 함께 유효해야 합니다. `claude-code`는 `claude_code`의 별칭으로 허용됩니다. |
| `--scope user|project|local|export` | 항상 필수 | 호스트 설정 또는 내보내기 대상을 선택합니다. 값은 선택한 `--host`와 함께 유효해야 합니다. |
| `--project-id ID` | 조건부 필수 | 선택된 프로젝트를 이름으로 가리킵니다. 등록되지 않은 프로젝트에는 필수이고, `--repo-root`가 기존 등록과 일치하지 않을 때도 필수이며, 제공된 `--repo-root`가 모호할 때도 필수입니다. `--repo-root`가 실행 가능한 기존 프로젝트 등록 하나와만 일치할 때만 생략할 수 있습니다. |
| `--repo-root PATH` | 조건부 필수 | 프로젝트 선택과 등록에 사용할 선택된 프로젝트의 `Product Repository`를 식별합니다. `--project-id`가 가리키는 프로젝트가 아직 등록되어 있지 않으면 함께 필요합니다. 이미 등록된 `--project-id`에 대해 생략하면 명령은 등록된 저장소 경로를 재사용합니다. |
| `--connection-id ID` | 선택 사항 | 기존 Agent Connection 또는 새 Agent Connection에 원하는 식별자를 선택합니다. 생략하면 명령은 선택된 호스트, 범위, 프로젝트, 대상, 서버 이름에서 안정적이고 결정적인 연결 식별자를 파생합니다. |
| `--mode read_only|workflow` | 선택 사항 | Agent Connection의 도구 노출 모드를 선택합니다. 생략하면 `read_only`이며, 워크플로 도구에는 `--mode workflow`가 필요합니다. |
| `--server-name NAME` | 선택 사항 | 호스트 MCP 서버 이름을 선택합니다. 생략하면 `volicord`를 사용합니다. |
| `--mcp-command PATH` | 선택 사항 | 명시적 명령이 허용되는 곳에서 `volicord-mcp` 실행 파일을 선택합니다. `project` 범위는 생략하면 이식 가능한 `volicord-mcp` 명령을 기본으로 사용하며 그 이식 가능한 명령을 유지해야 합니다. `user`, `local`, `export` 범위는 생략하면 현재 `volicord` 실행 파일의 형제 디렉터리와 `PATH` 순서로 실행 파일을 찾습니다. 이 범위에서 명시적 명령은 기존 실행 파일 경로 규칙을 만족해야 합니다. |
| `--runtime-home PATH` | 선택 사항 | 관리 명령이 사용할 `Volicord Runtime Home`을 선택합니다. 생략하면 위 Runtime Home 해석 순서를 사용합니다. `project`가 아닌 호스트 범위에서는 선택된 Runtime Home이 관리 호스트 설정에 `VOLICORD_HOME`으로 저장될 수 있습니다. `project` 범위에서는 공유 호스트 설정에 개발자별 Runtime Home 경로를 넣으면 안 됩니다. 기본값이 아닌 Runtime Home을 사용해야 하는 `project` 범위 호스트 프로세스는 실제 실행 환경을 통해 `VOLICORD_HOME`을 받아야 합니다. 관리 연결 명령에만 설정된 환경 변수는 이후 호스트 프로세스에 자동으로 상속되지 않습니다. |
| `--export-path PATH` | 선택 사항 | `generic` `export`에서 내보낸 MCP 설정을 쓸 명시적 출력 경로를 선택합니다. 생략하면 내보내기 경로는 `--export-dir` 또는 현재 작업 디렉터리에서 파생됩니다. |
| `--export-dir PATH` | 선택 사항 | `generic` `export`에서 `--export-path`가 생략됐을 때 생성 파일 이름 `volicord-<connection>.mcp.json`과 함께 사용할 디렉터리를 선택합니다. 두 내보내기 대상이 모두 생략되면 명령은 현재 작업 디렉터리를 사용하고 그 파일 이름을 파생합니다. |
| `--output text|json` | 선택 사항 | 사람이 읽는 text 출력 또는 기계 판독 JSON 출력을 선택합니다. 생략하면 출력은 `text`입니다. |
| `--dry-run` | 선택 사항 | zero-write dry-run 계약에 따라 연결 계획을 미리 보여 줍니다. 생략하면 명령은 실제 연결을 수행합니다. Dry-run은 `--allow-repository-write`를 요구하지 않습니다. |
| `--allow-repository-write` | 조건부 필수 권한 부여 | Dry-run이 아닌 `project` 범위 Agent Connection에는 필요합니다. 이 명령이 `Product Repository` 안의 호스트 설정을 쓰기 때문입니다. |
| `--replace-managed` | 선택 사항 | 기존 관리 소유권 제한이 일치하는 이전 관리 내용의 교체를 허용하는 경우에만 교체를 승인합니다. 생략하면 교체를 승인하지 않습니다. |

프로젝트 선택과 등록:

- 연결은 정확히 하나의 선택된 프로젝트를 해석해야 합니다.
- 등록되지 않은 프로젝트에는 `--project-id`와 `--repo-root`가 모두 필요합니다.
- 이미 등록된 프로젝트는 `--project-id`만으로 등록된 저장소 경로를 재사용할 수 있습니다.
- 이미 등록된 프로젝트에 `--project-id`와 `--repo-root`가 모두 제공되면 제공된 저장소 경로는 등록과 일치해야 합니다.
- `--repo-root`만으로 프로젝트를 선택할 수 있는 경우는 실행 가능한 기존 프로젝트 등록 하나와만 일치할 때뿐입니다.
- 제공된 `--repo-root`가 기존 등록과 일치하지 않으면 프로젝트를 등록할 수 있도록 `--project-id`가 필요합니다.
- 제공된 `--repo-root`가 둘 이상의 기존 등록과 일치하면 사용자는 `--project-id`를 제공해야 합니다.

연결 규칙:

- 명령은 Runtime Home의 모든 프로젝트를 연결하면 안 됩니다.
- project와 local 범위는 연결된 프로젝트 하나를 허용합니다.
- user 범위는 나중에 `volicord agent project add`로 프로젝트를 더 추가할 수 있습니다.
- 호스트 설정 쓰기는 관리 소유 마커 또는 동등한 관리 지문을 사용합니다.
- 관리 호스트 항목 지문은 형식 식별자 `volicord-host-entry-v1`을 사용합니다.
- 같은 호스트 대상과 서버 이름에 대한 기존 비관리 설정은 충돌입니다. `--replace-managed`는 소유 마커가 맞는 이전 관리 블록에만 적용됩니다.
- Dry-run이 아닌 `project` 범위 Agent Connection에는 `--allow-repository-write`가 필요합니다. Dry-run에는 필요하지 않습니다.
- `--dry-run`은 [Dry-run과 기계 판독 출력](#dry-run)이 정한 zero-write 계약에 따라 모든 저장소 및 파일 동작을 미리 보여 줍니다.

검증:

- 설치된 설정에서 호스트를 시작할 수 있으면 검증은 MCP 사전 점검, MCP 초기화, `tools/list` 탐색을 시도해야 합니다.
- 설정은 설치되었지만 호스트 신뢰나 승인이 로드를 막으면 결과는 `failed`가 아니라 `action_required`입니다.
- `volicord-mcp --check --connection <connection_id>`는 통과했지만 MCP 초기화나 도구 탐색이 성공하지 않았다면 결과는 `complete`가 될 수 없습니다.
- Volicord가 직접 시작한 MCP handshake는 Codex 또는 Claude Code가 서버를 로드, 신뢰, 승인, 노출했다는 사실을 증명하지 않습니다.

## Connection Project 멤버십 명령

`volicord agent project add`는 기존 Agent Connection에 연결 프로젝트 하나를 추가하거나 복원합니다.

규칙:

- `--connection-id`와 `--project-id`는 필수입니다.
- 프로젝트가 선택된 Runtime Home에 유효한 현재 프로젝트 등록으로 이미 등록되어 있으면 명령은 그 등록을 재사용합니다.
- 프로젝트가 등록되어 있지 않으면, 필요한 `--repo-root` 값이 제공될 때 명령이 프로젝트를 등록한 뒤 Connection Project 기록을 추가할 수 있습니다.
- 프로젝트가 등록되어 있지 않고 필요한 저장소 정보가 없으면 명령은 저장소 위치를 만들어 내지 않고 실패합니다.
- 프로젝트를 추가해도 inactive이거나 그 밖의 이유로 실행 부적격인 프로젝트가 실행 시점에 사용 가능해지는 것은 아닙니다.
- `project` 또는 `local` 범위 Agent Connection에 두 번째 프로젝트를 추가하는 것은 충돌입니다.
- 이 명령은 호스트 설정을 다시 쓰지 않습니다. 접근 철회와 추가는 레지스트리 변경입니다.

`volicord agent project remove`는 기존 Agent Connection에서 연결 프로젝트 하나를 제거합니다.

규칙:

- `--connection-id`와 `--project-id`는 필수입니다.
- 멤버십 제거는 프로젝트 상태, Core 기록, 호스트 설정, 조언용 Product Repository 지침 파일을 삭제하지 않습니다.
- 마지막 Connection Project를 제거하면 남아 있는 Agent Connection 기록은 프로젝트가 다시 연결되기 전까지 프로젝트 범위 워크플로에 사용할 수 없습니다.

## 상태, 활성화, 검증 명령

`volicord agent list`는 선택된 Runtime Home의 Agent Connection을 나열합니다.

`volicord agent status`는 호스트를 시작하지 않고 Agent Connection 하나를 보고합니다.

최소 보고 항목:

- `connection_id`
- `host_kind`
- `host_scope`
- `mode`
- 활성화 상태
- 연결된 프로젝트
- `last_verified_status`
- `server_name`
- `config_target`

`volicord agent enable`과 `volicord agent disable`은 Agent Connection 하나의 저장된 활성화 상태를 전환합니다. 호스트 설정을 다시 쓰지 않으며 사용자 소유 판단을 만들지 않습니다.

`volicord agent verify`는 Agent Connection 하나의 검증 상태를 갱신합니다.

검증해야 하는 항목:

- Agent Connection이 존재하고 활성화되어 있습니다.
- 검증 경로에 프로젝트 맥락이 필요하면 연결된 프로젝트를 읽을 수 있습니다.
- 직접 호스트 설정이 대상을 소유한다면 호스트 설정 대상이 존재하고 관리 지문과 여전히 일치해야 합니다.
- `volicord-mcp --check --connection <connection_id>`가 성공합니다.
- MCP 초기화가 성공합니다.
- `tools/list`가 연결 모드에 필요한 도구를 노출합니다.

검증은 Agent Connection의 `last_verified_status`에 지원되는 검증 상태 결과를 저장합니다. 명령 출력은 해당 점검이 실행된 경우 호스트 점검, 사전 점검, MCP handshake 점검을 보고합니다.

## 제거

`volicord agent uninstall`은 소유권과 안전 점검이 허용할 때 선택된 관리 Agent Connection 호스트 설정을 제거하고, 해당 Connection Project 기록을 제거하며, 더 이상 사용되지 않는 Agent Connection 기록을 제거합니다.

규칙:

- 제거는 적용 전에 관리 파일 편집을 미리 보여 줘야 합니다.
- 일치하는 Volicord 소유 마커 또는 관리 지문을 가진 블록, 파일, 항목만 제거해야 합니다.
- `Product Repository`, 프로젝트 상태, Core 기록, `Volicord Runtime Home` 위치 자체, 아티팩트 저장소, 관련 없는 호스트 설정을 삭제하면 안 됩니다.
- 프로젝트 범위 파일 편집은 비대화식 실행에서 `--allow-repository-write`를 요구합니다.
- 사용자가 호스트 파일을 이미 바꾼 경우 제거는 관련 없는 내용을 제거하지 말고 충돌을 보고해야 합니다.

## Product Repository 지침 경계

`Product Repository` 지침은 로컬 에이전트를 위한 조언 텍스트입니다. 도구 선택과 작업 흐름 일관성을 돕기 위해 `AGENTS.md`, 생성된 호스트 지침, MCP 서버 지침, 호스트별 규칙 파일에 있을 수 있습니다.

현재 `volicord` 관리 CLI에는 이 자료를 다루는 전용 명령군이 없습니다. Agent Connection 상태, Connection Projects, `connection.mode`, `last_verified_status`는 Runtime Home 레지스트리 상태에 저장되며 Product Repository 파일에 저장되지 않습니다. `volicord agent status`는 위에 나열한 Agent Connection 필드를 보고합니다. 조언 텍스트 파일을 위한 출력 필드는 없습니다.

사용자 판단은 `User Channel`을 통해 기록됩니다. 조언 텍스트는 사용자 판단, `Write Check`, 연결된 Project 멤버십, `connection.mode`, 증거, 수락, 닫기 준비 상태, 잔여 위험 수락, 접근 제어, 보안 강제, 모델이 Volicord 도구를 선택한다는 증명을 만들 수 없습니다. 정확한 `Product Repository` 쓰기 경계는 [런타임 경계](runtime-boundaries.md#explicit-integration-files-in-product-repositories)가 담당합니다.

<a id="dry-run"></a>
## Dry-run과 기계 판독 출력

`--dry-run`은 영속 변경 없이 계획, 검증, 충돌 감지, 호스트 대상 렌더링, 출력 형태 만들기를 수행합니다.

Dry-run이 하지 않는 것:

- `Volicord Runtime Home` 생성
- SQLite 데이터베이스 생성 또는 수정
- SQLite WAL 또는 SHM 파일 생성
- 레지스트리 또는 프로젝트 상태 마이그레이션 적용
- 프로젝트, Agent Connection, Connection Project, 검증 상태 행 등록 또는 갱신
- 호스트 설정 파일 생성, 수정, 제거
- `Product Repository` 파일이나 디렉터리 생성, 수정, 제거
- generic export 파일 생성, 수정, 제거
- `volicord-mcp --check` 호출
- MCP 초기화 또는 도구 탐색 수행

선택된 Runtime Home이 현재 저장소 프로필의 현재 registry를 가지고 있으면 dry-run은 마이그레이션 없이 이를 검사할 수 있고 registry 마이그레이션 계획이 없다고 보고합니다. 레지스트리를 마이그레이션하거나, 새 레지스트리 테이블을 만들거나, 프로젝트 상태 데이터베이스를 만들거나, 마이그레이션 메타데이터를 쓰면 안 됩니다. 지원되지 않는 registry 버전이나 저장소 프로필은 변환 또는 복구 없이 실패합니다.

Text 출력은 사람이 읽을 수 있어야 하며 각 리소스 작업을 `created`, `reused`, `updated`, `removed`, `skipped`, `conflict`, `planned` 중 하나로 식별해야 합니다.

<a id="setup-output"></a>
`volicord agent` 명령이 에이전트 결과 객체를 반환하는 경우 JSON 출력은 아래
최상위 키를 갖습니다.

```text
action
status
connection
verification
```

`volicord agent list --output json`은 단일 연결 결과 객체 대신 최상위
`connections` 배열 하나를 반환합니다.

필수 JSON 값:

- `status`: `complete`, `action_required`, `failed`, `not_verified`, 또는 `dry_run`
- `host_kind`: `codex`, `claude_code`, 또는 `generic`
- `host_scope`: `user`, `project`, `local`, 또는 `export`
- `mode`: `read_only` 또는 `workflow`
- `verification_status`: `not_verified`, `complete`, `action_required`, `failed`, 또는 `dry_run`

JSON 출력은 관리 CLI 출력이지 공개 Volicord API 응답 스키마가 아닙니다.

<a id="noninteractive-approval-behavior"></a>
## 비대화식 승인 동작

비대화식 명령은 명시적 사용자 승인이 없으면 프롬프트를 표시하지 말고 실패해야 합니다.

규칙:

- 프로젝트 범위 호스트 설정을 쓰거나, 교체하거나, 제거하는 모든 명령에는 `--allow-repository-write`가 필요합니다.
- `--replace-managed`는 소유 마커나 관리 지문이 일치하는 Volicord 관리 내용에만 적용됩니다.
- 포괄적 셸 승인, 쓰기 승인, 호스트 신뢰 결정, 민감 동작 승인, Write Check는 이 CLI 계약이 요구하는 명시적 관리 플래그를 대신하지 않습니다.
- 호스트 신뢰, 프로젝트 신뢰, 프로젝트 MCP 승인, OAuth, restart, reload 동작은 계속 사용자 통제 호스트 동작이며 CLI가 대신 제공할 수 없습니다.

## 프로젝트 등록

`volicord project register --project-id ID --repo-root PATH [--status active]`는 로컬 `Product Repository`를 선택된 Runtime Home에 등록합니다.

규칙:

- `--project-id`는 필수입니다.
- `--repo-root`는 필수입니다.
- `--status`의 기본값은 `active`입니다.
- 기준 등록은 `status=active`를 받습니다.
- `--repo-root`는 프로젝트 등록에 쓰는 로컬 저장소 루트를 식별합니다.
- 선택된 Runtime Home과 `--repo-root`는 등록이 기록되기 전에 [Runtime Home/Product Repository 분리 계약](runtime-boundaries.md#runtime-home-product-repository-separation)을 만족해야 합니다.

`volicord project list`는 선택된 Runtime Home의 현재 유효한 등록 프로젝트를 나열합니다.

선택된 프로젝트 registry 행 중 하나라도 잘못된 형태이거나 현재 등록 불변 조건을 위반하면, `volicord project list`는 그 행을 생략하거나 정상 프로젝트로 반환하지 않고 표준 CLI 오류 경로로 실패합니다. 잘못된 형태의 원시 registry 내용은 검사 계층에서 계속 진단할 수 있습니다.

`Product Repository`와 `Volicord Runtime Home`의 구분을 포함한 런타임 위치 경계는 [런타임 경계](runtime-boundaries.md#runtime-home-product-repository-separation)가 담당합니다.

## 관리 경계

관리 CLI는 로컬 리소스를 초기화하고 등록할 수 있습니다. 그 자체로 공개 Volicord API 메서드를 만들지 않으며 Core 권한, Write Check 호환성, 증거 충분성, 닫기 준비 상태, 사용자 소유 판단, 수락, 잔여 위험 수락, 아티팩트 권한, 보안 보장을 만들지 않습니다.

담당 문서 경로:

- 공개 메서드 목록과 메서드 경로: [API 메서드](api/methods.md).
- 공통 요청/응답 스키마: [API 코어 스키마](api/schema-core.md).
- Agent Connection, Connection Project, 행위자 맥락 의미: [Agent Connection](agent-connection.md).
- MCP 프로세스 동작: [MCP 전송](mcp-transport.md).
- 런타임 위치와 저장소 쓰기 경계: [런타임 경계](runtime-boundaries.md).
