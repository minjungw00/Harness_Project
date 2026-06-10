# 현재 MVP API

## 이 문서로 할 수 있는 일

이 참조 문서는 현재 MVP API 표면을 찾아볼 때 사용합니다. [API 값 집합](schema-value-sets.md)이 담당하는 활성 메서드 이름 집합에 대해, 메서드 수준 요청, 응답, 상태 효과, 저장소 담당 문서, 오류, 보안 경계 요약을 이 문서가 담당합니다.

이 문서는 계획과 검토를 위한 향후 하네스 서버 동작을 설명합니다. 이 저장소에는 현재 하네스 런타임이나 서버 구현이 없습니다. 향후 API 또는 스키마 후보는 이 활성 참조가 아니라 [이후 후보 색인](../../later/index.md)에 둡니다. 저장소 DDL과 전체 공통 스키마 본문은 이 메서드 참조 밖의 담당 문서가 소유합니다.

## 핵심 생각

현재 MVP API는 한 사용자 작업 루프를 위한 작은 로컬 MCP 표면입니다. 작업을 접수하고, 상태를 보여 주고, 활성 범위를 갱신하고, 제안된 제품 쓰기를 현재 Core 상태와 비교하며, Run과 증거 참조를 기록하고, 사용자 소유 판단을 묻고 기록하며, 활성 차단 사유가 허용할 때만 닫을 수 있습니다.

이 API는 OS 권한, 임의 도구 샌드박스, 변조 방지 파일, 도구 실행 전 차단, 보안 격리를 제공하지 않습니다. `harness.prepare_write`는 협력형 하네스 기록/확인만 반환합니다.

요구사항 구체화는 활성 Task, Change Unit, `user_judgment`, 증거 요약, 차단 사유 경로, 다음 행동, 파생된 `ShapingReadiness` 보기를 사용합니다. API는 모호한 요청에서 안전한 첫 Change Unit으로 이동하기 위해 별도의 활성 Discovery Brief, Question Queue, Assumption Register 또는 비슷한 커밋된 계획 아티팩트를 도입하면 안 됩니다.

<a id="active-mvp-method-behavior"></a>

## 현재 MVP 메서드 동작

정확한 활성 메서드 이름 값 집합은 [API 값 집합](schema-value-sets.md)이 담당합니다. 이 페이지는 현재 메서드의 동작을 담당합니다.

