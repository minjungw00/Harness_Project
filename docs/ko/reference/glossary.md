# 용어집

이 용어집은 하네스 문서의 용어를 사람이 읽기 쉽게 안내합니다. 용어의 뜻을 빠르게 확인하고, 이어서 읽을 주 담당 문서를 찾을 때 사용합니다.

구조화된 용어 메타데이터, 식별자 보존 통제, 한국어 혼합어 통제는 [docs/terminology-map.yaml](../../terminology-map.yaml)에 있습니다. 정확한 API 동작, 스키마, 저장 효과, 보안 보장, 닫기 준비 상태 동작, 오류 처리 경로는 연결된 담당 문서를 따릅니다.

## 요약

| 용어 | 한국어 용어 | 짧은 의미 | 주 담당 문서 |
|---|---|---|---|
| Harness | 하네스 | AI 지원 제품 작업을 위한 로컬 작업 권한 서버입니다. | [기준 범위](scope.md) |
| Product Repository | Product Repository; 사용자 문서에서는 제품 저장소 | 하네스 런타임 상태와 구분되는 사용자의 프로젝트 작업 공간입니다. | [런타임 경계](runtime-boundaries.md) |
| Harness Runtime Home | Harness Runtime Home; 사용자 문서에서는 런타임 홈 | 하네스 기록과 아티팩트를 담는 운영 데이터 공간입니다. | [런타임 경계](runtime-boundaries.md) |
| documentation | 문서 | 유지되는 원천 자료이며 런타임 출력, 제품 구현, 수락 상태와 구분됩니다. | [작성 가이드](../maintain/authoring-guide.md) |
| semantic skeleton | 의미 골격 | 중요한 참조 섹션을 쓰거나 다듬기 전에 정하는 의미 단위 구조입니다. | [작성 가이드](../maintain/authoring-guide.md) |
| baseline scope | 기준 범위 | 하네스 문서가 정의한 안정적인 지원 경계입니다. | [기준 범위](scope.md) |
| supported scope | 지원 범위 | 지원된다고 문서화된 동작이나 역량입니다. | [기준 범위](scope.md) |
| supported behavior | 지원 동작 | 기준 범위와 영향받는 의미 담당 문서가 지원한다고 문서화한 동작입니다. | [기준 범위](scope.md) |
| supported API method | 지원되는 API 메서드 | 지원된다고 문서화된 공개 API 메서드입니다. | [API 메서드](api/methods.md) |
| supported API value | 지원되는 API 값 | 단순히 예약되었거나 이름만 있는 값이 아니라 지원된다고 문서화된 API 값입니다. | [API 값 집합](api/schema-value-sets.md) |
| out-of-scope capability | 지원 범위 밖 기능 | 적용되는 담당 문서들이 승격하기 전까지 기준 지원 경계 밖에 있는 보류된 기능입니다. | [기준 범위](scope.md) |
| evidence collection workflow | 증거 수집 흐름 | 지원 여부가 기준 범위에 속하는 증거 흐름 기능을 가리키는 표현입니다. | [기준 범위](scope.md) |
| expanded or additional evidence collection workflows | 확장 또는 추가 증거 수집 흐름 | 기준 범위와 영향받는 담당 문서가 승격하기 전까지 지원 범위 밖에 있는 증거 수집 흐름 묶음입니다. | [기준 범위](scope.md) |
| owner document | 담당 문서 | 용어, 제품 개념, 계약을 정의하는 기준 문서입니다. | [작성 가이드](../maintain/authoring-guide.md) |
| owner contract | 담당 계약 | 관련 담당 문서가 정의한 계약입니다. | [작성 가이드](../maintain/authoring-guide.md) |
| applicable owner path | 적용되는 담당 경로 | 질문이나 개념에 맞는 집중 담당 문서로 가는 문서 경로입니다. | [작성 가이드](../maintain/authoring-guide.md) |
| applicable reference | 적용되는 참조 문서 | 관련 계약을 정의하는 참조 문서입니다. | [참조 색인](README.md) |
| existing owner | 기존 담당 문서 | 이미 존재하며 규범 의미를 담을 수 있는 기준 담당 문서입니다. | [작성 가이드](../maintain/authoring-guide.md) |
| promotion-time owner update | 승격 시점의 담당 문서 갱신 | 지원 범위 밖 기능을 지원 범위로 승격할 때 필요한 담당 문서 변경입니다. | [기준 범위](scope.md) |
| owner placeholder | 담당 문서 자리표시자 | 지원 전까지 담당 문서를 만들거나 지정해야 함을 표시하는 표지입니다. | [작성 가이드](../maintain/authoring-guide.md) |
| `Task` | `Task` | 작업 범위, 권한 맥락, 판단, 증거, 닫기 준비 상태를 함께 묶는 하네스 엔터티입니다. | [Core 모델](core-model.md) |
| scope | 범위 | `Task` 또는 Change Unit 맥락에 붙는 작업 또는 권한 경계입니다. | [Core 모델](core-model.md) |
| active scope | 현재 적용 범위 | `Task` 또는 Change Unit 맥락 안에서 현재 적용되는 범위입니다. | [Core 모델](core-model.md) |
| active Change Unit | 현재 적용 Change Unit | 권한 모델에서 현재 적용되는 Change Unit입니다. | [Core 모델](core-model.md) |
| user-owned judgment | 사용자 소유 판단 | 하네스가 기록하지만 Core 소유 사실로 바꾸지 않는 사용자 소유의 결정이나 평가입니다. | [Core 모델](core-model.md) |
| `UserJudgment` | `UserJudgment` | 사용자 소유 판단 데이터를 나타내는 API 스키마 식별자입니다. | [API 판단 스키마](api/schema-judgment.md) |
| close readiness | 닫기 준비 상태 | 현재 상태에서 `Task`를 닫을 준비가 되었는지를 나타내는 Core 개념입니다. | [Core 모델](core-model.md) |
| close readiness evaluation | 닫기 준비 상태 평가 | `harness.close_task`가 사용하는 메서드별 평가입니다. | [Task 닫기 메서드](api/method-close-task.md) |
| close task | Task 닫기 | `Task` 닫기를 시도하는 사용자 또는 API 동작입니다. | [Task 닫기 메서드](api/method-close-task.md) |
| close task behavior | Task 닫기 동작 | `Task` 닫기에 적용되는 메서드별 요청, 평가, 결과 동작입니다. | [Task 닫기 메서드](api/method-close-task.md) |
| `harness.close_task` | `harness.close_task` | Task 닫기 동작의 공개 API 메서드 식별자입니다. | [Task 닫기 메서드](api/method-close-task.md) |
| close-readiness blocker | 닫기 차단 사유 | 닫기 준비 상태가 진행될 수 없을 때 드러나는 사유입니다. | [API 차단 사유 처리 경로](api/blocker-routing.md) |
| `CloseReadinessBlocker` | `CloseReadinessBlocker` | 닫기 차단 사유 데이터를 나타내는 스키마 식별자입니다. | [API 상태 스키마](api/schema-state.md) |
| blocker category | 차단 사유 범주 | 닫기 차단 사유의 범주 개념과 값 묶음입니다. | [API 값 집합](api/schema-value-sets.md) |
| blocker | 차단 사유 | 막힘의 이유를 가리키는 일반 산문 용어입니다. | [용어 지도](../../terminology-map.yaml) |
| complete intent | `complete` | 일반 산문의 "전체" 의미와 구분되는 `complete` intent 값입니다. | [API 값 집합](api/schema-value-sets.md) |
| full evaluation order | 전체 평가 순서 | `complete` 값이 아니라 평가 순서 전체를 가리키는 일반 산문 표현입니다. | [번역 가이드](../maintain/translation-guide.md) |
| artifact | 아티팩트 | 참조되거나 스테이징되는 작업 자료를 가리키는 하네스 아티팩트 개념입니다. | [API 아티팩트 스키마](api/schema-artifacts.md) |
| evidence | 증거 | 주장, 검증 결과, 사용자 판단 맥락을 뒷받침하기 위해 기록되는 자료입니다. | [Core 모델](core-model.md) |
| `ArtifactRef` | `ArtifactRef` | 지속된 아티팩트 참조를 나타내는 스키마 식별자입니다. | [API 아티팩트 스키마](api/schema-artifacts.md) |
| `ArtifactInput` | `ArtifactInput` | 아티팩트 입력 데이터를 나타내는 스키마 식별자입니다. | [API 아티팩트 스키마](api/schema-artifacts.md) |
| `StagedArtifactHandle` | `StagedArtifactHandle` | 스테이징된 아티팩트 핸들을 나타내는 식별자입니다. | [API 아티팩트 스키마](api/schema-artifacts.md) |
| projection | 상태 보기 | 읽기 전용 상태 보기입니다. | [상태 보기 권한 참조](projection-and-templates.md) |
| `Projection` | `Projection` | 읽기 전용 상태 보기 개념을 가리키는 정확한 제품 라벨입니다. | [상태 보기 권한 참조](projection-and-templates.md) |
| surface | 접점 | 맥락이 드러나는 통합 또는 상호작용 경계입니다. | [에이전트 통합](agent-integration.md) |
| `surface_id` | `surface_id` | 접점을 나타내는 정확한 식별자입니다. | [에이전트 통합](agent-integration.md) |
| active surface context | 현재 적용 접점 맥락 | 요청이나 상호작용에 현재 적용되는 접점 맥락입니다. | [에이전트 통합](agent-integration.md) |
| `state_version` | `state_version` | 저장된 프로젝트 상태의 상태 시계 식별자입니다. | [저장소 버전 관리](storage-versioning.md) |
| runtime | 런타임 | 하네스의 운영 실행과 데이터 맥락입니다. | [런타임 경계](runtime-boundaries.md) |
| `Write Authorization` | 쓰기 권한 부여 | 하네스의 쓰기 권한 부여 개념을 가리키는 정확한 제품 라벨입니다. | [Core 모델](core-model.md) |
| sensitive approval | 민감 동작 승인 | `Write Authorization`과 구분되는 민감 동작에 대한 사용자 승인입니다. | [Core 모델](core-model.md) |
| access class | 접근 등급 | 접근 맥락을 나타내는 값 범주입니다. | [API 값 집합](api/schema-value-sets.md) |
| baseline guarantee | 기준 범위 보장 | 기준 범위에 대해 문서화된 보장입니다. | [보안](security.md) |
| cooperative guarantee | 협력형 보장 | 협력적 동작을 전제로 하는 보안 보장 유형입니다. | [보안](security.md) |
| detective guarantee | 탐지형 보장 | 관찰 가능한 탐지를 바탕으로 하는 보안 보장 유형입니다. | [보안](security.md) |
| design-quality owner boundary | 설계 품질 담당 경계 | 설계 품질 관찰을 관련 담당 문서로 보내는 경계입니다. | [설계 품질](design-quality.md) |
| reserved value | 예약된 값 | 그 자체만으로 기준 동작을 뜻하지 않는, 어휘나 표면 영역으로 예약된 값입니다. | [기준 범위](scope.md) |
| profile-gated value | 프로필 조건부 값 | 문서화된 프로필이나 게이트가 지원할 때만 사용할 수 있는 값입니다. | [기준 범위](scope.md) |
| `ErrorCode` | `ErrorCode` | 공개 API 오류 코드 식별자입니다. | [API 오류 코드](api/error-codes.md) |
| error code meanings | 공개 오류 코드 의미 | 공개 API 오류 코드의 의미와 발생 요약입니다. | [API 오류 코드](api/error-codes.md) |
| error precedence | 오류 우선순위 | 공개 API 오류를 선택하고 정렬하는 규칙입니다. | [API 오류 우선순위](api/error-precedence.md) |
| error routing | 오류 처리 경로 | 거부 응답, 차단 결과, dry-run 응답 분기를 처리하는 경로입니다. | [API 오류 처리 경로](api/error-routing.md) |
| blocker routing | 차단 사유 처리 경로 | 닫기 차단 사유와 API 응답 분기 사이의 경계입니다. | [API 차단 사유 처리 경로](api/blocker-routing.md) |
| error/blocker boundary | 오류와 차단 사유의 경계 | 공개 API 오류와 닫기 차단 사유 데이터의 구분입니다. | [API 차단 사유 처리 경로](api/blocker-routing.md) |
| public error as blocker | 공개 오류 코드가 차단 사유로 표현되는 경우 | 공개 오류 코드 표현이 차단 사유 데이터에 나타나는 경우를 설명하는 경계 용어입니다. | [API 차단 사유 처리 경로](api/blocker-routing.md) |
| `ToolError.details` | `ToolError.details` | 기계 판독용 오류 세부사항 필드입니다. | [API 오류 세부사항](api/error-details.md) |
| error detail helper values | 오류 세부사항 보조 값 | 기계 판독용 오류 세부사항 안에 들어가는 보조 값입니다. | [API 오류 세부사항](api/error-details.md) |
| dry-run | dry-run 미리보기 | `dry_run`을 사용하는 API 호출의 미리보기 모드입니다. | [API 코어 스키마](api/schema-core.md) |
| dry-run preview routing | dry-run 미리보기 처리 경로 | `dry_run` 미리보기의 응답 분기를 처리하는 경로입니다. | [API 오류 처리 경로](api/error-routing.md) |
| blocked result | 차단 결과 | 거부 응답이 아니라 차단을 보고하는 결과 분기입니다. | [API 오류 처리 경로](api/error-routing.md) |
| rejected response | 거부 응답 | 작업이 진행되기 전에 요청이 거부되었음을 나타내는 API 응답입니다. | [API 오류 처리 경로](api/error-routing.md) |
| migration | 마이그레이션 | 스키마, 저장소, 데이터, 문서에 적용되는 기술적 마이그레이션입니다. | [저장소 버전 관리](storage-versioning.md) |
| lifecycle | 생명주기 | 엔터티나 아티팩트가 시간에 따라 거치는 단계입니다. | [Core 모델](core-model.md) |

