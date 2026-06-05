# Documentation Checks

Use this checklist before final documentation acceptance or a major review handoff. It is a practical docs-maintenance checklist for Markdown documentation only: a read-only documentation quality profile.

This checklist is not a runtime conformance suite. It does not run fixtures, seed runtime state, compare runtime state/events/artifacts/projections/errors, append `task_events`, create artifacts, refresh projections, create generated operational artifacts, create conformance reports, create QA or acceptance state, record evidence, record QA, record Acceptance, record Residual Risk, affect close readiness, close work, or prove implementation readiness.

Docs-maintenance `PASS`, `WARN`, and `FAIL` labels may help manual review decide what to inspect or edit next. They are not manual acceptance, final acceptance, close readiness, implementation readiness, or runtime fixture results.

Runtime conformance is separate. It applies only to implemented Core/API/storage/surface behavior and is judged by executable fixtures and state assertions, not documentation prose. No runtime conformance result should be implied before runtime implementation and materialized fixture suites exist.

## Review Types

Use these labels when reporting a check result.

| Review type | Meaning |
|---|---|
| `manual` | A reviewer must make the judgment. Search tools may collect candidates, but a script-only pass is not enough. |
| `scriptable` | A local documentation script or parser can check the stated condition directly. A reviewer still handles documented exceptions. |
| `future-runtime-only` | Only a future runtime implementation and its future proof path can validate the behavior. Current documentation review can only check that the docs do not overclaim it. |

## Result Meanings

Use these meanings for every item, including the final pre-implementation consistency checklist.

| Result | Meaning |
|---|---|
| `PASS` | The documentation is internally consistent for that item, and owner links point to the expected source. `PASS` does not mean document acceptance, final acceptance, implementation readiness, development readiness, runtime conformance, or permission to start server coding. |
| `WARN` | Manual attention is needed, but the reviewer has not confirmed a contract contradiction. Use this for ambiguous wording, stale-looking routes, owner text that may need classification, or Korean/English wording that needs review. |
| `FAIL` | The docs contain a contradiction, duplicate owner conflict, or active/later boundary break for that item. Report it as docs-maintenance only and route it to the owner or stage; it does not create runtime state, acceptance status, or readiness status. |

This checklist does not decide documentation acceptance. Maintainers decide documentation acceptance and implementation-planning readiness manually in the Build handoff owners.

## Final Pre-Implementation Consistency Checklist

Use this as a final manual sweep before maintainers consider documentation acceptance or implementation-planning readiness. It is a manual aid, not a gate result. For each row, inspect the active English and Korean docs, the owner docs, and entrypoint summaries. A `PASS` means only that the documentation is internally consistent for that item; it does not mean the documentation is accepted, implementation-ready, development-ready, or runtime-proven.

