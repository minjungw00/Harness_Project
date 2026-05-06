# Reference MVP

## 문서 역할

이 문서는 MVP implementation sequence와 reference implementation detail을 담당한다. SQLite DDL draft, migration, lock policy, artifact directory layout, baseline capture format, projection job table, validator runner skeleton, reference surface behavior, minimal operator tooling plan을 포함한다.

Public MCP schema source of truth는 담당하지 않는다. 구현자는 public request/response contract에 `05-mcp-api-and-schemas.md`를 사용해야 한다.

## MVP 범위

MVP는 broad agent-surface integration project가 아니라 core invariant validation project다.

MVP 포함 항목:

- 하나의 local project registration
- 하나의 reference agent surface
- API document의 public tool을 expose하는 MCP server
- `state.sqlite` current table과 `state.sqlite.task_events`
- artifact registry와 durable artifact file
- baseline capture
- scope, approval, baseline, capability check가 있는 `prepare_write` gate
- approval, evidence, verification, Manual QA, acceptance gate support
- TASK, APR, RUN-SUMMARY, EVIDENCE-MANIFEST, EVAL, DIRECT-RESULT projection
- policy가 요구하는 경우 optional minimal TDD-TRACE 및 MANUAL-QA projection
- detached verification bundle 또는 manual evaluator instruction bundle
- doctor, recover, reconcile, export, conformance smoke entrypoint

MVP는 broader surface expansion, richer capture automation, advanced orchestration, analytics, team profile export/import를 포함해 `appendix/C-later-roadmap.md`에 정리된 later automation을 제외한다.

## 구현 순서

### MVP-0: Runtime Bootstrap

Runtime home을 만들고, 하나의 project를 register하고, `project.yaml`을 만들고, `registry.sqlite`와 `state.sqlite`를 initialize하고, artifact directory를 만들고, cooperative/detective capability profile로 reference surface를 register한다.

Exit criteria:

- project가 `registry.sqlite.projects`에 나타남
- reference surface가 `registry.sqlite.project_surfaces`에 나타남
- project runtime directory가 `project.yaml`, `state.sqlite`, artifact directory를 포함함
- doctor가 project/runtime readiness를 report할 수 있음

### MVP-1: Core State And MCP Facade

Core transaction wrapper, lock, state version check, idempotency replay record, read resource, `harness.status`, `harness.intake`, `harness.next`를 구현한다.

Exit criteria:

- active Task absent status가 동작함
- advisor Task가 Core를 통해 intake, read-only run, close될 수 있음
- 모든 state mutation이 current record를 update하고 하나의 transaction 안에서 `state.sqlite.task_events`를 append함

### MVP-2: Write Gate, Approval, Baseline, Artifacts

Change Unit record, gate record, baseline capture, artifact registration, `harness.prepare_write`, approval request/decision flow, minimal changed-path/scope/approval/baseline validator를 구현한다.

Exit criteria:

- active scoped Change Unit이 없는 product write가 blocked됨
- sensitive dependency 또는 schema change에 approval이 필요함
- approval scope drift가 approval을 expire 또는 block할 수 있음
- raw artifact가 hash와 redaction metadata와 함께 저장됨

### MVP-3: Runs, Evidence, Projection, Reconcile

`harness.record_run`, run record, evidence manifest record, projection job, TASK/APR/RUN-SUMMARY/EVIDENCE-MANIFEST/DIRECT-RESULT renderer, managed block hash, managed drift 또는 human-editable proposal에 대한 reconcile item creation을 구현한다.

Exit criteria:

- implementation 및 direct run이 artifact를 register하고 evidence를 update함
- projection job failure가 state failure와 분리됨
- managed Markdown edit가 state를 mutate하지 않고 reconcile item을 생성함

### MVP-4: Verification, Manual QA, Close

`harness.launch_verify`, `harness.record_eval`, `harness.record_manual_qa`, `harness.close_task`, verification independence check, Manual QA aggregation, close blocker를 구현한다.

Exit criteria:

- work가 same-session self-review로 `detached_verified` close될 수 없음
- verification waiver는 `detached_verified`가 아니라 `completed_with_risk_accepted`로 close됨
- required Manual QA와 acceptance가 독립적으로 close를 block함
- policy 또는 user가 detached verification을 요청하지 않는 한 direct work는 self-checked로 close될 수 있음

