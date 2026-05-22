# Translation Guide

## Purpose

Use this guide when editing English and Korean Harness documentation together.

The goal is semantic parity, not sentence-by-sentence translation. Korean should read like natural technical Korean while preserving official identifiers, exact contracts, and product terms.

## Keep exact

Keep these unchanged across English and Korean docs:

- API names
- schema names
- enum values
- DDL names
- code identifiers
- file names and path names
- error codes and validator IDs

Keep these stable product terms exact when they refer to Harness concepts:

- Task
- Change Unit
- Decision Packet
- Write Authorization
- Evidence Manifest
- ProjectionKind
- MCP
- Core
- state.sqlite
- task_events
- prepare_write
- record_run
- close_task

## Translate naturally

Use the term that fits the sentence and reader context.

| English term | Korean guidance |
|---|---|
| context | Use `context` in identifier-like or AI-session phrasing. Use `맥락` in ordinary prose. |
| boundary | Keep `boundary` in code or identifier context. Use `경계` in prose. |
| authority | Use `권한` for operational authority. Use `기준 권한` when the sentence needs to stress the source of authority. |
| canonical | Keep `canonical` in identifier context. Use `기준` or `기준 기록` in Korean prose. |
| mutate | Prefer `change` or `modify` in English prose. Use `변경하다` in Korean. |
| surface | Choose the concrete meaning: `interface`, `view`, `entrypoint`, `display area`, or the Korean equivalent by context. |
| evidence | Use `evidence` only when it is a product term. Use `근거` or `증거` in Korean prose. |

## Avoid examples

Avoid mixed-language phrases that preserve English technical words without helping the reader:

- `state를 mutate한다`
- `authority boundary를 preserve한다`
- `surface에 expose한다`
- `projection freshness를 report한다`

## Prefer examples

Prefer natural phrases that preserve the technical meaning:

- `상태를 변경한다`
- `권한 경계를 유지한다`
- `화면에 보여준다`
- `projection이 최신인지 표시한다`

## Korean heading policy

Korean headings should be natural Korean headings, not mechanical English-heading copies.

Keep official identifiers exact inside headings when the heading is about that identifier. Otherwise, choose the heading a Korean technical reader would expect.

Heading order and document meaning should remain aligned with the English document. Heading text does not need to match word for word.

## Bilingual review checklist

```text
[ ] Does the Korean page preserve the same meaning as the English page?
[ ] Does the Korean prose read naturally to a Korean technical reader?
[ ] Are API names, schema names, enum values, DDL names, identifiers, paths, error codes, and validator IDs exact?
[ ] Are stable product terms preserved when they refer to Harness concepts?
[ ] Are mixed-language phrases replaced with natural Korean where possible?
[ ] Are headings idiomatic while preserving the same document structure and scope?
[ ] Were English and Korean link changes made in the same batch?
```
