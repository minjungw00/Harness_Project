-- Harness State Schema v04
-- This file defines the MVP reference SQLite schema.
-- Apply the registry section to registry.sqlite and the state section to each project state.sqlite.

PRAGMA foreign_keys = ON;

-- ============================================================
-- registry.sqlite
-- ============================================================

CREATE TABLE IF NOT EXISTS schema_migrations (
  version INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  applied_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);

CREATE TABLE IF NOT EXISTS projects (
  project_id TEXT PRIMARY KEY,
  display_name TEXT NOT NULL,
  repo_root TEXT NOT NULL,
  repo_root_fingerprint TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','archived','missing','disabled')),
  default_agent_surface TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  UNIQUE(repo_root_fingerprint)
);

CREATE TABLE IF NOT EXISTS project_surfaces (
  project_id TEXT NOT NULL,
  surface_id TEXT NOT NULL,
  surface_type TEXT NOT NULL CHECK (surface_type IN ('reference','codex','claude_code','gemini','copilot','cursor','other')),
  target_profile TEXT NOT NULL,
  enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0,1)),
  support_tier TEXT NOT NULL DEFAULT 'T0' CHECK (support_tier IN ('T0','T1','T2','T3','T4','T5')),
  config_ref TEXT,
  last_verified_at TEXT,
  capability_profile_version INTEGER NOT NULL DEFAULT 1,
  capabilities_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(capabilities_json)),
  risks_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(risks_json)),
  fallbacks_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(fallbacks_json)),
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  PRIMARY KEY(project_id, surface_id),
  FOREIGN KEY(project_id) REFERENCES projects(project_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS connector_manifests (
  connector_id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  surface_id TEXT NOT NULL,
  generated_path TEXT NOT NULL,
  content_hash TEXT,
  managed_block_hash TEXT,
  status TEXT NOT NULL DEFAULT 'current' CHECK (status IN ('current','stale','drifted','failed','removed')),
  metadata_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(metadata_json)),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY(project_id) REFERENCES projects(project_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_project_surfaces_project ON project_surfaces(project_id);
CREATE INDEX IF NOT EXISTS idx_connector_manifests_project_surface ON connector_manifests(project_id, surface_id);

-- ============================================================
-- state.sqlite
-- ============================================================

CREATE TABLE IF NOT EXISTS state_schema_migrations (
  version INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  applied_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);

CREATE TABLE IF NOT EXISTS tasks (
  task_id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  mode TEXT NOT NULL CHECK (mode IN ('advisor','direct','work')),
  phase TEXT NOT NULL CHECK (phase IN ('intake','shaping','ready','executing','verifying','waiting_user','blocked','completed','cancelled')),
  result TEXT NOT NULL DEFAULT 'none' CHECK (result IN ('none','advice_only','passed','failed','cancelled')),
  assurance_level TEXT NOT NULL DEFAULT 'none' CHECK (assurance_level IN ('none','self_checked','detached_verified')),
  approval_state TEXT NOT NULL DEFAULT 'not_required' CHECK (approval_state IN ('not_required','pending','granted','denied','expired')),
  acceptance_state TEXT NOT NULL DEFAULT 'not_requested' CHECK (acceptance_state IN ('not_requested','pending','accepted','rejected')),
  risk_level TEXT NOT NULL DEFAULT 'low' CHECK (risk_level IN ('low','medium','high')),
  evidence_state TEXT NOT NULL DEFAULT 'none' CHECK (evidence_state IN ('none','partial','sufficient','stale')),

  state_version INTEGER NOT NULL DEFAULT 1 CHECK (state_version > 0),
  current_change_unit_id TEXT,
  active_run_id TEXT,
  active_profile TEXT,
  active_surface_id TEXT,
  next_action TEXT NOT NULL DEFAULT '',
  pending_decision_summary TEXT,

  latest_eval_artifact_id TEXT,
  latest_direct_result_artifact_id TEXT,
  latest_run_summary_artifact_id TEXT,
  latest_evidence_manifest_id TEXT,
  latest_checkpoint_artifact_id TEXT,
  latest_approval_id TEXT,

  projection_version INTEGER NOT NULL DEFAULT 1 CHECK (projection_version > 0),
  projected_version INTEGER NOT NULL DEFAULT 0 CHECK (projected_version >= 0),
  projection_freshness TEXT NOT NULL DEFAULT 'stale' CHECK (projection_freshness IN ('current','stale','failed','unknown')),

  last_baseline_ref TEXT,
  repo_task_doc_path TEXT,
  mcp_session_ref TEXT,
  conversation_surface_ref TEXT,

  goal_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(goal_json)),
  scope_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(scope_json)),
  acceptance_criteria_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(acceptance_criteria_json)),
  spine_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(spine_json)),

  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  completed_at TEXT
);

