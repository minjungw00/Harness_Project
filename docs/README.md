# Harness Documentation / 하네스 문서

This directory contains the active bilingual documentation set for a future local Harness Server. The repository is documentation-only today. It is not a running Harness instance, not the user's Product Repository, and not a Harness Runtime Home.

이 디렉터리는 향후 로컬 하네스 서버를 위한 현재 한영 문서 세트를 담고 있습니다. 이 저장소는 현재 문서 전용입니다. 실행 중인 하네스 인스턴스도, 사용자의 Product Repository도, Harness Runtime Home도 아닙니다.

Harness documentation is planning source material. It is not runtime state, generated projections, evidence, QA, final acceptance, residual-risk records, close records, server code, or product code.

하네스 문서는 계획을 위한 원천 자료입니다. 런타임 상태, 생성된 상태 보기, 증거, QA, 최종 수락, 잔여 위험 기록, 닫기 기록, 서버 코드, 제품 코드가 아닙니다.

## Choose A Language / 언어 선택

| Language / 언어 | Entry / 진입점 |
|---|---|
| English | [en/README.md](en/README.md) |
| 한국어 | [ko/README.md](ko/README.md) |

## Current Routes / 현재 경로

English and Korean docs are both active. Every major active doc should have a paired path. Keep semantic parity across paired docs; line-by-line translation is not required.

영어와 한국어 문서는 모두 활성 문서입니다. 주요 활성 문서에는 대응 경로가 있어야 합니다. 대응 문서는 의미 일치를 유지합니다. 줄 단위 번역은 요구하지 않습니다.

| Purpose / 목적 | English | 한국어 |
|---|---|---|
| Start / 시작 | [Start](en/start.md) | [시작하기](ko/start.md) |
| User guide / 사용자 가이드 | [User Guide](en/use/user-guide.md) | [사용자 가이드](ko/use/user-guide.md) |
| Agent guide / 에이전트 가이드 | [Agent Guide](en/use/agent-guide.md) | [에이전트 가이드](ko/use/agent-guide.md) |
| Judgment examples / 판단 예시 | [Judgment Examples](en/use/judgment-examples.md) | [판단 예시](ko/use/judgment-examples.md) |
| Surface recipes / 접점별 사용법 | [Surface Recipes](en/use/surface-recipes.md) | [접점별 사용법](ko/use/surface-recipes.md) |
| Current MVP plan / 현재 MVP 계획 | [MVP Plan](en/build/mvp-plan.md) | [MVP 계획](ko/build/mvp-plan.md) |
| Contract owner index / 계약 담당 문서 색인 | [Reference Index](en/reference/README.md) | [참조 색인](ko/reference/README.md) |
| Later candidates / 이후 후보 | [Later Index](en/later/index.md) | [이후 후보 색인](ko/later/index.md) |
| Authoring guide / 작성 가이드 | [Authoring Guide](en/maintain/authoring-guide.md) | [작성 가이드](ko/maintain/authoring-guide.md) |
| Translation guide / 번역 가이드 | [Translation Guide](en/maintain/translation-guide.md) | [번역 가이드](ko/maintain/translation-guide.md) |
| Checks / 문서 점검 | [Checks](en/maintain/checks.md) | [문서 점검](ko/maintain/checks.md) |
| Route index / 경로 색인 | [doc-index.yaml](doc-index.yaml) | [doc-index.yaml](doc-index.yaml) |

## Reader Guidance / 독자 안내

Use `start.md` for the first model, `use/*` for user and agent behavior, `build/mvp-plan.md` for implementation-readiness decisions, `reference/README.md` for exact contract owners, `later/index.md` for later-only candidate material, `maintain/*` for documentation work, and `doc-index.yaml` for stable `doc_id` routing metadata.

첫 이해 모델은 `start.md`에서 봅니다. 사용자와 에이전트 동작은 `use/*`, 구현 준비 결정은 `build/mvp-plan.md`, 정확한 계약의 담당 문서는 `reference/README.md`, 이후 전용 후보 자료는 `later/index.md`, 문서 작업 규칙은 `maintain/*`, 안정적인 `doc_id` 경로 정보는 `doc-index.yaml`에서 봅니다.

README and route documents do not own technical contracts. Add or change normative API, storage, security, projection, template, terminology, or current-MVP-scope content in the canonical owner selected through the Reference Index.

README와 경로 문서는 기술 계약을 담당하지 않습니다. API, 저장소, 보안, 상태 보기, 템플릿, 용어, 현재 MVP 범위의 규범 내용을 추가하거나 바꿀 때는 참조 색인에서 고른 기준 담당 문서에 반영합니다.
