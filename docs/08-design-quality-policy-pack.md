# Design Quality Policy Pack

## Document Role

This document owns design-quality policies as policy contracts. These policies guide how AI-assisted work stays aligned with product design, domain language, module boundaries, testing discipline, human QA, and context hygiene.

Design-quality policies are not additional kernel invariants. The kernel owns lifecycle, gates, close semantics, and state transitions. This document tells policy evaluators when `design_gate`, `qa_gate`, evidence, or close blockers should be affected.

This document does not define MCP schemas, SQLite DDL, state transition tables, or full templates.

## Policy Contract Shape

Each policy uses the same fields:

| Field | Meaning |
|---|---|
| `name` | Stable policy name. |
| `applies_when` | Conditions that make the policy relevant. |
| `default_requirement` | What should happen by default when it applies. |
| `allowed_waiver` | Who may waive it and what must be recorded. |
| `required_record` | Canonical state record or record family that stores the result. |
| `validator` | Validator that reports compliance, warning, failure, or blocker. |
| `evidence` | Evidence or projection refs expected by the policy. |
| `close_impact` | How unmet requirements affect close or gates. |

Policy validators return the validator result schema owned by the MCP API document.

## Policy Contracts

### Shared Design

| Field | Contract |
|---|---|
| `name` | `shared_design` |
| `applies_when` | Work request is ambiguous, scope/non-scope is unclear, user value needs alignment, public interface/schema/auth/UX/workflow is affected, or a `work` task needs shaping. |
| `default_requirement` | Record goal, scope, non-goals, acceptance criteria, blocking decisions, assumptions, rejected options, domain-language impact, module/interface impact, and first Change Unit shape. Ask the most blocking questions one at a time and stop when the first safe Change Unit can be proposed. |
| `allowed_waiver` | Allowed for small obvious `direct` work, docs-only edits, or emergency fixes when the user/operator records a reason and a follow-up if design risk remains. |
| `required_record` | Shared Design record, Task shaping fields, decision records, and optionally `DESIGN` or `DEC` projections. |
| `validator` | `shared_design_alignment` |
| `evidence` | Task summary, acceptance criteria, decision refs, rejected option refs, domain/module/interface impact refs. |
| `close_impact` | If required and absent, set or keep `design_gate=pending` or `partial`. If risk is high and no waiver exists, block close. A valid waiver may allow `design_gate=waived`. |

### Domain Language

| Field | Contract |
|---|---|
| `name` | `domain_language` |
| `applies_when` | New product term appears, an existing term is used with a new meaning, code and product language diverge, multiple names refer to one concept, or reviewer/evaluator finds a term mismatch. |
| `default_requirement` | Record or update affected terms with meaning, code representation, "not this" boundary, related terms, source, and status. Implementation agents pull only task-relevant terms; reviewers/evaluators receive relevant terms. |
| `allowed_waiver` | Allowed when the work has no domain term impact or the term is intentionally local/temporary. Waiver must record why no canonical term update is needed. |
| `required_record` | `domain_terms` records; `DOMAIN-LANGUAGE` is projection only. |
| `validator` | `domain_language_consistency` |
| `evidence` | Domain term refs, code refs, test naming refs, reconcile item refs for proposals. |
| `close_impact` | If required terms are missing or conflicting, mark `design_gate=partial` or `stale`; block close when the mismatch affects acceptance criteria, public behavior, or verification confidence. |

### Vertical Slice

| Field | Contract |
|---|---|
| `name` | `vertical_slice` |
| `applies_when` | Feature work, user-visible behavior, workflow change, integration behavior, or medium/large `work` task. |
| `default_requirement` | Prefer a thin end-to-end Change Unit that connects trigger/input, domain logic, persistence or state, API/caller boundary, observable output, test evidence, and optional Manual QA. |
| `allowed_waiver` | Horizontal/enabling Change Units are allowed when scaffold, test harness, deep module boundary, migration safety, or public interface decisions must come first. The Change Unit must record `horizontal_exception_reason` and a follow-up vertical Change Unit when applicable. |
| `required_record` | Change Unit fields: `slice_type`, end-to-end path, completion conditions, follow-up vertical Change Unit, and validator results. |
| `validator` | `vertical_slice_shape` |
| `evidence` | Change Unit record, run summary, evidence manifest, tests, Manual QA refs if user-visible. |
| `close_impact` | If vertical slice is required and neither satisfied nor waived, set `design_gate=partial` or `blocked`. A justified horizontal exception may allow close only when the follow-up risk is recorded. |

### TDD Trace

| Field | Contract |
|---|---|
| `name` | `tdd_trace` |
| `applies_when` | Domain logic, service module, bug fix, parser/validator, state transition, deep module internals, or edge-case-heavy behavior. Recommended for API/caller boundaries and integration behavior. |
| `default_requirement` | Record red, green, and refactor evidence for at least one acceptance criterion or behavior slice. Link the trace to the evidence manifest. |
| `allowed_waiver` | Allowed for docs, typos, throwaway prototypes, exploratory UI prototypes, initial scaffolds, or when the user/operator records a non-TDD justification and alternate feedback loop. |
| `required_record` | `tdd_traces` records and `TDD-TRACE` projection when rendered. |
| `validator` | `tdd_trace_required` |
| `evidence` | Failing test log, passing test log, refactor check log, diff refs, non-TDD justification when waived. |
| `close_impact` | Missing required TDD trace makes `design_gate=partial` and may make evidence insufficient. A valid non-TDD justification may satisfy design policy but does not by itself prove behavior. |

