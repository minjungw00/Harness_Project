# 하네스 에이전트 통합 가이드 v04

## 1. 문서 목적

이 문서는 하네스를 Codex, Claude Code, Gemini, GitHub Copilot, Cursor 같은 agent surface와 연결하는 기준을 정의한다.

이 문서는 다음을 다룬다.

- 에이전트 표면별 통합 목표
- 공통 통합 구조
- surface capability profile
- connector 생성물과 drift 관리
- 표면별 연결 기준
- fallback 원칙
- 설치 wizard와 진단 기준
- 통합 conformance 기준

이 문서는 다음을 직접 다루지 않는다.

- 하네스 코어의 데이터베이스 스키마 전문
- MCP tool의 내부 구현
- 저장소 문서 템플릿 전문
- 사용자 작업 절차의 자세한 설명
- 특정 제품 버전에 종속되는 기능 보장

사용자는 이 문서를 직접 읽지 않아도 하네스를 사용할 수 있어야 한다.

이 문서는 하네스 연결을 구현하거나 검토하는 운영자와 adapter 작성자를 위한 문서다.

## 2. 통합 목표

하네스 통합의 목표는 사용자가 에이전트와 대화하는 것만으로 하네스의 작업 구조를 얻는 것이다.

사용자가 직접 조합하지 않아야 하는 내부 요소는 다음이다.

- Task 생성
- mode 분류
- Change Unit 생성
- scope와 acceptance 기준 정리
- approval 요청 생성
- evidence 등록
- evidence manifest 갱신
- validator 실행
- detached verify launch
- acceptance 기록
- projection 갱신
- reconcile 처리

에이전트 표면은 사용자의 자연어 요청을 받고, Harness Skill과 Harness MCP를 사용해 이 내부 요소를 처리한다.

## 3. 공통 통합 구조

모든 표면은 가능한 범위에서 다음 네 층을 가진다.

```text
사용자 대화
  ↓
항상 읽히는 rule/context
  ↓
Harness Skill 또는 playbook
  ↓
Harness MCP server
  ↓
Harness Core + adapter/hook/sidecar
```

### 3.1 항상 읽히는 rule/context

항상 읽히는 파일은 짧은 원칙만 담는다.

예시는 다음이다.

- `AGENTS.md`
- `CLAUDE.md`
- `GEMINI.md`
- GitHub Copilot custom instructions
- Cursor rules

이 층에는 긴 절차, 상태 전이, validator 설명을 넣지 않는다.

### 3.2 Harness Skill 또는 playbook

Skill은 에이전트가 다음을 알게 한다.

- 언제 하네스를 써야 하는가
- 새 작업을 어떻게 시작하는가
- direct와 work를 어떻게 구분하는가
- approval이 필요하면 무엇을 해야 하는가
- work가 끝나면 왜 detached verify가 필요한가
- evidence manifest를 언제 갱신해야 하는가
- 사용자에게 어떤 상태 카드를 보여줘야 하는가

Skill은 절차 안내다. Skill 자체는 정책을 집행하지 않는다.

### 3.3 Harness MCP server

MCP server는 에이전트가 하네스를 실제로 조작하는 기본 API다.

MCP server는 다음을 제공한다.

- 현재 프로젝트와 active Task resource
- 상태 카드 resource
- direct/work/verify/approval prompt
- intake, next, approval, record, evidence, verify, close 같은 high-level tool

MCP tool은 CLI 명령의 1:1 노출이 아니라 상태기계 기반 intent tool이어야 한다.

### 3.4 hook / adapter / sidecar

hook, adapter, sidecar는 다음을 집행한다.

- allowed path guard
- approval scope guard
- artifact capture
- command output capture
- same-session verify guard
- evaluator read-only guard
- projection refresh
- stale evidence detection
- generated file drift detection

표면이 native hook을 제공하지 않으면 sidecar와 validator가 보완한다.

## 4. Capability tier

각 표면은 하네스가 보장할 수 있는 수준이 다르다.

하네스는 표면을 다음 tier로 평가한다.

