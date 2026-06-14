# Glossary

This document owns official terminology for Harness documentation. It defines term-level meaning, Korean terminology choices, and card-level routing for product terms.

It does not define exact schemas, value sets, DDL, storage effects, security mechanisms, API behavior, runtime behavior, or baseline implementation reading paths.

## How to use this glossary

Use the summary table as a compact routing aid. Use the term cards as the editable terminology source for each term. Term cards stay short: they define the term, name the Korean term, identify the term type, and route readers to the primary owner.

Each term card uses these ownership fields:

- `Primary owner` is the canonical owner for the term's definition or contract.
- `Related references` are adjacent documents that help interpret the term but do not own it.

Prefer one `Primary owner` per term. When a concept needs a different canonical owner, split it into a more precise glossary term instead of adding another primary owner.

Use this glossary with [docs/terminology-map.yaml](../../terminology-map.yaml), which owns machine-readable bilingual term controls, identifier preservation controls, and Korean mixed-language expressions to avoid.

When a card points to a schema, API, storage, security, projection, or runtime contract, follow the `Primary owner` instead of copying contract detail into the glossary.

## Summary table

| Term | Korean term | Primary owner |
|---|---|---|
| Harness | 하네스 | [Scope](scope.md) |
| Product Repository | Product Repository | [Runtime Boundaries](runtime-boundaries.md) |
| Harness Runtime Home | Harness Runtime Home | [Runtime Boundaries](runtime-boundaries.md) |
| documentation | 문서 | [Authoring Guide](../maintain/authoring-guide.md) |
| baseline scope | 기준 범위 | [Scope](scope.md) |
| supported scope | 지원 범위 | [Scope](scope.md) |
| supported behavior | 지원 동작 | [Scope](scope.md) |
| supported API method | 지원되는 API 메서드 | [API Methods](api/methods.md) |
| supported API value | 지원되는 API 값 | [API Value Sets](api/schema-value-sets.md) |
| out-of-scope capability | 지원 범위 밖 기능 | [Scope](scope.md) |
| evidence collection workflow | 증거 수집 흐름 | [Scope](scope.md) |
| expanded or additional evidence collection workflows | 확장 또는 추가 증거 수집 흐름 | [Scope](scope.md) |
| owner document | 담당 문서 | [Authoring Guide](../maintain/authoring-guide.md) |
| owner contract | 담당 계약 | [Authoring Guide](../maintain/authoring-guide.md) |
| applicable owner path | 적용되는 담당 경로 | [Authoring Guide](../maintain/authoring-guide.md) |
| applicable reference | 적용되는 참조 문서 | [Reference Index](README.md) |
| existing owner | 기존 담당 문서 | [Authoring Guide](../maintain/authoring-guide.md) |
| promotion-time owner update | 승격 시점의 담당 문서 갱신 | [Scope](scope.md) |
| owner placeholder | 담당 문서 자리표시자 | [Authoring Guide](../maintain/authoring-guide.md) |
| `Task` | `Task` | [Core Model](core-model.md) |
| scope | 범위 | [Core Model](core-model.md) |
| active scope | 현재 적용 범위 | [Core Model](core-model.md) |
| active Change Unit | 현재 적용 Change Unit | [Core Model](core-model.md) |
| user-owned judgment | 사용자 소유 판단 | [Core Model](core-model.md) |
| `UserJudgment` | `UserJudgment` | [API Judgment Schemas](api/schema-judgment.md) |
| close readiness | 닫기 준비 상태 | [Core Model](core-model.md) |
| close readiness evaluation | 닫기 준비 상태 평가 | [Close-task method](api/method-close-task.md) |
| close task | Task 닫기 | [Close-task method](api/method-close-task.md) |
| close task behavior | Task 닫기 동작 | [Close-task method](api/method-close-task.md) |
| `harness.close_task` | `harness.close_task` | [Close-task method](api/method-close-task.md) |
| close-readiness blocker | 닫기 차단 사유 | [API blocker routing](api/blocker-routing.md) |
| `CloseReadinessBlocker` | `CloseReadinessBlocker` | [API State Schemas](api/schema-state.md) |
| blocker category | 차단 사유 범주 | [API Value Sets](api/schema-value-sets.md) |
| complete intent | `complete` | [API Value Sets](api/schema-value-sets.md) |
| full evaluation order | 전체 평가 순서 | [Translation Guide](../maintain/translation-guide.md) |
| artifact | 아티팩트 | [API Artifact Schemas](api/schema-artifacts.md) |
| evidence | 증거 | [Core Model](core-model.md) |
| `ArtifactRef` | `ArtifactRef` | [API Artifact Schemas](api/schema-artifacts.md) |
| `ArtifactInput` | `ArtifactInput` | [API Artifact Schemas](api/schema-artifacts.md) |
| `StagedArtifactHandle` | `StagedArtifactHandle` | [API Artifact Schemas](api/schema-artifacts.md) |
| projection | 상태 보기 | [Projection Authority Reference](projection-and-templates.md) |
| `Projection` | `Projection` | [Projection Authority Reference](projection-and-templates.md) |
| surface | 접점 | [Agent Integration](agent-integration.md) |
| `surface_id` | `surface_id` | [Agent Integration](agent-integration.md) |
| active surface context | 현재 적용 접점 맥락 | [Agent Integration](agent-integration.md) |
| `state_version` | `state_version` | [Storage Versioning](storage-versioning.md) |
| runtime | 런타임 | [Runtime Boundaries](runtime-boundaries.md) |
| `Write Authorization` | 쓰기 권한 부여 | [Core Model](core-model.md) |
| sensitive approval | 민감 동작 승인 | [Core Model](core-model.md) |
| access class | 접근 등급 | [API Value Sets](api/schema-value-sets.md) |
| baseline guarantee | 기준 범위 보장 | [Security](security.md) |
| cooperative guarantee | 협력형 보장 | [Security](security.md) |
| detective guarantee | 탐지형 보장 | [Security](security.md) |
| design-quality owner boundary | 설계 품질 담당 경계 | [Design Quality](design-quality.md) |
| reserved value | 예약된 값 | [Scope](scope.md) |
| profile-gated value | 프로필 조건부 값 | [Scope](scope.md) |
| `ErrorCode` | `ErrorCode` | [API error codes](api/error-codes.md) |
| error code meanings | 공개 오류 코드 의미 | [API error codes](api/error-codes.md) |
| error precedence | 오류 우선순위 | [API error precedence](api/error-precedence.md) |
| error routing | 오류 처리 경로 | [API error routing](api/error-routing.md) |
| blocker routing | 차단 사유 처리 경로 | [API blocker routing](api/blocker-routing.md) |
| error/blocker boundary | 오류와 차단 사유의 경계 | [API blocker routing](api/blocker-routing.md) |
| public error as blocker | 공개 오류 코드가 차단 사유로 표현되는 경우 | [API blocker routing](api/blocker-routing.md) |
| `ToolError.details` | `ToolError.details` | [API error details](api/error-details.md) |
| detail helper values | 오류 세부사항 보조 값 | [API error details](api/error-details.md) |
| dry-run preview routing | dry-run 미리보기 처리 경로 | [API error routing](api/error-routing.md) |
| blocked result | 차단 결과 | [API error routing](api/error-routing.md) |
| rejected response | 거부 응답 | [API error routing](api/error-routing.md) |
| migration | 마이그레이션 | [Storage Versioning](storage-versioning.md) |
| lifecycle | 생명주기 | [Core Model](core-model.md) |

