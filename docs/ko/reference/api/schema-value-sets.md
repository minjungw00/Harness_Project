# API 값 집합

이 문서는 현재 MVP의 활성 API 값 집합과 enum 형태 값을 담당합니다. 문서 원천 자료일 뿐이며 이후 후보 이름을 적는 것만으로 활성 범위를 넓히지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- 활성 공개 메서드 이름 값
- API `response_kind`와 `effect_kind` 값
- 활성 `access_class` 값
- 생명주기, 닫기 상태, 출처 종류, 판단 종류, 표시 형식, 필요 판단 위치, 선택지, 아티팩트, 가림 처리, validator, 보장 표시 등 API 값 집합
- 렌더링된 라벨이 기준 스키마 값이 아니라는 규칙

이 문서는 담당하지 않습니다.

- 공개 `ErrorCode` 값과 우선순위: [API Errors](errors.md)
- 이 값을 쓰는 필드 형태: [API Schema Core](schema-core.md), [API 상태 스키마](schema-state.md), [API 아티팩트 스키마](schema-artifacts.md), [API 판단 스키마](schema-judgment.md)
- 메서드 동작: [MVP API](mvp-api.md)
- 승격 전 이후 후보 값 이름: [이후 후보 색인](../../later/index.md)

## 경계

이 담당 문서가 활성 값으로 둔 값만 활성 API 값입니다. profile-gated 값은 사용하는 자리에서 프로필이나 역량 조건을 이름 붙여야 합니다. 이후 이름은 승격된 담당 문서가 정확한 활성 필드, 대체 동작, 증명 기대를 추가하기 전까지 목록 전용입니다.

## 관련 담당 문서

- [현재 MVP 범위](../active-mvp-scope.md): 값이 현재 MVP에 속하는지 판단.
- [API Errors](errors.md): 공개 오류 코드.
- [API Schema Core](schema-core.md), [API 상태 스키마](schema-state.md), [API 아티팩트 스키마](schema-artifacts.md), [API 판단 스키마](schema-judgment.md): 이 값을 쓰는 필드.
- [이후 후보 색인](../../later/index.md): 비활성 값 이름.
