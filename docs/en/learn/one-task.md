# Harness in One Task

## Start With One Normal Request

This page follows one future Harness-assisted task. It is a learning story, not a runtime transcript. This repository is documentation-only and does not contain a running Harness Server.

The user asks:

```text
Add remember-me behavior to login, but clarify the plan before implementation.
```

The user does not need to know internal labels. The agent should make the work understandable through plain questions: what is the work, what is in scope, what does the user need to decide, what evidence will support the result, what was checked, and what still blocks close?

## 1. Clarify Before Writing

The agent should not start by changing files when the request still hides product or security choices.

```text
What I can inspect:
- current login form structure;
- current session lifetime settings;
- existing tests around sign-in and sign-out.

What you likely need to decide:
- should "remember me" keep the user signed in, remember the email, or both?
- if it keeps the user signed in, what session lifetime and storage risk are acceptable?

Safe next step:
Inspect the auth shape and return a narrow plan. No product writes yet.
```

The plan is useful, but it is not permission to implement and not acceptance of any risk.

## 2. Separate Facts From User Judgment

After inspection, the agent can summarize:

```text
Confirmed facts:
- The login form can add a checkbox without a layout rewrite.
- Session lifetime is controlled server-side.
- Tests already cover sign-in and sign-out.

Still yours to decide:
- the product meaning of "remember me";
- the acceptable session lifetime if sessions are extended.
```

If the user says:

```text
Show what I need to decide and what you can check yourself.
```

the agent should keep that split visible:

```text
User decision:
Should "remember me" mean remembered email, longer session, or both?

Agent-checkable:
Whether the existing session mechanism can support the selected behavior.
```

Tool output can inform the choice. It does not make the user's choice.

## 3. Name The Scope

Once the user chooses longer sessions, the agent should name the work boundary before implementing:

```text
In scope:
Login checkbox, selected longer-session behavior, focused tests, and directly related copy.

Out of scope:
Passwordless login, account recovery, global session redesign, unrelated auth cleanup.

Still user-owned if it matters:
Session lifetime, storage risk, QA expectation, residual-risk acceptance, and final work acceptance.
```

If a small side request appears, such as renaming the login button from "Sign in" to "Log in," the agent can keep it light only while the boundary holds. If the label turns out to be shared across unrelated flows, the agent should stop and say the scope grew.

## 4. Ask Specific Questions

Harness should not turn every uncertainty into a heavy ceremony. It should ask the specific question that blocks honest progress.

```text
Decision needed:
How long should a remembered session last on this device?

Recommendation:
Use the existing session mechanism with a shorter remembered-session lifetime before adding a separate persistent token.

Uncertainty:
I still need to confirm how the current session cleanup handles extended lifetime.
```

A broad "go ahead" should not be stretched to cover unrelated product, technical, QA, acceptance, or risk decisions.

## 5. Keep Sensitive Permission Narrow

If the chosen approach requires a sensitive step, the prompt should say exactly what permission covers.

```text
Sensitive step permission needed:
May I update the dependency lockfile to add the session helper required by this approach?

This only covers that step. It does not accept the finished result, waive QA, or accept residual risk.
```

Permission for a named step is not work acceptance.

## 6. Show Evidence And Checks

After implementation, the agent should connect claims to support:

```text
Claim:
Remembered sessions survive browser restart.

Evidence:
- changed login form and session paths;
- focused test output for remembered and non-remembered sessions;
- notes from inspecting the session cleanup path.

Checks:
- focused automated tests passed;
- login flow still needs human QA for copy, keyboard behavior, and error-state layout.
```

Evidence, verification, and manual QA are related but separate. A test pass does not prove the human experience, and a self-check is weaker than a genuinely separate verification path.

## 7. Show Residual Risk Before Acceptance

Before asking the user to accept the work, the agent should show known remaining uncertainty:

```text
Residual risk:
Remembered-session behavior was checked in the local browser path, but not across every supported browser policy combination.
```

If the user says:

```text
Looks good.
```

the agent should connect that phrase to the named judgment:

```text
To close this work, please confirm:
Do you accept the remember-me result with the residual risk shown above?
```

Work acceptance is the user's result judgment. It is separate from sensitive-step permission, evidence, tests, manual QA, verification, and residual-risk visibility.

## 8. Close Honestly

A useful close summary is short and grounded:

```text
Close readiness:
- Scope stayed within login checkbox, selected session behavior, focused tests, and related copy.
- User-owned product meaning was decided.
- Evidence supports remembered and non-remembered session behavior.
- Manual QA is handled, or its waiver and impact are visible.
- Residual risk was shown before work acceptance.
```

If something is missing, close should name the smallest unblocker:

```text
Close blocked:
Manual QA for the login screen has not passed or been explicitly waived.
```

## What To Remember

Harness should make one task easier to trust, not harder to start. Users speak normally. Agents clarify when needed. User-owned judgments stay with the user. Evidence, verification, manual QA, work acceptance, residual risk, and close readiness stay distinct.

## Where To Go Next

- Read [Concepts](concepts.md) for the minimum vocabulary.
- Read [User Guide](../use/user-guide.md) for practical session behavior.
- Use [Core Model Reference](../reference/core-model.md) only when exact future contracts are needed.
