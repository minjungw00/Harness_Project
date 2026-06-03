# DOMAIN-LANGUAGE Template

## Used when

Use `DOMAIN-LANGUAGE` when domain terms need a readable projection for current meanings, code representations, pending term decisions, deprecated terms, and human proposals.

Boundary: projection template only; it does not authorize runtime/server implementation or generated operational outputs. Shared phase and projection rules live in [Template Reference](README.md#used-when).

Implementation tier: Future/diagnostic projections. Domain-language maps remain available for stewardship profiles and are not mandatory early projections.

## Source records

- `domain_terms`
- reconcile items that propose domain term changes
- Task refs that introduced or reconciled terms
- related user judgments when a domain-language conflict requires user-owned judgment
- design-quality validator results related to `domain_language`
- routed stewardship findings that affect domain-language refs, when displayed
- projection freshness inputs

## Rendered sections

- Summary
- Terms
- Pending Term Decisions
- Deprecated Terms
- User Notes and Proposals

## Full template

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

> Projection view: rendered from `domain_terms` and related refs at `source_state_version` / `updated_at`. Managed sections are generated display; use `User Notes and Proposals` for reconcile input.

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
<!-- Human-editable: term proposals here are not canonical domain terms until accepted through reconcile/Core. -->
-
````

## Notes

This template is a rendered shape, not canonical state. Canonical domain term refs use `StateRecordRef.record_kind=domain_term`. Pending term decisions, latest-review text, and human proposals are display or reconcile input; they do not satisfy gates, approve writes, create evidence, accept risk, or close work by themselves.

When a term conflict changes product meaning, public behavior, API/interface naming, documentation promises, acceptance criteria, or module responsibility, route the judgment through the existing design-quality and user judgment paths. Rendering the conflict here does not resolve the `design_gate`, `decision_gate`, or close impact by itself.
