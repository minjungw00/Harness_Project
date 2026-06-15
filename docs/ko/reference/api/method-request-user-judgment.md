<a id="harnessrequest_user_judgment"></a>

# `harness.request_user_judgment` 참조

## 담당하는 것

이 문서는 기준 범위의 `harness.request_user_judgment` 메서드 동작을 담당합니다.

- 메서드별 필수 입력, 접근 요구사항, 상태 버전 동작, 결과 분기, `dry_run` 동작
- 초점이 분명한 사용자 소유 판단 하나에 대해 대기 중인 `UserJudgment`를 만드는 동작
- 계정 데이터 내보내기 확인 시나리오의 최소 요청과 대표 응답

## 담당하지 않는 것

이 문서는 아래 항목을 담당하지 않습니다.

- `ToolEnvelope`, `ToolResultBase`, `ToolRejectedResponse`, `ToolDryRunResponse`의 공통 스키마 본문
- `UserJudgment`, 선택지, 맥락, 답변 요청 본문, 값 집합, 상태 필드 정의
- Core의 사용자 소유 판단 의미, 최종 수락 의미, 잔여 위험 의미, 민감 동작 승인 의미, `Write Authorization` 의미
- 저장 기록 레이아웃, 정확한 저장 효과, 공개 오류 코드 의미, 공개 오류 우선순위

## 목적

`harness.request_user_judgment`는 초점이 분명한 사용자 소유 판단 하나에 대해 대기 중인 `UserJudgment`를 만듭니다. 이 메서드는 사용자에게 묻는 경로입니다. 에이전트는 사용자를 대신해 답하거나, 추론하거나, 판단 범위를 넓히거나, 결정해서는 안 됩니다.

대기 중인 판단은 결정을 요청하는 기록입니다. 결정 자체가 아니며, 증거를 만들거나, 현재 적용 범위를 바꾸거나, `Write Authorization`을 만들거나, `Task`를 닫지 않습니다.

## 필수 입력

- `ToolEnvelope`: `dry_run=false` 커밋에는 `null`이 아닌 `idempotency_key`와 현재 `expected_state_version`이 필요합니다.
- `task_id`, `change_unit_id`, `judgment_kind`, `presentation`, `question`, `options`, `context`, `affected_refs`, `required_for`, `expires_at`.
- 서로 이해할 수 있는 `options`를 가진 초점이 분명한 `question`.
- 사용자가 숨은 대화 상태에 기대지 않고 정확한 사안을 판단할 수 있는 충분한 `context`.

`UserJudgment`, `UserJudgmentOption`, `UserJudgmentContext`, 참조의 공통 필드 형태는 [API 판단 스키마](schema-judgment.md)와 [API 상태 스키마](schema-state.md)가 담당합니다.

## 접근 요구사항

`VerifiedSurfaceContext.access_class=core_mutation`과 `verified=true`가 필요합니다.

요청은 같은 프로젝트의 호환되는 `Task`와 선택적 Change Unit을 대상으로 해야 합니다. 로컬 접근 실패, 읽을 수 없는 프로젝트나 `Task` 식별자, 부족한 로컬 역량은 커밋 전에 거절됩니다.

## 상태 버전 동작

커밋된 `dry_run=false` 결과:

- `project_state.state_version`을 정확히 한 번 올립니다.
- 대기 중인 `UserJudgment` 하나를 만듭니다.
- 저장 효과 담당 문서가 허용하는 경우에만 영향받은 차단 사유 상태를 갱신할 수 있습니다.

비주장:

- 다른 메서드가 반환한 `UserJudgmentCandidate`는 `harness.request_user_judgment`가 커밋하기 전까지 지속 판단이 아닙니다.
- `dry_run`과 거절은 대기 중인 판단, 차단 사유 갱신, 이벤트, 재실행 행, 상태 버전 증가를 만들지 않습니다.

## 성공 결과

아래 값을 담은 `RequestUserJudgmentResult`를 반환합니다.

- `base.response_kind=result`
- `base.effect_kind=core_committed`
- `user_judgment_ref`
- 대기 중인 `user_judgment`
- 영향받은 `blocker_refs`
- 현재 `state`

## 차단 결과

이 메서드에는 별도의 커밋된 차단 응답 분기가 없습니다.

대기 중인 판단을 만들 수 없으면 메서드는 커밋 전에 거절합니다. 일반적인 원인은 유효하지 않은 요청 형태, 지원되지 않거나 호환되지 않는 `judgment_kind`, 없거나 호환되지 않는 `Task` 식별자, 미해결 선행 판단, 로컬 접근 실패, 부족한 역량, 오래된 `expected_state_version`, 검증기 실패입니다.

## 거절 결과

커밋 전 실패에는 `ToolRejectedResponse`를 반환합니다. 공개 오류 코드 의미는 [API 오류 코드](error-codes.md)가 담당합니다. 공개 오류 우선순위는 [API 오류 우선순위](error-precedence.md)가 담당합니다.

