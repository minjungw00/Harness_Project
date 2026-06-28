# 저장소 DDL

이 문서는 [저장소 기록](storage-records.md)이 설명하는 저장소 배치를 위한 기준 SQLite DDL 계약을 담당합니다. 기준 `registry.sqlite`와 프로젝트 `state.sqlite` 배치를 구현할 수 있게 하되, 메서드 효과, 아티팩트 생명주기 규칙, 상태 버전 의미, API 스키마, 보안 보장을 이 문서로 옮기지 않습니다.

## 담당 경계

이 문서가 담당합니다.

- `registry.sqlite`와 프로젝트 `state.sqlite`의 기준 SQLite 테이블 형태
- 기준 인덱스, 외래 키, 마이그레이션 테이블, 물리 제약
- `project_state.state_version`, 재실행 행, 현재 적용 Change Unit 고유성, Write Check 기준 버전, 스테이징된 아티팩트 출처에 대한 SQLite 제약
- Runtime Home 등록 데이터와 프로젝트별 Core 상태 사이의 DDL 수준 분리

이 문서는 담당하지 않습니다.

- 기록 계열 목적, 저장 위치, 저장소 소유 값, JSON 배치 범주: [저장소 기록](storage-records.md)
- 메서드 분기별 저장 효과: [저장 효과](storage-effects.md)
- 아티팩트 스테이징, 승격, 연결, 본문 읽기, 보존, 무결성 생명주기: [아티팩트 저장소](storage-artifacts.md)
- 상태 버전, 멱등성, 이벤트, 잠금, 마이그레이션 의미: [저장소 버전 관리](storage-versioning.md)
- API 요청 또는 응답 스키마: [API 코어 스키마](api/schema-core.md)가 안내하는 API 스키마 담당 문서
- 런타임 위치 경계: [런타임 경계](runtime-boundaries.md)
- 보안 보장 수준: [보안](security.md)

## 연결과 트랜잭션 요구사항

SQLite 외래 키는 이 DDL 계약의 일부입니다. 이 데이터베이스들을 읽거나 쓰는 모든 연결은 아래 설정을 활성화해야 합니다.

```sql
PRAGMA foreign_keys = ON;
```

상태 변경 커밋을 위해 최신성, Write Check, 스테이징, 재실행 행을 읽는 변경 트랜잭션은 `BEGIN IMMEDIATE` 또는 동등한 직렬화된 쓰기 경계를 사용해야 합니다.

기준 테이블은 `ON DELETE CASCADE`를 사용하지 않습니다. 담당 저장소 또는 마이그레이션 계약이 복구나 보존 경로를 정의하지 않는 한 권한을 지닌 행은 계속 주소 지정 가능해야 합니다.

`_json`으로 끝나는 SQLite `TEXT` 열은 JSON을 저장하는 표현 선택입니다. 권한, 생명주기, 범위, 증거, 완료, 닫기 준비 상태, 쓰기 호환성에 쓰이는 JSON은 타입이 지정된 담당 상태입니다. 타입을 아는 Core 코드는 커밋 전에 해당 API 스키마 담당 문서, 저장소 담당 문서, 또는 아티팩트 담당 문서에 맞게 이 열을 파싱하고 검증해야 합니다. 타입이 지정된 담당 상태를 디코드하지 못하는 경우는 손상이며 빈 객체, 빈 배열, false 값, 기본 enum, 또는 "요구사항 없음" 해석으로 바꾸면 안 됩니다. SQL `NULL`은 담당 스키마가 그 필드를 명시적으로 선택 필드라고 표시할 때만 부재를 뜻할 수 있습니다. 선택 열의 형식이 잘못된 JSON도 부재가 아니라 손상입니다. 열린 표시 메타데이터는 권한이나 닫기 판단에 쓰이지 않을 때만 타입을 지정하지 않은 채로 둘 수 있습니다. 안전한 진단은 테이블, 기록 참조, 논리 열, 손상 범주를 식별할 수 있지만 원본 저장 JSON, 비밀값, SQL 텍스트, 민감한 절대 경로를 노출하면 안 됩니다. `'{}'`, `'[]'` 같은 SQLite 기본값은 API 필드를 선택 필드로 만들지 않습니다.

`project_state.state_version`은 기준 범위의 유일한 공개 상태 시계입니다. 기준 SQLite DDL은 `tasks.state_version`을 만들면 안 됩니다.

Write Check 행은 제품 파일 쓰기 시도에 대한 Core 상태 호환성을 기록합니다. OS 권한, 파일시스템 ACL, sandboxing, 네트워크 정책, 비밀 격리가 아닙니다.

## `registry.sqlite`