### MVP-5: Operator Smoke And Conformance

Minimal doctor, recover, reconcile, export, artifact integrity check, fixture-based conformance smoke를 구현한다.

Exit criteria:

- conformance smoke가 no-active-task status, advisor close, direct close, approval-required block, evidence-insufficient close block, same-session verification guard, projection failure separation, reconcile required, MCP-unavailable write hold를 cover함
- export가 state snapshot, report projection, artifact ref, redaction status를 포함함

## Runtime Storage

Reference storage는 registry와 per-project state에 SQLite를 사용한다. DDL은 draft implementation contract다. Field name은 index나 migration helper를 얻을 수 있지만 table ownership과 authority boundary는 stable하게 유지되어야 한다.

### `project.yaml`

`project.yaml`은 static project configuration만 저장한다. Current Task state를 저장하면 안 된다.

```yaml
project_id: PRJ-0001
display_name: my-app
repo_root: /abs/path/to/my-app
default_agent_surface: reference

agent_surfaces:
  reference:
    enabled: true
    capability_profile_id: SURF-PROFILE-0001

default_checks:
  lint: []
  test: []
  build: []

design_quality:
  vertical_slice_default: true
  tdd_required_for: []
  manual_qa_default_for: []

network_policy:
  default_write: deny
  allowed_read_domains: []
  allowed_write_targets: []

secret_policy:
  env_allowlist: []
  allow_secret_access_without_approval: false
```

### `registry.sqlite`