CREATE TABLE IF NOT EXISTS change_units (
  task_id TEXT NOT NULL,
  change_unit_id TEXT NOT NULL,
  title TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'planned' CHECK (status IN ('planned','ready','executing','implemented','verifying','verified','blocked','deferred','cancelled','completed')),
  purpose TEXT NOT NULL DEFAULT '',
  non_goals_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(non_goals_json)),
  allowed_paths_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(allowed_paths_json)),
  allowed_tools_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(allowed_tools_json)),
  approval_categories_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(approval_categories_json)),
  validator_profile_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(validator_profile_json)),
  done_definition_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(done_definition_json)),
  eval_focus_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(eval_focus_json)),
  risk_flags_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(risk_flags_json)),
  current_run_id TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  PRIMARY KEY(task_id, change_unit_id),
  FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS baselines (
  baseline_ref TEXT PRIMARY KEY,
  task_id TEXT NOT NULL,
  run_id TEXT,
  head_ref TEXT,
  branch_ref TEXT,
  worktree_ref TEXT,
  has_uncommitted_changes INTEGER NOT NULL CHECK (has_uncommitted_changes IN (0,1)),
  file_hash_summary_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(file_hash_summary_json)),
  lockfile_hash TEXT,
  config_hash TEXT,
  captured_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS runs (
  run_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL,
  change_unit_id TEXT,
  profile TEXT NOT NULL CHECK (profile IN ('advisor','direct','lead','evaluator','system','operator')),
  action TEXT NOT NULL CHECK (action IN ('advise','shape','implement','verify','recover','reconcile','export','doctor')),
  surface_id TEXT NOT NULL,
  mcp_session_ref TEXT,
  write_capable INTEGER NOT NULL DEFAULT 0 CHECK (write_capable IN (0,1)),
  writes_product_files INTEGER NOT NULL DEFAULT 0 CHECK (writes_product_files IN (0,1)),
  started_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  finished_at TEXT,
  outcome TEXT NOT NULL DEFAULT 'running' CHECK (outcome IN ('running','passed','failed','blocked','cancelled','interrupted')),
  baseline_ref TEXT,
  parent_run_id TEXT,
  source_bundle_artifact_id TEXT,
  checkpoint_artifact_id TEXT,
  verification_independence_json TEXT CHECK (verification_independence_json IS NULL OR json_valid(verification_independence_json)),
  metadata_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(metadata_json)),
  FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE CASCADE,
  FOREIGN KEY(task_id, change_unit_id) REFERENCES change_units(task_id, change_unit_id),
  FOREIGN KEY(parent_run_id) REFERENCES runs(run_id)
);

CREATE TABLE IF NOT EXISTS approvals (
  approval_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL,
  category TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','granted','denied','expired','superseded')),
  summary TEXT NOT NULL,
  allowed_paths_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(allowed_paths_json)),
  allowed_tools_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(allowed_tools_json)),
  allowed_network_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(allowed_network_json)),
  required_secrets_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(required_secrets_json)),
  baseline_ref TEXT,
  diff_envelope_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(diff_envelope_json)),
  expires_on_scope_drift INTEGER NOT NULL DEFAULT 1 CHECK (expires_on_scope_drift IN (0,1)),
  alternatives_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(alternatives_json)),
  recommendation TEXT,
  requested_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  decided_at TEXT,
  decision_note TEXT,
  decided_by TEXT,
  approval_doc_path TEXT,
  superseded_by TEXT,
  FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE CASCADE,
  FOREIGN KEY(superseded_by) REFERENCES approvals(approval_id)
);

CREATE TABLE IF NOT EXISTS evidence_manifests (
  evidence_manifest_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL,
  change_unit_id TEXT,
  baseline_ref TEXT,
  status TEXT NOT NULL DEFAULT 'partial' CHECK (status IN ('none','partial','sufficient','stale')),
  manifest_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(manifest_json)),
  repo_doc_path TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE CASCADE,
  FOREIGN KEY(task_id, change_unit_id) REFERENCES change_units(task_id, change_unit_id)
);

