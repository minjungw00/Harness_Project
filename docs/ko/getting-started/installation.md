# 설치

이 문서는 현재 저장소 실행 파일의 소스 전제 조건과 빌드 절차를 담당합니다. 패키지 관리자 배포, 운영체제 지원, 공개 API 동작, 저장 효과, MCP 와이어 동작을 정의하지 않습니다.

## 전제 조건

소스 빌드 경로에는 아래가 필요합니다.

- 이 저장소의 로컬 복제본
- 워크스페이스를 빌드할 수 있는 Rust 도구체인과 Cargo
- Cargo와 로컬 실행 파일을 실행할 수 있는 셸
- 첫 설정에 바인딩할 로컬 `Product Repository` 디렉터리
- `Product Repository`, `Harness Runtime Home`, 실행 파일 경로를 설정에 전달할 때 사용할 절대 경로

MCP 호스트는 생성된 호스트 중립 설정을 실제 호스트에 연결할 때만 필요합니다. 빌드와 설정 사전 점검은 특정 외부 호스트 이름 없이 실행할 수 있습니다.

## 저장소 루트에서 빌드

빠른 로컬 빌드는 아래처럼 실행합니다.

```sh
cargo build -p harness-cli -p harness-mcp
```

예상되는 디버그 실행 파일:

- `target/debug/harness`
- `target/debug/harness-mcp`

릴리스 실행 파일은 아래처럼 빌드합니다.

```sh
cargo build --release -p harness-cli -p harness-mcp
```

예상되는 릴리스 실행 파일:

- `target/release/harness`
- `target/release/harness-mcp`

Cargo 패키지 이름은 `harness-cli`와 `harness-mcp`입니다. 실행 파일 이름은 `harness`와 `harness-mcp`입니다.

## 빌드 확인

빠른 로컬 빌드 뒤 저장소 루트에서 실행합니다.

```sh
target/debug/harness --version
target/debug/harness setup local-mcp --help
target/debug/harness-mcp --version
target/debug/harness-mcp --help
```

버전 명령은 `harness <version>`과 `harness-mcp <version>`을 출력합니다. 도움말 명령은 로컬 관리 설정 사용법과 MCP 환경 사용법을 출력해야 합니다.

## 설정 중 실행 파일 탐색

`harness setup local-mcp`가 설정을 수행합니다. `harness-mcp`는 설정 뒤 MCP 호스트가 시작하는 자식 프로세스입니다.

설정은 아래 조건에서 `harness-mcp`를 찾을 수 있습니다.

- `target/debug/` 또는 `target/release/`처럼 `harness-mcp`가 실행 중인 `harness` 옆에 있음
- `harness-mcp`가 `PATH`에 있음

설정이 정확히 하나의 실행 파일을 사용하게 하려면 `--mcp-command /absolute/path/to/harness-mcp`를 전달합니다. 생성되는 호스트 중립 설정에는 선택된 명령 경로가 기록됩니다.

## 다음 단계

가장 짧은 로컬 MCP 설정 경로는 [빠른 시작](quickstart.md)으로 이어집니다.

정확한 명령 동작은 [관리 CLI](../reference/admin-cli.md)가 담당합니다. 정확한 `harness-mcp` 시작, 환경, stdio 전송, 사전 점검, 종료 동작은 [MCP 전송](../reference/mcp-transport.md)이 담당합니다.
