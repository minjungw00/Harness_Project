# Harness Project / 하네스 프로젝트

Harness is planned as a local work-authority server for AI-assisted product work. It is not a prompt pack, operating-system permission control, arbitrary-tool sandbox, tamper-proof store, default pre-tool blocker, or security isolation.

하네스는 AI 지원 제품 작업을 위한 향후 로컬 작업 권한 서버로 계획되어 있습니다. 프롬프트 묶음, 운영체제 권한 제어, 임의 도구 샌드박스, 변조 방지 저장소, 기본 도구 실행 전 차단, 보안 격리가 아닙니다.

## Repository State / 저장소 상태

This repository is documentation-only today. It contains bilingual planning source material for a future Harness Server. It has no server/runtime implementation, product implementation code, runtime state, generated projections, generated operational artifacts, executable fixtures, or conformance runner.

이 저장소는 현재 문서 전용입니다. 향후 하네스 서버를 위한 한영 문서 동시 유지 계획 자료를 담고 있습니다. 서버/런타임 구현, 제품 구현 코드, 런타임 상태, 생성된 상태 보기, 생성된 운영 산출물, 실행 가능한 fixture, 적합성 실행기는 없습니다.

This repository is not the user's Product Repository and not a Harness Runtime Home. Documentation acceptance, when it happens, is a maintainer documentation milestone only. Server/runtime implementation still requires a separate readiness decision in the MVP plan.

이 저장소는 사용자의 Product Repository도, Harness Runtime Home도 아닙니다. 문서 수락이 이루어져도 유지보수자의 문서 이정표일 뿐입니다. 서버/런타임 구현에는 MVP 계획의 별도 준비 결정이 필요합니다.

## Current Routes / 현재 경로

Start at [docs/README.md](docs/README.md) to choose a language. English and Korean docs are both active, must keep semantic parity, and do not require line-by-line translation.

언어 선택은 [docs/README.md](docs/README.md)에서 시작합니다. 영어와 한국어 문서는 모두 활성 문서이며 의미 일치를 유지해야 합니다. 줄 단위 번역은 요구하지 않습니다.

| Need / 필요 | English | 한국어 |
|---|---|---|
| Start / 시작 | [docs/en/start.md](docs/en/start.md) | [docs/ko/start.md](docs/ko/start.md) |
| User work / 사용자 작업 | [docs/en/use/user-guide.md](docs/en/use/user-guide.md) | [docs/ko/use/user-guide.md](docs/ko/use/user-guide.md) |
| Agent behavior / 에이전트 동작 | [docs/en/use/agent-guide.md](docs/en/use/agent-guide.md) | [docs/ko/use/agent-guide.md](docs/ko/use/agent-guide.md) |
| Judgment examples / 판단 예시 | [docs/en/use/judgment-examples.md](docs/en/use/judgment-examples.md) | [docs/ko/use/judgment-examples.md](docs/ko/use/judgment-examples.md) |
| Surface recipes / 접점별 사용법 | [docs/en/use/surface-recipes.md](docs/en/use/surface-recipes.md) | [docs/ko/use/surface-recipes.md](docs/ko/use/surface-recipes.md) |
| Current MVP plan / 현재 MVP 계획 | [docs/en/build/mvp-plan.md](docs/en/build/mvp-plan.md) | [docs/ko/build/mvp-plan.md](docs/ko/build/mvp-plan.md) |
| Contract owner index / 계약 담당 문서 색인 | [docs/en/reference/README.md](docs/en/reference/README.md) | [docs/ko/reference/README.md](docs/ko/reference/README.md) |
| Later candidates / 이후 후보 | [docs/en/later/index.md](docs/en/later/index.md) | [docs/ko/later/index.md](docs/ko/later/index.md) |
| Authoring rules / 작성 규칙 | [docs/en/maintain/authoring-guide.md](docs/en/maintain/authoring-guide.md) | [docs/ko/maintain/authoring-guide.md](docs/ko/maintain/authoring-guide.md) |
| Translation rules / 번역 규칙 | [docs/en/maintain/translation-guide.md](docs/en/maintain/translation-guide.md) | [docs/ko/maintain/translation-guide.md](docs/ko/maintain/translation-guide.md) |
| Documentation checks / 문서 점검 | [docs/en/maintain/checks.md](docs/en/maintain/checks.md) | [docs/ko/maintain/checks.md](docs/ko/maintain/checks.md) |
| Route index / 경로 색인 | [docs/doc-index.yaml](docs/doc-index.yaml) | [docs/doc-index.yaml](docs/doc-index.yaml) |

## Ownership / 담당 문서

Use the [Reference Index](docs/en/reference/README.md) / [참조 색인](docs/ko/reference/README.md) to choose the one canonical owner for exact technical contracts. README and route documents should summarize and route, not define API, storage, security, projection, template, or MVP-scope contracts.

정확한 기술 계약의 기준 담당 문서는 [Reference Index](docs/en/reference/README.md) / [참조 색인](docs/ko/reference/README.md)에서 고릅니다. README와 경로 문서는 API, 저장소, 보안, 상태 보기, 템플릿, 현재 MVP 범위 계약을 정의하지 않고 요약과 경로 안내만 해야 합니다.

Future agents should follow [AGENTS.md](AGENTS.md) before editing. Keep context small, pull owner docs only when needed, and do not load paired English/Korean docs for the same `doc_id` in one prompt unless translation or semantic-parity review requires it.

향후 에이전트는 편집 전에 [AGENTS.md](AGENTS.md)를 따라야 합니다. 작은 맥락을 유지하고 필요한 담당 문서만 불러오며, 번역이나 의미 일치 검토가 필요한 경우가 아니면 같은 `doc_id`의 영어/한국어 문서를 한 프롬프트에 함께 넣지 않습니다.