`registry.sqlite`는 Runtime Home 식별 정보, setup 프로필 기록, 프로젝트 등록, Agent Connection 기록, Connection Projects 멤버십, 호스트 설정 인벤토리를 저장합니다. 프로젝트별 Core 상태는 저장하지 않습니다.

아래 DDL은 저장소 프로필 `baseline_sqlite_v3` 스키마 버전 `1`의 초기 물리 registry 스키마입니다. 저장소 프로필과 마이그레이션 경계 동작은 [저장소 버전 관리](storage-versioning.md)가 담당합니다.

```sql
CREATE TABLE schema_migrations (
  database_kind TEXT NOT NULL CHECK (database_kind = 'registry'),
  version INTEGER NOT NULL CHECK (version > 0),
  name TEXT NOT NULL,
  storage_profile TEXT NOT NULL,
  applied_at TEXT NOT NULL,
  checksum_sha256 TEXT,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (database_kind, version)
);

CREATE TABLE runtime_home (
  singleton_id INTEGER PRIMARY KEY CHECK (singleton_id = 1),
  runtime_home_id TEXT NOT NULL UNIQUE,
  storage_profile TEXT NOT NULL,
  schema_version INTEGER NOT NULL CHECK (schema_version > 0),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}'
);

CREATE TABLE setup_profile (
  singleton_id INTEGER PRIMARY KEY CHECK (singleton_id = 1),
  runtime_home_id TEXT NOT NULL,
  mcp_command TEXT NOT NULL,
  linked_bin_path TEXT,
  installation_profile_json TEXT NOT NULL DEFAULT '{}',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  FOREIGN KEY (runtime_home_id) REFERENCES runtime_home (runtime_home_id)
);

CREATE TABLE projects (
  project_id TEXT PRIMARY KEY,
  runtime_home_id TEXT NOT NULL,
  project_name TEXT NOT NULL UNIQUE,
  repo_root TEXT NOT NULL,
  project_home TEXT NOT NULL UNIQUE,
  state_db_path TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'active' CHECK (status = 'active'),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  FOREIGN KEY (runtime_home_id) REFERENCES runtime_home (runtime_home_id)
);

CREATE UNIQUE INDEX idx_projects_repo_root_unique ON projects (repo_root);
CREATE INDEX idx_projects_status ON projects (status);

CREATE TABLE agent_connections (
  connection_id TEXT PRIMARY KEY,
  host_kind TEXT NOT NULL CHECK (host_kind IN ('codex', 'claude_code', 'generic')),
  connection_intent TEXT NOT NULL
    CHECK (connection_intent IN ('personal', 'shared', 'global', 'export')),
  server_name TEXT NOT NULL DEFAULT 'volicord',
  config_target TEXT NOT NULL,
  mode TEXT NOT NULL DEFAULT 'workflow' CHECK (mode IN ('workflow', 'read_only')),
  enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
  managed_fingerprint TEXT NOT NULL,
  last_verified_status TEXT NOT NULL DEFAULT 'not_verified'
    CHECK (last_verified_status IN ('not_verified', 'complete', 'action_required', 'failed')),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  CHECK (
    (host_kind IN ('codex', 'claude_code')
      AND connection_intent IN ('personal', 'shared', 'global'))
    OR (host_kind = 'generic' AND connection_intent = 'export')
  )
);

CREATE TABLE connection_projects (
  connection_id TEXT NOT NULL,
  project_id TEXT NOT NULL,
  created_at TEXT NOT NULL,
  PRIMARY KEY (connection_id, project_id),
  FOREIGN KEY (connection_id)
    REFERENCES agent_connections (connection_id)
    ON DELETE RESTRICT
    DEFERRABLE INITIALLY DEFERRED,
  FOREIGN KEY (project_id) REFERENCES projects (project_id) ON DELETE RESTRICT
);

CREATE INDEX idx_connection_projects_project
  ON connection_projects (project_id);
CREATE INDEX idx_agent_connections_enabled
  ON agent_connections (enabled);
CREATE UNIQUE INDEX idx_agent_connections_target
  ON agent_connections (host_kind, connection_intent, config_target, server_name);
```

Registry 제약:

