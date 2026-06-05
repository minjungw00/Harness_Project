# Run Evidence Summary Template

## Used when

Use `run-evidence-summary` after advice, a run, a check, or a change needs a minimal summary of what happened and what evidence now supports the current claim.

Implementation tier: MVP-1 User Work Loop view. Detailed run reports and detailed evidence catalogs are later/full-profile templates.

Boundary: this template displays Run and evidence refs only. It is not the evidence itself, not a detailed evidence catalog, not verification, not QA, not final acceptance, not residual-risk acceptance, and not close readiness authority.

## Source records

- Run refs and command/check summaries
- changed paths or no-file outcome
- consumed Write Authorization ref, no-write basis, or attempted invalid authorization context when relevant
- evidence refs, artifact refs, redaction, and availability notes
- completion claims, acceptance criteria, or close-relevant claims supported by the evidence
- evidence gaps, stale inputs, or unresolved support
- next safe evidence action

## Rendered sections

- run or action
- changed paths
- checks
- evidence refs
- supported claims
- gaps or stale support
- redaction and availability
- next evidence action

## Full template

````text
Run/evidence summary
Display only: refs and summaries; not evidence, verification, QA, final acceptance, residual-risk acceptance, or close.

Action: {run_or_action_summary}
Changed paths: {changed_paths|none}
Checks: {checks_run_or_reason_not_run}
Write check: {write_check_summary|no product write}
Evidence: {evidence_status}. {evidence_summary}
Evidence refs: {evidence_refs|none}
Artifact refs: {artifact_ref_summary|none}
Redaction or availability: {redaction_availability_summary|none}
Supports: {supported_claims_or_criteria|none}
Still missing or stale: {evidence_gaps_or_stale_inputs|none}
Next safe evidence action: {next_evidence_action|none}
Sources/freshness: {source_freshness_summary}
````

## Notes

Evidence sufficiency is coverage, not volume. If a claim has no current supporting ref, or a critical artifact ref lacks owner relation, integrity metadata, redaction state, or availability, show the gap and current evidence status instead of treating a long artifact list or report prose as proof.

Only a compatible consumed Write Authorization may be displayed as the product-write compatibility record for a product-write Run. Attempted invalid authorization refs may be shown only as violation/audit or validator-finding context, and they must not be rendered as a consumed Write Authorization or completion evidence.

Keep this summary intentionally smaller than a full evidence report. Show the evidence refs and visible gaps needed for the user's next decision; do not expand full artifact inventories or raw artifact bodies.
