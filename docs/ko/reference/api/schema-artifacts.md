# API 아티팩트 스키마

이 문서는 현재 MVP의 아티팩트 형태 API 스키마를 담당합니다. 문서 원천 자료일 뿐이며 로컬 파일 접근 권한이나 아티팩트 저장소를 만들지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- `ArtifactRef`
- `ArtifactInput`
- `StagedArtifactHandle`
- 스테이징, 연결, 본문 읽기 참조에 쓰이는 아티팩트 형태 요청/응답 필드
- 아티팩트 형태 응답에 나타나는 API 가림 처리와 가용성 필드

이 문서는 담당하지 않습니다.

- 아티팩트 저장소 배치, 스테이징 기록, 승격 지속 효과, 보존, 본문 읽기 저장소 자격: [아티팩트 저장소](../storage-artifacts.md)
- `harness.stage_artifact`, `harness.record_run`, 아티팩트 읽기 메서드 동작: [MVP API](mvp-api.md)
- 활성 아티팩트 값 집합: [API 값 집합](schema-value-sets.md)
- 접근이나 격리에 대한 보안 주장: [보안](../security.md)

## 경계

아티팩트 스키마는 호출자가 보낸 경로 문자열을 권한으로 만들지 않습니다. 새 아티팩트 바이트는 활성 스테이징 경로로만 현재 MVP에 들어오고, 기존 아티팩트는 호환되는 지속 `ArtifactRef` 기록을 통해서만 연결됩니다. 검증, 승격, 연결, 읽기 자격은 저장소와 API 메서드 담당 문서가 정합니다.

## 관련 담당 문서

- [MVP API](mvp-api.md): 아티팩트 관련 메서드 동작.
- [아티팩트 저장소](../storage-artifacts.md): 스테이징, 승격, 지속 연결, 본문 읽기 생명주기.
- [API 값 집합](schema-value-sets.md): `ArtifactInput.source_kind`, `ArtifactRef.kind` 등 관련 값.
- [런타임 경계](../runtime-boundaries.md)와 [보안](../security.md): 로컬 접근과 비보장 경계.
