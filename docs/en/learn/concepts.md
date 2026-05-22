# Concepts

## What this document helps you do

This document introduces the smallest concept set you need before reading Harness reference specs. Each concept starts with a plain example, then gives a tighter definition.

The reference links below are part of the documentation redesign path. Until the reference tree is fully moved, the current owner documents remain in the top-level spec files such as [03-kernel-spec.md](../03-kernel-spec.md), [04-runtime-architecture.md](../04-runtime-architecture.md), and [07-document-projection.md](../07-document-projection.md).

## Read this when

Read this when Harness terms are starting to appear in examples, status summaries, or reference specs and you want the smallest useful vocabulary.

## Before you read

[Overview](overview.md) is recommended first. No schema or implementation knowledge is required.

## Main idea

Harness vocabulary names a small work journey: request, scope, judgment, support, checks, acceptance, remaining risk, and close.

## The smallest concept set

Harness is easiest to understand if you start with the work journey:

- A user asks for a Task.
- Product writes happen inside a Change Unit.
- Important claims need Evidence.
- Sensitive actions need Approval, and product writes need Write Authorization.
- Checks create Verification, and human inspection may create Manual QA.
- The user gives Acceptance when the task path requires it.
- Remaining uncertainty is Residual Risk.
- Readable documents are Projections, and human edits become state through Reconcile.

These concepts are intentionally small here. During the redesign, strict definitions remain with the current owners: [Kernel Spec](../03-kernel-spec.md) for `reference/kernel.md`, [MCP API and Schemas](../05-mcp-api-and-schemas.md) for `reference/mcp-api-and-schemas.md`, and [Document Projection](../07-document-projection.md) for `reference/document-projection.md`.

## Task

A user says, "Add email login and show a helpful error when the password is wrong." The chat may include many turns, but the work still needs one durable unit that says what the user wants done and what state the work is in.

A Task is the user value unit: the thing the user wants completed, answered, investigated, or decided. Harness uses the Task to keep status, next action, blockers, evidence, QA, acceptance, and close behavior connected.

Current owner: [Kernel Spec](../03-kernel-spec.md). Future reference path: `reference/kernel.md`.

## Change Unit

The email login task may require changes to the login form, one API call, and session handling. That is a bounded slice. If the work suddenly starts rewriting the whole authentication system, the scope has changed and should be visible.

A Change Unit is the bounded product-write scope for a Task. It names the part of the product that may change so the agent, user, and Harness can tell whether a write is inside the agreed work.

Current owner: [Kernel Spec](../03-kernel-spec.md). Future reference path: `reference/kernel.md`.

## Decision Packet

The agent finds two reasonable failed-login behaviors: a generic security-safe message or a detailed message that is easier for users but leaks more information. The agent should not quietly choose the product trade-off if that choice blocks progress.

A Decision Packet records a user-owned decision that blocks progress, write, close, waiver, acceptance, residual-risk acceptance, product direction, scope, design trade-off, stewardship judgment, or public commitment. It names the decision, options, trade-offs, evidence, affected scope, residual risk, and next action.

Current owner: [Kernel Spec](../03-kernel-spec.md). Future reference path: `reference/kernel.md`.

## Evidence

The agent says the login flow works. Useful support might include the diff, the test output, a screenshot of the error state, and a note about the manual browser check. Without those records, "works" is only a chat claim.

Evidence is recorded support for claims about the work. It can include diffs, logs, tests, screenshots, run summaries, evaluation records, Manual QA records, or other durable artifacts tied to the task.

Current owner: [Operations and Conformance](../11-operations-and-conformance.md). Future reference path: `reference/operations-and-conformance.md`.

## Approval

The task needs a new dependency, a network call, or access to a sensitive file. Even if the change is useful, the user may need to approve that category of action before it proceeds.

Approval answers whether a sensitive action may proceed inside a defined scope. Approval is not the same as accepting the final result, choosing a design trade-off, or accepting residual risk.

Current owner: [Kernel Spec](../03-kernel-spec.md). Future reference path: `reference/kernel.md`.

## Write Authorization

The agent is ready to edit the login code. Harness needs to check whether there is an active Change Unit, whether the target path is in scope, whether required approvals exist, and whether any blocking decision must be resolved first.

Write Authorization is the Harness decision that a product write may proceed now. In the current spec language, `prepare_write` is the product-write decision point.

Current owner: [MCP API and Schemas](../05-mcp-api-and-schemas.md). Future reference path: `reference/mcp-api-and-schemas.md`.

## Verification

The agent runs tests after editing the login flow. That is useful, but it is not the same as an independent check by another session, tool path, or evaluator bundle.

Verification records how the result was checked and how independent that check was. Harness separates self-checks from detached verification so confidence is not confused with independence.

Current owner: [Operations and Conformance](../11-operations-and-conformance.md). Future reference path: `reference/operations-and-conformance.md`.

## Manual QA

A test can pass while the error message is confusing, clipped on mobile, or visually inconsistent. A human may need to look at the result and record what they saw.

Manual QA is human inspection of the experiential result where that matters, especially UI, UX, copy, accessibility, visual output, product taste, and other judgment-heavy outcomes.

Current owner: [Design Quality Policy Pack](../08-design-quality-policy-pack.md). Future reference path: `reference/design-quality-policies.md`.

## Acceptance

The work may be implemented and checked, but the user still needs to decide whether the result satisfies the request and whether the remaining trade-offs are acceptable.

Acceptance is the user's judgment that the task result can be accepted. It is separate from Approval, Verification, Manual QA, and Residual Risk.

Current owner: [Kernel Spec](../03-kernel-spec.md). Future reference path: `reference/kernel.md`.

## Residual Risk

The login flow is done, but rate limiting was not added in this task, or the detached verifier could not run in the current environment. That remaining uncertainty should be named instead of disappearing behind "done."

Residual Risk is known remaining uncertainty, limitation, or trade-off after the work. When task close depends on accepting that risk, the user's residual-risk acceptance must be explicit.

Current owner: [Kernel Spec](../03-kernel-spec.md). Future reference path: `reference/kernel.md`.

## Projection

Harness can generate a readable task report or Journey Card from recorded state. A user can read it quickly, but editing that report should not silently rewrite the operating record.

A Projection is a human-readable rendering of Harness state records and artifact references. Markdown reports, Journey Cards, and Journey Spine views are projections; they display state but do not replace it.

Current owner: [Document Projection](../07-document-projection.md). Future reference path: `reference/document-projection.md`.

## Reconcile

A user edits a notes section in a generated report and proposes a different next action. Harness should not pretend the operational state changed just because a Markdown line changed. The proposal needs a deliberate path into state.

Reconcile is the explicit path for turning human-editable notes, proposals, or projection drift into accepted state changes, rejected proposals, notes, decisions, or deferred items.

Current owner: [Document Projection](../07-document-projection.md). Future reference path: `reference/document-projection.md`.
