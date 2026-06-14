# Glossary

This glossary is the human-readable term guide for Harness documentation. Use it to understand a term and find the single focused owner to read next.

Structured terminology metadata, identifier-preservation controls, and Korean mixed-language controls live in [docs/terminology-map.yaml](../../terminology-map.yaml). Exact API behavior, schemas, storage effects, security guarantees, close-readiness behavior, and error routing live in the linked owner documents.

## Terms

### Harness

- Term: Harness
- Korean term: 하네스
- Meaning: The local work-authority server for AI-assisted product work.
- Primary owner: [Scope](scope.md)
- See also: [Runtime Boundaries](runtime-boundaries.md)

### Product Repository

- Term: Product Repository
- Korean term: Product Repository; 제품 저장소 in user-facing prose.
- Meaning: The user's project workspace, separate from Harness runtime state.
- Primary owner: [Runtime Boundaries](runtime-boundaries.md)

### Harness Runtime Home

- Term: Harness Runtime Home
- Korean term: Harness Runtime Home; 런타임 홈 in user-facing prose.
- Meaning: The operational data space for Harness records and artifacts.
- Primary owner: [Runtime Boundaries](runtime-boundaries.md)

### documentation

- Term: documentation
- Korean term: 문서
- Meaning: Maintained source material, separate from runtime output, product implementation, and acceptance state.
- Primary owner: [Authoring Guide](../maintain/authoring-guide.md)
- See also: [Runtime Boundaries](runtime-boundaries.md), [Implementation Guide](../build/implementation-guide.md)

### semantic skeleton

- Term: semantic skeleton
- Korean term: 의미 골격
- Meaning: The planned meaning-unit structure for an important Reference section.
- Primary owner: [Authoring Guide](../maintain/authoring-guide.md)
- See also: [Structure checks](../maintain/checks/structure.md)

### baseline scope

- Term: baseline scope
- Korean term: 기준 범위
- Meaning: The stable support boundary documented for Harness.
- Primary owner: [Scope](scope.md)
- See also: [API Value Sets](api/schema-value-sets.md)

### supported scope

- Term: supported scope
- Korean term: 지원 범위
- Meaning: A capability or behavior documented as supported.
- Primary owner: [Scope](scope.md)

### supported behavior

- Term: supported behavior
- Korean term: 지원 동작
- Meaning: Behavior documented as supported by Scope and the affected semantic owner.
- Primary owner: [Scope](scope.md)
- See also: [API Value Sets](api/schema-value-sets.md)

### supported API method

- Term: supported API method
- Korean term: 지원되는 API 메서드
- Meaning: A public API method documented as supported.
- Primary owner: [API Methods](api/methods.md)

### supported API value

- Term: supported API value
- Korean term: 지원되는 API 값
- Meaning: An API value documented as supported, rather than only reserved or named.
- Primary owner: [API Value Sets](api/schema-value-sets.md)
- See also: [Scope](scope.md)

### out-of-scope capability

- Term: out-of-scope capability
- Korean term: 지원 범위 밖 기능
- Meaning: A deferred capability outside the baseline support boundary until promoted by the applicable owners.
- Primary owner: [Scope](scope.md)

### evidence collection workflow

- Term: evidence collection workflow
- Korean term: 증거 수집 흐름
- Meaning: Wording for an evidence-workflow capability whose support status belongs to Scope.
- Primary owner: [Scope](scope.md)

### expanded or additional evidence collection workflows

- Term: expanded or additional evidence collection workflows
- Korean term: 확장 또는 추가 증거 수집 흐름
- Meaning: An out-of-scope family of evidence collection workflows until Scope and affected owners promote it.
- Primary owner: [Scope](scope.md)

### owner document

- Term: owner document
- Korean term: 담당 문서
- Meaning: The canonical document that defines a term, product concept, or contract.
- Primary owner: [Authoring Guide](../maintain/authoring-guide.md)
- See also: [Reference Index](README.md)

### owner contract

