# 저장소 기록

이 문서는 현재 MVP의 지속 저장 기록 형태를 담당합니다. 문서 원천 자료일 뿐이며 이 저장소에 런타임 데이터베이스, 생성된 기록, 구현 준비가 끝난 DDL을 만들지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- 프로젝트 상태, Task, Change Unit, 사용자 판단, Run, 증거 요약, Write Authorization, 접점, 재실행 행, task event 같은 지속 기록 계열
- 현재 MVP 활성 저장소의 DDL 담당 관계와 열 의미
- 저장소가 소유하는 JSON 필드 배치
- 기록 수준의 active/later 제외 경계

이 문서는 담당하지 않습니다.

- 메서드별 저장 효과: [저장 효과](storage-effects.md)
- 아티팩트 스테이징, 승격, 연결, 본문 읽기, 보존, 무결성 생명주기: [아티팩트 저장소](storage-artifacts.md)
- `state_version`, 멱등성, 잠금, 마이그레이션: [저장소 버전 관리](storage-versioning.md)
- API wire 스키마: [API](api/schema-core.md) 아래 스키마 담당 문서
- 런타임/저장소/서버 경계: [런타임 경계](runtime-boundaries.md)

## 경계

저장소 기록은 향후 Harness Runtime Home 기록이지 이 문서 저장소의 파일이 아닙니다. 문서 편집은 런타임 상태, 생성된 기록, 운영 산출물, 적합성 결과, 수락 기록을 만들면 안 됩니다.

## 관련 담당 문서

- [저장 효과](storage-effects.md): 어떤 메서드가 기록을 만들거나, 바꾸거나, 관찰하거나, 건드리지 않는지.
- [아티팩트 저장소](storage-artifacts.md): 아티팩트 전용 저장 생명주기.
- [저장소 버전 관리](storage-versioning.md): 시계, 멱등성, 잠금, 마이그레이션 의미.
- [MVP API](api/mvp-api.md): 기록을 사용하는 공개 메서드 동작.
