# 99. Authoring Guide

## 1. 문서 역할

이 문서는 하네스 문서 세트의 작성과 개정 기준을 정의한다. 문서 층위, 문제/목표/수단 구분, 현재 상태 중심 서술, 용어 안정성, source-of-truth 표현, 중복 서술 처리, drift 점검, 확정/제안/보류/폐기 구분을 소유한다.

하네스 기능 계약과 사용자 절차는 각 기준 문서가 소유한다.

## 2. 문서 층위

| 문서 | 층위 |
|---|---|
| `01-project-charter.md` | 프로젝트 목표와 상위 기준 |
| `02-strategy.md` | 개념 모델과 전략적 불변식 |
| `03-architecture.md` | 로컬 배치와 런타임 구조 |
| `04-reference-implementation.md` | 구현 계약 |
| `05-user-guide.md` | 사용자 절차 |
| `06-agent-integration.md` | agent surface 통합 기준 |
| `07-document-and-artifact-contracts.md` | 문서와 artifact projection 계약 |
| `08-operations-and-conformance.md` | 운영과 검증 기준 |
| `09-design-quality-playbooks.md` | 설계 품질 playbook |
| `glossary.md` | 공통 용어 |

문서 작성자는 현재 내용의 소유 문서를 먼저 확인한다.

## 3. 현재 상태 직접 서술

기준 문서 본문은 현재 기준과 현재 구조를 직접 설명한다.

권장:

```text
하네스는 여러 축의 상태를 사용한다.
approval은 명시적 사용자 결정을 기록하는 별도 artifact로 둔다.
```

피할 표현 유형:

```text
버전 비교 중심 표현
작성 경위 중심 표현
이전 초안 대비 표현
부정형 대조 표현
```

버전 비교, 폐기 이유, 변경 전후 차이는 개정 이력, DEC, 설계 메모, migration note에 둔다.

## 4. 문제, 목표, 수단 분리

문서에는 다음이 구분되어야 한다.

- 어떤 문제가 있는가
- 무엇을 목표로 하는가
- 그 목표를 위해 어떤 수단을 쓰는가

상위 문서에서는 목적과 원칙을 먼저 적는다. 구현안은 구현 문서로 보낸다. 예시는 이해를 돕는 보조 자료로 사용한다.

## 5. 목표 연결 원칙

중요한 수정은 프로젝트 목표와 연결한다.

점검 질문:

- 어떤 목표를 강화하는가
- 어떤 목표를 약화할 수 있는가
- 사용자 가시성에 어떤 영향을 주는가
- 품질 통제에 어떤 영향을 주는가
- 설계 품질에 어떤 영향을 주는가
- 프로젝트 비종속성에 어떤 영향을 주는가

## 6. 층위 분리 원칙

상위 기준, 전략, 아키텍처, 구현, 사용 절차, 템플릿, 운영은 서로 다른 층위다.

예시:

- “work는 detached verify 없이 닫지 않는다”는 전략 불변식이다.
- “기능 Change Unit은 vertical slice를 기본값으로 둔다”는 전략 불변식이다.
- `harness.record_run`의 입력 schema는 구현 계약이다.
- `TDD-TRACE` 문서의 front matter는 문서 계약이다.
- `TDD trace 기준으로 진행해`는 사용자 가이드 문장이다.

한 문서에 다른 층위 내용을 넣을 수는 있지만, 기준의 소유 위치를 흐리지 않는다.

## 7. 중복 서술 처리 원칙

같은 개념의 canonical 설명은 한 문서에 둔다. 다른 문서는 한 문장 요약과 참조를 둔다.

권장 소유 위치:

| 개념 | 기준 설명 위치 |
|---|---|
| 하네스 목적 | `01-project-charter.md` |
| 불변식과 상태 모델 | `02-strategy.md` |
| 세 공간과 런타임 흐름 | `03-architecture.md` |
| 구현 계약과 state/MCP | `04-reference-implementation.md` |
| 사용자 절차 | `05-user-guide.md` |
| agent 통합 | `06-agent-integration.md` |
| 문서 템플릿 | `07-document-and-artifact-contracts.md` |
| 운영과 conformance | `08-operations-and-conformance.md` |
| 설계 품질 playbook | `09-design-quality-playbooks.md` |
| 작성 규칙 | `99-authoring-guide.md` |
| 용어 정의 | `glossary.md` |

중복을 남길 수 있는 경우:

- 사용자에게 반복 노출되어야 하는 핵심 문장
- 문서 독립성을 위해 필요한 한 문장 요약
- 예시가 해당 문서 이해에 직접 필요한 경우

## 8. Source-of-truth 표현 기준

문서 세트의 기준 문장은 다음이다.

```text
운영 상태 전이의 canonical source는 state.sqlite와 event log다.
문서는 사람이 읽는 projection이다.
Artifact store는 raw evidence의 canonical source다.
Human-editable 문서 영역은 사용자 입력 표면이며, 상태 반영은 reconcile 또는 MCP tool을 통해 수행한다.
```

