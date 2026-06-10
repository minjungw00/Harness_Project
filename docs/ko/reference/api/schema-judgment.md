# API 판단 스키마

이 문서는 현재 MVP의 사용자 소유 판단 API 스키마를 담당합니다. 문서 원천 자료일 뿐이며 그 자체로 사용자 결정을 기록하지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- `UserJudgment`
- `UserJudgmentCandidate`
- `UserJudgmentOption`
- `UserJudgmentContext`
- `UserJudgmentResolution`
- `RecordUserJudgmentPayload`
- `SensitiveActionScope`
- `AcceptedRiskInput`

이 문서는 담당하지 않습니다.

- 사용자 소유 판단의 제품 의미와 대체 불가능 규칙: [Core Model](../core-model.md)
- 판단 요청과 기록 메서드 동작: [MVP API](mvp-api.md)
- 활성 판단 종류 값: [API 값 집합](schema-value-sets.md)
- 최종 수락이나 잔여 위험 수락의 닫기 효과: [Core Model](../core-model.md), [MVP API](mvp-api.md)

## 경계

판단 스키마는 사용자가 소유한 선택의 구조를 보존합니다. 넓은 승인이 제품 판단, 기술 판단, 범위 판단, 민감 동작 승인, 최종 수락, 잔여 위험 수락, 취소 판단, later QA 면제 판단, later 검증 위험 수락을 대신하게 만들지 않습니다.

## 관련 담당 문서

- [Core Model](../core-model.md): 사용자 소유 판단 의미.
- [MVP API](mvp-api.md): `harness.request_user_judgment`, `harness.record_user_judgment`.
- [API 값 집합](schema-value-sets.md): `judgment_kind`, `presentation`, `required_for`, 선택지 값.
- [이후 후보 색인](../../later/index.md): 이후 판단 표시와 later/reserved 판단 경로.
