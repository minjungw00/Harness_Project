# 06. Agent Integration Guide

## 1. 문서 역할

이 문서는 하네스를 Codex, Claude Code, Gemini, GitHub Copilot, Cursor 같은 agent surface와 연결하는 기준을 정의한다. 공통 통합 계약, capability profile, generated file 관리, push/pull context, fallback, conformance, surface별 차이를 소유한다.

사용자는 이 문서를 읽지 않아도 하네스를 사용할 수 있어야 한다.

## 2. 통합 목표

하네스 통합의 목표는 사용자가 에이전트와 대화하는 것만으로 하네스 작업 구조를 얻는 것이다.

사용자가 직접 조합하지 않는 내부 요소는 다음이다.

- Task 생성과 재개
- mode 분류
- shared design shaping
- Change Unit 작성
- vertical slice 판정
- scope와 acceptance criteria 정리
- domain language와 module impact 확인
- approval request와 decision
- TDD trace와 evidence 기록
- detached verify launch
- manual QA 기록
- acceptance 기록
- projection 갱신
- reconcile 처리

Agent surface는 사용자의 자연어 요청을 받고, Harness Skill과 Harness MCP를 사용해 내부 요소를 처리한다.

## 3. 공통 통합 구조

```text
사용자 대화
  ↓
짧은 항상-on rule/context
  ↓
Harness Skill 또는 playbook
  ↓
Harness MCP server
  ↓
Harness Core
  ↓
adapter / hook / sidecar / validator
```

### 3.1 Rule/context

항상 읽히는 파일은 짧은 원칙만 담는다.

예시는 다음이다.

- `AGENTS.md`
- `CLAUDE.md`
- `GEMINI.md`
- GitHub Copilot custom instructions
- Cursor rules

이 층에는 긴 절차, 상태 전이, validator 설명을 넣지 않는다.

### 3.2 Harness Skill

Skill은 에이전트가 다음을 알게 한다.

- 언제 하네스를 사용하는가
- status와 intake를 어떻게 시작하는가
- advisor/direct/work를 어떻게 구분하는가
- work shaping에서 어떤 질문을 하는가
- domain language와 module map을 언제 확인하는가
- Change Unit을 vertical slice로 어떻게 나누는가
- approval이 필요하면 무엇을 하는가
- TDD trace와 evidence를 어떻게 기록하는가
- work 후 detached verify가 왜 필요한가
- manual QA와 acceptance를 어떻게 표시하는가

Skill은 절차 안내다. 정책 집행은 Core와 validator가 수행한다.

### 3.3 Harness MCP server

MCP server는 하네스 상태와 도구를 조작하는 기본 API다.

공개 tool은 high-level intent로 유지한다.

```text
harness.status
harness.intake
harness.next
harness.prepare_write
harness.record_run
harness.request_user_decision
harness.record_user_decision
harness.launch_verify
harness.record_eval
harness.record_manual_qa
harness.close_task
```

### 3.4 Adapter / hook / sidecar

이 계층은 guard와 capture를 보완한다.

- allowed path guard
- approval scope guard
- artifact capture
- command output capture
- same-session verify guard
- evaluator read-only guard
- projection freshness detection
- generated file drift detection
- vertical slice shape detection
- public interface change detection
- manual QA artifact capture 보조

표면이 native hook을 제공하지 않으면 sidecar와 validator가 보완한다.

## 4. Capability tier

| Tier | 의미 | 필요한 기능 |
|---|---|---|
| T0 Context | 하네스 원칙을 읽을 수 있음 | rules/context file |
| T1 Skill | 절차를 호출할 수 있음 | Skill, custom command, prompt file |
| T2 MCP | 상태와 도구를 조작할 수 있음 | MCP server 연결 |
| T3 Capture | diff, logs, run output을 안정적으로 회수할 수 있음 | structured output, CLI wrapper, adapter |
| T4 Guard | 파일, 명령, network, approval 범위를 사전에 막을 수 있음 | hook, permission, policy engine, sidecar |
| T5 Isolation | verify 또는 risky run을 분리 환경에서 실행할 수 있음 | worktree, sandbox, fresh run launcher |
| T6 QA Capture | browser/screenshot/manual QA artifact를 구조화해 남길 수 있음 | browser runner, screenshot capture, QA note capture |

일상 하네스 사용은 T2 이상에서 자연스럽다. Work의 신뢰 가능한 detached verify는 T3 이상을 요구한다. 고위험 작업은 T4 또는 T5가 필요하다. UI/UX 작업의 manual QA 자동 캡처는 T6가 있으면 품질이 좋아지며, 사람이 작성한 QA note로도 시작할 수 있다.

