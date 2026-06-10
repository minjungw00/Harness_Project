# Runtime Boundaries Reference

This document owns the boundary model between Product Repository, Harness Server, and Harness Runtime Home. It is source documentation only; no Harness Server/runtime implementation, Runtime Home, generated projection system, conformance runner, or runtime data exists in this repository today.

## Owns / Does not own

This document owns:

- Product Repository / Harness Server / Runtime Home separation
- mutation-authority boundaries between product files, Harness records, and rendered displays
- non-isolation and OS-sandboxing non-claims
- the rule that generated display and Product Repository text do not create Harness authority

This document does not own:

- storage record shapes, effects, artifacts, versioning, locks, or migrations; see storage owners through [Reference Index](README.md)
- public API schemas or method behavior; see API owners through [Reference Index](README.md)
- security guarantee meanings; see [Security](security.md)
- projection authority; see [Projection And Templates](projection-and-templates.md)

## Product Repository

The Product Repository is the user's project workspace. Product files may be input to future Harness checks, but Product Repository content is not Harness state, not a Runtime Home, and not proof of Harness authority.

## Harness Server

The future Harness Server would mediate Harness records and API behavior. It is not implemented in this repository. Documentation edits do not create server code or authorize product/runtime writes.

## Harness Runtime Home

Harness Runtime Home is the future per-user or per-installation operational data space. Exact storage records, paths, artifacts, locks, migrations, and versioning belong to the split storage owners. This documentation repository is not a Runtime Home.

## Mutation Boundary

Core-owned Harness records can change only through owner-defined paths. Product Repository edits, generated Markdown, rendered projections, chat text, connector prose, or agent memory do not directly mutate Harness records.

Artifact body reads and artifact promotion require the API, storage, local-surface, and security owners to agree on the relevant boundary. A copied `surface_id`, displayed `ArtifactRef`, or rendered projection is not authority by itself.

## Non-Isolation Boundary

The current documentation does not claim OS-level permission control, arbitrary-tool sandboxing, tamper-proof storage, default pre-tool blocking, security isolation, or permission isolation. Such claims require a promoted owner with a documented mechanism and proof path.
