# 용어집

이 문서는 하네스 문서의 공식 용어를 담당합니다. 제품 용어의 의미, 한국어 표현, 용어 카드 단위의 담당 경로를 정의합니다.

정확한 스키마, 값 집합, DDL, 저장 효과, 보안 메커니즘, API 동작, 런타임 동작, 기준 범위 구현 읽기 경로는 이 문서가 정의하지 않습니다.

## 이 용어집을 사용하는 방법

요약 표는 빠른 경로 확인용으로 사용합니다. 각 용어의 실제 통제 내용은 용어 카드에서 관리합니다. 용어 카드는 짧게 유지합니다. 용어의 뜻, 한국어 표현, 유형, 담당 문서, 관련 참조만 분명히 드러내면 됩니다.

각 용어 카드는 아래 담당 필드를 사용합니다.

- `Primary owner`는 해당 용어의 정의나 계약을 담당하는 기준 문서입니다.
- `Related references`는 용어 해석을 돕는 주변 문서이며, 그 용어를 담당하지 않습니다.

용어 하나에는 `Primary owner` 하나를 두는 것을 원칙으로 합니다. 다른 기준 담당 문서가 필요해 보이면 한 용어에 여러 담당 문서를 붙이지 말고 더 정확한 용어로 나눕니다.

이 용어집은 [docs/terminology-map.yaml](../../terminology-map.yaml)과 함께 사용합니다. 용어 지도는 한영 용어 통제, 식별자 보존 통제, 피해야 할 한국어 혼합 표현을 기계 판독 가능한 형태로 관리합니다.

카드가 스키마, API, 저장소, 보안, 상태 보기, 런타임 계약을 가리킬 때는 계약 세부사항을 용어집에 복사하지 말고 `Primary owner`를 따릅니다.

## 요약 표

| Term | Korean term | Primary owner |
|---|---|---|
| Harness | 하네스 | [기준 범위](scope.md) |
| Product Repository | Product Repository | [런타임 경계](runtime-boundaries.md) |
| Harness Runtime Home | Harness Runtime Home | [런타임 경계](runtime-boundaries.md) |
| documentation | 문서 | [작성 가이드](../maintain/authoring-guide.md) |
| baseline scope | 기준 범위 | [기준 범위](scope.md) |
| supported scope | 지원 범위 | [기준 범위](scope.md) |
| supported behavior | 지원 동작 | [기준 범위](scope.md) |
| supported API method | 지원되는 API 메서드 | [API 메서드](api/methods.md) |
| supported API value | 지원되는 API 값 | [API 값 집합](api/schema-value-sets.md) |
| out-of-scope capability | 지원 범위 밖 기능 | [기준 범위](scope.md) |
| evidence collection workflow | 증거 수집 흐름 | [기준 범위](scope.md) |
| expanded or additional evidence collection workflows | 확장 또는 추가 증거 수집 흐름 | [기준 범위](scope.md) |
| owner document | 담당 문서 | [작성 가이드](../maintain/authoring-guide.md) |
| owner contract | 담당 계약 | [작성 가이드](../maintain/authoring-guide.md) |
| applicable owner path | 적용되는 담당 경로 | [작성 가이드](../maintain/authoring-guide.md) |
| applicable reference | 적용되는 참조 문서 | [참조 색인](README.md) |
| existing owner | 기존 담당 문서 | [작성 가이드](../maintain/authoring-guide.md) |
| promotion-time owner update | 승격 시점의 담당 문서 갱신 | [기준 범위](scope.md) |
| owner placeholder | 담당 문서 자리표시자 | [작성 가이드](../maintain/authoring-guide.md) |
| `Task` | `Task` | [Core 모델](core-model.md) |
| scope | 범위 | [Core 모델](core-model.md) |
| active scope | 현재 적용 범위 | [Core 모델](core-model.md) |
| active Change Unit | 현재 적용 Change Unit | [Core 모델](core-model.md) |
| user-owned judgment | 사용자 소유 판단 | [Core 모델](core-model.md) |
| `UserJudgment` | `UserJudgment` | [API 판단 스키마](api/schema-judgment.md) |
| close readiness | 닫기 준비 상태 | [Core 모델](core-model.md) |
| close readiness evaluation | 닫기 준비 상태 평가 | [Task 닫기 메서드](api/method-close-task.md) |
| close task | Task 닫기 | [Task 닫기 메서드](api/method-close-task.md) |
| close task behavior | Task 닫기 동작 | [Task 닫기 메서드](api/method-close-task.md) |
| `harness.close_task` | `harness.close_task` | [Task 닫기 메서드](api/method-close-task.md) |
| close-readiness blocker | 닫기 차단 사유 | [API 차단 사유 처리 경로](api/blocker-routing.md) |
| `CloseReadinessBlocker` | `CloseReadinessBlocker` | [API 상태 스키마](api/schema-state.md) |
| blocker category | 차단 사유 범주 | [API 값 집합](api/schema-value-sets.md) |
| complete intent | `complete` | [API 값 집합](api/schema-value-sets.md) |
| full evaluation order | 전체 평가 순서 | [번역 가이드](../maintain/translation-guide.md) |
| artifact | 아티팩트 | [API 아티팩트 스키마](api/schema-artifacts.md) |
| evidence | 증거 | [Core 모델](core-model.md) |
| `ArtifactRef` | `ArtifactRef` | [API 아티팩트 스키마](api/schema-artifacts.md) |
| `ArtifactInput` | `ArtifactInput` | [API 아티팩트 스키마](api/schema-artifacts.md) |
| `StagedArtifactHandle` | `StagedArtifactHandle` | [API 아티팩트 스키마](api/schema-artifacts.md) |
| projection | 상태 보기 | [상태 보기 권한 참조](projection-and-templates.md) |
| `Projection` | `Projection` | [상태 보기 권한 참조](projection-and-templates.md) |
| surface | 접점 | [에이전트 통합](agent-integration.md) |
| `surface_id` | `surface_id` | [에이전트 통합](agent-integration.md) |
| active surface context | 현재 적용 접점 맥락 | [에이전트 통합](agent-integration.md) |
| `state_version` | `state_version` | [저장소 버전 관리](storage-versioning.md) |
| runtime | 런타임 | [런타임 경계](runtime-boundaries.md) |
| `Write Authorization` | 쓰기 권한 부여 | [Core 모델](core-model.md) |
| sensitive approval | 민감 동작 승인 | [Core 모델](core-model.md) |
| access class | 접근 등급 | [API 값 집합](api/schema-value-sets.md) |
| baseline guarantee | 기준 범위 보장 | [보안](security.md) |
| cooperative guarantee | 협력형 보장 | [보안](security.md) |
| detective guarantee | 탐지형 보장 | [보안](security.md) |
| design-quality owner boundary | 설계 품질 담당 경계 | [설계 품질](design-quality.md) |
| reserved value | 예약된 값 | [기준 범위](scope.md) |
| profile-gated value | 프로필 조건부 값 | [기준 범위](scope.md) |
| `ErrorCode` | `ErrorCode` | [API 오류 코드](api/error-codes.md) |
| error code meanings | 공개 오류 코드 의미 | [API 오류 코드](api/error-codes.md) |
| error precedence | 오류 우선순위 | [API 오류 우선순위](api/error-precedence.md) |
| error routing | 오류 처리 경로 | [API 오류 처리 경로](api/error-routing.md) |
| blocker routing | 차단 사유 처리 경로 | [API 차단 사유 처리 경로](api/blocker-routing.md) |
| error/blocker boundary | 오류와 차단 사유의 경계 | [API 차단 사유 처리 경로](api/blocker-routing.md) |
| public error as blocker | 공개 오류 코드가 차단 사유로 표현되는 경우 | [API 차단 사유 처리 경로](api/blocker-routing.md) |
| `ToolError.details` | `ToolError.details` | [API 오류 세부사항](api/error-details.md) |
| detail helper values | 오류 세부사항 보조 값 | [API 오류 세부사항](api/error-details.md) |
| dry-run preview routing | dry-run 미리보기 처리 경로 | [API 오류 처리 경로](api/error-routing.md) |
| blocked result | 차단 결과 | [API 오류 처리 경로](api/error-routing.md) |
| rejected response | 거부 응답 | [API 오류 처리 경로](api/error-routing.md) |
| migration | 마이그레이션 | [저장소 버전 관리](storage-versioning.md) |
| lifecycle | 생명주기 | [Core 모델](core-model.md) |