## 용어

### Harness

- 용어: Harness
- 한국어 용어: 하네스
- 의미: AI 지원 제품 작업을 위한 로컬 작업 권한 서버입니다.
- 주 담당 문서: [기준 범위](scope.md)
- 함께 볼 문서: [런타임 경계](runtime-boundaries.md)

### Product Repository

- 용어: Product Repository
- 한국어 용어: Product Repository; 사용자 문서에서는 제품 저장소
- 의미: 하네스 런타임 상태와 구분되는 사용자의 프로젝트 작업 공간입니다.
- 주 담당 문서: [런타임 경계](runtime-boundaries.md)

### Harness Runtime Home

- 용어: Harness Runtime Home
- 한국어 용어: Harness Runtime Home; 사용자 문서에서는 런타임 홈
- 의미: 하네스 기록과 아티팩트를 담는 운영 데이터 공간입니다.
- 주 담당 문서: [런타임 경계](runtime-boundaries.md)

### documentation

- 용어: documentation
- 한국어 용어: 문서
- 의미: 유지되는 원천 자료이며 런타임 출력, 제품 구현, 수락 상태와 구분됩니다.
- 주 담당 문서: [작성 가이드](../maintain/authoring-guide.md)
- 함께 볼 문서: [런타임 경계](runtime-boundaries.md), [구현 가이드](../build/implementation-guide.md)

