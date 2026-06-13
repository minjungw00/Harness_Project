# API methods

## What this document owns

This document is the stable route document for the active API method family. It owns:

- the active public API method list
- method owner routing

## What this document does not own

This document does not own:

- full per-method behavior details, including method-specific required inputs, access requirements, result fields, dry-run behavior, or representative request and response bodies
- common API envelope bodies, response branch schema bodies, or schema field definitions
- state, artifact, judgment, value-set, or error schema definitions
- API example consistency rules or field-name consistency rules
- storage-effect details, storage DDL, storage record layouts, artifact lifecycle, state-version storage rules, or security guarantees
- public error-code semantics
- out-of-scope API methods

## Supported-method boundary

Only methods listed below are supported public API methods routed by this document. A method name not listed here is outside the supported public method family.

Method-specific behavior belongs to the method owner documents. Out-of-scope API or schema capabilities remain outside this method router unless [Scope](../scope.md) and the affected owners define them as active.

<a id="baseline-scope-method-behavior"></a>

## Current API method list

This document owns the supported public method list and routes each method to its behavior owner. Schema shapes, storage effects, and public errors stay with their own owner documents and are linked from the relevant method owner.

<a id="harnessintake"></a>
<a id="harnessupdate_scope"></a>
<a id="harnessstatus"></a>
<a id="harnessprepare_write"></a>
<a id="harnessstage_artifact"></a>
<a id="harnessrecord_run"></a>
<a id="harnessrequest_user_judgment"></a>
<a id="harnessrecord_user_judgment"></a>
<a id="harnessclose_task"></a>

| Method | Owner |
|---|---|
| `harness.intake` | [Intake method](method-intake.md) |
| `harness.update_scope` | [Update-scope method](method-update-scope.md) |
| `harness.status` | [Status method](method-status.md) |
| `harness.prepare_write` | [Prepare-write method](method-prepare-write.md) |
| `harness.stage_artifact` | [Stage-artifact method](method-stage-artifact.md) |
| `harness.record_run` | [Record-run method](method-record-run.md) |
| `harness.request_user_judgment` | [User-judgment method owner](method-user-judgment.md#harnessrequest_user_judgment) |
| `harness.record_user_judgment` | [User-judgment method owner](method-user-judgment.md#harnessrecord_user_judgment) |
| `harness.close_task` | [Close-task method](method-close-task.md) |

<a id="method-owner-routing-table"></a>

## Method owner routing

Use this table for behavior questions about a supported method.

| Method behavior question | Owner |
|---|---|
| `harness.intake` | [Intake method](method-intake.md) |
| `harness.update_scope` | [Update-scope method](method-update-scope.md) |
| `harness.status` | [Status method](method-status.md) |
| `harness.prepare_write` | [Prepare-write method](method-prepare-write.md) |
| `harness.stage_artifact` | [Stage-artifact method](method-stage-artifact.md) |
| `harness.record_run` | [Record-run method](method-record-run.md) |
| `harness.request_user_judgment` and `harness.record_user_judgment` | [User-judgment method owner](method-user-judgment.md) |
| `harness.close_task` | [Close-task method](method-close-task.md) |
