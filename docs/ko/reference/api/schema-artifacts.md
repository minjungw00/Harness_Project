# API 아티팩트 스키마

이 문서는 현재 MVP의 아티팩트 형태 API 스키마를 담당합니다. 문서 원천 자료일 뿐이며 로컬 파일 접근 권한, 아티팩트 본문, 저장소 행, 증거 충분성을 만들지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- `ArtifactRef`
- `ArtifactInput`
- `StagedArtifactHandle`
- 스테이징된 아티팩트 입력과 기존 아티팩트 입력의 구분
- 스테이징, 연결, 본문 읽기 참조에 쓰이는 아티팩트 형태 요청/응답 필드
- 참조별 아티팩트 접근 제약
- 아티팩트 형태 API 응답에 나타나는 가림 처리, 가용성, 체크섬, 크기 필드

이 문서는 담당하지 않습니다.

- 아티팩트 저장소 배치, 스테이징 기록, 승격 지속 효과, 보존, 본문 읽기 저장소 자격: [아티팩트 저장소](../storage-artifacts.md)
- `harness.stage_artifact`, `harness.record_run`, 아티팩트 읽기 메서드 동작: [MVP API](mvp-api.md)
- 활성 아티팩트 값 집합: [API 값 집합](schema-value-sets.md)
- 증거 충분성: [Core 모델](../core-model.md), [API 상태 스키마](schema-state.md)
- 접근, 차단, 격리에 대한 보안 주장: [보안](../security.md)

## 경계

아티팩트 스키마는 호출자가 보낸 경로 문자열을 권한으로 만들지 않습니다. 새 아티팩트 바이트는 임시 `StagedArtifactHandle`을 반환하는 `harness.stage_artifact`를 통해서만 현재 MVP에 들어옵니다. 기존 아티팩트는 호환되는 지속 `ArtifactRef` 기록을 통해서만 연결됩니다. 검증, 승격, 연결, 읽기 자격은 저장소와 API 메서드 담당 문서가 정합니다.

## `ArtifactRef`

`ArtifactRef`는 담당 경로가 이미 등록한 지속 아티팩트를 가리키는 공개 포인터입니다.

```yaml
ArtifactRef:
  artifact_id: string
  project_id: string
  task_id: string
  display_name: string
  content_type: string
  sha256: string
  size_bytes: integer
  redaction_state: string
  availability: string
  created_by_run_ref: StateRecordRef | null
  created_by_surface_id: string | null
  created_by_surface_instance_id: string | null
  storage_ref: string | null
```

`ArtifactRef`는 참조와 메타데이터 형태입니다. 이 값만으로 아티팩트 본문을 읽을 수 있는 것도 아니고, 그 본문이 닫기에 충분한 증거라는 뜻도 아닙니다. 아티팩트 본문 읽기에는 `access_class=artifact_read`를 쓰는 담당 경로가 필요합니다.

## `StagedArtifactHandle`

`StagedArtifactHandle`은 성공한 `harness.stage_artifact`가 반환하는 임시 핸들입니다. 지속 아티팩트가 아니라 저장소가 소유하는 임시 스테이징을 나타냅니다.

```yaml
StagedArtifactHandle:
  handle_id: string
  project_id: string
  task_id: string
  created_by_surface_id: string
  created_by_surface_instance_id: string
  content_type: string
  sha256: string
  size_bytes: integer
  redaction_state: string
  expires_at: string
  consumed: boolean
```

호출자는 `created_by_surface_id`나 `created_by_surface_instance_id`를 권한 주장으로 제출하지 않습니다. 이 값들은 스테이징 요청의 확인된 로컬 접점 맥락에서 서버가 기록합니다. 스테이징된 핸들은 범위가 정해져 있고, 만료되며, 한 번만 소비됩니다. 어떤 로컬 호출자나 사용할 수 있는 bearer token이 아니며, 호환되는 `harness.record_run` 승격이 지속 `ArtifactRef`를 만들기 전까지 증거 권한도 아닙니다.

## `ArtifactInput`

`ArtifactInput`은 Run이나 증거 출력에 아티팩트를 연결하는 메서드가 사용합니다.

```yaml
ArtifactInput:
  artifact_input_id: string
  source_kind: string
  staged_artifact_handle: StagedArtifactHandle | null
  existing_artifact_ref: ArtifactRef | null
  relation_hint: string | null
  claim: string | null
  expected_sha256: string | null
  expected_size_bytes: integer | null
  redaction_state: string | null
```

각 입력에서는 정확히 하나의 출처 필드만 활성입니다.

| `source_kind` | 필요한 출처 필드 | 의미 |
|---|---|---|
| `staged_artifact` | `staged_artifact_handle` | `harness.record_run` 중 호환되는 임시 스테이징 핸들을 승격합니다. |
| `existing_artifact` | `existing_artifact_ref` | 새 바이트를 등록하지 않고 이미 지속되는 같은 프로젝트 아티팩트를 연결합니다. |

`captured_artifact`, 접점 자체 캡처 핸들, 원시 캡처 어댑터 출력, 원시 파일시스템 경로, 임의 로컬 경로 문자열, 권한 주장으로서의 원시 로그는 현재 MVP의 활성 `ArtifactInput` 출처가 아닙니다.

## 참조별 접근 제약

아티팩트 형태 참조는 담당 경로를 통해 확인합니다.

- `harness.stage_artifact`는 `access_class=artifact_registration`을 사용하고 임시 `StagedArtifactHandle`만 만듭니다.
- `harness.record_run`은 `ArtifactInput[]`에 `source_kind=staged_artifact`가 있어도 `access_class=run_recording`을 사용합니다.
- 스테이징된 아티팩트 승격에는 현재 확인된 `surface_id`와 `surface_instance_id`가 스테이징된 핸들의 서버 기록 `created_by_surface_id`, `created_by_surface_instance_id`와 일치해야 합니다.
- `existing_artifact`에는 같은 프로젝트와 허용된 Task 범위에서 유효한 지속 `ArtifactRef`가 필요합니다.
- 아티팩트 본문 읽기는 스테이징과 승격과 별개입니다. `access_class=artifact_read`와 아티팩트 본문 담당 경로가 필요합니다.
- `ArtifactInput[]`은 공개 API 요청에 두 번째 요청 수준 접근 등급을 더하지 않습니다.

출처 필드 형태가 잘못되었거나 스테이징된 핸들 검증이 실패하면 [API 오류](errors.md)가 담당하는 공개 오류 의미에 따라 `ToolRejectedResponse`로 반환합니다. 스테이징된 핸들의 저장소 검증과 승격 생명주기는 [아티팩트 저장소](../storage-artifacts.md)가 담당합니다.

## 관련 담당 문서

- [MVP API](mvp-api.md): 아티팩트 관련 메서드 동작.
- [아티팩트 저장소](../storage-artifacts.md): 스테이징, 승격, 지속 연결, 본문 읽기 생명주기.
- [API 값 집합](schema-value-sets.md): `ArtifactInput.source_kind`, `redaction_state`, 가용성, 관련 값.
- [API 상태 스키마](schema-state.md): `ArtifactRef`를 언급하는 증거 요약.
- [런타임 경계](../runtime-boundaries.md)와 [보안](../security.md): 로컬 접근과 비보장 경계.
