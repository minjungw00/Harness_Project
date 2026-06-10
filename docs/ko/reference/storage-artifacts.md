# 아티팩트 저장소

이 문서는 현재 MVP의 아티팩트 저장소 생명주기를 담당합니다. 문서 원천 자료일 뿐이며 아티팩트 바이트, 아티팩트 디렉터리, 런타임 저장소를 만들지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- 스테이징된 아티팩트 저장 생명주기
- 저장된 스테이징 기록에 대한 `StagedArtifactHandle` 검증
- 호환되는 스테이징된 핸들에서 지속 `ArtifactRef`로 승격하는 경로
- 지속 `existing_artifact` 연결 자격
- 아티팩트 본문 읽기의 저장소 자격, 가용성, 가림 처리, 보존, 무결성 경계

이 문서는 담당하지 않습니다.

- API 아티팩트 wire 스키마: [API 아티팩트 스키마](api/schema-artifacts.md)
- 메서드 동작: [MVP API](api/mvp-api.md)
- 일반 기록 DDL: [저장소 기록](storage-records.md)
- 일반 저장 효과: [저장 효과](storage-effects.md)
- 로컬 접근 보안 주장: [보안](security.md), [런타임 경계](runtime-boundaries.md)

## 경계

아티팩트 저장소는 스테이징, 승격, 지속 연결, 본문 읽기를 구분합니다. `existing_artifact`는 기존 지속 아티팩트를 연결하는 경로이지 새 아티팩트 본문 바이트를 등록하지 않습니다. 스테이징된 핸들의 형태만으로 권한이 생기지 않으며, 호환되는 저장 스테이징 기록으로 해석되어야 합니다.

## 관련 담당 문서

- [API 아티팩트 스키마](api/schema-artifacts.md): `ArtifactRef`, `ArtifactInput`, `StagedArtifactHandle` 형태.
- [MVP API](api/mvp-api.md): `harness.stage_artifact`, `harness.record_run`, 아티팩트 읽기 동작.
- [저장 효과](storage-effects.md): 응답 분기가 저장 효과를 만드는지 여부.
- [보안](security.md): 접근과 보장 비주장.
