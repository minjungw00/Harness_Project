# 하네스 MVP 빌드 명세 v04

## 1. 문서 목적

이 문서는 하네스 MVP 참조 구현의 닫힌 구현 범위를 정의한다.

이 문서는 다음을 확정한다.

- MVP에서 구현할 기능
- MVP에서 제외할 기능
- 참조 구현의 기술 선택
- 패키지와 모듈 경계
- 구현 에이전트가 따라야 할 계약 문서
- conformance 통과 기준

이 문서는 하네스의 전략 원칙을 다시 정의하지 않는다.

## 2. 구현 목표

MVP 참조 구현의 목표는 agent surface를 많이 연결하는 것이 아니라 core invariant를 실제로 강제하는 것이다.

MVP는 다음 문장을 입증해야 한다.

```text
하네스는 제품 파일 쓰기 전 scope와 approval을 확인하고,
변경 후 evidence를 남기며,
work 작업을 실행자의 자기 보고만으로 닫지 못하게 한다.
```

## 3. 참조 구현 기술 선택

MVP 참조 구현은 다음 기술을 사용한다.

| 항목 | 선택 |
|---|---|
| 런타임 | Node.js 22 LTS 이상 |
| 언어 | TypeScript |
| 패키지 관리 | pnpm |
| 상태 저장 | SQLite |
| API 표면 | MCP server, local stdio 기본 |
| 보조 CLI | setup, doctor, serve, reconcile, recover, export, conformance |
| schema 검증 | JSON Schema 기반 validator, 구현체 내부에서는 schema adapter 사용 가능 |
| 문서 projection | Markdown + YAML front matter |
| artifact hash | SHA-256 |
| 참조 agent surface | `reference_local_mcp` |

기술 선택은 참조 구현의 선택이다. 하네스 코어 계약은 특정 언어나 특정 MCP SDK에 묶이지 않는다.

## 4. MVP 범위

MVP는 다음을 구현한다.

- 단일 로컬 프로젝트 등록
- 하나의 reference agent surface 연결
- Harness MCP server
- `registry.sqlite`와 `state.sqlite`
- append-only event log
- idempotent state-changing tool 처리
- projection outbox
- Markdown projector
- document reconcile 감지
- artifact registry
- baseline capture
- approval service
- evidence manifest service
- detached verification bundle 생성
- manual 또는 fresh evaluator instruction bundle
- 핵심 validator
- 최소 CLI
- conformance scenario 실행기

MVP가 생성하는 저장소 문서는 다음이다.

- `TASK`
- `APR`
- `DEC`
- `RUN-SUMMARY`
- `EVAL`
- `DIRECT-RESULT`
- `EVIDENCE-MANIFEST`
- `AGENTS.md`
- reference surface Skill/playbook

## 5. MVP 제외 범위

MVP는 다음을 구현하지 않는다.

- 모든 agent surface connector 완성
- cross-surface orchestration
- dashboard
- team profile export/import
- 복잡한 multi-agent policy
- 장기 trace analytics
- 완전 자동 병렬 실행
- 제품별 hook behavior의 버전별 자동 보장
- cloud agent remote execution 제어
- 조직 정책과 연동된 권한 관리

제외 범위는 core invariant를 약화하는 근거가 아니다.

## 6. 패키지 구조

참조 구현은 다음 패키지 구조를 사용한다.

```text
harness/
  package.json
  pnpm-workspace.yaml
  tsconfig.base.json
  packages/
    shared-schema/
    core/
    state-store/
    artifact-store/
    projector/
    validators/
    mcp-server/
    cli/
    sidecar/
    connectors/
      reference-local-mcp/
    conformance/
  templates/
    docs/
    agents/
  contracts/
    state-schema.sql
    state-machine.yaml
    mcp-api.schema.json
  docs/
    generated-from-this-set/
```

### 6.1 `shared-schema`

공통 enum, ID 규칙, JSON Schema loader, validation helper를 제공한다.

### 6.2 `state-store`

SQLite connection, migration, transaction, lock, idempotency table 처리를 담당한다.

### 6.3 `core`

Task state service, approval service, evidence service, run service, verification launcher, recovery service를 제공한다.

### 6.4 `artifact-store`

artifact 파일 저장, hash 계산, manifest 기록, retention class, rescan을 담당한다.

### 6.5 `projector`

projection outbox job을 처리하고 Markdown 문서를 생성한다.

### 6.6 `validators`

validator runner와 기본 validator를 제공한다.

### 6.7 `mcp-server`

MCP resource, prompt, tool을 노출한다.

### 6.8 `cli`

운영자용 명령을 제공한다. 일상 작업 조합 API가 아니다.

### 6.9 `sidecar`

filesystem watch, command wrapper, process supervision, stale detection, generated file drift detection을 제공한다.

### 6.10 `connectors/reference-local-mcp`

MVP 참조 surface의 generated files, profile, smoke test를 제공한다.

### 6.11 `conformance`

문서화된 시나리오를 상태 저장소, MCP tool, projection output 기준으로 검사한다.

## 7. 저장 위치

참조 구현은 다음 로컬 저장 위치를 사용한다.

```text
~/.harness/
  config.yaml
  registry.sqlite
  logs/
  projects/
    PRJ-0001/
      project.yaml
      state.sqlite
      artifacts/
        baselines/
        bundles/
        checkpoints/
        diffs/
        logs/
        manifests/
        reports/
        validator-output/
        exports/
```