### semantic skeleton

- 용어: semantic skeleton
- 한국어 용어: 의미 골격
- 의미: 중요한 참조 섹션을 쓰거나 다듬기 전에 정하는 의미 단위 구조입니다.
- 주 담당 문서: [작성 가이드](../maintain/authoring-guide.md)
- 함께 볼 문서: [구조 점검](../maintain/checks/structure.md)

### baseline scope

- 용어: baseline scope
- 한국어 용어: 기준 범위
- 의미: 하네스 문서가 정의한 안정적인 지원 경계입니다.
- 주 담당 문서: [기준 범위](scope.md)
- 함께 볼 문서: [API 값 집합](api/schema-value-sets.md)

### supported scope

- 용어: supported scope
- 한국어 용어: 지원 범위
- 의미: 지원된다고 문서화된 동작이나 역량입니다.
- 주 담당 문서: [기준 범위](scope.md)

### supported behavior

- 용어: supported behavior
- 한국어 용어: 지원 동작
- 의미: 기준 범위와 영향받는 의미 담당 문서가 지원한다고 문서화한 동작입니다.
- 주 담당 문서: [기준 범위](scope.md)
- 함께 볼 문서: [API 값 집합](api/schema-value-sets.md)

### supported API method

- 용어: supported API method
- 한국어 용어: 지원되는 API 메서드
- 의미: 지원된다고 문서화된 공개 API 메서드입니다.
- 주 담당 문서: [API 메서드](api/methods.md)