- Term: owner contract
- Korean term: 담당 계약
- Meaning: The contract defined by the relevant owner document.
- Primary owner: [Authoring Guide](../maintain/authoring-guide.md)

### applicable owner path

- Term: applicable owner path
- Korean term: 적용되는 담당 경로
- Meaning: The documentation route to the focused owner for a question or concept.
- Primary owner: [Authoring Guide](../maintain/authoring-guide.md)
- See also: [Reference Index](README.md), [doc-index.yaml](../../doc-index.yaml)

### applicable reference

- Term: applicable reference
- Korean term: 적용되는 참조 문서
- Meaning: The reference page that defines the relevant contract.
- Primary owner: [Reference Index](README.md)
- See also: [Authoring Guide](../maintain/authoring-guide.md)

### existing owner

- Term: existing owner
- Korean term: 기존 담당 문서
- Meaning: A canonical owner that already exists and can carry normative meaning.
- Primary owner: [Authoring Guide](../maintain/authoring-guide.md)
- See also: [Reference Index](README.md)

### promotion-time owner update

- Term: promotion-time owner update
- Korean term: 승격 시점의 담당 문서 갱신
- Meaning: Owner changes required when an out-of-scope capability is promoted into support.
- Primary owner: [Scope](scope.md)
- See also: [Authoring Guide](../maintain/authoring-guide.md)

### owner placeholder

- Term: owner placeholder
- Korean term: 담당 문서 자리표시자
- Meaning: A marker that an out-of-scope capability needs an owner created or designated before support.
- Primary owner: [Authoring Guide](../maintain/authoring-guide.md)
- See also: [Scope](scope.md)

### `Task`

- Term: `Task`
- Korean term: `Task`
- Meaning: The Harness entity that gathers work scope, authority context, judgments, evidence, and close-readiness state.
- Primary owner: [Core Model](core-model.md)
- See also: [API State Schemas](api/schema-state.md)

### scope

- Term: scope
- Korean term: 범위
- Meaning: The work or authority boundary attached to a `Task` or Change Unit context.
- Primary owner: [Core Model](core-model.md)
- See also: [Update-scope method](api/method-update-scope.md)

### active scope

- Term: active scope
- Korean term: 현재 적용 범위
- Meaning: The scope currently applied inside a `Task` or Change Unit context.
- Primary owner: [Core Model](core-model.md)
- See also: [Update-scope method](api/method-update-scope.md)

### active Change Unit

- Term: active Change Unit
- Korean term: 현재 적용 Change Unit
- Meaning: The Change Unit currently applied in the authority model.
- Primary owner: [Core Model](core-model.md)
- See also: [Update-scope method](api/method-update-scope.md)

### user-owned judgment

- Term: user-owned judgment
- Korean term: 사용자 소유 판단
- Meaning: A user-owned decision or assessment that Harness records without turning it into Core-owned fact.
- Primary owner: [Core Model](core-model.md)
- See also: [API Judgment Schemas](api/schema-judgment.md)

### `UserJudgment`

- Term: `UserJudgment`
- Korean term: `UserJudgment`
- Meaning: The API schema identifier for user-owned judgment data.
- Primary owner: [API Judgment Schemas](api/schema-judgment.md)
- See also: [Core Model](core-model.md)

### close readiness

- Term: close readiness
- Korean term: 닫기 준비 상태
- Meaning: The Core concept for whether a `Task` is ready to close from its current state.
- Primary owner: [Core Model](core-model.md)
- See also: [Close-task method](api/method-close-task.md), [API blocker routing](api/blocker-routing.md)

### close readiness evaluation

- Term: close readiness evaluation
- Korean term: 닫기 준비 상태 평가
- Meaning: The method-specific evaluation used by `harness.close_task`.
- Primary owner: [Close-task method](api/method-close-task.md)
- See also: [Core Model](core-model.md), [API blocker routing](api/blocker-routing.md)

### close task

