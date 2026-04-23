# Reference Surface Spec v04

## 1. 문서 목적

이 문서는 MVP 참조 구현에서 사용할 단일 agent surface인 `reference_local_mcp`의 구현 계약을 정의한다.

이 문서는 다음을 다룬다.

- reference surface의 역할
- capability profile
- 생성 파일
- MCP 연결 방식
- sidecar와 command wrapper
- write guard와 artifact capture
- detached verification 방식
- smoke conformance 기준

이 문서는 Codex, Claude Code, Gemini, GitHub Copilot, Cursor의 실제 제품별 설정 위치를 확정하지 않는다.

## 2. Reference surface의 역할

`reference_local_mcp`는 특정 제품 기능에 의존하지 않고 하네스 MVP core invariant를 검증하기 위한 표면이다.

이 표면은 다음을 입증한다.

- 에이전트가 MCP tool로 하네스 상태를 조작한다.
- 제품 파일 쓰기 전 `harness.prepare_write`가 호출된다.
- 변경 후 `harness.record_change`와 evidence manifest 갱신이 수행된다.
- `work`는 fresh 또는 manual verification bundle 없이 닫히지 않는다.
- 같은 세션 self-review는 detached verification으로 기록되지 않는다.
- generated rule/Skill과 projection drift를 감지한다.

## 3. Surface identity

```yaml
surface_id: reference_local_mcp
surface_type: reference
target_profile: local_mcp_sidecar
support_tier: T3
capability_profile_version: 1
capabilities:
  project_rules: true
  skills: true
  mcp_tools: true
  mcp_resources: true
  mcp_prompts: true
  structured_output: true
  hooks: false
  pre_tool_guard: via_mcp_and_wrapper
  explicit_permissions: false
  fresh_verify: manual_bundle_or_spawned_fresh_session
  worktree_isolation: via_sidecar_optional
  subagents: false
  local_sidecar: true
guarantee_level:
  default: detective
  command_wrapper: preventive_for_wrapped_commands
  file_watcher: detective
  fresh_worktree: isolated_when_enabled
```

`reference_local_mcp`는 native product hook을 전제하지 않는다. 따라서 사전 차단은 MCP `prepare_write`와 command wrapper가 가능한 범위에서 담당하고, file watcher와 validator가 detective guard를 보완한다.

## 4. 생성 파일

Reference connector는 저장소에 다음 파일을 생성한다.

```text
repo/
  AGENTS.md
  .harness/
    agent/
      reference-local-mcp/
        SKILL.md
        mcp.json
        surface-profile.json
        command-wrapper.json
        evaluator-instructions.md
      generated/
        connector-manifest.json
      surface-status.json
```

### 4.1 `AGENTS.md`

저장소 공통 규칙만 담는다.

필수 내용:

- Harness 상태를 먼저 확인한다.
- 제품 파일 쓰기 전 Harness MCP를 사용한다.
- 민감 범주는 approval 없이 진행하지 않는다.
- work는 detached verify 없이 닫지 않는다.
- 대화보다 Harness state와 evidence를 우선한다.

### 4.2 `SKILL.md`

하네스 작업 절차를 담는다.

필수 MCP tool flow:

- status 또는 intake
- next
- prepare_write
- request_approval when needed
- record_change
- update_evidence_manifest
- finish_direct 또는 finish_implementation
- launch_verify 또는 record_eval
- close_task

### 4.3 `mcp.json`

참조 MCP 연결은 local stdio를 기본으로 한다.

```json
{
  "servers": {
    "harness": {
      "command": "harness",
      "args": ["serve", "mcp", "--stdio"],
      "env": {
        "HARNESS_PROJECT": "auto"
      }
    }
  }
}
```

실제 adapter는 target client 형식에 맞춰 이 내용을 변환할 수 있다.

### 4.4 `surface-profile.json`

Capability profile을 JSON으로 저장한다.

필수 필드:

```json
{
  "surface_id": "reference_local_mcp",
  "surface_type": "reference",
  "target_profile": "local_mcp_sidecar",
  "support_tier": "T3",
  "capabilities": {
    "project_rules": true,
    "skills": true,
    "mcp_tools": true,
    "mcp_resources": true,
    "mcp_prompts": true,
    "structured_output": true,
    "hooks": false,
    "pre_tool_guard": "via_mcp_and_wrapper",
    "fresh_verify": "manual_bundle_or_spawned_fresh_session",
    "local_sidecar": true
  }
}
```

### 4.5 `command-wrapper.json`

Command wrapper 설정은 다음을 가진다.

```json
{
  "default_mode": "detective",
  "wrapped_commands": ["pnpm", "npm", "yarn", "git", "node", "python"],
  "capture_stdout": true,
  "capture_stderr": true,
  "redact_secrets": true,
  "require_prepare_write_for_modifying_commands": true
}
```

## 5. MCP 연결

Reference surface는 다음 MCP resource를 읽을 수 있어야 한다.

- `harness://project/current`
- `harness://task/active`
- `harness://status/card`
- `harness://policy/sensitive-categories`

Reference surface는 다음 MCP tool을 호출할 수 있어야 한다.

- `harness.status`
- `harness.intake`
- `harness.next`
- `harness.prepare_write`
- `harness.request_approval`
- `harness.user_decision`
- `harness.record_change`
- `harness.update_evidence_manifest`
- `harness.finish_direct`
- `harness.finish_implementation`
- `harness.launch_verify`
- `harness.record_eval`
- `harness.close_task`

