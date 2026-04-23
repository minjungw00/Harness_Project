# 하네스 Conformance Tests v04

## 1. 문서 목적

이 문서는 하네스 구현체가 core invariant를 실제로 지키는지 확인하는 최소 conformance suite를 정의한다.

이 문서는 다음을 다룬다.

- scenario 구조
- initial state 형식
- MCP call sequence
- expected state
- expected events
- expected projection
- expected artifacts
- blocked reasons

이 문서는 특정 테스트 프레임워크를 강제하지 않는다. 참조 구현은 이 문서와 `conformance-fixtures.yaml`을 기반으로 자동 테스트를 제공한다.

## 2. Conformance 원칙

Conformance는 문구가 아니라 동작을 검증한다.

구현체는 다음 기준을 만족해야 한다.

1. 상태 변경은 MCP tool 또는 명시적 operator action으로만 발생한다.
2. 제품 파일 쓰기 전 scope와 approval을 확인한다.
3. 민감 범주는 approval 없이 진행하지 않는다.
4. 변경 후 durable evidence를 남긴다.
5. work는 실행자의 자기 보고만으로 닫지 않는다.
6. same-session self-review는 detached verification으로 승격하지 않는다.
7. state와 projection freshness를 분리한다.
8. 사람이 managed 영역을 수정하면 reconcile item을 만든다.

## 3. Scenario 형식

각 scenario는 다음 필드를 가진다.

```yaml
id: CF-001
title: string
initial:
  project: object
  task: optional object
  files: object
steps:
  - call: harness.status
    input: object
    expect: object
expected:
  final_state: object
  events: []
  projections: []
  artifacts: []
  validators: []
  blocked_reasons: []
```

## 4. 공통 assertion

모든 state-changing scenario는 다음을 검증한다.

- `state_version`이 증가한다.
- `task_events`에 event가 append된다.
- `projection_version`이 증가한다.
- projection job이 enqueue된다.
- idempotency key 재호출이 같은 result를 반환한다.
- expected_state_version mismatch는 `STATE_CONFLICT`를 반환한다.

## 5. Scenario 목록

### CF-001 Active Task 없음 상태 조회

초기 상태에 active Task가 없다.

Call:

```yaml
call: harness.status
input:
  detail: compact
```

기대:

- `ok=true`
- compact status card 반환
- state mutation 없음
- projection job 없음

### CF-002 Advisor 종료

사용자가 설명 또는 비교 요청을 한다.

Steps:

1. `harness.intake` with read-only request
2. advisor response 기록

기대:

- `mode=advisor`
- `phase=executing` 또는 advisor 완료 후 `completed`
- 제품 파일 변경 없음
- `result=advice_only` 가능
- approval 없음

### CF-003 Direct 성공

작은 오타 수정 요청을 direct로 처리한다.

Steps:

1. `harness.intake`
2. `harness.prepare_write`
3. 파일 변경 simulation
4. `harness.record_change`
5. `harness.finish_direct`

기대:

- `mode=direct`
- final `phase=completed`
- `result=passed`
- `assurance_level=self_checked`
- DIRECT-RESULT projection job
- changed_paths validator passed

### CF-004 Direct에서 Work 전환

Direct로 시작했으나 변경 범위가 커진다.

Steps:

1. `harness.intake` small request
2. `harness.prepare_write`
3. 에이전트가 비국소 변경 필요를 발견
4. `harness.finish_direct` with `escalated_to_work=true`

기대:

- 같은 `task_id` 유지
- `mode=work`
- `phase=shaping`
- direct result에 escalation reason 기록

### CF-005 Dependency 추가 요청은 approval 없이 차단

Steps:

1. `harness.intake` work request
2. `harness.prepare_write` with `intended_sensitive_categories=[dependency_change]`

기대:

- `ok=false`
- error `APPROVAL_REQUIRED`
- blocked_reasons includes dependency approval
- 제품 파일 변경 없음

### CF-006 Approval 생성, 승인, 재시도

Steps:

1. CF-005 상태에서 `harness.request_approval`
2. `harness.user_decision` with approval
3. `harness.prepare_write` retry

기대:

- APR 생성
- `approval_state=granted`
- retry allowed
- approval scope에 allowed paths/tools 저장

### CF-007 Approval scope 밖 수정 시도

Steps:

1. granted approval with allowed path `src/auth/**`
2. `harness.record_change` with changed file `src/billing/Billing.ts`

기대:

- `approval_scope` validator failed
- `SCOPE_VIOLATION` 또는 approval expired
- task phase blocked 또는 next action 재승인

### CF-008 Work 구현 완료 후 바로 close 차단

Steps:

1. work shaping 완료
2. `harness.prepare_write`
3. `harness.record_change`
4. `harness.update_evidence_manifest`
5. `harness.finish_implementation`
6. `harness.close_task`