## 용어

### Harness

Term:
- Harness

Korean term:
- 하네스

Type:
- 제품 개념

Meaning:
- 하네스는 AI 지원 제품 작업을 위한 로컬 작업 권한 서버입니다.

Primary owner:
- [기준 범위](scope.md)

Related references:
- [런타임 경계](runtime-boundaries.md)

Usage note:
- 제품 이름에는 Harness를 씁니다.

### Product Repository

Term:
- Product Repository

Korean term:
- Product Repository; 사용자 문서에서는 제품 저장소를 쓸 수 있습니다.

Type:
- 제품 라벨

Meaning:
- `Product Repository`는 사용자의 프로젝트 작업 공간이며 하네스 런타임 상태와 구분됩니다.

Primary owner:
- [런타임 경계](runtime-boundaries.md)

Related references:
- 없음.

Usage note:
- 이 경계를 이름 붙일 때는 정확한 라벨을 씁니다.

### Harness Runtime Home

Term:
- Harness Runtime Home

Korean term:
- Harness Runtime Home; 사용자 문서에서는 런타임 홈을 쓸 수 있습니다.

Type:
- 제품 라벨

Meaning:
- `Harness Runtime Home`은 하네스 기록과 아티팩트를 담는 운영 데이터 공간입니다.

Primary owner:
- [런타임 경계](runtime-boundaries.md)

Related references:
- 없음.

Usage note:
- 이 경계를 이름 붙일 때는 정확한 라벨을 씁니다.

### documentation

Term:
- documentation

Korean term:
- 문서

Type:
- 문서 용어

Meaning:
- 문서는 유지되는 원천 자료이며 런타임 구현, 생성된 런타임 출력, 수락 상태가 아닙니다.

Primary owner:
- [작성 가이드](../maintain/authoring-guide.md)

Related references:
- [런타임 경계](runtime-boundaries.md)
- [구현 가이드](../build/implementation-guide.md)

Usage note:
- 문서 권한을 런타임 동작이나 제품 구현 출력과 구분합니다.

### baseline scope

Term:
- baseline scope

Korean term:
- 기준 범위

Type:
- 범위 용어

Meaning:
- 기준 범위는 하네스가 문서화한 안정적인 지원 경계입니다.

Primary owner:
- [기준 범위](scope.md)

Related references:
- [API 값 집합](api/schema-value-sets.md)

Usage note:
- 지원 상태 세부사항은 기준 범위 문서를 따릅니다.

### supported scope

Term:
- supported scope

Korean term:
- 지원 범위; 수식어가 필요할 때는 지원되는 범위를 씁니다.

Type:
- 범위 용어

Meaning:
- 지원 범위는 지원된다고 문서화된 동작이나 역량입니다.