## Terms

### Harness

Term:
- Harness

Korean term:
- 하네스

Type:
- product concept

Meaning:
- Harness is the local work-authority server for AI-assisted product work.

Primary owner:
- [Scope](scope.md)

Related references:
- [Runtime Boundaries](runtime-boundaries.md)

Usage note:
- Use Harness for the product name.

### Product Repository

Term:
- Product Repository

Korean term:
- Product Repository; user-facing prose may use 제품 저장소.

Type:
- product label

Meaning:
- `Product Repository` is the user's project workspace, separate from Harness runtime state.

Primary owner:
- [Runtime Boundaries](runtime-boundaries.md)

Related references:
- None.

Usage note:
- Use the exact label when naming this boundary.

### Harness Runtime Home

Term:
- Harness Runtime Home

Korean term:
- Harness Runtime Home; user-facing prose may use 런타임 홈.

Type:
- product label

Meaning:
- `Harness Runtime Home` is the operational data space for Harness records and artifacts.

Primary owner:
- [Runtime Boundaries](runtime-boundaries.md)

Related references:
- None.

Usage note:
- Use the exact label when naming this boundary.

### documentation

Term:
- documentation

Korean term:
- 문서

Type:
- documentation term

Meaning:
- Documentation is maintained source material, not runtime implementation, generated runtime output, or acceptance state.

Primary owner:
- [Authoring Guide](../maintain/authoring-guide.md)

Related references:
- [Runtime Boundaries](runtime-boundaries.md)
- [Implementation Guide](../build/implementation-guide.md)