- Term: close task
- Korean term: Task 닫기
- Meaning: The user or API action that attempts to close a `Task`.
- Primary owner: [Close-task method](api/method-close-task.md)
- See also: [API Methods](api/methods.md)

### close task behavior

- Term: close task behavior
- Korean term: Task 닫기 동작
- Meaning: Method-specific request, evaluation, and result behavior for closing a `Task`.
- Primary owner: [Close-task method](api/method-close-task.md)
- See also: [API Methods](api/methods.md)

### `harness.close_task`

- Term: `harness.close_task`
- Korean term: `harness.close_task`
- Meaning: The public API method identifier for close-task behavior.
- Primary owner: [Close-task method](api/method-close-task.md)
- See also: [API Methods](api/methods.md)

### close-readiness blocker

- Term: close-readiness blocker
- Korean term: 닫기 차단 사유
- Meaning: A reason surfaced when close readiness cannot proceed.
- Primary owner: [API blocker routing](api/blocker-routing.md)
- See also: [Core Model](core-model.md), [API State Schemas](api/schema-state.md)

### `CloseReadinessBlocker`

- Term: `CloseReadinessBlocker`
- Korean term: `CloseReadinessBlocker`
- Meaning: The schema identifier for close-readiness blocker data.
- Primary owner: [API State Schemas](api/schema-state.md)
- See also: [API blocker routing](api/blocker-routing.md)

### blocker category

- Term: blocker category
- Korean term: 차단 사유 범주
- Meaning: The category concept and value family for close-readiness blockers.
- Primary owner: [API Value Sets](api/schema-value-sets.md)
- See also: [API State Schemas](api/schema-state.md), [API blocker routing](api/blocker-routing.md)

### blocker

- Term: blocker
- Korean term: 차단 사유
- Meaning: A general prose term for a blocking reason.
- Primary owner: [Terminology Map](../../terminology-map.yaml)
- See also: [API blocker routing](api/blocker-routing.md)

### complete intent

- Term: complete intent
- Korean term: `complete`
- Meaning: The `complete` intent value, distinct from ordinary prose meaning "full" or "entire".
- Primary owner: [API Value Sets](api/schema-value-sets.md)
- See also: [Close-task method](api/method-close-task.md)

### full evaluation order

- Term: full evaluation order
- Korean term: 전체 평가 순서
- Meaning: Ordinary prose for the entire evaluation order, not the `complete` value.
- Primary owner: [Translation Guide](../maintain/translation-guide.md)
- See also: [Terminology Map](../../terminology-map.yaml)

### artifact

- Term: artifact
- Korean term: 아티팩트
- Meaning: A Harness artifact concept used for referenced or staged work material.
- Primary owner: [API Artifact Schemas](api/schema-artifacts.md)
- See also: [Artifact Storage](storage-artifacts.md)

### evidence

- Term: evidence
- Korean term: 증거
- Meaning: Recorded support for claims, verification results, or user judgment context.
- Primary owner: [Core Model](core-model.md)
- See also: [API State Schemas](api/schema-state.md), [Record-run method](api/method-record-run.md)

### `ArtifactRef`

- Term: `ArtifactRef`
- Korean term: `ArtifactRef`
- Meaning: The schema identifier for a persisted artifact reference.
- Primary owner: [API Artifact Schemas](api/schema-artifacts.md)
- See also: [Artifact Storage](storage-artifacts.md)

### `ArtifactInput`

- Term: `ArtifactInput`
- Korean term: `ArtifactInput`
- Meaning: The schema identifier for artifact input data.
- Primary owner: [API Artifact Schemas](api/schema-artifacts.md)

### `StagedArtifactHandle`

- Term: `StagedArtifactHandle`
- Korean term: `StagedArtifactHandle`
- Meaning: The identifier for a staged artifact handle.
- Primary owner: [API Artifact Schemas](api/schema-artifacts.md)
- See also: [Artifact Storage](storage-artifacts.md)

### projection

