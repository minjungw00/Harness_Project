# Glossary

## Acceptance

사용자가 결과와 남은 trade-off를 받아들이는 사후 판단이다. Approval, assurance, manual QA와 구분된다.

## Agent Surface

사용자가 대화하거나 에이전트가 코드를 읽고 수정하는 LLM 개발 도구 표면이다. Codex, Claude Code, Gemini, GitHub Copilot, Cursor 같은 도구가 여기에 속한다. Agent Surface는 하네스를 사용하는 조작 표면이다.

## Approval

민감한 변경을 진행해도 되는지에 대한 사전 사용자 결정이다. Scope-bound contract이며 허용 경로, 허용 도구, network target, secret scope, baseline, 만료 조건을 가진다.

## APR

Approval Request의 사람용 문서 projection이다. Approval의 목적, 범위, 영향, 리스크, 대안, 권장안, 사용자 결정을 기록한다.

## Architecture Drift

변경이 기존 module map, public interface, dependency direction, test boundary, domain language와 어긋나는 현상이다. Architecture drift는 코드베이스의 장기 변경 가능성을 악화시킬 수 있다.

## Artifact

재개, 판정, 추적, 감사에 사용하는 durable output이다. 예시는 diff, logs, checkpoint, bundle, manifest, run summary, eval report, TDD trace, manual QA record다. Raw evidence의 canonical source는 artifact store다.

## Assurance

기술적으로 결과가 어느 수준까지 확인되었는지를 나타낸다.

```text
none
self_checked
detached_verified
```

Assurance는 approval, manual QA, acceptance와 구분된다.

## Baseline

Write-capable run이 시작되기 전의 저장소 기준 상태다. 예시는 HEAD commit, worktree snapshot, uncommitted changes, branch, 파일 hash 요약, lockfile hash, config hash다.

## Bundle

Evaluator나 후속 run이 사용하는 구조화된 검증 입력 묶음이다. Task summary, rolling spine, acceptance criteria snapshot, changed files, baseline ref, run summary ref, evidence manifest ref, diff/log refs, approvals, decisions, design refs, TDD trace refs, manual QA refs를 포함할 수 있다.

## Change Unit

실제 구현 단위다. 목적, 비목표, slice type, end-to-end path, 허용 경로, 허용 도구, validator profile, TDD 요구, manual QA 요구, 완료 조건, evaluator focus, dependency relation을 가진다.

## Change Unit DAG

Change Unit 사이의 blocked_by, unblocks, parallelizable_with, merge_risk 관계다. 하네스는 병렬 실행을 기본값으로 삼지 않더라도 Change Unit을 DAG로 이해할 수 있게 표현한다.

## Connector

Agent Surface를 하네스에 연결하는 구성 요소다. Rule/context, Skill, MCP 설정, hook/adapter/sidecar 연결, capability profile, generated file manifest를 관리한다.

## DEC

Decision 문서다. 중요한 선택지, trade-off, 권장안, 최종 결정, 영향, 후속 작업을 기록한다.

## Deep Module

단순한 public interface 뒤에 충분한 기능과 내부 복잡성을 숨긴 모듈이다. Test boundary가 명확하고, 호출자가 내부 구현을 몰라도 사용할 수 있다.

## Detached Verification

구현 run과 분리된 검증 경계에서 수행하는 verification이다. Fresh session, fresh worktree, sandbox, manual bundle 기반 검증이 여기에 속한다. Same-session self-review는 detached verification으로 인정하지 않는다.

## Direct

작고 저위험이며 결과 확인이 분명한 변경을 빠르게 처리하는 작업 모드다. Direct 결과는 보통 `self_checked` assurance를 가진다. 범위가 커지면 같은 Task를 `work`로 전환한다.

## Domain Language

제품 도메인에서 사용하는 공통 용어와 의미의 사전이다. 코드, 테스트, 문서, 사람-AI 대화가 같은 용어를 같은 의미로 사용하도록 돕는다.

## DOMAIN-LANGUAGE

Domain Language의 사람용 문서 projection이다. 용어, 의미, 코드 표현, 혼동 금지 의미, 관련 모듈, 출처를 기록한다.

## EVAL

Verification 결과의 사람용 문서 projection이다. Verdict, assurance impact, verification independence, acceptance impact, checks performed, evidence reviewed, rationale, blockers, user follow-up을 기록한다.

## Evidence

검증과 재개에 사용할 수 있는 근거다. 예시는 test result, command log, diff, checkpoint, bundle, run summary, approval ref, TDD trace, manual QA screenshot이다.

## Evidence Manifest

Acceptance criteria와 evidence의 대응 관계를 기록하는 manifest다. 각 criterion은 supported, unsupported, not_applicable 중 하나로 표시된다.

## Feedback Loop

작업 결과를 빠르게 검증하는 장치다. 예시는 typecheck, lint, unit test, integration test, browser check, manual QA, runtime validation이다.

## Grill Protocol

AI가 사용자에게 한 번에 하나씩 질문하고, 각 질문에 추천안과 trade-off를 제시하며 shared design concept에 도달하는 shaping 절차다.

## Harness Core

Task, Change Unit, approval, evidence, artifact, verification, projection, recovery, design-quality record를 처리하는 하네스의 운영 로직이다.

## Harness Runtime Home

하네스 운영 상태와 artifact를 저장하는 로컬 홈이다. 기본 위치는 `~/.harness`다.

## Harness Server / Installation

Harness MCP server, Core, adapter, validator, connector, projector를 실행하는 설치물이다. 개발자는 별도 Git 리포지토리로 둘 수 있고, 일반 사용자는 패키지나 바이너리 설치물로 접할 수 있다.

## Horizontal Slice

