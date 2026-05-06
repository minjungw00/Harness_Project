# Strategy

## Document Role

This document owns the strategic layer of the harness: why the harness exists, what failure modes it prevents, which principles are true kernel invariants, and which quality rules are policy defaults. The operational state machine is defined in `docs/03-kernel-spec.md`; design-quality policy details are expanded in `docs/08-design-quality-policy-pack.md`.

This document does not define lifecycle transition tables, gate enum details, MCP request or response schemas, SQLite DDL, full projection templates, or surface-specific connector behavior.

## Strategic Thesis

The harness is a small local operating kernel for AI-assisted development. Its purpose is not to make the chat transcript longer or to turn every task into heavyweight ceremony. Its purpose is to keep product work inside explicit state, scope, evidence, and human judgment boundaries.

The central thesis is:

```text
AI agents can move quickly when the kernel keeps the durable truth small, explicit, and checkable.
```

The user should be able to begin in ordinary language. The agent should be able to ask clarifying questions, shape work, make changes, record evidence, and request decisions. But the durable facts of the work live outside the chat transcript. Completion is not a feeling in the conversation; it is a state transition judged by the kernel.

The harness therefore separates three concerns:

- Conversation is the operating surface.
- Kernel state is the canonical operating record.
- Markdown documents are human-readable projections and proposal surfaces.

## Failure Model

The harness is designed around failures that appear repeatedly in AI development workflows.

### Context Failure

The user loses the thread because the current state, next action, open decisions, and evidence are buried in conversation. When a chat disappears or an agent session resumes cold, the task cannot be reconstructed reliably.

The harness responds by keeping Task state, Change Units, runs, decisions, evidence, and close status in canonical records, with projections generated for human reading.

### Scope And Approval Failure

Work expands during a conversation. A small request becomes a broad rewrite, or a sensitive change proceeds without explicit approval. Approval may be granted for one scope while the actual write touches a different path, command, network target, secret, or baseline.

The harness responds by requiring scoped Change Units for product writes and explicit approval for sensitive categories.

### Evidence Failure

An agent reports that work is done without durable evidence tied to acceptance criteria. Logs, diffs, checks, and evaluation reports remain in the chat or vanish with the session.

The harness responds by requiring evidence coverage where evidence is required and by storing raw evidence in the artifact store.

### Verification Failure

The same agent that implemented the work self-reviews it and the system treats that as independent verification. This confuses confidence with independence.

The harness responds by separating self-checks from detached verification and by refusing to upgrade assurance from same-session review alone.

### Human Judgment Failure

Approval, technical assurance, manual QA, and acceptance collapse into one vague "looks good." The user cannot tell which question has been answered.

The harness responds by separating those judgments:

- Approval: may this sensitive change proceed?
- Assurance: how has the result been technically checked?
- Manual QA: has a human inspected the experiential result where needed?
- Acceptance: does the user accept the result and remaining trade-offs?

### Projection Failure

Generated documents, stale summaries, or human-edited notes are treated as canonical state. A document change silently changes the operational truth.

The harness responds by treating Markdown reports as projections. Human-editable areas are input surfaces; they become state only through reconcile and accepted state events.

## Minimal Harness Kernel

The minimal kernel is the smallest implementable mechanism that preserves the core invariants:

- Task and Change Unit records for continuity and write scope.
- Lifecycle plus gates for state compatibility.
- Approval, evidence, verification, QA, and acceptance records for distinct judgments.
- `prepare_write` as the product-write decision point.
- `close_task` as the completion decision point.
- `state.sqlite` current records plus `state.sqlite.task_events` for operational history.
- Artifact store for raw evidence.
- Projections for human-readable reports and user proposal surfaces.

The kernel specification is the owner for entity semantics, lifecycle fields, gate enums, transition rules, close semantics, waiver semantics, and invariant enforcement.

## Core Invariants

