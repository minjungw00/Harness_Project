# API 상태 스키마

이 문서는 현재 MVP의 상태 형태 API 스키마를 담당합니다. 문서 원천 자료일 뿐이며 런타임 상태나 생성된 상태 보기를 만들지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- `StateSummary`와 상태 형태의 공개 응답 필드
- `StateRecordRef`
- `ShapingReadiness` API 필드
- `NextActionSummary`, `CloseReadinessBlocker`, `ValidatorResult` 같은 현재 위치 표시 스키마
- 상태 형태 데이터와 응답 효과의 경계

이 문서는 담당하지 않습니다.

- 공통 봉투 구조와 응답 분기: [API Schema Core](schema-core.md)
- 활성 enum 형태 값: [API 값 집합](schema-value-sets.md)
- 메서드 동작: [MVP API](mvp-api.md)
- Core 생명주기 의미: [Core Model](../core-model.md)
- 저장소 기록과 지속 효과: [저장소 기록](../storage-records.md), [저장 효과](../storage-effects.md)

## 경계

상태 스키마는 API 데이터 형태를 설명합니다. 상태처럼 보이는 필드가 있다고 해서 그 자체로 지속 저장, Core 전이, 재실행 행, `task_events`, 아티팩트 효과, Write Authorization 효과, `state_version` 증가가 생기지 않습니다. 효과는 응답 분기와 메서드 동작 담당 문서가 정합니다.

## 관련 담당 문서

- [API Schema Core](schema-core.md): `ToolEnvelope`, `ToolResultBase`, `ToolRejectedResponse`, `ToolDryRunResponse`.
- [API 값 집합](schema-value-sets.md): 상태 필드가 쓰는 정확한 값.
- [MVP API](mvp-api.md): 이 스키마를 반환하는 메서드.
- [저장 효과](../storage-effects.md): 지속 저장과 상태 효과.