## 5. Surface capability profile

하네스는 제품명만으로 capability를 단정하지 않는다.

```yaml
surface_id: SURF-0001
surface_type: claude_code
target_profile: local_cli
detected_version: optional string
capability_profile_version: 4
last_verified_at: 2026-05-06T10:05:00+09:00
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
  local_sidecar: true
  browser_qa_capture: false
  screenshot_capture: false
risks:
  - version-dependent hook behavior
fallbacks:
  - sidecar file watcher
  - changed_paths validator
  - fresh worktree evaluator
  - manual QA note artifact
```

Target profile 예시는 다음이다.

- `local_cli`
- `ide_chat`
- `ide_agent`
- `cloud_agent`
- `extension`
- `custom_agent`
- `manual_bundle`

Capability는 제품 또는 extension 버전 변경, MCP 설정 변경, hook/permission 변경, workspace policy 변경, generated drift, conformance 실패, QA capture 방식 변경 시 재검증한다.

## 6. Push / pull context 원칙

구현 에이전트에는 긴 기준을 기본 context에 넣지 않는다. 필요한 자료를 MCP resource나 artifact ref로 pull하게 둔다.

구현 에이전트가 pull하는 자료:

- coding standards
- domain language
- module map
- interface contracts
- TDD playbook
- architecture playbook
- old PRD/DESIGN/closed issue

Reviewer/evaluator에는 관련 기준을 push한다.

- acceptance criteria snapshot
- allowed paths
- changed files
- approval scope
- relevant coding standards
- relevant domain language
- module/interface contract
- forbidden patterns
- evidence manifest
- TDD trace
- manual QA requirement

항상 push하지 않는 자료:

- 완료된 PRD
- 오래된 DESIGN
- closed issue
- 긴 과거 대화 요약
- raw logs
- large diff 전문

## 7. 공통 설치 wizard

기본 연결:

```bash
harness connect agents --auto
```

명시적 표면 선택:

```bash
harness connect agents --surfaces codex,claude,gemini,copilot,cursor
```

Wizard 수행 항목:

1. 저장소 루트를 확인한다.
2. 프로젝트 등록 상태를 확인한다.
3. Harness MCP server 실행 방법을 등록한다.
4. 표면별 rule/context 파일을 생성하거나 갱신한다.
5. 표면별 Skill 또는 playbook을 생성하거나 갱신한다.
6. 가능한 hook, adapter, sidecar 연결을 설정한다.
7. target profile과 capability를 진단한다.
8. domain-language, module-map, interface-contract projection을 준비한다.
9. conformance smoke test를 실행한다.
10. 사용자가 말할 수 있는 첫 대화 예시를 출력한다.

진단 명령:

```bash
harness doctor agents
```

## 8. 공통 생성물

### 8.1 AGENTS.md

저장소 루트에는 얇은 `AGENTS.md`를 둔다.

포함할 원칙:

- 하네스 상태를 먼저 확인한다.
- 작은 작업은 direct로 처리한다.
- 큰 작업은 work로 처리한다.
- Work는 shared design 질문부터 시작한다.
- 기능 Change Unit은 vertical slice를 기본값으로 둔다.
- 제품 파일 쓰기 전 Harness MCP를 사용한다.
- 민감 범주는 approval 없이 진행하지 않는다.
- 가능한 경우 TDD trace를 남긴다.
- Work는 detached verify 없이 닫지 않는다.
- Manual QA가 필요한 작업은 QA 상태를 남긴다.
- 대화보다 하네스 상태와 증거를 우선한다.

### 8.2 Harness Skill

Skill description은 다음 상황에서 호출되도록 작성한다.

- 사용자가 코드 변경을 요청한다.
- 사용자가 작업 상태를 묻는다.
- 사용자가 멈춘 작업을 이어달라고 한다.
- 사용자가 승인 또는 거절을 말한다.
- 사용자가 검증, QA, 완료를 요청한다.
- 사용자가 설계 결정을 남겨달라고 한다.
- 사용자가 domain language, module map, TDD, vertical slice를 언급한다.

### 8.3 MCP 설정

각 표면은 Harness MCP server를 호출할 수 있어야 한다. 기본 권장은 local stdio다. Local HTTP도 profile로 허용할 수 있다.

### 8.4 상태 카드

모든 표면은 같은 compact card를 보여준다.

```text
TASK-XXXX 제목
상태: mode / phase
다음 행동:
사용자 판단:
리스크:
증거:
설계:
QA:
최신 보고:
```

