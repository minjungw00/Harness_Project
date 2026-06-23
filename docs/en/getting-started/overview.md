# Getting started overview

This is the first-read overview for Volicord. It explains the product thesis in ordinary language and routes exact contract questions to the Reference owners.

## What Volicord Is

Volicord is the local work-authority product/system for AI-assisted product work. Its thesis is simple: AI-assisted work should keep the user's authority basis visible while the work moves.

Volicord itself is not the local authority record. Core is the local authority record for Volicord state. Volicord is the broader product/system around that record, including its local runtime components, surfaces, documentation, and workflows.

## The Ordinary Problem

A user might ask an agent to change product behavior, investigate a failure, or prepare a release note. The agent may inspect files, propose a plan, write code, run tests, and summarize the outcome. That speed is useful, but it can also hide substitutions:

- A small request becomes a broader product change.
- A product decision gets buried inside implementation.
- Evidence for one claim starts sounding like evidence for everything.
- A passing test is treated as final acceptance.
- A user's casual approval is treated as every unresolved judgment being settled.

Volicord exists to make those substitutions visible. It gives the agent and user a local place to keep scope, judgment, evidence, verification criteria, acceptance, residual risk, and close readiness distinct.

## Local Pieces

These names are related, but they are not interchangeable.

| Name | First-read meaning | Exact owner |
|---|---|---|
| Volicord | The local work-authority product/system for AI-assisted product work. | [Scope](../reference/scope.md) |
| Core | The local authority record for Volicord state. | [Core Model](../reference/core-model.md) |
| Volicord implementation | The server implementation set maintained by this repository, not a synonym for Volicord as a whole. | [Runtime Boundaries](../reference/runtime-boundaries.md) |
| `volicord` | The local administrative CLI that builds setup, project, surface, integration, host, and guidance records. | [Administrative CLI](../reference/admin-cli.md) |
| `volicord-mcp` | The stdio MCP adapter process that an MCP host starts as a child process. | [MCP Transport](../reference/mcp-transport.md) |
| `Volicord Runtime Home` | The local runtime data space for Volicord operational data as storage/runtime owners define it. | [Runtime Boundaries](../reference/runtime-boundaries.md) |
| `Product Repository` | The user's project workspace and product files. It may contain explicitly selected integration files. | [Runtime Boundaries](../reference/runtime-boundaries.md) |
| Agent host configuration | Codex, Claude Code, or exported MCP configuration that starts `volicord-mcp --integration <integration_id>`. | [Administrative CLI](../reference/admin-cli.md) |

The current baseline agent integration is integration-bound, not fixed-project. One `volicord-mcp` process binds to one Agent Integration Profile. Each public tool call then selects and validates one explicitly allowed project.

## What Setup Does

Agent setup can:

- create or reuse Runtime Home records
- register or reuse a `Product Repository`
- create an Agent Integration Profile and explicit project allowlist
- install Codex or Claude Code host configuration, or export generic configuration
- run setup verification and report `complete`, `action_required`, `partial_failure`, or `failed`
- optionally write repository guidance when explicitly selected and authorized

Agent setup must not:

- grant access to every project in the Runtime Home
- store Volicord runtime databases or runtime records in a `Product Repository`
- bypass Codex project trust, Claude Code project MCP approval, OAuth, reloads, restarts, or other host-owned actions
- promise that a model will choose Volicord tools automatically

## First-Read Authority Concepts

Volicord documentation keeps these concepts separate:

- User-owned judgment is a decision or assessment the user owns. An agent may explain options, but it must not invent the judgment.
- Evidence is material support for a specific claim, such as a diff, test output, screenshot, log, source citation, review note, or artifact reference.
- Verification criteria are user-visible criteria for checking work. They guide what should be checked; they are not themselves evidence or acceptance.
- `Write Authorization` is the exact product label for Core authority around one compatible product-file write attempt. It is distinct from ordinary write approval.
- Final acceptance and residual-risk acceptance are user-owned judgments.
- Close readiness is the reference concept for whether a task can honestly close from its current state.

For exact authority rules and non-substitution boundaries, use [Core Model](../reference/core-model.md).

## What Volicord Is Not

Volicord is not a prompt pack, chat script, API wrapper, workflow engine, report generator, dashboard, hosted agent platform, `Product Repository`, or `Volicord Runtime Home`.

Volicord also does not turn a polished chat answer, generated summary, readable status card, copied identifier, optional repository guidance, or `Projection` into the authority record. Exact display boundaries belong to [Projection and Templates](../reference/projection-and-templates.md), runtime and location boundaries belong to [Runtime Boundaries](../reference/runtime-boundaries.md), and security wording belongs to [Security](../reference/security.md).

## Next Reader Journeys

| Reader | Next path |
|---|---|
| New product reader | [User Guide](../guides/user-workflow.md) |
| Environment check | [System Requirements](../reference/system-requirements.md) |
| First setup | [Installation](installation.md) -> [Quickstart](quickstart.md) |
| Agent host operator | [Quickstart](quickstart.md) -> [Agent Host Setup](../guides/agent-host-setup.md) -> [Agent Host Troubleshooting](../guides/agent-host-troubleshooting.md) |
| Multi-repository operator | [Multi-Repository Agent Setup](../guides/multi-repository-agent-setup.md) |
| Agent author | [Agent Guide](../guides/agent-workflow.md) -> [Agent Integration](../reference/agent-integration.md) |
| Source-code learner | [Implementation Guide](../development/change-guide.md) -> [Architecture](../development/architecture.md) |
| Reference reader | [Reference Index](../reference/README.md), [Administrative CLI](../reference/admin-cli.md), [API Methods](../reference/api/methods.md) |

New readers should not need API schemas or owner metadata to understand what Volicord is. Use the [Reference Index](../reference/README.md) when you need exact contract owners.
