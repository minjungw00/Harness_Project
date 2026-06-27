# Agent integration reference

This document owns how Agent Connections are registered, selected for current invocation context, and described by capability declarations. It also defines the boundary for carrying owner-result Volicord context into an Agent Connection.

It does not define API schemas, method behavior, storage effects, security guarantee meanings, projection/display authority boundaries, or rendered template wording.

## Owns / Does not own

This document owns:

- Agent Connection meaning and Connection Project membership rules
- host configuration inventory meaning and host trust boundary
- current invocation and actor context boundaries, including `actor_source`, `operation_category`, verification basis, and assurance level
- User Channel versus Agent Connection boundaries for authority-bearing judgment resolution
- capability declaration boundaries
- MCP project selection and per-project execution validation boundaries
- agent context transfer rules between owner results and an Agent Connection
- fallback display when the selected Agent Connection or current invocation context is unavailable, mismatched, stale, or capability-limited
- one-language-per-`doc_id` retrieval guidance for agent context

This document does not own:

- interface-specific workflows; see [Surface Recipes](../guides/surface-recipes.md)
- API request envelopes, response branches, schema shapes, method access requirements, or operation-category value names; see [API Schema Core](api/schema-core.md), [API Methods](api/methods.md), method owners, and [API Value Sets](api/schema-value-sets.md)
- `volicord-mcp` executable startup, process environment, stdio framing, startup validation, response wrapping, or shutdown; see [MCP Transport](mcp-transport.md)
- storage layout, artifact lifecycle, or staged-handle validation; see storage and artifact owners through [Reference Index](README.md)
- security guarantee meanings or access-boundary wording; see [Security](security.md)
- authority versus projected display rules; see [Projection and template display boundaries](projection-and-templates.md)
- rendered body wording, public display labels, or template phrasing; see [Template Bodies](template-bodies.md)

## Agent Connection

An Agent Connection is the durable registry record for one coding-agent connection. One `volicord-mcp` process is bound to one Agent Connection, not to one fixed `Product Repository`.

Stored connection fields:

- `connection_id`
- `host_kind`
- `host_scope`
- `mode`
- `server_name`
- `config_target`
- `enabled`
- `managed_fingerprint`
- `last_verified_status`
- creation and update timestamps

Rules:

- An Agent Connection is agent-facing and cannot act as the local `User Channel`.
- A connection can be enabled or disabled without editing host configuration.
- Registering a connection does not automatically grant access to every project in the `Volicord Runtime Home`.
- A connection has access only to projects that are explicitly present in its Connection Project records.
- A connection mode controls exposed tool sets; `read_only` is not a workflow-write capability.

Storage record families and DDL belong to [Storage Records](storage-records.md) and [Storage DDL](storage-ddl.md). Administrative creation, update, verification, and removal commands belong to [Administrative CLI](admin-cli.md).

## Connection Project membership

Connection Project membership is an explicit registry relationship between an Agent Connection and registered projects.

Membership fields:

- `connection_id`
- `project_id`
- creation timestamp
- a composite primary key over `connection_id` and `project_id`

Rules:

- Project membership does not bypass project status, path separation, storage executability, Agent Connection mode, or method-owned invocation requirements.
- Invalid current project registrations must be rejected by Connection Project listing and access resolution instead of returned as connected project records.
- Inactive or otherwise execution-ineligible valid projects remain unavailable at execution time even if a stale membership row exists.
- Removing a Connection Project or disabling the Agent Connection must take effect without requiring host configuration to be rewritten.
- An Agent Connection with no connected projects may remain stored, and host configuration may also remain on disk. That stored state does not mean a new `volicord-mcp` process can start successfully.
- New MCP stdio startup and `volicord-mcp --check --connection <connection_id>` fail startup validation when the Agent Connection has zero connected projects. Administrative verification that depends on that same startup path cannot succeed in that state.
- A `volicord-mcp` process that already started while at least one project was connected can observe later membership changes without host configuration being rewritten. After the last membership is removed, `volicord.list_projects` may return an empty project list, but project-routed public tools cannot proceed normally because no connected project remains.
- The Agent Connection is executable again only after a project is connected and the startup or per-call project checks can validate the required project state.

<a id="host-installation"></a>
## Host configuration inventory