Primary owner:
- [기준 범위](scope.md)

Related references:
- 없음.

Usage note:
- 현재 적용 범위는 active scope 용어로 구분합니다.

### supported behavior

Term:
- supported behavior

Korean term:
- 지원 동작

Type:
- 지원 경계 용어

Meaning:
- 지원 동작은 기준 범위와 영향받는 의미 담당 문서가 지원된다고 문서화한 동작입니다.

Primary owner:
- [기준 범위](scope.md)

Related references:
- [API 값 집합](api/schema-value-sets.md)

Usage note:
- 지원 여부 질문은 기준 범위와 의미 담당 문서로 보냅니다.

### supported API method

Term:
- supported API method

Korean term:
- 지원되는 API 메서드

Type:
- API 용어

Meaning:
- 지원되는 API 메서드는 지원된다고 문서화된 공개 메서드입니다.

Primary owner:
- [API 메서드](api/methods.md)

Related references:
- 없음.

Usage note:
- 공개 API 메서드를 이름 붙일 때는 정확한 메서드 식별자를 씁니다.

### supported API value

Term:
- supported API value

Korean term:
- 지원되는 API 값

Type:
- API 값 용어

Meaning:
- 지원되는 API 값은 단순 어휘가 아니라 지원된다고 문서화된 값입니다.

Primary owner:
- [API 값 집합](api/schema-value-sets.md)

Related references:
- [기준 범위](scope.md)

Usage note:
- 정확한 값 이름 질문은 API 값 집합으로 보내고, 지원 여부 질문은 기준 범위나 의미 담당 문서로 보냅니다.

### out-of-scope capability

Term:
- out-of-scope capability

Korean term:
- 지원 범위 밖 기능

Type:
- 범위 경계 용어

Meaning:
- 지원 범위 밖 기능은 지원 범위 밖에 있는 기능을 가리킵니다.

Primary owner:
- [기준 범위](scope.md)

Related references:
- 없음.

Usage note:
- 지원 상태는 기준 범위 문서를 따릅니다.

### evidence collection workflow

Term:
- evidence collection workflow

Korean term:
- 증거 수집 흐름

Type:
- 지원 범위 밖 기능 표현

Meaning:
- 증거 수집 흐름은 기능 영역을 가리키는 표현입니다.

Primary owner:
- [기준 범위](scope.md)

Related references:
- [용어 지도](../../terminology-map.yaml)

Usage note:
- 지원 상태는 기준 범위 문서에서 확인합니다.

### expanded or additional evidence collection workflows

Term:
- expanded or additional evidence collection workflows
- expanded evidence collection workflows
- additional evidence collection workflows

Korean term:
- 확장 또는 추가 증거 수집 흐름

Type:
- 지원 범위 밖 기능 묶음

Meaning:
- 이 표현은 지원 범위 밖에 있는 증거 흐름 기능 묶음을 가리킵니다.

Primary owner:
- [기준 범위](scope.md)

Related references:
- [용어 지도](../../terminology-map.yaml)

Usage note:
- 지원 상태는 기준 범위 문서를 따릅니다.

### owner document

Term:
- owner document

Korean term:
- 담당 문서

Type:
- 담당 경로 용어

Meaning:
- 담당 문서는 제품 개념, 계약, 경로, 용어 규칙의 기준 출처입니다.

Primary owner:
- [작성 가이드](../maintain/authoring-guide.md)

Related references:
- [참조 색인](README.md)

Usage note:
- 파일 경로는 문서 경로 안내일 뿐 제품 동작의 주체가 아닙니다.

### owner contract

Term:
- owner contract

Korean term:
- 담당 계약; 더 분명할 때는 담당 문서가 정의한 계약이라고 씁니다.

Type:
- 담당 경로 용어

Meaning:
- 담당 계약은 관련 담당 문서가 정의한 계약을 가리킵니다.

Primary owner:
- [작성 가이드](../maintain/authoring-guide.md)

Related references:
- [용어 지도](../../terminology-map.yaml)

Usage note:
- 담당 문서가 정의한 계약을 가리킬 때 씁니다.

### applicable owner path

Term:
- applicable owner path

Korean term:
- 적용되는 담당 경로

Type:
- 담당 경로 용어

Meaning:
- 적용되는 담당 경로는 어떤 주제에 적용되는 담당 문서 경로입니다.

Primary owner:
- [작성 가이드](../maintain/authoring-guide.md)

Related references:
- [참조 색인](README.md)
- [doc-index.yaml](../../doc-index.yaml)

Usage note:
- 문서 경로 안내에만 씁니다.

### applicable reference

Term:
- applicable reference

Korean term:
- 적용되는 참조 문서

Type:
- 참조 경로 용어

Meaning:
- 적용되는 참조 문서는 관련 계약을 정의하는 참조 문서를 가리킵니다.

Primary owner:
- [참조 색인](README.md)

Related references:
- [작성 가이드](../maintain/authoring-guide.md)
- [용어 지도](../../terminology-map.yaml)

Usage note:
- 문서 경로 안내를 위한 줄임말로만 씁니다.

### existing owner

Term:
- existing owner
- existing canonical owner
- existing owner document

Korean term:
- 기존 담당 문서

Type:
- 담당 경로 용어

Meaning:
- 기존 담당 문서는 이미 존재하며 규범 의미의 출처로 연결할 수 있는 기준 담당 문서입니다.

Primary owner:
- [작성 가이드](../maintain/authoring-guide.md)

Related references:
- [참조 색인](README.md)
- [doc-index.yaml](../../doc-index.yaml)

Usage note:
- 이미 존재하는 담당 문서에만 씁니다.

