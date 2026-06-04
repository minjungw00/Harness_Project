# 이후: 보증 프로필

이 문서는 보증 프로필의 향후 hardening 내용을 MVP 구현 경로에 섞지 않고 찾아가도록 돕습니다.

향후 하네스 동작을 위한 계획 및 길잡이 문서입니다. 이 저장소에서 런타임/서버 구현, 생성된 운영 파일, 실행 가능한 fixture, 런타임 데이터, 제품 코드를 허가하지 않습니다.

## 이런 때 읽기

- MVP-1 사용자 작업 루프 이후에 무엇이 속하는지 확인할 때.
- 검증, 수동 QA, 작업 수락, 잔여 위험, 민감 동작 승인, stewardship, context hygiene를 서로 구분해야 할 때.
- 보증 관련 계약의 담당 문서를 찾아야 할 때.

## 읽는 경로

먼저 [MVP-1 사용자 작업 루프](../build/mvp-user-work-loop.md)에서 MVP 경계를 확인합니다. 그다음 필요한 질문의 담당 문서만 엽니다.

| 필요한 것 | 담당 문서 |
|---|---|
| Core gate, 사용자 판단, 닫기, waiver, 작업 수락, 잔여 위험 의미 | [Core Model 참조](../reference/core-model.md) |
| 이후/profile-gated API method와 schema material | [API Schema Later](../reference/api/schema-later.md) |
| 설계 품질 정책, validator ID, severity composition, waiver 영향 | [설계 품질 정책](../reference/design-quality-policies.md) |
| Fixture mechanics와 profile 증명 모델 | [Conformance Fixtures 참조](../reference/conformance-fixtures.md) |
| 향후 보증 scenario 후보 | [향후 Fixtures](future-fixtures.md) |
| 보증 report의 읽기용 표시 경계 | [Projection과 Template 참조](../reference/projection-and-templates.md)와 [Template 참조](../reference/templates/README.md) |

## 경계

보증 프로필은 MVP-1 이후 범위입니다. 검증, QA, 작업 수락, 잔여 위험, 민감 동작 승인, stewardship, context hygiene를 더 단단하게 만들 수 있습니다. 하지만 첫 사용자 가치 경로가 아니고, 운영/export/recover 프로필도 아닙니다.

Dashboard, hosted workflow UI, broad connector automation, team workflow, orchestration, preventive security, isolation은 owner가 구체적인 mechanism을 승격하고 증명하기 전까지 로드맵 후보로 남습니다.