### Deep Module / Interface

| Field | Contract |
|---|---|
| `name` | `deep_module_interface` |
| `applies_when` | Public interface changes, module boundary changes, schema/data model changes, auth/security boundaries, compatibility impact, deep module internals, or shallow-module risk. |
| `default_requirement` | Identify affected modules, current role, proposed public interface, internal complexity hidden behind the interface, callers impacted, compatibility impact, and test boundary. Prefer small simple public interfaces with enough internal capability behind them. |
| `allowed_waiver` | Allowed for localized internal changes with no public boundary impact, no dependency direction change, and low compatibility risk. Must record why module/interface review is unnecessary. |
| `required_record` | `module_map_items`, `interface_contracts`, decision records, and optionally `MODULE-MAP` / `INTERFACE-CONTRACT` projections. |
| `validator` | `module_interface_review` |
| `evidence` | Module map refs, interface contract refs, caller impact list, boundary tests, design decisions, compatibility notes. |
| `close_impact` | Missing required review keeps `design_gate=pending` or `partial`; public interface or compatibility risk without review can block close or require user acceptance of residual risk. |

### Manual QA

| Field | Contract |
|---|---|
| `name` | `manual_qa` |
| `applies_when` | UI change, UX flow change, copy/error message change, onboarding/checkout/auth/billing or other critical flow, accessibility impact, visual output, browser-only behavior, or any result that needs product taste judgment. |
| `default_requirement` | Record a Manual QA profile, setup, checklist, result, findings, evidence refs, performer, and next action. Profiles include `ui_quality`, `workflow`, `copy`, `accessibility`, `browser_smoke`, and `performance_smoke`. |
| `allowed_waiver` | Allowed when the user/operator explicitly waives QA and records a waiver reason. Not appropriate when legal, safety, privacy, or high-impact user harm requires inspection. |
| `required_record` | `manual_qa_records`; `qa_gate` is the canonical aggregate gate. |
| `validator` | `manual_qa_required` |
| `evidence` | Manual QA record, screenshots, notes, browser logs, walkthrough refs, finding refs. |
| `close_impact` | If Manual QA is required, `qa_gate=pending` or `failed` blocks successful close. `qa_gate=waived` requires a waiver reason. QA failed should create rework, block close, or require an explicit follow-up path. |

### Context Hygiene

| Field | Contract |
|---|---|
| `name` | `context_hygiene` |
| `applies_when` | Work resumes after interruption, old PRDs/design docs/issues exist, code paths have moved, acceptance criteria changed, module/interface/domain records changed, or evaluator/reviewer needs a focused bundle. |
| `default_requirement` | Push current Task summary, rolling spine, latest run/eval/evidence refs, relevant policy refs, and current acceptance criteria. Pull older PRDs, closed issues, coding standards, and long logs only when needed. Mark stale docs and avoid treating chat as state. |
| `allowed_waiver` | Allowed for short advisor-only work where no product state, design state, or evidence state is being changed. |
| `required_record` | Task summary, projection freshness, reconcile items for drift, evidence manifest, and validator results. |
| `validator` | `context_hygiene_check` |
| `evidence` | Current projection refs, freshness state, stale refs, reconcile item refs, bundle contents for evaluator. |
| `close_impact` | Stale critical context may mark `design_gate=stale`, evidence stale, or projection stale. It can block write or close when the agent cannot safely determine scope, evidence, or current acceptance criteria. |

## Waiver Rules

Waivers must be explicit, scoped, and recorded. A waiver should include:

- policy name
- task and Change Unit
- reason
- accepted risk
- actor who waived
- expiry or follow-up when needed
- affected gate or close impact

Policy waivers can satisfy a design-quality requirement only where the policy contract allows it. They do not waive scope for product writes, sensitive-change approval, required evidence coverage, or required acceptance. Verification waivers are owned by the kernel close semantics and must not produce `assurance_level=detached_verified`.

## Policy-To-Validator Mapping

| Policy | Validator | Primary gate or state impact |
|---|---|---|
| `shared_design` | `shared_design_alignment` | `design_gate` pending/partial/passed/waived |
| `domain_language` | `domain_language_consistency` | `design_gate` partial/stale/passed |
| `vertical_slice` | `vertical_slice_shape` | `design_gate` partial/blocked/passed |
| `tdd_trace` | `tdd_trace_required` | `design_gate` and evidence sufficiency |
| `deep_module_interface` | `module_interface_review` | `design_gate` partial/blocked/passed |
| `manual_qa` | `manual_qa_required` | `qa_gate` pending/passed/failed/waived |
| `context_hygiene` | `context_hygiene_check` | projection freshness, reconcile, evidence/design stale |

The reference MVP may implement minimal validators first, but it should keep validator IDs stable so conformance fixtures can grow without changing policy names.