### promotion-time owner update

Term:
- promotion-time owner update

Korean term:
- 승격 시점의 담당 문서 갱신

Type:
- 범위 승격 용어

Meaning:
- 승격 시점의 담당 문서 갱신은 지원 범위 밖 기능을 지원 범위로 승격할 때 필요한 담당 문서 변경을 뜻합니다.

Primary owner:
- [기준 범위](scope.md)

Related references:
- [작성 가이드](../maintain/authoring-guide.md)

Usage note:
- 승격 계획을 말할 때 쓰며, 지원 경계는 기준 범위 문서가 담당합니다.

### owner placeholder

Term:
- owner placeholder

Korean term:
- 담당 문서 자리표시자

Type:
- 담당 공백 용어

Meaning:
- 담당 문서 자리표시자는 기능을 승격하기 전에 담당 문서를 만들거나 지정해야 할 수 있음을 나타냅니다.

Primary owner:
- [작성 가이드](../maintain/authoring-guide.md)

Related references:
- [기준 범위](scope.md)

Usage note:
- 담당 문서 공백을 표시할 때만 씁니다.

### `Task`

Term:
- `Task`

Korean term:
- `Task`; 정확한 엔티티를 가리킬 필요가 없을 때는 작업을 쓸 수 있습니다.

Type:
- Core 엔티티

Meaning:
- `Task`는 구체화, 실행, 차단, 닫기의 대상이 되는 사용자 가치 단위입니다.

Primary owner:
- [Core 모델](core-model.md)

Related references:
- [API 상태 스키마](api/schema-state.md)
- [API 값 집합](api/schema-value-sets.md)

Usage note:
- 엔티티나 필드를 이름 붙일 때는 정확한 식별자를 씁니다.

### scope

Term:
- scope

Korean term:
- 범위

Type:
- Core 권한 용어

Meaning:
- 범위는 현재 `Task`나 Change Unit이 포함하고 제외하는 합의된 경계입니다.

Primary owner:
- [Core 모델](core-model.md)

Related references:
- [범위 갱신 메서드](api/method-update-scope.md)
- [API 판단 스키마](api/schema-judgment.md)

Usage note:
- 스키마나 API 필드를 이름 붙일 때는 정확한 식별자를 씁니다.

### active scope

Term:
- active scope
- currently applied scope

Korean term:
- 현재 적용 범위

Type:
- Core 권한 용어

Meaning:
- active scope는 `Task`나 Change Unit 맥락 안에서 현재 적용되는 범위입니다.

Primary owner:
- [Core 모델](core-model.md)

Related references:
- [범위 갱신 메서드](api/method-update-scope.md)

Usage note:
- active scope는 기준 범위나 지원 범위와 구분합니다.

### active Change Unit

Term:
- active Change Unit

Korean term:
- 현재 적용 Change Unit

Type:
- Core 권한 용어

Meaning:
- active Change Unit은 권한 모델에서 현재 적용되는 Change Unit입니다.

Primary owner:
- [Core 모델](core-model.md)

Related references:
- [범위 갱신 메서드](api/method-update-scope.md)

Usage note:
- 한국어 산문에서도 Change Unit은 제품 용어로 둡니다.

### user-owned judgment

Term:
- user-owned judgment

Korean term:
- 사용자 소유 판단; 사용자 문서에서는 사용자 판단을 쓸 수 있습니다.

Type:
- Core 권한 용어

Meaning:
- 사용자 소유 판단은 사용자에게 남겨진 결정입니다.

Primary owner:
- [Core 모델](core-model.md)

Related references:
- [API 판단 스키마](api/schema-judgment.md)

Usage note:
- 수락과 권한 부여 계열 용어와 구분합니다.

### `UserJudgment`

Term:
- `UserJudgment`

Korean term:
- `UserJudgment`; 스키마를 말하지 않을 때는 사용자 소유 판단을 씁니다.

Type:
- API 스키마

Meaning:
- `UserJudgment`는 대기 중이거나 해결된 사용자 소유 판단을 나타내는 API 스키마 식별자입니다.

Primary owner:
- [API 판단 스키마](api/schema-judgment.md)

Related references:
- [Core 모델](core-model.md)
- [사용자 판단 메서드](api/method-user-judgment.md)

Usage note:
- 개념은 산문 용어로, 스키마는 식별자로 이름 붙입니다.

### close readiness

Term:
- close readiness

Korean term:
- 닫기 준비 상태; 사용자 문서에서는 닫기 가능 여부를 쓸 수 있습니다.

Type:
- Core 닫기 준비 상태 개념

Meaning:
- 닫기 준비 상태는 작업을 닫을 준비가 되었는지를 나타내는 Core 개념입니다.

Primary owner:
- [Core 모델](core-model.md)

Related references:
- [Task 닫기 메서드](api/method-close-task.md)
- [API 차단 사유 처리 경로](api/blocker-routing.md)

Usage note:
- 스키마를 이름 붙일 때만 스키마 용어를 씁니다.

### close readiness evaluation

Term:
- close readiness evaluation

Korean term:
- 닫기 준비 상태 평가

Type:
- Task 닫기 메서드 용어

Meaning:
- 닫기 준비 상태 평가는 닫기 준비 상태를 다루는 Task 닫기 메서드 관심사입니다.

Primary owner:
- [Task 닫기 메서드](api/method-close-task.md)

Related references:
- [Core 모델](core-model.md)

Usage note:
- 평가 세부사항은 메서드 담당 문서를 따릅니다.

### close task

Term:
- close task

Korean term:
- Task 닫기

Type:
- API 메서드 용어

