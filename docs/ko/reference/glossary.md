# 용어집

이 용어집은 하네스의 핵심 용어를 사람이 읽기 쉽게 안내하는 간결한 문서입니다. 주요 개념의 뜻과 각 용어의 주 담당 문서를 확인할 때 사용합니다.

주제별 담당 문서 찾기는 [참조 색인](README.md)을 사용합니다. `doc_id` 기준의 정확한 기계 판독 경로는 [`docs/doc-index.yaml`](../../doc-index.yaml)을 사용합니다.

전체 구조화 용어 목록, 식별자 보존 규칙, 한국어 혼합어 규칙은 [docs/terminology-map.yaml](../../terminology-map.yaml)에 있습니다. 정확한 API 동작, 스키마, 저장 효과, 보안 보장, 닫기 준비 상태 동작, 오류 처리 경로는 집중 담당 문서를 따릅니다.

## 용어

| 용어 | 한국어 표현 | 짧은 뜻 | 주 담당 문서 |
|---|---|---|---|
| Harness | 하네스 | AI 지원 제품 작업을 위한 로컬 작업 권한 서버입니다. | [기준 범위](scope.md) |
| Core | Core | 하네스 상태와 권한 전이를 다루는 로컬 기준 기록입니다. | [Core 모델](core-model.md) |
| `Product Repository` | 제품 저장소 | 하네스 런타임 상태와 구분되는 사용자의 프로젝트 작업 공간입니다. | [런타임 경계](runtime-boundaries.md) |
| `Harness Runtime Home` | 런타임 홈 | 하네스 기록과 아티팩트를 담는 운영 데이터 공간입니다. | [런타임 경계](runtime-boundaries.md) |
| runtime | 런타임 | 하네스의 운영 실행과 데이터 맥락입니다. | [런타임 경계](runtime-boundaries.md) |
| baseline scope | 기준 범위 | 하네스가 안정적으로 지원한다고 문서화한 경계입니다. | [기준 범위](scope.md) |
| out-of-scope capability | 지원 범위 밖 기능 | 기준 지원 경계 밖에 있는 유예된 기능입니다. | [기준 범위](scope.md) |
| owner document | 담당 문서 | 용어, 제품 개념, 계약을 정의하는 기준 문서입니다. | [작성 가이드](../maintain/authoring-guide.md) |
| applicable owner path | 적용되는 담당 경로 | 질문이나 개념에 맞는 집중 담당 문서로 가는 문서 경로입니다. | [작성 가이드](../maintain/authoring-guide.md) |
| `Task` | `Task` | 범위, 권한 맥락, 판단, 증거, 닫기 준비 상태를 묶는 하네스 개체입니다. | [Core 모델](core-model.md) |
| Change Unit | Change Unit | `Task` 안에서 쓰기 가능한 작업의 현재 적용 범위 경계입니다. | [Core 모델](core-model.md) |
| scope | 범위 | `Task` 또는 Change Unit 맥락에 붙는 작업 또는 권한 경계입니다. | [Core 모델](core-model.md) |
| Autonomy Boundary | Autonomy Boundary | 현재 적용 Change Unit 안에서 에이전트가 가질 수 있는 재량 범위입니다. | [Core 모델](core-model.md) |
| user-owned judgment | 사용자 소유 판단 | 하네스가 기록하지만 Core 소유 사실로 바꾸지 않는 사용자 결정이나 평가입니다. | [Core 모델](core-model.md) |
| Run | 실행 기록 | 사용 가능한 맥락과 참조를 함께 남기는 실행이나 관찰 기록입니다. | [Core 모델](core-model.md) |
| evidence | 증거 | 특정 범위에서 특정 주장을 뒷받침하는 기록입니다. | [Core 모델](core-model.md) |
| artifact | 아티팩트 | 하네스 아티팩트 개념으로 참조되거나 스테이징되는 작업 자료입니다. | [API 아티팩트 스키마](api/schema-artifacts.md) |
| `Write Authorization` | 쓰기 권한 부여 | 호환되는 제품 파일 쓰기 시도 하나를 위한 하네스 권한 부여 개념입니다. | [Core 모델](core-model.md) |
| sensitive-action approval | 민감 동작 승인 | `Write Authorization`과 구분되는 이름 붙은 민감 단계에 대한 사용자 승인입니다. | [Core 모델](core-model.md) |
| close readiness | 닫기 준비 상태 | 현재 상태에서 `Task`를 닫을 준비가 되었는지를 나타내는 Core 개념입니다. | [Core 모델](core-model.md) |
| close-readiness blocker | 닫기 차단 사유 | 닫기 준비 상태가 진행되지 못하는 사유입니다. | [API 차단 사유 처리 경로](api/blocker-routing.md) |
| final acceptance | 최종 수락 | 요청한 닫기에 대해 보이는 닫기 근거가 받아들일 만한지 사용자가 판단하는 것입니다. | [Core 모델](core-model.md) |
| residual risk | 잔여 위험 | 닫기에 의미가 있는 알려진 남은 불확실성, 한계, 절충점입니다. | [Core 모델](core-model.md) |
| `Projection` | 상태 보기 | 읽기 전용 상태 보기 개념이자 정확한 제품 라벨입니다. | [상태 보기 권한 참조](projection-and-templates.md) |
| surface | 접점 | 맥락이 드러나는 통합 또는 상호작용 접점입니다. | [에이전트 통합](agent-integration.md) |
| access class | 접근 등급 | 검증된 접점과 접근 맥락을 분류하는 값 범주입니다. | [API 값 집합](api/schema-value-sets.md) |
| baseline guarantee | 기준 범위 보장 | 기준 범위에서 지원되는 보장을 말할 때 쓰는 보안 표현입니다. | [보안](security.md) |
| `ErrorCode` | `ErrorCode` | 공개 API 오류 코드 식별자입니다. | [API 오류 코드](api/error-codes.md) |
