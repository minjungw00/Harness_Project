# Kernel Spec

## Document Role

하네스 운영 커널 사양.

## Owns

- entity model
- Task / Change Unit / Run / Approval / Evidence / Eval / QA / Artifact / Reconcile Item 개념 계약
- lifecycle model
- gate model
- state compatibility matrix
- transition table
- close semantics
- waiver semantics
- prepare_write decision algorithm의 state-level logic
- close_task decision algorithm의 state-level logic
- invariant enforcement mapping

## Does Not Own

- MCP wire schema
- SQLite DDL column list
- projection template
- design-quality playbook procedure
- connector capability details
- capability as a first-class kernel gate

## Sections

### Kernel Scope

TODO_REWRITE: Define the implementable kernel boundary from `docs/legacy-v1/02-strategy.md` and `docs/legacy-v1/04-reference-implementation.md`.

### Entity Model

TODO_REWRITE: Migrate entity contracts for Task, Change Unit, Run, Approval, Evidence, Eval, QA, Artifact, and Reconcile Item.

### Lifecycle Model

TODO_REWRITE: Specify lifecycle mode, phase, result, and close reason using the lifecycle plus gates model.

### Gate Model

TODO_REWRITE: Specify scope, approval, design, evidence, verification, QA, and acceptance gates.

### Compatibility Matrix

TODO_CONTENT: Add allowed and impossible combinations after Batch B completes the state model.

### Transition Table

TODO_CONTENT: Add canonical transition table in Batch B.

### Prepare Write State Logic

TODO_REWRITE: Migrate state-level `prepare_write` logic without adding MCP request or response schema.

### Close Task State Logic

TODO_REWRITE: Migrate close semantics, waiver semantics, and result assignment.

### Invariant Enforcement Mapping

TODO_CONTENT: Map the seven approved core invariants to kernel enforcement points.
