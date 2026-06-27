# 설치

이 튜토리얼은 첫 호스트 setup 전에 Volicord 실행 파일을 준비합니다. 실행 파일 출처를 선택하고 `volicord`와 `volicord-mcp`를 확인하며, 선택한 바이너리가 [Quickstart](quickstart.md)에 준비되었는지 판단합니다.

이 문서는 공개 API 동작, 저장 효과, `Product Repository` 등록, 호스트 trust, MCP wire 동작을 정의하지 않습니다.

## 대상, 목표, 완료

대상: 에이전트 호스트를 연결하기 전에 동작하는 로컬 `volicord` 관리 CLI와 `volicord-mcp` MCP 어댑터가 필요한 첫 사용자, 운영자, 구현자.

목표: 소스 빌드 출력 디렉터리나 별도로 설치된 실행 파일 디렉터리 하나를 선택하고, 같은 POSIX 스타일 셸에서 두 실행 파일이 모두 실행되는지 증명합니다.

완료 상태: `VOLICORD_BIN`이 실행 가능한 `volicord`와 `volicord-mcp` 파일을 모두 포함하는 절대 디렉터리 하나를 가리키고, 아래 version/help 확인이 모두 성공합니다. 이는 실행 파일이 Agent Connection setup에 준비되었다는 뜻입니다. `Volicord Runtime Home`, `Product Repository`, 호스트 설정이 만들어졌다는 뜻은 아닙니다.

## 전제 조건

경로를 고르기 전에 [시스템 요구사항](../reference/system-requirements.md)을 읽습니다. 이 페이지의 명령 예시는 `export`, `$(pwd)`, 따옴표로 감싼 변수 확장, 인라인 `PATH=...`, `test -x` 같은 POSIX 스타일 셸 문법을 사용합니다.

아래 setup 경로 중 하나를 사용합니다.

| 경로 | 사용할 때 | 계속하기 전 |
|---|---|---|
| 소스 빌드 | 이 저장소 checkout이 있고 현재 workspace 실행 파일을 빌드하려는 경우. | Rust 1.85 이상과 Cargo를 사용할 수 있고 Cargo가 workspace 의존성을 해석할 수 있어야 합니다. |
| 별도 설치 실행 파일 | Volicord 설치 디렉터리가 이미 있는 경우. | 하나의 절대 디렉터리에 `volicord`와 `volicord-mcp`가 모두 있어야 합니다. |

다음 setup 단계에는 로컬 `Product Repository`, 별도 `Volicord Runtime Home`, Codex나 Claude Code 같은 지원 호스트 경로도 필요합니다.

## 경로 A: 소스에서 빌드

작업 디렉터리: Volicord 소스 저장소 루트.

먼저 비변경 toolchain 확인을 실행합니다.

```sh
cargo --version
rustc --version
```

둘 중 하나를 사용할 수 없거나 선택한 Rust 컴파일러가 1.85보다 오래되었으면 빌드하기 전에 toolchain을 고칩니다.

디버그 빌드:

```sh
cargo build -p volicord-cli -p volicord-mcp
export VOLICORD_BIN="$(pwd)/target/debug"
```

릴리스 빌드:

```sh
cargo build --release -p volicord-cli -p volicord-mcp
export VOLICORD_BIN="$(pwd)/target/release"
```

나머지 셸 세션에서 사용할 빌드 출력 하나를 선택합니다. Cargo 패키지 이름은 `volicord-cli`와 `volicord-mcp`이고 실행 파일 이름은 `volicord`와 `volicord-mcp`입니다.

## 경로 B: 설치된 실행 파일 선택

실행 파일이 소스 checkout과 별도로 설치되었다면 이 경로를 사용합니다.

```sh
export VOLICORD_BIN="/absolute/path/to/installed/bin"
```

`/absolute/path/to/installed/bin`을 두 실행 파일이 들어 있는 실제 절대 디렉터리로 바꿉니다. 예시 값을 그대로 복사하지 않습니다.

## 선택한 디렉터리 확인

`VOLICORD_BIN`을 설정한 같은 셸에서 실행합니다.

```sh
test -x "$VOLICORD_BIN/volicord"
test -x "$VOLICORD_BIN/volicord-mcp"

"$VOLICORD_BIN/volicord" --version
"$VOLICORD_BIN/volicord" agent --help
"$VOLICORD_BIN/volicord-mcp" --version
"$VOLICORD_BIN/volicord-mcp" --help
```

version 명령은 `volicord <version>`과 `volicord-mcp <version>`을 출력합니다. help 명령은 `volicord agent connect` 명령군과 `volicord-mcp --connection <connection_id>` 프로세스 사용법을 보여야 합니다.