### 8.5 generated file 관리

하네스가 생성한 파일과 managed block은 connector manifest에 기록한다.

```md
<!-- HARNESS:BEGIN managed -->
...
<!-- HARNESS:END managed -->
```

수동 수정은 덮어쓰기 전에 diff를 보여주거나 reconcile 대상으로 올린다.

## 9. 공통 동작 계약

모든 surface는 같은 의미의 절차를 따른다.

```text
1. rule/context를 읽는다.
2. Harness Skill을 사용한다.
3. harness.status 또는 harness.intake를 호출한다.
4. 하네스가 반환한 mode와 next action을 따른다.
5. work면 shared design, domain language, module impact를 확인한다.
6. Change Unit이 vertical slice인지 확인한다.
7. 코드 변경 전 harness.prepare_write를 호출한다.
8. 변경 후 harness.record_run을 호출한다.
9. work면 harness.launch_verify를 호출하거나 verify request를 만든다.
10. manual QA가 필요하면 harness.record_manual_qa를 사용한다.
11. 사용자 결정은 harness.record_user_decision으로 남긴다.
12. close는 harness.close_task가 completion 조건을 확인한 뒤 수행한다.
```

## 10. Surface별 addendum

Surface별 섹션은 공통 동작의 반복을 담지 않고 차이만 기록한다.

### 10.1 Codex

```yaml
surface_type: codex
target_profile: local_cli | ide_chat | custom
primary_strength:
  - AGENTS.md 기반 항상-on 규칙
  - code edit surface
  - MCP 사용 가능 profile에서 자연스러운 하네스 조작
common_fallbacks:
  - sidecar file watcher
  - changed_paths validator
  - manual verify bundle
```

Codex connector는 `AGENTS.md`, Harness Skill, MCP config, artifact capture adapter를 설치한다. Pre-tool guard가 약한 profile에서는 `prepare_write`, sidecar, changed_paths validator로 보완한다.

### 10.2 Claude Code

```yaml
surface_type: claude_code
target_profile: local_cli | ide_chat | custom
primary_strength:
  - CLAUDE.md와 Skill
  - hook 기반 guard 후보
  - fresh evaluator profile 구성 후보
common_fallbacks:
  - read-only evaluator profile
  - fresh worktree evaluator
  - stop hook report draft
```

Hook 지점:

| 지점 | 하네스 동작 |
|---|---|
| SessionStart | active Task와 status card 주입 |
| UserPromptSubmit | intake 유도 |
| PreToolUse | edit/write/bash/network/secret 접근 전 scope와 approval 검사 |
| PostToolUse | changed files, command output, logs artifact candidate 등록 |
| Stop | run summary 초안, verify/QA 필요 여부 표시 |
| PreCompact | Task summary와 artifact refs 저장 |

Evaluator profile은 read-only를 기본값으로 둔다.

### 10.3 Gemini

```yaml
surface_type: gemini
target_profile: local_cli | extension | ide_chat | custom
primary_strength:
  - extension 또는 prompt package
  - MCP 연결
  - sidecar 보완에 적합
common_fallbacks:
  - CLI wrapper
  - sidecar-controlled run
  - manual QA note artifact
```

Gemini connector는 extension context를 짧게 유지하고, 긴 기준은 MCP resource로 pull하게 구성한다.

### 10.4 GitHub Copilot

```yaml
surface_type: copilot
target_profile: vscode_chat | vscode_agent | cloud_agent | custom
primary_strength:
  - workspace custom instructions
  - VS Code task/terminal integration
  - MCP 연결 가능 profile
common_fallbacks:
  - VS Code task wrapper
  - sidecar adapter
  - explicit approval card
```

Copilot connector는 status card 표시, MCP tool 호출, approval/QA/acceptance 카드 표시를 우선한다.

### 10.5 Cursor

```yaml
surface_type: cursor
target_profile: ide_agent | local_cli | custom
primary_strength:
  - project rules
  - MCP 연결
  - agent workflow와 sidecar 보완
common_fallbacks:
  - sidecar changed file detection
  - generated file drift detection
  - manual verify bundle
```

Cursor connector는 project rule을 짧게 유지하고 Skill과 MCP로 절차를 수행한다.

## 11. Surface 선택 기준

사용자는 특별한 이유가 없으면 surface를 지정하지 않는다. 하네스와 adapter가 기본 surface를 선택한다.