Usage note:
- Keep documentation authority separate from runtime behavior and product implementation output.

### baseline scope

Term:
- baseline scope

Korean term:
- 기준 범위

Type:
- scope term

Meaning:
- Baseline scope is the stable support boundary documented for Harness.

Primary owner:
- [Scope](scope.md)

Related references:
- [API Value Sets](api/schema-value-sets.md)

Usage note:
- Use Scope for support-status detail.

### supported scope

Term:
- supported scope

Korean term:
- 지원 범위; 지원되는 범위 when grammar needs a modifier.

Type:
- scope term

Meaning:
- Supported scope is behavior or capability documented as supported.

Primary owner:
- [Scope](scope.md)

Related references:
- None.

Usage note:
- Use active scope for the currently applied `Task` or Change Unit boundary.

### supported behavior

Term:
- supported behavior

Korean term:
- 지원 동작

Type:
- support-boundary term

Meaning:
- Supported behavior is behavior documented as supported by Scope and the affected semantic owner.

Primary owner:
- [Scope](scope.md)

Related references:
- [API Value Sets](api/schema-value-sets.md)

Usage note:
- Use Scope and the semantic owner for support questions.

### supported API method

Term:
- supported API method

Korean term:
- 지원되는 API 메서드

Type:
- API term

Meaning:
- A supported API method is a public method documented as supported.

Primary owner:
- [API Methods](api/methods.md)

Related references:
- None.

Usage note:
- Use exact method identifiers when naming public API methods.

### supported API value

Term:
- supported API value

Korean term:
- 지원되는 API 값

Type:
- API value term

Meaning:
- A supported API value is a value documented as supported, not merely present as vocabulary.

Primary owner:
- [API Value Sets](api/schema-value-sets.md)

Related references:
- [Scope](scope.md)

Usage note:
- Route exact value-name questions to API Value Sets and support-availability questions to Scope or the semantic owner.

### out-of-scope capability

Term:
- out-of-scope capability

Korean term:
- 지원 범위 밖 기능

Type:
- scope boundary term

Meaning:
- An out-of-scope capability names a capability outside the supported boundary.

Primary owner:
- [Scope](scope.md)

Related references:
- None.

Usage note:
- Use Scope for support status.

### evidence collection workflow

Term:
- evidence collection workflow

Korean term:
- 증거 수집 흐름

Type:
- out-of-scope capability wording

Meaning:
- Evidence collection workflow wording names a capability area.

Primary owner:
- [Scope](scope.md)

Related references:
- [Terminology Map](../../terminology-map.yaml)

Usage note:
- Use Scope for support status.

### expanded or additional evidence collection workflows

Term:
- expanded or additional evidence collection workflows
- expanded evidence collection workflows
- additional evidence collection workflows

Korean term:
- 확장 또는 추가 증거 수집 흐름

Type:
- out-of-scope capability family

Meaning:
- This phrase names an evidence-workflow capability family outside the supported boundary.

Primary owner:
- [Scope](scope.md)

Related references:
- [Terminology Map](../../terminology-map.yaml)

Usage note:
- Use Scope for support status.

### owner document

Term:
- owner document

Korean term:
- 담당 문서

Type:
- owner-routing term

Meaning:
- An owner document is the canonical source for a product concept, contract, route, or terminology rule.

Primary owner:
- [Authoring Guide](../maintain/authoring-guide.md)

Related references:
- [Reference Index](README.md)

Usage note:
- A file path is documentation routing, not a product actor.

### owner contract

Term:
- owner contract

Korean term:
- 담당 계약; 담당 문서가 정의한 계약 when clearer.

Type:
- owner-routing term

Meaning:
- An owner contract names the contract defined by the relevant owner document.

Primary owner:
- [Authoring Guide](../maintain/authoring-guide.md)

Related references:
- [Terminology Map](../../terminology-map.yaml)

Usage note:
- Use it for the contract defined by an owner document.

### applicable owner path

Term:
- applicable owner path

Korean term:
- 적용되는 담당 경로

Type:
- owner-routing term

Meaning:
- An applicable owner path is the owner route that applies to a topic.

Primary owner:
- [Authoring Guide](../maintain/authoring-guide.md)

Related references:
- [Reference Index](README.md)
- [doc-index.yaml](../../doc-index.yaml)

Usage note:
- Use only for documentation routing.

### applicable reference

Term:
- applicable reference

Korean term:
- 적용되는 참조 문서

Type:
- reference-routing term

Meaning:
- Applicable reference names the reference document that defines the relevant contract.

Primary owner:
- [Reference Index](README.md)

