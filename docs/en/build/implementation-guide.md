# Implementation guide

This guide gives implementers a stable route from Harness product scope to the owner documents that define runtime behavior. It is a reading guide for implementation work, not a product contract.

This guide does not define baseline scope, API behavior, schemas, storage effects, security guarantees, runtime locations, connector behavior, conformance authority, or example validity. Those contracts stay in the Reference owners.

## Baseline implementation reading path

Read owner documents before encoding behavior.

Use this path when interpreting baseline implementation work:

1. Confirm the baseline scope and supported behavior boundary in [Scope](../reference/scope.md).
2. Use the [Reference Index](../reference/README.md) to choose the applicable owner for each contract question.
3. Read [Core Model](../reference/core-model.md) for authority concepts that cross APIs, storage, and close readiness.
4. Use [API Methods](../reference/api/methods.md) for the supported public method list and method-owner routing.
5. Read the relevant method owner, schema owner, storage owner, [Runtime Boundaries](../reference/runtime-boundaries.md), [Security](../reference/security.md), and [Conformance](../reference/conformance.md) together when supported behavior crosses those areas.
6. Use [Agent Integration](../reference/agent-integration.md) and [Surface Recipes](../use/surface-recipes.md) only for the surface or connector boundary they own.
7. Keep user-owned judgment, evidence, verification expectations, acceptance, close readiness, and residual risk as separate authority concepts.

## Baseline implementation interpretation

[Scope](../reference/scope.md) is the baseline scope gate. A capability is implementable as baseline behavior only when Scope includes it and the applicable owners define the supported behavior, shape, storage, runtime, security, and conformance detail the implementation needs.

Do not infer supported behavior from value names, examples, route summaries, or schema vocabulary. Use Scope for the baseline scope boundary and the relevant owners for exact method behavior, fields, effects, guarantees, and assertions.

## Contract owner combinations

Most implementation work needs more than one owner. Start from the owner closest to the question, then add the neighboring owners that define the shape, effect, or guarantee.

| Implementation question | Read together |
|---|---|
| Is this capability in baseline scope? | [Scope](../reference/scope.md), then the applicable semantic owner from the [Reference Index](../reference/README.md) |
| Which public method exists? | [API Methods](../reference/api/methods.md) and [API Value Sets](../reference/api/schema-value-sets.md) |
| What does one method do? | The method owner listed by [API Methods](../reference/api/methods.md), plus [API Schema Core](../reference/api/schema-core.md) for shared envelopes and response branches |
| Which fields and nested shapes are valid? | [API Schema Core](../reference/api/schema-core.md), [API State Schemas](../reference/api/schema-state.md), [API Artifact Schemas](../reference/api/schema-artifacts.md), [API Judgment Schemas](../reference/api/schema-judgment.md), and [API Value Sets](../reference/api/schema-value-sets.md) as applicable |
| Which public errors or close-readiness blocker routes are valid? | [API Error Family Index](../reference/api/errors.md), [API Error Codes](../reference/api/error-codes.md), [API Error Precedence](../reference/api/error-precedence.md), [API Error Routing](../reference/api/error-routing.md), [API Blocker Routing](../reference/api/blocker-routing.md), [API Error Details](../reference/api/error-details.md), plus the relevant method and state-schema owners |
| What changes in storage? | [Storage Effects](../reference/storage-effects.md) first, then [Storage](../reference/storage.md), [Storage Records](../reference/storage-records.md), [Artifact Storage](../reference/storage-artifacts.md), or [Storage Versioning](../reference/storage-versioning.md) as applicable |
| Where do product files, server files, and runtime data live? | [Runtime Boundaries](../reference/runtime-boundaries.md), with storage owners for data detail |
| What security wording or guarantee level is valid? | [Security](../reference/security.md), with [Scope](../reference/scope.md) for supported availability and [API Value Sets](../reference/api/schema-value-sets.md) for exact value names |
| What should a conformance check assert? | [Conformance](../reference/conformance.md), then the API, schema, storage, security, runtime, and Core owners that make each asserted fact authoritative |
| How should a surface or connector behave? | [Agent Integration](../reference/agent-integration.md), [Surface Recipes](../use/surface-recipes.md), and the relevant API/security owners |
| What can a read-only display show? | [Projection Authority](../reference/projection-and-templates.md), [Template Bodies](../reference/template-bodies.md), and the state/schema owners for source facts |

When owners appear to disagree, do not resolve the mismatch in implementation code. Treat it as an owner gap: Scope gates supported availability, method owners define method behavior, schema owners define shapes, storage owners define effects, Runtime Boundaries define locations, Security defines guarantee wording, error owners define public error meanings and routing, and Conformance defines assertion authority.

## Use documents and reference contracts

Use documents explain workflows, reader decisions, and expected outcomes. They are useful for understanding how a user or agent should move through Harness, but they do not override Reference contracts.

Implementers may use [User Guide](../use/user-guide.md), [Agent Guide](../use/agent-guide.md), [Judgment Examples](../use/judgment-examples.md), and [Surface Recipes](../use/surface-recipes.md) to understand reader intent, surface behavior, and judgment boundaries. For API payloads, storage effects, security guarantees, close-readiness rules, access boundaries, or error behavior, route back to the applicable Reference owner.

If a use document and a Reference owner seem to differ, implement the Reference owner and report the route or guide mismatch as documentation maintenance work.

## Out-of-scope behavior

Do not implement an excluded capability because it is named in Scope, examples, conformance scenario IDs, schema value sets, or route summaries. A name may be vocabulary or reserved surface area without being supported behavior.

An out-of-scope capability becomes implementable only after [Scope](../reference/scope.md) and the relevant owners define a supported contract. Until then, implementation code should reject, ignore, or avoid the behavior according to the applicable owners.

## Conformance scenarios

[Conformance](../reference/conformance.md) owns documentation-level conformance meaning, assertion authority, and compact scenario routing. Use scenarios as coverage prompts only; bind every assertion to an owner-defined fact before writing a test or check.

Do not treat scenario prose, generated summaries, rendered reports, documentation-check labels, or status display text as runtime authority.

## Examples as implementation inputs

Examples are reading aids, not complete schemas or behavior sources. Use them to understand a representative branch, scenario, or compact request/response shape.

Do not infer fields, optionality, storage effects, security guarantees, out-of-scope behavior, or implementation shortcuts from examples. If an example conflicts with a method, schema, storage, security, runtime, conformance, or error owner, the relevant owner wins.

## Small baseline build shape

A small baseline build can carry one ordinary user task by following [Scope](../reference/scope.md) for included capabilities and the relevant owners for exact requests, responses, storage effects, errors, blockers, security wording, and conformance assertions.

This is an implementation shape, not a separate contract.

## Repository boundary

Runtime state, generated artifacts, evidence outputs, QA results, acceptance decisions, close-readiness state, residual-risk decisions, fixture outputs, and product implementation files are not stored in this documentation tree.

Path allowlists, route tables, and documentation batch boundaries in these docs are maintainer editing controls for the documentation set. They are not Harness runtime permissions, write authorizations, sandbox guarantees, or proof of enforcement.
