# 저장소 참조 경로

이 문서는 오래된 `storage.md` 링크를 위한 호환 경로로만 남습니다. 분리된 담당 문서 모델에서는 저장소 계약의 기준 담당 문서가 아닙니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- 오래된 `storage.md` 링크에서 현재 저장소 담당 문서로 가는 경로
- 문서 편집이 런타임 저장소를 만들면 안 된다는 안내

이 문서는 담당하지 않습니다.

- 저장소 기록이나 DDL: [저장소 기록](storage-records.md)
- 저장 효과: [저장 효과](storage-effects.md)
- 아티팩트 저장소 생명주기: [아티팩트 저장소](storage-artifacts.md)
- 상태 버전, 멱등성, 잠금, 마이그레이션: [저장소 버전 관리](storage-versioning.md)
- API 스키마, 메서드 동작, 보안 주장, 런타임 배포

## 저장소 담당 경로

| 필요 | 담당 문서 |
|---|---|
| 지속 기록, DDL, 열 의미, 저장소 소유 JSON | [저장소 기록](storage-records.md) |
| 메서드별 저장 효과와 효과 없음 분기 | [저장 효과](storage-effects.md) |
| 스테이징된 아티팩트, 승격, 지속 연결, 본문 읽기 자격, 보존, 무결성 | [아티팩트 저장소](storage-artifacts.md) |
| 프로젝트 전체 `state_version`, 멱등성, 잠금, 마이그레이션 | [저장소 버전 관리](storage-versioning.md) |
| Runtime Home 분리 | [런타임 경계](runtime-boundaries.md) |

저장소 담당 문서는 향후 Harness Runtime Home 기록만 설명합니다. 이 문서 저장소는 Runtime Home이 아니며 생성된 런타임 상태를 담으면 안 됩니다.