```sql
CREATE TABLE projects (
  project_id TEXT PRIMARY KEY,
  display_name TEXT NOT NULL,
  repo_root TEXT NOT NULL,
  repo_fingerprint TEXT NOT NULL,
  runtime_path TEXT NOT NULL,
  project_yaml_path TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE project_surfaces (
  surface_id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL REFERENCES projects(project_id),
  surface_kind TEXT NOT NULL,
  display_name TEXT NOT NULL,
  capability_profile_id TEXT NOT NULL,
  guarantee_level TEXT NOT NULL,
  enabled INTEGER NOT NULL DEFAULT 1,
  mcp_config_ref TEXT,
  last_seen_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE connector_manifests (
  manifest_id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL REFERENCES projects(project_id),
  surface_id TEXT NOT NULL REFERENCES project_surfaces(surface_id),
  manifest_version INTEGER NOT NULL,
  generated_paths_json TEXT NOT NULL,
  managed_hash TEXT NOT NULL,
  capability_profile_json TEXT NOT NULL,
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

### `state.sqlite`

```sql
CREATE TABLE tasks (
  task_id TEXT PRIMARY KEY,
  state_version INTEGER NOT NULL,
  mode TEXT NOT NULL,
  lifecycle_phase TEXT NOT NULL,
  result TEXT NOT NULL,
  close_reason TEXT NOT NULL,
  assurance_level TEXT NOT NULL,
  title TEXT NOT NULL,
  current_summary TEXT NOT NULL DEFAULT '',
  acceptance_criteria_json TEXT NOT NULL DEFAULT '[]',
  active_change_unit_id TEXT,
  active_run_id TEXT,
  latest_evidence_manifest_id TEXT,
  latest_eval_id TEXT,
  latest_manual_qa_record_id TEXT,
  projection_version INTEGER NOT NULL DEFAULT 0,
  projected_version INTEGER NOT NULL DEFAULT 0,
  projection_status TEXT NOT NULL DEFAULT 'unknown',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE task_gates (
  task_id TEXT PRIMARY KEY REFERENCES tasks(task_id),
  scope_gate TEXT NOT NULL,
  approval_gate TEXT NOT NULL,
  design_gate TEXT NOT NULL,
  evidence_gate TEXT NOT NULL,
  verification_gate TEXT NOT NULL,
  qa_gate TEXT NOT NULL,
  acceptance_gate TEXT NOT NULL,
  waiver_json TEXT NOT NULL DEFAULT '{}',
  updated_at TEXT NOT NULL
);

CREATE TABLE change_units (
  change_unit_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL REFERENCES tasks(task_id),
  title TEXT NOT NULL,
  purpose TEXT NOT NULL,
  non_goals_json TEXT NOT NULL DEFAULT '[]',
  slice_type TEXT NOT NULL,
  horizontal_exception_reason TEXT,
  follow_up_vertical_change_unit_id TEXT,
  allowed_paths_json TEXT NOT NULL DEFAULT '[]',
  allowed_tools_json TEXT NOT NULL DEFAULT '[]',
  allowed_commands_json TEXT NOT NULL DEFAULT '[]',
  allowed_network_json TEXT NOT NULL DEFAULT '[]',
  secret_scope_json TEXT NOT NULL DEFAULT '[]',
  sensitive_categories_json TEXT NOT NULL DEFAULT '[]',
  validator_profile_json TEXT NOT NULL DEFAULT '[]',
  completion_conditions_json TEXT NOT NULL DEFAULT '[]',
  evaluator_focus_json TEXT NOT NULL DEFAULT '[]',
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE runs (
  run_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL REFERENCES tasks(task_id),
  change_unit_id TEXT,
  kind TEXT NOT NULL,
  actor_kind TEXT NOT NULL,
  surface_id TEXT NOT NULL,
  baseline_ref TEXT,
  summary TEXT NOT NULL DEFAULT '',
  observed_changes_json TEXT NOT NULL DEFAULT '{}',
  command_results_json TEXT NOT NULL DEFAULT '[]',
  artifact_refs_json TEXT NOT NULL DEFAULT '[]',
  status TEXT NOT NULL,
  started_at TEXT NOT NULL,
  completed_at TEXT
);

CREATE TABLE approvals (
  approval_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL REFERENCES tasks(task_id),
  change_unit_id TEXT,
  decision_request_id TEXT,
  status TEXT NOT NULL,
  sensitive_categories_json TEXT NOT NULL DEFAULT '[]',
  allowed_paths_json TEXT NOT NULL DEFAULT '[]',
  allowed_tools_json TEXT NOT NULL DEFAULT '[]',
  allowed_commands_json TEXT NOT NULL DEFAULT '[]',
  allowed_network_targets_json TEXT NOT NULL DEFAULT '[]',
  secret_scope_json TEXT NOT NULL DEFAULT '[]',
  baseline_ref TEXT,
  expires_at TEXT,
  decision_note TEXT,
  created_at TEXT NOT NULL,
  decided_at TEXT
);

CREATE TABLE decision_requests (
  decision_request_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL REFERENCES tasks(task_id),
  change_unit_id TEXT,
  decision_kind TEXT NOT NULL,
  status TEXT NOT NULL,
  prompt TEXT NOT NULL,
  options_json TEXT NOT NULL DEFAULT '[]',
  recommendation TEXT,
  approval_scope_json TEXT NOT NULL DEFAULT '{}',
  reconcile_item_id TEXT,
  expires_at TEXT,
  decided_option_id TEXT,
  decision_json TEXT NOT NULL DEFAULT '{}',
  note TEXT,
  waiver_reason TEXT,
  created_at TEXT NOT NULL,
  decided_at TEXT
);

CREATE TABLE evidence_manifests (
  evidence_manifest_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL REFERENCES tasks(task_id),
  change_unit_id TEXT,
  baseline_ref TEXT,
  criteria_json TEXT NOT NULL DEFAULT '[]',
  changed_files_json TEXT NOT NULL DEFAULT '[]',
  supporting_refs_json TEXT NOT NULL DEFAULT '[]',
  stale_if_json TEXT NOT NULL DEFAULT '[]',
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE evals (
  eval_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL REFERENCES tasks(task_id),
  evaluator_run_id TEXT,
  target_run_id TEXT,
  verdict TEXT NOT NULL,
  checks_json TEXT NOT NULL DEFAULT '[]',
  evidence_reviewed_json TEXT NOT NULL DEFAULT '[]',
  independence_json TEXT NOT NULL DEFAULT '{}',
  blockers_json TEXT NOT NULL DEFAULT '[]',
  artifact_refs_json TEXT NOT NULL DEFAULT '[]',
  created_at TEXT NOT NULL
);

CREATE TABLE manual_qa_records (
  manual_qa_record_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL REFERENCES tasks(task_id),
  qa_profile TEXT NOT NULL,
  performed_by TEXT NOT NULL,
  result TEXT NOT NULL,
  findings_json TEXT NOT NULL DEFAULT '[]',
  artifact_refs_json TEXT NOT NULL DEFAULT '[]',
  waiver_reason TEXT,
  next_action TEXT NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE artifacts (
  artifact_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL REFERENCES tasks(task_id),
  run_id TEXT,
  kind TEXT NOT NULL,
  relative_path TEXT NOT NULL,
  sha256 TEXT NOT NULL,
  size_bytes INTEGER NOT NULL,
  content_type TEXT NOT NULL,
  redaction_state TEXT NOT NULL,
  produced_by TEXT NOT NULL,
  retention_class TEXT NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE task_events (
  event_id TEXT PRIMARY KEY,
  task_id TEXT,
  state_version INTEGER NOT NULL,
  event_type TEXT NOT NULL,
  actor_kind TEXT NOT NULL,
  surface_id TEXT,
  request_id TEXT,
  idempotency_key TEXT,
  payload_json TEXT NOT NULL DEFAULT '{}',
  created_at TEXT NOT NULL
);

CREATE TABLE tool_invocations (
  invocation_id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  task_id TEXT,
  tool_name TEXT NOT NULL,
  request_id TEXT NOT NULL,
  idempotency_key TEXT NOT NULL,
  request_hash TEXT NOT NULL,
  response_json TEXT NOT NULL DEFAULT '{}',
  state_version INTEGER NOT NULL,
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  completed_at TEXT,
  UNIQUE(project_id, tool_name, idempotency_key)
);

CREATE TABLE projection_jobs (
  projection_job_id TEXT PRIMARY KEY,
  task_id TEXT,
  projection_kind TEXT NOT NULL,
  target_ref TEXT NOT NULL,
  projection_version INTEGER NOT NULL,
  status TEXT NOT NULL,
  attempts INTEGER NOT NULL DEFAULT 0,
  output_path TEXT,
  managed_hash TEXT,
  error_json TEXT NOT NULL DEFAULT '{}',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE reconcile_items (
  reconcile_item_id TEXT PRIMARY KEY,
  task_id TEXT,
  source_kind TEXT NOT NULL,
  source_path TEXT,
  source_hash TEXT,
  target_record_kind TEXT,
  target_record_id TEXT,
  proposed_change_json TEXT NOT NULL DEFAULT '{}',
  status TEXT NOT NULL,
  decision_json TEXT NOT NULL DEFAULT '{}',
  created_at TEXT NOT NULL,
  resolved_at TEXT
);

CREATE TABLE domain_terms (
  domain_term_id TEXT PRIMARY KEY,
  term TEXT NOT NULL,
  meaning TEXT NOT NULL,
  code_representation TEXT,
  not_this_json TEXT NOT NULL DEFAULT '[]',
  related_terms_json TEXT NOT NULL DEFAULT '[]',
  source_ref TEXT,
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE module_map_items (
  module_map_item_id TEXT PRIMARY KEY,
  module_path TEXT NOT NULL,
  responsibility TEXT NOT NULL,
  public_interface_json TEXT NOT NULL DEFAULT '[]',
  dependencies_json TEXT NOT NULL DEFAULT '[]',
  test_boundary TEXT,
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE interface_contracts (
  interface_contract_id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  owner_module TEXT NOT NULL,
  change_type TEXT NOT NULL,
  inputs_json TEXT NOT NULL DEFAULT '[]',
  outputs_json TEXT NOT NULL DEFAULT '[]',
  errors_json TEXT NOT NULL DEFAULT '[]',
  compatibility_impact TEXT NOT NULL,
  callers_impacted_json TEXT NOT NULL DEFAULT '[]',
  boundary_tests_json TEXT NOT NULL DEFAULT '[]',
  review_status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE tdd_traces (
  tdd_trace_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL REFERENCES tasks(task_id),
  change_unit_id TEXT,
  status TEXT NOT NULL,
  red_refs_json TEXT NOT NULL DEFAULT '[]',
  green_refs_json TEXT NOT NULL DEFAULT '[]',
  refactor_refs_json TEXT NOT NULL DEFAULT '[]',
  non_tdd_justification TEXT,
  artifact_refs_json TEXT NOT NULL DEFAULT '[]',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE validator_runs (
  validator_run_id TEXT PRIMARY KEY,
  task_id TEXT,
  change_unit_id TEXT,
  run_id TEXT,
  validator_id TEXT NOT NULL,
  validator_kind TEXT NOT NULL,
  status TEXT NOT NULL,
  guarantee_level TEXT NOT NULL,
  findings_json TEXT NOT NULL DEFAULT '[]',
  blocked_reasons_json TEXT NOT NULL DEFAULT '[]',
  created_at TEXT NOT NULL
);

CREATE TABLE locks (
  lock_id TEXT PRIMARY KEY,
  scope TEXT NOT NULL,
  owner TEXT NOT NULL,
  acquired_at TEXT NOT NULL,
  expires_at TEXT NOT NULL,
  heartbeat_at TEXT NOT NULL
);
```

`task_events`는 append-only event history로 남는다. `tool_invocations`는 original committed response를 반환하는 데 필요한 request replay metadata를 저장한다. 같은 idempotency key를 다른 `request_hash`로 reuse하면 `STATE_CONFLICT`를 반환한다.

권장 index:

```sql
CREATE INDEX idx_task_events_task_version ON task_events(task_id, state_version);
CREATE INDEX idx_decision_requests_task_status ON decision_requests(task_id, status);
CREATE INDEX idx_projection_jobs_status ON projection_jobs(status, projection_version);
CREATE INDEX idx_artifacts_task_run ON artifacts(task_id, run_id);
CREATE INDEX idx_runs_task_status ON runs(task_id, status);
CREATE INDEX idx_reconcile_items_status ON reconcile_items(status);
```

`task_events`는 application policy상 append-only다. Recovery는 compensating event를 append할 수 있지만 historical row를 rewrite해서는 안 된다.

## Migration And Versioning

MVP는 작은 internal migration ledger에 기록되는 integer schema version을 사용한다.

```sql
CREATE TABLE schema_migrations (
  database_name TEXT NOT NULL,
  version INTEGER NOT NULL,
  applied_at TEXT NOT NULL,
  checksum TEXT NOT NULL,
  PRIMARY KEY (database_name, version)
);
```

MVP migration은 forward-only여야 한다. Migration이 실패하면 doctor/recover가 failure가 repairable인지 report할 때까지 project를 unavailable로 둔다.

## Lock Policy

State-changing operation은 실용적으로 가장 좁은 scope의 lock을 acquire한다.

| Operation | Lock scope |
|---|---|
| project registration | project |
| task intake/close | task |
| prepare_write | task and active Change Unit |
| record_run | task and run |
| projection render | projection job |
| artifact registration | artifact path |
| reconcile decision | reconcile item and affected task/design record |

Lock이 expired되면 다음 operation은 recovery event를 append한 뒤 lock을 가져갈 수 있다. `expected_state_version`이 stale이면 operation은 mutation 전에 `STATE_CONFLICT`를 반환한다.

## Artifact Directory Layout

Reference layout:

```text
~/.harness/
  registry.sqlite
  projects/
    PRJ-0001/
      project.yaml
      state.sqlite
      artifacts/
        bundles/
        diffs/
        logs/
        screenshots/
        checkpoints/
        manifests/
        qa/
        tdd/
        exports/
        tmp/
```

Artifact filename은 collision을 피할 만큼 stable identity를 포함해야 한다.

```text
{task_id}/{run_id-or-record_id}/{artifact_id}-{kind}.{ext}
```

Product Repository의 Markdown report는 기본적으로 raw artifact가 아니다. Export에 report snapshot이 필요하면 report projection과 raw evidence의 구분을 보존하면서 그 snapshot을 export component artifact로 저장할 수 있다.

### Artifact Registration Contract

Artifact registration은 producing Run, Eval, Manual QA record, verification bundle, export component를 기록하는 Core transition의 일부다.

MVP registration step:

1. Project artifact `tmp/` directory 아래 staging path 또는 approved capture adapter에서만 connector-captured 또는 operator-supplied file을 accept한다.
2. Hashing 전에 redaction 또는 omission을 적용한다. Raw secret은 durable artifact storage로 copy하면 안 된다.
3. Matching kind directory 아래 `{task_id}/{run_id-or-record_id}/{artifact_id}-{kind}.{ext}`를 사용해 stored bytes를 artifact directory로 move 또는 copy한다.
4. Stored bytes에서 `sha256`, `size_bytes`, `content_type`, `redaction_state`를 compute한다.
5. Related state record를 기록하고 `task_events`를 append하는 같은 Core transaction 안에서 `artifacts` row를 insert한다.
6. Artifact registry row를 통해 `uri`가 resolve되는 `ArtifactRef`를 반환한다.
7. File move는 성공했지만 transaction이 실패하면 file을 `tmp/`에 남기거나 `recover`용 orphaned로 mark한다. Committed artifact ref는 만들지 않는다.

`redaction_state` implementation:

| State | Stored artifact bytes |
|---|---|
| `none` | original non-sensitive evidence |
| `redacted` | redacted evidence; unredacted original은 harness가 retain하지 않음 |
| `secret_omitted` | secret value가 omitted되거나 handle로 대체된 evidence |
| `blocked` | capture가 blocked되었음을 설명하는 small metadata-only notice artifact; forbidden content는 저장하지 않음 |

Artifact integrity failure는 `ARTIFACT_MISSING` 또는 validator failure를 반환하고, kernel rule에 따라 related evidence 또는 projection freshness를 stale로 mark한다.

## Baseline Capture

Baseline capture는 write, approval, evidence, verification check에 사용되는 repository state를 기록한다.

```yaml
BaselineCapture:
  baseline_ref: BASE-0001
  project_id: PRJ-0001
  task_id: TASK-0001
  change_unit_id: CU-0001
  repo_root: /abs/path/to/repo
  vcs:
    kind: git
    head: string
    branch: string
    dirty: boolean
    diff_artifact_ref: ArtifactRef | null
  file_snapshot:
    included_paths: string[]
    ignored_paths: string[]
    tree_hash: string
  approval_scope_refs: string[]
  captured_at: string
```

Relevant HEAD, dirty diff, allowed path content, approval scope, verification bundle input이 captured baseline과 더 이상 match하지 않으면 baseline은 stale이다. Stale baseline은 affected record에 따라 approval, evidence, verification을 stale로 mark할 수 있다.

## Verification Bundle Shape

`harness.launch_verify`는 detached verification 또는 manual evaluator handoff를 위한 bundle artifact를 만든다. Bundle은 raw evidence metadata이지 Eval verdict가 아니다.

최소 bundle contents:

```text
verify-bundle/
  manifest.json
  task-summary.json
  change-unit.json
  baseline.json
  evidence-manifest.json
  approvals.json
  run-refs.json
  artifact-refs.json
  design-refs.json
  evaluator-instructions.md
```

Manifest는 task id, Change Unit id, baseline ref, source state version, included artifact id, redaction summary, evaluator focus, expected independence context를 기록한다. Retention과 redaction policy가 허용하면 bundle은 copied raw artifact를 포함할 수 있다. 아니면 evaluator가 harness를 통해 resolve할 수 있는 artifact ref를 포함한다.

Verification launch는 `verification_gate=pending`을 set 또는 keep한다. Verdict를 기록하고 assurance를 update할 수 있는 것은 `harness.record_eval`뿐이다.

## Projection Jobs

Projection job은 committed state와 Product Repository Markdown file 사이의 durable outbox다. 위의 `projection_jobs` table이 job persistence를 담당한다.

MVP job lifecycle:

```text
pending -> running -> completed
pending -> running -> failed -> pending
pending -> skipped
```

규칙:

- older projection version을 newer version 위에 render하지 않는다
- human-editable section을 preserve한다
- overwrite 전에 managed hash를 compare한다
- managed drift에는 reconcile item을 생성한다
- projection failure를 Task result와 분리해 둔다

### Projection Worker Execution

Reference projector는 Core transaction commit 뒤 pending job을 실행한다.

MVP worker step:

1. Target projection의 oldest `pending` job을 select하고 projection-job lock을 acquire한다.
2. Job을 `running`으로 mark하고 latest state record, artifact ref, previous managed hash를 read한다.
3. Job의 `projection_version`이 target의 current projection version보다 older이면 `skipped`로 mark한다.
4. Committed record와 artifact ref에서 managed block을 render한다.
5. Existing managed block hash가 last recorded hash와 다르면 `reconcile_items` row를 create/update하고, job을 `skipped`로 mark하고, projection status를 `stale`로 set한다.
6. Human-editable section을 preserve하고 temporary file과 atomic rename으로 projection을 write한다.
7. New managed hash, output path, projected version, `completed` status를 record한다.
8. Render 또는 write failure에서는 job을 `failed`로 mark하고 state result를 unchanged로 유지하며 projection freshness를 `failed` 또는 `stale`로 surface한다.

Projection refresh는 newer attempt count를 가진 `pending` job을 만들거나 reset해 `failed` job을 retry한다. Reconcile이 drift를 resolve할 때까지 managed block이 drifted된 projection을 overwrite하면 안 된다.

## Reference Surface Behavior

Reference surface는 단일 MVP agent integration target이다. Broad surface support를 주장하지 않고 kernel을 demonstrate한다.

Required reference behavior:

- repository rule과 harness instruction을 read할 수 있음
- MCP tool과 resource를 call할 수 있음
- tracked work 전에 `harness.intake`를 call함
- product write 전에 `harness.prepare_write`를 call함
- `harness.record_run`을 통해 run을 기록함
- diff/log/bundle에 artifact ref를 사용함
- MCP를 통해 approval/scope/user decision을 request함
- `harness.launch_verify`를 통해 verification을 launch 또는 prepare함
- same-session self-review로 detached verification을 claim하지 않음
- MCP가 unavailable이면 product write를 hold함

Default guarantee display는 cooperative/detective다. Preventive 또는 isolated claim에는 implemented guard 또는 isolation path와 passing `surface_capability_check`가 필요하다.

## Validator Runner Skeleton

MVP validator는 API document의 shared result shape를 사용한다. Runner는 의도적으로 작다.

```text
run_validators(context, validator_ids):
  results = []
  for validator_id in validator_ids:
    load validator definition
    read only the state/artifact/repo inputs declared by the validator
    execute validator
    normalize output to ValidatorResult
    persist result in validator_runs
    results.append(result)
  return results
```

MVP validator set:

| Validator | Purpose |
|---|---|
| `active_task` | required한 곳에서 Task가 exists하고 non-terminal인지 확인 |
| `active_change_unit` | write-capable run에 active scoped Change Unit이 있는지 확인 |
| `changed_paths` | observed 또는 intended path가 scope 안에 머무는지 확인 |
| `approval_scope` | sensitive path/tool/network/secret이 granted approval에 맞는지 확인 |
| `baseline_freshness` | baseline이 relevant repo state와 여전히 match하는지 확인 |
| `artifact_integrity` | artifact file이 존재하고 hash/size와 match하는지 확인 |
| `evidence_sufficiency` | acceptance criteria가 supporting evidence에 map되는지 확인 |
| `same_session_verify_guard` | same-session review가 assurance를 upgrade하지 못하게 함 |
| `manual_qa_required` | required QA가 passed 또는 validly waived인지 확인 |
| `docs_consistency` | projection version과 managed hash가 consistent한지 확인 |
| `surface_capability_check` | connected surface가 required behavior를 satisfy할 수 있는지 확인 |
| `vertical_slice_shape` | required vertical slice 또는 exception이 recorded되었는지 확인 |
| `tdd_trace` | required TDD evidence 또는 allowed waiver가 있는지 확인 |
| `module_boundary_review` | module/interface review requirement가 충족됐는지 확인 |

Validator failure는 state, blocked reason, close blocker로 visible해야 한다. Prose-only agent output 안에 숨기면 안 된다.

## Minimal CLI Plan

MVP CLI는 같은 Core logic 위의 operator/debug surface다. State semantic이 다른 두 번째 API가 되면 안 된다.

Minimum entrypoint:

- 하나의 local project와 reference surface connect
- MCP server connection information을 start 또는 print
- project/runtime/MCP/artifact/projection doctor
- projection refresh
- pending item reconcile
- interrupted run, stale projection, artifact registry mismatch recover
- Task bundle export
- conformance smoke fixture run

Detailed operator procedure는 operations and conformance document가 담당한다.

## Export Bundle Shape

Export는 review 또는 archival을 위해 state snapshot, projection snapshot, artifact ref를 package한다.

```text
export/
  manifest.json
  state/
    task.json
    runs.json
    approvals.json
    evidence-manifest.json
    evals.json
    manual-qa.json
  projections/
    TASK.md
    APR-*.md
    RUN-SUMMARY-*.md
    EVIDENCE-MANIFEST-*.md
    EVAL-*.md
    DIRECT-RESULT-*.md
  artifacts/
    ...
```

Raw secret value, unredacted sensitive log, PII는 export 전에 omitted 또는 redacted된다.