### supported API value

- 용어: supported API value
- 한국어 용어: 지원되는 API 값
- 의미: 단순히 예약되었거나 이름만 있는 값이 아니라 지원된다고 문서화된 API 값입니다.
- 주 담당 문서: [API 값 집합](api/schema-value-sets.md)
- 함께 볼 문서: [기준 범위](scope.md)

### out-of-scope capability

- 용어: out-of-scope capability
- 한국어 용어: 지원 범위 밖 기능
- 의미: 적용되는 담당 문서들이 승격하기 전까지 기준 지원 경계 밖에 있는 보류된 기능입니다.
- 주 담당 문서: [기준 범위](scope.md)

### evidence collection workflow

- 용어: evidence collection workflow
- 한국어 용어: 증거 수집 흐름
- 의미: 지원 여부가 기준 범위에 속하는 증거 흐름 기능을 가리키는 표현입니다.
- 주 담당 문서: [기준 범위](scope.md)

### expanded or additional evidence collection workflows

- 용어: expanded or additional evidence collection workflows
- 한국어 용어: 확장 또는 추가 증거 수집 흐름
- 의미: 기준 범위와 영향받는 담당 문서가 승격하기 전까지 지원 범위 밖에 있는 증거 수집 흐름 묶음입니다.
- 주 담당 문서: [기준 범위](scope.md)

### owner document

- 용어: owner document
- 한국어 용어: 담당 문서
- 의미: 용어, 제품 개념, 계약을 정의하는 기준 문서입니다.
- 주 담당 문서: [작성 가이드](../maintain/authoring-guide.md)
- 함께 볼 문서: [참조 색인](README.md)

### owner contract

- 용어: owner contract
- 한국어 용어: 담당 계약
- 의미: 관련 담당 문서가 정의한 계약입니다.
- 주 담당 문서: [작성 가이드](../maintain/authoring-guide.md)

### applicable owner path

- 용어: applicable owner path
- 한국어 용어: 적용되는 담당 경로
- 의미: 질문이나 개념에 맞는 집중 담당 문서로 가는 문서 경로입니다.
- 주 담당 문서: [작성 가이드](../maintain/authoring-guide.md)
- 함께 볼 문서: [참조 색인](README.md), [doc-index.yaml](../../doc-index.yaml)

### applicable reference

- 용어: applicable reference
- 한국어 용어: 적용되는 참조 문서
- 의미: 관련 계약을 정의하는 참조 문서입니다.
- 주 담당 문서: [참조 색인](README.md)
- 함께 볼 문서: [작성 가이드](../maintain/authoring-guide.md)