These are the only core invariants. A system that violates one of them is no longer implementing the harness kernel.

1. Chat is not state.
2. Product write requires an active scoped Change Unit.
3. Sensitive change requires explicit approval.
4. Completion requires evidence coverage where evidence is required.
5. Work cannot self-certify detached verification.
6. Required QA and acceptance are separate gates.
7. Projection cannot override canonical state.

## Policy Defaults

The following are design-quality policy defaults, not core invariants. They are important because they improve product quality, but they have applicability rules, allowed waivers, required records, validators, and close impact defined by the policy pack.

- Shared design for work.
- Domain language consistency.
- Vertical slice default.
- TDD trace for suitable work.
- Module and interface review.
- Manual QA for UI, UX, copy, accessibility, visual output, and product taste.
- Context hygiene.

The strategy keeps these defaults visible because they shape the product experience. The policy pack owns their detailed contracts.

## Human Judgment Model

The harness assumes that the human provides direction and judgment, while agents provide options, implementation, evidence, and structured status.

The human owns:

- goals and priorities
- scope confirmation
- sensitive-change approval
- product trade-off decisions
- manual QA results where human inspection is required
- final acceptance or rejection

The agent owns:

- surfacing choices and risks
- proposing a Change Unit
- staying inside approved scope
- recording runs and evidence
- requesting decisions when gates require them
- launching or preparing detached verification when required

The kernel owns:

- whether a write is allowed
- whether a task can close
- whether evidence, verification, QA, and acceptance states are compatible
- whether projections are fresh enough to trust as display

This model keeps the user in charge without requiring the user to manually police every file write or status claim.

## Source-Of-Truth Summary

The canonical operating state is `state.sqlite`. It contains current state records and the append-only `state.sqlite.task_events` table. There is no separate MVP event store.

Raw evidence is canonical in the artifact store. Artifact records and references connect durable files to Tasks, Runs, Evidence Manifests, Evals, Manual QA records, and projections.

Markdown reports are projections generated from state records and artifact references. A projection may be useful, stale, or failed, but it does not override canonical state.

Human-editable sections are input surfaces. User Notes follow this authority path:

```text
human-editable input -> reconcile_items -> accepted state event/record
```

Domain Language, Module Map, and Interface Contract projections follow the same source-of-truth principle: their canonical records live in kernel state, and their Markdown forms are human-readable projections and proposal surfaces.

## Guarantee Level Summary

Guarantee level describes how strongly a connected agent surface can help enforce harness rules.

- Cooperative guarantee: the surface is expected to follow harness instructions and MCP results.
- Detective guarantee: the harness can detect violations and mark state blocked, stale, or partial after observation.
- Preventive guarantee: a guard can block a violation before execution.
- Isolated guarantee: separate worktree, sandbox, or process boundaries isolate risky work.

The MVP reference surface is primarily cooperative and detective. Preventive and isolated guarantees require stronger connector or runtime capabilities. Capability is not a kernel gate; it appears in surface capability validation, `prepare_write` blocked reasons, and user-facing guarantee display.

## MVP Boundary

MVP is a core invariant validation project, not a broad agent-integration platform.

MVP includes:

- one local project registration
- one reference agent surface
- `state.sqlite` current records plus `state.sqlite.task_events`
- artifact registry and artifact store
- public MCP tool surface
- `prepare_write` gatekeeping
- approval, evidence, verification, manual QA, and acceptance gate enforcement
- required MVP report projections for Task status, approval, runs, evidence, eval, and direct results
- detached verification bundle or manual evaluator instruction bundle
- basic doctor, recover, reconcile, export, and conformance smoke paths

MVP does not include:

- all agent surface connectors
- dashboard
- browser QA automatic capture
- cross-surface orchestration
- native hook coverage for every surface
- fully automatic parallel execution
- long-term analytics
- team workflow management

Later automation can strengthen the guarantee level, but it must not weaken the core invariant model.
