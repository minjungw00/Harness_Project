# Verification Result Card Template

## Used when

Use the verification result card when an Eval result needs a compact user-facing display of verdict, assurance impact, reviewed evidence, remaining work, and user follow-up.

## Source records

- Eval record
- assurance impact and verification independence state
- Manual QA and acceptance impact
- reviewed task, run, Evidence Manifest, TDD trace, diff, log, approval, and design refs
- blockers or rework
- user follow-up

## Rendered sections

- verification completion
- Eval identity
- verdict
- assurance
- verification independence
- Manual QA
- acceptance
- evidence reviewed
- remaining work
- user follow-up

## Full template

````text
Verification complete.
Display only: Eval records and gate state remain canonical.

{eval_id}
Verdict: {verdict}
Assurance: {assurance_impact}
Verification independence: {verification_independence}
Manual QA: {manual_qa_impact}
Acceptance: {acceptance_impact}

Evidence reviewed:
- task summary: {task_summary_ref}
- run summary: {run_summary_ref}
- evidence manifest: {evidence_manifest_ref}
- TDD trace: {tdd_trace_ref}
- diff: {diff_ref}
- logs: {logs_ref}
- approvals: {approval_refs}
- design refs: {design_refs}
- redaction or blocked input: {redaction_availability_summary|none}

Remaining work:
{blockers_or_rework}

User follow-up:
{user_followup}
````

## Notes

This template is a rendered card shape, not verification authority. Eval records and gate state remain canonical.

The card must not imply omitted or blocked raw bytes were reviewed. `secret_omitted` can support only visible nonsecret claims; `blocked` is unavailable input unless a documented resolution exists.