### existing owner

- 용어: existing owner
- 한국어 용어: 기존 담당 문서
- 의미: 이미 존재하며 규범 의미를 담을 수 있는 기준 담당 문서입니다.
- 주 담당 문서: [작성 가이드](../maintain/authoring-guide.md)
- 함께 볼 문서: [참조 색인](README.md)

### promotion-time owner update

- 용어: promotion-time owner update
- 한국어 용어: 승격 시점의 담당 문서 갱신
- 의미: 지원 범위 밖 기능을 지원 범위로 승격할 때 필요한 담당 문서 변경입니다.
- 주 담당 문서: [기준 범위](scope.md)
- 함께 볼 문서: [작성 가이드](../maintain/authoring-guide.md)

### owner placeholder

- 용어: owner placeholder
- 한국어 용어: 담당 문서 자리표시자
- 의미: 지원 전까지 담당 문서를 만들거나 지정해야 함을 표시하는 표지입니다.
- 주 담당 문서: [작성 가이드](../maintain/authoring-guide.md)
- 함께 볼 문서: [기준 범위](scope.md)

### `Task`

- 용어: `Task`
- 한국어 용어: `Task`
- 의미: 작업 범위, 권한 맥락, 판단, 증거, 닫기 준비 상태를 함께 묶는 하네스 엔터티입니다.
- 주 담당 문서: [Core 모델](core-model.md)
- 함께 볼 문서: [API 상태 스키마](api/schema-state.md)

### scope

- 용어: scope
- 한국어 용어: 범위
- 의미: `Task` 또는 Change Unit 맥락에 붙는 작업 또는 권한 경계입니다.
- 주 담당 문서: [Core 모델](core-model.md)
- 함께 볼 문서: [범위 갱신 메서드](api/method-update-scope.md)

### active scope

- 용어: active scope
- 한국어 용어: 현재 적용 범위
- 의미: `Task` 또는 Change Unit 맥락 안에서 현재 적용되는 범위입니다.
- 주 담당 문서: [Core 모델](core-model.md)
- 함께 볼 문서: [범위 갱신 메서드](api/method-update-scope.md)

### active Change Unit

- 용어: active Change Unit
- 한국어 용어: 현재 적용 Change Unit
- 의미: 권한 모델에서 현재 적용되는 Change Unit입니다.
- 주 담당 문서: [Core 모델](core-model.md)
- 함께 볼 문서: [범위 갱신 메서드](api/method-update-scope.md)

### user-owned judgment

- 용어: user-owned judgment
- 한국어 용어: 사용자 소유 판단
- 의미: 하네스가 기록하지만 Core 소유 사실로 바꾸지 않는 사용자 소유의 결정이나 평가입니다.
- 주 담당 문서: [Core 모델](core-model.md)
- 함께 볼 문서: [API 판단 스키마](api/schema-judgment.md)

### `UserJudgment`

- 용어: `UserJudgment`
- 한국어 용어: `UserJudgment`
- 의미: 사용자 소유 판단 데이터를 나타내는 API 스키마 식별자입니다.
- 주 담당 문서: [API 판단 스키마](api/schema-judgment.md)
- 함께 볼 문서: [Core 모델](core-model.md)

### close readiness

- 용어: close readiness
- 한국어 용어: 닫기 준비 상태
- 의미: 현재 상태에서 `Task`를 닫을 준비가 되었는지를 나타내는 Core 개념입니다.
- 주 담당 문서: [Core 모델](core-model.md)
- 함께 볼 문서: [Task 닫기 메서드](api/method-close-task.md), [API 차단 사유 처리 경로](api/blocker-routing.md)

### close readiness evaluation

- 용어: close readiness evaluation
- 한국어 용어: 닫기 준비 상태 평가
- 의미: `harness.close_task`가 사용하는 메서드별 평가입니다.
- 주 담당 문서: [Task 닫기 메서드](api/method-close-task.md)
- 함께 볼 문서: [Core 모델](core-model.md), [API 차단 사유 처리 경로](api/blocker-routing.md)

### close task

- 용어: close task
- 한국어 용어: Task 닫기
- 의미: `Task` 닫기를 시도하는 사용자 또는 API 동작입니다.
- 주 담당 문서: [Task 닫기 메서드](api/method-close-task.md)
- 함께 볼 문서: [API 메서드](api/methods.md)

### close task behavior

- 용어: close task behavior
- 한국어 용어: Task 닫기 동작
- 의미: `Task` 닫기에 적용되는 메서드별 요청, 평가, 결과 동작입니다.
- 주 담당 문서: [Task 닫기 메서드](api/method-close-task.md)
- 함께 볼 문서: [API 메서드](api/methods.md)

### `harness.close_task`

- 용어: `harness.close_task`
- 한국어 용어: `harness.close_task`
- 의미: Task 닫기 동작의 공개 API 메서드 식별자입니다.
- 주 담당 문서: [Task 닫기 메서드](api/method-close-task.md)
- 함께 볼 문서: [API 메서드](api/methods.md)

