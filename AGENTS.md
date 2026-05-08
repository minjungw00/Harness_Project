# Codex Working Rules

This repo is in documentation redesign mode. Keep this file as a short operating compass, not a procedure manual, schema reference, or project history.

- Do not implement the harness server or add runtime code yet.
- Work in `docs/en` first; mirror semantic documentation changes in `docs/ko` in the same batch.
- Keep source-of-truth boundaries strict: operational state in `state.sqlite` current records plus `state.sqlite.task_events`, raw evidence in the artifact store, and Markdown projections as derived views.
- Keep MCP schemas in `docs/*/05-mcp-api-and-schemas.md`, SQLite DDL in `docs/*/06-reference-mvp.md`, and projection template bodies in `docs/*/appendix/A-template-library.md`.
- Read current harness status before resuming work; show a Journey Card before significant resume.
- Before product writes, call `prepare_write`; if MCP is unavailable, hold product writes.
- Use Decision Packets, not broad approval, for blocking product judgment.
- Use small batches and report changed files.

## Harness Write Gate

Default:
- Before editing product/runtime/code files, read current Harness status/next and call `harness.prepare_write`.
- If MCP is unavailable, hold product/runtime/code writes.

Pre-MVP docs-authoring exception:
- When this repository is being edited to author the Harness documentation itself before a working Harness MCP is available, the user may grant a one-time `DOCS_AUTHORING_OVERRIDE`.
- The override must list exact allowed paths.
- It permits only documentation edits in those paths.
- It is not `prepare_write`, Write Authorization, evidence, verification, QA, acceptance, residual-risk acceptance, task close, or canonical state transition.
- Do not present a state-derived Journey Card when MCP is unavailable; use “Preflight status.”
- Do not use this exception for runtime code, product implementation code, generated operational files, secrets, destructive writes, or broad refactors.
- If no explicit override is present, hold writes.