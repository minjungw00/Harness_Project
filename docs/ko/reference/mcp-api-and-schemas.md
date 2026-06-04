# MCP API와 스키마

이 페이지는 분리된 API 참조로 안내하는 routing page입니다. 이 문서는 향후 하네스 서버 동작을 계획하고 검토하기 위한 참조이며, 현재 저장소에 MCP server나 runtime 구현이 있다는 뜻이 아닙니다.

큰 schema 파일 하나를 기본으로 읽지 말고 아래 분리 문서를 사용합니다.

| 필요한 것 | Owner |
|---|---|
| MVP-1 active public API | [MVP API](api/mvp-api.md) |
| Shared core schemas, read-only resources, artifact refs, staged value sets | [API Schema Core](api/schema-core.md) |
| Later/profile-gated methods and future schema material | [API Schema Later](api/schema-later.md) |
| Error taxonomy, primary precedence, idempotency, state conflict behavior | [API Errors](api/errors.md) |

API를 읽을 때도 같은 맥락/읽기용 보기 분리를 유지합니다. `harness.status`와 `harness://status/card`는 사용자 상태 카드 또는 간결한 현재 위치 요약을 반환합니다. Agent 접점은 current Core state와 ref에서 에이전트 맥락 패킷을 만들 수 있습니다. 사용자 판단, 실행/근거, 닫기 표시는 필요할 때 작은 MVP-1 보기 세트를 사용합니다. Core 상태가 유일한 운영 기준입니다. 상태 카드, read-only resource, 렌더링된 template, Projection은 상태가 아니며 민감 동작 승인, 작업 수락, 잔여 위험 수용, 근거, 닫기 준비 상태, Write Authorization, close를 만들지 않습니다.

## MVP-1 shortcut

MVP-1 public tool surface는 의도적으로 작습니다.

- `harness.intake`
- `status.next_actions`를 포함한 `harness.status`
- `harness.prepare_write`
- `harness.record_run`
- `harness.request_user_judgment`
- `harness.record_user_judgment`
- `harness.close_task`

`harness.next`는 별도 MVP-1 method가 아닙니다. [Schema Later](api/schema-later.md#harnessnext)의 later/compatibility material로 남습니다.

`harness.prepare_write`는 MVP-1의 쓰기 전 범위 확인입니다. 내부 Write Authorization record를 만들 수 있지만, 이 record는 하네스 수준의 협력형 기록/확인입니다. OS 권한, sandboxing, 변조 방지 storage, 사전 차단을 주장하지 않습니다.

## Legacy anchor map

이 페이지로 향하는 오래된 link는 위 split owner로 update해야 합니다. 아래 anchor는 문서 분리 중 reader를 안내하기 위해서만 남깁니다.

<a id="public-tools"></a>
Public tools: [MVP API](api/mvp-api.md).

<a id="harnessstatus"></a>
`harness.status`: [MVP API: `harness.status`](api/mvp-api.md#harnessstatus).

<a id="harnessintake"></a>
`harness.intake`: [MVP API: `harness.intake`](api/mvp-api.md#harnessintake).

<a id="harnessnext"></a>
`harness.next`: [Schema Later: `harness.next`](api/schema-later.md#harnessnext).

<a id="harnessprepare_write"></a>
`harness.prepare_write`: [MVP API: `harness.prepare_write`](api/mvp-api.md#harnessprepare_write).

<a id="harnessrecord_run"></a>
`harness.record_run`: [MVP API: `harness.record_run`](api/mvp-api.md#harnessrecord_run).

<a id="harnessrequest_user_judgment"></a>
<a id="harnessrequest_user_decision"></a>
`harness.request_user_judgment`: [MVP API: `harness.request_user_judgment`](api/mvp-api.md#harnessrequest_user_judgment).

<a id="harnessrecord_user_judgment"></a>
<a id="harnessrecord_user_decision"></a>
`harness.record_user_judgment`: [MVP API: `harness.record_user_judgment`](api/mvp-api.md#harnessrecord_user_judgment).

<a id="harnessclose_task"></a>
`harness.close_task`: [MVP API: `harness.close_task`](api/mvp-api.md#harnessclose_task).

<a id="read-only-resources"></a>
Read-only resources: [Schema Core: Read-only resources](api/schema-core.md#read-only-resources).

<a id="schema-notation-convention"></a>
Schema notation convention: [Schema Core: Schema notation convention](api/schema-core.md#schema-notation-convention).

<a id="stage-profile-manifest"></a>
Stage Profile Manifest: [Schema Core: Stage Profile Manifest](api/schema-core.md#stage-profile-manifest).

<a id="stage-specific-active-value-sets"></a>
Stage-specific active value sets: [Schema Core: Stage-Specific Active Value Sets](api/schema-core.md#stage-specific-active-value-sets).

<a id="tool-envelope"></a>
Tool envelope: [Schema Core: Tool envelope](api/schema-core.md#tool-envelope).

<a id="mcp-경계와-호출자-신뢰"></a>
<a id="mcp-boundary-and-caller-trust"></a>
MCP boundary and caller trust: [Schema Core: MCP boundary and caller trust](api/schema-core.md#mcp-boundary-and-caller-trust).

<a id="common-response"></a>
Common response: [Schema Core: Common response](api/schema-core.md#common-response).

<a id="shared-schemas"></a>
Shared schemas: [Schema Core: Shared schemas](api/schema-core.md#shared-schemas).

<a id="sensitive-categories"></a>
Sensitive Categories: [Schema Core: Sensitive Categories](api/schema-core.md#sensitive-categories).

<a id="artifactref"></a>
ArtifactRef: [Schema Core: ArtifactRef](api/schema-core.md#artifactref).

<a id="validatorresult"></a>
ValidatorResult: [Schema Core: ValidatorResult](api/schema-core.md#validatorresult).

<a id="error-taxonomy"></a>
Error taxonomy: [API Errors: Error taxonomy](api/errors.md#error-taxonomy).

<a id="primary-error-code-precedence"></a>
Primary Error Code Precedence: [API Errors: Primary Error Code Precedence](api/errors.md#primary-error-code-precedence).

<a id="harnessclose_task-close-blockers"></a>
`harness.close_task` Close Blockers: [API Errors: `harness.close_task` Close Blockers](api/errors.md#harnessclose_task-close-blockers).

<a id="idempotency"></a>
Idempotency: [API Errors: Idempotency](api/errors.md#idempotency).

<a id="state-conflict-동작"></a>
<a id="state-conflict-behavior"></a>
State conflict behavior: [API Errors: State conflict behavior](api/errors.md#state-conflict-behavior).

<a id="harnesslaunch_verify"></a>
`harness.launch_verify`: [Schema Later: `harness.launch_verify`](api/schema-later.md#harnesslaunch_verify).

<a id="harnessrecord_eval"></a>
`harness.record_eval`: [Schema Later: `harness.record_eval`](api/schema-later.md#harnessrecord_eval).

<a id="harnessrecord_manual_qa"></a>
`harness.record_manual_qa`: [Schema Later: `harness.record_manual_qa`](api/schema-later.md#harnessrecord_manual_qa).