Related references:
- [Authoring Guide](../maintain/authoring-guide.md)
- [Terminology Map](../../terminology-map.yaml)

Usage note:
- Use only as documentation-routing shorthand.

### existing owner

Term:
- existing owner
- existing canonical owner
- existing owner document

Korean term:
- 기존 담당 문서

Type:
- owner-routing term

Meaning:
- An existing owner is a canonical owner document that already exists and can be linked as the source of normative meaning.

Primary owner:
- [Authoring Guide](../maintain/authoring-guide.md)

Related references:
- [Reference Index](README.md)
- [doc-index.yaml](../../doc-index.yaml)

Usage note:
- Use only for owner documents that already exist.

### promotion-time owner update

Term:
- promotion-time owner update

Korean term:
- 승격 시점의 담당 문서 갱신

Type:
- scope-promotion term

Meaning:
- Promotion-time owner update names the owner changes needed when an out-of-scope capability is promoted into support.

Primary owner:
- [Scope](scope.md)

Related references:
- [Authoring Guide](../maintain/authoring-guide.md)

Usage note:
- Use when discussing promotion planning; Scope owns the support boundary.

### owner placeholder

Term:
- owner placeholder

Korean term:
- 담당 문서 자리표시자

Type:
- owner-gap term

Meaning:
- An owner placeholder signals that a capability may need an owner created or designated before promotion.

Primary owner:
- [Authoring Guide](../maintain/authoring-guide.md)

Related references:
- [Scope](scope.md)

Usage note:
- Use only to signal an owner gap.

### `Task`

Term:
- `Task`

Korean term:
- `Task`; user-facing prose may use 작업 when exact entity identity is not needed.

Type:
- Core entity

Meaning:
- `Task` is the user-value unit being shaped, executed, blocked, or closed.

Primary owner:
- [Core Model](core-model.md)

Related references:
- [API State Schemas](api/schema-state.md)
- [API Value Sets](api/schema-value-sets.md)

Usage note:
- Use exact identifiers when naming the entity or fields.

### scope

Term:
- scope

Korean term:
- 범위

Type:
- Core authority term

Meaning:
- Scope is the accepted boundary for what the current `Task` or Change Unit covers and excludes.

Primary owner:
- [Core Model](core-model.md)

Related references:
- [Update-scope method](api/method-update-scope.md)
- [API Judgment Schemas](api/schema-judgment.md)

Usage note:
- Use exact identifiers when naming schema or API fields.

### active scope

Term:
- active scope
- currently applied scope

Korean term:
- 현재 적용 범위

Type:
- Core authority term

Meaning:
- Active scope is the scope currently applied inside a `Task` or Change Unit context.

Primary owner:
- [Core Model](core-model.md)

Related references:
- [Update-scope method](api/method-update-scope.md)

Usage note:
- Keep active scope distinct from baseline or supported scope.

### active Change Unit

Term:
- active Change Unit

Korean term:
- 현재 적용 Change Unit

Type:
- Core authority term

Meaning:
- An active Change Unit is the currently applied Change Unit in the authority model.

Primary owner:
- [Core Model](core-model.md)

Related references:
- [Update-scope method](api/method-update-scope.md)

Usage note:
- Keep Change Unit as the product term in Korean prose.

### user-owned judgment

Term:
- user-owned judgment

Korean term:
- 사용자 소유 판단; user-facing prose may use 사용자 판단.

Type:
- Core authority term

Meaning:
- User-owned judgment is a decision reserved to the user.

Primary owner:
- [Core Model](core-model.md)

Related references:
- [API Judgment Schemas](api/schema-judgment.md)

Usage note:
- Keep it distinct from acceptance and authorization terms.

### `UserJudgment`

Term:
- `UserJudgment`

Korean term:
- `UserJudgment`; prose should use 사용자 소유 판단 when not naming the schema.

Type:
- API schema

Meaning:
- `UserJudgment` is the API schema identifier for a pending or resolved user-owned judgment.

Primary owner:
- [API Judgment Schemas](api/schema-judgment.md)

Related references:
- [Core Model](core-model.md)
- [User judgment methods](api/method-user-judgment.md)

Usage note:
- Use the prose term for the concept and the schema identifier for the schema.

### close readiness

Term:
- close readiness

Korean term:
- 닫기 준비 상태; user-facing prose may use 닫기 가능 여부.

Type:
- Core close-readiness concept

Meaning:
- Close readiness is the Core concept for whether a task is ready to close.

Primary owner:
- [Core Model](core-model.md)

Related references:
- [Close-task method](api/method-close-task.md)
- [API blocker routing](api/blocker-routing.md)

