# Runtime boundaries reference

This document owns the boundary between Product Repository, Harness Server or installation, and Harness Runtime Home.

Conditions:
- Use this page when a claim depends on where product files, server installation files, or future Harness runtime data live.
- Treat this repository as source documentation for a future Harness Server only.

May claim:
- A Markdown file in this repository may describe a future Harness rule.
- Product Repository, Harness Server or installation, and Harness Runtime Home are distinct locations.

Must not claim:
- This repository contains a Harness Server/runtime implementation, Runtime Home, generated projection system, conformance runner, runtime state, or runtime data.
- Documentation files are runtime state, Harness records, generated artifacts, projections, acceptance records, evidence records, or operational files.

Owner links:
- [Security](security.md) owns security guarantee meanings and non-claims.
- [Storage Records](storage-records.md), [Storage Effects](storage-effects.md), and [Artifact Storage](storage-artifacts.md) own storage details.

## Owns / Does not own

This document owns:

- the separation between Product Repository, Harness Server or installation, and Harness Runtime Home
- the rule that Product Repository files, generated displays, chat text, connector prose, and agent memory do not create Harness authority
- the distinction between server installation location and runtime data location
- the boundary statement that a Runtime Home is not automatically a security boundary

This document does not own:

- storage record shapes, effects, artifacts, versioning, locks, or migrations; see storage owners through [Reference Index](README.md)
- public API schemas or method behavior; see API owners through [Reference Index](README.md)
- security guarantee meanings or detailed security non-claims; see [Security](security.md)
- projection authority; see [Projection Authority Reference](projection-and-templates.md)

## Three locations

Harness documentation must keep these locations distinct:

