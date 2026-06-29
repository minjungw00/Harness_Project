# 설치

이 튜토리얼은 로컬 `volicord`와 `volicord-mcp` 실행 파일을 준비하고, 이후
프로젝트, 연결, export, `User Channel` 명령이 사용할 setup 프로필을 기록합니다.
[Quickstart](quickstart.md) 전에 수행하는 설정 단계입니다.

정확한 명령 동작은 [관리 CLI 참조](../reference/admin-cli.md)가 담당합니다.
런타임 위치와 저장소 분리는 [런타임 경계](../reference/runtime-boundaries.md)가
담당합니다.

## 전제 조건

- [시스템 요구사항](../reference/system-requirements.md)에 적힌 Rust 1.85 이상.
- Cargo와 로컬 바이너리를 실행할 수 있는 셸.
- 호스트를 연결할 준비가 되었을 때 사용할 Git 기반 제품 저장소.

## 소스에서 빌드하기

Volicord 소스 저장소에서 실행합니다.

```sh
cargo build --workspace --bins
```

이 명령은 두 로컬 실행 파일을 빌드합니다.

- `./target/debug/volicord`
- `./target/debug/volicord-mcp`

그다음 setup 프로필을 만듭니다.

```sh
./target/debug/volicord setup --link-bin ~/.local/bin
```

`volicord setup`은 기본 `Volicord Runtime Home`을 준비하고, `volicord-mcp`를
찾고, setup 프로필을 기록하며, `--link-bin`이 제공되면 `volicord` 명령 링크를
만듭니다. `~/.local/bin`이 아직 셸 `PATH`에 없다면 추가합니다.

Setup 준비 상태를 확인합니다.

```sh
volicord doctor
```

Runtime Home, setup 프로필, MCP 명령이 준비되면 `doctor`가 `complete`를
보고합니다. `action_required`는 setup 재실행이나 실행 파일 경로 수정처럼 구체적인
로컬 복구 동작을 찾았다는 뜻입니다.

## 설치된 실행 파일 사용하기

`volicord`와 `volicord-mcp`가 이미 `PATH`에 있다면 아래처럼 실행합니다.

```sh
volicord setup
volicord doctor
```

Setup은 실행 중인 설치에서 MCP 명령을 찾습니다. 기본값이 아닌 Runtime Home이나
MCP 실행 파일 위치를 의도적으로 선택할 때만 setup 옵션을 사용합니다. 일반적인 첫
연결 명령에서 내부 호스트와 registry 값은 Volicord가 관리합니다.

## Setup이 하지 않는 일

Setup은 제품 저장소를 등록하지 않고 호스트 설정을 설치하지도 않습니다. 프로젝트
등록은 Git 저장소 안에서 `volicord project use`나 `volicord connect` 같은 명령을
실행할 때 이루어집니다.

저장소 프로젝트 이름은 저장소 디렉터리에서 파생되고, 선택된 Runtime Home 안에서
필요하면 고유하게 만들어집니다. 내부 ID는 Volicord가 저장하며 첫 setup 입력이
아닙니다.

## 다음 단계

제품 저장소로 이동해 호스트를 연결합니다.

```sh
cd /work/acme-api
volicord connect codex
```

전체 첫 실행 경로는 [Quickstart](quickstart.md)를 계속 읽습니다. 호스트별
세부사항은 [에이전트 호스트 Setup](../guides/agent-host-setup.md)을 봅니다.