기대:

- close blocked
- reason: missing passed EVAL
- `phase=verifying`
- RUN-SUMMARY와 EVIDENCE-MANIFEST 존재

### CF-009 Same-session 자기 검토는 detached_verified 불가

Steps:

1. work in verifying
2. `harness.record_eval` with parent run id same as lead run and assurance_impact detached_verified

기대:

- `ok=false`
- error `VERIFY_NOT_DETACHED`
- no assurance upgrade
- validator `same_session_verify_guard` hard block

### CF-010 Fresh evaluator + bundle 검증 통과

Steps:

1. work in verifying with sufficient evidence
2. `harness.launch_verify` with `manual_bundle` or `fresh_session`
3. `harness.record_eval` with valid independence and verdict passed

기대:

- EVAL 생성
- `assurance_level=detached_verified`
- acceptance 없으면 completed
- acceptance 필요하면 waiting_user

### CF-011 검증 통과 but trade-off 남음

Steps:

1. CF-010과 동일
2. `harness.record_eval` with `acceptance_impact=pending` and user_followup non-empty

기대:

- `phase=waiting_user`
- `acceptance_state=pending`
- close blocked until acceptance

### CF-012 Acceptance 후 close

Steps:

1. waiting_user with passed EVAL
2. `harness.user_decision` with acceptance
3. `harness.close_task`

기대:

- `phase=completed`
- `result=passed`
- event `task_closed`

### CF-013 Projection job 실패

Steps:

1. state-changing operation 발생
2. projector write failure simulation

기대:

- DB state current
- projection freshness failed
- status card에 projection failed 표시
- state rollback 없음

### CF-014 사용자가 TASK managed 영역 직접 수정

Steps:

1. current TASK projection 존재
2. managed block 수동 수정 simulation
3. reconcile scan 실행

기대:

- reconcile item 생성
- canonical state 자동 변경 없음
- close may block when drift affects active state

### CF-015 Human notes proposal 감지

Steps:

1. User Notes and Proposals에 새 제안 추가
2. reconcile scan 실행

기대:

- user observation 또는 pending reconcile item 생성
- 자동 approval 또는 acceptance 없음

### CF-016 Repo drift after baseline

Steps:

1. baseline capture
2. unrelated file change after baseline
3. verify or close attempt

기대:

- baseline_freshness warning 또는 block
- active changed files에 영향 있으면 evidence_state=stale

### CF-017 MCP 없이 Skill만 있음

Steps:

1. generated Skill exists
2. MCP unavailable simulation
3. product file write request

기대:

- product file write 보류
- connection diagnostic suggested
- no state mutation except optional diagnostic event

### CF-018 MCP resource read와 tool write 구분

Steps:

1. resource read request
2. attempt to mutate through resource

기대:

- resource read-only
- state change denied
- tool call required

### CF-019 Connector generated drift

Steps:

1. generated Skill block current
2. user modifies managed block
3. connector drift scan

기대:

- connector manifest status drifted
- repair flow requires user/operator decision

### CF-020 Artifact missing after registry record

Steps:

1. artifact registered
2. file deleted
3. artifact rescan

기대:

- missing artifact detected
- evidence stale candidate
- recovery event recorded

## 6. Required event assertions

| Scenario | Required events |
|---|---|
| CF-003 | task_intaken, write_prepared, change_recorded, direct_finished |
| CF-006 | approval_requested, user_decision_recorded, write_prepared |
| CF-008 | implementation_finished, projection_enqueued |
| CF-009 | eval_recorded must not be committed as detached verified |
| CF-010 | verify_launched, eval_recorded |
| CF-012 | user_decision_recorded, task_closed |
| CF-014 | reconcile_detected |
| CF-020 | recovery_recorded or artifact_rescan_recorded |

## 7. Required projection assertions

- TASK projection must include Current Summary.
- TASK projection must include Rolling Spine.
- APR projection must include scope.
- RUN-SUMMARY must include changed files and commands.
- EVAL must include verification independence.
- EVIDENCE-MANIFEST must map AC to evidence refs.
- Projection stale/failed must be visible in status card.

## 8. Required artifact assertions

- Baseline artifact exists before write-capable run.
- Diff artifact exists after product file change.
- Command log artifact exists when command was executed.
- Bundle artifact exists before detached manual verification.
- Validator output artifact exists for hard block validators.

## 9. Passing criteria

Minimum conformance passes when:

- CF-001 through CF-020 all pass.
- Every state-changing call records an event.
- Every state-changing call supports idempotent retry.
- No scenario requires user to compose internal CLI workflow.
- No scenario treats prompt/rule/Skill as policy enforcement by itself.