Meaning:
- Task 닫기는 Task를 닫는 API 작업을 가리킵니다.

Primary owner:
- [Task 닫기 메서드](api/method-close-task.md)

Related references:
- [API 메서드](api/methods.md)

Usage note:
- 정확한 공개 메서드를 이름 붙일 때는 `harness.close_task`를 씁니다.

### close task behavior

Term:
- close task behavior
- `harness.close_task` behavior
- close-task method behavior

Korean term:
- Task 닫기 동작

Type:
- API 메서드 용어

Meaning:
- Task 닫기 동작은 `harness.close_task` 담당 문서가 다루는 동작 영역을 가리킵니다.

Primary owner:
- [Task 닫기 메서드](api/method-close-task.md)

Related references:
- [API 메서드](api/methods.md)

Usage note:
- 세부사항은 Task 닫기 메서드 담당 문서를 따릅니다.

### `harness.close_task`

Term:
- `harness.close_task`

Korean term:
- `harness.close_task`

Type:
- API 메서드 식별자

Meaning:
- `harness.close_task`는 Task 닫기 요청에 쓰는 공개 메서드 식별자입니다.

Primary owner:
- [Task 닫기 메서드](api/method-close-task.md)

Related references:
- [API 메서드](api/methods.md)

Usage note:
- 세부사항은 Task 닫기 메서드 담당 문서를 따릅니다.

### close-readiness blocker

Term:
- close-readiness blocker
- close blocker

Korean term:
- 닫기 차단 사유

Type:
- API 차단 사유 처리 용어

Meaning:
- 닫기 차단 사유는 작업을 닫을 준비가 되지 않은 이유를 가리킵니다.

Primary owner:
- [API 차단 사유 처리 경로](api/blocker-routing.md)

Related references:
- [Core 모델](core-model.md)
- [API 상태 스키마](api/schema-state.md)

Usage note:
- 한국어 산문에서는 닫기 차단 사유를 쓰고, 스키마에는 `CloseReadinessBlocker`를 씁니다.

### `CloseReadinessBlocker`

Term:
- `CloseReadinessBlocker`

Korean term:
- `CloseReadinessBlocker`; 스키마 이름을 말하지 않을 때는 닫기 차단 사유를 씁니다.

Type:
- API 스키마

Meaning:
- `CloseReadinessBlocker`는 닫기 차단 사유 데이터를 나타내는 API 스키마 식별자입니다.

Primary owner:
- [API 상태 스키마](api/schema-state.md)

Related references:
- [API 차단 사유 처리 경로](api/blocker-routing.md)

Usage note:
- 개념은 산문 용어로, 스키마는 식별자로 이름 붙입니다.

### blocker category

Term:
- blocker category

Korean term:
- 차단 사유 범주

Type:
- API 값 개념

Meaning:
- 차단 사유 범주는 닫기 차단 사유를 묶어 설명하는 산문 개념입니다.

Primary owner:
- [API 값 집합](api/schema-value-sets.md)

Related references:
- [API 상태 스키마](api/schema-state.md)
- [API 차단 사유 처리 경로](api/blocker-routing.md)

Usage note:
- 정확한 필드를 이름 붙일 때는 `CloseReadinessBlocker.category`를 씁니다.

### complete intent

Term:
- complete intent
- 의도 값 이름으로서 `complete`

Korean term:
- `complete`

Type:
- API 값 용어

Meaning:
- complete intent는 `complete` 의도 값 뒤의 산문 개념입니다.

Primary owner:
- [API 값 집합](api/schema-value-sets.md)

Related references:
- [Task 닫기 메서드](api/method-close-task.md)
- [용어 지도](../../terminology-map.yaml)

Usage note:
- `complete`는 enum 값이나 정확한 식별자일 때만 씁니다.

### full evaluation order

Term:
- full evaluation order
- entire evaluation order

Korean term:
- 전체 평가 순서; 닫기 준비 상태 맥락에서는 전체 닫기 준비 상태 평가 순서.

Type:
- 번역 용어

Meaning:
- 전체 평가 순서는 `complete` enum 값을 뜻하지 않고 전체 평가 흐름을 가리킵니다.

Primary owner:
- [번역 가이드](../maintain/translation-guide.md)

Related references:
- [용어 지도](../../terminology-map.yaml)

Usage note:
- 일반 산문 의미에는 full이나 entire를 씁니다.

### artifact

Term:
- artifact

Korean term:
- 아티팩트

Type:
- 아티팩트 용어

Meaning:
- 아티팩트는 하네스가 참조할 수 있는 제품 작업 자료입니다.

Primary owner:
- [API 아티팩트 스키마](api/schema-artifacts.md)

Related references:
- [아티팩트 저장소](storage-artifacts.md)

Usage note:
- 아티팩트가 있다는 사실만으로 증거 충분성이 성립하지는 않습니다.

### evidence

Term:
- evidence

Korean term:
- 증거

Type:
- Core 증거 개념

Meaning:
- 증거는 기록된 범위 안에서 기록된 주장을 뒷받침합니다.

Primary owner:
- [Core 모델](core-model.md)

Related references:
- [API 상태 스키마](api/schema-state.md)
- [실행 기록 메서드](api/method-record-run.md)

Usage note:
- 수락과 잔여 위험 계열 용어와 구분합니다.

### `ArtifactRef`

Term:
- `ArtifactRef`

Korean term:
- `ArtifactRef`; 스키마를 말하지 않을 때는 아티팩트 참조를 쓸 수 있습니다.

Type:
- API 스키마

Meaning:
- `ArtifactRef`는 아티팩트 참조를 나타내는 API 스키마 식별자입니다.

