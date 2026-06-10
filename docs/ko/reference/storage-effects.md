# 저장 효과

이 문서는 현재 MVP 원천 설계의 메서드별 저장 효과 의미를 담당합니다. 문서 원천 자료일 뿐이며 하네스 런타임 절차를 실행하거나 모의 실행하지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- 읽기 전용, dry-run, 거절, 스테이징 생성, Core 커밋, 커밋된 차단 결과의 저장 효과 구분.
- 메서드 분기가 재실행 행, `task_events`, 기록 변경, 상태 버전 증가, 스테이징 핸들 소비, 아티팩트 승격, Write Authorization 변경을 만드는지 여부.
- 차단 사유형 응답 데이터의 지속 저장 경계.
- 거절 분기와 유효한 dry-run 미리보기 분기의 효과 없음 보장.

이 문서는 담당하지 않습니다.

- 기록 배치나 DDL: [저장소 기록](storage-records.md)
- 아티팩트 생명주기 세부사항: [아티팩트 저장소](storage-artifacts.md)
- 멱등성, 잠금, 상태 버전 시계, 이벤트 순서, 마이그레이션: [저장소 버전 관리](storage-versioning.md)
- 공개 응답 분기나 스키마: [API 코어 스키마](api/schema-core.md)
- API 메서드 동작: [MVP API](api/mvp-api.md)
- 공개 오류 코드 우선순위: [API 오류](api/errors.md)

## 형태와 효과의 구분

응답 데이터 형태와 저장 효과는 별개입니다. `CloseReadinessBlocker`, `WriteDecisionReason`, `PlannedBlocker`, `ArtifactRef`, `StagedArtifactHandle`은 API 데이터 형태입니다. 응답에 이 값들이 있다는 사실만으로 지속 저장, 아티팩트 승격, 스테이징 핸들 소비, 재실행 저장, 닫기 상태 변경, `project_state.state_version` 증가가 증명되지 않습니다.

효과는 선택된 메서드 동작과 응답 분기가 정합니다.

| 분기 | 저장 효과 |
|---|---|
| 읽기 전용 `MethodResult` | 응답에만 남습니다. 재실행 행, 이벤트, 현재 행 변경, 아티팩트 효과, Write Authorization 효과, 상태 버전 증가가 없습니다. |
| `ToolRejectedResponse` | 효과 없음입니다. 현재 행, 재실행 행, 이벤트, 아티팩트 효과, Write Authorization 생성/소비, 상태 버전 증가가 없습니다. |
| 유효한 `ToolDryRunResponse` | 미리보기 전용입니다. 현재 행, 생성된 지속 참조, 재실행 행, 이벤트, 스테이징 핸들, 아티팩트 승격/연결, 상태 버전 증가가 없습니다. |
| `effect_kind=staging_created`인 `StageArtifactResult` | 저장소 소유 임시 스테이징만 만듭니다. Core 현재 행, 재실행 행, 이벤트, 지속 `ArtifactRef`, 상태 버전 증가가 없습니다. |
| Core 커밋 `MethodResult` | 메서드 담당 문서가 허용한 현재 행 변경, `task_events` 추가, 재실행 행 생성, `project_state.state_version` 정확히 한 번 증가를 만들 수 있습니다. |
| 커밋된 차단 `MethodResult` | 메서드 담당 문서가 명시적으로 허용한 차단 사유 상태, 이벤트, 재실행 행, 상태 버전 효과만 만들 수 있습니다. 부족하다고 보고한 권한을 만들면 안 됩니다. |

## 효과 없음 분기

잘못된 요청, 커밋 전 검증 실패, 보호된 동작이 진행되기 전의 로컬 접근 실패, 역량 실패, 오래된 `expected_state_version`, 오래된 `WriteAuthorization.basis_state_version`, 멱등 요청 해시 충돌, 거절된 아티팩트 입력은 효과 없음 분기를 반환합니다. 현재 행을 만들거나, `task_events`를 추가하거나, `tool_invocations.response_json`을 쓰거나, 재실행 행을 만들거나, 증거 요약을 갱신하거나, 닫기 상태를 바꾸거나, Write Authorization을 만들거나 소비하거나, `artifact_staging.status`를 바꾸거나, `consumed_by_run_id` 또는 `promoted_artifact_id`를 설정하거나, 아티팩트를 승격/연결하거나, `project_state.state_version`을 올리면 안 됩니다.

유효한 dry-run 미리보기는 `DryRunSummary.would_blockers: PlannedBlocker[]` 또는 계획된 효과를 포함할 수 있습니다. 이 미리보기 항목은 `task_event` 없음, `task_events` 추가 없음, 재실행 행 없음, `tool_invocations.response_json` 없음, `close_state` 변경 없음, Write Authorization 변경 없음, 스테이징 핸들 생성 또는 소비 없음, 아티팩트 효과 없음, 증거 업데이트 없음, `CloseReadinessBlocker` 저장 없음, `project_state.state_version` 증가 없음입니다.

