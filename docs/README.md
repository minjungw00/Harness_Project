# 하네스 문서 세트 v04

## 1. 문서 세트의 목적

이 문서 세트는 AI와 함께 개발할 때 작업 상태, 작업 범위, 사용자 판단, 검증 근거가 계속 읽히도록 만드는 하네스의 기준과 구현 계약을 정의한다.

하네스는 특정 IDE, 특정 모델, 특정 CLI에 묶인 기능 묶음이 아니다. 하네스는 사용자가 AI 작업을 읽고, 제한하고, 검증하고, 다시 이어받을 수 있게 하는 운영 구조다.

## 2. 문서 층위

각 문서는 서로 다른 층위를 가진다. 한 문서가 다른 문서의 책임을 대신하지 않는다.

| 층위 | 문서 | 역할 | 다루지 않는 것 |
|---|---|---|---|
| 전략 | `strategy/harness-strategy.md` | 문제, 목표, 설계 입장, 핵심 불변식 | DB, API, 파일 경로, 제품별 설정 |
| 구현 목표 | `implementation/build-spec.md` | MVP 참조 구현의 닫힌 범위와 기술 선택 | 하네스의 상위 목표 재정의 |
| 구현 구조 | `implementation/harness-implementation-guide.md` | 서비스 경계, 트랜잭션, 운영 흐름 | 템플릿 전문, 사용자 예시 전체 |
| 상태 계약 | `contracts/state-schema.sql`, `contracts/state-machine.yaml` | 저장 스키마와 상태 전이 규칙 | 사용자 설명문, 표면별 문구 |
| MCP 계약 | `contracts/mcp-api.schema.json` | MCP resource, prompt, tool schema | 내부 서비스 구현 방식 |
| 검증 계약 | `contracts/validator-spec.md` | validator 입력, 판정, 차단 효과 | UI 문구, 템플릿 전문 |
| 증거 계약 | `contracts/artifact-contract.md` | artifact, bundle, hash, retention | 상태 전이 세부 |
| projection 계약 | `contracts/projection-contract.md` | Markdown projection과 reconcile 알고리즘 | 전략 원칙, DB DDL |
| 문서 템플릿 | `templates/harness-document-templates.md` | 사람이 읽는 문서 양식 | authoritative state 정의 |
| 통합 | `integration/harness-agent-integration-guide.md` | agent surface 연결 기준 | 코어 DB, 사용자 절차 전체 |
| 참조 표면 | `integration/reference-surface-spec.md` | MVP용 reference surface 구현 계약 | 모든 제품별 connector 완성 |
| 사용자 | `user/harness-user-guide.md` | 사용자가 대화로 하네스를 쓰는 방법 | 내부 API와 스키마 |
| conformance | `conformance/conformance-tests.md`, `conformance/conformance-fixtures.yaml` | 구현체가 지켜야 할 테스트 시나리오 | 성능 벤치마크, 제품별 E2E 전체 |

## 3. 읽는 순서

### 3.1 구현 에이전트

1. `strategy/harness-strategy.md`
2. `implementation/build-spec.md`
3. `contracts/state-schema.sql`
4. `contracts/state-machine.yaml`
5. `contracts/mcp-api.schema.json`
6. `implementation/harness-implementation-guide.md`
7. `contracts/validator-spec.md`
8. `contracts/artifact-contract.md`
9. `contracts/projection-contract.md`
10. `integration/reference-surface-spec.md`
11. `conformance/conformance-tests.md`

구현 중 문서에 없는 결정을 해야 할 경우, core invariant를 약화하지 않는 선에서 `ASSUMPTIONS.md`에 결정과 근거를 남긴다.

### 3.2 사용자

1. `user/harness-user-guide.md`
2. 필요한 경우 `strategy/harness-strategy.md`
3. 작업 문서를 직접 읽어야 할 때 `templates/harness-document-templates.md`

### 3.3 adapter 또는 connector 작성자

1. `integration/harness-agent-integration-guide.md`
2. `integration/reference-surface-spec.md`
3. `contracts/mcp-api.schema.json`
4. `contracts/projection-contract.md`
5. `conformance/conformance-tests.md`

## 4. source-of-truth 요약

하네스는 같은 사실의 기준 원본을 하나로 유지한다.

| 정보 | authoritative source |
|---|---|
| 프로젝트 등록과 surface 연결 | `registry.sqlite` |
| 정적 프로젝트 설정 | `project.yaml` |
| 현재 운영 상태 | `state.sqlite` |
| 상태 변경 이력 | append-only event log |
| 작업의 현재 의도와 연속 맥락 | `TASK` 문서의 managed projection과 DB 상태의 결합 |
| 결정 근거 | `DEC` 문서 |
| 승인 요청의 사람용 요약 | `APR` 문서 |
| acceptance criteria와 evidence 대응 | `EVIDENCE-MANIFEST` |
| raw evidence | artifact store |
| 사용자 대화 | 조작 표면이며 authoritative source가 아님 |
| rule, Skill, connector 파일 | generated projection |

## 5. 핵심 구현 기준

구현체는 최소한 다음 문장을 강제해야 한다.

```text
제품 파일 쓰기 전 scope와 approval을 확인한다.
변경 후 durable evidence를 남긴다.
work 작업은 실행자의 자기 보고만으로 닫지 않는다.
```

이 세 기준을 instruction이나 prompt만으로 만족했다고 보지 않는다. 상태 전이, MCP tool, validator, artifact registry, projection, sidecar 또는 adapter가 함께 집행해야 한다.

## 6. 문서 개정 원칙

문서 본문은 현재 기준을 직접 서술한다. 작성 경위, 이전 초안과의 비교, 폐기된 안의 설명은 본문에 섞지 않는다.

전략, 구현, 계약, 템플릿, 통합, 사용자 가이드는 독립적으로 고칠 수 있지만, 한 층위의 변경이 다른 층위의 계약을 바꾸는 경우 관련 문서를 함께 갱신한다.