MCP가 연결되지 않으면 product file write를 보류한다.

## 6. Sidecar

Reference sidecar는 다음 기능을 제공한다.

- MCP server process supervision
- filesystem watch
- git diff sampling
- command output capture
- projection stale detection
- generated file drift detection
- optional fresh worktree setup
- artifact registration helper

Sidecar는 다음 모드로 동작한다.

| mode | 의미 |
|---|---|
| `off` | sidecar 없음, MCP와 validators만 사용 |
| `watch` | file watcher와 projection drift 감지 |
| `wrap` | command wrapper로 modifying command를 감시 또는 차단 |
| `isolate` | fresh worktree 또는 sandbox를 사용한 verify 지원 |

MVP 기본값은 `watch`다. high risk work는 `wrap` 또는 `isolate`를 요구할 수 있다.

## 7. Write guard

Reference surface의 write guard는 세 계층으로 동작한다.

### 7.1 MCP guard

에이전트는 제품 파일 쓰기 전 `harness.prepare_write`를 호출해야 한다.

`prepare_write`는 다음을 반환한다.

- allowed 여부
- required approval ids
- baseline ref
- allowed paths
- allowed tools
- validator profile

### 7.2 Wrapper guard

Wrapper를 통해 실행되는 modifying command는 다음을 확인한다.

- active run 존재
- `prepare_write` 성공 기록
- command가 allowed tools에 포함됨
- approval scope 안에 있음

### 7.3 Detective guard

File watcher와 git diff validator는 사후 위반을 감지한다.

위반 시 처리:

- 관련 run을 blocked 또는 interrupted로 표시
- `changed_paths` validator failure 기록
- projection에 risk와 next action 표시

## 8. Artifact capture

Reference surface는 다음 artifact를 capture해야 한다.

- baseline
- changed files
- diff
- command result
- stdout/stderr logs
- checkpoint
- run summary
- evidence manifest
- verification bundle
- eval result

Capture 우선순위:

1. structured output
2. command wrapper output
3. sidecar log capture
4. git diff sampling
5. manual MCP `record_change` payload

## 9. Detached verification

Reference surface는 두 가지 verification 방식을 지원한다.

### 9.1 Manual bundle verify

`harness.launch_verify`가 verification bundle과 evaluator instruction을 생성한다.

사용자 또는 별도 evaluator는 bundle을 읽고 검증한 뒤 `harness.record_eval`을 호출한다.

Manual bundle verify는 source input이 bundle이어야 하며, lead long chat history를 primary evidence로 사용하지 않는다.

### 9.2 Fresh session verify

가능하면 하네스가 별도 process 또는 별도 session을 시작한다.

Fresh session verify 조건:

- parent run id와 verify run id가 다름
- evaluator는 write-capable이 아님
- product file write allowed가 false
- baseline을 재확인함
- evidence manifest를 독립적으로 검토함

Optional fresh worktree verify는 sidecar `isolate` mode에서 지원한다.

## 10. 설치와 진단

설치:

```bash
harness connect agents --surfaces reference
```

진단:

```bash
harness doctor agents --surface reference_local_mcp
```

진단은 다음을 확인한다.

- project registration
- MCP server launch
- MCP resources read
- MCP tools call dry-run
- `AGENTS.md` generated block
- `SKILL.md` generated block
- surface profile
- sidecar status
- command wrapper availability
- projection freshness
- conformance smoke test

## 11. Smoke conformance

Reference surface는 최소 다음 smoke test를 통과해야 한다.

| ID | 시나리오 | 기대 결과 |
|---|---|---|
| RS-01 | active task 없음 상태 조회 | compact status card 반환 |
| RS-02 | small direct intake | mode=direct, phase=ready |
| RS-03 | prepare_write without approval category | allowed=true, baseline_ref 생성 |
| RS-04 | dependency_change prepare_write | APPROVAL_REQUIRED |
| RS-05 | approval granted 후 write | allowed=true |
| RS-06 | record_change | diff/log artifact 등록 |
| RS-07 | finish_direct | DIRECT-RESULT 생성, self_checked |
| RS-08 | work finish implementation | RUN-SUMMARY 생성, phase=verifying |
| RS-09 | same-session record_eval detached | VERIFY_NOT_DETACHED |
| RS-10 | launch_verify manual_bundle | BND artifact 생성 |
| RS-11 | projection stale | state current와 projection stale 분리 표시 |
| RS-12 | managed file drift | reconcile item 생성 |

## 12. 제한 사항

Reference surface는 특정 제품의 native hook을 대체하지 않는다.

따라서 다음 한계를 명시한다.

- 모든 파일 쓰기를 사전에 차단하지 못할 수 있다.
- wrapper 밖에서 실행된 command는 detective guard에 의존한다.
- manual bundle verify는 evaluator discipline이 필요하다.
- high risk 작업은 sidecar isolate 또는 더 높은 tier surface를 요구할 수 있다.

이 한계는 상태 카드의 risk와 assurance에 반영한다.

## 13. 완료 기준

Reference surface 구현은 다음이 가능해야 한다.

- 자연어 요청을 받아 `harness.intake`로 Task 생성
- `harness.prepare_write`를 통한 write preflight
- approval required scenario 차단
- artifact capture와 evidence manifest 갱신
- direct 작업 완료 기록
- work 구현 후 verification pending 전환
- verification bundle 생성
- same-session self-review 차단
- projection freshness 표시
- generated drift 감지
- conformance smoke test 통과
