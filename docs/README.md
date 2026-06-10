# Harness documentation / 하네스 문서

This directory is the active bilingual documentation set for Harness planning. It routes readers and agents to the right owner documents; it is not a contract reference, runtime state, generated output, or implementation record.

이 디렉터리는 하네스 계획을 위한 현재 한영 문서 세트입니다. 이 문서는 사람과 에이전트가 알맞은 담당 문서를 찾도록 안내합니다. 계약 참조 문서, 런타임 상태, 생성 결과, 구현 기록이 아닙니다.

## Choose a language / 언어 선택

English and Korean are both active documentation languages. Choose the language you want to read first, then stay in that language unless you are checking translation parity.

영어와 한국어 문서는 모두 활성 문서입니다. 먼저 읽을 언어를 고르고, 번역 의미 일치를 확인하는 경우가 아니라면 같은 언어 안에서 읽습니다.

- English: [`docs/en/start.md`](en/start.md)
- 한국어: [`docs/ko/start.md`](ko/start.md)

## Choose a reading path / 읽기 경로 선택

| Reader             | English                               | Korean                                |
| ------------------ | ------------------------------------- | ------------------------------------- |
| New user           | `docs/en/start.md`                    | `docs/ko/start.md`                    |
| Working user       | `docs/en/use/user-guide.md`           | `docs/ko/use/user-guide.md`           |
| Agent behavior     | `docs/en/use/agent-guide.md`          | `docs/ko/use/agent-guide.md`          |
| Technical contract | `docs/en/reference/README.md`         | `docs/ko/reference/README.md`         |
| Maintenance        | `docs/en/maintain/authoring-guide.md` | `docs/ko/maintain/authoring-guide.md` |

## Reference owner routing / 참조 담당 문서 찾기

Use the reference README to find the canonical owner for API, schema, storage, security, scope, and other contract details. This router intentionally does not repeat those contracts.

API, 스키마, 저장소, 보안, 범위, 그 밖의 계약 세부사항은 참조 README에서 담당 문서를 찾아 읽습니다. 이 안내 문서는 그런 계약 내용을 반복하지 않습니다.

- English reference index: [`docs/en/reference/README.md`](en/reference/README.md)
- Korean reference index: [`docs/ko/reference/README.md`](ko/reference/README.md)
- Active MVP scope: [`docs/en/reference/active-mvp-scope.md`](en/reference/active-mvp-scope.md), [`docs/ko/reference/active-mvp-scope.md`](ko/reference/active-mvp-scope.md)
- LLM owner routing metadata: [`docs/doc-index.yaml`](doc-index.yaml)
- Bilingual terminology controls: [`docs/terminology-map.yaml`](terminology-map.yaml)

## Maintenance documents / 유지보수 문서

Use maintain documents for documentation editing rules, bilingual practice, and checks. They guide documentation work; they do not own runtime or technical contracts.

유지보수 문서는 문서 편집 규칙, 한영 문서 동시 유지 방식, 점검 절차를 다룹니다. 문서 작업을 안내할 뿐, 런타임이나 기술 계약을 담당하지 않습니다.

- English authoring: [`docs/en/maintain/authoring-guide.md`](en/maintain/authoring-guide.md)
- Korean authoring: [`docs/ko/maintain/authoring-guide.md`](ko/maintain/authoring-guide.md)
- English translation guide: [`docs/en/maintain/translation-guide.md`](en/maintain/translation-guide.md)
- Korean translation guide: [`docs/ko/maintain/translation-guide.md`](ko/maintain/translation-guide.md)
- Documentation checks: [`docs/en/maintain/checks.md`](en/maintain/checks.md), [`docs/ko/maintain/checks.md`](ko/maintain/checks.md)

## Agent retrieval rule / 에이전트 검색 규칙

Agents should read only one language version of the same `doc_id` unless they are checking translation parity. Use [`docs/doc-index.yaml`](doc-index.yaml) to choose the owner route before loading reference content.

에이전트는 번역 의미 일치를 확인하는 경우가 아니라면 같은 `doc_id`의 한 언어 버전만 읽어야 합니다. 참조 내용을 불러오기 전에 [`docs/doc-index.yaml`](doc-index.yaml)에서 담당 경로를 먼저 고릅니다.
