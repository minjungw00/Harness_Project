# API 판단 스키마

이 문서는 기준 범위의 사용자 소유 판단 API 스키마를 담당합니다. 스키마는 사용자 소유 판단 형태의 API 데이터를 정의하지만 그 자체로 사용자 결정을 기록하지 않습니다.

## 담당하는 것 / 담당하지 않는 것

이 문서가 담당합니다.

- `UserJudgment`
- `UserJudgmentCandidate`
- `UserJudgmentOption`
- `UserJudgmentContext`
- `UserJudgmentResolution`
- `RecordUserJudgmentPayload`
- `SensitiveActionScope`
- `AcceptedRiskInput`
- 사용자 소유 판단의 스키마 필드와 중첩 구조

이 문서는 담당하지 않습니다.

- 사용자 소유 판단의 제품 의미와 비대체 규칙: [Core 모델](../core-model.md)
- 판단 요청 메서드 동작: [사용자 소유 판단 요청 메서드](method-request-user-judgment.md)
- 판단 기록 메서드 동작: [사용자 소유 판단 기록 메서드](method-record-user-judgment.md)
- 지원되는 판단 종류 값, 상태 값, 표시 형식 값, 필요 판단 위치 값: [API 값 집합](schema-value-sets.md)
- 최종 수락이나 잔여 위험 수락의 닫기 효과: [Core 모델](../core-model.md), [Task 닫기 메서드](method-close-task.md)
- 판단 누락, 미해결, 거절, 만료에 대한 공개 오류 의미: [API 오류 코드](error-codes.md)

## 경계

판단 스키마는 사용자가 소유한 선택의 필드 구조를 보존합니다. 제품 판단, 기술 판단, 범위 판단, 민감 동작 승인, 최종 수락, 잔여 위험 수락, 취소 판단, 지원되지 않는 판단 범주의 동작 계약이 아닙니다. 그 의미는 Core와 메서드 담당 문서에 둡니다.

`UserJudgmentCandidate`는 대기 중인 판단이 아닙니다.

`UserJudgment`와 `UserJudgmentCandidate`는 서로 다른 형태입니다. 각 형태가 응답에 나타나는 조건은 메서드 담당 문서가 정의합니다.

`RecordUserJudgmentPayload`는 현재 적용 범위, 증거, `Write Authorization`, 닫기 결과, 넓은 승인에 대한 스키마가 아닙니다.

## `UserJudgment`

```yaml
UserJudgment:
  judgment_id: string
  project_id: string
  task_id: string
  change_unit_id: string | null
  judgment_kind: string
  status: string
  presentation: string
  question: string
  options: UserJudgmentOption[]
  context: UserJudgmentContext
  affected_refs: StateRecordRef[]
  required_for: string
  resolution: UserJudgmentResolution | null
  expires_at: string | null
  created_at: string
  resolved_at: string | null
```

