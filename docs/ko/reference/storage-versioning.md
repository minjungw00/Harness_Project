# 저장소 버전 관리

이 문서는 현재 MVP 저장소의 상태 버전, 멱등성, 잠금, 마이그레이션 의미를 담당합니다. 문서 원천 자료일 뿐이며 마이그레이션을 실행하거나 런타임 잠금을 만들지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- 공개 프로젝트 전체 `project_state.state_version` 충돌 기준
- 저장소 의미 수준의 상태 버전 증가 규칙
- 멱등성과 요청 해시 재실행 의미
- 잠금 정책
- 마이그레이션 의미와 active/later 마이그레이션 경계

이 문서는 담당하지 않습니다.

- 기록 형태나 DDL: [저장소 기록](storage-records.md)
- 어떤 메서드 분기가 효과를 만드는지: [저장 효과](storage-effects.md), [MVP API](api/mvp-api.md)
- 공개 오류 코드와 우선순위: [API Errors](api/errors.md)
- 런타임 배포나 운영 명령

## 경계

현재 MVP의 공개 충돌 시계는 담당 문서가 범위와 증명 기대를 갖춰 다른 시계를 승격하기 전까지 프로젝트 전체 기준입니다. Task 범위 시계는 담당 문서가 승격하기 전까지 비공개 또는 이후 경계에만 둘 수 있습니다.

## 관련 담당 문서

- [API Errors](api/errors.md): `STATE_VERSION_CONFLICT` 같은 공개 충돌 오류.
- [저장 효과](storage-effects.md): 어떤 분기가 상태를 올리거나 올리지 않는지.
- [저장소 기록](storage-records.md): 버전 관리나 재실행 데이터를 저장하는 열.
- [런타임 경계](runtime-boundaries.md): Runtime Home 분리.
