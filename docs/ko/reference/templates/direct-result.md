# DIRECT-RESULT Template

## 사용 시점

작은 direct 작업이 닫혔거나 `work`로 전환된 뒤 결과를 간결하고 부담 없이 보여줘야 할 때 `DIRECT-RESULT`를 사용합니다. 전체 Task gate 보고서가 아니라 direct 결과처럼 읽혀야 합니다.

이 문서는 template 참조 문서입니다. 재설계 문서가 승인되기 전에는 runtime/server 구현, 생성된 운영 파일, 실행 가능한 fixture 파일, runtime data를 만들라는 뜻이 아닙니다. 첫 구현/증명 대상은 계속 Kernel Smoke입니다. Agency-Hardened MVP와 post-MVP automation은 owner 문서가 승격하고 증명하기 전까지 범위 밖입니다.

## 기준 기록

- direct run 기록
- direct 작업에 제품 파일 쓰기가 있었다면 consumed Write Authorization 참조
- 변경 경로
- 범위 밖 또는 유지된 범위 summary
- 실행한 check
- 표시되는 claim이 있을 때 Evidence Manifest, Eval, Manual QA, Acceptance Decision Packet, Residual Risk, Approval, artifact refs
- artifact 참조
- escalation flag
- close assurance
- 해당되는 경우 근거, 검증, Manual QA, 수락, Residual Risk 관련 닫기 영향 요약

닫기 요약 줄(Close Summary line)은 기존 gate와 owner-record ref에서 파생한 표시 전용 요약입니다. Direct 작업은 자신이 요약하는 기록 밖에 별도의 close field를 만들지 않습니다.

## 렌더링 섹션

- Request
- Scope
- Outcome
- Changed Scope
- Checks
- Assurance
- Authority Refs
- 닫기 영향 요약
- Escalation
- Evidence Refs

## 전체 템플릿

````md
---
doc_type: direct_result
task_id: TASK-0001
run_id: RUN-20260506-093015-LEAD-01
result: passed
assurance_level: self_checked
surface_id: reference
source_state_version: 41
updated_at: 2026-05-06T09:40:00+09:00
---

# DIRECT-RESULT

> Projection 보기: `source_state_version`와 `updated_at` 기준으로 렌더링되며 direct Run result를 표시합니다. 이 문서를 편집해도 result, assurance, escalation, close state는 바뀌지 않습니다.

## Request
- user request:

## Scope
- direct run scope:
- limits:
- write authorization:
- allowed paths:
- approval refs:

## Outcome
- result summary:
- close reason:

## Changed Scope
- changed files: `path/to/file`
- no-file result:
- 범위 밖 유지:

## Checks
- self-check:
- tests/build:
- validator outcomes:
- artifact availability:

## Assurance
- assurance_level:
- meaning:
- detached verify needed:

## Authority Refs
- write authorization:
- approval:
- Evidence Manifest:
- Eval:
- Manual QA:
- Acceptance Decision Packet:
- Residual Risk:

## 닫기 영향 요약
- 근거:
- 검증:
- Manual QA:
- 수락:
- Residual Risk status:
- Residual Risk refs:
- 후속 작업:

## Escalation
- escalated_to_work: yes | no
- reason:

## Evidence Refs
- logs:
- diff:
- 후속 보고서:
- 생략/차단 artifact 영향:
````

## 메모

정책 또는 사용자가 detached verification 또는 다른 gate를 요구하지 않으면 direct 작업은 기본적으로 자체 확인(self-checked) 상태로 닫힐 수 있습니다. Consumed Write Authorization 참조를 표시할 수 있지만, projection이 기준 authorization 기록이 되는 것은 아닙니다.

Direct Result의 checks와 tests는 evidence 또는 자체 확인(self-check) 맥락입니다. 조건을 충족하는 Eval 없이는 detached verification이 되지 않고, Manual QA 결과 또는 유효한 waiver 없이는 Manual QA가 되지 않으며, 최종 수락을 암시하지도 않습니다. Direct 작업이 남은 위험을 받아들이는 판단으로 닫힌다면 닫기 영향 요약은 결과를 detached verified처럼 보여주는 대신 받아들인 Residual Risk refs, 필요한 경우 남은 위험을 받아들이는 판단을 기록한 Decision Packet, 후속 작업을 가리켜야 합니다. 알려진 close-relevant risk가 없다면 gate 목록을 덧붙이기보다 그 사실을 직접 말합니다.

Direct Result의 authority claim은 source ref 또는 명시적인 absence를 cite해야 합니다. Write permission에는 Write Authorization, evidence sufficiency에는 Evidence Manifest, detached verification에는 Eval, QA에는 Manual QA record 또는 waiver path, final acceptance에는 Acceptance Decision Packet, residual-risk visibility에는 Residual Risk refs 또는 `ResidualRiskSummary.status=none`을 사용합니다. `not_visible` residual risk를 "none"처럼 렌더링하면 안 됩니다.

`DIRECT-RESULT`는 direct 작업을 위한 가벼운 close impact 표시입니다. `TASK`는 진행 중이거나 최근 닫힌 `work` Task의 이어가기용 Close Summary 표시를 담당하고, Journey Card close context는 compact status/resume 표시입니다. 이 표시들은 [projection/report 경계](../document-projection.md#projection-principles)를 따르며, close와 gate effect는 여전히 owner record에서 옵니다.

Direct Result의 ArtifactRef는 `redaction_state`를 보이게 유지해야 합니다. `secret_omitted`는 보이는 nonsecret evidence만 뒷받침하고, `blocked`는 replacement, waiver, Decision Packet outcome, 받아들인 위험, documented fallback으로 해소될 때까지 원본 입력을 사용할 수 없다는 뜻입니다.