거절된 시도는 대기 중인 판단을 만들지 않으며, 요청처럼 보이는 차단 사유 데이터를 부수 효과로 지속하지 않습니다.

## `dry_run` 동작

`dry_run=true`에서 유효한 미리보기는 `ToolDryRunResponse`를 반환합니다. 분기 형태는 [API 코어 스키마](schema-core.md)가 담당하고, 저장 효과 없음 의미는 [저장 효과](../storage-effects.md)가 담당합니다.

미리보기는 지속되는 `user_judgment_ref`를 반환하거나 대기 중인 `UserJudgment`를 만들면 안 됩니다.

## 저장 효과

커밋 시 대기 중인 `user_judgments` 행과 관련 차단 사유 상태를 지속할 수 있습니다. 정확한 저장 효과는 [저장 효과](../storage-effects.md#harnessrequest_user_judgment)가 담당합니다.

## 예시

예시 전제:

- `proj_123`, `task_456`, `cu_001`은 같은 프로젝트에 이미 있습니다.
- 현재 프로젝트 `state_version`은 `21`입니다.
- 계정 데이터 내보내기 확인 문구는 사용자 소유 제품 판단을 받을 준비가 되어 있습니다.

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
  judgment_kind: product_decision
  presentation: short
  question: "계정 데이터 내보내기 파일에 개인정보가 포함될 수 있음을 알리는 확인 문구를 충분한 것으로 수락해도 됩니까?"
  options:
    - option_id: accept
      label: "충분함"
      description: "확인 문구가 충분하다는 사용자 소유 판단을 기록합니다."
      consequence: "닫기 준비 상태가 제품 판단을 해결된 것으로 평가할 수 있습니다."
      is_default: true
    - option_id: revise
      label: "수정 필요"
      description: "확인 문구를 더 수정해야 하므로 작업을 열어 둡니다."
      consequence: "제품 판단이 남아 있어 닫기가 계속 차단됩니다."
      is_default: false
  context:
    summary: "다운로드 전에 표시되는 확인 문구는 계정 데이터 내보내기 파일에 개인정보가 포함될 수 있음을 알립니다."
    related_refs: []
    artifact_refs: []
    visible_risks: []
    constraints:
      - "계정 데이터 내보내기 흐름과 계정 데이터 내보내기 확인 테스트는 범위 안에 있고, 계정 삭제 동작은 범위 밖입니다."
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
  judgment_kind: product_decision
  status: pending
  presentation: short
  question: "계정 데이터 내보내기 파일에 개인정보가 포함될 수 있음을 알리는 확인 문구를 충분한 것으로 수락해도 됩니까?"
  options:
    - option_id: accept
      label: "충분함"
      description: "확인 문구가 충분하다는 사용자 소유 판단을 기록합니다."
      consequence: "닫기 준비 상태가 제품 판단을 해결된 것으로 평가할 수 있습니다."
      is_default: true
    - option_id: revise
      label: "수정 필요"
      description: "확인 문구를 더 수정해야 하므로 작업을 열어 둡니다."
      consequence: "제품 판단이 남아 있어 닫기가 계속 차단됩니다."
      is_default: false
  context:
    summary: "다운로드 전에 표시되는 확인 문구는 계정 데이터 내보내기 파일에 개인정보가 포함될 수 있음을 알립니다."
    related_refs: []
    artifact_refs: []
    visible_risks: []
    constraints:
      - "계정 데이터 내보내기 흐름과 계정 데이터 내보내기 확인 테스트는 범위 안에 있고, 계정 삭제 동작은 범위 밖입니다."
  affected_refs:
    - record_kind: task
      record_id: task_456
      project_id: proj_123
      task_id: task_456
      state_version: 21
  required_for: close
  resolution: null
  expires_at: null
  created_at: "<example-created-at>"
  resolved_at: null
blocker_refs: []
state:
  project_id: proj_123
  state_version: 22
```

## 담당 문서 링크

- 요청 래퍼, 응답 분기, `dry_run` 요약: [API 코어 스키마](schema-core.md).
- `UserJudgment`, 선택지, 맥락, 답변 요청 본문: [API 판단 스키마](schema-judgment.md).
- 상태 참조와 요약: [API 상태 스키마](schema-state.md).
- 판단 종류와 지원 값: [API 값 집합](schema-value-sets.md).
- 사용자 소유 판단과 비대체 규칙: [Core 모델](../core-model.md).
- 정확한 저장 효과: [저장 효과](../storage-effects.md#harnessrequest_user_judgment).
- 공개 오류: [API 오류 코드](error-codes.md)와 [API 오류 우선순위](error-precedence.md).
- 대기 중인 판단에 대한 사용자 답변 기록: [`harness.record_user_judgment`](method-record-user-judgment.md).
