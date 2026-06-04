# Reference 색인

Reference는 schema, gate, 상태 전이, DDL profile, 읽기용 요약(Projection) 규칙, template body, 보안 의미, conformance 규칙, connector 동작, policy, 용어의 정확한 owner 계약이 필요할 때 사용합니다.

이 owner 문서들은 향후 하네스 서버 계약을 계획하고 검토하기 위한 문서입니다. 지금 이 저장소에 서버/런타임, Harness Runtime Home, conformance runner, 생성된 읽기용 요약 시스템, 구현이 있다는 뜻이 아닙니다.

Reference 전체를 기본으로 읽지 않습니다. 지금 앞에 있는 질문의 owner를 고른 뒤, 그 owner가 더 엄격한 세부사항을 위임할 때만 링크를 따라갑니다.

## Owner-Contract 지도

| 계약 영역 | Owner |
|---|---|
| Task, scope/Change Unit, `user_judgment`, `evidence_ref`, blocker와 닫기 준비 상태의 의미, gate, 상태 전이, `prepare_write`, `record_run`, `close_task`, 상태 불변 조건, 대체 불가능한 경계 | [Core Model 참조](core-model.md) |
| Active MVP-1 public method와 method별 request/response 동작 | [MVP API](api/mvp-api.md) |
| MVP-1 shared schema, envelope, read-only resource, ref, `ArtifactRef`, `ValidatorResult`, 단계별 active value set, API가 소유한 enum | [API Schema Core](api/schema-core.md) |
| Error taxonomy, 사용자에게 보이는 error label, primary error precedence, close-blocker error mapping, idempotency, state conflict behavior | [API Errors](api/errors.md) |
| Later/profile-gated API method, schema branch, enum extension, future validator ID | [API Schema Later](api/schema-later.md). 이후 독자 경로는 [보증 프로필](../later/assurance-profile.md)을 사용합니다. |
| Runtime home layout, persisted state model, DDL profile, storage-owned JSON `TEXT`, artifact storage, migration, lock, baseline capture, projection-job storage, validator-run storage | [Storage](storage.md) |
| 파생 보기, 상태 카드, 에이전트 맥락 패킷, managed block, 사람이 편집할 수 있는 projection section, template implementation class, artifact-ref rendering, freshness/failure behavior | [Projection과 Template 참조](projection-and-templates.md) |
| 전체 rendered template body와 display card shape | [Template 참조](templates/README.md) |
| Guarantee level, threat model, asset, trust boundary, threat/control category, 정직한 보안 표현 | [보안 참조](security.md) |
| Agent가 context를 과하게 싣지 않고 Core/API와 상호작용하는 방법: connector profile, generated manifest, context push/pull, fallback behavior, Role Lens, reference-surface behavior | [Agent 통합 참조](agent-integration.md). [Surface Cookbook](surface-cookbook.md)은 surface recipe를 담당합니다. |
| Operator behavior, diagnostic, conformance run entrypoint, recovery/export/reconcile operation, docs-maintenance reporting | [운영과 Conformance 참조](operations-and-conformance.md). 이후 독자 경로는 [운영 프로필](../later/operations-profile.md)을 사용합니다. |
| Fixture body shape, runner behavior, assertion semantics, fixture profile, suite metadata boundary, current-phase fixture status, 축소된 Kernel Smoke queue | [Conformance Fixtures 참조](conformance-fixtures.md) |
| 간결한 향후 scenario family 목록, 승격 조건, suite-family label, catalog-only candidate | [향후 Fixtures](../later/future-fixtures.md) |
| Design-quality policy, validator ID, severity composition, waiver semantics, evidence expectation, close impact | [설계 품질 정책](design-quality-policies.md) |
| Public/internal terminology definition, capitalization, record-name orientation, owner routing | [용어집 참조](glossary.md) |
| Runtime space, Core process placement, Core-only mutation authority, transaction ordering, artifact, projection/reconcile placement, recovery overview | [런타임 아키텍처 참조](runtime-architecture.md) |

## 독자별 바로가기

- 향후 서버 구현자는 [구현 개요](../build/implementation-overview.md)에서 시작한 뒤 [MVP-1 사용자 작업 루프](../build/mvp-user-work-loop.md) -> [MVP API](api/mvp-api.md) -> [Storage](storage.md) 순서로 봅니다. 다른 Reference owner는 정확한 질문이 있을 때만 엽니다.
- 첫 내부 점검을 계획한다면 [내부 엔지니어링 점검](../build/engineering-checkpoint.md)에서 시작한 뒤 [Core Model 참조](core-model.md), [MVP API](api/mvp-api.md), [Storage](storage.md)를 사용합니다.
- 에이전트 지침을 작성한다면 [에이전트 가이드](../use/agent-guide.md)에서 시작한 뒤 connector-specific 계약이 필요할 때만 [Agent 통합 참조](agent-integration.md)와 [Surface Cookbook](surface-cookbook.md)을 사용합니다.
- MVP-1 method를 확인한다면 [MVP API](api/mvp-api.md)에서 시작합니다. Shared ref나 envelope를 확인한다면 [API Schema Core](api/schema-core.md)를 사용합니다. Later method는 [API Schema Later](api/schema-later.md)를 사용하되, 승격 전에는 MVP 경로에 넣지 않습니다.
- Persisted shape를 확인한다면 [Storage](storage.md)에서 시작합니다.
- `harness://` resource를 확인한다면 URI를 delivery stage requirement로 취급하기 전에 staged [Read-only resources](api/schema-core.md#read-only-resources) table에서 시작합니다.
- 사용자에게 보이는 문구 주장을 확인한다면 그 밑의 사실을 담당하는 owner에서 시작합니다. 읽기용 요약(Projection)과 template 문서는 표시를 담당하지만 권한을 만들지 않습니다.
- 향후 보증, 운영, fixture catalog material을 확인한다면 [보증 프로필](../later/assurance-profile.md), [운영 프로필](../later/operations-profile.md), [향후 Fixtures](../later/future-fixtures.md)를 사용합니다. 이 경로는 MVP 구현 경로가 아닙니다.