Usage note:
- Use schema terms only when naming the schema.

### close readiness evaluation

Term:
- close readiness evaluation

Korean term:
- 닫기 준비 상태 평가

Type:
- close-task method term

Meaning:
- Close readiness evaluation names the close-task method concern for close readiness.

Primary owner:
- [Close-task method](api/method-close-task.md)

Related references:
- [Core Model](core-model.md)

Usage note:
- Use the method owner for evaluation detail.

### close task

Term:
- close task

Korean term:
- Task 닫기

Type:
- API method term

Meaning:
- Close task names the Task-closing API operation.

Primary owner:
- [Close-task method](api/method-close-task.md)

Related references:
- [API Methods](api/methods.md)

Usage note:
- Use `harness.close_task` when naming the exact public method.

### close task behavior

Term:
- close task behavior
- `harness.close_task` behavior
- close-task method behavior

Korean term:
- Task 닫기 동작

Type:
- API method term

Meaning:
- Close task behavior names the behavior area owned by `harness.close_task`.

Primary owner:
- [Close-task method](api/method-close-task.md)

Related references:
- [API Methods](api/methods.md)

Usage note:
- Use the close-task method owner for details.

### `harness.close_task`

Term:
- `harness.close_task`

Korean term:
- `harness.close_task`

Type:
- API method identifier

Meaning:
- `harness.close_task` is the public method identifier for close-task requests.

Primary owner:
- [Close-task method](api/method-close-task.md)

Related references:
- [API Methods](api/methods.md)

Usage note:
- Use the close-task method owner for details.

### close-readiness blocker

Term:
- close-readiness blocker
- close blocker

Korean term:
- 닫기 차단 사유

Type:
- API blocker-routing term

Meaning:
- A close-readiness blocker names a reason a task is not ready to close.

Primary owner:
- [API blocker routing](api/blocker-routing.md)

Related references:
- [Core Model](core-model.md)
- [API State Schemas](api/schema-state.md)

Usage note:
- Use 닫기 차단 사유 in Korean prose; use `CloseReadinessBlocker` for the schema.

### `CloseReadinessBlocker`

Term:
- `CloseReadinessBlocker`

Korean term:
- `CloseReadinessBlocker`; user-facing prose should use 닫기 차단 사유 when not naming the schema.

Type:
- API schema

Meaning:
- `CloseReadinessBlocker` is the API schema identifier for close-readiness blocking data.

Primary owner:
- [API State Schemas](api/schema-state.md)

Related references:
- [API blocker routing](api/blocker-routing.md)

Usage note:
- Use the prose term for the concept and the schema identifier for the schema.

### blocker category

Term:
- blocker category

Korean term:
- 차단 사유 범주

Type:
- API value concept

Meaning:
- Blocker category is the prose concept for grouping close-readiness blockers.

Primary owner:
- [API Value Sets](api/schema-value-sets.md)

Related references:
- [API State Schemas](api/schema-state.md)
- [API blocker routing](api/blocker-routing.md)

Usage note:
- Use `CloseReadinessBlocker.category` when naming the exact field.

### complete intent

Term:
- complete intent
- `complete` when naming the intent value

Korean term:
- `complete`

Type:
- API value term

Meaning:
- Complete intent is the prose concept behind the `complete` intent value.

Primary owner:
- [API Value Sets](api/schema-value-sets.md)

Related references:
- [Close-task method](api/method-close-task.md)
- [Terminology Map](../../terminology-map.yaml)

Usage note:
- Use `complete` only for the enum value or exact identifier.

### full evaluation order

Term:
- full evaluation order
- entire evaluation order

Korean term:
- 전체 평가 순서; in close-readiness context, 전체 닫기 준비 상태 평가 순서.

Type:
- translation term

Meaning:
- Full evaluation order names an entire evaluation sequence without invoking the `complete` enum value.

Primary owner:
- [Translation Guide](../maintain/translation-guide.md)

Related references:
- [Terminology Map](../../terminology-map.yaml)

Usage note:
- Use full or entire for ordinary prose meaning.

### artifact

Term:
- artifact

Korean term:
- 아티팩트

Type:
- artifact term

Meaning:
- An artifact is product work material that Harness can refer to.

Primary owner:
- [API Artifact Schemas](api/schema-artifacts.md)

Related references:
- [Artifact Storage](storage-artifacts.md)

Usage note:
- Artifact availability alone is not evidence sufficiency.

### evidence

Term:
- evidence

Korean term:
- 증거

Type:
- Core evidence concept

Meaning:
- Evidence supports recorded claims at recorded scope.

