# Agent Working Rules

These rules apply to agents and maintainers changing this repository, including maintained Harness documentation and implementation files. They are repository working guidance only. They do not define Harness runtime behavior, public API behavior, storage effects, security guarantees, schemas, Core authority semantics, conformance results, QA results, acceptance decisions, residual-risk decisions, or implementation output.

Harness is the local work-authority product/system for AI-assisted product work. Core is the local authority record for Harness state. Documented Harness runtime principles are implementation targets owned by maintained documents, not rules for how repository edits are executed. No functioning Harness runtime is assumed by this file.

## Scope And Stop Conditions

- Use these rules for repository edits, review, validation, and final reporting.
- Keep `AGENTS.md`, README files, Maintain pages, and route documents from becoming Harness runtime contracts, API contracts, storage contracts, security contracts, Core authority contracts, or implementation logs.
- Stop and report before broad edits if `docs/doc-index.yaml` is missing or malformed enough that the applicable shared or paired document entry cannot be identified.
- Stop and report if the repository structure no longer matches the maintained shape described by `AGENTS.md`, `docs/doc-index.yaml`, `docs/en`, and `docs/ko`.
- Stop and report if the requested change would require defining API behavior, storage behavior, schema meaning, security guarantees, or Core authority semantics directly in `AGENTS.md` or another non-owner route.

## First Reads

- Read this `AGENTS.md` before changing repository documentation or implementation files.
- Use `docs/doc-index.yaml` as the canonical machine-readable owner route. It owns `doc_id`, paired paths, role, owner scope, non-owner scope, dependencies, normative level, and audience metadata. It is maintenance metadata only, not runtime configuration or product contract data.
- Use `docs/terminology-map.yaml` as the terminology and identifier-preservation source of truth.
- For English-facing documentation edits, read `docs/en/maintain/authoring-guide.md`.
- For Korean-facing documentation edits, read `docs/ko/maintain/authoring-guide.md`.
- For bilingual edits or terminology-affecting edits, read both translation guides, `docs/terminology-map.yaml`, and the relevant glossary entries.
- For implementation work, start with `docs/en/build/implementation-guide.md` or `docs/ko/build/implementation-guide.md`, then follow `docs/doc-index.yaml` and the Reference owners for the affected behavior.
- For public API work, use `docs/en/reference/api/methods.md` and `docs/ko/reference/api/methods.md` for the supported public method list and method-owner routing.
- After documentation edits, use the relevant check guidance under `docs/*/maintain/checks.md`.
- After Rust implementation edits, inspect the Cargo workspace or crate layout before choosing validation commands.

## Owner Lookup And Contract-First Changes

README files, Start pages, Use pages, Build pages, Maintain pages, and reference indexes are route documents. They help readers choose the next document; they do not define API behavior, storage effects, security guarantees, schemas, close-readiness contracts, Core authority semantics, or detailed owner maps.

For exact owner lookup, read `docs/doc-index.yaml` first. Use the human-readable `docs/*/reference/README.md` index when a reader-facing reference route is useful, but do not copy its owner map into `AGENTS.md`, README files, or maintain guidance.

When a documentation change affects normative meaning, edit the canonical owner document selected from `docs/doc-index.yaml`. This includes baseline scope, API behavior, schema meaning, storage effects, security wording, access boundaries, close readiness, product terminology, out-of-scope promotion rules, and value-set meaning.

When an implementation change needs API behavior, storage behavior, schema shape, security wording, runtime boundary, error behavior, scope support, or Core authority semantics that are missing or unclear, update the owning maintained document before encoding that behavior only in code, tests, fixtures, CLI help, adapter behavior, or comments. If no applicable owner exists, name the owner gap or route to the closest applicable owner instead of filling the gap in a non-owner document.

If implementation and documentation appear to disagree, treat that as an owner-routing or implementation gap to resolve. Do not infer a new contract from existing code, examples, logs, generated output, or route metadata.

## Language And Terminology

English and Korean documentation are both maintained. Neither language is an archive, appendix, or translation-only copy.

For ordinary lookup, read the language that matches the request or the default language in `docs/doc-index.yaml`. Read both paired documents when doing bilingual editing, translation review, parity review, or terminology work.

Do not finish a meaning-changing documentation batch with only one language updated when the changed document has a maintained paired path. Preserve the same reader purpose, normative strength, owner routing, baseline and out-of-scope boundaries, user-judgment boundary, and security guarantee level by meaning unit.

Korean documentation must use natural Korean technical prose. Preserve exact identifiers, file paths, API methods, schema names, field names, enum values, status values, product labels, anchors, and code literals exactly as written, with backticks where clarity or searchability requires them.

Use the terminology map's Harness/Core distinction:

- Harness is the local work-authority product/system for AI-assisted product work.
- Core is the local authority record for Harness state.

## Documentation Editing