| 작업 | 권장 선택 |
|---|---|
| 상태 확인 | 현재 대화 surface |
| advisor | 현재 대화 surface |
| small direct | 현재 대화 surface |
| work shaping | 현재 대화 surface |
| medium work implement | 기본 구현 surface |
| high-risk work implement | T4 이상 guard 또는 sidecar-controlled run |
| detached verify | 구현 run과 다른 fresh context 또는 fresh worktree evaluator |
| manual QA | 사용자 대화 surface + 가능한 QA capture surface |
| approval UX | 현재 사용자 대화 surface |
| acceptance UX | 현재 사용자 대화 surface |

## 12. Fallback 원칙

### MCP unavailable

Product file write를 보류하고 진단을 안내한다.

```text
Harness MCP 연결이 확인되지 않았습니다. 연결 진단이 필요합니다.
```

```bash
harness doctor agents
harness connect agents --auto
```

### Skill만 있고 MCP가 없음

Skill만으로는 state를 authoritative하게 조작할 수 없다. 코드 변경을 보류한다.

### MCP는 있으나 guard가 약함

Low-risk direct는 진행할 수 있다. Medium/high-risk work는 sidecar, worktree isolation, stricter validator, explicit approval, detached verify를 요구한다.

### Projection stale

```text
state는 최신입니다.
TASK projection은 stale입니다.
projection refresh 또는 reconcile이 필요합니다.
```

### Evidence 부족

```text
검증에 필요한 증거가 부족합니다.
AC-02는 supporting evidence가 없습니다.
```

### Design-quality 부족

```text
이 작업은 public interface 변경을 포함하지만 interface contract가 없습니다.
interface review가 필요합니다.
```

### Capability 부족

제품명 대신 부족한 capability를 말한다.

```text
현재 surface profile은 pre-tool guard를 제공하지 않습니다.
이 작업은 sidecar guard 또는 다른 evaluator surface가 필요합니다.
```

## 13. 통합 conformance

각 surface connector는 다음 시나리오를 통과한다.

1. status: active Task 없음/있음 처리
2. intake: advisor/direct/work 분류
3. work shaping: shared design 질문과 결정 기록
4. domain language: 새 용어 proposal과 reconcile 처리
5. Change Unit: vertical slice와 horizontal exception 처리
6. direct: 작은 변경 후 record_run과 DIRECT-RESULT 생성
7. escalation: direct가 커졌을 때 같은 Task를 work로 전환
8. approval: request, granted, denied 처리
9. work implement: prepare_write, record_run, evidence manifest 생성
10. TDD trace: red/green evidence 또는 waiver 처리
11. verify: fresh evaluator 또는 verify request 생성
12. same-session verify guard 동작
13. manual QA: QA required 작업에서 QA state 표시
14. resume: Task와 최신 report 기반으로 next action 제시
15. stale projection: state와 projection freshness 구분
16. reconcile: human-editable proposal 감지
17. generated drift: connector managed file 수정 감지
18. capability fallback: 부족한 tier에서 보완책 제시
19. MCP unavailable: product write 보류

## 14. 통합 문구 표준

| 상황 | 문구 |
|---|---|
| 시작 | `하네스로 작업을 시작합니다.` |
| shaping | `먼저 shared design을 맞추기 위해 질문하겠습니다.` |
| domain language | `이 작업은 다음 도메인 용어에 영향을 줍니다.` |
| vertical slice | `첫 Change Unit은 vertical slice로 제안합니다.` |
| approval | `승인이 필요합니다.` |
| TDD | `이 Change Unit은 TDD trace를 남깁니다.` |
| evidence 부족 | `검증에 필요한 증거가 부족합니다.` |
| verify 필요 | `work 작업이므로 detached verify가 필요합니다.` |
| manual QA 필요 | `사람이 확인해야 하는 QA가 남아 있습니다.` |
| acceptance 필요 | `기술 검증은 통과했지만 사용자 수용 판단이 남아 있습니다.` |
| projection stale | `state는 최신이지만 문서 projection은 stale입니다.` |
| reconcile 필요 | `문서에 직접 수정된 항목이 있어 reconcile이 필요합니다.` |
| 완료 | `작업을 닫았습니다. 최신 상태와 report는 다음과 같습니다.` |

## 15. 요약

```text
항상 읽히는 규칙은 짧게 둔다.
절차는 Harness Skill로 둔다.
상태 조작은 Harness MCP로 한다.
정책 집행은 Core, validator, hook, adapter, sidecar가 한다.
구현 에이전트는 기준을 필요할 때 pull한다.
리뷰/evaluator는 관련 기준을 push받는다.
표면별 capability 차이는 profile로 선언한다.
부족한 capability는 fallback으로 보완한다.
```

