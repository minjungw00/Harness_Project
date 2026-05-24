# Approval Card Template

## Used when

Use the approval card when a pending approval needs a compact user-facing display of requested scope, purpose, boundaries, risks, alternatives, and recommendation.

## Source records

- approval record
- approval-shaped Decision Packet
- sensitive category and requested scope
- allowed paths, tools, commands, network targets, and secrets
- baseline ref
- risks, alternatives, and recommendation

## Rendered sections

- approval requirement
- request identity
- purpose
- allowed paths
- allowed tools
- allowed commands (`allowed_commands`)
- network
- required secrets
- baseline
- risks
- alternatives
- recommendation
- approval question

## Full template

````text
Approval is required.

{approval_id} {category}
Request: {summary}
Purpose: {why_needed}
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

Risks:
{risks}

Alternatives:
{alternatives}

Recommendation:
{recommendation}

Do you approve this scope?
````

## Notes

This template is a rendered card shape, not approval authority. Approval still requires the canonical approval decision path.

Approval does not resolve user-owned product or material technical judgment, prove correctness, replace verification, replace Manual QA, imply acceptance, accept residual risk, or create Write Authorization.