- Keep route documents short and navigational. If a route page starts to need field tables, status-value tables, storage-effect detail, error behavior, or guarantee levels, move that content to the applicable owner.
- Keep user-owned judgments distinct from Core-owned state and artifact authority. Evidence, verification criteria, QA, acceptance, waiver, and residual-risk boundaries must not collapse into one broad approval.
- Keep baseline behavior separate from reserved, profile-gated, and out-of-scope material. Do not describe out-of-scope capabilities as baseline requirements.
- Match guard, freeze, careful-mode, and security wording to the guarantee level documented by the security owner.
- Use stable product or user scenarios in examples. Do not make documentation maintenance, route reshaping, or section restructuring the API example scenario unless the document is specifically about documentation maintenance.
- Keep API examples internally consistent across request data, visible response state, `state_version`, refs, paths, artifact refs, run refs, judgment refs, sensitive approval reasons, and close-readiness evidence.
- Preserve exact identifiers in prose, tables, examples, and route metadata.
- Treat `docs/en/build/implementation-guide.md` and `docs/ko/build/implementation-guide.md` as implementation reading paths and interpretation guides. They may route implementation questions to owner documents and describe guide-level supported implementation shape; they do not own product, API, storage, security, scope, schema, error, or Core authority semantics.
- Do not put implementation logs, PR notes, migration records, or one-off decision records in maintained documentation. README files and route pages also stay short and navigational; contract-changing details belong in the relevant Reference owner.
- Treat path allowlists and documentation batch boundaries as maintainer editing controls, not Harness runtime override capabilities.
- Use durable maintenance wording. Avoid task-specific, PR-specific, or short-lived wording in maintained documentation.

## Implementation Rules

- Keep implementation code, tests, fixtures, and build configuration in ordinary implementation paths selected by the repository structure. Do not put product implementation code under `docs/`, and do not use maintained documentation files as implementation logs.
- Implement contract-first. If code needs behavior that the maintained owners do not define, update the applicable owner documents first or report the owner gap.
- Core implementation must not depend on CLI or MCP adapter layers. CLI and MCP adapters may call into Core-facing interfaces; Core-facing code must stay independent of those adapters.
- Public Harness API methods are limited to the documented public method list in `docs/*/reference/api/methods.md`. Admin CLI commands are not public API methods.
- Do not add or expose a new public API method, request field, response field, storage effect, error meaning, security guarantee, or Core authority rule solely in code. Route the contract change to the applicable maintained owner first.
- Use examples as reading aids, not as complete schemas or behavior sources. Implementation decisions come from the focused owners for scope, methods, schemas, storage, security, runtime boundaries, errors, blockers, and conformance.
- Keep user-owned judgment, evidence, verification criteria, ordinary approval, write approval, sensitive-action approval, `Write Authorization`, final acceptance, close readiness, and residual-risk acceptance distinct in code, tests, and documentation.
- Keep tests aligned to owner-defined facts. A test fixture or assertion should not become the only place a product contract is defined.

## Storage, Runtime Output, And Local Data

- Maintained documentation under `docs/`, shared metadata, README files, and `AGENTS.md` are not Harness runtime homes and are not places for generated runtime state.
- Do not store runtime data, generated logs, SQLite files, product runtime homes, test runtime homes, generated projections, fixture output, QA results, acceptance records, close-readiness state, residual-risk records, or work notes in maintained documentation.
- For local test runs, use the Cargo build output, another ignored test-output location already used by the repository, or `/tmp`. If a test needs a runtime home, point it at a disposable per-test path, not at maintained documentation, shared metadata, or user product data.
- Do not add persistent output directories, generated records, or local runtime homes to the repository unless the user asks for a durable implementation artifact and the repository's ignore rules and documentation owners support it.
- If a tool creates generated output during editing or validation, remove it before finishing unless it is ordinary ignored build output.

## Scratch Notes And Work Records

Keep planning in the conversation unless the user explicitly asks for a maintained documentation artifact. Scratch notes, archive copies, conversion notes, unresolved review notes, generated runtime records, implementation logs, PR notes, migration records, and work logs do not belong in maintained documentation.

If a user asks for a maintained planning document, place it only in an appropriate maintained documentation path and make sure it has durable reader value.

## Validation

After documentation edits, run or perform the checks that match the changed files. For route and entry changes, include structure, links/indexes, terminology, and language parity checks when applicable.

After Rust implementation edits, run the applicable Rust validation from the workspace or changed crate:

- `cargo fmt`
- `cargo clippy --all-targets --all-features`
- `cargo test --all-targets --all-features`

Use narrower Cargo commands only when the repository structure or task scope clearly calls for them, and report the reason. If a validation command cannot run because the relevant workspace, crate, toolchain, dependency, or network access is unavailable, report that as skipped validation with the reason.

Before finishing, confirm changed links, file paths, anchors, paired-language links, owner routing, and terminology. Confirm no scratch files, archive copies, generated records, runtime homes, SQLite files, generated logs, or work notes remain from the edit.

## Reporting

Final reports should be concise and should stay in the conversation, not in repository files. Include changed files, the summary of changes, validation performed and results, skipped validation with reasons, and remaining risks or follow-up items.

Treat validation results as repository maintenance or implementation-check results only. Do not describe them as Harness runtime conformance, product acceptance, QA completion, close readiness, security proof, or residual-risk acceptance.