Primary owner:
- [API 아티팩트 스키마](api/schema-artifacts.md)

Related references:
- [아티팩트 저장소](storage-artifacts.md)

Usage note:
- 참조 세부사항은 아티팩트 스키마 담당 문서를 따릅니다.

### `ArtifactInput`

Term:
- `ArtifactInput`

Korean term:
- `ArtifactInput`; 스키마를 말하지 않을 때는 제공할 아티팩트를 쓸 수 있습니다.

Type:
- API 스키마

Meaning:
- `ArtifactInput`은 아티팩트를 다루는 메서드에 제공되는 아티팩트 데이터의 스키마 식별자입니다.

Primary owner:
- [API 아티팩트 스키마](api/schema-artifacts.md)

Related references:
- 없음.

Usage note:
- 입력 세부사항은 아티팩트 스키마 담당 문서를 따릅니다.

### `StagedArtifactHandle`

Term:
- `StagedArtifactHandle`

Korean term:
- `StagedArtifactHandle`; 사용자 문서에서는 스테이징된 아티팩트 핸들을 쓸 수 있습니다.

Type:
- API 스키마

Meaning:
- `StagedArtifactHandle`은 임시로 스테이징된 아티팩트 핸들의 스키마 식별자입니다.

Primary owner:
- [API 아티팩트 스키마](api/schema-artifacts.md)

Related references:
- [아티팩트 저장소](storage-artifacts.md)

Usage note:
- 핸들 세부사항은 아티팩트 스키마 담당 문서를 따릅니다.

### projection

Term:
- projection

Korean term:
- 상태 보기

Type:
- 상태 보기 용어

Meaning:
- 상태 보기는 읽기 전용 상태 또는 표시 보기입니다.

Primary owner:
- [상태 보기 권한 참조](projection-and-templates.md)

Related references:
- [템플릿 본문](template-bodies.md)

Usage note:
- 권한 질문은 상태 보기 담당 문서를 따릅니다.

### `Projection`

Term:
- `Projection`

Korean term:
- `Projection`; 참조 산문에서 처음 설명할 때는 `Projection`(읽기 전용 상태 보기)을 쓸 수 있습니다.

Type:
- 제품 라벨

Meaning:
- `Projection`은 읽기 전용 파생 상태 보기나 표시 보기를 가리키는 정확한 Harness 라벨입니다.

Primary owner:
- [상태 보기 권한 참조](projection-and-templates.md)

Related references:
- [템플릿 본문](template-bodies.md)
- [API 상태 스키마](api/schema-state.md)

Usage note:
- 라벨 세부사항은 상태 보기 담당 문서를 따릅니다.

### surface

Term:
- surface

Korean term:
- 접점

Type:
- 통합 용어

Meaning:
- 접점은 하네스가 쓰이거나 관찰되는 사용자, 에이전트, 도구, 커넥터, 로컬 맥락입니다.

Primary owner:
- [에이전트 통합](agent-integration.md)

Related references:
- [보안](security.md)

Usage note:
- `surface_id`는 권한 증거가 아닙니다.

### `surface_id`

Term:
- `surface_id`

Korean term:
- `surface_id`

Type:
- 접점 식별자

Meaning:
- `surface_id`는 등록된 로컬 접점을 고르는 요청 수준 선택자입니다.

Primary owner:
- [에이전트 통합](agent-integration.md)

Related references:
- [API 코어 스키마](api/schema-core.md)
- [보안](security.md)

Usage note:
- 접점 식별 세부사항은 에이전트 통합 문서를 따릅니다.

### active surface context

Term:
- active surface context

Korean term:
- 현재 적용 접점 맥락

Type:
- 통합 용어

Meaning:
- active surface context는 요청이나 상호작용에 현재 적용되는 접점 맥락입니다.

Primary owner:
- [에이전트 통합](agent-integration.md)

Related references:
- [보안](security.md)

Usage note:
- 접점 맥락 세부사항은 에이전트 통합 문서를 따릅니다.

### `state_version`

Term:
- `state_version`
- `project_state.state_version`

Korean term:
- `state_version`

Type:
- 저장소 버전 관리 식별자

Meaning:
- `state_version`은 프로젝트 전체 상태 버전 식별자를 가리킵니다.

Primary owner:
- [저장소 버전 관리](storage-versioning.md)

Related references:
- [API 코어 스키마](api/schema-core.md)

Usage note:
- 상태 시계 세부사항은 저장소 버전 관리 문서를 따릅니다.

### runtime

Term:
- runtime

Korean term:
- 런타임

Type:
- 런타임 용어

Meaning:
- 런타임은 실행되는 하네스 서버/런타임 동작과 런타임 데이터 공간을 뜻합니다.

Primary owner:
- [런타임 경계](runtime-boundaries.md)

Related references:
- [보안](security.md)

Usage note:
- Markdown 원천 문서는 런타임 상태나 생성된 런타임 출력이 아닙니다.

### `Write Authorization`

Term:
- `Write Authorization`

Korean term:
- 쓰기 권한 부여

Type:
- Core 권한 부여 용어

Meaning:
- `Write Authorization`은 호환되는 제품 파일 쓰기 시도 하나를 위한 Core 권한 부여입니다.

Primary owner:
- [Core 모델](core-model.md)

Related references:
- [보안](security.md)
- [쓰기 준비 메서드](api/method-prepare-write.md)

Usage note:
- 명령 승인, 민감 동작 승인과 구분합니다.

### sensitive approval

Term:
- sensitive approval
- sensitive-action approval

Korean term:
- 민감 동작 승인

Type:
- 승인 용어