## 읽기 전용 효과

읽기 전용 결과는 응답에만 남으며 재실행 행이 아닙니다. `harness.status`와 `harness.close_task intent=check`는 응답을 위해 차단 사유, `CloseReadinessBlocker[]`, 증거 요약, 아티팩트 참조, 진단, 다음 행동을 계산할 수 있습니다. 하지만 읽기가 일어났다는 이유만으로 그 계산값을 저장하면 안 됩니다.

`harness.status`가 `close_blockers: CloseReadinessBlocker[]`를 반환하는 경우도 읽기 전용 관찰입니다. `task_event` 없음, `task_events` 추가 없음, 재실행 행 없음, `tool_invocations.response_json` 없음, `close_state` 변경 없음, Write Authorization 변경 없음, 스테이징 핸들 소비 없음, 아티팩트 효과 없음, 증거 업데이트 없음, `project_state.state_version` 증가 없음입니다.

`harness.close_task intent=check`는 `base.effect_kind=read_only`인 `CloseTaskResult`를 반환합니다. 같은 선택 동작에 `dry_run=true`가 있어도 응답은 `base.dry_run=true`, `base.effect_kind=read_only`인 `CloseTaskResult`로 유지됩니다. `ToolDryRunResponse`가 아닙니다. `blockers: CloseReadinessBlocker[]`를 포함하더라도 두 형태 모두 읽기 전용입니다.

## 커밋된 차단 효과

커밋된 차단 결과와 거절 응답은 다릅니다. `harness.prepare_write` 또는 `harness.close_task`의 커밋된 차단 결과는 [MVP API](api/mvp-api.md)가 차단 커밋을 허용할 때만 `MethodResult`입니다.

`decision=blocked`, `decision=approval_required`, `decision=decision_required`인 커밋된 `dry_run=false` `PrepareWriteResult`는 메서드 상태 효과 계약이 그 판단 커밋을 허용할 때 응답과 재실행 페이로드에 `write_decision_reasons: WriteDecisionReason[]`를 담을 수 있습니다. 이 사유는 `prepare_write` 판단 사유이지 닫기 준비 상태 차단 사유도 아니고 `CloseReadinessBlocker[]`도 아니며 닫기 준비 상태 차단 사유 기록도 아닙니다. 이 분기는 소비 가능한 Write Authorization을 만들지 않고, `close_state`를 바꾸지 않고, 닫기 준비 상태 평가를 실행하지 않고, `CloseReadinessBlocker` 저장을 만들지 않으며, 증거를 갱신하거나, 아티팩트를 바꾸거나, 스테이징 핸들을 소비하거나, `close_task` 효과를 수행하면 안 됩니다.

`CloseTaskResult(close_state=blocked)`는 닫기 준비 상태 평가가 실행되었고 `harness.close_task` 메서드 계약이 차단 결과 커밋을 허용할 때만 저장 효과가 있습니다. `blockers: CloseReadinessBlocker[]`를 포함할 수 있고, API/저장소 계약이 명시적으로 허용한 차단 사유 상태, `task_events`, 재실행 행, `project_state.state_version` 효과만 만들 수 있습니다. Task는 열린 상태로 남습니다. `STATE_VERSION_CONFLICT`에는 이 분기를 사용하면 안 됩니다. 그 코드는 사전 확인의 `ToolRejectedResponse` 분기에 속하며 재실행으로 저장하지 않습니다.

## 메서드별 효과

아래 표는 지속 저장 효과를 요약합니다. 메서드 동작과 응답 공용체는 [MVP API](api/mvp-api.md)가 계속 담당합니다.