| Tier | 의미 | 필요한 기능 |
|---|---|---|
| T0 Context | 하네스 원칙을 읽을 수 있음 | rules/context file |
| T1 Skill | 하네스 절차를 호출할 수 있음 | Skill, custom command, prompt file |
| T2 MCP | 하네스 상태와 도구를 조작할 수 있음 | MCP server 연결 |
| T3 Capture | diff, logs, run output을 안정적으로 회수할 수 있음 | structured output, CLI wrapper, adapter |
| T4 Guard | 파일, 명령, network, approval 범위를 사전에 막을 수 있음 | hook, permission, policy engine, sidecar |
| T5 Isolation | verify 또는 risky run을 분리 환경에서 실행할 수 있음 | worktree, sandbox, fresh run launcher |

통합은 최소 T2를 목표로 한다.

일상 하네스 사용은 T2 이상에서 자연스럽다.

`work`의 신뢰 가능한 detached verify는 T3 이상을 요구하고, 고위험 작업은 T4 또는 T5가 필요하다.

## 5. Surface capability profile

하네스는 제품명만으로 capability를 단정하지 않는다.

각 연결은 target profile과 검증 시점을 가진다.

예시:

```yaml
surface_id: SURF-0001
surface_type: claude_code
target_profile: local_cli
detected_version: optional string
capability_profile_version: 3
last_verified_at: 2026-04-23T10:05:00+09:00
support_tier: T4
capabilities:
  project_rules: true
  skills: true
  mcp_tools: true
  mcp_resources: true
  mcp_prompts: true
  structured_output: true
  hooks: true
  pre_tool_guard: true
  explicit_permissions: true
  fresh_verify: true
  worktree_isolation: via_sidecar
  subagents: supported_but_bounded
  local_sidecar: true
risks:
  - version-dependent hook behavior
fallbacks:
  - sidecar file watcher
  - changed_paths validator
  - fresh worktree evaluator
```

### 5.1 target profile

같은 제품군도 실행 형태가 다르면 capability가 다를 수 있다.

권장 target profile 예시는 다음이다.

- `local_cli`
- `ide_chat`
- `ide_agent`
- `cloud_agent`
- `extension`
- `custom_agent`
- `manual_bundle`

### 5.2 capability 재검증

다음 경우 capability를 재검증한다.

- 제품 또는 extension 버전 변경
- MCP 설정 변경
- hook 또는 permission 설정 변경
- workspace policy 변경
- connector generated file drift 발생
- conformance 실패

## 6. 공통 설치 wizard

하네스는 표면별 설정을 사용자가 직접 작성하게 하지 않고, 설치 wizard가 생성한다.

예시:

```bash
harness connect agents --auto
```

명시적으로 표면을 고를 수도 있다.

```bash
harness connect agents --surfaces codex,claude,gemini,copilot,cursor
```

설치 wizard는 다음을 수행한다.

1. 저장소 루트를 확인한다.
2. 프로젝트가 하네스에 연결되어 있는지 확인한다.
3. Harness MCP server 실행 방법을 등록한다.
4. 표면별 rule/context 파일을 생성하거나 갱신한다.
5. 표면별 Skill 또는 playbook을 생성하거나 갱신한다.
6. 가능한 경우 hook, adapter, sidecar 연결을 설정한다.
7. target profile과 capability를 진단한다.
8. conformance smoke test를 실행한다.
9. 사용자가 실제로 말할 수 있는 첫 대화 예시를 출력한다.

진단 명령은 다음이다.

```bash
harness doctor agents
```

## 7. 공통 생성물

표면별 차이가 있어도 다음 생성물은 공통 개념을 가진다.

### 7.1 저장소 공통 규칙

저장소 루트에는 얇은 `AGENTS.md`를 둔다.

이 파일은 다음 정도만 말한다.

- 하네스 상태를 먼저 확인한다.
- 작은 작업은 direct로 처리한다.
- 큰 작업은 work로 처리한다.
- 제품 파일 쓰기 전 Harness MCP를 사용한다.
- 민감 범주는 approval 없이 진행하지 않는다.
- work는 detached verify 없이 완료로 닫지 않는다.
- 대화보다 하네스 상태와 증거를 우선한다.

### 7.2 Harness Skill

Skill은 표면별 규격에 맞게 설치되지만 내용은 같은 의도를 가진다.

Skill description은 다음 상황에서 자동 호출되도록 작성한다.

- 사용자가 코드 변경을 요청한다.
- 사용자가 작업 상태를 묻는다.
- 사용자가 멈춘 작업을 이어달라고 한다.
- 사용자가 승인 또는 거절을 말한다.
- 사용자가 검증 또는 완료를 요청한다.
- 사용자가 설계 결정을 남겨달라고 한다.