Meaning:
- 민감 동작 승인은 민감 동작 경계에 대한 사용자 허락입니다.

Primary owner:
- [Core 모델](core-model.md)

Related references:
- [API 판단 스키마](api/schema-judgment.md)
- [보안](security.md)

Usage note:
- 영어 산문에서는 sensitive-action approval을 기본 표현으로 씁니다.

### access class

Term:
- access class

Korean term:
- 접근 등급

Type:
- 접근 용어

Meaning:
- 접근 등급은 보호된 접근 기대를 설명할 때 쓰는 분류입니다.

Primary owner:
- [API 값 집합](api/schema-value-sets.md)

Related references:
- [에이전트 통합](agent-integration.md)
- [보안](security.md)

Usage note:
- 값 이름은 API 값 집합, 보장 표현은 보안 문서를 따릅니다.

### baseline guarantee

Term:
- baseline guarantee

Korean term:
- 기준 범위 보장

Type:
- 보안 용어

Meaning:
- 기준 범위 보장은 보안 문서가 정의하는 기준 범위 보장을 가리킵니다.

Primary owner:
- [보안](security.md)

Related references:
- [기준 범위](scope.md)

Usage note:
- 보장 수준은 보안 문서를 따릅니다.

### cooperative guarantee

Term:
- cooperative guarantee

Korean term:
- 협력형 보장

Type:
- 보안 용어

Meaning:
- 협력형 보장은 보안 문서가 정의하는 보장 유형입니다.

Primary owner:
- [보안](security.md)

Related references:
- 없음.

Usage note:
- 보장 강도는 보안 문서를 따릅니다.

### detective guarantee

Term:
- detective guarantee

Korean term:
- 탐지형 보장

Type:
- 보안 용어

Meaning:
- 탐지형 보장은 보안 문서가 정의하는 보장 유형입니다.

Primary owner:
- [보안](security.md)

Related references:
- 없음.

Usage note:
- 보장 강도는 보안 문서를 따릅니다.

### design-quality owner boundary

Term:
- design-quality owner boundary
- design-quality routing boundary
- design-quality boundary

Korean term:
- 설계 품질 담당 경계

Type:
- 설계 품질 용어

Meaning:
- 설계 품질 담당 경계는 설계 품질 관찰 사항을 관련 담당 문서나 담당 계약으로 보내는 경계입니다.

Primary owner:
- [설계 품질](design-quality.md)

Related references:
- 없음.

Usage note:
- 설계 품질 문구는 독립적인 QA, 수락, 잔여 위험, 증거, 닫기 권한이 아닙니다.

### reserved value

Term:
- reserved value

Korean term:
- 예약된 값

Type:
- 값 상태 용어

Meaning:
- 예약된 값은 그 자체만으로 지원 동작이 되지 않는 어휘입니다.

Primary owner:
- [기준 범위](scope.md)

Related references:
- [API 값 집합](api/schema-value-sets.md)

Usage note:
- 값 집합에 있다는 사실만으로 동작이 지원되지는 않습니다.

### profile-gated value

Term:
- profile-gated value

Korean term:
- 프로필 조건부 값

Type:
- 값 상태 용어

Meaning:
- 프로필 조건부 값은 명시된 프로필에 따라 지원 상태가 정해지는 어휘입니다.

Primary owner:
- [기준 범위](scope.md)

Related references:
- [API 값 집합](api/schema-value-sets.md)

Usage note:
- 지원 상태는 기준 범위 문서를 따릅니다.

### `ErrorCode`

Term:
- `ErrorCode`

Korean term:
- `ErrorCode`; 정확한 식별자를 말하지 않을 때는 공개 오류 코드를 쓸 수 있습니다.

Type:
- API 오류 식별자

Meaning:
- `ErrorCode`는 공개 API 오류 코드의 식별자 공간입니다.

Primary owner:
- [API 오류 코드](api/error-codes.md)

Related references:
- [API 오류 우선순위](api/error-precedence.md)
- [API 오류 처리 경로](api/error-routing.md)

Usage note:
- 공개 코드 의미는 API 오류 코드 문서를 따릅니다.

### error code meanings

Term:
- error code meanings
- public error code meanings

Korean term:
- 공개 오류 코드 의미

Type:
- API 오류 코드 용어

Meaning:
- 공개 오류 코드 의미는 `ErrorCode` 값의 공개 의미 관심사를 가리킵니다.

Primary owner:
- [API 오류 코드](api/error-codes.md)

Related references:
- [API 오류 우선순위](api/error-precedence.md)

Usage note:
- 이웃 관심사는 각 API 오류 담당 문서를 따릅니다.

### error precedence

Term:
- error precedence
- primary public-error selection

Korean term:
- 오류 우선순위

Type:
- API 오류 우선순위 용어

Meaning:
- 오류 우선순위는 대표 공개 오류 선택 관심사를 가리킵니다.

Primary owner:
- [API 오류 우선순위](api/error-precedence.md)

Related references:
- [API 오류 코드](api/error-codes.md)

Usage note:
- 이웃 관심사는 각 API 오류 담당 문서를 따릅니다.

### error routing

Term:
- error routing
- API response branch routing
- 담당 문서 제목을 말할 때 API error routing

Korean term:
- 오류 처리 경로

Type:
- API 오류 처리 용어

Meaning:
- 오류 처리 경로는 API 응답 분기 처리 관심사를 가리킵니다.

Primary owner:
- [API 오류 처리 경로](api/error-routing.md)

Related references:
- [API 오류 코드](api/error-codes.md)
- [API 차단 사유 처리 경로](api/blocker-routing.md)

Usage note:
- 이웃 관심사는 각 API 오류 담당 문서를 따릅니다.