### close-readiness blocker

- 용어: close-readiness blocker
- 한국어 용어: 닫기 차단 사유
- 의미: 닫기 준비 상태가 진행될 수 없을 때 드러나는 사유입니다.
- 주 담당 문서: [API 차단 사유 처리 경로](api/blocker-routing.md)
- 함께 볼 문서: [Core 모델](core-model.md), [API 상태 스키마](api/schema-state.md)

### `CloseReadinessBlocker`

- 용어: `CloseReadinessBlocker`
- 한국어 용어: `CloseReadinessBlocker`
- 의미: 닫기 차단 사유 데이터를 나타내는 스키마 식별자입니다.
- 주 담당 문서: [API 상태 스키마](api/schema-state.md)
- 함께 볼 문서: [API 차단 사유 처리 경로](api/blocker-routing.md)

### blocker category

- 용어: blocker category
- 한국어 용어: 차단 사유 범주
- 의미: 닫기 차단 사유의 범주 개념과 값 묶음입니다.
- 주 담당 문서: [API 값 집합](api/schema-value-sets.md)
- 함께 볼 문서: [API 상태 스키마](api/schema-state.md), [API 차단 사유 처리 경로](api/blocker-routing.md)

### blocker

- 용어: blocker
- 한국어 용어: 차단 사유
- 의미: 막힘의 이유를 가리키는 일반 산문 용어입니다.
- 주 담당 문서: [용어 지도](../../terminology-map.yaml)
- 함께 볼 문서: [API 차단 사유 처리 경로](api/blocker-routing.md)

### complete intent

- 용어: complete intent
- 한국어 용어: `complete`
- 의미: 일반 산문의 "전체" 의미와 구분되는 `complete` intent 값입니다.
- 주 담당 문서: [API 값 집합](api/schema-value-sets.md)
- 함께 볼 문서: [Task 닫기 메서드](api/method-close-task.md)

### full evaluation order

- 용어: full evaluation order
- 한국어 용어: 전체 평가 순서
- 의미: `complete` 값이 아니라 평가 순서 전체를 가리키는 일반 산문 표현입니다.
- 주 담당 문서: [번역 가이드](../maintain/translation-guide.md)
- 함께 볼 문서: [용어 지도](../../terminology-map.yaml)

### artifact

- 용어: artifact
- 한국어 용어: 아티팩트
- 의미: 참조되거나 스테이징되는 작업 자료를 가리키는 하네스 아티팩트 개념입니다.
- 주 담당 문서: [API 아티팩트 스키마](api/schema-artifacts.md)
- 함께 볼 문서: [아티팩트 저장소](storage-artifacts.md)

### evidence

- 용어: evidence
- 한국어 용어: 증거
- 의미: 주장, 검증 결과, 사용자 판단 맥락을 뒷받침하기 위해 기록되는 자료입니다.
- 주 담당 문서: [Core 모델](core-model.md)
- 함께 볼 문서: [API 상태 스키마](api/schema-state.md), [실행 기록 메서드](api/method-record-run.md)

### `ArtifactRef`

- 용어: `ArtifactRef`
- 한국어 용어: `ArtifactRef`
- 의미: 지속된 아티팩트 참조를 나타내는 스키마 식별자입니다.
- 주 담당 문서: [API 아티팩트 스키마](api/schema-artifacts.md)
- 함께 볼 문서: [아티팩트 저장소](storage-artifacts.md)

### `ArtifactInput`

- 용어: `ArtifactInput`
- 한국어 용어: `ArtifactInput`
- 의미: 아티팩트 입력 데이터를 나타내는 스키마 식별자입니다.
- 주 담당 문서: [API 아티팩트 스키마](api/schema-artifacts.md)

### `StagedArtifactHandle`

- 용어: `StagedArtifactHandle`
- 한국어 용어: `StagedArtifactHandle`
- 의미: 스테이징된 아티팩트 핸들을 나타내는 식별자입니다.
- 주 담당 문서: [API 아티팩트 스키마](api/schema-artifacts.md)
- 함께 볼 문서: [아티팩트 저장소](storage-artifacts.md)

### projection

- 용어: projection
- 한국어 용어: 상태 보기
- 의미: 읽기 전용 상태 보기입니다.
- 주 담당 문서: [상태 보기 권한 참조](projection-and-templates.md)
- 함께 볼 문서: [템플릿 본문](template-bodies.md)

### `Projection`

- 용어: `Projection`
- 한국어 용어: `Projection`
- 의미: 읽기 전용 상태 보기 개념을 가리키는 정확한 제품 라벨입니다.
- 주 담당 문서: [상태 보기 권한 참조](projection-and-templates.md)
- 함께 볼 문서: [템플릿 본문](template-bodies.md)

### surface

