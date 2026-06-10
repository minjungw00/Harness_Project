# 저장 효과

이 문서는 현재 MVP의 메서드별 저장 효과 의미를 담당합니다. 문서 원천 자료일 뿐이며 하네스 런타임 절차를 실행하거나 모의 실행하지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- 읽기 전용, dry-run, 거절, 스테이징 생성, Core 커밋, 커밋된 차단 결과의 저장 효과 구분
- 메서드가 재실행 행, task event, 기록 변경, 상태 버전 증가, 스테이징된 핸들 소비, 아티팩트 승격, Write Authorization 변경을 만드는지 여부
- 차단 사유형 응답 데이터의 지속 저장 경계
- 거절 응답과 유효한 dry-run 미리보기 분기의 효과 없음 보장

이 문서는 담당하지 않습니다.

- 기록 형태나 DDL: [저장소 기록](storage-records.md)
- 아티팩트 생명주기 세부사항: [아티팩트 저장소](storage-artifacts.md)
- 멱등성, 잠금, 상태 버전 시계, 마이그레이션: [저장소 버전 관리](storage-versioning.md)
- 공개 응답 분기나 스키마: [API Schema Core](api/schema-core.md)
- 메서드 동작: [MVP API](api/mvp-api.md)

## 경계

응답 데이터 구조와 저장 효과는 별개입니다. `CloseReadinessBlocker` 같은 필드가 있다고 해서 그 자체로 지속 저장이나 변경이 증명되지 않습니다. 효과는 선택된 메서드 동작과 응답 분기가 정합니다.

## 관련 담당 문서

- [MVP API](api/mvp-api.md): 선택된 메서드 동작과 응답 공용체.
- [API Errors](api/errors.md): 거절 응답의 공개 오류.
- [저장소 기록](storage-records.md): 효과가 건드릴 수 있는 기록.
- [저장소 버전 관리](storage-versioning.md): 상태 시계와 재실행/멱등성 의미.