`judgment_kind`, `status`, `presentation`, `required_for` 값은 [판단 값](schema-value-sets.md#judgment-values)이 담당합니다. 제품 의미는 [Core 모델의 사용자 소유 판단](../core-model.md#4-user-owned-judgment)이 담당합니다.

`judgment_id`, `project_id`, `task_id`, `change_unit_id`는 불투명 식별자입니다. `question`은 자유 형식 표시 문자열입니다.

## `UserJudgmentCandidate`

`UserJudgmentCandidate`는 제안된 집중 질문의 후보 형태입니다. `judgment_id`, `status`, `resolution`, `created_at`, `resolved_at` 필드가 없습니다.

```yaml
UserJudgmentCandidate:
  judgment_kind: string
  presentation: string
  question: string
  options: UserJudgmentOption[]
  context: UserJudgmentContext
  affected_refs: StateRecordRef[]
  required_for: string
  expires_at: string | null
```

## 선택지와 맥락 형태

```yaml
UserJudgmentOption:
  option_id: string
  label: string
  description: string
  consequence: string
  is_default: boolean

UserJudgmentContext:
  summary: string
  related_refs: StateRecordRef[]
  artifact_refs: ArtifactRef[]
  visible_risks: AcceptedRiskInput[]
  constraints: string[]
```

`option_id`는 그 판단 안에서만 유효합니다. `label`, `description`, `consequence`, `summary`, `constraints` 항목은 자유 형식 표시 문자열입니다. 화면에 보이는 라벨은 표시 텍스트이며 기준 스키마 값이 아닙니다.

## 해결과 답변 요청 본문

```yaml
UserJudgmentResolution:
  selected_option_id: string
  answer: RecordUserJudgmentPayload
  note: string | null
  accepted_risks: AcceptedRiskInput[]
  resolved_by_actor_kind: string

RecordUserJudgmentPayload:
  product_decision: object | null
  technical_decision: object | null
  scope_decision: object | null
  sensitive_action_scope: SensitiveActionScope | null
  final_acceptance: object | null
  residual_risk_acceptance: object | null
  cancellation: object | null
```

`selected_option_id`와 `note`는 요청 수준이자 해결 수준의 필드입니다. `selected_option_id`는 판단 선택지 집합 안에서만 유효합니다. `note`는 자유 형식 표시 문자열입니다.

`resolved_by_actor_kind`는 `ToolEnvelope.actor_kind`와 같은 제어 값 집합을 사용합니다. [행위자 값](schema-value-sets.md#actor-values)을 보세요.

형태 규칙:
- 선택된 `judgment_kind`에 맞는 판단별 요청 본문 분기 하나만 채웁니다.

담당 문서 예외:
- 메서드 담당 문서가 더 좁은 요청 본문 구조를 명시적으로 정의할 수 있습니다.

판단별 요청 본문 객체 안의 문자열 필드는 메서드 담당 문서가 더 좁은 로컬 코드 목록이나 값 목록을 명시적으로 정의하지 않는 한 그 요청 본문 구조 안에서만 유효합니다. 전역 API 값 집합이 아닙니다.

허용되지 않는 것:
- `RecordUserJudgmentPayload`에는 `selected_option_id`나 `note`가 없습니다.

## `SensitiveActionScope`

`SensitiveActionScope`는 이름 붙은 민감 동작 승인 맥락의 스키마 형태입니다. `AuthorizedAttemptScope`도 아니고, `Write Authorization`도 아니며, 보안 권한도 아닙니다. [보안](../security.md)을 확인하세요.

```yaml
SensitiveActionScope:
  action_kind: string
  description: string
  intended_paths: string[]
  sensitive_categories: string[]
  command_or_tool_summary: string | null
  network_or_host_summary: string | null
  secret_or_credential_summary: string | null
  capability_claim: string
  expires_at: string | null
```

`SensitiveActionScope`의 존재는 민감 동작 승인이 필요한 위치를 정의하지 않습니다. 이 형태가 나타나는 위치는 메서드 담당 문서가 정의하며, 제품 파일 쓰기에 대한 `harness.prepare_write` 경로를 대신하지 않습니다.

`SensitiveActionScope.action_kind`와 `sensitive_categories[]`는 영향받는 메서드나 프로필 담당 문서가 더 좁은 로컬 목록을 공개하지 않는 한 불투명 민감 동작 분류 문자열입니다. `description`, `command_or_tool_summary`, `network_or_host_summary`, `secret_or_credential_summary`, `capability_claim`은 표시 또는 주장 문자열입니다. 기준 값 집합이나 보안 권한이 아닙니다.

## `AcceptedRiskInput`

`AcceptedRiskInput`은 판단 요청 본문 안에서 보이는 잔여 위험의 이름을 담는 형태입니다.

```yaml
AcceptedRiskInput:
  risk_id: string | null
  summary: string
  consequence: string
  related_refs: StateRecordRef[]
  accepted_for_close: boolean
```

이 형태는 검증, 증거 충분성, QA, 최종 수락, 결과에 위험이 없다는 증명이 아닙니다. 잔여 위험의 의미는 [Core 모델](../core-model.md)이 담당합니다.

`risk_id`는 값이 있을 때 불투명 위험 식별자입니다. `summary`와 `consequence`는 자유 형식 표시 문자열입니다.

## 관련 담당 문서

- [Core 모델](../core-model.md): 사용자 소유 판단 의미와 비대체 규칙.
- [사용자 소유 판단 요청 메서드](method-request-user-judgment.md): `harness.request_user_judgment`.
- [사용자 소유 판단 기록 메서드](method-record-user-judgment.md): `harness.record_user_judgment`.
- [API 값 집합](schema-value-sets.md): `judgment_kind`, `presentation`, `required_for`, 상태, 행위자 값, 선택지 표시 경계.
- [API 상태 스키마](schema-state.md): `StateRecordRef`.
- [API 아티팩트 스키마](schema-artifacts.md): `ArtifactRef`.
- [범위 참조](../scope.md): 예약된 판단 경로와 기준 범위 경계 확인.
