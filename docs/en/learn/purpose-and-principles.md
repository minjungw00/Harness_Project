# Purpose and Principles

## Start Here

Read this after the short Learn path when you are reviewing whether documentation wording still matches the Harness thesis. First-time readers can start with [Overview](overview.md) and [Harness in One Task](one-task.md) instead.

This is a principles page, not an implementation status report. This repository is documentation-only and does not contain a Harness Server runtime.

## Purpose

Harness exists to make AI-assisted product work followable while preserving user judgment.

Users should be able to begin in ordinary language. Agents should clarify, inspect, recommend, implement when appropriate, check, and report. The fragile part is authority: scope, user-owned decisions, evidence, verification, QA expectations, final acceptance, close readiness, and residual risk should not leak into chat phrasing, generated Markdown, connector output, test logs, or agent confidence.

## Principles

Harness keeps authority local. The durable work basis should not depend on a remote chat transcript or a generated report.

Harness separates unlike things. Scope, sensitive-step permission, product decision, technical decision, evidence, verification, manual QA, final acceptance, and residual-risk acceptance answer different questions.

Harness preserves user agency. The user owns goals, scope, product decisions, material technical decisions, QA expectations, final acceptance, and residual-risk acceptance.

Harness is honest about support. It should say what was checked, what kind of check it was, what evidence supports the claim, what remains unverified, and what still needs a person.

Harness keeps small work small. A narrow typo, copy edit, or leaf fix should not become ceremony. It should also stop being treated as small when scope, meaning, risk, UX, public behavior, sensitive action, or shared-contract impact appears.

Harness describes guarantees by actual capability. Cooperative guidance is not the same as preventive blocking, and early Harness wording must not imply OS-level isolation, arbitrary-tool sandboxing, or tamper-proof storage.

## Failure Model

Harness is designed around recurring failures in AI-assisted work:

- scope becomes implicit;
- user judgment is silently replaced by implementation;
- evidence, verification, QA, final acceptance, residual-risk visibility, and residual-risk acceptance collapse into one "done";
- chat, tool output, or Markdown looks authoritative because it is well written.

Harness responds by making boundaries visible and keeping each kind of support in its own lane.

## Non-Goals

Harness does not replace the product repository, version control, tests, review, product specifications, user judgment, or team process.

Harness is not a prompt pack, MCP itself, an API wrapper, a workflow engine, a report generator, a dashboard, a hosted agent platform, a sandbox, or an OS permission system.

Harness can use instructions, tool output, tests, reviews, reports, dashboards, and specs as context or evidence sources. They do not become Harness authority just because they are useful.

## MVP Boundary

The first future implementation work is about proving the local work-authority model, not building a broad platform.

The early slices should show that ordinary AI-assisted work can be represented as local scope, user-owned judgment, evidence references, close readiness, final acceptance, and residual risk without confusing those records with chat, Markdown, tool output, or product files.

Broader automation, richer readable views, connector ecosystems, hosted workflows, dashboards, analytics, and large conformance suites stay outside the first user-value thesis unless owner docs promote them later.
