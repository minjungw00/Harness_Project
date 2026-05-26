# DIRECT-RESULT Template

## Used when

Use `DIRECT-RESULT` for a compact, low-ceremony result report after small direct work closes or escalates. It should read like a direct outcome, not a full task-level gate report.

This is template reference documentation. It does not authorize runtime/server implementation, generated operational files, executable fixtures, or runtime data before the redesigned docs are accepted. The first implementation/proof target remains Kernel Smoke; Agency-Hardened MVP and post-MVP automation stay out of scope unless their owner docs promote and prove them.

## Source records

- direct run record
- consumed Write Authorization ref, when present for direct product writes
- changed paths
- out-of-bounds or unchanged scope summary
- checks performed
- Evidence Manifest, Eval, Manual QA, Acceptance Decision Packet, Residual Risk, Approval, and artifact refs when those claims are displayed
- artifact refs
- escalation flag
- close assurance
- evidence, verification, Manual QA, acceptance, and residual-risk close summaries when applicable

Close Summary lines are derived display summaries from existing gate and owner-record refs. Direct work does not create additional close fields beyond the records it summarizes.

## Rendered sections

- Request
- Scope
- Outcome
- Changed Scope
- Checks
- Assurance
- Authority Refs
- Close Summary
- Escalation
- Evidence Refs

## Full template

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

> Projection view: rendered from `source_state_version` at `updated_at`; displays the direct Run result. Editing it does not change result, assurance, escalation, or close state.

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
- out of bounds kept:

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

## Close Summary
- evidence:
- verification:
- Manual QA:
- acceptance:
- residual risk status:
- residual risk refs:
- follow-up:

## Escalation
- escalated_to_work: yes | no
- reason:

## Evidence Refs
- logs:
- diff:
- follow-up report:
- omitted or blocked artifact impact:
````

## Notes

Direct work may close self-checked by default unless policy or the user requires detached verification or other gates. A consumed Write Authorization ref may be displayed, but the projection does not become the canonical authorization record.

Checks and tests in a Direct Result are evidence or self-check context. They do not become detached verification without a qualifying Eval, do not become Manual QA without a Manual QA result or valid waiver, and do not imply final acceptance. If direct work closes with accepted risk, the Close Summary should point to accepted Residual Risk refs, the Decision Packet that recorded the risk acceptance when one was required, and follow-up instead of presenting the result as detached verified. If no close-relevant risk is known, say that directly rather than adding gate inventory.

Authority claims in a Direct Result should cite source refs or explicit absence: Write Authorization for write permission, Evidence Manifest for evidence sufficiency, Eval for detached verification, Manual QA record or waiver path for QA, Acceptance Decision Packet for final acceptance, and Residual Risk refs or `ResidualRiskSummary.status=none` for residual-risk visibility. Do not render `not_visible` residual risk as "none."

`DIRECT-RESULT` is the low-ceremony close impact display for direct work. `TASK` owns continuity Close Summary display for active or recently closed `work` tasks, and Journey Card close context is compact status/resume display. These displays follow the [projection/report boundary](../document-projection.md#projection-principles); close and gate effects still come from owner records.

Direct result artifact refs must keep redaction state visible. `secret_omitted` supports only visible nonsecret evidence, and `blocked` means the raw input is unavailable until resolved by a replacement, waiver, Decision Packet outcome, accepted risk, or documented fallback.
