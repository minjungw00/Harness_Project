# Later: Assurance Profile

Use this page to route later assurance hardening without pulling it into the MVP implementation path.

This is planning and navigation documentation for future Harness behavior. It does not authorize runtime/server implementation, generated operational files, executable fixtures, runtime data, or product code in this repository.

## Read This When

- You are checking what belongs after MVP-1 User Work Loop.
- You need to keep verification, Manual QA, work acceptance, residual risk, sensitive-action Approval, stewardship, and context hygiene separate.
- You need the right owner document for an assurance contract.

## Main Path

Start with the MVP boundary in [MVP-1 User Work Loop](../build/mvp-user-work-loop.md), then use only the owner needed for the assurance question:

| Need | Owner |
|---|---|
| Core gates, user judgment, close, waiver, acceptance, and residual-risk meaning | [Core Model Reference](../reference/core-model.md) |
| Later/profile-gated API methods and schema material | [API Schema Later](../reference/api/schema-later.md) |
| Design-quality policies, validator IDs, severity composition, and waiver impact | [Design Quality Policies](../reference/design-quality-policies.md) |
| Fixture mechanics and profile proof model | [Conformance Fixtures Reference](../reference/conformance-fixtures.md) |
| Future assurance scenario candidates | [Future Fixtures](future-fixtures.md) |
| Projection display boundaries for assurance reports | [Projection And Templates Reference](../reference/projection-and-templates.md) and [Template Reference](../reference/templates/README.md) |

## Boundary

Assurance Profile is later than MVP-1. It can harden the user-value loop with stronger verification, QA, work-acceptance, residual-risk, sensitive-action Approval, stewardship, and context-hygiene behavior. It is not the first user-value path and not an operations/export/recover profile.

Dashboard, hosted workflow UI, broad connector automation, team workflow, orchestration, preventive security, and isolation remain Roadmap candidates unless an owner promotes and proves a concrete mechanism.