`TASK`는 현재 continuity를 읽는 중심 projection이다. 이 표현은 `TASK`의 중요성과 운영 상태의 canonical source를 동시에 보존한다.

## 9. 용어 안정성 원칙

공식 용어는 `glossary.md`에 정의한다. 제품별 용어는 `DOMAIN-LANGUAGE`에 정의한다.

같은 단어를 문맥에 따라 다른 의미로 쓰지 않는다. 새 용어를 만들거나 기존 용어의 뜻을 바꿀 때는 문서 전체 영향을 확인한다.

첫 등장 방식:

```text
Change Unit은 실제 구현 단위다.
```

이후에는 `Change Unit`이라고 쓴다.

## 10. 합의 상태 명시

확정, 제안, 보류, 폐기를 구분한다.

권장 라벨:

```text
[배경]
[목표]
[원칙]
[제안]
[결정]
[보류]
[폐기]
```

라벨은 문장의 성격을 식별하는 보조 장치다.

## 11. 논쟁 분리 원칙

해결되지 않은 쟁점은 기준 문서 본문에서 얼버무리지 않는다. 선택지는 다음처럼 분리한다.

- 선택지
- 장점
- 비용
- 리스크
- 보류 이유
- 결정 조건

종료된 논쟁은 `DEC` 문서로 내리고, 기준 문서에는 현재 합의된 기준을 쓴다.

## 12. 도구 종속 표현 절제

특정 도구 이름, 파일 경로, 명령 예시, 제품 기능명은 필요한 위치에서만 쓴다.

상위 문서는 왜 그 선택이 필요한지를 중심으로 서술한다. 도구별 세부는 통합 문서 또는 운영 문서에서 다룬다.

## 13. 문장을 과장하지 않기

확인되지 않은 효과를 이미 확보된 장점처럼 쓰지 않는다.

구분한다.

- 기대 효과
- 설계 의도
- 실제 검증 결과

## 14. Design-quality 문서 기준

설계 품질 문서는 다음 원칙을 따른다.

- Shared design은 문서 하나로 완전히 대체하지 않는다.
- Grill Protocol은 질문과 결정을 남기는 절차다.
- Domain Language는 하네스 운영 용어가 아니라 제품별 도메인 용어를 다룬다.
- Module Map은 구현 파일 목록이 아니라 책임, interface, test boundary를 다룬다.
- Interface Contract는 public interface 변경에 집중한다.
- TDD Trace는 테스트 존재 여부가 아니라 red/green/refactor 근거를 다룬다.
- Manual QA는 acceptance와 다르다.
- 완료된 PRD/DESIGN/issue는 기본 context에 계속 push하지 않는다.

## 15. 재작성 허용 원칙

전략 문서와 설계 문서는 필요하면 완전히 다시 쓸 수 있다. 전면 재작성은 기존 문서를 버리는 행위가 아니라, 무엇을 보존하고 무엇을 폐기하는지 명시하는 설계 작업이다.

## 16. 드리프트 점검 원칙

큰 수정이나 전면 재구성은 다음을 확인한다.

- 무엇이 바뀌는가
- 무엇을 유지해야 하는가
- 왜 지금 이 변경이 필요한가
- 핵심 목표를 흐리게 만들 가능성은 있는가
- 사용자 가시성, 판단 기회, 프로젝트 비종속성, 품질 통제, 설계 품질에 어떤 영향을 주는가

## 17. 문서 개정 체크리스트

```text
[ ] 변경 범위가 명확한가
[ ] 변경이 해결하는 문제가 명확한가
[ ] 상위 목표와 연결되는가
[ ] 문서 층위가 맞는가
[ ] source-of-truth 표현이 흔들리지 않는가
[ ] approval/assurance/manual QA/acceptance 의미가 유지되는가
[ ] work detached verify 불변식이 약화되지 않는가
[ ] shared design, domain language, vertical slice, TDD, module/interface, manual QA 원칙이 충돌하지 않는가
[ ] agent surface를 source-of-truth처럼 쓰지 않는가
[ ] 사용자 일상 작업이 CLI 조합으로 바뀌지 않는가
[ ] 용어가 glossary와 일치하는가
[ ] 제품별 용어는 DOMAIN-LANGUAGE로 분리되는가
[ ] 오래된 문서를 기본 context에 push하지 않는가
[ ] 같은 개념의 canonical 설명이 한 문서에만 있는가
```

## 18. 요약

```text
문제와 목표를 먼저 분리한다.
문서 층위를 지킨다.
현재 상태를 직접 서술한다.
중복 설명의 소유 위치를 고정한다.
source-of-truth와 projection을 혼동하지 않는다.
용어를 안정적으로 유지한다.
설계 품질 원칙을 운영 계약과 분리하지 않는다.
```