- 용어: surface
- 한국어 용어: 접점
- 의미: 맥락이 드러나는 통합 또는 상호작용 경계입니다.
- 주 담당 문서: [에이전트 통합](agent-integration.md)
- 함께 볼 문서: [보안](security.md)

### `surface_id`

- 용어: `surface_id`
- 한국어 용어: `surface_id`
- 의미: 접점을 나타내는 정확한 식별자입니다.
- 주 담당 문서: [에이전트 통합](agent-integration.md)
- 함께 볼 문서: [API 코어 스키마](api/schema-core.md)

### active surface context

- 용어: active surface context
- 한국어 용어: 현재 적용 접점 맥락
- 의미: 요청이나 상호작용에 현재 적용되는 접점 맥락입니다.
- 주 담당 문서: [에이전트 통합](agent-integration.md)
- 함께 볼 문서: [보안](security.md)

### `state_version`

- 용어: `state_version`
- 한국어 용어: `state_version`
- 의미: 저장된 프로젝트 상태의 상태 시계 식별자입니다.
- 주 담당 문서: [저장소 버전 관리](storage-versioning.md)
- 함께 볼 문서: [API 코어 스키마](api/schema-core.md)

### runtime

- 용어: runtime
- 한국어 용어: 런타임
- 의미: 하네스의 운영 실행과 데이터 맥락입니다.
- 주 담당 문서: [런타임 경계](runtime-boundaries.md)
- 함께 볼 문서: [보안](security.md)

### `Write Authorization`

- 용어: `Write Authorization`
- 한국어 용어: 쓰기 권한 부여
- 의미: 하네스의 쓰기 권한 부여 개념을 가리키는 정확한 제품 라벨입니다.
- 주 담당 문서: [Core 모델](core-model.md)
- 함께 볼 문서: [보안](security.md), [쓰기 준비 메서드](api/method-prepare-write.md)

### sensitive approval

- 용어: sensitive approval
- 한국어 용어: 민감 동작 승인
- 의미: `Write Authorization`과 구분되는 민감 동작에 대한 사용자 승인입니다.
- 주 담당 문서: [Core 모델](core-model.md)
- 함께 볼 문서: [API 판단 스키마](api/schema-judgment.md), [보안](security.md)

### access class

- 용어: access class
- 한국어 용어: 접근 등급
- 의미: 접근 맥락을 나타내는 값 범주입니다.
- 주 담당 문서: [API 값 집합](api/schema-value-sets.md)
- 함께 볼 문서: [에이전트 통합](agent-integration.md), [보안](security.md)

### baseline guarantee

- 용어: baseline guarantee
- 한국어 용어: 기준 범위 보장
- 의미: 기준 범위에 대해 문서화된 보장입니다.
- 주 담당 문서: [보안](security.md)
- 함께 볼 문서: [기준 범위](scope.md)

### cooperative guarantee

- 용어: cooperative guarantee
- 한국어 용어: 협력형 보장
- 의미: 협력적 동작을 전제로 하는 보안 보장 유형입니다.
- 주 담당 문서: [보안](security.md)

### detective guarantee

- 용어: detective guarantee
- 한국어 용어: 탐지형 보장
- 의미: 관찰 가능한 탐지를 바탕으로 하는 보안 보장 유형입니다.
- 주 담당 문서: [보안](security.md)
- 함께 볼 문서: [에이전트 통합](agent-integration.md)

### design-quality owner boundary

- 용어: design-quality owner boundary
- 한국어 용어: 설계 품질 담당 경계
- 의미: 설계 품질 관찰을 관련 담당 문서로 보내는 경계입니다.
- 주 담당 문서: [설계 품질](design-quality.md)

### reserved value

- 용어: reserved value
- 한국어 용어: 예약된 값
- 의미: 그 자체만으로 기준 동작을 뜻하지 않는, 어휘나 표면 영역으로 예약된 값입니다.
- 주 담당 문서: [기준 범위](scope.md)
- 함께 볼 문서: [API 값 집합](api/schema-value-sets.md)

### profile-gated value

- 용어: profile-gated value
- 한국어 용어: 프로필 조건부 값
- 의미: 문서화된 프로필이나 게이트가 지원할 때만 사용할 수 있는 값입니다.
- 주 담당 문서: [기준 범위](scope.md)
- 함께 볼 문서: [API 값 집합](api/schema-value-sets.md)

### `ErrorCode`

- 용어: `ErrorCode`
- 한국어 용어: `ErrorCode`
- 의미: 공개 API 오류 코드 식별자입니다.
- 주 담당 문서: [API 오류 코드](api/error-codes.md)
- 함께 볼 문서: [API 오류 우선순위](api/error-precedence.md), [API 오류 처리 경로](api/error-routing.md)

### error code meanings

- 용어: error code meanings
- 한국어 용어: 공개 오류 코드 의미
- 의미: 공개 API 오류 코드의 의미와 발생 요약입니다.
- 주 담당 문서: [API 오류 코드](api/error-codes.md)
- 함께 볼 문서: [API 오류 우선순위](api/error-precedence.md)