Primary owner:
- [Core Model](core-model.md)

Related references:
- [API State Schemas](api/schema-state.md)
- [Record-run method](api/method-record-run.md)

Usage note:
- Keep acceptance and residual-risk terms separate.

### `ArtifactRef`

Term:
- `ArtifactRef`

Korean term:
- `ArtifactRef`; user-facing prose may use 아티팩트 참조 when not naming the schema.

Type:
- API schema

Meaning:
- `ArtifactRef` is the API schema identifier for an artifact reference.

Primary owner:
- [API Artifact Schemas](api/schema-artifacts.md)

Related references:
- [Artifact Storage](storage-artifacts.md)

Usage note:
- Use the artifact schema owner for reference detail.

### `ArtifactInput`

Term:
- `ArtifactInput`

Korean term:
- `ArtifactInput`; user-facing prose may use 제공할 아티팩트 when not naming the schema.

Type:
- API schema

Meaning:
- `ArtifactInput` is the schema identifier for artifact data supplied to an artifact-owning method.

Primary owner:
- [API Artifact Schemas](api/schema-artifacts.md)

Related references:
- None.

Usage note:
- Use the artifact schema owner for input detail.

### `StagedArtifactHandle`

Term:
- `StagedArtifactHandle`

Korean term:
- `StagedArtifactHandle`; user-facing prose may use 스테이징된 아티팩트 핸들.

Type:
- API schema

Meaning:
- `StagedArtifactHandle` is the schema identifier for a transient staged artifact handle.

Primary owner:
- [API Artifact Schemas](api/schema-artifacts.md)

Related references:
- [Artifact Storage](storage-artifacts.md)

Usage note:
- Use the artifact schema owner for handle detail.

### projection

Term:
- projection

Korean term:
- 상태 보기

Type:
- projection term

Meaning:
- A projection is a read-only status or display view.

Primary owner:
- [Projection Authority Reference](projection-and-templates.md)

Related references:
- [Template Bodies](template-bodies.md)

Usage note:
- Use the projection owner for authority questions.

### `Projection`

Term:
- `Projection`

Korean term:
- `Projection`; reference prose may introduce it as `Projection`(읽기 전용 상태 보기).

Type:
- product label

Meaning:
- `Projection` is the exact Harness label for a read-only derived status or display view.

Primary owner:
- [Projection Authority Reference](projection-and-templates.md)

Related references:
- [Template Bodies](template-bodies.md)
- [API State Schemas](api/schema-state.md)

Usage note:
- Use the projection owner for label detail.

### surface

Term:
- surface

Korean term:
- 접점

Type:
- integration term

Meaning:
- A surface is a user, agent, tool, connector, or local context where Harness is used or observed.

Primary owner:
- [Agent Integration](agent-integration.md)

Related references:
- [Security](security.md)

Usage note:
- `surface_id` is not authority proof.

### `surface_id`

Term:
- `surface_id`

Korean term:
- `surface_id`

Type:
- surface identifier

Meaning:
- `surface_id` is the request-level selector for a registered local surface.

Primary owner:
- [Agent Integration](agent-integration.md)

Related references:
- [API Schema Core](api/schema-core.md)
- [Security](security.md)

Usage note:
- Use Agent Integration for surface-identity detail.

### active surface context

Term:
- active surface context

Korean term:
- 현재 적용 접점 맥락

Type:
- integration term

Meaning:
- Active surface context is the current surface context for a request or interaction.

Primary owner:
- [Agent Integration](agent-integration.md)

Related references:
- [Security](security.md)

Usage note:
- Use Agent Integration for context detail.

### `state_version`

Term:
- `state_version`
- `project_state.state_version`

Korean term:
- `state_version`

Type:
- storage versioning identifier

Meaning:
- `state_version` names the project-wide state version identifier.

Primary owner:
- [Storage Versioning](storage-versioning.md)

Related references:
- [API Schema Core](api/schema-core.md)

Usage note:
- Use Storage Versioning for state-clock detail.

### runtime

Term:
- runtime

Korean term:
- 런타임

Type:
- runtime term

Meaning:
- Runtime means executing Harness server/runtime behavior and runtime data space.

Primary owner:
- [Runtime Boundaries](runtime-boundaries.md)

Related references:
- [Security](security.md)

Usage note:
- Markdown source docs are not runtime state or generated runtime output.

### `Write Authorization`

Term:
- `Write Authorization`

Korean term:
- 쓰기 권한 부여

Type:
- Core authorization term

Meaning:
- `Write Authorization` is the named Core authorization for one compatible product-file write attempt.