### 7.3 Harness MCP 설정

각 표면은 하네스 MCP server를 호출할 수 있어야 한다.

MCP server는 local stdio 또는 local HTTP 방식 중 하나를 사용할 수 있다.

기본 권장은 local stdio다.

### 7.4 상태 카드 형식

모든 표면은 사용자에게 같은 형식의 기본 상태 카드를 보여줘야 한다.

```text
TASK-XXXX 제목
상태: mode / phase
다음 행동:
사용자 판단:
리스크:
증거:
최신 보고:
```

사용자가 자세한 상태를 요청하면 상세 상태 카드를 보여준다.

### 7.5 generated file 관리

하네스가 생성한 파일이나 블록은 connector manifest에 기록한다.

가능한 경우 managed marker를 사용한다.

```md
<!-- HARNESS:BEGIN managed -->
...
<!-- HARNESS:END managed -->
```

수동 수정된 파일은 덮어쓰기 전에 diff를 보여주거나 reconcile 대상으로 올린다.

## 8. Codex 통합 기준

### 8.1 역할

Codex는 하네스의 coding surface 후보 중 하나다.

Codex 통합은 다음을 목표로 한다.

- `AGENTS.md`를 통한 얇은 항상-on 규칙
- Harness Skill을 통한 절차 로딩
- Harness MCP를 통한 상태 조작
- CLI/IDE surface를 통한 코드 변경
- adapter 또는 sidecar를 통한 artifact capture
- detached verify를 위한 fresh run 또는 다른 evaluator surface launch

### 8.2 target profile

Codex connector는 실제 사용 형태를 target profile로 기록한다.

```yaml
surface_type: codex
target_profile: local_cli | ide_chat | custom
```

각 target profile은 capability를 별도로 검증한다.

### 8.3 생성물

Codex 연결 시 하네스는 다음 개념의 파일 또는 설정을 생성한다.

```text
repo/
  AGENTS.md
  .harness/agent/codex/harness-skill/SKILL.md
  .harness/agent/codex/mcp.json
```

실제 설치 위치는 Codex 설정 방식에 맞춰 adapter가 결정한다.

### 8.4 기본 동작

Codex는 다음 순서로 동작해야 한다.

1. `AGENTS.md`를 확인한다.
2. Harness Skill을 사용한다.
3. `harness.status` 또는 `harness.intake`를 호출한다.
4. 하네스가 반환한 mode와 next action을 따른다.
5. 코드 변경 전 `harness.prepare_write`를 호출한다.
6. 변경 후 `harness.record_change`를 호출한다.
7. acceptance criteria가 영향을 받으면 `harness.update_evidence_manifest`를 호출한다.
8. direct면 `harness.finish_direct`, work 구현이면 `harness.finish_implementation`을 호출한다.
9. work면 `harness.launch_verify`를 호출하거나 사용자의 검증 확인을 요청한다.

### 8.5 guard 수준

Codex 통합에서 guard는 다음 순서로 보장한다.

1. MCP의 `prepare_write` 정책 확인
2. adapter의 allowed path 검사
3. sidecar watcher 또는 command wrapper
4. git diff 기반 `changed_paths` validator
5. detached verify의 재확인

사전 tool hook이 충분하지 않은 target profile에서는 sidecar와 validator가 강제 집행을 보완한다.

## 9. Claude Code 통합 기준

### 9.1 역할

Claude Code는 하네스의 high-control surface 후보 중 하나다.

Claude Code 통합은 다음을 목표로 한다.

- `CLAUDE.md`를 통한 짧은 항상-on 규칙
- Harness Skill
- Harness MCP server 연결
- 가능한 경우 hook 기반 사전/사후 guard
- read-only evaluator profile 또는 fresh run
- stop 시점 report 초안 생성

### 9.2 target profile

```yaml
surface_type: claude_code
target_profile: local_cli | ide_chat | custom
```

각 target profile은 hooks, permissions, subagent, fresh verify 지원을 별도로 검증한다.

### 9.3 hook 책임

hook 통합이 가능한 target profile에서는 다음 지점에서 하네스를 호출한다.