- `runtime_home`은 단일 행 테이블입니다. 저장된 `runtime_home_id`는 Runtime Home 기록을 식별하며 보안 보장이 아닙니다.
- `setup_profile`은 단일 행 테이블입니다. setup 시점의 MCP 명령과 설치 프로필 메타데이터를 저장합니다. 호스트 신뢰, 사용자 권한, 공개 API 상태가 아닙니다.
- `projects.project_name`, `projects.repo_root`, `projects.project_home`은 고유합니다. `repo_root`는 사용자 대상 저장소 루트 조회 키이며 내부 프로젝트 식별을 대신하지 않습니다.
- `projects.state_db_path`는 저장 열로 유지됩니다. Store 애플리케이션 수준 현재 등록 검증은 운영 `ProjectRecord` 조회나 목록 조회, 프로젝트 상태 마이그레이션 또는 쓰기 가능 열기, Agent Connection 프로젝트 라우팅, Core 실행, setup 재사용, MCP 프로젝트 가용성 전에 이 값이 `project_home/state.sqlite`와 같은지 확인해야 합니다.
- `projects.status`는 저장소 소유 값이며 기준 범위에서 유효한 값은 `active`뿐입니다.
- `agent_connections`는 로컬 MCP 호스트 연결 단위 하나, 그 연결 의도, 모드, 활성 상태, 호스트 설정 인벤토리, 마지막 검증 상태를 저장합니다.
- `agent_connections.connection_intent`는 `personal`, `shared`, `global`, 또는 generic MCP 설정 export를 위한 내부 export 상태로 제한됩니다.
- `agent_connections.server_name`의 기본값은 내부 호스트 설정 키 `volicord`입니다.
- `agent_connections.mode`는 `workflow` 또는 `read_only`로 제한되며 기본값은 `workflow`입니다.
- `connection_projects`는 Agent Connection 하나에 대한 명시적 프로젝트 허용 목록입니다. 아직 멤버십이 남은 프로젝트나 연결 삭제는 제한됩니다.
- `agent_connections.host_kind`와 `agent_connections.connection_intent`는 [관리 CLI](admin-cli.md)가 정의하는 지원 직접 호스트 연결 의도와 별도 generic export 흐름으로 제한됩니다.
- `schema_migrations`는 적용된 registry 스키마 버전을 기록합니다. 마이그레이션 실행 의미는 [저장소 버전 관리](storage-versioning.md)가 담당합니다.

## 프로젝트 `state.sqlite`

등록된 프로젝트마다 프로젝트별 `state.sqlite`가 하나 있습니다. 이 데이터베이스는 그 프로젝트의 Core 상태를 저장하며, 외래 키와 인덱스가 같은 프로젝트 관계를 강제할 수 있도록 프로젝트 범위 행에 `project_id`를 반복해 저장합니다.

아래 DDL은 저장소 프로필 `baseline_sqlite_v3` 스키마 버전 `1`의 초기 물리 프로젝트 상태 스키마입니다. 저장소 프로필과 마이그레이션 경계 동작은 [저장소 버전 관리](storage-versioning.md)가 담당합니다.

