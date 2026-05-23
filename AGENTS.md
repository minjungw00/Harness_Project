# Codex Working Rules

This repo is in pre-MVP Harness documentation review / documentation acceptance mode. This file is repository working guidance for documentation work, not Harness runtime guidance.

- Do not implement the Harness server, runtime code, product implementation code, generated operational files, or state/projection/artifact outputs.
- Documentation edits are allowed in this phase.
- When changing meaning, work in `docs/en` first and mirror semantic changes in `docs/ko` in the same batch.
- Maintain semantic parity between English and Korean docs, while allowing natural Korean headings and prose.
- Use the current documentation tree: `docs/*/learn/*`, `docs/*/use/*`, `docs/*/build/*`, `docs/*/reference/*`, `docs/*/maintain/*`, and `docs/*/roadmap.md`.
- Use small batches and report changed files.
- Documentation edits do not require Journey Card, `prepare_write`, Write Authorization, `task_events`, MCP state transitions, evidence, QA, acceptance, residual-risk acceptance, or close.
- Do not create commits unless the user explicitly asks for commits.