저장소 표면에는 다음 구조를 만든다.

```text
repo/
  AGENTS.md
  docs/
    tasks/
      active/
      completed/
    decisions/
    approvals/
    reports/
      runs/
      evals/
      directs/
    evidence/
    design/
  .harness/
    agent/
      generated/
      reference-local-mcp/
      surface-status.json
    reconcile/
      pending/
```

상태 저장소와 artifact store는 운영 원본이다. 저장소 Markdown 문서는 사람이 읽는 projection이다.

## 8. 구현 계약 문서

구현체는 다음 계약을 따른다.

| 계약 | 문서 |
|---|---|
| 상태 저장 | `contracts/state-schema.sql` |
| 상태 전이 | `contracts/state-machine.yaml` |
| MCP API | `contracts/mcp-api.schema.json` |
| validator | `contracts/validator-spec.md` |
| artifact와 bundle | `contracts/artifact-contract.md` |
| projection과 reconcile | `contracts/projection-contract.md` |
| 문서 템플릿 | `templates/harness-document-templates.md` |
| reference surface | `integration/reference-surface-spec.md` |
| conformance | `conformance/conformance-tests.md` |

문서 간 충돌이 있으면 다음 우선순위를 적용한다.

1. 전략 문서의 core invariant
2. build spec의 MVP 범위
3. 상태 전이 계약
4. DB schema와 MCP schema
5. validator, artifact, projection 계약
6. 문서 템플릿
7. surface integration 문서

충돌은 `ASSUMPTIONS.md`로 조용히 덮지 않는다. 구현 중 발견하면 `SPEC-CONFLICTS.md`에 기록한다.

## 9. 상태 변경 처리 규칙

모든 state-changing operation은 다음 절차를 따른다.

1. input schema 검증
2. idempotency key 확인
3. project lock 획득
4. expected state version 확인
5. 상태 전이 guard 실행
6. current state 갱신
7. append-only event append
8. projection version 증가
9. projection job enqueue
10. idempotency result 저장
11. transaction commit
12. artifact 또는 projection 파일 쓰기

DB state와 파일 projection을 하나의 분산 트랜잭션처럼 묶지 않는다.

## 10. 최소 CLI

MVP CLI는 다음 명령을 제공한다.

```bash
harness init
harness connect agents --auto
harness doctor
harness doctor agents
harness serve mcp
harness reconcile
harness recover TASK-0001
harness export TASK-0001
harness conformance run
```

CLI는 일상 작업 조합 표면이 아니다. 에이전트는 가능하면 Harness MCP tool을 사용한다.

## 11. 기본 validator 구현 범위

MVP는 다음 validator를 구현한다.

- `changed_paths`
- `approval_scope`
- `evidence_sufficiency`
- `same_session_verify_guard`
- `docs_consistency`
- `required_docs_present`
- `baseline_freshness`
- `reconcile_required`

`lint`, `test`, `build`, `bundle_integrity`, `acceptance_review`, `surface_capability_check`는 MVP에서 stub 또는 basic implementation으로 시작할 수 있다. stub은 항상 pass로 처리하지 않고 `skipped` 또는 `warning`을 명시한다.

## 12. 완료 기준

MVP 참조 구현은 다음을 만족해야 한다.

- `harness.intake`가 advisor/direct/work를 분류한다.
- `harness.prepare_write`가 민감 범주와 scope 위반을 차단한다.
- approval 없는 dependency change가 차단된다.
- 승인 범위 밖 변경 시 approval이 만료되거나 재승인을 요구한다.
- `direct` 작업이 `DIRECT-RESULT`와 self_checked assurance를 남긴다.
- `direct`가 커질 때 같은 Task를 `work`로 전환한다.
- `work` 구현 완료 후 `RUN-SUMMARY`와 `EVIDENCE-MANIFEST`가 생성된다.
- `work`는 passed EVAL 없이 close되지 않는다.
- same-session self-review는 detached_verified로 기록되지 않는다.
- fresh verify bundle을 만들 수 있다.
- projection stale과 state current를 분리해 보여준다.
- 사람이 managed 영역을 수정하면 reconcile item을 만든다.
- conformance minimum scenarios가 통과한다.

## 13. 구현 에이전트 규칙

구현 에이전트는 다음을 지킨다.

- 문서에 없는 결정을 해야 하면 `ASSUMPTIONS.md`에 남긴다.
- core invariant를 약화하는 임의 결정은 하지 않는다.
- 모든 surface connector를 한 번에 구현하려 하지 않는다.
- Skill이나 rule 파일만 만들고 하네스 구현을 완료했다고 판단하지 않는다.
- source-of-truth를 Markdown 문서로 바꾸지 않는다.
- approval, assurance, acceptance를 하나의 상태로 합치지 않는다.
- same-session self-review를 detached verification으로 기록하지 않는다.
- conformance 실패를 문구 수정으로 우회하지 않는다.

## 14. 산출물 목록

구현 완료 시 저장소에는 다음 산출물이 있어야 한다.

```text
packages/*
contracts/state-schema.sql
contracts/state-machine.yaml
contracts/mcp-api.schema.json
templates/docs/*
templates/agents/*
README.md
ASSUMPTIONS.md
SPEC-CONFLICTS.md
CONFORMANCE-RESULTS.md
```

`ASSUMPTIONS.md`는 비어 있을 수 있다. 비어 있지 않다면 각 assumption은 관련 contract, 구현 영향, future resolution 조건을 가져야 한다.