```sql
CREATE TABLE schema_migrations (
  database_kind TEXT NOT NULL CHECK (database_kind = 'project_state'),
  version INTEGER NOT NULL CHECK (version > 0),
  name TEXT NOT NULL,
  storage_profile TEXT NOT NULL,
  applied_at TEXT NOT NULL,
  checksum_sha256 TEXT,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (database_kind, version)
);

CREATE TABLE project_state (
  project_id TEXT PRIMARY KEY,
  storage_profile TEXT NOT NULL,
  schema_version INTEGER NOT NULL CHECK (schema_version > 0),
  state_version INTEGER NOT NULL DEFAULT 0 CHECK (state_version >= 0),
  active_task_id TEXT,
  enforcement_profile_json TEXT NOT NULL DEFAULT '{"profile_id":"baseline_cooperative","guarantee_level":"cooperative","enabled_mechanisms":[],"source":"baseline_scope","status":"active"}',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  FOREIGN KEY (project_id, active_task_id)
    REFERENCES tasks (project_id, task_id)
    DEFERRABLE INITIALLY DEFERRED
);

CREATE TABLE tasks (
  project_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  created_by_actor_source TEXT NOT NULL,
  mode TEXT NOT NULL,
  lifecycle_phase TEXT NOT NULL,
  result TEXT,
  title TEXT,
  summary TEXT,
  shaping_summary_json TEXT NOT NULL DEFAULT '{}',
  bounded_context_json TEXT NOT NULL DEFAULT '[]',
  autonomy_boundary_json TEXT NOT NULL DEFAULT '{}',
  scope_revision INTEGER NOT NULL DEFAULT 0 CHECK (scope_revision >= 0),
  close_basis_revision INTEGER NOT NULL DEFAULT 0 CHECK (close_basis_revision >= 0),
  close_basis_json TEXT,
  close_summary_json TEXT NOT NULL DEFAULT '{}',
  completion_policy_json TEXT NOT NULL DEFAULT '{}',
  current_change_unit_id TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  closed_at TEXT,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (project_id, task_id),
  FOREIGN KEY (project_id) REFERENCES project_state (project_id),
  FOREIGN KEY (project_id, task_id, current_change_unit_id)
    REFERENCES change_units (project_id, task_id, change_unit_id)
    DEFERRABLE INITIALLY DEFERRED
);

CREATE TABLE change_units (
  project_id TEXT NOT NULL,
  change_unit_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('proposed', 'active', 'replaced', 'closed')),
  is_current INTEGER NOT NULL DEFAULT 0 CHECK (is_current IN (0, 1)),
  basis_state_version INTEGER CHECK (basis_state_version >= 0),
  scope_summary_json TEXT NOT NULL DEFAULT '{}',
  bounded_paths_json TEXT NOT NULL DEFAULT '[]',
  write_basis_json TEXT NOT NULL DEFAULT '{}',
  effect_contract_json TEXT NOT NULL DEFAULT 'null',
  lifecycle_json TEXT NOT NULL DEFAULT '{}',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  closed_at TEXT,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (project_id, change_unit_id),
  UNIQUE (project_id, task_id, change_unit_id),
  FOREIGN KEY (project_id, task_id) REFERENCES tasks (project_id, task_id)
);

CREATE UNIQUE INDEX idx_change_units_one_current_active
  ON change_units (project_id, task_id)
  WHERE status = 'active' AND is_current = 1;

CREATE TABLE user_judgments (
  project_id TEXT NOT NULL,
  judgment_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  change_unit_id TEXT,
  judgment_kind TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('pending', 'resolved', 'stale', 'superseded', 'expired')),
  request_json TEXT NOT NULL DEFAULT '{}',
  context_json TEXT NOT NULL DEFAULT '{}',
  options_json TEXT NOT NULL DEFAULT '{"schema_version":1,"options":[]}',
  affected_refs_json TEXT NOT NULL DEFAULT '[]',
  artifact_refs_json TEXT NOT NULL DEFAULT '[]',
  sensitive_action_scope_json TEXT NOT NULL DEFAULT '{}',
  basis_json TEXT NOT NULL,
  basis_status TEXT NOT NULL DEFAULT 'current'
    CHECK (basis_status IN ('current', 'stale', 'superseded')),
  resolution_outcome TEXT
    CHECK (resolution_outcome IS NULL OR resolution_outcome IN ('accepted', 'rejected', 'deferred')),
  resolution_machine_action TEXT
    CHECK (resolution_machine_action IS NULL OR resolution_machine_action IN ('accept', 'reject', 'defer')),
  resolution_json TEXT,
  resolution_rationale_json TEXT,
  requested_by_actor_source TEXT NOT NULL,
  resolved_by_actor_source TEXT,
  resolved_verification_basis TEXT,
  resolved_assurance_level TEXT,
  requested_at TEXT NOT NULL,
  resolved_at TEXT,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (project_id, judgment_id),
  CHECK (
    (
      status IN ('pending', 'expired')
      AND resolution_outcome IS NULL
      AND resolution_machine_action IS NULL
      AND resolution_json IS NULL
      AND resolution_rationale_json IS NULL
      AND resolved_by_actor_source IS NULL
      AND resolved_verification_basis IS NULL
      AND resolved_assurance_level IS NULL
      AND resolved_at IS NULL
    )
    OR (
      status = 'resolved'
      AND resolution_outcome IS NOT NULL
      AND resolution_machine_action IS NOT NULL
      AND resolution_json IS NOT NULL
      AND resolution_rationale_json IS NOT NULL
      AND resolved_by_actor_source IS NOT NULL
      AND resolved_verification_basis IS NOT NULL
      AND resolved_assurance_level IS NOT NULL
      AND resolved_at IS NOT NULL
    )
    OR (
      status IN ('stale', 'superseded')
      AND (
        (
          resolution_outcome IS NULL
          AND resolution_machine_action IS NULL
          AND resolution_json IS NULL
          AND resolution_rationale_json IS NULL
          AND resolved_by_actor_source IS NULL
          AND resolved_verification_basis IS NULL
          AND resolved_assurance_level IS NULL
          AND resolved_at IS NULL
        )
        OR (
          resolution_outcome IS NOT NULL
          AND resolution_machine_action IS NOT NULL
          AND resolution_json IS NOT NULL
          AND resolution_rationale_json IS NOT NULL
          AND resolved_by_actor_source IS NOT NULL
          AND resolved_verification_basis IS NOT NULL
          AND resolved_assurance_level IS NOT NULL
          AND resolved_at IS NOT NULL
        )
      )
    )
  ),
  CHECK (
    resolution_machine_action IS NULL
    OR (
      (resolution_machine_action = 'accept' AND resolution_outcome = 'accepted')
      OR (resolution_machine_action = 'reject' AND resolution_outcome = 'rejected')
      OR (resolution_machine_action = 'defer' AND resolution_outcome = 'deferred')
    )
  ),
  FOREIGN KEY (project_id, task_id) REFERENCES tasks (project_id, task_id),
  FOREIGN KEY (project_id, task_id, change_unit_id)
    REFERENCES change_units (project_id, task_id, change_unit_id)
);

CREATE TABLE project_continuity_records (
  project_id TEXT NOT NULL,
  continuity_record_id TEXT NOT NULL,
  source_task_id TEXT NOT NULL,
  source_change_unit_id TEXT,
  kind TEXT NOT NULL CHECK (kind IN ('decision', 'obligation', 'known_limit', 'accepted_risk', 'constraint')),
  title TEXT NOT NULL CHECK (length(trim(title)) > 0),
  summary TEXT NOT NULL CHECK (length(trim(summary)) > 0),
  rationale TEXT CHECK (rationale IS NULL OR length(trim(rationale)) > 0),
  applies_to_paths_json TEXT NOT NULL DEFAULT '[]',
  applies_to_refs_json TEXT NOT NULL DEFAULT '[]',
  source_refs_json TEXT NOT NULL DEFAULT '[]',
  artifact_refs_json TEXT NOT NULL DEFAULT '[]',
  status TEXT NOT NULL CHECK (status IN ('active', 'superseded', 'closed')),
  supersedes_refs_json TEXT NOT NULL DEFAULT '[]',
  review_triggers_json TEXT NOT NULL DEFAULT '[]',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (project_id, continuity_record_id),
  FOREIGN KEY (project_id) REFERENCES project_state (project_id),
  FOREIGN KEY (project_id, source_task_id) REFERENCES tasks (project_id, task_id),
  FOREIGN KEY (project_id, source_task_id, source_change_unit_id)
    REFERENCES change_units (project_id, task_id, change_unit_id)
);

CREATE TABLE write_checks (
  project_id TEXT NOT NULL,
  write_check_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  change_unit_id TEXT,
  basis_state_version INTEGER NOT NULL CHECK (basis_state_version > 0),
  status TEXT NOT NULL CHECK (status IN ('active', 'consumed', 'expired', 'stale', 'revoked')),
  attempt_scope_json TEXT NOT NULL DEFAULT '{}',
  created_by_actor_source TEXT NOT NULL,
  created_by_judgment_id TEXT,
  expires_at TEXT NOT NULL,
  consumed_by_run_id TEXT,
  consumed_at TEXT,
  revoked_at TEXT,
  created_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (project_id, write_check_id),
  FOREIGN KEY (project_id, task_id) REFERENCES tasks (project_id, task_id),
  FOREIGN KEY (project_id, task_id, change_unit_id)
    REFERENCES change_units (project_id, task_id, change_unit_id),
  FOREIGN KEY (project_id, created_by_judgment_id)
    REFERENCES user_judgments (project_id, judgment_id),
  FOREIGN KEY (project_id, consumed_by_run_id)
    REFERENCES runs (project_id, run_id)
    DEFERRABLE INITIALLY DEFERRED
);

CREATE UNIQUE INDEX idx_write_checks_consumed_run
  ON write_checks (project_id, consumed_by_run_id)
  WHERE consumed_by_run_id IS NOT NULL;

CREATE TABLE runs (
  project_id TEXT NOT NULL,
  run_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  change_unit_id TEXT,
  write_check_id TEXT,
  kind TEXT NOT NULL,
  status TEXT NOT NULL,
  summary_json TEXT NOT NULL DEFAULT '{}',
  observed_changes_json TEXT NOT NULL DEFAULT '{}',
  evidence_updates_json TEXT NOT NULL DEFAULT '[]',
  write_check_effect_json TEXT NOT NULL DEFAULT '{}',
  scope_revision INTEGER NOT NULL CHECK (scope_revision >= 0),
  created_by_actor_source TEXT NOT NULL,
  started_at TEXT,
  completed_at TEXT,
  created_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (project_id, run_id),
  FOREIGN KEY (project_id, task_id) REFERENCES tasks (project_id, task_id),
  FOREIGN KEY (project_id, task_id, change_unit_id)
    REFERENCES change_units (project_id, task_id, change_unit_id),
  FOREIGN KEY (project_id, write_check_id)
    REFERENCES write_checks (project_id, write_check_id)
    DEFERRABLE INITIALLY DEFERRED
);

CREATE UNIQUE INDEX idx_runs_write_check
  ON runs (project_id, write_check_id)
  WHERE write_check_id IS NOT NULL;

CREATE TABLE artifact_staging (
  project_id TEXT NOT NULL,
  handle_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  created_by_actor_source TEXT NOT NULL,
  artifact_json TEXT NOT NULL DEFAULT '{}',
  safe_metadata_json TEXT NOT NULL DEFAULT '{}',
  tmp_path TEXT,
  sha256 TEXT,
  size_bytes INTEGER CHECK (size_bytes IS NULL OR size_bytes >= 0),
  content_type TEXT,
  redaction_state TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('staged', 'consumed', 'expired', 'discarded')),
  expires_at TEXT NOT NULL,
  consumed_by_run_id TEXT,
  promoted_artifact_id TEXT,
  consumed_at TEXT,
  created_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (project_id, handle_id),
  FOREIGN KEY (project_id, task_id) REFERENCES tasks (project_id, task_id),
  FOREIGN KEY (project_id, consumed_by_run_id)
    REFERENCES runs (project_id, run_id)
    DEFERRABLE INITIALLY DEFERRED,
  FOREIGN KEY (project_id, promoted_artifact_id)
    REFERENCES artifacts (project_id, artifact_id)
    DEFERRABLE INITIALLY DEFERRED
);

CREATE UNIQUE INDEX idx_artifact_staging_promoted_artifact
  ON artifact_staging (project_id, promoted_artifact_id)
  WHERE promoted_artifact_id IS NOT NULL;

CREATE TABLE artifacts (
  project_id TEXT NOT NULL,
  artifact_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  producer_run_id TEXT,
  source_staging_handle_id TEXT,
  uri TEXT NOT NULL,
  body_path TEXT,
  sha256 TEXT,
  size_bytes INTEGER CHECK (size_bytes IS NULL OR size_bytes >= 0),
  content_type TEXT,
  integrity_status TEXT NOT NULL DEFAULT 'verified'
    CHECK (integrity_status IN ('verified', 'corrupt')),
  redaction_state TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('available', 'missing', 'integrity_failed', 'unavailable')),
  retention_json TEXT NOT NULL DEFAULT '{}',
  producer_json TEXT NOT NULL DEFAULT '{}',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (project_id, artifact_id),
  CHECK (
    integrity_status <> 'verified'
    OR (
      content_type IS NOT NULL
      AND length(trim(content_type)) > 0
      AND sha256 IS NOT NULL
      AND length(sha256) = 64
      AND sha256 NOT GLOB '*[^0-9a-f]*'
      AND size_bytes IS NOT NULL
      AND size_bytes >= 0
    )
  ),
  CHECK (
    body_path IS NULL
    OR (
      length(trim(body_path)) > 0
      AND body_path NOT GLOB '/*'
      AND body_path NOT GLOB '[A-Za-z]:*'
      AND instr(body_path, '\') = 0
      AND body_path <> '..'
      AND body_path NOT GLOB '../*'
      AND body_path NOT GLOB '*/../*'
      AND body_path NOT GLOB '*/..'
      AND body_path <> 'artifacts'
      AND body_path NOT GLOB 'artifacts/*'
    )
  ),
  FOREIGN KEY (project_id, task_id) REFERENCES tasks (project_id, task_id),
  FOREIGN KEY (project_id, producer_run_id) REFERENCES runs (project_id, run_id),
  FOREIGN KEY (project_id, source_staging_handle_id)
    REFERENCES artifact_staging (project_id, handle_id)
    DEFERRABLE INITIALLY DEFERRED
);

CREATE UNIQUE INDEX idx_artifacts_source_staging
  ON artifacts (project_id, source_staging_handle_id)
  WHERE source_staging_handle_id IS NOT NULL;

CREATE TABLE artifact_links (
  project_id TEXT NOT NULL,
  artifact_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  owner_record_kind TEXT NOT NULL CHECK (
    owner_record_kind IN ('task', 'change_unit', 'run', 'user_judgment', 'evidence_summary', 'evidence_observation', 'blocker')
  ),
  owner_record_id TEXT NOT NULL,
  created_by_run_id TEXT,
  created_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (project_id, artifact_id, owner_record_kind, owner_record_id),
  FOREIGN KEY (project_id, artifact_id) REFERENCES artifacts (project_id, artifact_id),
  FOREIGN KEY (project_id, task_id) REFERENCES tasks (project_id, task_id),
  FOREIGN KEY (project_id, created_by_run_id) REFERENCES runs (project_id, run_id)
);

CREATE TABLE evidence_summaries (
  project_id TEXT NOT NULL,
  evidence_summary_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  change_unit_id TEXT,
  status TEXT NOT NULL,
  coverage_json TEXT NOT NULL DEFAULT '[]',
  supporting_refs_json TEXT NOT NULL DEFAULT '[]',
  gap_refs_json TEXT NOT NULL DEFAULT '[]',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (project_id, evidence_summary_id),
  FOREIGN KEY (project_id, task_id) REFERENCES tasks (project_id, task_id),
  FOREIGN KEY (project_id, task_id, change_unit_id)
    REFERENCES change_units (project_id, task_id, change_unit_id)
);

CREATE TABLE evidence_observations (
  project_id TEXT NOT NULL,
  evidence_observation_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  change_unit_id TEXT,
  run_id TEXT,
  claim TEXT NOT NULL,
  source_kind TEXT NOT NULL CHECK (
    source_kind IN ('agent_report', 'connection_observation', 'external_tool', 'user_observation', 'reused_evidence', 'unverified_claim')
  ),
  assurance_level TEXT NOT NULL CHECK (
    assurance_level IN ('cooperative_report', 'registered_connection_observed', 'external_tool_result', 'user_observed', 'unverified')
  ),
  observed_by_actor_source TEXT,
  tool_name TEXT,
  tool_invocation_id TEXT,
  tool_metadata_json TEXT NOT NULL DEFAULT '{}',
  input_refs_json TEXT NOT NULL DEFAULT '[]',
  output_artifact_refs_json TEXT NOT NULL DEFAULT '[]',
  limitations_json TEXT NOT NULL DEFAULT '[]',
  observed_at TEXT NOT NULL,
  recorded_at TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (project_id, evidence_observation_id),
  FOREIGN KEY (project_id, task_id) REFERENCES tasks (project_id, task_id),
  FOREIGN KEY (project_id, task_id, change_unit_id)
    REFERENCES change_units (project_id, task_id, change_unit_id),
  FOREIGN KEY (project_id, run_id)
    REFERENCES runs (project_id, run_id)
    DEFERRABLE INITIALLY DEFERRED
);

CREATE TABLE blockers (
  project_id TEXT NOT NULL,
  blocker_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  change_unit_id TEXT,
  status TEXT NOT NULL CHECK (status IN ('active', 'resolved', 'superseded')),
  category TEXT NOT NULL,
  code TEXT NOT NULL,
  owner_refs_json TEXT NOT NULL DEFAULT '[]',
  related_refs_json TEXT NOT NULL DEFAULT '[]',
  detail_json TEXT NOT NULL DEFAULT '{}',
  created_at TEXT NOT NULL,
  resolved_at TEXT,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (project_id, blocker_id),
  FOREIGN KEY (project_id, task_id) REFERENCES tasks (project_id, task_id),
  FOREIGN KEY (project_id, task_id, change_unit_id)
    REFERENCES change_units (project_id, task_id, change_unit_id)
);

CREATE TABLE task_events (
  project_id TEXT NOT NULL,
  event_seq INTEGER NOT NULL CHECK (event_seq > 0),
  event_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  change_unit_id TEXT,
  state_version INTEGER NOT NULL CHECK (state_version > 0),
  event_kind TEXT NOT NULL,
  event_payload_json TEXT NOT NULL DEFAULT '{}',
  created_at TEXT NOT NULL,
  PRIMARY KEY (project_id, event_seq),
  UNIQUE (project_id, event_id),
  FOREIGN KEY (project_id, task_id) REFERENCES tasks (project_id, task_id),
  FOREIGN KEY (project_id, task_id, change_unit_id)
    REFERENCES change_units (project_id, task_id, change_unit_id)
);

CREATE TABLE tool_invocations (
  project_id TEXT NOT NULL,
  tool_name TEXT NOT NULL,
  idempotency_key TEXT NOT NULL,
  request_hash TEXT NOT NULL,
  basis_state_version INTEGER NOT NULL CHECK (basis_state_version >= 0),
  committed_state_version INTEGER NOT NULL CHECK (committed_state_version > basis_state_version),
  status TEXT NOT NULL DEFAULT 'committed' CHECK (status = 'committed'),
  actor_source TEXT NOT NULL,
  operation_category TEXT NOT NULL CHECK (operation_category IN ('read', 'agent_workflow', 'user_only', 'admin_local')),
  verification_basis TEXT,
  response_json TEXT NOT NULL,
  created_at TEXT NOT NULL,
  PRIMARY KEY (project_id, tool_name, idempotency_key),
  FOREIGN KEY (project_id) REFERENCES project_state (project_id)
);

CREATE INDEX idx_project_state_active_task
  ON project_state (project_id, active_task_id);

CREATE INDEX idx_tasks_lifecycle
  ON tasks (project_id, lifecycle_phase, result);

CREATE INDEX idx_tasks_current_change_unit
  ON tasks (project_id, current_change_unit_id);

CREATE INDEX idx_change_units_task_status
  ON change_units (project_id, task_id, status);

CREATE INDEX idx_user_judgments_task_status
  ON user_judgments (project_id, task_id, status);

CREATE INDEX idx_project_continuity_records_status
  ON project_continuity_records (project_id, status, kind, updated_at);

CREATE INDEX idx_project_continuity_records_source_task
  ON project_continuity_records (project_id, source_task_id);

CREATE INDEX idx_write_checks_task_status
  ON write_checks (project_id, task_id, status);

CREATE INDEX idx_runs_task_created
  ON runs (project_id, task_id, created_at);

CREATE INDEX idx_artifact_staging_task_status
  ON artifact_staging (project_id, task_id, status);

CREATE INDEX idx_artifact_staging_actor_source
  ON artifact_staging (project_id, created_by_actor_source);

CREATE INDEX idx_artifacts_task_status
  ON artifacts (project_id, task_id, status);

CREATE INDEX idx_artifact_links_owner
  ON artifact_links (project_id, owner_record_kind, owner_record_id);

CREATE INDEX idx_evidence_summaries_task_status
  ON evidence_summaries (project_id, task_id, status);

CREATE INDEX idx_evidence_observations_task_claim
  ON evidence_observations (project_id, task_id, claim);

CREATE INDEX idx_evidence_observations_run
  ON evidence_observations (project_id, run_id);

CREATE INDEX idx_blockers_task_status
  ON blockers (project_id, task_id, status);

CREATE INDEX idx_task_events_task_seq
  ON task_events (project_id, task_id, event_seq);
```

