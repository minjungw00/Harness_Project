# Approval Card Template

## Used when

Use the approval card when a pending approval needs a compact user-facing display of requested scope, purpose, boundaries, risks, alternatives, and recommendation.

## Source records

- approval record
- approval-shaped Decision Packet
- sensitive category and requested scope
- allowed paths, tools, network targets, and secrets
- baseline ref
- risks, alternatives, and recommendation

## Rendered sections

- approval requirement
- request identity
- purpose
- allowed paths
- allowed tools
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
