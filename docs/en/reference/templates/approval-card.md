# Approval Card Template

## Used when

Use the approval card when a later Approval profile needs a compact user-facing display of requested sensitive-action scope, purpose, boundaries, risks, alternatives, expiry/use behavior, and recommendation. The card asks permission for the sensitive action only; it is not user-owned product or material technical judgment, correctness review, work acceptance, residual-risk acceptance, QA waiver, verification waiver, or Write Authorization.

Boundary: projection template only; it does not authorize runtime/server implementation or generated operational outputs. Shared phase and projection rules live in [Template Reference](README.md#used-when).

Implementation tier: Assurance Profile reports. Use only when committed sensitive-action approval support is in the active profile; minimum MVP-1 Sensitive action approval user judgment requests can use the compact judgment prompt and do not require an approval card.

## Source records

- committed Approval record
- related Sensitive action approval `user_judgment`
- sensitive category and requested scope
- allowed paths, tools, commands, network targets, and secrets
- baseline ref
- risks, alternatives, and recommendation
- related Write Authorization boundary, artifact refs, redaction state, and projection freshness when displayed

Coverage placeholders such as `{approval_covers}` and `{approval_does_not_cover}` are derived display summaries from approval scope, related user judgment refs, linked Approval records when that profile is active, and current write or close context. They show the approval boundary only; the active owner path remains authoritative.

## Rendered sections

- approval requirement
- compact refs
- request identity
- purpose
- allowed paths
- allowed tools
- allowed commands (`allowed_commands`)
- network
- required secrets
- baseline
- expiry and use
- risks
- alternatives
- recommendation
- what this approval covers
- what this approval does not cover
- approval question

## Full template

````text
Approval is required.
Display only: approval must still be recorded through the canonical approval decision path.
Sensitive permission only: this is not user-owned product or material technical judgment, correctness, work acceptance, residual-risk acceptance, QA waiver, verification waiver, or Write Authorization.
Refs: approval={approval_id}; judgment={user_judgment_ref|none}; write={write_authorization_ref|none}; artifacts={artifact_refs|none}; redaction={redaction_availability_summary|none}; freshness={projection_freshness}

{approval_id} {category}
Request: {summary}
Purpose: {why_needed}
This approval would cover:
{approval_covers}

This approval would not cover:
{approval_does_not_cover}

Allowed paths:
{allowed_paths}

Allowed tools:
{allowed_tools}

Allowed commands:
{allowed_commands}

Network:
{allowed_network}

Required secrets:
{required_secrets}

Baseline:
{baseline_ref}

Expiry and use:
expires={expires_at|scope_drift|none}; single_use={single_use_behavior|not_applicable}; write_authorization_requirement={later compatible prepare_write required}

Risks:
{risks}

Alternatives:
{alternatives}

Recommendation:
{recommendation}

Do you approve this sensitive action and scope only, without resolving product/material technical judgment, work acceptance, residual-risk acceptance, or any waiver?
If you say "go ahead," "proceed," or "looks good," Harness records only this sensitive-action Approval. It must not automatically become work acceptance or residual-risk acceptance, and if the phrase is ambiguous, clarify before recording.
````

## Notes

This template is a rendered card shape, not approval authority. Committed Approval records are later-profile; minimum MVP-1 Sensitive action approval requests can be displayed through the compact user judgment prompt instead.

Approval does not resolve user-owned product or material technical judgment, prove correctness, replace verification, replace Manual QA, imply work acceptance, accept residual risk, waive QA or verification, or create Write Authorization; actual writes still require a later compatible `prepare_write` and Write Authorization.

Approval cards should make the Approval boundary explicit. For example, dependency install Approval is not an architecture decision; secret access Approval is not permission to expose secret values; auth or system Approval is not a session/JWT/social-login choice; work acceptance is not permission for additional writes; and residual-risk acceptance or waivers need their own scoped judgment path.
