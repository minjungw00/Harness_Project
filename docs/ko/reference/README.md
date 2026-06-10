# 참조 색인

정확한 하네스 계획 계약의 담당 문서를 고를 때 이 색인을 사용합니다. 이 README는 경로를 안내할 뿐이며 스키마, DDL, 상태 효과, 보안 보장, 템플릿 본문, 현재 MVP 범위를 정의하지 않습니다.

이 문서들은 향후 하네스 서버를 위한 원천 자료입니다. 이 저장소에 서버/런타임 구현, Harness Runtime Home, 런타임 상태, 생성된 상태 보기, 실행 가능한 fixture, 구현 완료 동작이 있다는 뜻이 아닙니다.

## 읽기 규칙

- 지금 확인할 계약에 맞는 담당 문서 하나를 고릅니다.
- 참조 문서 전체를 기본으로 읽지 않습니다.
- 번역이나 의미 일치 검토가 필요한 경우가 아니라면 같은 담당 문서의 영어/한국어 대응 문서를 한 프롬프트에 함께 넣지 않습니다.
- 담당 문서가 아닌 곳에서 계약이 반복되면 먼저 담당 문서를 고치고, 중복 문구는 짧은 결과 설명과 이 색인 경로로 바꿉니다.

## 기준 담당 문서

| 계약 영역 | 담당 문서 |
|---|---|
| 상세 현재 MVP 범위, 제외 항목, 보장 경계 | [active-mvp-scope.md](active-mvp-scope.md) |
| Core 제품 개념과 사용자 소유 판단 | [core-model.md](core-model.md) |
| API 메서드 동작 | [api/mvp-api.md](api/mvp-api.md) |
| 공통 API 봉투와 응답 분기 | [api/schema-core.md](api/schema-core.md) |
| API 상태 스키마 | [api/schema-state.md](api/schema-state.md) |
| API 아티팩트 스키마 | [api/schema-artifacts.md](api/schema-artifacts.md) |
| API 판단 스키마 | [api/schema-judgment.md](api/schema-judgment.md) |
| API 값 집합과 enum 형태 값 | [api/schema-value-sets.md](api/schema-value-sets.md) |
| 공개 API 오류 | [api/errors.md](api/errors.md) |
| 저장소 기록 | [storage-records.md](storage-records.md) |
| 저장 효과 | [storage-effects.md](storage-effects.md) |
| 아티팩트 저장소 생명주기 | [storage-artifacts.md](storage-artifacts.md) |
| 상태 버전, 멱등성, 잠금, 마이그레이션 | [storage-versioning.md](storage-versioning.md) |
| 런타임, 저장소, 서버 경계 | [runtime-boundaries.md](runtime-boundaries.md) |
| 보안 주장과 비보장 | [security.md](security.md) |
| 에이전트 커넥터 참조 | [agent-integration.md](agent-integration.md) |
| 접점별 사용법 | [../use/surface-recipes.md](../use/surface-recipes.md) |
| 상태 보기 권한 | [projection-and-templates.md](projection-and-templates.md) |
| 템플릿 본문 | [template-bodies.md](template-bodies.md) |
| 이후 후보 | [../later/index.md](../later/index.md) |
| 용어 | [glossary.md](glossary.md), [번역 가이드](../maintain/translation-guide.md), [docs/terminology-map.yaml](../../terminology-map.yaml) |
| 문서 작성 규칙 | [작성 가이드](../maintain/authoring-guide.md) |
| 문서 점검 | [문서 점검](../maintain/checks.md) |

## 담당 문서가 아닌 곳의 경로 규칙

README, Start, Use, Build, Maintain, route/index 문서는 독자에게 보이는 결과를 짧게 요약할 수 있지만 기술 계약의 두 번째 기준 문서가 되면 안 됩니다. 이 색인이나 여기서 고른 담당 문서로 연결해야 합니다.

[storage.md](storage.md)처럼 오래된 링크를 위해 남는 넓은 호환 문서는 분리된 담당 문서가 있을 때 경로 안내만 담당합니다.