`VOLICORD_BIN`은 이 예시를 위한 셸 편의 변수일 뿐입니다. Volicord는 이를 설정으로 읽지 않고 생성된 호스트 설정에 그대로 저장하지도 않습니다. 새 셸을 열면 다시 설정하거나 절대 경로를 직접 사용합니다.

## 호스트 setup에서 이 선택을 쓰는 방식

`volicord agent connect`는 `volicord-mcp --connection <connection_id>`를 시작하는 호스트 설정을 설치하거나 내보냅니다.

사용자 범위 Codex나 사용자/로컬 범위 Claude Code setup에서는 `--mcp-command "$VOLICORD_BIN/volicord-mcp"`로 선택한 절대 실행 파일 경로를 전달하거나, CLI가 찾을 수 있도록 `volicord-mcp`를 `volicord` 옆이나 `PATH`에 둡니다. 지속되는 호스트 설정에는 셸 변수가 아니라 해석된 절대 명령 경로가 들어갑니다.

프로젝트 범위 Codex 또는 Claude Code setup에서는 생성된 프로젝트 파일이 공유 가능해야 합니다. `PATH="$VOLICORD_BIN:$PATH"`로 setup을 실행하고 `--mcp-command`는 생략합니다. 프로젝트 파일은 이식 가능한 명령 이름을 유지하고, 나중의 호스트 프로세스는 자기 `PATH`에서 `volicord-mcp`를 찾을 수 있어야 합니다.

설치 위치는 런타임 상태가 아닙니다. Volicord 소스나 설치 파일은 실행 파일을 포함하고, `Volicord Runtime Home`은 Volicord 런타임 기록을 포함하며, `Product Repository`는 제품 파일과 선택된 프로젝트 범위 호스트 설정을 포함합니다. 실제 설정과 trust 상태는 에이전트 호스트가 소유합니다.

## 실패 라우팅

| 증상 | 안전한 다음 행동 | 경로 |
|---|---|---|
| `cargo` 또는 `rustc`를 사용할 수 없습니다. | Rust 1.85 이상과 Cargo를 설치하거나 선택한 뒤 preflight 확인을 다시 실행합니다. | [시스템 요구사항](../reference/system-requirements.md#toolchain-requirements) |
| Rust가 1.85보다 오래되었습니다. | `cargo build` 전에 Rust 1.85 이상 toolchain을 선택합니다. | [시스템 요구사항](../reference/system-requirements.md#toolchain-requirements) |
| `cargo build`가 실패합니다. | Cargo 진단을 읽고 보고된 toolchain, 의존성, 소스 문제를 고친 뒤 같은 빌드 명령을 다시 실행합니다. 첫 대응으로 Runtime Home이나 Product Repository를 삭제하지 않습니다. | [시스템 요구사항](../reference/system-requirements.md#toolchain-requirements) |
| `target/debug` 또는 `target/release`에 두 실행 파일이 모두 없습니다. | 성공한 빌드 명령을 확인하고 일치하는 출력 디렉터리를 선택한 뒤 `test -x` 확인을 다시 실행합니다. | [시스템 요구사항](../reference/system-requirements.md#executable-layout-and-discovery) |
| `test -x` 또는 help/version 명령이 실패합니다. | 실제 실행 가능한 `volicord`와 `volicord-mcp`가 들어 있는 디렉터리를 선택하거나 선택된 사용자의 실행 권한을 고칩니다. | [에이전트 호스트 문제 해결](../guides/agent-host-troubleshooting.md#missing-volicord-mcp) |
| `VOLICORD_BIN`이 잘못된 디렉터리를 가리킵니다. | 같은 셸에서 올바른 절대 디렉터리를 내보낸 뒤 모든 확인 명령을 다시 실행합니다. | [에이전트 호스트 문제 해결](../guides/agent-host-troubleshooting.md#wrong-absolute-mcp-command) |
| 이후 프로젝트 범위 호스트가 `volicord-mcp`를 찾지 못합니다. | 프로젝트 파일은 이식 가능한 형태로 유지하고 호스트 시작 환경의 `PATH`를 고칩니다. | [에이전트 호스트 문제 해결](../guides/agent-host-troubleshooting.md#portable-project-command-not-on-path) |

## 다음 단계

이 페이지의 모든 확인 명령이 성공하면 [Quickstart](quickstart.md)로 계속합니다.

정확한 명령 동작은 [관리 CLI](../reference/admin-cli.md)가 담당합니다. 정확한 `volicord-mcp` 시작, 환경, stdio 전송, preflight, 종료 동작은 [MCP 전송](../reference/mcp-transport.md)이 담당합니다.