- Term: projection
- Korean term: 상태 보기
- Meaning: A read-only state view.
- Primary owner: [Projection Authority Reference](projection-and-templates.md)
- See also: [Template Bodies](template-bodies.md)

### `Projection`

- Term: `Projection`
- Korean term: `Projection`
- Meaning: The exact product label for the read-only state-view concept.
- Primary owner: [Projection Authority Reference](projection-and-templates.md)
- See also: [Template Bodies](template-bodies.md)

### surface

- Term: surface
- Korean term: 접점
- Meaning: An integration or interaction boundary where context appears.
- Primary owner: [Agent Integration](agent-integration.md)
- See also: [Security](security.md)

### `surface_id`

- Term: `surface_id`
- Korean term: `surface_id`
- Meaning: The exact identifier for a surface.
- Primary owner: [Agent Integration](agent-integration.md)
- See also: [API Schema Core](api/schema-core.md)

### active surface context

- Term: active surface context
- Korean term: 현재 적용 접점 맥락
- Meaning: The current surface context for a request or interaction.
- Primary owner: [Agent Integration](agent-integration.md)
- See also: [Security](security.md)

### `state_version`

- Term: `state_version`
- Korean term: `state_version`
- Meaning: The state-clock identifier for stored project state.
- Primary owner: [Storage Versioning](storage-versioning.md)
- See also: [API Schema Core](api/schema-core.md)

### runtime

- Term: runtime
- Korean term: 런타임
- Meaning: The operational Harness execution and data context.
- Primary owner: [Runtime Boundaries](runtime-boundaries.md)
- See also: [Security](security.md)

### `Write Authorization`

- Term: `Write Authorization`
- Korean term: 쓰기 권한 부여
- Meaning: The exact product label for the Harness write-authorization concept.
- Primary owner: [Core Model](core-model.md)
- See also: [Security](security.md), [Prepare-write method](api/method-prepare-write.md)

### sensitive approval

- Term: sensitive approval
- Korean term: 민감 동작 승인
- Meaning: User approval for a sensitive action, separate from `Write Authorization`.
- Primary owner: [Core Model](core-model.md)
- See also: [API Judgment Schemas](api/schema-judgment.md), [Security](security.md)

### access class

- Term: access class
- Korean term: 접근 등급
- Meaning: A value category for access context.
- Primary owner: [API Value Sets](api/schema-value-sets.md)
- See also: [Agent Integration](agent-integration.md), [Security](security.md)

### baseline guarantee

- Term: baseline guarantee
- Korean term: 기준 범위 보장
- Meaning: A guarantee documented for the baseline scope.
- Primary owner: [Security](security.md)
- See also: [Scope](scope.md)

### cooperative guarantee

- Term: cooperative guarantee
- Korean term: 협력형 보장
- Meaning: A security guarantee type based on cooperative behavior.
- Primary owner: [Security](security.md)

### detective guarantee

- Term: detective guarantee
- Korean term: 탐지형 보장
- Meaning: A security guarantee type based on observable detection.
- Primary owner: [Security](security.md)
- See also: [Agent Integration](agent-integration.md)

### design-quality owner boundary

- Term: design-quality owner boundary
- Korean term: 설계 품질 담당 경계
- Meaning: The boundary that routes design-quality observations to the relevant owner.
- Primary owner: [Design Quality](design-quality.md)

### reserved value

- Term: reserved value
- Korean term: 예약된 값
- Meaning: A value reserved as vocabulary or surface area without baseline behavior by itself.
- Primary owner: [Scope](scope.md)
- See also: [API Value Sets](api/schema-value-sets.md)

### profile-gated value

- Term: profile-gated value
- Korean term: 프로필 조건부 값
- Meaning: A value available only when the documented profile or gate supports it.
- Primary owner: [Scope](scope.md)
- See also: [API Value Sets](api/schema-value-sets.md)

### `ErrorCode`

- Term: `ErrorCode`
- Korean term: `ErrorCode`
- Meaning: The public API error-code identifier.
- Primary owner: [API error codes](api/error-codes.md)
- See also: [API error precedence](api/error-precedence.md), [API error routing](api/error-routing.md)

