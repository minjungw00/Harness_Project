# API Schema Core

이 문서는 현재 MVP의 공통 API envelope와 응답 분기 스키마를 담당합니다. 문서 원천 자료일 뿐이며 메서드 동작, 저장 효과, 아티팩트 생명주기, 사용자 판단 의미, 활성 값 집합을 정의하지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- `ToolEnvelope`
- 공통 응답 분기 구조
- `ToolResultBase`
- `ToolRejectedResponse`
- `ToolDryRunResponse`
- 공통 응답 메타데이터 필드와 분기 구분
- 응답 분기를 보조하는 공통 `ToolError`와 `EventRef` 형태

이 문서는 담당하지 않습니다.

- 메서드 동작: [MVP API](mvp-api.md)
- API 상태 스키마: [API 상태 스키마](schema-state.md)
- API 아티팩트 스키마: [API 아티팩트 스키마](schema-artifacts.md)
- API 판단 스키마: [API 판단 스키마](schema-judgment.md)
- API 값 집합과 enum 형태 값: [API 값 집합](schema-value-sets.md)
- 공개 오류 코드와 우선순위: [API Errors](errors.md)
- 저장소 기록과 효과: [저장소 기록](../storage-records.md), [저장 효과](../storage-effects.md)

## 스키마 표기 규칙

이 문서의 스키마 블록은 계획 표기입니다. 생성된 코드가 아니라 향후 API 계약 형태를 설명합니다.

<a id="tool-envelope"></a>
## ToolEnvelope

`ToolEnvelope`는 메서드 담당 문서가 다르게 말하지 않는 한 공개 메서드가 사용하는 공통 요청 envelope입니다.

```yaml
ToolEnvelope:
  project_id: string
  actor_kind: string
  surface_id: string
  request_id: string
  idempotency_key: string | null
  expected_state_version: integer | null
  dry_run: boolean
  locale: string | null
```

envelope 필드의 정확한 활성 값은 [API 값 집합](schema-value-sets.md)이 담당합니다. 메서드별 요청 본문과 필수/선택 동작은 [MVP API](mvp-api.md)가 담당합니다.

<a id="common-response"></a>
## 공통 응답 분기

공개 메서드 응답은 정확히 하나의 분기를 사용합니다.

- 메서드별 `MethodResult`
- `ToolRejectedResponse`
- 선택된 상태 효과 동작에 유효한 미리보기 분기가 있을 때의 `ToolDryRunResponse`

```yaml
ToolResultBase:
  response_kind: string
  effect_kind: string
  dry_run: boolean
  state_version: integer | null
  events: EventRef[]

ToolRejectedResponse:
  base: ToolResultBase
  errors: ToolError[]

ToolDryRunResponse:
  base: ToolResultBase
  preview: object
```

메서드별 결과 필드는 그 메서드 결과 분기에만 둡니다. 거절 응답과 dry-run 미리보기 응답은 성공한 메서드의 결과 전용 필드를 요구하지 않습니다.

## 공통 보조 형태

```yaml
ToolError:
  code: string
  message: string
  retryable: boolean
  details: object | null

EventRef:
  event_id: string
  event_kind: string
```

공개 `ErrorCode` 값과 우선순위는 [API Errors](errors.md)가 담당합니다. 활성 `response_kind`, `effect_kind`, 그 밖의 enum 형태 값은 [API 값 집합](schema-value-sets.md)이 담당합니다.

<a id="local-surface-access-values"></a>
## 로컬 접점 맥락

공통 로컬 접점 맥락 필드는 요청/응답 계약에 속하지만 활성 값과 커넥터 동작은 분리되어 있습니다.

- [API 값 집합](schema-value-sets.md)이 활성 `access_class`와 관련 값을 담당합니다.
- [MVP API](mvp-api.md)가 메서드 요청 조건을 담당합니다.
- [에이전트 통합](../agent-integration.md)이 커넥터 동작을 담당합니다.
- [보안](../security.md)이 보장 주장과 비보장을 담당합니다.

<a id="state-summary"></a>
## 상태 스키마 경로

`StateSummary`, `StateRecordRef`, `ShapingReadiness`는 [API 상태 스키마](schema-state.md)가 담당합니다.

<a id="artifactref"></a>
## 아티팩트 스키마 경로

`ArtifactRef`, `ArtifactInput`, `StagedArtifactHandle`은 [API 아티팩트 스키마](schema-artifacts.md)가 담당합니다.

<a id="current-position-display-schemas"></a>
## 현재 위치 표시 스키마 경로

닫기 준비 상태와 다음 행동 데이터 형태를 포함한 현재 위치 표시 스키마는 [API 상태 스키마](schema-state.md)가 담당합니다.

<a id="validatorresult"></a>
## ValidatorResult 경로

`ValidatorResult` 형태는 [API 상태 스키마](schema-state.md)가 담당합니다. 활성 validator ID와 severity 형태 값은 [API 값 집합](schema-value-sets.md)이 담당합니다.

<a id="current-mvp-value-sets"></a>
## 현재 MVP 값 집합 경로

활성 메서드 이름, API enum 형태 값, profile-gated 값 경계는 [API 값 집합](schema-value-sets.md)이 담당합니다. 이 앵커는 오래된 링크를 위해서만 남아 있습니다.