| Location | Details |
|---|---|
| Product Repository | See [Product Repository location](#runtime-location-product-repository) |
| Harness Server or installation | See [Harness Server or installation location](#runtime-location-server-installation) |
| Harness Runtime Home | See [Harness Runtime Home location](#runtime-location-runtime-home) |

<a id="runtime-location-product-repository"></a>
### Product Repository location

Conditions:
- The user's project workspace.

May claim:
- It can supply product files as input.

Must not claim:
- It is Harness runtime state or a Runtime Home by default.

Owner links:
- [Storage Effects](storage-effects.md) owns product-file and Harness-record storage effects.
- [Security](security.md) owns security non-claims.

<a id="runtime-location-server-installation"></a>
### Harness Server or installation location

Conditions:
- The future server process, package, or installed application location.

May claim:
- It may mediate Harness APIs and records.

Must not claim:
- The install location is automatically where runtime data lives.

Owner links:
- [MVP API router](api/mvp-api.md) routes method behavior to API owners.
- [Storage Records](storage-records.md) owns runtime data record layout.

<a id="runtime-location-runtime-home"></a>
### Harness Runtime Home location

Conditions:
- The future operational data space for Harness records, local store metadata, and artifact storage.

May claim:
- Storage/runtime owners may define what operational data belongs there.

Must not claim:
- It is the Product Repository.
- It is proof of security authority.

Owner links:
- [Storage Records](storage-records.md), [Storage Effects](storage-effects.md), and [Artifact Storage](storage-artifacts.md) own runtime data details.
- [Security](security.md) owns security guarantee wording.

## Product repository

The Product Repository is the user's project workspace.

Conditions:
- Product files may be input to future Harness checks or user-owned judgments.
- A future implementation may allow project-local Harness metadata only when storage/runtime owners define it.

May claim:
- Product Repository files are user workspace files.
- Owner-defined project-local Harness metadata may exist in a future implementation.

Must not claim:
- Product Repository content is Harness state.
- Product Repository content is generated Harness output.
- Product Repository content is proof of Harness authority.
- A Product Repository is automatically the Harness Runtime Home.
- Ordinary product files become Harness records because project-local metadata exists.

Owner links:
- [Storage Effects](storage-effects.md) owns product-file and Harness-record effects.
- [Core Model](core-model.md) owns user-owned judgment boundaries.
- [Security](security.md) owns security authority non-claims.

## Harness server or installation

The future Harness Server would mediate Harness records and API behavior.

Conditions:
- A server installation location is where server code, packages, configuration, or application resources may live in a future implementation.
- A directory is a Runtime Home only when storage/runtime owners define it as such.

May claim:
- A future Harness Server mediates Harness records and API behavior.
- A server installation location can be distinct from runtime data storage.

Must not claim:
- The installation location and runtime data location are the same by default.
- Installing or running a server from a directory makes that directory the Runtime Home.
- This repository contains a Harness Server implementation.
- Documentation edits create server code, start runtime behavior, or authorize product/runtime writes.

Owner links:
- [MVP API router](api/mvp-api.md) routes API method behavior.
- [Storage Records](storage-records.md) owns runtime data record layout.
- [Storage Effects](storage-effects.md) owns product/runtime write effects.

## Harness runtime home

Harness Runtime Home is the future per-user or per-installation operational data space.

Conditions:
- Storage/runtime owners define what operational data belongs in the Runtime Home.
- Operational data may include Harness-owned records, local store metadata, staged or persisted artifact data, locks, migrations, and related diagnostics.

May claim:
- A future Runtime Home can hold Harness operational data when storage/runtime owners define the data and validation rules.

Must not claim:
- A Runtime Home is the Product Repository.
- A Runtime Home is automatically a security boundary.
- This documentation repository is a Runtime Home.

Owner links:
- [Security](security.md) owns security guarantee wording and non-claims.
- [Storage Records](storage-records.md), [Storage Effects](storage-effects.md), [Artifact Storage](storage-artifacts.md), and [Storage Versioning](storage-versioning.md) own storage and runtime data details.

## What may be stored where

| Location | Details |
|---|---|
| Product Repository | See [Product Repository storage](#runtime-storage-product-repository) |
| Harness Server or installation | See [Harness Server or installation storage](#runtime-storage-server-installation) |
| Harness Runtime Home | See [Harness Runtime Home storage](#runtime-storage-runtime-home) |
| This documentation repository | See [Documentation repository storage](#runtime-storage-documentation-repository) |

<a id="runtime-storage-product-repository"></a>
### Product Repository storage

Conditions:
- The location is the user's project workspace.

May claim:
- Product source, product docs, tests, project configuration, and product files that future Harness checks may inspect.

Must not claim:
- Product Repository storage is Harness runtime state, generated Harness records, a Runtime Home, or authority proof.

Owner links:
- [Storage Effects](storage-effects.md) owns product-file effects.
- [Security](security.md) owns authority and guarantee non-claims.

<a id="runtime-storage-server-installation"></a>
### Harness Server or installation storage

Conditions:
- The location is the future server process, package, or installed application location.

May claim:
- Future server executable code, installed packages, server configuration, and application resources.

Must not claim:
- Harness Server or installation storage is product workspace content, canonical runtime data, or proof that a Runtime Home exists.

Owner links:
- [MVP API router](api/mvp-api.md) routes method behavior.
- [Storage Records](storage-records.md) owns runtime data records.

<a id="runtime-storage-runtime-home"></a>
### Harness Runtime Home storage

Conditions:
- Storage/runtime owners define the future operational data space.

May claim:
- Future Harness operational records, runtime metadata, local store data, artifacts, locks, migrations, and related diagnostics defined by storage/runtime owners.

Must not claim:
- Harness Runtime Home storage is product source, server install files, or a security boundary.

Owner links:
- [Storage Records](storage-records.md), [Storage Effects](storage-effects.md), [Artifact Storage](storage-artifacts.md), and [Storage Versioning](storage-versioning.md) own runtime data details.
- [Security](security.md) owns security boundary claims.

<a id="runtime-storage-documentation-repository"></a>
### Documentation repository storage

Conditions:
- The location is this documentation repository.

May claim:
- Source documentation for future Harness behavior.

Must not claim:
- Documentation repository storage is runtime state, server implementation, generated projections, evidence, QA, acceptance, close records, or conformance output.

Owner links:
- [MVP Plan](../build/mvp-plan.md) owns maintainer handoff status.
- [Security](security.md) owns security non-claims.

## What must not be inferred

- Do not infer that this repository contains a working Harness Server, Runtime Home, or runtime data.
- Do not infer that documentation files are Harness records or generated operational files.
- Do not infer that the Product Repository is the Runtime Home unless an owner-defined runtime configuration says so.
- Do not infer that the server installation directory is the runtime data directory.
- Do not infer that a Runtime Home is a security boundary.
- Do not infer Harness authority from Product Repository text, generated Markdown, rendered displays, chat text, connector prose, agent memory, copied `surface_id` values, displayed `ArtifactRef` values, or rendered projections.

## Security boundary links

This page states the location boundary and the non-inference rules. Detailed guarantee levels, capability-gated detective wording, explicit non-claims, and later preventive-control requirements belong to [Security](security.md).

## Owner links

- [Reference Index](README.md): routes questions to canonical owners.
- [Security](security.md): owns security claims, non-claims, trust boundaries, and guarantee levels.
- [Storage Records](storage-records.md), [Storage Effects](storage-effects.md), [Artifact Storage](storage-artifacts.md), and [Storage Versioning](storage-versioning.md): own storage layout, effects, artifacts, locks, migrations, and versioning.
- [MVP API router](api/mvp-api.md), method owner documents, and API schema owners: own method routing, method behavior, and API shapes.
- [Projection Authority Reference](projection-and-templates.md): owns projection authority and source-state/freshness boundaries.
- [Template Bodies](template-bodies.md): owns status card, judgment request, run/evidence summary, close result, and agent context packet bodies.
