# DOMAIN-LANGUAGE Template

## 사용 시점

현재 domain term의 의미, code representation, 대기 중인 term decision, deprecated term, 사람이 제안한 변경 사항을 읽기 쉬운 projection으로 볼 때 `DOMAIN-LANGUAGE`를 사용합니다.

## 기준 기록

- `domain_terms`
- domain term 변경을 제안하는 reconcile item
- term을 도입하거나 reconcile한 Task 참조
- `domain_language` 관련 design-quality validator 결과
- projection 최신성 입력

## 렌더링 섹션

- Summary
- Terms
- Pending Term Decisions
- Deprecated Terms
- User Notes and Proposals

## 전체 템플릿

````md
---
doc_type: domain_language
project_id: PRJ-0001
status: active
projection_version: 1
source_state_version: 12
updated_at: 2026-05-06T09:30:15+09:00
---

# Domain Language

<!-- HARNESS:BEGIN managed -->
## Summary
- current status:
- latest reconciled task:
- stale conditions:

## Terms
| Term | Meaning | Code Representation | Not This | Related Terms | Source | Status |
|---|---|---|---|---|---|---|
| Account | login-capable user identity | `src/auth/account.ts` | Profile | User, Session | TASK-0001 | active |

## Pending Term Decisions
| Term | Question | Options | Recommendation | Owner |
|---|---|---|---|---|

## Deprecated Terms
| Term | Replaced By | Reason | Since |
|---|---|---|---|
<!-- HARNESS:END managed -->

## User Notes and Proposals
-
````

## 메모

이 template은 렌더링 결과일 뿐 기준 상태가 아닙니다. 기준 domain term 참조는 `StateRecordRef.record_kind=domain_term`을 사용합니다.