Primary owner:
- [Core Model](core-model.md)

Related references:
- [Security](security.md)
- [Prepare-write method](api/method-prepare-write.md)

Usage note:
- Keep this label distinct from command approval and sensitive approval.

### sensitive approval

Term:
- sensitive approval
- sensitive-action approval

Korean term:
- 민감 동작 승인

Type:
- approval term

Meaning:
- Sensitive-action approval is user permission for a sensitive action boundary.

Primary owner:
- [Core Model](core-model.md)

Related references:
- [API Judgment Schemas](api/schema-judgment.md)
- [Security](security.md)

Usage note:
- Prefer sensitive-action approval in English prose.

### access class

Term:
- access class

Korean term:
- 접근 등급

Type:
- access term

Meaning:
- Access class is a classification used to describe protected access expectations.

Primary owner:
- [API Value Sets](api/schema-value-sets.md)

Related references:
- [Agent Integration](agent-integration.md)
- [Security](security.md)

Usage note:
- Use API Value Sets for value names and Security for guarantee wording.

### baseline guarantee

Term:
- baseline guarantee

Korean term:
- 기준 범위 보장

Type:
- security term

Meaning:
- Baseline guarantee names a Security-defined guarantee in the baseline scope.

Primary owner:
- [Security](security.md)

Related references:
- [Scope](scope.md)

Usage note:
- Use Security for guarantee level.

### cooperative guarantee

Term:
- cooperative guarantee

Korean term:
- 협력형 보장

Type:
- security term

Meaning:
- Cooperative guarantee names a Security-defined guarantee type.

Primary owner:
- [Security](security.md)

Related references:
- None.

Usage note:
- Use Security for guarantee strength.

### detective guarantee

Term:
- detective guarantee

Korean term:
- 탐지형 보장

Type:
- security term

Meaning:
- Detective guarantee names a Security-defined guarantee type.

Primary owner:
- [Security](security.md)

Related references:
- None.

Usage note:
- Use Security for guarantee strength.

### design-quality owner boundary

Term:
- design-quality owner boundary
- design-quality routing boundary
- design-quality boundary

Korean term:
- 설계 품질 담당 경계

Type:
- design-quality term

Meaning:
- Design-quality owner boundary routes design-quality observations to the relevant owner documents or owner contracts.

Primary owner:
- [Design Quality](design-quality.md)

Related references:
- None.

Usage note:
- Design-quality wording is not independent QA, acceptance, residual-risk, evidence, or close authority.

### reserved value

Term:
- reserved value

Korean term:
- 예약된 값

Type:
- value-status term

Meaning:
- Reserved value names vocabulary that is not supported behavior by itself.

Primary owner:
- [Scope](scope.md)

Related references:
- [API Value Sets](api/schema-value-sets.md)

Usage note:
- Value-set presence does not make behavior supported.

### profile-gated value

Term:
- profile-gated value

Korean term:
- 프로필 조건부 값

Type:
- value-status term

Meaning:
- Profile-gated value names vocabulary whose support depends on an explicit profile.

Primary owner:
- [Scope](scope.md)

Related references:
- [API Value Sets](api/schema-value-sets.md)

Usage note:
- Use Scope for support status.

### `ErrorCode`

Term:
- `ErrorCode`

Korean term:
- `ErrorCode`; user-facing prose may use 공개 오류 코드 when not naming the identifier.

Type:
- API error identifier

Meaning:
- `ErrorCode` is the public API error-code identifier space.

Primary owner:
- [API error codes](api/error-codes.md)

Related references:
- [API error precedence](api/error-precedence.md)
- [API error routing](api/error-routing.md)

Usage note:
- Use the error-code owner for public code meanings.

### error code meanings

Term:
- error code meanings
- public error code meanings

Korean term:
- 공개 오류 코드 의미

Type:
- API error-code term

Meaning:
- Error code meanings name the public meaning concern for `ErrorCode` values.

Primary owner:
- [API error codes](api/error-codes.md)

Related references:
- [API error precedence](api/error-precedence.md)

Usage note:
- Use the focused API error owners for neighboring concerns.

### error precedence

Term:
- error precedence
- primary public-error selection

Korean term:
- 오류 우선순위

Type:
- API error-precedence term

Meaning:
- Error precedence names the primary public-error selection concern.

Primary owner:
- [API error precedence](api/error-precedence.md)

Related references:
- [API error codes](api/error-codes.md)

Usage note:
- Use the focused API error owners for neighboring concerns.

### error routing

Term:
- error routing
- API response branch routing
- API error routing, when naming the owner document

Korean term:
- 오류 처리 경로

