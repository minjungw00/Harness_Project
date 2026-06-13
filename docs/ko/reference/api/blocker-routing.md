# API 차단 사유 처리 경로

이 문서는 닫기 준비 상태 차단 사유 처리 경로와 공개 API 오류와 닫기 차단 사유의 경계를 담당합니다.

Core 닫기 준비 상태 권한, `CloseReadinessBlocker` 형태, `harness.close_task` 메서드 동작, 표시 문구, 저장 효과, API 응답 분기 경로는 정의하지 않습니다.

## 담당 경계

이 문서가 담당합니다.

- 닫기 준비 상태 차단 사유 처리 범주와 공개 오류 코드 묶음의 관계.
- `ToolRejectedResponse.errors[]` 공개 API 오류와 `CloseReadinessBlocker[]` 닫기 차단 사유 데이터 사이의 경계.
- 공개 `ErrorCode`를 `CloseReadinessBlocker.code`에 복사하지 않고 공개 오류 코드 묶음을 닫기 차단 사유로 표현하는 조건.
- 사전 확인 거부, 읽기 전용 닫기 확인, 차단된 닫기 시도, 닫힌 결과, 유효하지 않은 종료 전이에 대한 `harness.close_task` 차단 사유 매핑.
- 닫기 준비 상태 차단 사유 처리 경로가 거부 응답, 차단 결과, `dry_run` 미리보기와 연결되는 방식.

이 문서는 담당하지 않습니다.