| 메서드 또는 선택된 intent | 커밋된 `dry_run=false` 효과 | 읽기 전용, dry-run, 거절 경계 |
|---|---|---|
| `harness.intake` | Task, 선택적 Change Unit, 구체화 기록, 이벤트, 재실행 행, `project_state.state_version` 한 번 증가를 만들 수 있습니다. | 유효한 `dry_run=true`는 `ToolDryRunResponse`를 반환하며 Task, 참조, 이벤트, 재실행 행, 상태 버전 증가를 만들지 않습니다. 거절은 효과 없음입니다. |
| `harness.update_scope` | 활성 Task 범위 필드를 갱신하고, 활성 `change_units`를 만들거나 교체하고, 메서드 담당 문서가 허용한 차단 사유 또는 stale Write Authorization 참조를 갱신하고, 이벤트와 재실행 행을 만들며 상태를 한 번 올릴 수 있습니다. | 유효한 dry-run은 범위, Change Unit, 차단 사유, stale 승인 효과만 미리 설명합니다. 거절은 효과 없음입니다. |
| `harness.status` | 없음. 읽기 전용 응답입니다. | `dry_run=true`도 `effect_kind=read_only`인 `StatusResult`로 유지되며 `ToolDryRunResponse`가 아닙니다. 재실행 행이나 변경이 없습니다. |
| `harness.prepare_write` | `decision=allowed`는 호환되는 활성 Write Authorization을 만들거나 반환하고, 이벤트와 재실행 행을 만들며 상태를 한 번 올릴 수 있습니다. 커밋된 비허용 판단은 허용된 판단 상태/재실행 효과만 지속할 수 있습니다. | 거절과 유효한 dry-run 분기는 재실행 행, Write Authorization, 이벤트, 닫기 상태 변경, 아티팩트/증거 효과, 상태 버전 증가를 만들지 않습니다. |
| `harness.stage_artifact` | 성공한 스테이징은 `artifact_staging` 또는 동등한 저장소 소유 스테이징 기록과 `artifacts/tmp/` 아래 임시 안전 바이트 또는 알림만 만듭니다. | 유효한 `dry_run=true`는 바이트, 스테이징 기록, `StagedArtifactHandle`, 재실행 행, 상태 버전 증가를 만들지 않습니다. 잘못된 스테이징 요청은 효과 없음 거절을 제외하고 Core/저장소 변경을 만들지 않습니다. |
| `harness.record_run` | `runs` 생성, 호환되는 `write_authorizations` 소비, 적격 `artifact_staging` 소비, `artifacts` 승격/연결, `evidence_summaries` 또는 허용된 `blockers` 갱신, 이벤트 추가, 재실행 행 생성, 상태 한 번 증가를 만들 수 있습니다. | 유효한 dry-run은 `run_summary`, 지속 아티팩트, 아티팩트 연결, 증거 갱신, 차단 사유 갱신, 이벤트, 재실행 행, 스테이징 핸들 소비, Write Authorization 소비, 상태 버전 증가를 만들지 않습니다. 거절된 시도는 스테이징 행이나 아티팩트를 바꾸지 않습니다. |
| `harness.request_user_judgment` | 대기 중인 `user_judgments` 행 생성, 영향받은 차단 사유 갱신, 이벤트 추가, 재실행 행 생성, 상태 한 번 증가를 만들 수 있습니다. | 유효한 dry-run은 실제 `user_judgment_ref`, 대기 중인 판단, 차단 사유 갱신, 이벤트, 재실행 행, 상태 버전 증가를 만들지 않습니다. |
| `harness.record_user_judgment` | `user_judgments` 행 해결, 종속 차단 사유 또는 다음 행동 갱신, 이벤트 추가, 재실행 행 생성, 상태 한 번 증가를 만들 수 있습니다. | 유효한 dry-run은 판단 해결, 차단 사유 갱신, 이벤트, 재실행 행, 상태 버전 증가를 만들지 않습니다. |
| `harness.close_task intent=check` | 없음. 닫기 준비 상태를 계산하는 읽기 전용 응답입니다. | `dry_run=true`도 `effect_kind=read_only`인 `CloseTaskResult`로 유지됩니다. 재실행 행, 이벤트, 차단 사유 행, 닫기 상태 변경, 아티팩트/증거 효과, 상태 버전 증가가 없습니다. |
| `harness.close_task intent=complete` | 차단 사유가 없으면 Task를 닫거나, Task를 열린 상태로 둔 채 허용된 차단 complete 효과를 커밋할 수 있습니다. 커밋 시 이벤트, 재실행 행, 상태 한 번 증가를 만듭니다. | 유효한 `dry_run=true`는 `ToolDryRunResponse`를 반환합니다. 사전 확인 실패는 효과 없음 `ToolRejectedResponse`입니다. |
| `harness.close_task intent=cancel` | Task를 취소하거나, Task를 열린 상태로 둔 채 cancellation 자체를 무효화하는 차단 사유를 커밋할 수 있습니다. 커밋 시 이벤트, 재실행 행, 상태 한 번 증가를 만듭니다. | 유효한 `dry_run=true`는 `ToolDryRunResponse`를 반환합니다. 사전 확인 실패는 효과 없음입니다. 취소는 증거 충분성이 아닙니다. |
| `harness.close_task intent=supersede` | Task를 대체하고 같은 변경에서 `project_state.active_task_id`를 갱신하거나, supersession 자체를 무효화하는 차단 사유를 커밋할 수 있습니다. 커밋 시 이벤트, 재실행 행, 상태 한 번 증가를 만듭니다. | 유효한 `dry_run=true`는 `ToolDryRunResponse`를 반환합니다. 사전 확인 실패는 효과 없음입니다. 대체는 증거 충분성이 아닙니다. |

## 관련 담당 문서

- [MVP API](api/mvp-api.md): 선택된 메서드 동작과 응답 공용체.
- [API 오류](api/errors.md): 거절 응답의 공개 오류.
- [저장소 기록](storage-records.md): 효과가 건드릴 수 있는 기록.
- [아티팩트 저장소](storage-artifacts.md): 스테이징 핸들과 아티팩트 생명주기 세부사항.
- [저장소 버전 관리](storage-versioning.md): 상태 시계와 재실행/멱등성 의미.
