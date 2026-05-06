# User Guide

## Document Role

This document explains how users talk to the agent, how to read state, and which judgments to make at which points.

It does not cover implementation internals or installation details.

## Starting Phrase

Everyday work starts as a conversation, not as a command.

```text
Run this work under the harness.
```

This means: check state, shape the scope, confirm allowed boundaries before writing, and proceed while recording evidence, verification, and user judgment.

Common phrases:

```text
Show me the status.
Continue this work. Check harness state first.
Start with the scope and questions.
If this is small, handle it as direct; if it grows, move it to work.
Approved. The scope is only what you just described.
Start detached verify.
Decide whether Manual QA is needed.
Accepted. Close this task.
```

## Reading A Status Card

A good harness session first shows a short status card.

```text
TASK-0044 Add email login flow
Mode: work
State: shaping
Next action: decide failed-login UX
Scope: login form, login API call, session storage
Approval: dependency_change required
Evidence: none
Verification: not started
Manual QA: pending
Acceptance: pending
Projection: current
```

Look for four things.

- Does the request match the scope?
- What decision do I need to answer?
- What remains among approval, evidence, verification, Manual QA, and acceptance?
- Is the next action safe to proceed with?

If the status looks wrong, say:

```text
Show the current status and next action again from state.
```

## advisor, direct, work

`advisor` is for reading, explaining, comparing, and reviewing. It does not write product files.

```text
Explain this module's role.
Summarize the trade-offs of this design choice.
```

`direct` handles small, low-risk changes quickly. Direct still needs a defined scope before writing product files, and its default assurance is `self_checked`.

```text
Fix the typo on the profile save button. If it is small, handle it as direct.
```

`work` is for feature additions, structural changes, risky fixes, or multi-file work that needs scope shaping, evidence, and independent verification.

```text
Add the email login flow. Run it under the harness.
```

If the work starts small but grows, the agent should say that it is moving the same Task to `work`.

## Four Judgments

Approval, assurance, Manual QA, and acceptance answer different questions.

| Judgment | Question it answers | It cannot replace |
|---|---|---|
| Approval | May this sensitive change proceed? | verification, QA, acceptance |
| Assurance | How far was this technically checked? | approval, QA, acceptance |
| Manual QA | Did a human inspect the actual experience quality? | verification, acceptance |
| Acceptance | Does the user accept the result and remaining trade-offs? | approval, verification, QA |

Examples that need approval include dependency additions, auth/permission changes, schema changes, public API changes, destructive writes, secret access, and production config changes. Approval does not mean correctness or acceptance.

Assurance usually appears as `none`, `self_checked`, or `detached_verified`. `detached_verified` means the result passed a separate verification boundary, not a same-session self-review.

The user may accept verification risk and close the task, but that is a risk-accepted close, not `detached_verified`.

## Missing Evidence

Evidence is not a statement that something was done. It is a record that supports acceptance criteria.

```text
Evidence: partial
Close blocked: AC-02 supporting evidence missing
```

Say:

```text
Show which acceptance criteria are missing evidence, and suggest what additional checks would be enough.
```

If evidence is stale, the work may need a fresh run, fresh logs, a fresh diff, a fresh verification bundle, or scope reconfirmation.

## Verify

Work does not become `detached_verified` from the implementer's self-report alone.

```text
Start detached verify.
```

When verification passes, the agent should summarize what was checked, why the verification boundary counts as independent, and whether any blockers remain.

If you need to close without verification now, say:

```text
Accept the verification risk and close. Record the remaining risk.
```

In that case, the task can close successfully, but assurance is not displayed as `detached_verified`.

## Manual QA

Manual QA is the process for checking qualities that a person needs to inspect, such as UX, workflow, copy, accessibility, and visual result.

```text
Decide whether Manual QA is needed.
```

If QA fails, the task does not close and returns to rework or blocked. Skipping QA requires a waiver with a reason.

```text
Mark Manual QA waived for this internal CLI work. Reason: there is no user UI, and tests/logs are enough to verify it.
```

## Acceptance

Acceptance is the final user judgment that says, "I accept this result." Even if technical verification passes and Manual QA is complete, the task does not close unless the user accepts the remaining trade-offs.

```text
Accepted. Close this task.
```

The user can also reject it.

```text
I do not accept it. Rework the session-expiration UX.
```

Acceptance is not approval or Manual QA.

## Resuming Work

Resume from harness state instead of searching through old chat.

```text
Show the active task status for this project.
Continue TASK-0044. Check harness state first.
```

When resuming, check two questions.

```text
What is the next action now?
Why is the work stopped now?
```

If you left notes in a document, say:

```text
Check the user notes in the TASK document and reconcile anything that should be reflected in state.
```

Documents are human-readable projections. If state and documents seem out of sync, check projection freshness and ask for a state-based summary.
