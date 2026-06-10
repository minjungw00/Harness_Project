# Checks

Use these checks after documentation edits and before major review handoff. They are read-only Markdown documentation checks, not runtime checks.

Use `PASS`, `WARN`, and `FAIL` only as docs-maintenance labels. They do not decide documentation acceptance, implementation readiness, runtime conformance, final acceptance, close readiness, residual-risk acceptance, QA, or permission to start server coding.

## 1. What This Checks

These checks look for documentation drift:

- route files that define contracts instead of routing to owners
- broken links, stale paths, and stale route families
- bilingual semantic-parity problems
- Korean prose that reads like literal English
- exact identifiers translated or used as display labels in the wrong context
- strict contracts repeated outside the canonical owner named by [Reference Index](../reference/README.md)
- active/later boundary drift and profile-gated values shown as default active values
- unsupported security claims that overstate the guarantee level documented by the owner
- user-owned judgment routes that substitute for each other
- rendered projections or templates treated as source authority
- one-language-per-`doc_id` retrieval problems
- stale rewrite history, closed issue records, migration notes, and temporary files

## 2. What This Does Not Prove

This page does not prove runtime behavior, runtime conformance, implementation readiness, documentation acceptance, development readiness, final acceptance, close readiness, QA, evidence sufficiency, residual-risk acceptance, or permission to start server coding.

Do not use these checks to create runtime state, `task_events`, generated projections, generated operational artifacts, executable fixtures, conformance reports, QA records, acceptance records, close records, residual-risk records, or product writes.

## 3. Route Check

Inspect README files, Maintain docs, route tables, navigation summaries, paired-language links, and retrieval guidance.

Pass when route files point readers to:

- `docs/doc-index.yaml`
- `docs/*/start.md`
- `docs/*/use/user-guide.md`
- `docs/*/use/agent-guide.md`
- `docs/*/use/judgment-examples.md`
- `docs/*/use/surface-recipes.md`
- `docs/*/build/mvp-plan.md`
- `docs/*/reference/README.md`
- `docs/*/later/index.md`
- `docs/*/maintain/authoring-guide.md`
- `docs/*/maintain/translation-guide.md`
- `docs/*/maintain/checks.md`

Fail when active routing points to deleted files, stale route families, inactive migration records, wrong-language owners, or deep owner files where the compact owner index should be used.

## 4. Owner-Boundary Check

Use [Reference Index](../reference/README.md) to identify the canonical owner for the contract area. Inspect exact schemas, DDL, enum values, state transitions, algorithms, template bodies, storage rules, security guarantees, public errors, and official definitions only enough to verify that they live in that owner.

Pass when each strict contract is defined in one owner and non-owner docs use a short local consequence plus a route. Fail when README, Start, Use, Build, Maintain, examples, or non-owner reference summaries create a second normative definition.

## 5. Active/Later Check

Inspect active docs, examples, and route text for later-only material presented as current MVP scope.

Pass when active material is owned by an active owner, profile-gated values are labeled at the point of use, and later candidates remain in [Later Index](../later/index.md) until promoted. Fail when a later candidate, future operation, profile-only value, or unproved security behavior is described as a default active requirement.

## 6. Bilingual Check

Inspect paired English and Korean files for the same active file map, reader purpose, section coverage, owner routing, active/later boundary, and exact identifiers.

Pass when paired files preserve the same meaning while Korean remains natural. Fail when one language omits a meaning-changing edit, changes an owner route, translates an exact identifier, or turns later material into active scope.

## 7. Korean Quality Check

Inspect Korean explanatory prose, headings, examples, and maintain guidance.

Pass when Korean reads as natural Korean technical documentation, preserves exact identifiers, and avoids English noun phrases unless they are exact identifiers or intentional Harness labels. Fail when Korean preserves English sentence order where natural Korean explanation is required or compresses negative conditions in a meaning-changing way.

## 8. Security Wording Check

Inspect claims using cooperative, detective, preventive, isolated, guard, freeze, careful-mode, sandbox, permission, blocking, tamper-proof, isolation, `surface_id`, capability, local access, or surface binding language.

Pass when the wording routes guarantee meaning to the security owner and does not overstate the documented mechanism. Fail when text implies OS permissions, arbitrary-tool sandboxing, tamper-proof storage, default pre-tool blocking, security isolation, or unverified detective capability without an owner-backed proof path.

## 9. Judgment And Evidence Check

Inspect judgment prompts, examples, close wording, approval wording, final acceptance wording, residual-risk wording, and evidence wording.

Pass when product decisions, technical decisions, scope decisions, sensitive-action approval, final acceptance, residual-risk acceptance, cancellation, and later/reserved judgment routes remain distinct. Fail when broad approval or any one judgment substitutes for another, or when acceptance wording fills missing required evidence.

## 10. Projection And Template Check

Inspect projection, template, status-card, summary, and generated-display wording.

Pass when rendered outputs are described as read-only derived display or support context and exact template bodies live in [Template Bodies](../reference/template-bodies.md). Fail when rendered output is treated as source-of-truth state, evidence, QA, acceptance, Write Authorization, residual-risk acceptance, close result, or product/runtime write permission.

## 11. Retrieval Check

Inspect agent guidance, context-loading advice, README routes, Reference routes, and always-on context examples.

Pass when agent-facing docs retrieve only one language for a given `doc_id` during normal work, load paired languages only for translation or parity review, retrieve only the owner section needed for the next action, and keep context compact.

## 12. Stale Content Check

Inspect maintain docs and route text for past rewrite reviews, closed issue records, obsolete acceptance notes, stale alias history, old localization audits, past translation problem records, or temporary migration plans.

Pass when active docs contain only durable rules and owner-aligned content. Fail when stale history remains as active guidance or when temporary files, archive copies, or migration notes remain after the edit.