| 메서드 | 활성 역할 |
|---|---|
| [`harness.intake`](#harnessintake) | 평소 사용자 작업을 시작, 재개, 분류합니다. |
| [`harness.status`](#harnessstatus) | 현재 상태 요약, 차단 사유, 대기 중인 판단, 증거 요약, 닫기 상태, 다음 안전한 행동을 반환합니다. |
| [`harness.update_scope`](#harnessupdate_scope) | `harness.intake` 이후 활성 Task 범위와 활성 Change Unit을 갱신합니다. |
| [`harness.prepare_write`](#harnessprepare_write) | 제안된 제품 파일 쓰기를 현재 범위, 상태, 필요한 별도 민감 동작 승인, baseline, 접점 역량과 비교합니다. |
| [`harness.stage_artifact`](#harnessstage_artifact) | 호출자가 제공한 안전한 아티팩트 바이트 또는 안전한 알림을 나중에 `record_run`이 승격할 수 있는 임시 스테이징 핸들로 스테이징합니다. |
| [`harness.record_run`](#harnessrecord_run) | `shaping_update`, `direct`, `implementation` 종류의 작업과 간결한 증거/아티팩트 참조를 기록합니다. |
| [`harness.request_user_judgment`](#harnessrequest_user_judgment) | 대기 중인 사용자 소유 판단 요청 하나를 만듭니다. |
| [`harness.record_user_judgment`](#harnessrecord_user_judgment) | 기존 대기 중인 `UserJudgment`에 대한 사용자의 답을 기록합니다. |
| [`harness.close_task`](#harnessclose_task) | 닫기 준비 상태를 확인하고, 차단 사유가 허용할 때만 `complete`, `cancel`, `supersede` intent를 처리합니다. |

메서드 상태 효과는 아래 표로 고정됩니다. "이벤트 생성",
"`tool_invocations` 재실행 행 생성", "`state_version` 증가"는 새로 커밋되는
`dry_run=false` 상태 변경을 뜻합니다. 버전 증가는 항상 프로젝트 전체
`project_state.state_version`을 정확히 1 올리는 것입니다. 멱등 재실행은 기존
커밋 응답을 반환하며 두 번째 이벤트, 재실행 행, 버전 증가를 만들지 않습니다.
커밋된 차단 응답은 "커밋된 차단 응답 허용" 칸이 예라고 말하는 행에서만 그 효과를 가집니다.

| 메서드 | 읽기 전용 또는 상태 변경 | `dry_run` 허용 | `idempotency_key` 필요 | `expected_state_version` 필요 | 커밋된 차단 응답 허용 | 이벤트 생성 | `tool_invocations` 재실행 행 생성 | `state_version` 증가 |
|---|---|---|---|---|---|---|---|---|
| `harness.intake` | 상태 변경 | 예. 커밋하지 않음 | `dry_run=false`에는 필요 | `dry_run=false`에는 필요 | 예. 이 메서드가 쓰기 준비 경로 대신 구체화/차단 사유 상태를 커밋할 때 | 예. 커밋 시 | 예. 첫 커밋 시 | 예. 커밋 시 |
| `harness.status` | 읽기 전용 | 예. 읽기 전용 결과 반환 | 필요 없음 | 필요 없음. `null` 가능 | 아니요. 차단 사유는 계산된 응답 필드일 뿐입니다. | 아니요 | 아니요 | 아니요 |
| `harness.update_scope` | 상태 변경 | 예. 커밋하지 않음 | `dry_run=false`에는 필요 | `dry_run=false`에는 필요 | 예. 메서드가 소유한 차단 사유 또는 현재 행 갱신에 한정합니다. 충족되지 않은 선행조건이 범위 권한을 만들지는 않습니다. | 예. 커밋 시 | 예. 첫 커밋 시 | 예. 커밋 시 |
| `harness.prepare_write` | 상태 변경 | 예. 커밋하지 않음 | `dry_run=false`에는 필요 | `dry_run=false`에는 필요 | 예. 커밋된 `blocked`, `approval_required`, `decision_required` 쓰기 결정 이유 갱신에 한정합니다. 소비 가능한 쓰기 승인(`Write Authorization`)은 만들지 않습니다. | 예. 커밋된 `allowed` 또는 커밋된 쓰기 결정 이유 갱신 시 | 예. 커밋된 `allowed` 또는 커밋된 쓰기 결정 이유 갱신의 첫 커밋 시 | 예. 커밋된 `allowed` 또는 커밋된 쓰기 결정 이유 갱신 시 |
| `harness.stage_artifact` | 임시 아티팩트 유틸리티. Core 상태 변경 아님 | 예. 스테이징 미리보기 | 필요 없음 | 필요 없음. `null` 가능 | 아니요. 유효하지 않은 스테이징 요청은 Core 변경 없이 실패합니다. | 아니요 | 아니요 | 아니요 |
| `harness.record_run` | 상태 변경 | 예. 커밋하지 않음 | `dry_run=false`에는 필요 | `dry_run=false`에는 필요 | 예. 호환되는 Run 또는 Run 관련 차단 사유 상태를 기록할 때만 허용합니다. 거부된 시도는 커밋 전 실패입니다. | 예. 커밋 시 | 예. 첫 커밋 시 | 예. 커밋 시 |
| `harness.request_user_judgment` | 상태 변경 | 예. 커밋하지 않음 | `dry_run=false`에는 필요 | `dry_run=false`에는 필요 | 별도 차단 응답 커밋은 없습니다. 대기 중인 판단 경로를 커밋하거나 커밋 전 실패가 됩니다. | 예. 커밋 시 | 예. 첫 커밋 시 | 예. 커밋 시 |
| `harness.record_user_judgment` | 상태 변경 | 예. 커밋하지 않음 | `dry_run=false`에는 필요 | `dry_run=false`에는 필요 | 예. 지정된 판단을 rejected, deferred, blocked 또는 차단 사유를 만드는 상태로 커밋할 때 | 예. 커밋 시 | 예. 첫 커밋 시 | 예. 커밋 시 |
| `harness.close_task intent=check` | 읽기 전용 | 예. 플래그는 허용하지만 응답 분기는 `CloseTaskResult`로 유지 | 필요 없음 | 필요 없음. `null` 가능 | 아니요. 닫기 차단 사유는 계산된 응답 필드일 뿐입니다. | 아니요 | 아니요 | 아니요 |
| `harness.close_task intent=complete` | 상태 변경 완료 시도 | 예. 커밋하지 않음 | `dry_run=false`에는 필요 | `dry_run=false`에는 필요 | 예. Task를 열린 상태로 둔 채 complete 차단 사유를 저장할 때 | 예. completed 커밋 또는 커밋된 차단 complete 시 | 예. completed 커밋 또는 커밋된 차단 complete의 첫 커밋 시 | 예. completed 커밋 또는 커밋된 차단 complete 시 |
| `harness.close_task intent=cancel` | 상태 변경 종료 취소 시도 | 예. 커밋하지 않음 | `dry_run=false`에는 필요 | `dry_run=false`에는 필요 | 예. cancellation 자체를 무효로 만드는 차단 사유를 Task가 열린 상태로 남기며 저장할 때만 | 예. cancelled 커밋 또는 커밋된 차단 cancellation 시 | 예. cancelled 커밋 또는 커밋된 차단 cancellation의 첫 커밋 시 | 예. cancelled 커밋 또는 커밋된 차단 cancellation 시 |
| `harness.close_task intent=supersede` | 상태 변경 종료 대체 시도 | 예. 커밋하지 않음 | `dry_run=false`에는 필요 | `dry_run=false`에는 필요 | 예. supersession 자체를 무효로 만드는 차단 사유를 Task가 열린 상태로 남기며 저장할 때만 | 예. superseded 커밋 또는 커밋된 차단 supersession 시 | 예. superseded 커밋 또는 커밋된 차단 supersession의 첫 커밋 시 | 예. superseded 커밋 또는 커밋된 차단 supersession 시 |

<a id="shared-request-rules"></a>

## 공통 요청 규칙

모든 메서드는 [`ToolEnvelope`](schema-core.md#tool-envelope)를 사용합니다. 각 공개 메서드 응답은 정확히 하나의 응답 분기입니다. 구체적인 메서드별 `MethodResult`, `ToolRejectedResponse`, 또는 `ToolDryRunResponse` 중 하나입니다. 메서드 결과 스키마는 실제 읽기 결과, 성공한 스테이징 결과, Core 커밋 결과, 또는 메서드 상태 효과 표가 허용하는 커밋된 차단 결과에 대해 구체적인 결과를 이름 붙입니다. 메서드 결과는 `response_kind=result`인 [`ToolResultBase`](schema-core.md#common-response)를 사용합니다. `ToolRejectedResponse`와 `ToolDryRunResponse`는 [API 코어 스키마](schema-core.md#common-response)의 공통 응답 스키마를 사용하며 메서드별 result 전용 필드를 상속하지 않습니다.

아래 예시는 간결한 분기 예시이지 전체 스키마 정의가 아닙니다. 최소 요청 예시는 해당 메서드의 유효한 호출을 구성하는 데 필요한 필드를 포함합니다. 대표 응답 예시는 분기 이해에 중요한 필드를 보여 주며, 설명 중인 동작에 영향을 주지 않는 스키마 담당 중첩 필드는 생략할 수 있습니다. 전체 형태는 연결된 스키마 담당 문서를 사용합니다.

커밋되는 `dry_run=false` 상태 변경 호출은 `null`이 아닌 `idempotency_key`와 현재 프로젝트 전체 `expected_state_version`을 요구합니다. `harness.stage_artifact`, `harness.status`, `harness.close_task intent=check`, `dry_run` 호출은 `idempotency_key: null`과 `expected_state_version: null`을 사용할 수 있습니다. `harness.stage_artifact`는 저장소가 소유하는 임시 스테이징만 만들며, Core 상태 전이가 아니고 재실행 행이나 `project_state.state_version` 증가를 만들지 않습니다.

응답 분기 선택은 규범적이며 아래 우선순위를 따릅니다.

1. 커밋 전 실패는 `dry_run` 여부와 관계없이 `ToolRejectedResponse`를 반환합니다.
2. 선택한 동작이 읽기 전용이면 `dry_run=true`여도 메서드별 `MethodResult`를 반환합니다. 이 결과는 `base.dry_run=true`와 `base.effect_kind=read_only`를 사용합니다.
3. 선택한 동작이 상태 효과 또는 스테이징 효과를 가지고 `dry_run=true`이며, 요청이 그 외에는 유효하고 미리보기 가능하면 `ToolDryRunResponse`를 반환합니다.
4. `dry_run=false` 커밋 동작 또는 성공한 스테이징 동작은 메서드별 `MethodResult`를 반환합니다.

혼합 intent 메서드는 메서드 이름이 아니라 선택된 intent의 상태 효과로 응답 분기를 고릅니다. `harness.close_task`에서 `intent=check`는 읽기 전용이고, `intent=complete`, `intent=cancel`, `intent=supersede`는 상태 효과가 있는 intent입니다.

응답 분기와 상태 효과는 이렇게 대응합니다. 읽기 전용 결과는 `effect_kind=read_only`인 `MethodResult`입니다. Core 커밋 결과는 `effect_kind=core_committed`인 `MethodResult`입니다. 성공한 `harness.stage_artifact` 스테이징 결과는 `effect_kind=staging_created`인 `StageArtifactResult`입니다. 커밋 전 실패는 `response_kind=rejected`, `effect_kind=no_effect`인 `ToolRejectedResponse`입니다. 상태 효과가 있는 선택 동작 또는 스테이징 선택 동작의 유효한 `dry_run`은 `response_kind=dry_run`, `effect_kind=no_effect`인 `ToolDryRunResponse`입니다.

`ToolRejectedResponse`는 오래된 `expected_state_version` / `STATE_VERSION_CONFLICT`, 요청 검증 실패, 유효하지 않은 스테이징 핸들, MCP/Core 또는 로컬 접점 사용 불가, 로컬 접근 불일치, 역량 실패처럼 메서드 커밋 전에 발생한 실패에 사용합니다. `decision`, `task_ref`, `run_summary`, `staged_artifact_handle`, `close_state` 같은 메서드별 result 전용 필드는 포함하지 않습니다.

`ToolDryRunResponse`는 `dry_run=true` 호출이 상태 효과나 저장소 소유 스테이징 효과를 가진 선택 동작이고, Core가 요청 형태, 로컬 접근, 역량, 도달 가능한 상태/선행조건을 충분히 평가해 미리보기를 만들 수 있을 때 사용합니다. 이 응답은 `effect_kind=no_effect`이며 상태 효과가 없고, `task_ref`, `run_summary`, `staged_artifact_handle`, `write_authorization_ref`, `user_judgment_ref` 같은 메서드별 result 전용 필드나 실제 생성된 참조를 포함하지 않습니다. 미리보기의 예상 차단 사유는 `DryRunSummary.would_blockers: PlannedBlocker[]` 항목일 뿐이며 저장된 `WriteDecisionReason`이나 실제 `CloseReadinessBlocker` 객체가 아닙니다. `dry_run` 요청이 읽기 전용 결과나 미리보기를 만들기 전에 검증, 로컬 접근 확인, 역량 확인, 상태 조회에서 실패하면 응답은 `dry_run=true`, `effect_kind=no_effect`인 `ToolRejectedResponse`입니다.

명시적인 읽기 전용 예시는 다음과 같습니다. `harness.status`에 `dry_run=true`를 보내면 `base.dry_run=true`, `base.effect_kind=read_only`인 `StatusResult`를 반환합니다. `harness.close_task`에 `intent=check`와 `dry_run=true`를 보내면 `base.dry_run=true`, `base.effect_kind=read_only`인 `CloseTaskResult`를 반환합니다. 반대로 `harness.close_task`에 `intent=complete`, `intent=cancel`, 또는 `intent=supersede`와 `dry_run=true`를 보내고 요청이 그 외에는 유효하며 미리보기 가능하면 `ToolDryRunResponse`를 반환합니다.

커밋된 차단 결과와 거절 응답은 다릅니다. `harness.prepare_write` 또는 `harness.close_task`의 커밋된 차단 결과는 메서드별 상태 효과 표가 차단 커밋을 허용할 때 `MethodResult`입니다. 오래된 `expected_state_version`, 검증 실패, 잘못된 스테이징 핸들, 사용할 수 없는 로컬 접점, 그와 비슷한 커밋 전 실패는 `ToolRejectedResponse`입니다.

메서드에 도구별 `task_id`가 있으면 Core는 기본 Task를 도구별 `task_id`, `ToolEnvelope.task_id`, 활성 Task 순서로 해석합니다. 이 해석은 담당 기록을 고르는 것이지 별도 상태 시계를 고르는 것이 아닙니다. 새로 커밋되는 모든 `dry_run=false` 상태 변경은 커밋 전에 `ToolEnvelope.expected_state_version`을 현재 `project_state.state_version`과 비교합니다.
불일치는 `STATE_VERSION_CONFLICT`를 반환합니다. 어느 메서드도 별도 공개 stale-state 오류나 저장소 계층 alias를 정의하지 않습니다.

`STATE_VERSION_CONFLICT`는 `ToolRejectedResponse.errors`에만 나타납니다. `WriteDecisionReason.code`, `CloseReadinessBlocker.code`, `PlannedBlocker.code`가 되면 안 됩니다.

읽기 전용 호출은 차단 사유, 닫기 차단 사유, 다음 행동, 진단을 계산해 반환할 수 있지만 그 값은 응답 필드일 뿐입니다. 차단 사유 저장, `task_events` 추가, `tool_invocations` 재실행 행 생성, `state_version` 증가, 닫기 상태 변경, 아티팩트 생성·갱신·연결, 스테이징 핸들 소비, 쓰기 승인 생성 또는 소비를 하면 안 됩니다.

`dry_run=true`는 절대 권한 근거가 아닙니다. 상태 효과 또는 스테이징 동작에 대한 유효한 `dry_run`은 설명적인 `PlannedEffect` 미리보기 데이터, `PlannedBlocker` 후보 차단 사유, 미리보기 가능한 예상 진단, 다음 행동을 담은 `ToolDryRunResponse.dry_run_summary`를 반환합니다. 현재 기록, `task_events` 행, 지속 아티팩트, 스테이징 핸들, 쓰기 승인 생성 또는 소비, 증거 요약, 닫기 상태, `tool_invocations` 재실행 행, 상태 버전 증가를 만들지 않으며, 미리보기 설명에는 존재하지 않는 기록의 가짜 참조를 넣으면 안 됩니다.

커밋된 `dry_run=false` 상태 변경만 `tool_invocations` 재실행 행을 만듭니다. 같은 `idempotency_key`와 같은 요청 해시로 재실행하면 기존 커밋 응답을 반환합니다. 같은 키에 다른 요청 해시를 사용하면 `STATE_VERSION_CONFLICT`를 반환합니다. `dry_run` 호출과 커밋 전 실패는 재실행 행을 만들거나 예약하지 않습니다.

오류 코드, 주 오류 우선순위, 멱등성, stale-state 동작, 닫기 차단 사유 순서, 사용자 표시 오류 라벨은 [API 오류](errors.md)가 담당합니다. 공통 요청 래퍼와 응답 분기는 [API 코어 스키마](schema-core.md)가 담당합니다. 상태, 아티팩트, 판단, 값 집합 스키마는 분리된 API 스키마 담당 문서가 소유합니다.

로컬 접근 등급은 하네스 API 호환성 등급이지 OS 권한 등급이 아닙니다. `ToolEnvelope.surface_id`는 모든 공개 요청에 필요하지만 선택자일 뿐입니다. 권한 증명이 아니며, API가 그 접점을 신뢰하려면 서버가 도출한 `VerifiedSurfaceContext`와 일치해야 합니다. 활성 `access_class` 값은 [API 값 집합](schema-value-sets.md#access-class-values)이 담당합니다. 서버는 사용자 산문, 생성된 Markdown, Product Repository 파일, Projection, 대화 텍스트, 에이전트 기억이 아니라 로컬 전송/세션/바인딩과 저장된 `LocalSurfaceRegistration`에서 `VerifiedSurfaceContext`를 도출합니다. 같은 서버 도출 context만 스테이징 핸들의 `created_by_surface_id`와 `created_by_surface_instance_id` 출처 기록의 근거입니다.

모든 접근 등급은 API가 접점에 의존하기 전에 `surface_id`가 같은 프로젝트의 `status=active`인 `LocalSurfaceRegistration`을 선택해야 합니다. 각 공개 API 요청에는 요청 수준 `VerifiedSurfaceContext.access_class`가 정확히 하나 있습니다. `ArtifactInput[]` 같은 중첩 페이로드는 두 번째 접근 등급을 추가하지 않습니다. 모든 상태 변경 API는 커밋 전에 해당 메서드 접근 등급에 대해 `VerifiedSurfaceContext.verified=true`가 필요합니다. 아티팩트 본문 읽기도 `access_class=artifact_read`에 대해 `VerifiedSurfaceContext.verified=true`가 필요합니다. 적용되는 경우 보호된 읽기가 Core 세부정보를 노출하거나 상태 변경이 커밋되기 전에 `project_id`, `surface_id`, `surface_instance_id`, `task_id`, 현재 프로젝트 전체 `expected_state_version`이 서로 호환되어야 합니다.

| 접근 등급 | 적용 범위 | 최소 접근 조건 |
|---|---|---|
| `read_status` | `harness.status`, 읽기 전용 상태 리소스, `harness.close_task intent=check` 같은 읽기 전용 상태/Projection 메서드. | 같은 프로젝트의 `LocalSurfaceRegistration`, `status=active`, 요청한 읽기에 필요한 Core/접점 도달 가능성, 보호된 Core 세부정보에는 `VerifiedSurfaceContext.access_class=read_status`, Task 범위 읽기라면 호환되는 `task_id`가 필요합니다. 상태 읽기는 표시해도 안전한 가용성 또는 불일치 진단을 반환할 수 있지만, 오래된 텍스트에서 상태를 만들어 내거나 로컬 접근을 확인할 수 없을 때 보호되어야 할 Core 세부정보를 노출하면 안 됩니다. |
| `core_mutation` | 별도 분류가 없는 Core 상태 변경입니다. `harness.intake`를 통한 Task 생성, `harness.update_scope`, `harness.request_user_judgment`, `harness.record_user_judgment`, 상태를 바꾸는 `harness.close_task`가 여기에 속합니다. | `read_status` 조건에 더해 `VerifiedSurfaceContext.access_class=core_mutation`, `verified=true`, `dry_run=false` 커밋에는 `null`이 아닌 `idempotency_key`와 현재 프로젝트 전체 `expected_state_version`, 적용되는 경우 호환되는 `project_id`, `surface_id`, `surface_instance_id`, `task_id`, 담당 기록이 필요합니다. |
| `write_authorization` | `harness.prepare_write`. | `VerifiedSurfaceContext.access_class=write_authorization`, `verified=true`에 더해 의도한 제품 파일 쓰기 시도에 필요한 활성 Task/Change Unit 호환성, 범위, baseline, 필요한 별도 민감 동작 승인 호환성, 역량 확인이 필요합니다. |
| `run_recording` | `harness.record_run`만 해당합니다. | `VerifiedSurfaceContext.access_class=run_recording`, `verified=true`에 더해 호환되는 `task_id`, `change_unit_id`, `baseline_ref`, 관찰된 시도 사실, 그리고 제품 쓰기를 기록하는 Run이면 소비 가능한 활성 쓰기 승인(`Write Authorization`)이 필요합니다. 같은 `run_recording` 요청이 실행 결과 기록, 필요할 때 호환되는 쓰기 승인 소비, 호환되는 기존 아티팩트 연결, 스테이징 핸들 유효성 확인 뒤 적격 `staged_artifact` 승격을 다룹니다. 승격에는 현재 확인된 `surface_id`와 `surface_instance_id`가 스테이징 핸들에 서버가 기록한 `created_by_surface_id`와 `created_by_surface_instance_id`와 일치해야 합니다. 현재 MVP에는 접점 간 스테이징 핸들 인계가 없습니다. `ArtifactInput[]`에 `source_kind=staged_artifact`가 있어도 `harness.record_run`은 `artifact_registration`을 요구하지 않습니다. |
| `artifact_registration` | `harness.stage_artifact`만 해당합니다. | `VerifiedSurfaceContext.access_class=artifact_registration`, `verified=true`, 호환되는 `project_id`/`task_id`, 새 아티팩트 바이트나 안전 공지를 임시 `StagedArtifactHandle`로 스테이징하기 위한 `manual_artifact_attachment_supported=true`가 필요합니다. 성공하면 서버는 `VerifiedSurfaceContext`에서 `created_by_surface_id`와 `created_by_surface_instance_id`를 기록합니다. 호출자는 이 필드를 권한 주장으로 제출하지 않습니다. 이는 입력 스테이징이지 지속 `ArtifactRef` 승격이 아니고, 임의 로컬 파일이 안전하거나 허가되었다는 증명도 아니며, `harness.record_run`의 두 번째 접근 분류도 아닙니다. 호출자가 임의로 준 파일시스템 경로, 임의 로컬 경로 문자열, 권한 주장으로서의 원시 로그, 원시 비밀값, 토큰, 민감한 전체 로그, `captured_artifact` 핸들, 원시 캡처 어댑터 출력, 접점 자체 캡처 주장은 현재 MVP의 아티팩트 권한으로 인정하지 않습니다. |
| `artifact_read` | 담당 경로가 노출한 등록된 `ArtifactRef` 기록의 아티팩트 본문 읽기. | 같은 프로젝트의 `LocalSurfaceRegistration`, `status=active`, 등록된 `ArtifactRef`, 호환되는 `project_id`/`task_id`, 필요한 redaction과 availability 확인, `artifact_links`의 일치하는 담당 관계가 필요합니다. 아티팩트 본문 읽기는 `VerifiedSurfaceContext.access_class=artifact_read`와 `verified=true`가 필요합니다. 아티팩트 본문 읽기는 스테이징 핸들 승격과 별개이며, 원시 아티팩트 경로 읽기는 기본으로 부여되지 않습니다. |

필요한 MCP/Core 또는 접점 도달 가능성 자체가 없으면 `VerifiedSurfaceContext.failure_reason=unavailable`에 대응하는 `MCP_UNAVAILABLE`을 사용합니다. 등록된 로컬 접근 기대가 도달 가능한 전송/세션/바인딩과 맞지 않거나 로컬 접근이 취소되었으면 `failure_reason=mismatch` 또는 `revoked`에 대응하는 `LOCAL_ACCESS_MISMATCH`를 사용합니다. 접점은 인식되지만 접근 등급, 관찰, 캡처, 차단/격리 주장, 변경 경로 탐지 주장, 활성 동작에 필요한 역량이 없으면 `failure_reason=insufficient_capability`에 대응하는 `CAPABILITY_INSUFFICIENT`를 사용합니다. baseline 변경 경로 탐지에서 메서드가 그 역량을 요구하는 경우 `changed_path_detection_verification=failed` 또는 `stale`은 `CAPABILITY_INSUFFICIENT`를 만들어야 합니다. `not_run` 또는 legacy `planned_not_run`은 `detective` 라벨을 뒷받침할 수 없습니다.

<a id="harnessintake"></a>

## `harness.intake`

### 목적

평소 사용자 작업 루프를 시작, 재개, 대체, 거절하고 요청된 모드를 구체적인 `advisor`, `direct`, `work` Task 상태로 확정합니다. `harness.intake`는 쓰기 가능한 작업의 첫 범위 후보를 만들 수 있지만, 이후 범위 변경은 `harness.update_scope`가 담당합니다.

### 필수 입력

- `ToolEnvelope`: `project_id`, `surface_id`, `request_id`, `dry_run`이 필요하며, `dry_run=false` 커밋에는 `null`이 아닌 `idempotency_key`와 현재 `expected_state_version`이 필요합니다.
- `user_request`, `requested_mode`, `resume_policy`.
- 알고 있는 `acceptance_criteria`, `constraints.allowed_paths`, `constraints.non_goals`, `constraints.sensitive_categories`, `initial_context_refs`. 없으면 빈 배열을 사용합니다.

### 접근 요구사항

`dry_run=false` 커밋에는 `VerifiedSurfaceContext.access_class=core_mutation`과 `verified=true`가 필요합니다. `surface_id`는 등록된 로컬 접점을 고르는 선택자이며, 그 자체가 권한이 아닙니다.

### 상태 버전 동작

커밋된 `dry_run=false` 결과는 프로젝트 전체 `project_state.state_version`을 정확히 한 번 올리고 멱등 키에 대한 재실행 행을 만듭니다. `dry_run`, 읽기 실패, 검증 실패, 로컬 접근 실패, 오래된 `expected_state_version`은 Task, Change Unit, 이벤트, 재실행 행, 차단 사유 갱신, 상태 버전 증가를 만들지 않습니다.

### 성공 결과

`base.response_kind=result`, `base.effect_kind=core_committed`인 `IntakeResult`를 반환합니다. 결과에는 `task_ref`, 선택적 `change_unit_ref`, 현재 `state`, `next_actions`가 들어갑니다. `requested_mode=auto`라면 저장되고 표시되는 모드는 확정된 구체적 모드여야 하며 `auto`가 되면 안 됩니다.

### 차단 결과

이 메서드는 쓰기 준비 경로 대신 shaping 또는 차단 사유 상태를 기록하는 커밋된 `IntakeResult`를 반환할 수 있습니다. 차단 질문은 별도 Discovery Brief, Question Queue, Assumption Register 아티팩트가 아니라 Task, Change Unit, 사용자 판단, 증거, 차단 사유, 다음 행동 필드로 표현해야 합니다.

### 거절 결과

검증 실패, 오래된 `expected_state_version`, Core 또는 로컬 접점 사용 불가, 로컬 접근 불일치, 활성 Task 호환성 부족, validator 실패처럼 커밋 전 실패가 있으면 `ToolRejectedResponse`를 반환합니다. 공개 오류 코드 의미와 우선순위는 [API 오류](errors.md)가 담당합니다.

### `dry_run` 동작

`dry_run=true`에서 유효한 상태 효과 미리보기는 `IntakeResult`가 아니라 `ToolDryRunResponse`를 반환합니다. 미리보기는 `DryRunSummary.planned_effects`로 Task 또는 Change Unit 효과를 설명할 수 있지만 실제 `task_ref`, `change_unit_ref`, 재실행 행, 이벤트, 상태 버전 증가는 만들 수 없습니다.

### 저장 효과

커밋 시 `project_state`, `tasks`, `change_units`, `blockers`, `task_events`, `tool_invocations`를 건드릴 수 있습니다. 저장 효과 의미는 [저장 효과](../storage-effects.md)가 담당하고, 저장 기록 형태는 [저장소 기록](../storage-records.md)이 담당합니다.

### 최소 유효 요청

```yaml
method: harness.intake
params:
  envelope:
    project_id: proj_123
    task_id: null
    actor_kind: agent
    surface_id: surface_local
    request_id: req_intake_001
    idempotency_key: idem_intake_001
    expected_state_version: 17
    dry_run: false
    locale: ko-KR
  user_request: "MVP 문서의 API 참조 예시를 갱신해 주세요."
  requested_mode: auto
  resume_policy: create_new
  acceptance_criteria:
    - "각 활성 메서드에 최소 요청과 대표 응답이 있다."
  constraints:
    allowed_paths:
      - docs/ko/reference/api/mvp-api.md
    non_goals:
      - "런타임 구현"
    sensitive_categories: []
  initial_context_refs: []
```

### 대표 응답

결과 분기(`IntakeResult`, 커밋됨):

```yaml
base:
  response_kind: result
  effect_kind: core_committed
  dry_run: false
  state_version: 18
  events:
    - event_id: evt_1001
      event_kind: task_intake
task_ref:
  record_kind: task
  record_id: task_456
  project_id: proj_123
  task_id: task_456
  state_version: 18
change_unit_ref: null
state:
  project_id: proj_123
  state_version: 18
  task_ref:
    record_kind: task
    record_id: task_456
    project_id: proj_123
    task_id: task_456
    state_version: 18
  mode: work
  lifecycle:
    lifecycle_phase: shaping
    close_reason: none
    result: none
    closed_at: null
  goal_summary: "MVP API 메서드 예시 갱신"
  scope_summary: null
  active_change_unit_ref: null
  blocker_refs: []
next_actions:
  - action: harness.update_scope
    reason: "쓰기 확인 전에 첫 활성 Change Unit을 만든다."
```

### 담당 문서 링크

- 요청 래퍼와 응답 분기: [API 코어 스키마](schema-core.md#tool-envelope), [공통 응답 분기](schema-core.md#common-response).
- 상태 참조, `StateSummary`, `ShapingReadiness`, 다음 행동: [API 상태 스키마](schema-state.md).
- 활성 메서드 이름, 모드 값, `resume_policy`, `response_kind`, `effect_kind`, 접근 등급: [API 값 집합](schema-value-sets.md).
- 공개 오류와 상태 버전 충돌: [API 오류](errors.md).
- 저장 효과: [저장 효과](../storage-effects.md), [저장소 버전 관리](../storage-versioning.md).

<a id="harnessupdate_scope"></a>

## `harness.update_scope`

### 목적

`harness.intake` 이후 활성 Task의 목표 요약, 범위 경계, 범위 밖 항목, 수락 기준, 자율성 경계, baseline 참조, 활성 Change Unit을 갱신합니다. 사용자 소유 차단 사유가 처리되면 shaping 상태를 안전한 첫 Change Unit으로 옮기는 활성 경로입니다.

### 필수 입력

- `ToolEnvelope`: `dry_run=false` 커밋에는 `null`이 아닌 `idempotency_key`와 현재 `expected_state_version`이 필요합니다.
- `task_id`.
- 바꿀 상위 범위 필드. `null`은 현재 값을 유지한다는 뜻이고, 빈 배열은 해당 목록을 빈 목록으로 교체합니다.
- `change_unit.operation`과 그 작업에 필요한 필드.
- 해결된 `judgment_kind=scope_decision`을 적용한다면 `related_scope_decision_refs`.

### 접근 요구사항

`dry_run=false` 커밋에는 `VerifiedSurfaceContext.access_class=core_mutation`과 `verified=true`가 필요합니다. 요청은 같은 프로젝트의 호환되는 Task를 식별해야 하며, 활성 Change Unit을 만들거나 교체할 때는 다음 안전한 행동을 정직하게 만들 만큼의 범위를 제공해야 합니다.

### 상태 버전 동작

커밋된 `dry_run=false` 결과는 `project_state.state_version`을 정확히 한 번 올립니다. 범위, baseline, 수락 기준, 범위 밖 항목, 자율성 경계, Change Unit, 프로젝트 상태가 활성 쓰기 승인(`Write Authorization`)의 기준 상태와 더 이상 맞지 않으면 Core는 그 승인을 `status=stale`로 표시합니다. 소비, 철회, 만료, 조용한 재사용은 하지 않습니다.

### 성공 결과

`base.response_kind=result`, `base.effect_kind=core_committed`인 `UpdateScopeResult`를 반환합니다. 결과에는 `task_ref`, 선택적 `change_unit_ref`, 연결된 scope decision 참조, stale 쓰기 승인 참조, 차단 사유 참조, 현재 `state`, `next_actions`가 들어갑니다.

### 차단 결과

범위가 아직 준비되지 않았을 때 메서드가 소유한 차단 사유 또는 현재 행 갱신을 커밋할 수 있습니다. 커밋된 차단 범위 결과는 필요한 사용자 소유 판단 범주가 `product_decision`, `technical_decision`, `scope_decision`, `sensitive_approval` 중 무엇인지 식별해야 하며, 막연한 모호함 뒤에 숨기면 안 됩니다.

### 거절 결과

오래된 `expected_state_version`, 유효하지 않은 Task 식별, 유효하지 않은 Change Unit 작업, 필요한 범위 누락, 범위 위반, 미해결 필수 판단, 자율성 경계 위반, stale baseline, 로컬 접근 실패, validator 실패 같은 커밋 전 실패는 `ToolRejectedResponse`를 반환합니다. 공개 오류 코드 의미와 우선순위는 [API 오류](errors.md)가 담당합니다.

### `dry_run` 동작

`dry_run=true`에서 유효한 미리보기는 `ToolDryRunResponse`를 반환합니다. `DryRunSummary`로 범위, 활성 Change Unit, 차단 사유, stale 쓰기 승인 효과를 미리 보여 줄 수 있지만 범위 갱신, 실제 stale 승인 참조, 이벤트, 재실행 행, 상태 버전 증가는 만들지 않습니다.

### 저장 효과

커밋 시 `tasks`, `change_units`, `write_authorizations`, `blockers`, `task_events`, `tool_invocations`, 프로젝트 상태 시계를 건드릴 수 있습니다. 저장 효과 의미는 [저장 효과](../storage-effects.md)가 담당합니다.

### 최소 유효 요청

```yaml
method: harness.update_scope
params:
  envelope:
    project_id: proj_123
    task_id: task_456
    actor_kind: agent
    surface_id: surface_local
    request_id: req_scope_001
    idempotency_key: idem_scope_001
    expected_state_version: 18
    dry_run: false
    locale: ko-KR
  task_id: task_456
  goal_summary: "활성 MVP API 메서드 참조 섹션 재구성"
  scope_boundary: "docs/en/reference/api/mvp-api.md와 docs/ko/reference/api/mvp-api.md만."
  non_goals:
    - "런타임 API 코드 구현"
  acceptance_criteria:
    - "모든 활성 메서드가 표준 섹션 패턴을 따른다."
  autonomy_boundary: "문서 전용 편집."
  baseline_ref: baseline_docs_2026_06_10
  change_unit:
    operation: create_active
    scope_summary: "메서드 본문을 일관된 참조 섹션으로 교체한다."
    affected_areas:
      - "API 참조 문서"
    affected_paths:
      - docs/en/reference/api/mvp-api.md
      - docs/ko/reference/api/mvp-api.md
    constraints:
      - "메서드 식별자와 담당 문서 링크를 보존한다."
  related_scope_decision_refs: []
```

### 대표 응답

결과 분기(`UpdateScopeResult`, 커밋됨):

```yaml
base:
  response_kind: result
  effect_kind: core_committed
  dry_run: false
  state_version: 19
  events:
    - event_id: evt_1002
      event_kind: scope_updated
task_ref:
  record_kind: task
  record_id: task_456
  project_id: proj_123
  task_id: task_456
  state_version: 19
change_unit_ref:
  record_kind: change_unit
  record_id: cu_001
  project_id: proj_123
  task_id: task_456
  state_version: 19
linked_scope_decision_refs: []
stale_write_authorization_refs: []
blocker_refs: []
state:
  project_id: proj_123
  state_version: 19
  task_ref:
    record_kind: task
    record_id: task_456
    project_id: proj_123
    task_id: task_456
    state_version: 19
  mode: work
  lifecycle:
    lifecycle_phase: ready
    close_reason: none
    result: none
    closed_at: null
  goal_summary: "활성 MVP API 메서드 참조 섹션 재구성"
  scope_summary: "대응되는 MVP API 참조 문서만."
  active_change_unit_ref:
    record_kind: change_unit
    record_id: cu_001
    project_id: proj_123
    task_id: task_456
    state_version: 19
next_actions:
  - action: harness.prepare_write
    reason: "첫 문서 쓰기를 활성 범위와 비교한다."
```

### 담당 문서 링크

- 요청 래퍼와 응답 분기: [API 코어 스키마](schema-core.md).
- 상태 참조, `StateSummary`, `ShapingReadiness`, 차단 사유, 다음 행동: [API 상태 스키마](schema-state.md).
- 범위 관련 사용자 판단 형태: [API 판단 스키마](schema-judgment.md).
- 활성 값 집합과 접근 등급: [API 값 집합](schema-value-sets.md).
- 공개 오류: [API 오류](errors.md).
- 저장 효과와 stale 쓰기 승인 동작: [저장 효과](../storage-effects.md), [저장소 버전 관리](../storage-versioning.md).

<a id="harnessstatus"></a>

## `harness.status`

### 목적

Core 상태의 읽기 전용 현재 위치 보기를 반환합니다. 활성 Task 요약, 차단 사유, 대기 중인 사용자 판단, 쓰기 승인 요약, 증거 요약, 닫기 상태, 닫기 준비 상태 발견 사항, 보장 표시, 다음 안전한 행동을 포함할 수 있습니다.

### 필수 입력

- `ToolEnvelope`: `project_id`, `surface_id`, `request_id`, `dry_run`이 필요합니다. `idempotency_key`와 `expected_state_version`은 `null`일 수 있습니다.
- 호출자가 필요한 요약을 고르는 `include` 플래그.

### 접근 요구사항

보호된 Core 세부정보를 반환하려면 같은 프로젝트의 활성 로컬 접점과 `VerifiedSurfaceContext.access_class=read_status`가 필요합니다. 오래된 Projection, 대화 요약, 생성된 Markdown 파일, 캐시된 텍스트는 상태 권한 근거가 아닙니다.

### 상태 버전 동작

상태 변경은 없고 `project_state.state_version`을 올리지 않습니다. 결과는 현재 관찰된 상태 버전을 보고할 수 있지만 이벤트, 재실행 행, 닫기 변경, 아티팩트 효과, 스테이징 핸들 소비, 증거 갱신, 쓰기 승인 변경을 만들지 않습니다.

### 성공 결과

`base.response_kind=result`, `base.effect_kind=read_only`인 `StatusResult`를 반환합니다. `include.close=true`일 때 `StatusResult.close_blockers`는 읽기 전용 관찰인 `CloseReadinessBlocker[]`입니다. 저장된 `close_task` 결과가 아닙니다.

### 차단 결과

커밋된 차단 분기는 없습니다. `StatusResult`의 차단 사유와 닫기 차단 사유는 계산된 응답 필드일 뿐입니다.

### 거절 결과

Core 사용 불가, 로컬 접근 불일치, 요청한 보호 세부정보에 대한 역량 부족, Task 범위 읽기에 필요한 활성 Task 없음, 요청한 Projection이 stale 또는 사용 불가인 경우처럼 읽기를 안전하게 제공할 수 없으면 `ToolRejectedResponse`를 반환합니다. 공개 오류 코드 의미와 우선순위는 [API 오류](errors.md)가 담당합니다.

### `dry_run` 동작

이 읽기 전용 메서드에서는 `dry_run=true`가 `ToolDryRunResponse` 분기를 만들지 않습니다. 유효한 요청은 같은 `StatusResult` 형태를 반환하며 `base.dry_run=true`, `base.effect_kind=read_only`를 사용합니다.

### 저장 효과

`project_state`, `tasks`, `change_units`, `user_judgments`, `write_authorizations`, `runs`, `evidence_summaries`, `artifacts`, `artifact_links`, `blockers`를 읽기 전용으로 봅니다. 저장 효과 의미는 [저장 효과](../storage-effects.md)가 담당합니다.

### 최소 유효 요청

```yaml
method: harness.status
params:
  envelope:
    project_id: proj_123
    task_id: task_456
    actor_kind: agent
    surface_id: surface_local
    request_id: req_status_001
    idempotency_key: null
    expected_state_version: null
    dry_run: false
    locale: ko-KR
  include:
    task: true
    pending_user_judgments: true
    write_authority: true
    evidence: true
    close: true
    guarantees: true
```

### 대표 응답

결과 분기(`StatusResult`, 읽기 전용):

```yaml
base:
  response_kind: result
  effect_kind: read_only
  dry_run: false
  state_version: 19
  events: []
active_task:
  project_id: proj_123
  state_version: 19
  task_ref:
    record_kind: task
    record_id: task_456
    project_id: proj_123
    task_id: task_456
    state_version: 19
  mode: work
  lifecycle:
    lifecycle_phase: ready
    close_reason: none
    result: none
    closed_at: null
  goal_summary: "활성 MVP API 메서드 참조 섹션 재구성"
  active_change_unit_ref:
    record_kind: change_unit
    record_id: cu_001
    project_id: proj_123
    task_id: task_456
    state_version: 19
status_card: "Task가 쓰기 전 확인을 할 준비가 되었습니다."
next_actions:
  - action: harness.prepare_write
    reason: "다음 행동은 제품 파일 문서 편집입니다."
pending_user_judgments: []
write_authority_summary: null
evidence_summary: null
blocker_refs: []
close_state: blocked
close_blockers:
  - category: evidence
    code: EVIDENCE_INSUFFICIENT
    message: "아직 Run 증거가 기록되지 않았습니다."
    related_refs: []
guarantee_display:
  level: cooperative
  notes:
    - "더 강한 로컬 보장이 활성화되지 않았습니다."
```

### 담당 문서 링크

- 요청 래퍼와 응답 분기: [API 코어 스키마](schema-core.md).
- 상태, 닫기 준비 상태 형태, 증거 요약, 보장 표시: [API 상태 스키마](schema-state.md).
- 활성 값과 접근 등급: [API 값 집합](schema-value-sets.md).
- 공개 오류와 닫기 차단 사유 경로: [API 오류](errors.md), [`harness.close_task` 닫기 준비 상태 평가와 닫기 차단 사유](errors.md#harnessclose_task-close-blockers).
- 저장 효과: [저장 효과](../storage-effects.md).

<a id="harnessprepare_write"></a>

## `harness.prepare_write`

### 목적

제안된 제품 파일 쓰기 하나를 현재 Task, 활성 Change Unit, 범위, baseline, 필요한 별도 민감 동작 승인, 확인된 로컬 접점 역량과 비교합니다. 허용되면 소비 가능한 단일 사용 쓰기 승인(`Write Authorization`)을 만듭니다. 허용되지 않으면 그 쓰기 승인 경로를 거부하거나 미룹니다. 이는 일반 권한, OS 수준 강제, 샌드박스, 도구 실행 전 차단이 아닙니다.

### 필수 입력

- `ToolEnvelope`: `dry_run=false` 커밋에는 `null`이 아닌 `idempotency_key`와 현재 `expected_state_version`이 필요합니다.
- `task_id`와 `change_unit_id`. 담당 해석이 활성 Task와 활성 Change Unit을 모호하지 않게 사용할 수 있을 때만 `null`을 사용할 수 있습니다.
- `intended_operation`, `intended_paths`, `product_file_write_intended`, `sensitive_categories`, `baseline_ref`.

### 접근 요구사항

`VerifiedSurfaceContext.access_class=write_authorization`과 `verified=true`가 필요합니다. 메서드는 호환되는 활성 범위, baseline, 필요한 사용자 소유 판단, 별도 `sensitive_approval`, 의도한 제품 파일 쓰기 확인에 필요한 로컬 접점 역량도 요구합니다.

### 상태 버전 동작

커밋된 `decision=allowed`는 `project_state.state_version`을 정확히 한 번 올리고 경로 수준 `AuthorizedAttemptScope`에 대한 활성 쓰기 승인 하나를 만듭니다. 커밋된 `decision=blocked`, `decision=approval_required`, `decision=decision_required`는 메서드가 소유한 쓰기 결정 이유 상태를 저장하기 위해서만 상태 버전을 올릴 수 있으며 소비 가능한 쓰기 승인을 만들면 안 됩니다. 커밋 전 거절과 `dry_run`은 아무것도 올리지 않습니다.

### 성공 결과

`base.response_kind=result`, `base.effect_kind=core_committed`인 `PrepareWriteResult`를 반환합니다. `decision=allowed`이면 `write_authorization_ref`와 `write_authorization`이 `null`이 아니고, `authorization_effect`는 새 커밋에서 `created`, 멱등 재실행에서 `returned`입니다.

### 차단 결과

커밋된 차단 결정은 `decision=blocked`, `decision=approval_required`, `decision=decision_required`를 가진 `PrepareWriteResult`입니다. `write_decision_reasons`는 비어 있으면 안 됩니다. 이 이유들은 `CloseReadinessBlocker` 값이 아니며 닫기 준비 상태를 평가하지 않습니다. 소비 가능한 쓰기 승인은 만들어지지 않습니다.

### 거절 결과

오래된 `expected_state_version`, 멱등 요청 해시 충돌, 요청 검증 실패, 활성 Task 또는 Change Unit 없음, 로컬 접근 실패, Core 사용 불가, stale baseline, 유효하지 않은 요청 보장, 역량 실패처럼 `decision` 평가나 커밋 전 실패가 있으면 `ToolRejectedResponse`를 반환합니다. `STATE_VERSION_CONFLICT`는 항상 거절 응답 오류이며 쓰기 결정 이유가 아닙니다.

### `dry_run` 동작

`dry_run=true`에서 유효한 미리보기는 `ToolDryRunResponse`를 반환합니다. `DryRunSummary`는 `dry_run=false` 경로가 쓰기 승인을 만들지, 재사용할지, 거절할지를 설명할 수 있습니다. 실제 `write_authorization_ref`, 실제 `WriteDecisionReason`, 이벤트, 재실행 행, 상태 버전 증가는 만들거나 반환하면 안 됩니다.

### 저장 효과

`decision=allowed`이면 `write_authorizations`를 만들거나 반환하고, `task_events`를 추가하고, `tool_invocations`를 만들고, 프로젝트 상태 시계를 갱신할 수 있습니다. 커밋된 차단 결정은 소비 가능한 승인을 만들지 않고 메서드 소유 쓰기 결정 이유 또는 차단 사유 상태를 저장할 수 있습니다. 저장 효과 의미는 [저장 효과](../storage-effects.md)가 담당합니다.

### 최소 유효 요청

```yaml
method: harness.prepare_write
params:
  envelope:
    project_id: proj_123
    task_id: task_456
    actor_kind: agent
    surface_id: surface_local
    request_id: req_prepare_001
    idempotency_key: idem_prepare_001
    expected_state_version: 19
    dry_run: false
    locale: ko-KR
  task_id: task_456
  change_unit_id: cu_001
  intended_operation: "메서드 참조 섹션 교체"
  intended_paths:
    - docs/en/reference/api/mvp-api.md
    - docs/ko/reference/api/mvp-api.md
  product_file_write_intended: true
  sensitive_categories: []
  baseline_ref: baseline_docs_2026_06_10
```

### 대표 응답

결과 분기(`PrepareWriteResult`, `decision=allowed`):

```yaml
base:
  response_kind: result
  effect_kind: core_committed
  dry_run: false
  state_version: 20
  events:
    - event_id: evt_1003
      event_kind: write_authorization_created
decision: allowed
state:
  project_id: proj_123
  state_version: 20
  task_ref:
    record_kind: task
    record_id: task_456
    project_id: proj_123
    task_id: task_456
    state_version: 20
write_authorization_ref:
  record_kind: write_authorization
  record_id: wa_001
  project_id: proj_123
  task_id: task_456
  state_version: 20
write_authorization:
  authorization_id: wa_001
  status: active
  basis_state_version: 19
  authorized_paths:
    - docs/en/reference/api/mvp-api.md
    - docs/ko/reference/api/mvp-api.md
authorization_effect: created
active_user_judgment_refs: []
write_decision_reasons: []
user_judgment_candidate: null
guarantee_display:
  level: cooperative
  notes:
    - "쓰기 승인(`Write Authorization`)은 하네스 호환성 기록이며 OS 권한이 아닙니다."
```

### 담당 문서 링크

- 요청 래퍼, 공통 결과 분기, `dry_run` 요약: [API 코어 스키마](schema-core.md).
- `WriteAuthorizationSummary`, 상태 요약, 참조: [API 상태 스키마](schema-state.md).
- `SensitiveActionScope`와 사용자 소유 승인 경계: [API 판단 스키마](schema-judgment.md).
- 활성 값과 접근 등급: [API 값 집합](schema-value-sets.md).
- 공개 오류, `STATE_VERSION_CONFLICT`, 차단/`dry_run` 동작: [API 오류](errors.md).
- 저장 효과와 상태 시계: [저장 효과](../storage-effects.md), [저장소 버전 관리](../storage-versioning.md).

<a id="harnessstage_artifact"></a>

## `harness.stage_artifact`

### 목적

호출자가 제공한 안전한 아티팩트 바이트 또는 안전한 알림을 같은 프로젝트와 Task에 대한 임시 `StagedArtifactHandle`로 스테이징합니다. 스테이징은 입력 준비일 뿐입니다. 그 자체로 기준 증거, 지속 `ArtifactRef`, gate 충족, 최종 수락, 잔여 위험 수락, 닫기 준비 상태를 만들지 않습니다.

### 필수 입력

- `ToolEnvelope`: `project_id`, `task_id`, `surface_id`, `request_id`, `dry_run`이 필요합니다. `idempotency_key`와 `expected_state_version`은 `null`일 수 있습니다.
- `task_id`, `display_name`, `content_type`, `redaction_state`, `safe_bytes_or_notice`, `expected_sha256`, `expected_size_bytes`, `relation_hint`.

### 접근 요구사항

`VerifiedSurfaceContext.access_class=artifact_registration`, `verified=true`, 호환되는 `project_id`와 `task_id`, `manual_artifact_attachment_supported=true`가 필요합니다. 서버는 확인된 로컬 접점에서 `created_by_surface_id`와 `created_by_surface_instance_id`를 기록합니다. 호출자는 이 값을 권한 근거로 제출하지 않습니다.

### 상태 버전 동작

성공한 스테이징 결과는 Core 상태를 바꾸지 않고 `project_state.state_version`을 올리지 않습니다. `tool_invocations` 재실행 행도 만들지 않습니다. 거절과 `dry_run` 요청은 저장 효과가 없습니다.

### 성공 결과

`base.response_kind=result`, `base.effect_kind=staging_created`인 `StageArtifactResult`를 반환합니다. 결과에는 임시 `staged_artifact_handle`과 `expires_at`이 들어갑니다. 지속 `ArtifactRef`는 포함하지 않습니다.

### 차단 결과

커밋된 차단 분기는 없습니다. 유효하지 않은 스테이징 요청은 Core 변경 전에 거절됩니다. 스테이징 가용성이나 역량 문제는 차단 사유를 만들지 않습니다.

### 거절 결과

유효하지 않은 요청 형태, 체크섬 또는 크기 불일치, 안전하지 않은 아티팩트 입력, 지원하지 않는 redaction 상태, Core 또는 로컬 접점 사용 불가, 로컬 접근 불일치, 아티팩트 등록 역량 부족은 `ToolRejectedResponse`를 반환합니다. 공개 오류 코드 의미와 우선순위는 [API 오류](errors.md)가 담당합니다.

### `dry_run` 동작

`dry_run=true`에서 유효한 스테이징 미리보기는 `StageArtifactResult`가 아니라 `ToolDryRunResponse`를 반환합니다. 미리보기는 예정된 임시 스테이징 효과를 설명할 수 있지만 바이트나 알림, 저장소 소유 스테이징 manifest, `StagedArtifactHandle`, 재실행 행, 상태 버전 증가는 만들지 않습니다.

### 저장 효과

성공 시 `artifact_staging` 또는 동등한 저장소 소유 스테이징 manifest와 `artifacts/tmp/` 아래 임시 바이트 또는 안전 알림만 만듭니다. 지속 `artifacts`와 `artifact_links`는 이후 호환되는 `harness.record_run` 승격으로만 만들 수 있습니다. 저장 효과 의미는 [저장 효과](../storage-effects.md)가 담당하고, 아티팩트 생명주기 세부사항은 [아티팩트 저장소](../storage-artifacts.md)가 담당합니다.

### 최소 유효 요청

```yaml
method: harness.stage_artifact
params:
  envelope:
    project_id: proj_123
    task_id: task_456
    actor_kind: agent
    surface_id: surface_local
    request_id: req_stage_001
    idempotency_key: null
    expected_state_version: null
    dry_run: false
    locale: ko-KR
  task_id: task_456
  display_name: "문서 점검 요약"
  content_type: text/plain
  redaction_state: none
  safe_bytes_or_notice: "런타임 코드는 변경하지 않았습니다."
  expected_sha256: null
  expected_size_bytes: null
  relation_hint: "run_note"
```

### 대표 응답

결과 분기(`StageArtifactResult`, 스테이징 생성):

```yaml
base:
  response_kind: result
  effect_kind: staging_created
  dry_run: false
  state_version: null
  events: []
staged_artifact_handle:
  handle_id: sah_001
  project_id: proj_123
  task_id: task_456
  created_by_surface_id: surface_local
  created_by_surface_instance_id: surface_instance_01
  content_type: text/plain
  sha256: sha256:example
  size_bytes: 28
  redaction_state: none
  expires_at: "2026-06-10T12:30:00Z"
  consumed: false
expires_at: "2026-06-10T12:30:00Z"
```

### 담당 문서 링크

- 요청 래퍼, 응답 분기, `dry_run` 요약: [API 코어 스키마](schema-core.md).
- `StagedArtifactHandle`, `ArtifactInput`, `ArtifactRef`: [API 아티팩트 스키마](schema-artifacts.md).
- 활성 아티팩트 값과 접근 등급: [API 값 집합](schema-value-sets.md).
- 공개 오류: [API 오류](errors.md).
- 저장 효과와 아티팩트 생명주기: [저장 효과](../storage-effects.md), [아티팩트 저장소](../storage-artifacts.md).

<a id="harnessrecord_run"></a>

## `harness.record_run`

### 목적

`shaping_update`, `direct`, `implementation` 종류의 작업을 기록합니다. 또한 간결한 증거 범위를 갱신하고, 제품 쓰기를 기록할 때 호환되는 쓰기 승인을 소비하며, 기존 아티팩트를 연결하고, 허용되는 경우 적격 스테이징 핸들을 지속 `ArtifactRef`로 승격합니다.

### 필수 입력

- `ToolEnvelope`: `dry_run=false` 커밋에는 `null`이 아닌 `idempotency_key`와 현재 `expected_state_version`이 필요합니다.
- `task_id`, `change_unit_id`, `kind`, `run_id`, `baseline_ref`, `write_authorization_id`, `summary`, `observed_changes`, `artifact_inputs`, `evidence_updates`.
- 제품 쓰기 Run은 `harness.prepare_write`가 만든 호환되는 활성 쓰기 승인이 필요합니다.
- 새 아티팩트 바이트는 이미 유효한 `StagedArtifactHandle`로 표현되어 있어야 합니다. `record_run`은 새 바이트를 스테이징하지 않습니다.

### 접근 요구사항

`VerifiedSurfaceContext.access_class=run_recording`과 `verified=true`가 필요합니다. `ArtifactInput[]`는 `artifact_registration`을 추가하지 않습니다. `source_kind=staged_artifact`에서는 현재 확인된 `surface_id`와 `surface_instance_id`가 스테이징 핸들에 서버가 기록한 출처와 일치해야 합니다. 현재 MVP에는 접점 간 스테이징 핸들 인계가 없습니다.

### 상태 버전 동작

호환되는 커밋 결과는 `project_state.state_version`을 정확히 한 번 올립니다. 제품 쓰기 기록은 현재 상태 버전이 승인 기준 상태와 여전히 맞고 관찰된 변경 경로가 승인된 시도와 호환될 때만 활성 쓰기 승인을 소비합니다. 오래된 `expected_state_version` 또는 stale 승인 기준 상태는 소비 전에 거절됩니다.

### 성공 결과

`base.response_kind=result`, `base.effect_kind=core_committed`인 `RecordRunResult`를 반환합니다. 결과에는 `run_summary`, `registered_artifacts`, 갱신된 `evidence_summary`, `blocker_refs`, 현재 `state`가 들어갑니다.

### 차단 결과

Run 자체는 기록 가능하지만 결과가 증거 gap 같은 차단 사유를 만들거나 유지할 때 호환되는 Run 관련 차단 사유 상태를 커밋할 수 있습니다. 유효하지 않은 스테이징 핸들, 누락된 쓰기 승인, stale 상태, stale 승인 기준 상태, 로컬 접근 실패를 숨기기 위해 커밋된 차단 결과를 사용하면 안 됩니다. 그런 경우는 커밋 전에 거절됩니다.

### 거절 결과

오래된 `expected_state_version`, stale 쓰기 승인 기준 상태, 제품 쓰기에 필요한 쓰기 승인 누락 또는 무효, 유효하지 않은 스테이징 핸들, 스테이징 핸들 출처 불일치, 누락된 아티팩트, 범위 위반, baseline stale, 로컬 접근 실패, 역량 부족, validator 실패는 `ToolRejectedResponse`를 반환합니다. 유효하지 않은 스테이징 핸들은 아티팩트 입력 세부정보가 있는 검증 실패이며, 요청 수준 로컬 접근 자체가 실패한 경우가 아니라면 로컬 접근 불일치가 아닙니다.

### `dry_run` 동작

`dry_run=true`에서 유효한 미리보기는 `ToolDryRunResponse`를 반환합니다. Run, 아티팩트 승격, 증거, 차단 사유, 쓰기 승인 소비 효과를 설명할 수 있지만 `run_summary`, 지속 아티팩트, 아티팩트 링크, 증거 갱신, 차단 사유 갱신, 이벤트, 재실행 행, 스테이징 핸들 소비, 쓰기 승인 소비, 상태 버전 증가는 만들지 않습니다.

### 저장 효과

커밋 시 `runs`, `write_authorizations`, `artifact_staging`, `artifacts`, `artifact_links`, `evidence_summaries`, `blockers`, `task_events`, `tool_invocations`, 프로젝트 상태 시계를 건드릴 수 있습니다. 저장 효과 의미는 [저장 효과](../storage-effects.md)가 담당하고, 아티팩트 승격 세부사항은 [아티팩트 저장소](../storage-artifacts.md)가 담당합니다.

### 최소 유효 요청

```yaml
method: harness.record_run
params:
  envelope:
    project_id: proj_123
    task_id: task_456
    actor_kind: agent
    surface_id: surface_local
    request_id: req_run_001
    idempotency_key: idem_run_001
    expected_state_version: 20
    dry_run: false
    locale: ko-KR
  task_id: task_456
  change_unit_id: cu_001
  kind: implementation
  run_id: null
  baseline_ref: baseline_docs_2026_06_10
  write_authorization_id: wa_001
  summary: "메서드 섹션을 표준 API 참조 패턴으로 교체했습니다."
  observed_changes:
    changed_paths:
      - docs/en/reference/api/mvp-api.md
      - docs/ko/reference/api/mvp-api.md
    product_file_write_observed: true
    sensitive_categories: []
    baseline_ref: baseline_docs_2026_06_10
  artifact_inputs: []
  evidence_updates:
    - claim: "각 활성 메서드가 표준 섹션 패턴을 따른다."
      required_for_close: true
      coverage_state: supported
      supporting_refs: []
      supporting_artifact_refs: []
      gap_refs: []
```

### 대표 응답

결과 분기(`RecordRunResult`, 커밋됨):

```yaml
base:
  response_kind: result
  effect_kind: core_committed
  dry_run: false
  state_version: 21
  events:
    - event_id: evt_1004
      event_kind: run_recorded
run_summary:
  run_ref:
    record_kind: run
    record_id: run_001
    project_id: proj_123
    task_id: task_456
    state_version: 21
  kind: implementation
  summary: "메서드 섹션을 표준 API 참조 패턴으로 교체했습니다."
  observed_changes:
    changed_paths:
      - docs/en/reference/api/mvp-api.md
      - docs/ko/reference/api/mvp-api.md
    product_file_write_observed: true
    sensitive_categories: []
    baseline_ref: baseline_docs_2026_06_10
  artifact_refs: []
registered_artifacts: []
evidence_summary:
  status: sufficient
  coverage_items:
    - claim: "각 활성 메서드가 표준 섹션 패턴을 따른다."
      required_for_close: true
      coverage_state: supported
      supporting_refs:
        - record_kind: run
          record_id: run_001
          project_id: proj_123
          task_id: task_456
          state_version: 21
      supporting_artifact_refs: []
      gap_refs: []
  artifact_refs: []
blocker_refs: []
state:
  project_id: proj_123
  state_version: 21
  task_ref:
    record_kind: task
    record_id: task_456
    project_id: proj_123
    task_id: task_456
    state_version: 21
```

### 담당 문서 링크

- 요청 래퍼, 응답 분기, `dry_run` 요약: [API 코어 스키마](schema-core.md).
- `RunSummary`, `EvidenceSummary`, `EvidenceCoverageItem`, `StateSummary`, 참조: [API 상태 스키마](schema-state.md).
- `ArtifactInput`, `StagedArtifactHandle`, `ArtifactRef`: [API 아티팩트 스키마](schema-artifacts.md).
- 쓰기 승인과 닫기 관련 증거 경계: [Core 모델](../core-model.md).
- 활성 값과 접근 등급: [API 값 집합](schema-value-sets.md).
- 공개 오류: [API 오류](errors.md).
- 저장 효과와 아티팩트 승격: [저장 효과](../storage-effects.md), [아티팩트 저장소](../storage-artifacts.md).

<a id="harnessrequest_user_judgment"></a>

## `harness.request_user_judgment`

### 목적

초점이 분명한 사용자 소유 결정 하나에 대해 대기 중인 `UserJudgment`를 만듭니다. 이 메서드는 사용자에게 묻는 경로입니다. 에이전트가 사용자를 대신해 답하거나, 추론하거나, 질문 범위를 넓히거나, 결정을 내려서는 안 됩니다.

### 필수 입력

- `ToolEnvelope`: `dry_run=false` 커밋에는 `null`이 아닌 `idempotency_key`와 현재 `expected_state_version`이 필요합니다.
- `task_id`, `change_unit_id`, `judgment_kind`, `presentation`, `question`, `options`, `context`, `affected_refs`, `required_for`, `expires_at`.
- 사용자가 정확한 사안을 판단할 수 있도록 초점이 분명한 질문, 이해 가능한 선택지, 충분한 맥락.

### 접근 요구사항

`VerifiedSurfaceContext.access_class=core_mutation`과 `verified=true`가 필요합니다. 요청은 같은 프로젝트의 호환되는 Task와 선택적 Change Unit을 대상으로 해야 합니다.

### 상태 버전 동작

커밋된 `dry_run=false` 결과는 `project_state.state_version`을 정확히 한 번 올리고 대기 중인 판단을 만듭니다. 다른 메서드가 반환한 candidate는 이 메서드가 커밋하기 전까지 지속 기록이 아닙니다. `dry_run`과 거절은 대기 중인 판단, 차단 사유 갱신, 이벤트, 재실행 행, 상태 버전 증가를 만들지 않습니다.

### 성공 결과

`base.response_kind=result`, `base.effect_kind=core_committed`인 `RequestUserJudgmentResult`를 반환합니다. 결과에는 `user_judgment_ref`, 대기 중인 `user_judgment`, 영향을 받은 `blocker_refs`, 현재 `state`가 들어갑니다.

### 차단 결과

별도 커밋된 차단 응답 분기는 없습니다. 요청이 유효하지 않거나 선행조건을 확인할 수 없어 판단을 만들 수 없으면 메서드는 커밋 전에 거절합니다.

### 거절 결과

유효하지 않은 질문 형태, 유효하지 않은 `judgment_kind`, Task 없음, 미해결 선행 판단, 로컬 접근 실패, 역량 부족, 오래된 `expected_state_version`, validator 실패는 `ToolRejectedResponse`를 반환합니다. 공개 오류 코드 의미와 우선순위는 [API 오류](errors.md)가 담당합니다.

### `dry_run` 동작

`dry_run=true`에서 유효한 미리보기는 `ToolDryRunResponse`를 반환합니다. 대기 중인 `user_judgment` 계획 효과를 설명할 수 있지만 실제 `user_judgment_ref`, 대기 중인 판단, 차단 사유 갱신, 이벤트, 재실행 행, 상태 버전 증가는 만들면 안 됩니다.

### 저장 효과

커밋 시 `user_judgments`를 만들고, `blockers`를 갱신하거나 연결하며, `task_events`를 추가하고, `tool_invocations`를 만들고, 프로젝트 상태 시계를 갱신할 수 있습니다. 저장 효과 의미는 [저장 효과](../storage-effects.md)가 담당합니다.

### 최소 유효 요청

```yaml
method: harness.request_user_judgment
params:
  envelope:
    project_id: proj_123
    task_id: task_456
    actor_kind: agent
    surface_id: surface_local
    request_id: req_judgment_001
    idempotency_key: idem_judgment_001
    expected_state_version: 21
    dry_run: false
    locale: ko-KR
  task_id: task_456
  change_unit_id: cu_001
  judgment_kind: final_acceptance
  presentation: short
  question: "문서 전용 API 참조 재구성을 완료로 수락하시겠습니까?"
  options:
    - option_id: accept
      label: "수락"
      description: "이 문서 작업의 최종 수락을 기록합니다."
      consequence: "닫기 준비 상태 평가에서 최종 수락을 충족된 것으로 볼 수 있습니다."
      is_default: true
    - option_id: revise
      label: "수정"
      description: "추가 문서 편집을 위해 작업을 열어 둡니다."
      consequence: "최종 수락 때문에 닫기가 계속 차단됩니다."
      is_default: false
  context:
    summary: "대응되는 MVP API 메서드 섹션을 재구성했습니다."
    related_refs: []
    artifact_refs: []
    visible_risks: []
    constraints:
      - "문서 전용 저장소"
  affected_refs:
    - record_kind: task
      record_id: task_456
      project_id: proj_123
      task_id: task_456
      state_version: 21
  required_for: close
  expires_at: null
```

### 대표 응답

결과 분기(`RequestUserJudgmentResult`, 커밋됨):

```yaml
base:
  response_kind: result
  effect_kind: core_committed
  dry_run: false
  state_version: 22
  events:
    - event_id: evt_1005
      event_kind: user_judgment_requested
user_judgment_ref:
  record_kind: user_judgment
  record_id: uj_001
  project_id: proj_123
  task_id: task_456
  state_version: 22
user_judgment:
  judgment_id: uj_001
  project_id: proj_123
  task_id: task_456
  change_unit_id: cu_001
  judgment_kind: final_acceptance
  status: pending
  presentation: short
  question: "문서 전용 API 참조 재구성을 완료로 수락하시겠습니까?"
  options: []
  context:
    summary: "대응되는 MVP API 메서드 섹션을 재구성했습니다."
    related_refs: []
    artifact_refs: []
    visible_risks: []
    constraints:
      - "문서 전용 저장소"
  affected_refs: []
  required_for: close
  resolution: null
  expires_at: null
  created_at: "2026-06-10T12:00:00Z"
  resolved_at: null
blocker_refs: []
state:
  project_id: proj_123
  state_version: 22
```

### 담당 문서 링크

- 요청 래퍼, 응답 분기, `dry_run` 요약: [API 코어 스키마](schema-core.md).
- `UserJudgment`, 선택지, 맥락, 답변 페이로드: [API 판단 스키마](schema-judgment.md).
- 상태 참조와 요약: [API 상태 스키마](schema-state.md).
- 판단 종류와 활성 값: [API 값 집합](schema-value-sets.md).
- 사용자 소유 판단과 비대체 규칙: [Core 모델](../core-model.md).
- 공개 오류와 저장 효과: [API 오류](errors.md), [저장 효과](../storage-effects.md).

<a id="harnessrecord_user_judgment"></a>

## `harness.record_user_judgment`

### 목적

기존 대기 중인 `UserJudgment` 하나에 대한 사용자의 답을 기록합니다. 이 메서드는 사용자의 답에 따라 특정 대기 판단을 resolved, rejected, deferred, blocked 또는 해당 상태로 표시합니다. 답변을 관련 없는 승인, 범위 확장, 수락, 잔여 위험 수락, 쓰기 승인으로 넓히지 않습니다.

### 필수 입력

- `ToolEnvelope`: `dry_run=false` 커밋에는 `null`이 아닌 `idempotency_key`와 현재 `expected_state_version`이 필요합니다.
- `user_judgment_id`, 일치하는 `judgment_kind`, `selected_option_id`, `answer`, `note`, `accepted_risks`.
- `answer`에는 대기 중인 `judgment_kind`에 맞는 결정별 페이로드 분기만 담아야 합니다. `selected_option_id`와 `note`는 요청 수준에 남습니다.

### 접근 요구사항

`VerifiedSurfaceContext.access_class=core_mutation`과 `verified=true`가 필요합니다. 대기 중인 판단은 요청이 선택한 같은 프로젝트와 호환되는 Task에 속해야 합니다.

### 상태 버전 동작

커밋된 `dry_run=false` 결과는 `project_state.state_version`을 정확히 한 번 올리고 지정된 `user_judgments` 행을 갱신합니다. `dry_run`과 거절은 판단 해결, 차단 사유 갱신, 이벤트, 재실행 행, 상태 버전 증가를 만들지 않습니다.

### 성공 결과

`base.response_kind=result`, `base.effect_kind=core_committed`인 `RecordUserJudgmentResult`를 반환합니다. 결과에는 `user_judgment_ref`, 갱신된 `user_judgment`, `updated_refs`, 현재 `state`, `next_actions`가 들어갑니다.

### 차단 결과

사용자의 답이 그렇거나 초점이 맞는 판단의 호환 결과가 그렇다면 지정된 판단은 `rejected`, `deferred`, `blocked` 또는 차단 사유를 만드는 상태로 커밋될 수 있습니다. 이 결과는 포함된 차단 사유와 판단에 의존하는 요약만 갱신합니다. 해결된 `scope_decision`이라도 활성 범위나 활성 Change Unit 필드를 바꾸려면 여전히 `harness.update_scope`가 필요합니다.

### 거절 결과

오래된 `expected_state_version`, 알 수 없거나 `pending`이 아닌 판단, `judgment_kind` 불일치, 유효하지 않은 선택지, 유효하지 않은 답변 페이로드, 만료되었거나 호환되지 않는 승인, 로컬 접근 실패, validator 실패는 `ToolRejectedResponse`를 반환합니다. 공개 오류 코드 의미와 우선순위는 [API 오류](errors.md)가 담당합니다.

### `dry_run` 동작

`dry_run=true`에서 유효한 미리보기는 `ToolDryRunResponse`를 반환합니다. 판단 해결과 그에 따른 차단 사유 또는 다음 행동 효과를 미리 보여 줄 수 있지만 판단, 차단 사유, 이벤트, 재실행 행, 상태 버전을 실제로 갱신하지 않습니다.

### 저장 효과

커밋 시 `user_judgments`를 갱신하고, 포함된 `blockers`와 판단 의존 요약을 갱신할 수 있으며, `task_events`를 추가하고, `tool_invocations`를 만들고, 프로젝트 상태 시계를 갱신합니다. 현재 MVP에서는 수락된 위험 전용 행을 만들지 않습니다. 저장 효과 의미는 [저장 효과](../storage-effects.md)가 담당합니다.

### 최소 유효 요청

```yaml
method: harness.record_user_judgment
params:
  envelope:
    project_id: proj_123
    task_id: task_456
    actor_kind: user
    surface_id: surface_local
    request_id: req_judgment_answer_001
    idempotency_key: idem_judgment_answer_001
    expected_state_version: 22
    dry_run: false
    locale: ko-KR
  user_judgment_id: uj_001
  judgment_kind: final_acceptance
  selected_option_id: accept
  answer:
    product_decision: null
    technical_decision: null
    scope_decision: null
    sensitive_action_scope: null
    final_acceptance:
      accepted: true
      basis: "문서 전용 API 참조 갱신을 검토했습니다."
    residual_risk_acceptance: null
    cancellation: null
  note: "수락합니다."
  accepted_risks: []
```

### 대표 응답

결과 분기(`RecordUserJudgmentResult`, 커밋됨):

```yaml
base:
  response_kind: result
  effect_kind: core_committed
  dry_run: false
  state_version: 23
  events:
    - event_id: evt_1006
      event_kind: user_judgment_recorded
user_judgment_ref:
  record_kind: user_judgment
  record_id: uj_001
  project_id: proj_123
  task_id: task_456
  state_version: 23
user_judgment:
  judgment_id: uj_001
  project_id: proj_123
  task_id: task_456
  change_unit_id: cu_001
  judgment_kind: final_acceptance
  status: resolved
  presentation: short
  question: "문서 전용 API 참조 재구성을 완료로 수락하시겠습니까?"
  options: []
  context:
    summary: "대응되는 MVP API 메서드 섹션을 재구성했습니다."
    related_refs: []
    artifact_refs: []
    visible_risks: []
    constraints: []
  affected_refs: []
  required_for: close
  resolution:
    selected_option_id: accept
    answer:
      final_acceptance:
        accepted: true
        basis: "문서 전용 API 참조 갱신을 검토했습니다."
    note: "수락합니다."
    accepted_risks: []
    resolved_by_actor_kind: user
  expires_at: null
  created_at: "2026-06-10T12:00:00Z"
  resolved_at: "2026-06-10T12:05:00Z"
updated_refs:
  - record_kind: user_judgment
    record_id: uj_001
    project_id: proj_123
    task_id: task_456
    state_version: 23
state:
  project_id: proj_123
  state_version: 23
next_actions:
  - action: harness.close_task
    reason: "최종 수락 뒤 닫기 준비 상태를 평가한다."
```

### 담당 문서 링크

- 요청 래퍼, 응답 분기, `dry_run` 요약: [API 코어 스키마](schema-core.md).
- `UserJudgment`, `RecordUserJudgmentPayload`, `SensitiveActionScope`, `AcceptedRiskInput`: [API 판단 스키마](schema-judgment.md).
- 상태 참조와 요약: [API 상태 스키마](schema-state.md).
- 판단 값과 활성 메서드 내부 값: [API 값 집합](schema-value-sets.md).
- 사용자 소유 판단, 최종 수락, 잔여 위험 수락, 비대체 규칙: [Core 모델](../core-model.md).
- 공개 오류와 저장 효과: [API 오류](errors.md), [저장 효과](../storage-effects.md).

<a id="harnessclose_task"></a>

## `harness.close_task`

### 목적

활성 Task의 닫기 준비 상태를 평가하고, 선택한 intent가 허용하며 차단 사유가 없을 때 `complete`, `cancel`, `supersede`를 커밋합니다. `harness.close_task`는 닫기 차단 사유를 반환할 수 있습니다. 닫기는 Core 상태 전이이며, 대화, 상태 텍스트, 최종 수락만, 잔여 위험 수락만, 증거만, 렌더링된 보기에서 추론하는 보고서가 아닙니다.

### 필수 입력

- `ToolEnvelope`: `project_id`, `surface_id`, `request_id`, `dry_run`이 필요합니다.
- `task_id`, `intent`, `close_reason`, `superseding_task_id`, `user_note`.
- `intent=complete`, `intent=cancel`, `intent=supersede`와 `dry_run=false`에는 `null`이 아닌 `idempotency_key`와 현재 `expected_state_version`이 필요합니다.
- `intent=check`에서는 `idempotency_key`와 `expected_state_version`이 `null`일 수 있고, `close_reason`은 `null`이어야 합니다.

### 접근 요구사항

`intent=check`는 보호된 닫기 준비 상태 세부정보를 위해 `VerifiedSurfaceContext.access_class=read_status`가 필요합니다. 상태 변경 intent는 `VerifiedSurfaceContext.access_class=core_mutation`, `verified=true`, 호환되는 Task 식별, 유효한 생명주기, 닫기 관련 담당 기록을 요구합니다.

### 상태 버전 동작

`intent=check`는 `dry_run=true`여도 항상 읽기 전용이며 상태를 올리지 않습니다. 상태 변경 intent의 커밋된 종료 닫기 또는 커밋된 차단 닫기는 `project_state.state_version`을 정확히 한 번 올립니다. 닫기 사전 확인 거절, 오래된 `expected_state_version`, stale 닫기 관련 `WriteAuthorization.basis_state_version`, 멱등 요청 해시 충돌, `dry_run` 미리보기는 아무것도 올리지 않습니다.

### 성공 결과

`base.response_kind=result`인 `CloseTaskResult`를 반환합니다. `intent=check`에서는 `base.effect_kind=read_only`이고 `close_state`는 계산된 현재 닫기 상태입니다. 성공한 종료 상태 변경에서는 `base.effect_kind=core_committed`이고 `close_state`는 `closed`, `cancelled`, `superseded` 중 하나입니다.

### 차단 결과

닫기 사전 확인이 성공한 뒤 `intent=complete`는 `blockers: CloseReadinessBlocker[]`를 가진 `CloseTaskResult(close_state=blocked)`를 반환할 수 있습니다. 상태 변경 intent는 메서드 상태 효과 표가 그 커밋된 차단 결과를 허용할 때만 차단 사유 상태 효과를 저장할 수 있습니다. `CloseReadinessBlocker`가 있다는 사실만으로 저장을 뜻하지 않습니다. `STATE_VERSION_CONFLICT`는 절대 `CloseReadinessBlocker.code`가 아닙니다.

### 거절 결과

검증 실패, 로컬 접근 실패, 오래된 `expected_state_version`, stale 닫기 관련 `WriteAuthorization.basis_state_version`, 멱등 요청 해시 충돌, 잘못된 프로젝트 또는 읽을 수 없는 Task 식별, Core 사용 불가, 역량 부족처럼 닫기 준비 상태 평가 전 사전 확인 실패가 있으면 `ToolRejectedResponse`를 반환합니다. 거절 응답은 `CloseTaskResult.blockers`를 반환하지 않고 닫기 효과를 만들지 않습니다.

### `dry_run` 동작

`intent=check`와 `dry_run=true`는 여전히 `base.dry_run=true`, `base.effect_kind=read_only`인 `CloseTaskResult`를 반환합니다. `ToolDryRunResponse`를 반환하면 안 됩니다. `intent=complete`, `intent=cancel`, `intent=supersede`와 `dry_run=true`는 요청이 유효하고 미리보기 가능할 때 `ToolDryRunResponse`를 반환합니다. close `dry_run` 미리보기의 예상 닫기 차단 사유는 `source_kind=close_readiness`인 `DryRunSummary.would_blockers: PlannedBlocker[]`로만 표현하며, `CloseReadinessBlocker`로 표현하지 않습니다.

### 저장 효과

`intent=check`에는 저장 효과가 없습니다. 커밋된 종료 닫기는 `tasks.lifecycle_phase`, `tasks.close_reason`, `tasks.result`, `tasks.closed_at`, 영향을 받는 `change_units`, 차단 사유, 필요한 경우 프로젝트 활성 Task 상태, `task_events`, `tool_invocations`, 프로젝트 상태 시계를 갱신합니다. 커밋된 차단 닫기는 메서드 계약이 허용하는 차단 사유 상태, 이벤트, 재실행 행, 상태 버전 효과만 만들 수 있으며 Task를 열린 상태로 남겨야 합니다. 저장 효과 의미는 [저장 효과](../storage-effects.md)가 담당합니다.

### 최소 유효 요청

```yaml
method: harness.close_task
params:
  envelope:
    project_id: proj_123
    task_id: task_456
    actor_kind: agent
    surface_id: surface_local
    request_id: req_close_check_001
    idempotency_key: null
    expected_state_version: null
    dry_run: false
    locale: ko-KR
  task_id: task_456
  intent: check
  close_reason: null
  superseding_task_id: null
  user_note: null
```

### 대표 응답

차단된 읽기 전용 결과 분기(`CloseTaskResult`, `intent=check`):

```yaml
base:
  response_kind: result
  effect_kind: read_only
  dry_run: false
  state_version: 23
  events: []
close_state: blocked
state:
  project_id: proj_123
  state_version: 23
  task_ref:
    record_kind: task
    record_id: task_456
    project_id: proj_123
    task_id: task_456
    state_version: 23
blockers:
  - category: evidence
    code: EVIDENCE_INSUFFICIENT
    message: "닫기에 필요한 증거가 아직 충분하지 않습니다."
    related_refs: []
evidence_summary:
  status: insufficient
  coverage_items: []
  artifact_refs: []
artifact_refs: []
next_actions:
  - action: harness.record_run
    reason: "완료를 시도하기 전에 증거를 기록한다."
```

### 담당 문서 링크

- 요청 래퍼, 공통 응답 분기, `dry_run` 요약: [API 코어 스키마](schema-core.md).
- 닫기 준비 상태 형태, `CloseReadinessBlocker`, `EvidenceSummary`, `StateSummary`: [API 상태 스키마](schema-state.md).
- 닫기 상태, 생명주기, 닫기 이유, 차단 사유 값: [API 값 집합](schema-value-sets.md).
- complete 닫기 준비 상태 순서와 정직한 닫기: [Core 모델](../core-model.md#close_task).
- 공개 오류와 닫기 차단 사유 경로: [API 오류](errors.md), [`harness.close_task` 닫기 준비 상태 평가와 닫기 차단 사유](errors.md#harnessclose_task-close-blockers).
- 저장 효과와 상태 버전 동작: [저장 효과](../storage-effects.md), [저장소 버전 관리](../storage-versioning.md).
