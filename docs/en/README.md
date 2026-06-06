# Harness Documentation

This is the English entry point for the active Harness documentation set. Harness is a future local work-authority server for AI-assisted product work. Its planned authority is over Harness records and state transitions: scope, user-owned judgment, evidence, verification expectations, final acceptance, close readiness, and residual risk.

This repository is documentation-only today. It has no server/runtime implementation, runtime state, generated projections, generated operational artifacts, executable fixtures, conformance runner, or product implementation code. It is not the user's Product Repository, not a Harness Runtime Home, and not a running Harness instance.

Harness is not a prompt pack, operating-system permission control, arbitrary-tool sandboxing, tamper-proof storage, default pre-tool blocking, or security isolation. Treat the docs as planning source material for a future server unless the maintainer handoff status in [MVP Plan](build/mvp-plan.md) says otherwise.

## Current Routes

| Purpose | Route |
|---|---|
| First model | [Start](start.md) |
| User workflow | [User Guide](use/user-guide.md) |
| Agent behavior | [Agent Guide](use/agent-guide.md) |
| User-owned judgment examples | [Judgment Examples](use/judgment-examples.md) |
| Current MVP plan and implementation-readiness decisions | [MVP Plan](build/mvp-plan.md) |
| Exact contract owners | [Reference Index](reference/README.md) |
| Deferred material | [Later Index](later/index.md) |
| Documentation authoring rules | [Authoring Guide](maintain/authoring-guide.md) |
| Translation and semantic-parity rules | [Translation Guide](maintain/translation-guide.md) |
| Manual documentation checks | [Checks](maintain/checks.md) |

## How To Read

Start with [Start](start.md), then use [User Guide](use/user-guide.md) or [Agent Guide](use/agent-guide.md) depending on the task. Use [MVP Plan](build/mvp-plan.md) for the current MVP scope and server-coding decisions. Use [Reference Index](reference/README.md) to find the single owner for exact schemas, API behavior, storage, state transitions, security wording, projection/template rules, conformance meaning, integration behavior, and terminology.

Use [Later Index](later/index.md) only for material outside the active MVP path. Later material does not become active delivery unless the relevant owner promotes it with scope and proof expectations.

Use [Authoring Guide](maintain/authoring-guide.md), [Translation Guide](maintain/translation-guide.md), and [Checks](maintain/checks.md) for documentation work. Checks are manual maintenance aids; their labels do not create runtime conformance, final acceptance, close readiness, implementation readiness, or permission to start server coding.

## Bilingual Parity

English and Korean docs are both active. Major active docs should have paired paths under `docs/en` and `docs/ko`, including the Korean entry at [../ko/README.md](../ko/README.md).

Paired docs must preserve semantic parity, but they do not need line-by-line translation. Do not finish a meaning-changing edit with only one language updated.

Agents should keep context small, pull owner docs only when needed, and avoid loading paired English/Korean docs for the same `doc_id` in one prompt unless translation or parity review requires comparison.
