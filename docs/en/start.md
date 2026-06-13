# Start

## What Harness Is

Harness is a local authority record for AI-assisted product work. It keeps the fragile basis of a task out of chat-only memory: scope, user-owned judgment, evidence, verification expectations, acceptance, close readiness, and residual risk.

Users can speak normally:

```text
Make this plan concrete enough to implement.
Tell me if the scope is getting bigger.
Show what I need to decide and what you can verify.
Before you say it is done, show the evidence and residual risk.
```

The agent can answer in the same ordinary language. When a request hides product, technical, inspection, acceptance, or risk choices, Harness makes those choices visible instead of letting the agent decide them silently.

## What Harness Is Not

Harness is not a prompt pack, chat script, API wrapper, workflow engine, report generator, dashboard, hosted agent platform, `Product Repository`, or `Harness Runtime Home`.

Harness records authority boundaries. It does not turn a polished summary, readable view, status card, or chat answer into the authority record. It also should not claim stronger enforcement than the active scope and security owners support. For exact security wording, use [Security](reference/security.md).

## The Problem It Solves

AI-assisted work can move faster than the record around it:

- A small request grows.
- A product choice gets buried in implementation.
- A test pass starts sounding like proof of the whole user experience.
- A user says "looks good" and the agent treats every unresolved judgment as settled.

Harness exists to make those substitutions visible and invalid as authority.

It helps a later reader see:

- what was in scope
- what the user decided
- what evidence supports the claim
- what was checked
- what still needs human judgment
- whether the work can honestly close

## One Ordinary Task

A user might ask:

```text
Add remember-me behavior to login, but clarify the plan before changing files.
```

A useful agent response does not start by choosing hidden product and security behavior. It names the narrow goal, likely non-goals, facts it can inspect, and judgments the user still owns:

```text
Goal I heard:
Add remember-me behavior to login without redesigning authentication.

Out of scope unless you decide otherwise:
Password reset, signup, social login, and unrelated session maintenance.

I can inspect:
The login form, session settings, and focused tests.

You likely need to decide:
Whether "remember me" means a longer session, remembered email, or both, and what session risk is acceptable.

Safe next step:
Inspect and return a narrow plan. No product writes yet.
```

The user did not need to say an internal mode name. The agent clarified because the request touches product behavior, security expectations, checks, and user-owned judgment.

## What The User Owns

The user owns goals, meaningful scope changes, product behavior, material technical direction, user-visible inspection expectations, final acceptance, residual-risk acceptance, and scoped permission for named sensitive actions.

The agent may compare options, inspect source, run checks, name evidence gaps, and explain consequences. It must not turn tool output, passing tests, generated summaries, or its own confidence into the user's judgment.

Broad phrases such as "go ahead" or "looks good" apply only to the specific pending choice they reasonably answer. They do not automatically grant sensitive-action permission, accept the finished result, accept known residual risk, or settle another out-of-scope judgment candidate.

## What The Agent Does

The agent can clarify requirements, name non-goals, propose the next safe work slice, inspect relevant sources, show options with uncertainty, implement inside agreed scope, run focused checks, and report evidence.

When the work shape changes, the agent should say so. A typo fix can stay light. A bug fix can start with inspection and a narrow repair. A broad feature request should expose the goal, non-goals, success criteria, unknowns, user-owned judgments, evidence plan, and close blockers.

When the agent cannot honestly proceed without a user-owned judgment, it should ask a specific question rather than treating silence or momentum as permission.

## What Evidence Means

Evidence supports a claim. It can be a diff, test output, screenshot, log, source citation, review note, or artifact reference. Evidence should say what it supports and what it does not support.

Evidence shows what the work did, but it does not replace final acceptance. A test pass is not the same as user-visible review of copy, accessibility, visual layout, or the lived flow.

If evidence is missing, stale, weak, or limited to the agent's own check, Harness should keep that visible instead of rounding it into "done."

## What Close Readiness Means

Close readiness answers a simple question: can the task honestly finish now, and what still blocks it?

If something is missing, the close path should name the smallest unblocker. Exact close-readiness meaning belongs to [Core Model](reference/core-model.md); method behavior belongs to [Close-task Method](reference/api/method-close-task.md); error routing belongs to [API Errors](reference/api/errors.md).

## Baseline Scope

The baseline scope is intentionally narrow. Use [Scope](reference/scope.md) for the current active, profile-gated, and out-of-scope boundaries.

Out-of-scope capabilities are not active requirements until the scope owner and affected owners promote them. If a detail is not in current scope, treat it as deferred even when it appears in an example or reserved-value note.

## Where To Go Next

| Reader | Path |
|---|---|
| New user | [User Guide](use/user-guide.md) |
| Working user | [User Guide](use/user-guide.md) -> [Judgment Examples](use/judgment-examples.md) -> [Scope](reference/scope.md) |
| Agent author or operator | [Agent Guide](use/agent-guide.md) -> [Agent Integration Reference](reference/agent-integration.md) |
| Implementer | [Reference Index](reference/README.md) -> active scope -> API methods -> schema owners -> storage effects |
| Documentation maintainer | [Authoring Guide](maintain/authoring-guide.md) -> [Translation Guide](maintain/translation-guide.md) -> [Checks](maintain/checks.md) |

Use the [Reference Index](reference/README.md) when you need exact owner documents. New users should not need API schemas to understand the product.