| Check | Review type | Owner routes | `PASS` | `WARN` | `FAIL` |
|---|---|---|---|---|---|
| Write Authorization status set is identical everywhere. | `manual`, with `scriptable` candidate search. | [Core Model Reference](../reference/core-model.md#write-authorization), [MVP API](../reference/api/mvp-api.md#harnessprepare_write), [API Schema Core](../reference/api/schema-core.md#evidence-and-pre-write-scope-schemas), [Storage](../reference/storage.md#storage-validation-and-enum-hardening). | Every active mention uses the owner status set, and `prepare_write.decision` values are not confused with `write_authorizations.status`. | A summary uses loose allowed/denied wording but links to the owner and does not introduce a new lifecycle value. | A doc adds, removes, or renames a durable authorization status, treats `blocked` as a persisted authorization status, or makes Core/API/Storage disagree. |
| A blocked write does not create an authorization row. | `manual`, with `scriptable` candidate search. | [Core Model Reference: `prepare_write`](../reference/core-model.md#prepare_write), [MVP API: `harness.prepare_write`](../reference/api/mvp-api.md#harnessprepare_write), [Storage](../reference/storage.md#storage-validation-and-enum-hardening). | Blocked, denied, approval-required, decision-required, state-conflict, and dry-run responses remain response/blocker/error states only; only the owner-allowed non-dry-run path creates a durable Write Authorization row. | A page says "authorization blocked" or similar shorthand, but the surrounding prose and owner link keep the row boundary clear. | Any active doc says a blocked or dry-run write creates a consumable authorization row, replay row, evidence record, close state, or write authority. |
| The active MVP method set is fixed. | `manual`, with `scriptable` candidate search. | [MVP API: MVP-1 method set](../reference/api/mvp-api.md#mvp-1-method-set), [MVP-1 User Work Loop](../build/mvp-user-work-loop.md#main-idea), [API Schema Core](../reference/api/schema-core.md#stage-specific-active-value-sets). | Build, API, Reference, and surface docs keep one fixed owner method set and do not add or remove active MVP methods in summaries. | A tutorial or route names only a subset for a local example while clearly pointing to the owner method set. | A doc treats a later/compatibility method as active MVP, omits an active method from an owner summary, or creates a second active method list with different contents. |
| `harness.next` is not active MVP, and next actions use `harness.status.next_actions`. | `manual`, with `scriptable` candidate search. | [MVP API](../reference/api/mvp-api.md#mvp-1-method-set), [Schema Later: `harness.next`](../reference/api/schema-later.md#harnessnext), [MVP-1 User Work Loop](../build/mvp-user-work-loop.md#main-idea). | Active MVP docs consistently route next safe actions through `harness.status.next_actions`, and `harness.next` stays later/compatibility. | A page mentions "next" informally but does not name `harness.next` as active. | Any active MVP or Build text requires a separate `harness.next` method or treats it as equivalent to the active status path. |
| The active storage slice is implementable. | `manual`. | [Storage](../reference/storage.md#active-first-implementation-storage-slice), [Implementation Overview](../build/implementation-overview.md#implementation-readiness-criteria), [MVP-1 User Work Loop](../build/mvp-user-work-loop.md#storage-docs-needed-for-mvp-1), [Runtime Architecture Reference](../reference/runtime-architecture.md). | MVP storage remains limited to owner-approved active records and avoids requiring later-profile tables, projection jobs, rich reports, or generated documents as authority. | A row or summary needs maintainer classification before the active/later boundary is obvious, but it does not yet contradict the Storage owner. | MVP exit or Engineering Checkpoint wording requires later-profile storage, projection caches, rich Approval/residual-risk tables, full Evidence Manifest storage, or generated Markdown as source of truth. |
| `dry_run`, idempotency, and `state_version` rules match across API, Storage, and Core. | `manual`, with `scriptable` candidate search. | [MVP API: shared request rules](../reference/api/mvp-api.md#shared-request-rules), [API Errors: idempotency](../reference/api/errors.md#idempotency), [API Errors: state conflict behavior](../reference/api/errors.md#state-conflict-behavior), [API Schema Core](../reference/api/schema-core.md#tool-envelope), [Runtime Architecture Reference](../reference/runtime-architecture.md#state-transaction-flow), [Storage](../reference/storage.md#event-and-idempotency-semantics). | Docs agree that dry runs are non-authoritative, committed idempotent replays return the original response without duplicate side effects, and state-version/freshness wording follows the owner clocks. | A summary omits one of the details but points to the owner and does not assert a conflicting rule. | A doc says dry-run reserves an idempotency key or creates current records, replays recompute side effects, stale `expected_state_version` can be accepted as current, or `basis_state_version` and response `state_version` are treated as the same value. |
| Evidence sufficiency and close blockers are clear. | `manual`. | [Core Model Reference: close](../reference/core-model.md#close_task), [MVP API: `harness.close_task`](../reference/api/mvp-api.md#harnessclose_task), [API Schema Core: evidence and pre-write scope schemas](../reference/api/schema-core.md#evidence-and-pre-write-scope-schemas), [Design Quality Policies](../reference/design-quality-policies.md#active-mvp-blocking-set). | Evidence sufficiency, missing evidence, unresolved judgment, QA/verification status, final acceptance, residual-risk visibility, and close blockers remain separate and visible. | A reader-facing summary is compact enough that a reviewer should confirm the owner path before accepting the wording. | Tests, screenshots, generic summaries, final acceptance, QA waiver, projection prose, or status text are described as automatically satisfying evidence sufficiency or close readiness. |
| Broad approval does not replace product judgment. | `manual`. | [Agent Guide: Request user judgment narrowly](../use/agent-guide.md#5-request-user-judgment-narrowly), [Decision Packet Cookbook](../use/decision-packet-cookbook.md), [API Schema Core: `UserJudgment`](../reference/api/schema-core.md#userjudgment), [Glossary Reference: Approval](../reference/glossary.md#approval). | Broad phrases such as "go ahead" or "looks good" do not silently become product, technical, scope, QA waiver, final-acceptance, or residual-risk judgments. | An example is conversationally broad but immediately narrows which one judgment it resolves. | A doc treats generic approval as product judgment, technical judgment, scope expansion, QA waiver, final acceptance, residual-risk acceptance, or cancellation. |
| Sensitive approval does not replace product decision. | `manual`. | [API Schema Core: `UserJudgment`](../reference/api/schema-core.md#userjudgment), [Core Model Reference: Approval Gate](../reference/core-model.md#approval-gate), [Glossary Reference: Approval](../reference/glossary.md#approval). | `sensitive_approval` remains permission for a named sensitive action and does not decide product behavior, architecture, UX, scope, correctness, final acceptance, or risk acceptance. | A sensitive-action example also discusses a product choice, but it asks the product choice separately. | A dependency install, secret access, deployment, destructive write, or similar sensitive-action approval is described as deciding the product/technical direction by itself. |
| Final acceptance does not replace evidence or residual-risk visibility. | `manual`. | [Core Model Reference: Acceptance Gate](../reference/core-model.md#acceptance-gate), [Core Model Reference: Evidence, verification, QA, final acceptance, and risk](../reference/core-model.md#evidence-verification-qa-final-acceptance-and-risk), [API Schema Core: current-position display schemas](../reference/api/schema-core.md#current-position-display-schemas), [Glossary Reference: Acceptance](../reference/glossary.md#acceptance). | Final acceptance remains the user's result judgment after the close basis is visible; it does not create evidence, hide evidence gaps, erase known residual risk, or accept risk unless the residual-risk path asks that judgment. | A page says "accept the result" in plain language but nearby text keeps evidence and risk separate. | Final acceptance is described as sufficient evidence, verification, Manual QA, residual-risk acceptance, or close readiness when blockers remain. |
| Design-quality policy cannot create an endless planning loop. | `manual`. | [Design Quality Policies: Active MVP blocking set](../reference/design-quality-policies.md#active-mvp-blocking-set), [Impact classes and allowed routes](../reference/design-quality-policies.md#impact-classes-and-allowed-routes), [Core Model Reference: Design Gate](../reference/core-model.md#design-gate). | Active MVP design-quality findings route to the small Core-backed blocking set or to one bounded user judgment, evidence request, residual-risk marker, advisory next action, or no action; the broad catalog cannot keep ordinary work in indefinite planning. | A policy page needs clearer exit wording, but it still routes findings through an owner impact class. | A doc makes broad design-quality review, stewardship, TDD, Manual QA, or context-hygiene catalog completion a default requirement before ordinary write/close, without an owner-promoted activation rule or bounded route. |
| Cooperative/detective guarantees are not overclaimed as preventive/isolated. | `manual`; actual enforcement proof is `future-runtime-only`. | [Security Reference: Honest guarantee display](../reference/security.md#honest-guarantee-display), [Runtime Architecture Reference: guarantee level behavior map](../reference/runtime-architecture.md#guarantee-level-behavior-map), [Agent Integration Reference: guarantee levels](../reference/agent-integration.md#guarantee-levels). | Cooperative and detective wording stays honest; preventive or isolated claims name the exact mechanism, covered operation, owner, and proof status. | Friendly wording such as guard, freeze, or careful mode needs review but does not claim OS blocking or isolation. | A cooperative/detective path is described as OS permission, arbitrary-tool sandboxing, tamper-proof storage, universal pre-tool blocking, or security isolation without a proven owner path. |
| Reference surface capability profile matches displayed guarantees. | `manual`, with `scriptable` candidate search. | [Agent Integration Reference: capability profiles](../reference/agent-integration.md#capability-profiles), [Surface Cookbook: reference local surface](../reference/surface-cookbook.md#reference-local-surface), [Security Reference](../reference/security.md#honest-guarantee-display). | The reference `capability_profile` fields support the guarantee displayed to users, and unsupported capabilities lower or block claims instead of expanding authority. | A surface example omits a field but preserves the owner link and does not claim a stronger guarantee. | A surface name, connector label, or capability label grants write authority, hides an unsupported capability, or displays preventive/isolated guarantees that the profile cannot prove. |
| Artifact redaction, hash, and path validation rules are consistent. | `manual`, with `scriptable` candidate search. | [API Schema Core: `ArtifactRef`](../reference/api/schema-core.md#artifactref), [Storage: artifact and evidence boundary](../reference/storage.md#artifact-and-evidence-boundary), [Operations And Conformance: artifacts check](../reference/operations-and-conformance.md#artifacts-check). | Artifact refs, storage rows, operations checks, and evidence summaries agree on owner relation, integrity metadata, hash mismatch, redaction/omission/block behavior, and staged-path validation. | A display summary is shorter than the owner rule but does not permit unsafe raw content, arbitrary paths, or missing integrity facts. | A doc accepts arbitrary absolute paths or parent traversal as committed evidence, treats `hash_mismatch` as cosmetic, stores raw secrets/PII where redaction requires omission/blocking, or claims omitted/blocked bytes can be recovered from Harness. |
| Conformance fixtures assert Core state, not just rendered prose. | `manual`; future execution is `future-runtime-only`. | [Conformance Fixtures Reference](../reference/conformance-fixtures.md), [Operations And Conformance Reference](../reference/operations-and-conformance.md), [Future Fixtures](../later/future-fixtures.md). | Fixture docs stay future-oriented and say future assertions inspect Core state, storage rows, events when stable, errors, artifact refs, blockers, and guarantee facts; rendered Markdown/prose is never enough. | A behavior example is compact and needs reviewer classification, but it is not called a current runnable suite. | A doc treats rendered Markdown, generated prose, documentation checks, or current examples as runtime conformance results or sufficient fixture pass/fail evidence. |
| Docs-maintenance does not claim runtime readiness. | `manual`, with `scriptable` candidate search. | This page, [Authoring Guide](authoring-guide.md), [Implementation Overview](../build/implementation-overview.md#documentation-acceptance-status). | Documentation checks remain read-only Markdown review aids; `PASS`/`WARN`/`FAIL` do not accept docs, prove runtime behavior, update handoff status, or authorize implementation. | A route mentions "check before handoff" but also states the maintainer decision boundary. | A checklist, status table, README, or Build page says docs-maintenance results make the project accepted, implementation-ready, development-ready, close-ready, or runtime-conformant. |
| Korean/English contract terminology has semantic parity. | `manual`, with `scriptable` exact-identifier search. | [English Translation Guide](translation-guide.md), [Korean Translation Guide](../../ko/maintain/translation-guide.md), [Glossary Reference](../reference/glossary.md). | Paired English/Korean docs preserve meaning, owner links, active/later boundaries, exact identifiers, API/schema names, enum values, error codes, validator IDs, and natural Korean prose. | Korean style or phrasing needs human polish, but the contract meaning and identifiers remain intact. | A Korean/English pair changes a contract meaning, translates an exact identifier, moves active material into later or later material into active, or uses a different owner route. |

## Checklist

### Link Check

- Review type: `scriptable`.
- What to inspect: Relative Markdown links, README routes, paired-language links, owner-section links, and heading anchors in active docs.
- Common failure examples: A link points to a moved file. An anchor still uses an old heading. An English page links to a Korean-only anchor by accident. A README route points to a deleted or inactive page.
- Pass means: Every relative link and anchor resolves to an active document or to a clearly documented exception. Owner links point to the current owner document or owner section.

### Term Check

- Review type: `manual`.
- What to inspect: Learn and Use pages, examples, headings, summaries, and status text for internal labels used as default user-facing language.
- Common failure examples: A user-facing page opens with `Discovery`, `Change Unit`, `Decision Packet`, `Write Authorization`, `Evidence Manifest`, `Projection`, `Gate`, or `task_events` before explaining the ordinary user situation. A user example implies the user must say an internal label to get help.
- Pass means: User-facing prose starts from normal user language. Internal labels appear only when they help explain a visible boundary, blocker, record, API, template, or Reference link.

### Stage Check

- Review type: `manual`.
- What to inspect: MVP-1, Engineering Checkpoint, Kernel Smoke, Assurance Profile, Operations Profile, Later, and Roadmap wording in Build, Reference, Use, and Roadmap docs.
- Common failure examples: A Roadmap candidate is written as an MVP-1 requirement. `Kernel Smoke` is treated as a stage. Later-profile export, reporting, operations, or conformance-runner material is required for the smallest runnable slice.
- Pass means: Engineering Checkpoint stays an internal authority-loop smoke. MVP-1 User Work Loop stays the first user-value milestone. Later-profile and Roadmap material remains future scope unless an owner document has promoted it with scope, fallback behavior, and proof expectations.

### Status Check

- Review type: `manual`.
- What to inspect: Entrypoints, handoff sections, Build docs, Maintain docs, and any prose that could imply current implementation status.
- Common failure examples: A page says the Harness Server already exists in this repo. Documentation acceptance is treated as server-coding authorization. Reference design prose is framed as implemented runtime behavior without a future or design boundary.
- Pass means: Docs describe the current repo as documentation-only, in post-redesign review, and not implementation-ready unless the maintainer handoff owner explicitly says so. Intended future behavior is distinguishable from implemented behavior.

### Security Wording Check

- Review type: `manual` for documentation wording. Actual preventive or isolated enforcement proof is `future-runtime-only`.
- What to inspect: Claims using cooperative, detective, preventive, isolated, guard, freeze, careful-mode, sandbox, permission, blocking, tamper-proof, or isolation language.
- Common failure examples: Write Authorization is described as OS permission, sandboxing, tamper-proof enforcement, preventive blocking, or isolation. A connector is said to block arbitrary tool calls without a proven blocking path. A security boundary implies broader OS isolation than the owner document supports.
- Pass means: Each claim matches the documented guarantee level. Cooperative and detective surfaces do not claim preventive control. Preventive or isolated claims name the exact covered operation, mechanism, owner document, and proof status, or remain clearly future-oriented.

### User-Language Check

- Review type: `manual`.
- What to inspect: Openings, examples, commands users might say, status explanations, judgment prompts, close explanations, and recovery text in user-facing docs.
- Common failure examples: A Use page starts with a record taxonomy instead of what the user can ask. A judgment prompt leads with `Decision Packet` rather than the choice and consequence. A status view explanation says `ProjectionKind` before explaining the visible summary.
- Pass means: User docs begin with ordinary tasks, questions, visible blockers, needed judgments, available evidence, or close outcomes. Internal labels are introduced after the reader already knows what problem the label helps solve.

### Mermaid Check

- Review type: `scriptable` for syntax where a Mermaid parser is available; `manual` for usefulness.
- What to inspect: Mermaid fenced code blocks, nearby explanatory prose, diagram labels, and consistency with owner prose.
- Common failure examples: A diagram has Mermaid syntax that cannot render. A diagram is decorative but does not clarify a relationship, sequence, boundary, or lifecycle. A diagram contradicts the surrounding prose or owner contract.
- Pass means: Diagrams are syntactically reasonable, renderable in the expected docs toolchain, and useful enough to reduce reader effort. Nearby prose explains what to notice.

### Bilingual Check

- Review type: `manual`.
- What to inspect: English/Korean active file map, paired file purpose, section coverage, owner links, stable identifiers, exact code-like strings, and Korean prose quality.
- Common failure examples: A Korean page omits a section added in English. A path, enum value, error code, validator ID, or API name is translated or changed. Korean prose becomes English technical nouns joined by Korean particles. A paired link points to a different owner.
- Pass means: English and Korean docs preserve the same meaning, coverage, owner routing, and exact identifiers. Korean headings and prose may differ when they remain natural and semantically aligned.

### Owner Check

- Review type: `manual`.
- What to inspect: Strict contracts, schemas, DDL, enum values, state transitions, gate rules, algorithms, fixture body shapes, template bodies, storage rules, security guarantees, and official definitions.
- Common failure examples: A Use page repeats a full gate matrix. A Build page defines an enum table owned by Reference. A Maintain page gives a second normative definition of projection freshness. A glossary definition is copied and changed outside the Glossary owner.
- Pass means: Each strict contract is defined in one owner document. Non-owner docs use a short reader-facing summary, the local consequence, and an owner link.

### Repair-Target Owner Map Check

- Review type: `manual`.
- What to inspect: The known pre-implementation repair axes in [Authoring Guide: Pre-implementation repair target owner map](authoring-guide.md#pre-implementation-repair-target-owner-map), including owner contract, API/schema, Storage/DDL, Core transition, stage/profile, evidence/close, security/local-access, conformance proof, user-output/context, and design-quality drift.
- Common failure examples: A later-profile API branch is written as an MVP requirement. A status card is treated as gate authority. A design-quality validator becomes a blocker outside its owner activation rule. Documentation checks are described as runtime conformance. Security wording claims pre-tool blocking without a proven owner path.
- Pass means: Each observed repair axis routes to the canonical owner family, and non-owner docs keep only a short local summary plus owner link. Listed `FAIL` symptoms are reported as docs-maintenance failures only; this check does not decide documentation acceptance, manual acceptance, runtime conformance, or implementation readiness.

### Projection/State Check

- Review type: `manual`.
- What to inspect: Language about projections, rendered templates, Markdown status views, generated documents, state, artifacts, evidence, QA, Acceptance, close readiness, and operational truth.
- Common failure examples: A rendered Markdown view is called canonical state. A documentation file is treated as a runtime object, generated projection, evidence record, QA record, Acceptance record, Residual Risk record, close record, or operational artifact. A projection is described as gate authority.
- Pass means: Rendered views and generated documents are described as derived display. Future operational authority remains with Core-owned local state and artifact references, as owned by the relevant Reference docs.

### Template-Scope Check

- Review type: `manual`.
- What to inspect: Template references, projection/template pages, Use docs, Build docs, Later docs, and Roadmap items that mention future templates or rendered outputs.
- Common failure examples: A later-profile export template is required for MVP-1 close. Future template bodies are treated as current MVP requirements. A user-facing page duplicates full template bodies instead of linking to the Template Reference owner.
- Pass means: Future templates remain future or later-profile material unless an owner promotes them. Active MVP requirements name only the templates and rendered views required by the active stage. Full template bodies stay with the Template Reference owner.