프로젝트 상태 제약:

- `project_state.state_version`은 기준 범위의 유일한 공개 상태 시계이며 [저장소 버전 관리](storage-versioning.md)에 따라 단조롭게 진행해야 합니다.
- `tasks.created_by_actor_source`, `user_judgments.requested_by_actor_source`, `user_judgments.resolved_by_actor_source`, `write_checks.created_by_actor_source`, `runs.created_by_actor_source`, `artifact_staging.created_by_actor_source`, `evidence_observations.observed_by_actor_source`, `tool_invocations.actor_source`는 행위자 출처를 저장합니다.
- `tool_invocations.operation_category`는 `read`, `agent_workflow`, `user_only`, `admin_local`로 제한됩니다.
- 사용자 판단 행은 권한을 지니는 해결에 대한 User Channel 출처를 저장합니다. `status='resolved'`는 답변이 존재한다는 사실을 기록할 뿐이며, 승인 의미는 저장된 기계 동작, 결과, 근거, 출처, 메서드 담당 문서에서 나옵니다.
- `write_checks`는 단일 사용 Core 상태 쓰기 호환성을 기록합니다. `write_checks.consumed_by_run_id`와 `runs.write_check_id`의 고유 인덱스는 Write Check 소비 하나가 여러 실행으로 갈라지는 것을 막습니다.
- `artifact_staging.created_by_actor_source`는 스테이징 출처를 기록합니다. 스테이징된 바이트와 알림은 아티팩트 담당 상태이며 그 자체로 증거 권한이 아닙니다.
- `evidence_observations.source_kind`와 `assurance_level`은 협력적 에이전트 보고, 등록된 연결 관찰, 외부 도구 결과, 사용자 관찰, 재사용 증거, 미확인 주장을 구분합니다.
- `tool_invocations`는 행위자 출처와 작업 범주를 포함해 재실행 행을 저장합니다. 재실행 행은 호출자 권한이 아니며 현재 연결 맥락이나 User Channel 요구사항을 우회하지 않습니다.

## 관련 담당 문서

- [저장소 기록](storage-records.md): 영속 기록 계열, 배치, 관계 배치, 저장소 소유 값, JSON 배치를 정의합니다.
- [저장 효과](storage-effects.md): 어떤 메서드 분기가 기록을 만들거나, 바꾸거나, 관찰하거나, 건드리지 않는지 정의합니다.
- [저장소 버전 관리](storage-versioning.md): 상태 버전, 멱등성, 재실행, 이벤트, 잠금, 마이그레이션 계약을 정의합니다.
- [Agent Connection](agent-connection.md): Agent Connection, Connection Projects, 현재 연결 맥락, 모드 게이트, Agent Connection과 User Channel의 경계를 정의합니다.
- [보안](security.md): 보안 경계와 보장 수준을 정의합니다.