- Core 권한, 닫기 준비 상태 의미, 최종 수락, 잔여 위험 수락, 대체 불가 규칙: [Core 모델의 닫기 준비 상태](../core-model.md#close_task)를 봅니다.
- `CloseReadinessBlocker` 형태, 필드, 정확한 `CloseReadinessBlocker.category` 값: [API 상태 스키마](schema-state.md)와 [API 값 집합](schema-value-sets.md#state-and-blocker-values)을 봅니다.
- `harness.close_task` 요청 동작, 닫기 준비 상태 평가 순서, 커밋된 차단 결과: [`harness.close_task`](method-close-task.md)를 봅니다.
- 거부 응답, 차단 결과, `dry_run` 응답 분기 경로: [API 오류 경로](error-routing.md)를 봅니다.
- 공개 `ErrorCode` 의미와 우선순위: [API 오류 코드](error-codes.md)와 [API 오류 우선순위](error-precedence.md)를 봅니다.
- 표시 문구로만 쓰는 표시 라벨과 렌더링 문구: [템플릿 본문](../template-bodies.md)을 봅니다.

## 닫기 준비 상태 차단 사유 처리 범주

정확한 `CloseReadinessBlocker.category` 값 이름은 [API 값 집합](schema-value-sets.md#state-and-blocker-values)이 담당합니다. 이 문서는 그 값을 닫기 준비 상태 발견 사항을 적용되는 담당 문서로 보내는 데만 사용합니다.

| 처리 경로 묶음 | `CloseReadinessBlocker.category` 값 | 담당 경계 |
|---|---|---|
| Core 상태와 전이 | `task`, `open_run`, `write_compatibility`, `baseline`, `recovery` | Core 상태, 열린 실행, 쓰기 호환성, 기준 상태, 복구 관련 차단 사유를 보냅니다. Core 의미는 [Core 모델](../core-model.md)이 담당합니다. |
| 범위와 권한 경계 | `scope`, `user_judgment`, `sensitive_approval`, `surface_capability` | 범위 경로, 사용자 소유 판단, 민감 동작 승인, 접점 역량 해결이 필요한 차단 사유를 보냅니다. |
| 증거와 아티팩트 근거 | `evidence`, `artifact_availability` | 증거 충분성이나 지속 아티팩트 가용성에 관한 차단 사유를 보냅니다. 증거와 아티팩트 의미는 각 담당 문서에 남습니다. |
| 수락과 잔여 위험 | `final_acceptance`, `residual_risk_visibility`, `residual_risk_acceptance` | 최종 수락, 보이는 잔여 위험, 잔여 위험 수락 관련 차단 사유를 보냅니다. 이 경로 자체가 수락을 만들지는 않습니다. |

## API 오류와 차단 사유 경계

| 상황 | 경로 | 경계 |
|---|---|---|
| 유효한 닫기 준비 상태 평가 전 실패 | `ToolRejectedResponse.errors[]`와 `ToolError.code: ErrorCode` | 요청이 유효한 닫기 준비 상태 결과에 도달하지 않았습니다. `CloseReadinessBlocker[]`를 반환하지 않습니다. |
| 유효한 닫기 준비 상태 평가에서 닫기 차단 사유 발견 | 메서드 결과 또는 읽기 전용 상태 결과의 `CloseReadinessBlocker[]` | 데이터는 닫기가 막힌 이유를 설명합니다. 공개 전송 오류나 스키마 거부가 아닙니다. |
| 유효한 `dry_run` 미리보기에서 차단 사유형 결과 예상 | `DryRunSummary.would_blockers: PlannedBlocker[]` | 미리보기 차단 사유는 저장된 `CloseReadinessBlocker` 객체가 아니며 닫기 준비 상태를 만들지 않습니다. |
| 응답 분기 선택이 질문인 경우 | [API 오류 경로](error-routing.md) | 이 문서는 응답 분기가 정해진 뒤의 차단 사유 의미를 다룹니다. |

## 금지된 공개 오류의 표현

공개 `ErrorCode` 값은 공개 API 식별자이지 차단 사유 코드가 아닙니다. 어떤 조건이 유효한 닫기 준비 상태 평가 중 발견되고, 적용되는 담당 문서가 그 조건에 대해 지원되는 차단 사유 범주나 차단 사유 코드를 정의할 때만 닫기 차단 사유가 공개 오류 코드 묶음에 대응할 수 있습니다.

공개 `ErrorCode` 묶음은 매핑으로 언급할 수 있지만, 스키마나 메서드 담당 문서가 그 정확한 사용을 명시적으로 허용하지 않는 한 그 값을 `CloseReadinessBlocker.code`에 복사하지 않습니다.

| 공개 오류 묶음 | 닫기 준비 상태 차단 사유 표현 | 경계 |
|---|---|---|
| `EVIDENCE_INSUFFICIENT` | 유효한 평가에서 증거 공백을 찾으면 `category=evidence`로 보냅니다. | 사전 확인 실패는 계속 `ToolRejectedResponse.errors[]`를 사용합니다. |
| `ARTIFACT_MISSING` | 닫기에 영향을 주는 지속 아티팩트 문제는 `category=artifact_availability`로 보냅니다. | 아티팩트 형태와 저장 의미는 아티팩트 담당 문서에 남습니다. |
| `ACCEPTANCE_REQUIRED` | 최종 수락이 없거나 호환되지 않으면 `category=final_acceptance`로 보냅니다. | 차단 사유는 최종 수락을 만들지 않습니다. |
| `RESIDUAL_RISK_NOT_VISIBLE` | 닫기에 영향을 주는 알려진 잔여 위험이 보이지 않으면 `category=residual_risk_visibility`로 보냅니다. | 표시 여부는 잔여 위험 수락과 구분됩니다. |
| `DECISION_REQUIRED`, `DECISION_UNRESOLVED` | 미해결 사용자 소유 판단이나 잔여 위험 수락은 `category=user_judgment` 또는 `category=residual_risk_acceptance`로 보냅니다. | 차단 사유는 사용자의 결정을 기록하지 않습니다. |
| `APPROVAL_REQUIRED`, `APPROVAL_DENIED`, `APPROVAL_EXPIRED` | 민감 동작 승인 문제는 `category=sensitive_approval`로 보냅니다. | 차단 사유는 승인이나 `Write Authorization`을 만들지 않습니다. |
| `SCOPE_REQUIRED`, `SCOPE_VIOLATION`, `AUTONOMY_BOUNDARY_EXCEEDED`, `BASELINE_STALE`, `CAPABILITY_INSUFFICIENT` | 범위, 자율성 경계, 기준 상태, 접점 역량 발견 사항은 담당 문서가 허용할 때 `category=scope`, `category=baseline`, 또는 `category=surface_capability`로 보냅니다. | 공개 코드 이름만으로 지원 여부를 추론하지 않습니다. |
| `PROJECTION_STALE` | 읽기용 보기 최신성 문제는 관련 공개 코드 묶음으로 이름 붙일 수 있습니다. | `PROJECTION_STALE`만으로는 닫기 차단 사유가 아닙니다. |
| `STATE_VERSION_CONFLICT` | 닫기 준비 상태 차단 사유 표현이 없습니다. | 오래된 상태는 닫기 준비 상태 평가 전에 거부됩니다. |

## API 응답과의 관계

| API 응답 경로 | 차단 사유 처리 경로가 적용되는 방식 | 담당 경계 |
|---|---|---|
| 거부 응답 | 사전 확인, 검증, 최신성, 로컬 접근, 역량, 요청 실패는 `ToolRejectedResponse.errors[]`에 남습니다. | 거부 응답 분기 경로는 [API 오류 경로](error-routing.md)가 담당합니다. |
| 차단 결과 | 유효한 `CloseTaskResult(close_state=blocked)`는 `blockers: CloseReadinessBlocker[]`를 포함할 수 있습니다. | 메서드 동작과 커밋된 차단 효과는 [`harness.close_task`](method-close-task.md)와 [저장 효과](../storage-effects.md)가 담당합니다. |
| 읽기 전용 관찰 | `StatusResult.close_blockers` 또는 `harness.close_task intent=check`는 읽기 전용 차단 사유 관찰 데이터를 반환할 수 있습니다. | 읽기 전용 관찰은 차단 사유 상태를 저장하거나 `state_version`을 증가시키지 않습니다. |
| `dry_run` 미리보기 | `DryRunSummary.would_blockers: PlannedBlocker[]`는 차단 사유형 결과를 미리 보여 줄 수 있습니다. | `PlannedBlocker`는 `CloseReadinessBlocker`가 아닙니다. |

<a id="harnessclose_task-close-blockers"></a>

## `harness.close_task` 차단 사유 매핑

- 닫기 준비 상태 평가 전 사전 확인 실패:
  - [사전 확인 실패](#close-task-preflight-failure)
- 유효한 읽기인 `intent=check`:
  - [`intent=check`](#close-task-intent-check)
- 닫기 차단 사유를 찾은 `intent=complete`:
  - [차단된 `intent=complete`](#close-task-intent-complete-blocked)
- 닫기 차단 사유가 없는 `intent=complete`:
  - [닫힌 `intent=complete`](#close-task-intent-complete-closed)
- 유효하지 않은 `intent=cancel` 또는 `intent=supersede` 종료 전이:
  - [유효하지 않은 종료 전이](#close-task-invalid-terminal-transition)

<a id="close-task-preflight-failure"></a>
### 사전 확인 실패

조건:
- 닫기 준비 상태 평가 전에 오래된 상태, 오래된 `Write Authorization` 근거, 멱등성 충돌, 검증 실패, 로컬 접근 실패, 역량 실패, Core 상태 읽기 실패, 프로젝트/`Task` 식별 실패가 발생합니다.

응답 경로:
- `ToolRejectedResponse.errors[]`

공개 코드 규칙:
- `STATE_VERSION_CONFLICT`와 다른 커밋 전 오류는 거부 응답에 남습니다.

응답 경계:
- 사전 확인 실패는 `CloseReadinessBlocker` 항목을 반환하지 않습니다.

<a id="close-task-intent-check"></a>
### `intent=check`

조건:
- 요청이 유효한 읽기입니다.

응답 경로:
- 읽기 전용 `CloseTaskResult`

허용되는 것:
- `CloseReadinessBlocker` 관찰 데이터를 반환할 수 있습니다.

상태 영향:
- 저장된 차단 사유와 상태 버전 증가가 없습니다.

<a id="close-task-intent-complete-blocked"></a>
### 차단된 `intent=complete`

조건:
- 유효한 평가에서 닫기 차단 사유를 찾습니다.

응답 경로:
- `CloseTaskResult(close_state=blocked)`

허용되는 것:
- `CloseReadinessBlocker[]`를 반환할 수 있습니다.

공개 코드 경계:
- 차단된 `intent=complete`는 `STATE_VERSION_CONFLICT`를 사용하지 않습니다.

<a id="close-task-intent-complete-closed"></a>
### 닫힌 `intent=complete`

조건:
- 담당 문서가 정의한 닫기 차단 사유가 더 없습니다.

응답 경로:
- `CloseTaskResult(close_state=closed)`

공개 코드 규칙:
- 닫기 차단 사유가 없습니다.

<a id="close-task-invalid-terminal-transition"></a>
### 유효하지 않은 종료 전이

조건:
- `intent=cancel` 또는 `intent=supersede`의 종료 전이가 유효하지 않습니다.

응답 경로:
- 메서드 담당 결과 또는 거부 경로

공개 코드 규칙:
- 차단 사유는 전이 유효성으로 제한합니다.

전이 경계:
- 취소나 대체에 증거 충분성, 최종 수락, 잔여 위험 수락을 요구하지 않습니다.

## 닫기 준비 상태 발견 사항 코드 요약

이 표는 닫기 준비 상태 발견 사항에 대응하는 공개 오류 코드 묶음을 요약합니다. 공개 `ErrorCode` 값을 차단 사유 코드로 바꾸는 규칙이 아닙니다.

| 닫기 준비 상태 발견 사항 | 세부 항목 |
|---|---|
| 증거 공백 | [증거 공백](#close-mapping-evidence-gap) |
| 지속 아티팩트 문제 | [지속 아티팩트 문제](#close-mapping-artifact-issue) |
| 최종 수락 문제 | [최종 수락 문제](#close-mapping-final-acceptance) |
| 잔여 위험이 보이지 않음 | [잔여 위험이 보이지 않음](#close-mapping-residual-risk-not-visible) |
| 잔여 위험 수락 누락 | [잔여 위험 수락 누락](#close-mapping-unaccepted-residual-risk) |
| 미해결 판단 | [해결되지 않은 사용자 소유 판단](#close-mapping-unresolved-user-judgment) |
| 민감 동작 승인 문제 | [민감 동작 승인 문제](#close-mapping-sensitive-approval) |
| 범위, 경계, 기준 상태, 역량 | [범위, 경계, 기준 상태, 역량 차단 사유](#close-mapping-scope-boundary-baseline) |
| 읽기용 보기 최신성 | [읽기용 보기 최신성 문제](#close-mapping-readable-view-freshness) |
| 오래된 상태 거부 | [오래된 상태는 거부](#close-mapping-stale-state-rejected) |

<a id="close-mapping-evidence-gap"></a>
### 증거 공백

조건:
- 닫기 준비 상태 평가에서 증거 공백을 찾습니다.

차단 사유 처리 경로:
- `category=evidence`

공개 코드 매핑:
- `EVIDENCE_INSUFFICIENT`

<a id="close-mapping-artifact-issue"></a>
### 지속 아티팩트 문제

조건:
- 닫기에 영향을 주는 지속 아티팩트가 없거나, 사용할 수 없거나, 닫기 근거로 쓸 수 없거나, 실패했습니다.

차단 사유 처리 경로:
- `category=artifact_availability`

공개 코드 매핑:
- `ARTIFACT_MISSING`

<a id="close-mapping-final-acceptance"></a>
### 최종 수락 문제

조건:
- 필요한 최종 수락이 없거나 호환되지 않습니다.

차단 사유 처리 경로:
- `category=final_acceptance`

공개 코드 매핑:
- `ACCEPTANCE_REQUIRED`

<a id="close-mapping-residual-risk-not-visible"></a>
### 잔여 위험이 보이지 않음

조건:
- 닫기에 영향을 주는 알려진 잔여 위험이 보이지 않습니다.

차단 사유 처리 경로:
- `category=residual_risk_visibility`

공개 코드 매핑:
- `RESIDUAL_RISK_NOT_VISIBLE`

<a id="close-mapping-unaccepted-residual-risk"></a>
### 잔여 위험 수락 누락

조건:
- 잔여 위험은 보였지만 수락 기록이 없습니다.

차단 사유 처리 경로:
- `category=residual_risk_acceptance`

공개 코드 매핑:
- `category=residual_risk_acceptance`와 함께 `DECISION_REQUIRED` 또는 `DECISION_UNRESOLVED`

<a id="close-mapping-unresolved-user-judgment"></a>
### 해결되지 않은 사용자 소유 판단

조건:
- 사용자 소유 판단이 해결되지 않았습니다.

차단 사유 처리 경로:
- `category=user_judgment`

공개 코드 매핑:
- `DECISION_REQUIRED` 또는 `DECISION_UNRESOLVED`

<a id="close-mapping-sensitive-approval"></a>
### 민감 동작 승인 문제

조건:
- 민감 동작 승인이 없거나, 거부되었거나, 만료되었거나, 달라졌습니다.

차단 사유 처리 경로:
- `category=sensitive_approval`

공개 코드 매핑:
- `APPROVAL_REQUIRED`, `APPROVAL_DENIED`, `APPROVAL_EXPIRED`

<a id="close-mapping-scope-boundary-baseline"></a>
### 범위, 경계, 기준 상태, 역량 차단 사유

조건:
- 유효한 평가에서 범위, 자율성 경계, 기준 상태, 접점 역량 차단 사유를 찾습니다.

차단 사유 처리 경로:
- `category=scope`, `category=baseline`, 또는 `category=surface_capability`

공개 코드 매핑:
- `SCOPE_REQUIRED`, `SCOPE_VIOLATION`, `AUTONOMY_BOUNDARY_EXCEEDED`, `BASELINE_STALE`, 또는 `CAPABILITY_INSUFFICIENT`

담당 경계:
- 담당 문서가 매핑을 허용할 때만 공개 코드 매핑을 사용합니다.

<a id="close-mapping-readable-view-freshness"></a>
### 읽기용 보기 최신성 문제

조건:
- 읽기용 보기 최신성 문제가 있습니다.

공개 코드 매핑:
- `PROJECTION_STALE`

담당 경계:
- `PROJECTION_STALE`만으로 닫기 차단 사유를 만들지 않습니다.

<a id="close-mapping-stale-state-rejected"></a>
### 오래된 상태는 거부

조건:
- 프로젝트 전체 상태나 `WriteAuthorization.basis_state_version`이 오래된 상태입니다.

응답 경로:
- `STATE_VERSION_CONFLICT`를 담은 `ToolRejectedResponse.errors[]`

응답 경계:
- 오래된 상태는 닫기 차단 사유가 아닙니다.

## 비주장

차단 사유 처리 경로는 아래를 의미하지 않습니다.

- 최종 수락
- 잔여 위험 수락
- 사용자 승인, 민감 동작 승인, `Write Authorization`
- 증거 충분성 또는 아티팩트 가용성
- 닫기 완료 또는 종료 `Task` 상태
- 차단 사유 지속 저장 또는 상태 버전 증가
- 렌더링 표시 문구
- Core 권한, 메서드, 스키마, 저장소, 템플릿 담당 문서를 우회할 권한
