# 하네스 문서

이 문서는 현재 하네스 문서 세트의 한국어 진입점입니다. 하네스는 AI 지원 제품 작업을 위한 로컬 작업 권한 서버로 계획되어 있지만, 이 저장소는 현재 문서 전용입니다.

서버/런타임 구현, 런타임 상태, 생성된 상태 보기, 생성된 운영 산출물, 실행 가능한 fixture, 적합성 실행기, 제품 구현 코드는 없습니다. 사용자의 Product Repository도, Harness Runtime Home도, 실행 중인 하네스 인스턴스도 아닙니다.

## 현재 경로

이 진입점은 활성 문서 구조와 경로 색인만 가리킵니다.

| 목적 | 경로 |
|---|---|
| 첫 이해 모델 | [시작하기](start.md) |
| 사용자 작업 흐름 | [사용자 가이드](use/user-guide.md) |
| 에이전트 동작 | [에이전트 가이드](use/agent-guide.md) |
| 사용자 소유 판단 예시 | [판단 예시](use/judgment-examples.md) |
| 접점별 사용법 | [접점별 사용법](use/surface-recipes.md) |
| 구현 준비 결정 | [MVP 계획](build/mvp-plan.md) |
| 정확한 계약의 담당 문서 색인 | [참조 색인](reference/README.md) |
| 이후 후보 자료 | [이후 후보 색인](later/index.md) |
| 문서 작성 규칙 | [작성 가이드](maintain/authoring-guide.md) |
| 번역과 의미 일치 규칙 | [번역 가이드](maintain/translation-guide.md) |
| 수동 문서 점검 | [문서 점검](maintain/checks.md) |
| 안정적인 `doc_id` 경로 정보 | [doc-index.yaml](../doc-index.yaml) |

## 읽는 방법

먼저 [시작하기](start.md)를 읽습니다. 작업에 따라 [사용자 가이드](use/user-guide.md)나 [에이전트 가이드](use/agent-guide.md)를 이어서 봅니다. 정확한 기술 계약의 기준 담당 문서는 [참조 색인](reference/README.md)에서 고릅니다.

README와 경로 문서는 요약하고 안내해야 합니다. API, 스키마, 저장소, 보안, 상태 보기, 템플릿, 용어, 현재 MVP 범위 계약을 정의하면 안 됩니다.

현재 MVP 밖의 자료는 [이후 후보 색인](later/index.md)에서 봅니다. 이후 후보 자료는 관련 담당 문서가 범위와 증명 기대를 함께 승격하기 전까지 활성 전달 범위가 아닙니다.

문서 작업에는 [작성 가이드](maintain/authoring-guide.md), [번역 가이드](maintain/translation-guide.md), [문서 점검](maintain/checks.md)을 사용합니다. 문서 점검은 수동 유지보수 보조 자료입니다. 점검 라벨은 런타임 적합성, 최종 수락, 닫기 준비 상태, 구현 준비, 서버 코딩 시작 허가를 만들지 않습니다.

## 한영 문서 동시 유지

영어와 한국어 문서는 모두 활성 문서입니다. 주요 활성 문서는 `docs/en`과 `docs/ko` 아래에 대응 경로를 가져야 합니다. 영어 진입점은 [../en/README.md](../en/README.md)입니다.

대응 문서는 의미 일치를 유지해야 하지만 줄 단위 번역일 필요는 없습니다. 한국어 문서는 자연스러운 한국어 기술 문장으로 쓰고 정확한 식별자는 그대로 보존합니다.

에이전트는 작은 현재 맥락을 유지하고 필요한 담당 문서만 불러와야 합니다. 번역이나 의미 일치 검토가 필요한 경우가 아니면 같은 `doc_id`의 영어/한국어 문서를 한 프롬프트에 함께 넣지 않습니다.
