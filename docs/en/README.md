# Harness Documentation

This is the English entry point for the active Harness documentation set. Harness is planned as a local work-authority server for AI-assisted product work, but this repository is documentation-only today.

It has no server/runtime implementation, runtime state, generated projections, generated operational artifacts, executable fixtures, conformance runner, or product implementation code. It is not the user's Product Repository, not a Harness Runtime Home, and not a running Harness instance.

## Current Routes

This entry point routes only to the active documentation structure plus the route index.

| Purpose | Route |
|---|---|
| First model | [Start](start.md) |
| User workflow | [User Guide](use/user-guide.md) |
| Agent behavior | [Agent Guide](use/agent-guide.md) |
| User-owned judgment examples | [Judgment Examples](use/judgment-examples.md) |
| Surface-specific recipes | [Surface Recipes](use/surface-recipes.md) |
| Implementation-readiness decisions | [MVP Plan](build/mvp-plan.md) |
| Exact contract owner index | [Reference Index](reference/README.md) |
| Later candidates | [Later Index](later/index.md) |
| Documentation authoring rules | [Authoring Guide](maintain/authoring-guide.md) |
| Translation and semantic-parity rules | [Translation Guide](maintain/translation-guide.md) |
| Manual documentation checks | [Checks](maintain/checks.md) |
| Stable `doc_id` route metadata | [doc-index.yaml](../doc-index.yaml) |

## How To Read

Start with [Start](start.md), then use [User Guide](use/user-guide.md) or [Agent Guide](use/agent-guide.md) depending on the task. Use [Reference Index](reference/README.md) to choose the single owner for exact technical contracts.

README and route documents should summarize and route. They should not define API, schema, storage, security, projection, template, terminology, or current-MVP-scope contracts.

Use [Later Index](later/index.md) only for material outside the active MVP path. Later candidate material does not become active delivery unless the relevant owner promotes it with scope and proof expectations.

Use [Authoring Guide](maintain/authoring-guide.md), [Translation Guide](maintain/translation-guide.md), and [Checks](maintain/checks.md) for documentation work. Checks are manual maintenance aids; their labels do not create runtime conformance, final acceptance, close readiness, implementation readiness, or permission to start server coding.

## Bilingual Parity

English and Korean docs are both active. Major active docs should have paired paths under `docs/en` and `docs/ko`, including the Korean entry at [../ko/README.md](../ko/README.md).

Paired docs must preserve semantic parity, but they do not need line-by-line translation. Korean docs should read as natural Korean technical prose while preserving exact identifiers.

Agents should keep context small, pull owner docs only when needed, and avoid loading paired English/Korean docs for the same `doc_id` in one prompt unless translation or parity review requires comparison.