A stored Agent Connection is management inventory for Volicord-managed host configuration and verification state. The host configuration file remains the operational source of truth for the host. The registry record is management inventory and last-known verification state, not a substitute for the host configuration.

Stored connection fields:

- `connection_id`
- `host_kind`
- `host_scope`
- `mode`
- `server_name`
- `config_target`
- `enabled`
- `managed_fingerprint`
- `last_verified_status`
- creation and update timestamps

Supported host and scope matrix:

| Host kind | Baseline scopes | Scope meaning |
|---|---|---|
| `codex` | `user`, `project` | User scope may load across the user's Codex projects. Project scope writes project-scoped Codex MCP configuration and depends on Codex project trust before the host loads it. |
| `claude_code` | `local`, `project`, `user` | Local and project scopes load only for the associated project. User scope may load across the user's Claude Code projects. |
| `generic` | `export` | Volicord exports explicit configuration for a user-managed host and does not claim direct installation. |

Rules:

- Project and local scopes permit exactly the associated `Product Repository`.
- User scope may permit multiple explicitly added `Product Repository` registrations.
- Host trust, project trust, project MCP approval, OAuth, or any comparable host-controlled approval cannot be bypassed by Volicord.
- A host configuration write can be successful as a file operation while the result state remains `action_required` because the host has not yet trusted, approved, loaded, initialized, or exposed the server.
- `last_verified_status=complete` may be stored only for an administrative verification result that satisfied the operational gates owned by [Administrative CLI](admin-cli.md#agent-connection-result-states). A direct Volicord-spawned MCP handshake is not enough by itself.
- `last_verified_status=action_required` is the expected state when Volicord can manage or export configuration but a host-owned trust, approval, OAuth, reload, or restart action remains.
- `generic` export remains user-managed configuration inventory. It does not prove external host loading and must not become `complete` unless a host-specific owner later defines an observable loadability gate.
- Rejected, missing, changed, unavailable, and unknown host states are not `complete` Agent Connection states.
- Product Repository guidance, generated host instructions, and MCP server instructions can improve tool selection, but they are not enforcement mechanisms and cannot guarantee that a model will choose Volicord tools.

## Agent Connection boundary

Agent Connections carry context between Volicord owner results and an agent. They do not create Volicord authority.

Condition:
- An agent may rely on a connection only through owner-returned state or a compatible current invocation context.
- Display text, chat messages, generated files, connection descriptions, `Product Repository` files, projections, and agent memory are support context only.

Agent may:
- show owner-result state and display labels
- pass compact owner-result context to the agent

Agent must not:
- treat connection prose, copied identifiers, rendered displays, or agent memory as authority
- create Core state, `Write Authorization`, evidence sufficiency, user-owned judgment, close readiness, acceptance, residual-risk acceptance, artifact authority, or security guarantees from display text

Owner links:
- [Core Model](core-model.md) owns Core authority, user-owned judgment, close readiness, acceptance, and residual-risk boundaries.
- [Runtime Boundaries](runtime-boundaries.md) owns `Product Repository`, Volicord source repository/installation, executable-process, `Volicord Runtime Home`, and external MCP host configuration separation.
- [Projection and template display boundaries](projection-and-templates.md) owns authority versus projected display rules.

## User Channel and Agent Connections

Agent Connections are agent-facing connections. They are not the `User Channel`,
even when the model is relaying a user's words.

Condition:
- The supported local CLI path for a human user to inspect pending judgments and
  record a selected Core-generated option is the `volicord user` command group
  owned by [Administrative CLI](admin-cli.md#user-channel-commands).
- Authority-bearing user-judgment resolution requires
  `actor_source=local_user`, `operation_category=user_only`, and compatible
  User Channel provenance.
- `actor_source=agent_connection:<connection_id>` cannot become
  `local_user` provenance by relaying text from a user.

Agent may:
- request a missing user-owned judgment when a method owner supports that path
- display pending judgment state and Core-generated options returned by owners
- route the human user to the supported `User Channel`

Agent must not:
- record an authority-bearing user decision from an Agent Connection
- treat a natural-language approval, chat reply, generated Markdown status, or
  rendered projection as User Channel provenance
- broaden one selected option into final acceptance, residual-risk acceptance,
  sensitive-action approval, scope acceptance, or another judgment kind
- create evidence sufficiency, acceptance, residual-risk acceptance, close
  readiness, or security authority from the displayed judgment text

Owner links:
- [Core Model](core-model.md) owns the authority meaning of user-owned
  judgments, final acceptance, residual-risk acceptance, evidence, and close
  readiness.
- [Record-user-judgment method](api/method-record-user-judgment.md) owns public
  method behavior for resolving one pending judgment.
- [Projection and template display boundaries](projection-and-templates.md)
  owns generated display and projection authority boundaries.

## Surface registration

Surface registration names the user-selected surface and the facts method owners need when their contracts decide whether that surface can support a request.

Condition:
- `surface_id` is a selector for a registered local surface.
- `surface_instance_id` distinguishes a registered instance when a method owner returns or requires it.
- `surfaces.local_access_json` is the baseline source of registered local access grants for that surface instance.
- `authorized_access_classes: string[]` is required. It contains one or more documented access classes for the same surface instance.
- `access_class` is not a valid grant field in `surfaces.local_access_json`; capability profiles and invocation contexts have their own separate `access_class` fields.
- A baseline-workflow registration profile may expand to the explicit access-class set `read_status`, `core_mutation`, `write_authorization`, `artifact_registration`, and `run_recording`.
- A full-workflow profile must be explicitly selected and must not be the implicit default.
- `verification_basis: string` is required and must be non-empty. It is controlled registration or adapter-binding diagnostic metadata that explains how the grant was established. It does not grant access.
- `interaction_role: string` identifies whether the surface instance acts as `agent` or `user_interaction` for authority-resolution purposes. Baseline registration has no mixed-role surface instance.
- Registration facts are usable only through owner-returned verification for the current request.

Agent may:
- pass `surface_id` and `surface_instance_id` when a method owner requires them
- display owner-returned unavailable, mismatched, stale, or insufficient surface states

Agent must not:
- infer local reachability, access class, `verified=true`, or artifact provenance from caller prose, copied identifiers, generated Markdown, chat text, projection text, or agent memory
- treat `surface_id`, `surface_instance_id`, or a surface name as permission evidence
- treat `capability_profile`, requested invocation access, or `verification_basis` as an access grant
- treat environment variables, public request fields, or caller-supplied labels as trusted verification-basis text or audit facts

Owner links:
- [API Methods](api/methods.md) and method owners define method request conditions.
- [API Value Sets](api/schema-value-sets.md) owns access-class value names.
- [Security](security.md) owns access-boundary and guarantee wording.

## Current surface context

`VerifiedSurfaceContext` is the internal, derived context for one invocation. A Volicord executable role such as the `volicord-mcp` local adapter process derives it from the selected Agent Integration Profile, selected project, registered surface records, adapter-derived invocation context, and the requested invocation access. Method owners then decide whether the derived context is compatible with the request. It is not a public request payload.

An MCP session is bound at adapter startup to exactly one `integration_id`. The integration supplies `surface_id` and `surface_instance_id`. The selected project is determined per public MCP tool call, not fixed for the process lifetime.

Project selection for public MCP method calls is deterministic:

1. Use `ToolEnvelope.project_id` when supplied.
2. If it is absent and the integration permits exactly one available project, use that project.
3. If it is absent and a valid explicit `default_project_id` exists, use that default.
4. Otherwise reject the call as ambiguous and instruct the agent to call `volicord.list_projects`.

The adapter must not guess a project from folder names, process current working directory, host roots, host labels, or the first row returned by storage. MCP roots may be used only as optional future or host-provided hints. Roots do not change the deterministic selection order above.

`volicord.list_projects` is a read-only MCP adapter utility tool. It lists only projects explicitly allowed for the integration whose current registration can be validated, shows project availability and default status, and provides enough project identity information for an agent to choose a valid `project_id`. If an allowed project has an invalid current registration, the adapter fails the utility call instead of returning that project as a normal available or unavailable entry. It is outside the nine public Volicord Core API methods and must not be added to the public method list.

Before a public tool call enters Core, the MCP adapter must verify:

- the integration exists and is enabled
- the selected project is explicitly allowed for that integration
- the selected project is active and executable
- the integration's `surface_id` and `surface_instance_id` are registered for that project
- the requested access class is authorized for that surface instance

The MCP session does not bind one fixed access class for the whole process. The MCP adapter derives the requested invocation access from the public method name and typed params for the current call. MCP-visible public request params never contain `envelope.surface_id`, an invocation access class, invocation `surface_instance_id`, capability profile, verification basis, or `VerifiedSurfaceContext`. Core independently verifies both the selected integration/project binding and that the method-derived requested access is included in the registered grant in `surfaces.local_access_json` before it derives `VerifiedSurfaceContext`.

Method-derived requested access:

| Public method and typed params | Requested access |
|---|---|
| `volicord.status` | `read_status` |
| `volicord.intake` | `core_mutation` |
| `volicord.update_scope` | `core_mutation` |
| `volicord.prepare_write` | `write_authorization` |
| `volicord.stage_artifact` | `artifact_registration` |
| `volicord.record_run` | `run_recording` |
| `volicord.request_user_judgment` | `core_mutation` |
| `volicord.record_user_judgment` | `core_mutation` |
| `volicord.close_task` with `intent=check` | `read_status` |
| Other `volicord.close_task` intents | `core_mutation` |

`InvocationContext.access_class`, or an equivalent implementation concept, is the requested invocation access for the current call. It is not authority and cannot grant an access class. `VerifiedSurfaceContext` can be derived only when the requested invocation access is included in the registered grant in `surfaces.local_access_json`.

Verification basis for newly derived contexts is composed only from controlled registration and adapter-binding values. Environment variables and public request fields cannot supply arbitrary verification-basis text. Controlled examples include `local_admin_registration`, `agent_integration_binding`, `mcp_stdio_surface_binding`, `cli_direct_surface_binding`, and `test_fixture_binding`. Existing stored arbitrary basis strings may remain historical data, but newly written values use the controlled vocabulary. Verification basis is diagnostic metadata and never grants access.

Internal surface shape, not a public API schema:

```yaml
VerifiedSurfaceContext:
  project_id: string
  surface_id: string
  surface_instance_id: string
  access_class: string
  capability_profile: object
  verification_basis: string
```

`InvocationContext` is the internal, derived actor-provenance and operation context used when a method executes. It is derived from the local adapter or Core caller, not from public request payload text.

Internal actor shape, not a public API schema:

```yaml
InvocationContext:
  actor_source: local_user | system | agent_connection:<connection_id>
  operation_category: read | agent_workflow | user_only | admin_local
  verification_basis: string
  assurance_level: string
```

Baseline `assurance_level` means cooperative local provenance, not cryptographic human identity. Authority-bearing user-judgment resolution requires `actor_source=local_user`, `operation_category=user_only`, a compatible User Channel `verification_basis`, and `assurance_level=local_user_channel`. An Agent Connection cannot gain user authority by submitting copied user text or generated guidance.

Condition:
- A public API request has exactly one derived `InvocationContext`.
- Public `ToolEnvelope.project_id`, when present, is a deterministic project selector constrained by the Agent Connection's connected projects. It is not caller authority and cannot grant access to an unlisted, inactive, or invalid project.
- `ToolEnvelope` does not expose `actor_source` or `operation_category`. If raw MCP arguments include those fields, the adapter rejects the call before Core execution.
- Nested payloads such as `ArtifactInput` or `StagedArtifactHandle` do not add a second invocation context.
- Authority-provenance fields for resolved authority-bearing judgments come from the derived `InvocationContext`, not caller text, labels, answer payloads, copied refs, generated Markdown, or Product Repository guidance.
- Protected reads, mutations, and artifact operations can rely on an invocation only when the method owner accepts the derived context.
- Capability declarations can describe support, but they cannot grant or elevate `actor_source` or `operation_category`.

Agent may:
- preserve derived invocation context when displaying or passing context
- expose absent or incompatible context as unavailable, mismatched, stale, or insufficient Agent Connection state

Agent must not:
- submit `InvocationContext` as a request payload
- assert `verified=true`
- submit `actor_source=local_user` or `operation_category=user_only` from an Agent Connection to satisfy user authority
- submit capability profile or verification basis as public request authority
- fabricate staged artifact provenance
- use copied identifiers, generated Markdown, chat text, projection text, or agent memory as substitutes for verified context
- use capability declarations or requested invocation access as a substitute for the derived invocation context

Owner links:
- Exact request envelopes and response shapes belong to [API Schema Core](api/schema-core.md), [API Methods](api/methods.md), and method owners.
- Access-class values belong to [API Value Sets](api/schema-value-sets.md).
- `volicord-mcp` startup, integration binding, environment variables, stdio framing, startup validation, response wrapping, and shutdown belong to [MCP Transport](mcp-transport.md).

## Agent behavior guidance

Agent behavior guidance has two layers:

- MCP server instructions are always supplied by the server during MCP initialization.
- Optional `Product Repository` guidance is installed only with explicit user authorization.

Rules:

- MCP server instructions may describe cross-tool workflows, project selection rules, and limitations that apply across Volicord tools.
- Optional repository guidance may add a Volicord-managed block or host-specific rule file inside a `Product Repository` only under the boundary owned by [Runtime Boundaries](runtime-boundaries.md#explicit-integration-files-in-product-repositories).
- Guidance can improve tool selection, but it is not authority, access control, user judgment, security enforcement, or proof that a model will choose Volicord tools.

## Capability declaration

`capability_profile` is an integration declaration describing what a registered surface can support. It is not authority by itself.

Condition:
- A capability may be declared supported only when [Scope](scope.md) and the affected owners define it as baseline or profile-gated supported behavior.
- Protected reads, mutations, artifact operations, and guarantee displays may use a capability declaration only with compatible current surface context and owner-method support.
- Capability declarations remain non-authoritative and cannot add a grant to `surfaces.local_access_json`.

Agent may:
- describe supported access classes
- describe local reachability
- describe artifact staging or body-read support
- describe display limits
- show missing support as unavailable or capability-limited

Agent must not:
- use `capability_profile` to activate an out-of-scope capability
- use `capability_profile` to grant or elevate an access class
- use stale, copied, generated, or user-provided capability text to justify a stronger security guarantee
- replace method-owner access conditions or security-owner guarantee wording with a capability declaration

Owner links:
- [Scope](scope.md) owns baseline and profile-gated scope boundaries.
- [Security](security.md) owns guarantee vocabulary and guarantee-strength non-claims.
- [API Value Sets](api/schema-value-sets.md) owns access-class value names.

## Agent context transfer

Agent context transfer gives the agent enough owner context for the next action without turning the packet into an authority record.

Condition:
- Agent context should contain only owner results needed for the next action and current surface-context limits that affect that action.
- A context packet is support context, not Core state, storage state, evidence, acceptance, residual-risk acceptance, or close output.

Agent may:
- pass compact context containing the current Task summary, current scope, `state_version`, pending user-owned judgments, blockers, next safe action, evidence and artifact summaries, close-readiness and residual-risk summaries, owner-supported guarantee display, and source or limitation notes
- retrieve exact owner sections only when the next action needs them
- include both language versions for the same `doc_id` when bilingual maintenance requires semantic-parity review

Agent must not:
- inject full schemas, DDL, historical logs, artifact bodies, unrelated contract material, out-of-scope catalogs, exact template bodies, or both language versions for the same `doc_id` by default
- treat a stale or copied context packet as newer authority than the owner result or underlying record

Owner links:
- [Template Bodies](template-bodies.md) owns agent context packet wording.
- [Reference Index](README.md) routes exact owner sections.
- [Translation Policy](../maintain/translation-policy.md) owns bilingual semantic-parity review guidance.

## Fallback boundary

Fallback display applies when the current surface context or a required integration capability is unavailable, mismatched, stale, or insufficient for the requested operation.

Agent may:
- move to a capable surface
- narrow the operation
- request the missing user-owned judgment
- continue outside Volicord only when the user explicitly chooses that mode

Agent must:
- expose the limitation in support or display text
- route machine-readable failure meanings to [API error codes](api/error-codes.md) and [API error details](api/error-details.md)
- route user-facing wording to [Template Bodies](template-bodies.md) or [Surface Recipes](../guides/surface-recipes.md)

Agent must not:
- fabricate authority
- hide unavailable, mismatched, stale, or insufficient capability states inside ordinary success text
- continue outside Volicord without the user's explicit choice
