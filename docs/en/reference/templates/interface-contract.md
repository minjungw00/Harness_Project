# INTERFACE-CONTRACT Template

## Used when

Use `INTERFACE-CONTRACT` when a module interface, caller impact, compatibility risk, or test boundary needs a readable projection.

## Source records

- `interface_contracts`
- impacted caller refs
- related module map items
- related Decision Packets or design refs
- boundary, integration, or contract test refs
- design-quality validator results related to `deep_module_interface`
- projection freshness inputs

## Rendered sections

- Identity
- Contract
- Callers Impacted
- Test Boundary
- Review
- References
- User Notes and Proposals

## Full template

````md
---
doc_type: interface_contract
interface_contract_id: IFACE-0001
task_id: TASK-0001
review_status: pending
projection_version: 1
source_state_version: 42
updated_at: 2026-05-06T09:30:15+09:00
---

# IFACE-0001 Interface Title

> Projection view: rendered from `interface_contracts` and related refs at `source_state_version` / `updated_at`. Managed sections are generated display; use `User Notes and Proposals` for reconcile input.

<!-- HARNESS:BEGIN managed -->
## Identity
- module:
- interface:
- change type: new | changed | deprecated | removed

## Contract
- inputs:
- outputs:
- errors:
- side effects:
- compatibility impact: none | minor | breaking

## Callers Impacted
- caller:

## Test Boundary
- boundary tests:
- integration tests:
- contract tests:

## Review
- review_status: pending | reviewed
- reviewed by:
- decision:
- waiver reason:

## References
- TASK:
- DESIGN:
- DEC:
- EVIDENCE-MANIFEST:
<!-- HARNESS:END managed -->

## User Notes and Proposals
<!-- Human-editable: interface proposals here are not canonical Interface Contract records until accepted through reconcile/Core. -->
-
````

## Notes

This template is a rendered shape, not canonical state. Canonical interface refs use `StateRecordRef.record_kind=interface_contract`.
