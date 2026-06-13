# API 메서드

## 담당하는 것

이 문서는 현재 API 메서드 묶음의 안정적인 경로 문서입니다. 담당하는 것은 아래와 같습니다.

- 활성 공개 API 메서드 목록
- 메서드 담당 문서 경로

## 담당하지 않는 것

이 문서는 아래 항목을 담당하지 않습니다.

- 각 메서드 동작의 전체 세부사항. 여기에는 메서드별 필수 입력, 접근 요구사항, 결과 필드, `dry_run` 동작, 대표 요청과 응답 본문이 포함됩니다.
- 공통 API 요청 래퍼 본문, 응답 분기 스키마 본문, 스키마 필드 정의
- 상태, 아티팩트, 사용자 판단, 값 집합, 오류 스키마 정의
- API 예시 정합성 규칙 또는 필드 이름 일관성 규칙
- 저장 효과 세부사항, 저장 DDL, 저장 기록 레이아웃, 아티팩트 생명주기, 상태 버전 저장 규칙, 보안 보장
- 공개 오류 코드 의미
- 범위 밖 API 메서드

## 지원 메서드 경계

이 문서에 나열된 메서드만 지원되는 공개 API 메서드입니다. 여기에 없는 메서드 이름은 지원되는 공개 메서드 묶음 밖에 있습니다.

메서드별 동작은 각 메서드 담당 문서가 담당합니다. 범위 밖 API나 스키마 기능은 [범위](../scope.md)와 영향받는 담당 문서가 활성 동작으로 정의하지 않는 한 이 메서드 경로에 포함되지 않습니다.

<a id="baseline-scope-method-behavior"></a>

## 현재 API 메서드 목록

이 문서는 지원되는 공개 API 메서드 목록과 각 메서드의 동작 담당 문서 경로를 맡습니다. 스키마 형태, 저장 효과, 공개 오류는 각 담당 문서에 남고 관련 메서드 담당 문서에서 연결합니다.

<a id="harnessintake"></a>
<a id="harnessupdate_scope"></a>
<a id="harnessstatus"></a>
<a id="harnessprepare_write"></a>
<a id="harnessstage_artifact"></a>
<a id="harnessrecord_run"></a>
<a id="harnessrequest_user_judgment"></a>
<a id="harnessrecord_user_judgment"></a>
<a id="harnessclose_task"></a>

| 메서드 | 담당 문서 |
|---|---|
| `harness.intake` | [접수 메서드 담당 문서](method-intake.md) |
| `harness.update_scope` | [범위 갱신 메서드 담당 문서](method-update-scope.md) |
| `harness.status` | [상태 메서드 담당 문서](method-status.md) |
| `harness.prepare_write` | [쓰기 준비 메서드 담당 문서](method-prepare-write.md) |
| `harness.stage_artifact` | [아티팩트 스테이징 메서드 담당 문서](method-stage-artifact.md) |
| `harness.record_run` | [실행 기록 메서드 담당 문서](method-record-run.md) |
| `harness.request_user_judgment` | [사용자 판단 메서드 담당 문서](method-user-judgment.md#harnessrequest_user_judgment) |
| `harness.record_user_judgment` | [사용자 판단 메서드 담당 문서](method-user-judgment.md#harnessrecord_user_judgment) |
| `harness.close_task` | [Task 닫기 메서드 담당 문서](method-close-task.md) |

<a id="method-owner-routing-table"></a>

## 메서드 담당 문서 경로

아래 표는 지원되는 메서드의 동작 질문을 담당 문서로 보냅니다.

| 메서드 동작 질문 | 담당 문서 |
|---|---|
| `harness.intake` | [접수 메서드 담당 문서](method-intake.md) |
| `harness.update_scope` | [범위 갱신 메서드 담당 문서](method-update-scope.md) |
| `harness.status` | [상태 메서드 담당 문서](method-status.md) |
| `harness.prepare_write` | [쓰기 준비 메서드 담당 문서](method-prepare-write.md) |
| `harness.stage_artifact` | [아티팩트 스테이징 메서드 담당 문서](method-stage-artifact.md) |
| `harness.record_run` | [실행 기록 메서드 담당 문서](method-record-run.md) |
| `harness.request_user_judgment`와 `harness.record_user_judgment` | [사용자 판단 메서드 담당 문서](method-user-judgment.md) |
| `harness.close_task` | [Task 닫기 메서드 담당 문서](method-close-task.md) |