### error code meanings

- Term: error code meanings
- Korean term: 공개 오류 코드 의미
- Meaning: The meanings and occurrence summaries for public API error codes.
- Primary owner: [API error codes](api/error-codes.md)
- See also: [API error precedence](api/error-precedence.md)

### error precedence

- Term: error precedence
- Korean term: 오류 우선순위
- Meaning: The selection and ordering rules for public API errors.
- Primary owner: [API error precedence](api/error-precedence.md)
- See also: [API error codes](api/error-codes.md)

### error routing

- Term: error routing
- Korean term: 오류 처리 경로
- Meaning: The routing of rejected, blocked, and dry-run API response branches.
- Primary owner: [API error routing](api/error-routing.md)
- See also: [API error codes](api/error-codes.md), [API blocker routing](api/blocker-routing.md)

### blocker routing

- Term: blocker routing
- Korean term: 차단 사유 처리 경로
- Meaning: The boundary between close-readiness blockers and API response branches.
- Primary owner: [API blocker routing](api/blocker-routing.md)
- See also: [API error routing](api/error-routing.md), [Close-task method](api/method-close-task.md)

### error/blocker boundary

- Term: error/blocker boundary
- Korean term: 오류와 차단 사유의 경계
- Meaning: The distinction between public API errors and close-readiness blocker data.
- Primary owner: [API blocker routing](api/blocker-routing.md)
- See also: [API error codes](api/error-codes.md)

### public error as blocker

- Term: public error as blocker
- Korean term: 공개 오류 코드가 차단 사유로 표현되는 경우
- Meaning: Boundary wording for cases where public error-code wording appears in blocker data.
- Primary owner: [API blocker routing](api/blocker-routing.md)
- See also: [API error codes](api/error-codes.md)

### `ToolError.details`

- Term: `ToolError.details`
- Korean term: `ToolError.details`
- Meaning: The machine-readable error details field.
- Primary owner: [API error details](api/error-details.md)
- See also: [API error codes](api/error-codes.md)

### error detail helper values

- Term: error detail helper values
- Korean term: 오류 세부사항 보조 값
- Meaning: Helper values nested under machine-readable error details.
- Primary owner: [API error details](api/error-details.md)

### dry-run

- Term: dry-run
- Korean term: dry-run 미리보기
- Meaning: Preview mode for API calls using `dry_run`.
- Primary owner: [API Core Schemas](api/schema-core.md)
- See also: [API error routing](api/error-routing.md), [Storage Effects](storage-effects.md)

### dry-run preview routing

- Term: dry-run preview routing
- Korean term: dry-run 미리보기 처리 경로
- Meaning: Response-branch routing for `dry_run` previews.
- Primary owner: [API error routing](api/error-routing.md)
- See also: [API Core Schemas](api/schema-core.md)

### blocked result

- Term: blocked result
- Korean term: 차단 결과
- Meaning: A result branch that reports a block rather than a rejected response.
- Primary owner: [API error routing](api/error-routing.md)
- See also: [Close-task method](api/method-close-task.md), [Prepare-write method](api/method-prepare-write.md)

### rejected response

- Term: rejected response
- Korean term: 거부 응답
- Meaning: An API response for a request rejected before the operation proceeds.
- Primary owner: [API error routing](api/error-routing.md)
- See also: [API Core Schemas](api/schema-core.md), [API error codes](api/error-codes.md)

### migration

- Term: migration
- Korean term: 마이그레이션
- Meaning: A technical migration of schema, storage, data, or documentation.
- Primary owner: [Storage Versioning](storage-versioning.md)
- See also: [Storage overview](storage.md)

### lifecycle

- Term: lifecycle
- Korean term: 생명주기
- Meaning: The stages of an entity or artifact over time.
- Primary owner: [Core Model](core-model.md)
- See also: [API Value Sets](api/schema-value-sets.md)