### error precedence

- 용어: error precedence
- 한국어 용어: 오류 우선순위
- 의미: 공개 API 오류를 선택하고 정렬하는 규칙입니다.
- 주 담당 문서: [API 오류 우선순위](api/error-precedence.md)
- 함께 볼 문서: [API 오류 코드](api/error-codes.md)

### error routing

- 용어: error routing
- 한국어 용어: 오류 처리 경로
- 의미: 거부 응답, 차단 결과, dry-run 응답 분기를 처리하는 경로입니다.
- 주 담당 문서: [API 오류 처리 경로](api/error-routing.md)
- 함께 볼 문서: [API 오류 코드](api/error-codes.md), [API 차단 사유 처리 경로](api/blocker-routing.md)

### blocker routing

- 용어: blocker routing
- 한국어 용어: 차단 사유 처리 경로
- 의미: 닫기 차단 사유와 API 응답 분기 사이의 경계입니다.
- 주 담당 문서: [API 차단 사유 처리 경로](api/blocker-routing.md)
- 함께 볼 문서: [API 오류 처리 경로](api/error-routing.md), [Task 닫기 메서드](api/method-close-task.md)

### error/blocker boundary

- 용어: error/blocker boundary
- 한국어 용어: 오류와 차단 사유의 경계
- 의미: 공개 API 오류와 닫기 차단 사유 데이터의 구분입니다.
- 주 담당 문서: [API 차단 사유 처리 경로](api/blocker-routing.md)
- 함께 볼 문서: [API 오류 코드](api/error-codes.md)

### public error as blocker

- 용어: public error as blocker
- 한국어 용어: 공개 오류 코드가 차단 사유로 표현되는 경우
- 의미: 공개 오류 코드 표현이 차단 사유 데이터에 나타나는 경우를 설명하는 경계 용어입니다.
- 주 담당 문서: [API 차단 사유 처리 경로](api/blocker-routing.md)
- 함께 볼 문서: [API 오류 코드](api/error-codes.md)

### `ToolError.details`

- 용어: `ToolError.details`
- 한국어 용어: `ToolError.details`
- 의미: 기계 판독용 오류 세부사항 필드입니다.
- 주 담당 문서: [API 오류 세부사항](api/error-details.md)
- 함께 볼 문서: [API 오류 코드](api/error-codes.md)

### error detail helper values

- 용어: error detail helper values
- 한국어 용어: 오류 세부사항 보조 값
- 의미: 기계 판독용 오류 세부사항 안에 들어가는 보조 값입니다.
- 주 담당 문서: [API 오류 세부사항](api/error-details.md)

### dry-run

- 용어: dry-run
- 한국어 용어: dry-run 미리보기
- 의미: `dry_run`을 사용하는 API 호출의 미리보기 모드입니다.
- 주 담당 문서: [API 코어 스키마](api/schema-core.md)
- 함께 볼 문서: [API 오류 처리 경로](api/error-routing.md), [저장 효과](storage-effects.md)

### dry-run preview routing

- 용어: dry-run preview routing
- 한국어 용어: dry-run 미리보기 처리 경로
- 의미: `dry_run` 미리보기의 응답 분기를 처리하는 경로입니다.
- 주 담당 문서: [API 오류 처리 경로](api/error-routing.md)
- 함께 볼 문서: [API 코어 스키마](api/schema-core.md)

### blocked result

- 용어: blocked result
- 한국어 용어: 차단 결과
- 의미: 거부 응답이 아니라 차단을 보고하는 결과 분기입니다.
- 주 담당 문서: [API 오류 처리 경로](api/error-routing.md)
- 함께 볼 문서: [Task 닫기 메서드](api/method-close-task.md), [쓰기 준비 메서드](api/method-prepare-write.md)

### rejected response

- 용어: rejected response
- 한국어 용어: 거부 응답
- 의미: 작업이 진행되기 전에 요청이 거부되었음을 나타내는 API 응답입니다.
- 주 담당 문서: [API 오류 처리 경로](api/error-routing.md)
- 함께 볼 문서: [API 코어 스키마](api/schema-core.md), [API 오류 코드](api/error-codes.md)

### migration

- 용어: migration
- 한국어 용어: 마이그레이션
- 의미: 스키마, 저장소, 데이터, 문서에 적용되는 기술적 마이그레이션입니다.
- 주 담당 문서: [저장소 버전 관리](storage-versioning.md)
- 함께 볼 문서: [저장소 개요](storage.md)

### lifecycle

- 용어: lifecycle
- 한국어 용어: 생명주기
- 의미: 엔터티나 아티팩트가 시간에 따라 거치는 단계입니다.
- 주 담당 문서: [Core 모델](core-model.md)
- 함께 볼 문서: [API 값 집합](api/schema-value-sets.md)