| 지점 | 하네스 동작 |
|---|---|
| SessionStart | active Task와 status card 주입 |
| UserPromptSubmit | 요청이 작업인지 판단하고 `harness.intake` 유도 |
| PreToolUse | edit/write/bash/network/secret 접근 전에 approval 및 scope 검사 |
| PostToolUse | 변경 파일, command output, logs를 artifact candidate로 등록 |
| Stop | direct result 또는 run summary 초안 생성, verify 필요 여부 표시 |

### 9.4 evaluator

Evaluator는 다음 중 하나일 수 있다.

- fresh session
- read-only evaluator profile
- 다른 표면의 evaluator run
- manual verify bundle

read-only evaluator는 제품 코드 쓰기 권한을 가져서는 안 된다.

same-session self-review는 detached verify로 인정하지 않는다.

## 10. Gemini 통합 기준

Gemini 통합은 extension, prompt, MCP 연결을 중심으로 한다.

목표는 다음이다.

- Harness extension 또는 playbook으로 prompts, commands, MCP 설정을 묶는다.
- `GEMINI.md` 성격의 context는 짧게 유지한다.
- 하네스 MCP를 통해 Task 상태와 approval을 조작한다.
- CLI wrapper 또는 sidecar로 artifact capture와 guard를 보완한다.

Target profile 예시는 다음이다.

```yaml
surface_type: gemini
target_profile: local_cli | extension | ide_chat | custom
```

사전 guard가 표면 자체에서 충분하지 않으면 sidecar wrapper, git diff validator, command output capture, detached verify, worktree isolation으로 보완한다.

## 11. GitHub Copilot 통합 기준

Copilot 통합은 VS Code 또는 Copilot agent 계열 표면을 사용할 수 있다.

목표는 다음이다.

- workspace custom instructions
- Harness custom agent 또는 playbook
- Harness Skill
- Harness MCP server
- VS Code task/terminal integration 또는 sidecar adapter
- 가능한 범위의 tool restriction과 status card 표시

Target profile 예시는 다음이다.

```yaml
surface_type: copilot
target_profile: vscode_chat | vscode_agent | cloud_agent | custom
```

Copilot 통합에서 guard는 custom agent tool set 제한, MCP policy check, VS Code task/terminal wrapper, sidecar file watcher, validator, detached verify로 보완한다.

## 12. Cursor 통합 기준

Cursor 통합은 rules, Skill, MCP를 중심으로 한다.

목표는 다음이다.

- project rules로 짧은 기본 규칙 제공
- Harness Skill로 절차 제공
- Harness MCP server로 상태와 tool 제공
- Cursor Agent 또는 Cursor CLI에서 공통 동작
- hook 또는 sidecar로 변경 감시와 guard 보완

Target profile 예시는 다음이다.

```yaml
surface_type: cursor
target_profile: ide_agent | local_cli | custom
```

Cursor 통합에서 guard는 rules, Skill, MCP prepare_write, hook, sidecar file watcher, changed_paths validator, detached verify로 보완한다.

## 13. Surface 선택 기준

하네스는 표면을 사용자가 매번 고르게 만들지 않는다.

기본 surface 선택은 하네스와 adapter가 결정한다.

| 작업 | 권장 surface 선택 |
|---|---|
| 상태 확인 | 현재 사용 중인 agent surface |
| advisor | 현재 사용 중인 agent surface |
| small direct | 현재 사용 중인 agent surface |
| medium work implement | 연결된 기본 구현 surface |
| high-risk work implement | T4 이상 guard 또는 sidecar-controlled run |
| detached verify | 구현 run과 다른 fresh context 또는 fresh worktree evaluator |
| approval UX | 현재 사용자 대화 표면 |
| acceptance UX | 현재 사용자 대화 표면 |

사용자는 특별한 이유가 없으면 surface를 지정하지 않는다.

## 14. Fallback 원칙

### 14.1 MCP가 연결되지 않은 경우

에이전트는 사용자가 CLI를 조합하게 만들지 않고, 연결 진단이 필요하다고 알려야 한다.

운영자 명령은 다음 정도로 안내한다.

```bash
harness doctor agents
harness connect agents --auto
```

### 14.2 Skill은 있으나 MCP가 없는 경우

Skill만으로는 하네스 상태를 authoritative하게 조작할 수 없다.

이 경우 에이전트는 코드 변경을 보류하고 연결을 요청한다.

### 14.3 MCP는 있으나 guard가 없는 경우

low-risk direct는 진행할 수 있다.

medium/high-risk work는 다음 보완책을 요구한다.