DB, API, UI, 테스트처럼 한 층만 넓게 구현하는 작업 단위다. 기능 작업에서 horizontal slice는 exception reason과 후속 vertical slice를 가져야 한다.

## Human-editable 영역

사용자가 Markdown 문서에 직접 메모, 제안, 질문을 남길 수 있는 영역이다. 이 영역의 내용은 reconcile을 거쳐 운영 상태에 반영된다.

## Interface Contract

모듈이나 외부 boundary의 public interface, 입력, 출력, 오류, compatibility impact, test boundary를 기록하는 계약이다.

## INTERFACE-CONTRACT

Interface Contract의 사람용 문서 projection이다.

## Manual QA

사람이 실제 결과를 보고 UX, workflow, copy, visual quality, accessibility, taste, product fit을 확인하는 절차다. Automated verification, acceptance와 구분된다.

## Manual QA State

수동 QA 상태다.

```text
none
pending
passed
failed
waived
```

## MCP

Model Context Protocol. 하네스에서는 agent surface가 하네스 상태와 tool을 조작하는 기본 API 계층이다. MCP resource는 읽기 전용이고, 상태 변경은 MCP tool을 통해 수행한다.

## Module Map

코드베이스의 주요 모듈, 책임, public interface, dependency direction, test boundary, owner decision을 정리한 설계 지도다.

## MODULE-MAP

Module Map의 사람용 문서 projection이다.

## Product Repository

사용자의 실제 제품 코드와 사람이 읽는 하네스 projection 문서가 있는 저장소다.

## Projection

운영 상태와 artifact reference를 사람이 읽는 Markdown 문서로 투영한 결과다. Projection은 사용자 표면이다. 운영 상태 전이의 canonical source는 `state.sqlite`와 event log다.

## Public Interface

모듈 외부 호출자가 의존하는 API, function, class, route, command, event, schema, contract다. Public interface 변경은 module/interface review 대상이다.

## Pull Context

에이전트가 필요할 때 찾아 읽을 수 있게 둔 규칙, coding standard, domain language, playbook이다.

## Push Context

에이전트에게 항상 또는 특정 run 시작 시 주입하는 기준이다. Reviewer/evaluator에게는 coding standard, acceptance criteria, interface contract, forbidden pattern을 push하는 것이 기본값이다.

## Reconcile

사람이 문서를 수정하거나 generated file이 drift 되었을 때, 그 내용을 운영 상태에 반영할지 정리하는 절차다. 선택지는 merge, reject, convert-to-note, create-decision, defer가 될 수 있다.

## Red / Green / Refactor

TDD의 기본 루프다.

```text
red: 실패하는 테스트를 먼저 만든다.
green: 테스트를 통과하게 구현한다.
refactor: 동작을 유지하며 구조를 개선한다.
```

## Run

에이전트 또는 evaluator가 특정 Task/Change Unit을 수행한 실행 단위다. Lead run은 shaping/implementation을 수행하고, evaluator run은 기본적으로 read-only 검증을 수행한다.

## RUN-SUMMARY

Run이 남긴 변경, 검증, 이슈, spine update, evidence ref를 요약하는 문서 projection이다.

## Shallow Module

기능은 적고 interface나 의존성 노출이 많은 모듈이다. Shallow module이 많으면 코드 탐색과 테스트 경계 설정이 어려워진다.

## Shared Design Concept

사람과 AI가 함께 만들고자 하는 것에 대해 공유하는 설계 이해다. 질문/결정 로그, DESIGN, domain language, module map, Change Unit, acceptance criteria로 투영된다.

## Sidecar

Agent surface가 제공하지 않는 guard와 capture를 보완하는 표면 독립 운영 계층이다. Filesystem watch, git diff sampling, command wrapper, artifact capture, worktree setup, evaluator isolation 등을 수행할 수 있다.

## Skill

Agent surface가 하네스를 언제 어떻게 사용해야 하는지 알려주는 절차 안내 문서다. 정책 집행은 Core, validator, hook, sidecar가 수행한다.

## Source-of-truth

어떤 정보에 대해 기준이 되는 authoritative source다. 하네스에서는 운영 상태 전이의 canonical source가 `state.sqlite`와 event log이고, raw evidence의 canonical source가 artifact store다.

## Task

사용자 가치 단위다. 목적, 기대 결과, 범위, 비범위, acceptance criteria, 현재 요약, 다음 행동, 남은 사용자 판단, 관련 evidence와 decision을 가진다.

## Task Spine

Task continuity를 유지하는 요약 구조다.

```text
Current Summary
Rolling Spine
Snapshot References
```

## TDD Trace

특정 Change Unit에서 red → green → refactor가 어떻게 수행되었는지 기록한 evidence다. 테스트 파일, 실패 로그, 성공 로그, 리팩터링 노트, non-TDD justification을 포함할 수 있다.

## Tracer Bullet

실제 동작 경로를 얇게 끝까지 연결해 빠른 피드백을 얻는 구현 방식이다. 하네스에서는 Vertical Slice와 같은 방향의 개념으로 사용한다.

## Verification

결과가 acceptance criteria를 충족하는지 판정하는 과정이다. Work 작업은 가능한 한 detached verification을 거친다.

## Verification Independence

Verification이 얼마나 독립적인지 나타내는 qualifier다. 예시는 same_session, fresh_session, fresh_worktree, sandbox다.

## Vertical Slice

사용자 가치 또는 system-observable 결과를 끝까지 얇게 연결하는 구현 단위다. 예시는 UI input → API boundary → domain logic → persistence → 화면 표시 또는 event/log output이다.

## Work

기능 추가, 구조 변경, 비국소 수정, 리팩터링처럼 shaping, implementation, verification이 필요한 작업 모드다. Work는 detached verification 또는 그에 준하는 독립 검증 경계 없이 완료로 닫지 않는다.