CREATE TABLE IF NOT EXISTS decisions (
  decision_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL,
  title TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'proposed' CHECK (status IN ('proposed','accepted','rejected','deferred','superseded')),
  problem TEXT,
  options_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(options_json)),
  recommendation TEXT,
  final_decision TEXT,
  impact_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(impact_json)),
  repo_doc_path TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS artifacts (
  artifact_id TEXT PRIMARY KEY,
  task_id TEXT,
  run_id TEXT,
  kind TEXT NOT NULL CHECK (kind IN (
    'task_doc','decision_doc','approval_doc','design_doc','run_summary_doc','eval_doc','direct_result_doc','evidence_manifest_doc',
    'baseline','diff','log','checkpoint','bundle','command_result','validator_output','export','connector_manifest','other'
  )),
  path TEXT NOT NULL,
  sha256 TEXT NOT NULL,
  size_bytes INTEGER NOT NULL CHECK (size_bytes >= 0),
  content_type TEXT NOT NULL,
  retention_class TEXT NOT NULL CHECK (retention_class IN ('short','standard','long','permanent')),
  redaction_state TEXT NOT NULL DEFAULT 'not_needed' CHECK (redaction_state IN ('not_needed','redacted','blocked_sensitive')),
  metadata_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(metadata_json)),
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE SET NULL,
  FOREIGN KEY(run_id) REFERENCES runs(run_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS validator_runs (
  validator_run_id TEXT PRIMARY KEY,
  run_id TEXT,
  task_id TEXT NOT NULL,
  change_unit_id TEXT,
  validator_name TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('passed','failed','blocked','warning','skipped')),
  hard_block INTEGER NOT NULL DEFAULT 0 CHECK (hard_block IN (0,1)),
  output_artifact_id TEXT,
  summary TEXT,
  executed_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY(run_id) REFERENCES runs(run_id) ON DELETE SET NULL,
  FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE CASCADE,
  FOREIGN KEY(output_artifact_id) REFERENCES artifacts(artifact_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS task_events (
  event_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL,
  event_type TEXT NOT NULL,
  actor_kind TEXT NOT NULL CHECK (actor_kind IN ('agent','user','system','operator')),
  actor_ref TEXT NOT NULL,
  request_id TEXT,
  idempotency_key TEXT,
  state_version_before INTEGER,
  state_version_after INTEGER,
  event_payload_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(event_payload_json)),
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS projection_jobs (
  job_id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL,
  target_kind TEXT NOT NULL CHECK (target_kind IN ('task','decision','approval','design','run_summary','eval','direct_result','evidence_manifest','agents_rule','skill','surface_status')),
  target_path TEXT NOT NULL,
  target_version INTEGER NOT NULL CHECK (target_version > 0),
  status TEXT NOT NULL DEFAULT 'queued' CHECK (status IN ('queued','running','succeeded','failed','superseded')),
  attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
  last_error TEXT,
  enqueued_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS reconcile_items (
  reconcile_id TEXT PRIMARY KEY,
  task_id TEXT,
  doc_path TEXT NOT NULL,
  section TEXT NOT NULL CHECK (section IN ('managed','human_editable','front_matter','connector_generated','unknown')),
  detected_change_hash TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','merged','rejected','converted_to_note','superseded')),
  suggested_event_type TEXT,
  detected_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  resolved_at TEXT,
  resolution_note TEXT,
  FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS doc_refs (
  task_id TEXT NOT NULL,
  doc_kind TEXT NOT NULL CHECK (doc_kind IN ('task','decision','approval','design','run_summary','eval','direct_result','evidence_manifest')),
  doc_id TEXT NOT NULL,
  path TEXT NOT NULL,
  artifact_id TEXT,
  projection_version INTEGER NOT NULL DEFAULT 1,
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  PRIMARY KEY(task_id, doc_kind, doc_id),
  FOREIGN KEY(task_id) REFERENCES tasks(task_id) ON DELETE CASCADE,
  FOREIGN KEY(artifact_id) REFERENCES artifacts(artifact_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS mcp_sessions (
  mcp_session_id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  surface_id TEXT NOT NULL,
  started_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  last_seen_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  capabilities_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(capabilities_json)),
  status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','stale','closed','failed'))
);

CREATE TABLE IF NOT EXISTS idempotency_keys (
  idempotency_key TEXT PRIMARY KEY,
  request_hash TEXT NOT NULL,
  actor_kind TEXT NOT NULL CHECK (actor_kind IN ('agent','user','system','operator')),
  actor_ref TEXT NOT NULL,
  tool_name TEXT NOT NULL,
  task_id TEXT,
  response_json TEXT NOT NULL CHECK (json_valid(response_json)),
  state_version INTEGER,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  expires_at TEXT
);

CREATE TABLE IF NOT EXISTS locks (
  lock_name TEXT PRIMARY KEY,
  owner_ref TEXT NOT NULL,
  acquired_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  expires_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_tasks_phase ON tasks(phase);
CREATE INDEX IF NOT EXISTS idx_tasks_updated ON tasks(updated_at);
CREATE INDEX IF NOT EXISTS idx_change_units_task_status ON change_units(task_id, status);
CREATE INDEX IF NOT EXISTS idx_runs_task ON runs(task_id);
CREATE INDEX IF NOT EXISTS idx_runs_change_unit ON runs(task_id, change_unit_id);
CREATE INDEX IF NOT EXISTS idx_approvals_task_status ON approvals(task_id, status);
CREATE INDEX IF NOT EXISTS idx_evidence_task_cu ON evidence_manifests(task_id, change_unit_id);
CREATE INDEX IF NOT EXISTS idx_artifacts_task_kind ON artifacts(task_id, kind);
CREATE INDEX IF NOT EXISTS idx_validator_runs_task_name ON validator_runs(task_id, validator_name);
CREATE INDEX IF NOT EXISTS idx_task_events_task_created ON task_events(task_id, created_at);
CREATE INDEX IF NOT EXISTS idx_projection_jobs_status ON projection_jobs(status, enqueued_at);
CREATE INDEX IF NOT EXISTS idx_reconcile_items_status ON reconcile_items(status, detected_at);
CREATE INDEX IF NOT EXISTS idx_idempotency_task ON idempotency_keys(task_id);
