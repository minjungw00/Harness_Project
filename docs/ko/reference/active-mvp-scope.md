# 현재 MVP 범위 참조

이 문서는 하네스 계획 문서에서 현재 MVP 범위를 담당합니다. 문서 원천 자료일 뿐이며, 이 저장소에 하네스 서버, 런타임 상태, 생성된 상태 보기 시스템, 적합성 실행기, 구현 완료 동작이 있다는 뜻이 아닙니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- 닫힌 현재 MVP 기능 목록
- 제품 범위 수준의 active/later 경계
- 이후 후보가 현재 MVP로 섞이지 않게 하는 범위 수준 비보장
- 각 활성 범위 항목에서 정확한 계약 담당 문서로 가는 경로

이 문서는 담당하지 않습니다.

- 구현 순서, 유지보수자 준비 상태, 서버 코딩 인계: [MVP 계획](../build/mvp-plan.md)
- Core 전이 의미와 사용자 소유 판단 경계: [Core Model](core-model.md)
- API 메서드 동작: [MVP API](api/mvp-api.md)
- API 스키마와 값 집합: [API Schema Core](api/schema-core.md), [API 상태 스키마](api/schema-state.md), [API 아티팩트 스키마](api/schema-artifacts.md), [API 판단 스키마](api/schema-judgment.md), [API 값 집합](api/schema-value-sets.md)
- 저장소 기록, 효과, 아티팩트, 버전 관리, 잠금, 마이그레이션: [저장소 기록](storage-records.md), [저장 효과](storage-effects.md), [아티팩트 저장소](storage-artifacts.md), [저장소 버전 관리](storage-versioning.md)
- 보안 보장 의미: [보안](security.md)
- 이후 후보 담당 경계: [이후 후보 색인](../later/index.md)

## 현재 MVP 범위

현재 MVP는 범위, 사용자 소유 판단, 쓰기 호환성, 증거 참조, 아티팩트, 닫기 준비 상태, 최종 수락, 잔여 위험을 보존하는 가장 작은 로컬 작업 권한 루프입니다. 도구 샌드박스나 런타임 격리를 주장하지 않습니다.

현재 MVP에는 아래 항목만 포함됩니다.

- `harness.intake`를 통한 평소 말 입력과 Task 생성
- `harness.update_scope`를 통한 범위와 Change Unit 갱신
- `ShapingReadiness`를 통한 간결한 활성 상태 표시
- `harness.request_user_judgment`와 `harness.record_user_judgment`를 통한 사용자 소유 판단 요청과 답변 기록
- 활성 `sensitive_approval` 판단 경로로 다루는 민감 동작 승인
- 제품 파일 쓰기를 위한 경로 수준 `harness.prepare_write`와 1회용 Write Authorization
- 활성 스테이징된 아티팩트 입력을 위한 `harness.stage_artifact`
- 구체화, 직접 결과, 구현 Run을 기록하고 호환되는 지속 아티팩트를 승격하거나 연결하는 `harness.record_run`
- 간결한 `EvidenceSummary`
- 읽는 시점의 상태 출력을 위한 `harness.status`
- 닫기 준비 상태 확인과 닫기/취소/supersede 결과를 다루는 `harness.close_task`
- 파생 표시로서 읽는 시점에 만드는 상태 보기/상태 출력
- 기준 로컬 MCP 접점의 등록된 로컬 접점 접근
- 협력형 보장 표시
- 대상 관찰 범위에 필요한 역량 확인이 통과한 뒤에만 쓰는 탐지형 보장 표시

## 승격 전 범위 밖

현재 MVP에는 접점 자체 아티팩트 캡처, `captured_artifact`, 상태 보기 조정, 지속 저장되는 상태 보기 작업, 관리 블록 불일치 복구, 전체 Evidence Manifest, 수동 QA 작업 흐름, `qa_gate`, `verification_gate`, 명령/네트워크/비밀값 관찰, 명령/네트워크/비밀값 도구 실행 전 차단, 예방형 보장, 격리형 보장, 호스팅 대시보드, 커넥터 마켓플레이스, 내보내기/인계 형식, 실행 가능한 fixture 실행기, 생성된 적합성 산출물, 운영 프로필이 포함되지 않습니다.

예시나 경로 문구에서 이후 후보를 언급해도 승격이 아닙니다. 승격하려면 담당 문서가 범위, 대체 동작, 증명 기대, 한영 문서 동시 유지를 함께 추가해야 합니다.

## 관련 담당 문서

| 필요 | 담당 문서 |
|---|---|
| 구현 준비와 유지보수자 인계 상태 | [MVP 계획](../build/mvp-plan.md) |
| Core 의미와 사용자 소유 판단 | [Core Model](core-model.md) |
| API 메서드 | [MVP API](api/mvp-api.md) |
| API 스키마와 값 집합 | [API Schema Core](api/schema-core.md), [API 상태 스키마](api/schema-state.md), [API 아티팩트 스키마](api/schema-artifacts.md), [API 판단 스키마](api/schema-judgment.md), [API 값 집합](api/schema-value-sets.md) |
| 저장소 | [저장소 기록](storage-records.md), [저장 효과](storage-effects.md), [아티팩트 저장소](storage-artifacts.md), [저장소 버전 관리](storage-versioning.md) |
| 상태 보기 권한과 템플릿 본문 | [Projection과 템플릿](projection-and-templates.md), [템플릿 본문](template-bodies.md) |
| 접점 사용법과 커넥터 동작 | [접점별 사용법](../use/surface-recipes.md), [에이전트 통합](agent-integration.md) |
| 보안 주장과 비보장 | [보안](security.md) |
| 이후 자료 | [이후 후보 색인](../later/index.md) |
