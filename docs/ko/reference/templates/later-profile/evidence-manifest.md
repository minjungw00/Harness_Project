# EVIDENCE-MANIFEST 템플릿

## 사용 시점

수용 기준, 완료 조건, 닫기에 영향을 주는 주장이 어떤 뒷받침 근거와 아티팩트 참조로 뒷받침되는지 보여줘야 할 때 `EVIDENCE-MANIFEST`를 사용합니다.

경계: 상태 보기 템플릿(projection template)일 뿐이며 하네스 서버/런타임 구현이나 생성된 운영 산출물에 권한을 주지 않습니다. 공통 단계와 상태 보기 규칙은 [템플릿 참조](README.md#사용-시점)를 따릅니다.

구현 계층: 향후/진단용 상태 보기(projection)입니다. MVP-1은 상태 카드 또는 실행/근거 요약으로 근거 요약과 공백을 보여주며, 상세 근거 목록(Evidence Manifest) 상태 보기는 나중 프로필 범위입니다.

## 기준 기록

- 근거 목록(Evidence Manifest) 기록
- 수용 기준
- 완료 조건
- 변경 파일 뒷받침 범위
- 설계 품질 뒷받침 범위
- 민감 동작 승인 참조(나중의 민감 동작 승인(Approval) 프로필이 활성화된 경우에만; 그 외에는 none)
- 해시(hash), 크기(size), 가림 상태, 보존/사용 가능성, owner 관계, 후속 근거 영향을 포함한 아티팩트 참조
- 관련 실행(Run), Eval(분리 검증 결과), 피드백 루프, 수동 QA, TDD 트레이스 참조
- 닫기 맥락으로 렌더링할 때 닫기에 영향을 주는 검증, 수동 QA, 최종 수락, 잔여 위험 요약
- 닫기 맥락으로 렌더링할 때 쓰기 허가 기록(Write Authorization), 사용자 판단(User Judgment), 민감 동작 승인(Approval), 근거 목록(Evidence Manifest), Eval(분리 검증 결과), 수동 QA, 최종 수락 맥락, 잔여 위험(Residual Risk), 아티팩트 참조, 가림 상태, 읽기용 보기 최신성(projection freshness)을 보여주는 간결한 권한 참조

## 렌더링 섹션

- 식별 정보
- 요약
- 닫기 영향 요약
- 권한과 닫기 참조
- 수용 기준 뒷받침 범위
- 완료 조건 뒷받침 범위
- 변경 파일 뒷받침 범위
- 설계 품질 뒷받침 범위
- 민감 동작 승인 참조
- 근거 참조
- 가림과 사용 가능성
- 오래된 것으로 보는 조건

## 전체 템플릿

````md
---
doc_type: evidence_manifest
evidence_manifest_id: EM-0001
task_id: TASK-0001
change_unit_id: CU-01
status: partial
source_state_version: 44
updated_at: 2026-05-06T09:50:00+09:00
---

# EM-0001 근거 목록(Evidence Manifest)

> 상태 보기(Projection): `source_state_version`와 `updated_at` 기준으로 렌더링되며 owner 기록과 아티팩트 참조의 대응을 표시합니다. 닫기는 Markdown 편집이 아니라 기준 `evidence_gate`와 관련 상태를 따릅니다.

## 식별 정보
- task_id:
- change_unit_id:
- baseline_ref:
- 실행 요약(`run_summary`):
- 최신 Eval(`latest_eval`):

## 요약
- 근거 상태:
- 뒷받침되지 않는 기준:
- 생략/차단 근거 영향:
- 오래된 것으로 보는 조건:
- 다음 근거 행동:

## 닫기 영향 요약
- 근거가 뒷받침하는 것:
- 근거가 대체하지 않는 것: 검증, 수동 QA, 최종 수락, 잔여 위험 표시, 잔여 위험 수락
- 검증 상태:
- 수동 QA 상태:
- 최종 수락 상태:
- 잔여 위험 표시:
- 잔여 위험 수락:
- 닫기/보장 표시 구분:
- 다음 닫기 조치:

## 권한과 닫기 참조
- 간결한 참조: 쓰기={write_authorization_ref|none}; 판단={user_judgment_refs|none}; 민감동작승인={approval_refs|none}; 근거={evidence_manifest_id}; Eval={eval_ref|none}; 수동QA={manual_qa_ref|none}; 최종수락={acceptance_context_ref|none}; 잔여위험={residual_risk_refs|none}; 아티팩트={artifact_refs|none}
- 민감 동작 승인 참조(`approval_refs`)는 최소 MVP-1에서 `none`입니다. 민감 동작 뒷받침 범위는 나중의 민감 동작 승인(Approval) 담당 프로필이 활성화되지 않은 한 `judgment_kind=sensitive_approval`인 `user_judgment_refs`로 나타납니다.
- 가림 상태:
- 보기 최신성:

## 수용 기준 뒷받침 범위
| AC ID | 진술 | 뒷받침 상태 | 실행(Run) 참조 | ArtifactRef 참조 | 뒷받침 상태 참조 | 메모 |
|---|---|---|---|---|---|---|
| AC-01 | | supported | RUN-0001 | ART-TEST-0001, ART-DIFF-0001 | FBL-0001 | |
| AC-02 | | unsupported | | | | |

## 완료 조건 뒷받침 범위
| 조건 | 뒷받침 상태 | 실행(Run) 참조 | ArtifactRef 참조 | 뒷받침 상태 참조 | 메모 |
|---|---|---|---|---|---|
| | supported | RUN-0001 | ART-0001 | | |
| | unsupported | | | | |

## 변경 파일 뒷받침 범위
| 경로 | 뒷받침하는 기준 | 근거 참조 |
|---|---|---|
| `src/...` | AC-01 | DIFF-0001, LOG-0001 |

## 설계 품질 뒷받침 범위
| 항목 | 뒷받침 범위 / 관문 표시 상태 | 근거 참조 | 메모 |
|---|---|---|---|
| vertical_slice_shape | passed | CU-01 | |
| decision_quality_check | passed | UJ-0001 | |
| autonomy_boundary_check | passed | CU-01 | |
| feedback_loop_check | passed | FBL-0001, TDD-0001, LOG-0001 | |
| tdd_trace_required | passed | TDD-0001, RED-LOG-0001, GREEN-LOG-0001 | RED, GREEN, 관련 리팩터/확인 뒷받침 범위가 수용 기준 및 변경 파일로 연결된다. |
| module_interface_review | passed | module_map_item: MMI-0001, interface_contract: IFACE-0001, UJ-0001 | |
| codebase_stewardship_check | passed | domain_term: TERM-0001, module_map_item: MMI-0001, interface_contract: IFACE-0001, feedback_loop: FBL-0001 | |
| residual_risk_visibility_check | pending | RR-0001 | |
| manual_qa_required | pending | qa_gate; 충족하는 수동 QA 기록 아직 없음 | |

`뒷받침 범위 / 관문 표시 상태`는 이 근거 목록의 근거 뒷받침 범위 또는 닫기와 관련된 관문 표시 상태입니다. 이 열의 `pending` 같은 값은 `ValidatorResult.status` 값이 아닙니다.

## 민감 동작 승인 참조
- 나중의 민감 동작 승인(Approval) 담당 프로필이 활성화된 경우에만 채웁니다. 최소 MVP-1의 민감 동작 뒷받침 범위는 `judgment_kind=sensitive_approval`인 `user_judgment_refs`에 둡니다.
- APR-0001:

## 근거 참조
- 실행 요약:
- 피드백 루프:
- TDD 트레이스:
- TDD RED 대상 / 계획:
- TDD RED:
- TDD GREEN:
- TDD 리팩터/확인:
- 수동 QA:
- 변경 차이:
- 로그:
- 번들:
- 체크포인트:
- 테스트:
- 빌드:

## 가림과 사용 가능성
| 아티팩트 참조 | 해시 / 크기 | 가림 상태 | 보존 / 사용 가능성 | 근거 영향 | 메모 |
|---|---|---|---|---|---|
| ART-0001 | sha256:abc123... / 12 KB | secret_omitted | 보존된 참조; 원본 비밀 정보 생략 | 보이는 비밀 정보가 아닌 사실만 지원 | |
| ART-0002 | sha256:def456... / 1 KB | blocked | 메타데이터 전용 알림(metadata-only notice) | 사용할 수 없는 입력; 주장은 해소 전까지 `insufficient` | |

## 오래된 것으로 보는 조건
- 기록된 기준선에서 기준선 drift(불일치)가 발생함
- 뒷받침하는 실행(Run) 또는 Eval 이후 변경 파일이 수정됨
- 민감 동작 승인(Approval) 범위가 만료되거나 달라짐
- 뒷받침하는 아티팩트가 `missing`, `blocked`, 또는 `integrity_failure` 상태가 됨
- 뒷받침하는 아티팩트 해시(hash) 또는 크기(size)가 등록된 참조와 더 이상 일치하지 않음
- 관련 설정 변경
- 관련 공유 설계, 도메인 용어, 모듈 맵 항목, 인터페이스 계약 기록 변경
````

## 메모

근거(Evidence)가 필요한 경우 닫기 판단은 보고서 문장만이 아니라 기준 `evidence_gate`를 따릅니다.

근거 충분성은 아티팩트 개수가 아니라 수용 기준, 완료 조건, 닫기 관련 주장의 뒷받침 범위에 달려 있습니다. 필수 행에 현재 뒷받침 참조가 없으면 아티팩트가 많아도 근거 목록은 `partial`로 남습니다. 작은 직접 문서 전용 Task는 모든 필수 조건을 뒷받침한다면 실행(Run) 참조 하나와 변경 차이 아티팩트 하나만으로도 `sufficient`일 수 있습니다.

커버리지 매핑 예시:

| 기준 / 조건 | 실행(Run) 참조 | ArtifactRef 참조 | 뒷받침 상태 참조 | 충분성 메모 |
|---|---|---|---|---|
| AC-01 의미 변경 없이 문서 오타 수정 | RUN-DOCS-001 | ART-DIFF-001 | | 변경된 문서 경로와 자체 확인이 명시된 문서 전용 조건을 뒷받침할 때만 `sufficient`입니다. |
| AC-02 로그인 폼이 이메일을 제출함 | RUN-FEATURE-001 | ART-DIFF-002, ART-TEST-002 | FBL-001 | 실행(Run), 변경 차이, 테스트/로그 참조가 Task 전체가 아니라 이 AC에 대응될 때 `supported`입니다. |
| AC-03 최종 버튼 문구가 대상 뷰포트(viewport)에서 읽을 수 있음 | RUN-UI-001 | ART-SCREENSHOT-001, ART-DIFF-003 | QA-0001 | 수동 QA가 `required`이면 스크린샷이나 브라우저 스모크만으로 QA 경로를 충족하지 않습니다. |
| AC-04 내보내기가 승인된 가림 처리 필드만 포함함 | RUN-EXPORT-001 | ART-EXPORT-MANIFEST-001, ART-LOG-001 | APR-0001, DEC-0001 | `APR-0001`은 나중의 민감 동작 승인(Approval) 프로필이 활성화된 경우에만 있습니다. 민감 동작 승인(Approval)과 판단 요청(Decision) 참조는 범위 또는 사용자 판단 맥락을 보여줍니다. 가림 처리된 아티팩트 참조는 여전히 비밀 정보가 아닌 주장을 증명해야 합니다. |
| 완료 조건: 변경 범위를 독립 검증자가 검토함 | RUN-VERIFY-001 | ART-BUNDLE-001 | EVAL-0001 | Eval이 현재 참조를 검토했고 요청된 닫기에 필요한 독립성이 있을 때만 유효합니다. |

근거 목록(Evidence Manifest)은 주장을 뒷받침하지만 그 자체로 정확성을 증명하거나 분리 검증을 만들거나 수동 QA를 기록하거나 최종 수락을 암시하거나 잔여 위험을 보이게 하거나 잔여 위험을 수락하지 않습니다. 이 템플릿에서 닫기 영향 요약을 렌더링할 때는 테스트 통과, 자체 확인(self-check), QA 면제 판단, 사용자의 최종 수락이 서로 다른 닫기 조건으로 오해되지 않도록 각 줄을 분리해 보여줘야 합니다.

닫기 맥락을 보여줄 때 근거 목록은 잔여 위험 수락 닫기, `verification_gate=waived_by_user`/검증 위험 수락, QA 면제, self-checked, `detached_verified`를 owner 참조 또는 명시적인 부재와 함께 서로 다른 표시 상태로 렌더링해야 합니다. 이 라벨은 owner 기록을 읽기 쉽게 요약할 뿐이며 근거 목록(Evidence Manifest) 권한이 아닙니다.

뒷받침 범위 행은 큰 근거 본문을 붙여 넣는 대신 owner 기록과 아티팩트 참조(ArtifactRef)를 가리켜야 합니다. 어떤 기준, 조건, 주장을 뒷받침하는 참조가 없다면 문장으로 빈틈을 메우지 말고 `unsupported`, `insufficient`, `stale`, `blocked` 중 적절한 상태로 보여줍니다.

채팅 문장과 Markdown 보고서 문장은 근거 이야기를 설명할 수 있지만, 관련 기준이 호환되는 owner 기록과 등록된 ArtifactRef 참조를 가리키지 않는 한 충분성을 증명하기에는 충분하지 않습니다.

큰 로그, 변경 차이, 스크린샷, 트레이스(trace), 번들(bundle)은 짧은 결과와 함께 등록된 ArtifactRef 참조로 남겨야 합니다. 근거 목록은 독자가 아티팩트 본문을 열어 보기 전에 가림 상태와 사용 가능성을 먼저 보여줘야 합니다.

`secret_omitted` 아티팩트는 비밀 정보가 아닌 근거가 보이는 주장만 뒷받침할 수 있으며, 생략된 값이 필요한 주장은 뒷받침하지 못합니다. `blocked` 아티팩트는 커밋된 메타데이터 전용 알림(metadata-only notice)이지 사용 가능한 원본 근거가 아닙니다. 의존하는 기준은 대체 근거, 면제, 사용자 판단 결과, 수락한 위험, 문서화된 대체 경로(fallback)가 근거 경로를 해소할 때까지 `unsupported`, `insufficient`, `blocked` 중 적절한 상태로 남습니다. 이 템플릿은 생략된 비밀 정보/PII 값 또는 차단된 페이로드(payload)를 포함하면 안 됩니다.