- sidecar 실행
- worktree isolation
- stricter validator
- explicit approval
- detached verify

### 14.4 projection이 stale인 경우

에이전트는 state 기준 상태와 projection 최신성을 분리해서 보여준다.

```text
state는 최신입니다.
TASK projection은 stale입니다.
reconcile 또는 projection refresh가 필요합니다.
```

### 14.5 evidence가 부족한 경우

에이전트는 어떤 acceptance criterion에 어떤 증거가 부족한지 보여줘야 한다.

```text
검증에 필요한 증거가 부족합니다.
AC-02는 supporting evidence가 없습니다.
```

### 14.6 capability가 부족한 경우

에이전트는 제품명 대신 부족한 capability를 말한다.

```text
현재 surface profile은 pre-tool guard를 제공하지 않습니다.
이 작업은 sidecar guard 또는 다른 evaluator surface가 필요합니다.
```

## 15. 통합 conformance

각 표면은 최소한 다음 시나리오를 통과해야 한다.

1. 상태 조회: active Task 없음 / 있음 모두 처리
2. intake: 새 요청을 advisor/direct/work로 분류
3. direct: 작은 변경 후 record_change와 finish_direct 수행
4. escalation: direct가 커졌을 때 같은 Task를 work로 전환
5. approval: dependency_change 요청 생성, 승인, 거절 처리
6. work implement: prepare_write, record_change, evidence manifest, run summary 생성
7. verify: fresh evaluator 또는 verify request 생성
8. same-session verify guard: self-review를 detached verify로 인정하지 않음
9. resume: Task와 최신 report 기반으로 다음 행동 제시
10. stale projection: state와 projection freshness를 구분
11. reconcile: human-editable proposal을 pending reconcile로 감지
12. generated drift: connector managed file 수동 수정 감지
13. capability fallback: 부족한 tier에서 sidecar 또는 다른 surface 권고
14. MCP unavailable: product file write 보류

## 16. 표준 사용자 문구

각 표면은 사용자에게 비슷한 어휘를 사용해야 한다.

| 상황 | 문구 |
|---|---|
| 시작 | 하네스로 작업을 시작합니다. |
| approval | 승인이 필요합니다. |
| evidence 부족 | 검증에 필요한 증거가 부족합니다. |
| verify 필요 | work 작업이므로 detached verify가 필요합니다. |
| acceptance 필요 | 기술 검증은 통과했지만 사용자 수용 판단이 남아 있습니다. |
| projection stale | state는 최신이지만 문서 projection은 stale입니다. |
| reconcile 필요 | 문서에 직접 수정된 항목이 있어 reconcile이 필요합니다. |
| 완료 | 작업을 닫았습니다. 최신 상태와 report는 다음과 같습니다. |

## 17. 구현 우선순위

### 17.1 MVP 통합 범위

MVP 통합은 하나의 reference surface를 깊게 연결한다.

필수 조건은 다음이다.

- T2 MCP 연결
- 제품 파일 쓰기 전 `prepare_write` 호출
- 변경 후 `record_change` 호출
- evidence manifest 갱신
- work 후 verify bundle 생성
- same-session verify guard 동작
- compact status card 표시

### 17.2 v1 stable 통합 범위

v1 stable은 두 번째 surface를 추가하고, capability 차이와 fallback을 검증한다.

필수 조건은 다음이다.

- connector registry 안정화
- generated file drift repair
- sidecar guard 보완
- worktree verify
- 표면별 conformance 자동화

### 17.3 v1.5 통합 범위

v1.5는 cross-surface verify와 UX 개선을 다룬다.

## 18. 요약

하네스 통합은 CLI 명령을 에이전트에게 그대로 넘기는 것이 아니다.

하네스 통합의 핵심은 다음이다.

- 항상 읽히는 규칙은 짧게 둔다.
- 절차는 Harness Skill로 둔다.
- 상태 조작은 Harness MCP로 한다.
- 정책 집행은 hook, adapter, sidecar, validator가 한다.
- 표면별 capability 차이를 profile로 선언한다.
- 부족한 capability는 fallback으로 보완한다.
- 사용자는 대화로 요청, 승인, 검증, 수용을 처리한다.

Codex, Claude Code, Gemini, GitHub Copilot, Cursor는 하네스의 source-of-truth가 아니라 하네스를 조작하는 agent surface다.
