# EXPORT 템플릿

## 사용 시점

리뷰, 보관, 마이그레이션, 릴리스 인계(Release Handoff)를 위한 선택적 내보내기/보고서 상태 보기를 만들 때 `EXPORT`를 사용합니다.

경계: 상태 보기 템플릿(projection template)일 뿐이며 하네스 서버/런타임 구현이나 생성된 운영 산출물에 권한을 주지 않습니다. 공통 단계와 상태 보기 규칙은 [템플릿 참조](README.md#사용-시점)를 따릅니다.

구현 계층: 운영/내보내기 보고서입니다. 내보내기와 인계 번들(bundle)은 나중 운영/프로필 산출물이며 Core 상태 또는 아티팩트를 대체하지 않습니다.

## 기준 기록

- 포함된 Task와 관문 기록, 안전한 상태/이벤트 버전 범위(version range) 사실
- 작업 조각(Change Unit)
- 실행(Run)
- 민감 동작 승인 기록(나중의 민감 동작 승인(Approval) 프로필이 활성화된 경우에만)
- 증거 목록(Evidence Manifest)
- Eval(분리 검증 결과) 기록
- 수동 QA 기록
- 조정(reconcile) 항목
- 보고서 상태 보기 스냅샷과 읽기용 보기 최신성(projection freshness)
- 아티팩트 참조, 소유자(owner) 관계, `redaction_state`, 보존/사용 가능성, 무결성 메타데이터(metadata)
- 가림, 생략, 차단된 아티팩트 요약
- 생략된 비밀 정보 메모와 보존/만료 아티팩트 요약
- 검토 또는 릴리스 인계(Release Handoff) 표시에 포함될 때 쓰기 승인 기록(Write Authorization), 사용자 판단(User Judgment), 민감 동작 승인(Approval), 증거 목록(Evidence Manifest), Eval(분리 검증 결과), 수동 QA, 최종 수락 맥락, 잔여 위험(Residual Risk), 아티팩트 참조, `redaction_state`, 읽기용 보기 최신성(projection freshness)을 보여주는 간결한 권한 참조
- 내보내기 프로필 경계와 배포/병합이 아님을 알리는 안내 표시

## 렌더링 섹션

- 범위
- 상태 스냅샷
- 보고서 상태 보기 스냅샷
- 아티팩트 참조
- `redaction_state` 요약
- 생략되거나 차단된 내용
- 무결성
- 릴리스 인계(Release Handoff)

## 전체 템플릿

````md
---
doc_type: export_manifest
export_id: EXPORT-0001
project_id: PRJ-0001
profile: standard | release_handoff
export_bundle_status: current
source_state_version: 50
updated_at: 2026-05-06T10:30:00+09:00
---

# EXPORT-0001 하네스 내보내기

> 상태 보기(Projection): `source_state_version`와 `updated_at` 기준으로 렌더링된 보고서 스냅샷입니다. 릴리스 인계(Release Handoff)/내보내기 권한 경계는 [운영과 적합성(Operations And Conformance)](../../operations-and-conformance.md#release-handoff-export-profile)이 담당합니다.

## 범위
- project_id:
- task_ids:
- 포함된 상태 버전 범위(version range):
- 포함된 이벤트 버전 범위(version range):
- 정책 또는 프로필 때문에 생략된 것:
- 만든 사람:
- 만든 시각:

## 상태 스냅샷
- Task:
- Task 관문:
- 작업 조각(Change Unit):
- 실행(Run):
- 민감 동작 승인 참조(나중의 민감 동작 승인(Approval) 프로필이 활성화된 경우에만; 그 외에는 none):
- 증거 목록:
- Eval(분리 검증 결과) 기록:
- 수동 QA 기록:
- 조정(reconcile) 항목:
- 상태/이벤트 스냅샷 메모:

## 보고서 상태 보기 스냅샷
- TASK:
- APR(민감 동작 승인; 나중의 민감 동작 승인(Approval) 프로필이 활성화된 경우에만):
- RUN-SUMMARY:
- EVIDENCE-MANIFEST:
- EVAL:
- DIRECT-RESULT:
- 선택적 설계 상태 보기:

## 아티팩트 참조
| `artifact_id` | 종류 | 소유 기록 | `uri` | `sha256` | `size_bytes` | `redaction_state` | 보존 / 사용 가능성 | 내보내기 처리 | 생략/차단 메모 |
|---|---|---|---|---|---|---|---|---|---|

## `redaction_state` 요약
- 생략된 비밀 정보:
- 생략된 PII:
- 아티팩트 참조별 `redaction_state`:
- 가림 처리된 아티팩트:
- 차단된 아티팩트:
- 보존된 생략 메모:
- 포함된 보존 아티팩트 파일:
- 만료되었거나 사용할 수 없는 아티팩트 참조:
- 정책, 만료, 사용 불가, 생략, 차단 때문에 제외된 아티팩트 파일:

## 생략되거나 차단된 내용
| `artifact_id` | 영향받는 소유자(owner) 또는 표시 | `redaction_state` | 이후 영향 | 메모 |
|---|---|---|---|---|

## 무결성
- 내보내기 해시:
- 매니페스트 해시:
- 생성 시각:

## 릴리스 인계(Release Handoff)
- 닫기 준비 상태:
- 닫기 차단 사유:
- 권한 참조: 쓰기={write_authorization_refs|none}; 판단={user_judgment_refs|none}; 민감동작승인={approval_refs|none}; 증거={evidence_manifest_refs|none}; Eval={eval_refs|none}; 수동QA={manual_qa_refs|none}; 최종수락={acceptance_context_refs|none}; 잔여위험={residual_risk_refs|none}; 아티팩트={artifact_refs|none}; 가림={redaction_status_summary}; 최신성={projection_freshness}
- 민감 동작 승인 참조(`approval_refs`)는 최소 MVP-1에서 `none`입니다. 민감 동작 뒷받침 범위는 나중의 민감 동작 승인(Approval) 담당 프로필이 활성화되지 않은 한 `judgment_kind=sensitive_approval`인 `user_judgment_refs`로 나타납니다.
- 증거 참조:
- 검증 참조:
- 수동 QA 참조:
- 잔여 위험 참조:
- 닫기/보증 표시 구분: self_checked={self_check_refs|none}; detached_verified={eval_refs|none}; verification_waived={verification_waiver_refs|none}; qa_waived={qa_waiver_refs|none}; risk_accepted_close={accepted_residual_risk_refs|none}
- 변경된 파일:
- 보기 최신성:
- 아티팩트 보존/사용 가능성:
- 가림/생략/차단 메모:
- 제안 PR 점검표:
- 제안 배포 점검표:
- 제안 롤백 또는 모니터링 메모:
- 외부 권한 안내: 배포, 병합, 민감 동작 승인(Approval), 운영 모니터링(production monitoring), QA 면제 판단, 검증 위험 수락, 관문 충족, 최종 수락, 잔여 위험 수락, 보장 수준 상승, Task 닫기는 이 보고서 밖에 남는다.
````

## 메모

이 템플릿은 렌더링 결과일 뿐 기준 상태가 아닙니다. `EXPORT`는 `ProjectionKind`일 뿐이며, 내보내기 스냅샷과 구성 요소는 owner 기록 또는 상태 보기(projection) 참조에 연결된 아티팩트로 남습니다.

`EXPORT`의 릴리스 인계(Release Handoff) 표시는 자체 확인된 작업, `detached_verified`, `verification_gate=waived_by_user`, QA 면제 판단, 잔여 위험 수락 닫기를 참조 또는 명시적인 부재와 함께 분리해서 보여줘야 합니다. `EXPORT`는 이런 표시를 보존할 수 있지만 민감 동작 승인(Approval)을 부여하거나, 관문을 충족하거나, 최종 수락을 기록하거나, 잔여 위험 수락을 기록하거나, QA 면제 판단을 기록하거나, 검증 위험 수락을 기록하거나, 보장 수준을 높이거나, Task를 닫지 않습니다.

`EXPORT`는 기본적으로 원본 비밀 정보, PII, 민감 로그, 네트워크 트레이스, 스크린샷, 기타 민감 아티팩트 본문을 포함하면 안 됩니다. 크거나 민감한 아티팩트는 `ArtifactRef`로 나열합니다. 원본 파일은 정책과 보존 정책(retention)이 허용할 때만 포함하고, `secret_omitted` 또는 `blocked` 항목은 참조와 메모로만 표현합니다.

`EXPORT` 프로필이 보고서 상태 보기 스냅샷, 등록된 아티팩트 파일, 상태 스냅샷을 생략한다면 번들(bundle)이 완전한 것처럼 암시하지 말고 무엇이 빠졌는지와 검토 또는 릴리스 인계(Release Handoff)에 미치는 영향을 보여줍니다. 보존된 아티팩트는 소유자(owner) 관계, 무결성(integrity), `redaction_state`, 보존 정책(retention policy), `EXPORT` 프로필이 파일 포함을 허용할 때만 복사할 수 있습니다. 만료되었거나 사용할 수 없거나 `secret_omitted` 또는 `blocked`인 아티팩트는 참조, 안전한 메타데이터, 생략/차단 메모로만 남습니다. `EXPORT`는 상태 보기(projection), Markdown 보고서, 채팅 텍스트, 스테이징 경로(staging path)에서 원본 바이트(raw bytes)를 다시 만들면 안 됩니다.

`secret_omitted`에서는 `EXPORT`가 안전한 생략 메모 또는 핸들(handle), 안전하게 저장된 바이트(bytes)에 대한 `sha256`을 포함할 수 있지만 생략된 값을 포함하면 안 됩니다. `blocked`에서는 `EXPORT`가 커밋된 메타데이터 전용 알림 아티팩트(metadata-only notice artifact)와 그 `sha256`, `size_bytes`, `content_type`을 포함할 수 있습니다. 이 필드들은 금지된 원본 페이로드(payload)가 아니라 알림 바이트(notice bytes)를 설명합니다. 릴리스 인계(Release Handoff) 섹션은 내보내기 전에 문서화된 대체 증거, 면제, 사용자 판단 결과, 수락한 위험, 대체 경로(fallback)로 해소되지 않은 생략 또는 차단 영향을 `unavailable`, `insufficient`, `unresolved input` 중 적절한 상태로 표시해야 합니다.

복구 아티팩트가 `EXPORT`에 나타나면 복구 관찰로 라벨링합니다. 별도의 owner 기록이 이미 그 경로를 해결한 경우가 아니면, 복구 아티팩트는 성공적 완료의 증거가 아니며 증거, 검증, QA, 최종 수락, 닫기 증명으로 계산하면 안 됩니다.
