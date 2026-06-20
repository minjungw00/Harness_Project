# 설치

이 문서는 초기 설정의 1단계, 즉 `Harness Server` 실행 파일 준비를
담당합니다. 현재 저장소 실행 파일의 소스 전제 조건, 빌드 명령, 실행 파일
경로, 빌드 확인을 다룹니다. 패키지 관리자 배포, 운영체제 지원, 공개 API 동작,
저장 효과, `Product Repository` 등록, 외부 호스트 설정, MCP 와이어 동작을
정의하지 않습니다.

## 전제 조건

소스 빌드 경로에는 아래가 필요합니다.

- 이 저장소의 로컬 복제본
- 워크스페이스를 빌드할 수 있는 Rust 도구체인과 Cargo
- Cargo와 로컬 실행 파일을 실행할 수 있는 셸
- 다음 설정 단계에서 바인딩할 로컬 `Product Repository` 디렉터리
- 다음 설정 단계에서 사용할 별도의 `Harness Runtime Home`

MCP 호스트는 생성된 호스트 중립 설정을 실제 호스트에 연결할 때만 필요합니다. 빌드와 설정 사전 점검은 특정 외부 호스트 이름 없이 실행할 수 있습니다.

## 저장소 루트에서 빌드

작업 디렉터리: `Harness Server` 소스 저장소 루트.

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

작업 디렉터리: 빠른 로컬 빌드 뒤의 `Harness Server` 소스 저장소 루트.

```sh
target/debug/harness --version
target/debug/harness setup local-mcp --help
target/debug/harness-mcp --version
target/debug/harness-mcp --help
```

버전 명령은 `harness <version>`과 `harness-mcp <version>`을 출력합니다. 도움말 명령은 로컬 관리 설정 사용법과 MCP 환경 사용법을 출력해야 합니다.

## 설정 중 실행 파일 탐색

`harness setup local-mcp`는 다음 단계에서 설정을 수행합니다. `harness-mcp`는
설정 뒤 MCP 호스트가 시작하는 자식 프로세스입니다.

설정은 아래 조건에서 `harness-mcp`를 찾을 수 있습니다.

- `target/debug/` 또는 `target/release/`처럼 `harness-mcp`가 실행 중인 `harness` 옆에 있음
- `harness-mcp`가 `PATH`에 있음

설정이 정확히 하나의 실행 파일을 사용하게 하려면 `--mcp-command /absolute/path/to/harness-mcp`를 전달합니다. 생성되는 호스트 중립 설정에는 선택된 명령 경로가 기록됩니다.

설치 위치는 런타임 상태가 아닙니다. `Harness Server` 소스 저장소나 설치는 실행
파일을 담고, `Harness Runtime Home`은 하네스 런타임 기록을 담으며, `Product
Repository`는 제품 파일을 담습니다. 외부 MCP 호스트는 자기 실제 설정 파일을
소유합니다.

## 다음 단계

[빠른 시작](quickstart.md)으로 이어집니다. 빠른 시작은 `Product Repository`
루트에서 시작하며, 가장 짧은 로컬 MCP 설정 경로에 `--repo-root .`을 사용합니다.

정확한 명령 동작은 [관리 CLI](../reference/admin-cli.md)가 담당합니다. 정확한 `harness-mcp` 시작, 환경, stdio 전송, 사전 점검, 종료 동작은 [MCP 전송](../reference/mcp-transport.md)이 담당합니다.