Type:
- API error-routing term

Meaning:
- Error routing names the API response-branch routing concern.

Primary owner:
- [API error routing](api/error-routing.md)

Related references:
- [API error codes](api/error-codes.md)
- [API blocker routing](api/blocker-routing.md)

Usage note:
- Use the focused API error owners for neighboring concerns.

### blocker routing

Term:
- blocker routing
- close-readiness blocker routing
- API blocker routing, when naming the owner document

Korean term:
- 차단 사유 처리 경로

Type:
- API blocker-routing term

Meaning:
- Blocker routing covers the boundary between close-readiness blockers and API response branches.

Primary owner:
- [API blocker routing](api/blocker-routing.md)

Related references:
- [API error routing](api/error-routing.md)
- [Core Model](core-model.md)
- [Close-task method](api/method-close-task.md)

Usage note:
- Use the blocker-routing owner for this boundary.

### error/blocker boundary

Term:
- error/blocker boundary
- API error versus close-readiness blocker boundary

Korean term:
- 오류와 차단 사유의 경계

Type:
- API blocker-routing term

Meaning:
- The error/blocker boundary names the distinction between API errors and close-readiness blockers.

Primary owner:
- [API blocker routing](api/blocker-routing.md)

Related references:
- [API error codes](api/error-codes.md)

Usage note:
- Use API blocker routing for this boundary.

### public error as blocker

Term:
- public error as blocker
- public `ErrorCode` as blocker

Korean term:
- 공개 오류 코드가 차단 사유로 표현되는 경우

Type:
- API blocker-routing term

Meaning:
- Public error as blocker names a blocker-routing concern involving public error codes.

Primary owner:
- [API blocker routing](api/blocker-routing.md)

Related references:
- [API error codes](api/error-codes.md)

Usage note:
- Use API blocker routing for this narrow case.

### `ToolError.details`

Term:
- `ToolError.details`

Korean term:
- `ToolError.details`; user-facing prose may use 오류 세부사항 when not naming the exact API identifier.

Type:
- API detail identifier

Meaning:
- `ToolError.details` is the exact API detail identifier for machine-readable error details.

Primary owner:
- [API error details](api/error-details.md)

Related references:
- [API error codes](api/error-codes.md)

Usage note:
- Use API error details for nested detail values.

### detail helper values

Term:
- detail helper values
- error detail helper values

Korean term:
- 오류 세부사항 보조 값

Type:
- API detail term

Meaning:
- Detail helper values name nested values under `ToolError.details`.

Primary owner:
- [API error details](api/error-details.md)

Related references:
- None.

Usage note:
- Use API error details for nested detail values.

### dry-run preview routing

Term:
- dry-run preview routing
- `dry_run` preview response branch routing

Korean term:
- dry-run 미리보기 처리 경로

Type:
- API preview-routing term

Meaning:
- Dry-run preview routing names the response-branch routing concern for `dry_run` previews.

Primary owner:
- [API error routing](api/error-routing.md)

Related references:
- [API Methods](api/methods.md)

Usage note:
- Use focused method and schema owners for neighboring details.

### blocked result

Term:
- blocked result

Korean term:
- 차단 결과

Type:
- API result term

Meaning:
- Blocked result names an API result branch for a valid but blocked operation.

Primary owner:
- [API error routing](api/error-routing.md)

Related references:
- [API Methods](api/methods.md)

Usage note:
- Use API error routing for result-branch detail.

### rejected response

Term:
- rejected response

Korean term:
- 거부 응답

Type:
- API response branch

Meaning:
- Rejected response names an API response branch for a rejected request.

Primary owner:
- [API error routing](api/error-routing.md)

Related references:
- [API Schema Core](api/schema-core.md)
- [API error codes](api/error-codes.md)

Usage note:
- Use API error routing for branch detail.

### migration

Term:
- migration

Korean term:
- 마이그레이션

Type:
- storage term

Meaning:
- Migration is a technical schema, storage, data, or documentation migration concept.

Primary owner:
- [Storage Versioning](storage-versioning.md)

Related references:
- [Storage Overview](storage.md)

Usage note:
- Use 마이그레이션 for the technical concept in Korean prose.

### lifecycle

Term:
- lifecycle

Korean term:
- 생명주기

Type:
- lifecycle term

Meaning:
- Lifecycle is the phase progression of a concept such as a `Task` or artifact handle.

Primary owner:
- [Core Model](core-model.md)

Related references:
- [API Value Sets](api/schema-value-sets.md)

Usage note:
- Use exact identifiers when naming lifecycle fields or values.