### blocker routing

Term:
- blocker routing
- close-readiness blocker routing
- 담당 문서 제목을 말할 때 API blocker routing

Korean term:
- 차단 사유 처리 경로

Type:
- API 차단 사유 처리 용어

Meaning:
- 차단 사유 처리 경로는 닫기 차단 사유와 API 응답 분기 사이의 경계를 다룹니다.

Primary owner:
- [API 차단 사유 처리 경로](api/blocker-routing.md)

Related references:
- [API 오류 처리 경로](api/error-routing.md)
- [Core 모델](core-model.md)
- [Task 닫기 메서드](api/method-close-task.md)

Usage note:
- 이 경계는 [API 차단 사유 처리 경로](api/blocker-routing.md)에서 확인합니다.

### error/blocker boundary

Term:
- error/blocker boundary
- API error versus close-readiness blocker boundary

Korean term:
- 오류와 차단 사유의 경계

Type:
- API 차단 사유 처리 용어

Meaning:
- 오류와 차단 사유의 경계는 API 오류와 닫기 차단 사유 사이의 구분을 가리킵니다.

Primary owner:
- [API 차단 사유 처리 경로](api/blocker-routing.md)

Related references:
- [API 오류 코드](api/error-codes.md)

Usage note:
- 이 경계는 [API 차단 사유 처리 경로](api/blocker-routing.md)에서 확인합니다.

### public error as blocker

Term:
- public error as blocker
- public `ErrorCode` as blocker

Korean term:
- 공개 오류 코드가 차단 사유로 표현되는 경우

Type:
- API 차단 사유 처리 용어

Meaning:
- 공개 오류 코드가 차단 사유로 표현되는 경우는 공개 오류 코드가 관련된 차단 사유 처리 관심사를 가리킵니다.

Primary owner:
- [API 차단 사유 처리 경로](api/blocker-routing.md)

Related references:
- [API 오류 코드](api/error-codes.md)

Usage note:
- 이 좁은 경우는 [API 차단 사유 처리 경로](api/blocker-routing.md)에서 확인합니다.

### `ToolError.details`

Term:
- `ToolError.details`

Korean term:
- `ToolError.details`; 정확한 API 식별자를 말하지 않을 때는 오류 세부사항을 쓸 수 있습니다.

Type:
- API 세부 식별자

Meaning:
- `ToolError.details`는 기계 판독용 오류 세부사항을 나타내는 정확한 API 세부 식별자입니다.

Primary owner:
- [API 오류 세부사항](api/error-details.md)

Related references:
- [API 오류 코드](api/error-codes.md)

Usage note:
- 중첩 세부 값은 API 오류 세부사항 문서를 따릅니다.

### detail helper values

Term:
- detail helper values
- error detail helper values

Korean term:
- 오류 세부사항 보조 값

Type:
- API 세부사항 용어

Meaning:
- 세부사항 보조 값은 `ToolError.details` 아래의 중첩 값을 가리킵니다.

Primary owner:
- [API 오류 세부사항](api/error-details.md)

Related references:
- 없음.

Usage note:
- 중첩 세부 값은 API 오류 세부사항 문서를 따릅니다.

### dry-run preview routing

Term:
- dry-run preview routing
- `dry_run` preview response branch routing

Korean term:
- dry-run 미리보기 처리 경로

Type:
- API 미리보기 처리 용어

Meaning:
- dry-run 미리보기 처리 경로는 `dry_run` 미리보기의 응답 분기 처리 관심사를 가리킵니다.

Primary owner:
- [API 오류 처리 경로](api/error-routing.md)

Related references:
- [API 메서드](api/methods.md)

Usage note:
- 이웃 세부사항은 메서드와 스키마 담당 문서를 따릅니다.

### blocked result

Term:
- blocked result

Korean term:
- 차단 결과

Type:
- API 결과 용어

Meaning:
- 차단 결과는 유효하지만 차단된 동작을 위한 API 결과 분기를 가리킵니다.

Primary owner:
- [API 오류 처리 경로](api/error-routing.md)

Related references:
- [API 메서드](api/methods.md)

Usage note:
- 결과 분기 세부사항은 API 오류 처리 경로 문서를 따릅니다.

### rejected response

Term:
- rejected response

Korean term:
- 거부 응답

Type:
- API 응답 분기

Meaning:
- 거부 응답은 거부된 요청을 위한 API 응답 분기를 가리킵니다.

Primary owner:
- [API 오류 처리 경로](api/error-routing.md)

Related references:
- [API 코어 스키마](api/schema-core.md)
- [API 오류 코드](api/error-codes.md)

Usage note:
- 분기 세부사항은 API 오류 처리 경로 문서를 따릅니다.

### migration

Term:
- migration

Korean term:
- 마이그레이션

Type:
- 저장소 용어

Meaning:
- 마이그레이션은 스키마, 저장소, 데이터, 문서 구조를 옮기거나 갱신하는 기술 개념입니다.

Primary owner:
- [저장소 버전 관리](storage-versioning.md)

Related references:
- [저장소 개요](storage.md)

Usage note:
- 한국어 기술 문맥에서는 마이그레이션을 씁니다.

### lifecycle

Term:
- lifecycle

Korean term:
- 생명주기

Type:
- 생명주기 용어

Meaning:
- 생명주기는 `Task`나 아티팩트 핸들 같은 개념의 단계 진행입니다.

Primary owner:
- [Core 모델](core-model.md)

Related references:
- [API 값 집합](api/schema-value-sets.md)

Usage note:
- 생명주기 필드나 값은 정확한 식별자로 이름 붙입니다.
