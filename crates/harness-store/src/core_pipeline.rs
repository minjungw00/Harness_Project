use std::path::Path;

use harness_types::{IdempotencyKey, MethodName, ProjectId, RequestHash, SurfaceId, TaskId};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde_json::Value;

use crate::{
    bootstrap::{project_record, ProjectRecord, SurfaceRecord},
    sqlite::{begin_immediate_transaction, open_project_state_database},
    StoreError, StoreResult,
};

/// Project-local store handle used by the Core request pipeline.
#[derive(Debug)]
pub struct CoreProjectStore {
    project: ProjectRecord,
    conn: Connection,
}

/// Current project-state header values needed by request routing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectStateHeader {
    pub project_id: String,
    pub state_version: u64,
    pub active_task_id: Option<String>,
    pub default_surface_id: Option<String>,
    pub default_surface_instance_id: Option<String>,
}

/// Stored idempotency replay row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolInvocationRecord {
    pub project_id: String,
    pub tool_name: String,
    pub idempotency_key: String,
    pub request_hash: String,
    pub basis_state_version: u64,
    pub committed_state_version: u64,
    pub response_json: String,
}

/// Pending event supplied by a method-specific commit branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingTaskEvent {
    pub event_id: String,
    pub task_id: String,
    pub change_unit_id: Option<String>,
    pub event_kind: String,
    pub event_payload_json: String,
}

/// Event reference facts created by an atomic mutation commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommittedEventRef {
    pub event_id: String,
    pub event_kind: String,
}

/// Facts available to build the exact committed response before replay storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommittedMutationFacts {
    pub basis_state_version: u64,
    pub committed_state_version: u64,
    pub events: Vec<CommittedEventRef>,
}

/// Input for an atomic Core mutation commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitMutationInput {
    pub project_id: String,
    pub tool_name: String,
    pub idempotency_key: Option<String>,
    pub request_hash: String,
    pub expected_state_version: Option<u64>,
    pub events: Vec<PendingTaskEvent>,
}

/// Result of attempting a mutating commit through the replay/freshness gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationCommitOutcome {
    Replayed {
        response_json: String,
        basis_state_version: u64,
        committed_state_version: u64,
    },
    IdempotencyConflict {
        current_state_version: u64,
        idempotency_key: String,
        stored_request_hash: String,
        attempted_request_hash: String,
    },
    StaleExpectedState {
        current_state_version: u64,
        expected_state_version: u64,
    },
    Committed {
        response_json: String,
        basis_state_version: u64,
        committed_state_version: u64,
        events: Vec<CommittedEventRef>,
    },
}

/// Storage counters used to verify no-effect request branches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageEffectCounts {
    pub state_version: u64,
    pub tasks: u64,
    pub task_events: u64,
    pub tool_invocations: u64,
    pub user_judgments: u64,
    pub write_authorizations: u64,
    pub runs: u64,
    pub artifact_staging: u64,
    pub artifacts: u64,
    pub blockers: u64,
}

impl CoreProjectStore {
    /// Opens the registered project-local state store for Core pipeline work.
    pub fn open(runtime_home: impl AsRef<Path>, project_id: &ProjectId) -> StoreResult<Self> {
        let project = project_record(runtime_home, project_id.as_str())?.ok_or_else(|| {
            StoreError::NotFound {
                entity: "project",
                id: project_id.as_str().to_owned(),
            }
        })?;

        if !project.state_db_path.exists() {
            return Err(StoreError::NotFound {
                entity: "project_state_database",
                id: project.state_db_path.display().to_string(),
            });
        }

        let conn = open_project_state_database(&project.state_db_path)?;
        Ok(Self { project, conn })
    }

    /// Returns the registry project row that selected this project-local store.
    pub const fn project_record(&self) -> &ProjectRecord {
        &self.project
    }

    /// Reads the current project-state header.
    pub fn project_state(&self) -> StoreResult<ProjectStateHeader> {
        read_project_state(&self.conn, &self.project.project_id)
    }

    /// Reads one surface instance by exact project/surface/instance identity.
    pub fn surface(
        &self,
        surface_id: &SurfaceId,
        surface_instance_id: &str,
    ) -> StoreResult<Option<SurfaceRecord>> {
        surface_by_instance(
            &self.conn,
            &self.project.project_id,
            surface_id.as_str(),
            surface_instance_id,
        )
    }

    /// Lists candidate surface instances for a request-level `surface_id`.
    pub fn surface_candidates(&self, surface_id: &SurfaceId) -> StoreResult<Vec<SurfaceRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT
                project_id,
                surface_id,
                surface_instance_id,
                surface_kind,
                display_name,
                capability_profile_json,
                local_access_json,
                metadata_json
             FROM surfaces
             WHERE project_id = ?1
               AND surface_id = ?2
             ORDER BY surface_instance_id",
        )?;
        let rows = stmt.query_map(
            params![self.project.project_id, surface_id.as_str()],
            surface_record_from_row,
        )?;

        let mut surfaces = Vec::new();
        for row in rows {
            surfaces.push(row?);
        }
        Ok(surfaces)
    }

    /// Returns whether a Task exists in this project.
    pub fn task_exists(&self, task_id: &TaskId) -> StoreResult<bool> {
        self.conn
            .query_row(
                "SELECT COUNT(*)
                   FROM tasks
                  WHERE project_id = ?1
                    AND task_id = ?2",
                params![self.project.project_id, task_id.as_str()],
                |row| Ok(row.get::<_, i64>(0)? > 0),
            )
            .map_err(StoreError::from)
    }

    /// Reads a committed replay row without creating storage effects.
    pub fn tool_invocation(
        &self,
        method_name: MethodName,
        idempotency_key: &IdempotencyKey,
    ) -> StoreResult<Option<ToolInvocationRecord>> {
        tool_invocation(
            &self.conn,
            &self.project.project_id,
            method_name.as_str(),
            idempotency_key.as_str(),
        )
    }

    /// Reads the current storage-effect counters for this project.
    pub fn effect_counts(&self) -> StoreResult<StorageEffectCounts> {
        let state = self.project_state()?;
        Ok(StorageEffectCounts {
            state_version: state.state_version,
            tasks: table_count(&self.conn, "tasks", &self.project.project_id)?,
            task_events: table_count(&self.conn, "task_events", &self.project.project_id)?,
            tool_invocations: table_count(
                &self.conn,
                "tool_invocations",
                &self.project.project_id,
            )?,
            user_judgments: table_count(&self.conn, "user_judgments", &self.project.project_id)?,
            write_authorizations: table_count(
                &self.conn,
                "write_authorizations",
                &self.project.project_id,
            )?,
            runs: table_count(&self.conn, "runs", &self.project.project_id)?,
            artifact_staging: table_count(
                &self.conn,
                "artifact_staging",
                &self.project.project_id,
            )?,
            artifacts: table_count(&self.conn, "artifacts", &self.project.project_id)?,
            blockers: table_count(&self.conn, "blockers", &self.project.project_id)?,
        })
    }

    /// Commits one state-changing Core mutation or returns replay/conflict outcomes.
    ///
    /// The helper performs replay lookup, stale-state checking, project clock
    /// increment, event append, response construction, and replay-row insertion
    /// in one immediate transaction. Any error rolls back the whole attempt.
    pub fn commit_mutation(
        &mut self,
        input: CommitMutationInput,
        build_response_json: impl FnOnce(CommittedMutationFacts) -> StoreResult<String>,
    ) -> StoreResult<MutationCommitOutcome> {
        if input.project_id != self.project.project_id {
            return Err(StoreError::InvalidInput {
                detail: "commit project_id must match the opened project".to_owned(),
            });
        }
        if input.events.is_empty() {
            return Err(StoreError::InvalidInput {
                detail: "committed Core mutations must append at least one task_event".to_owned(),
            });
        }
        validate_identifier("tool_name", &input.tool_name)?;
        validate_identifier("request_hash", &input.request_hash)?;
        for event in &input.events {
            validate_pending_event(event)?;
        }

        let tx = begin_immediate_transaction(&mut self.conn)?;
        let current = read_project_state_tx(&tx, &self.project.project_id)?;

        if let Some(idempotency_key) = &input.idempotency_key {
            validate_identifier("idempotency_key", idempotency_key)?;
            if let Some(record) = tool_invocation_tx(
                &tx,
                &self.project.project_id,
                &input.tool_name,
                idempotency_key,
            )? {
                tx.rollback()?;
                if record.request_hash == input.request_hash {
                    return Ok(MutationCommitOutcome::Replayed {
                        response_json: record.response_json,
                        basis_state_version: record.basis_state_version,
                        committed_state_version: record.committed_state_version,
                    });
                }

                return Ok(MutationCommitOutcome::IdempotencyConflict {
                    current_state_version: current.state_version,
                    idempotency_key: idempotency_key.clone(),
                    stored_request_hash: record.request_hash,
                    attempted_request_hash: input.request_hash,
                });
            }
        }

        if let Some(expected_state_version) = input.expected_state_version {
            if expected_state_version != current.state_version {
                tx.rollback()?;
                return Ok(MutationCommitOutcome::StaleExpectedState {
                    current_state_version: current.state_version,
                    expected_state_version,
                });
            }
        }

        let committed_state_version =
            current
                .state_version
                .checked_add(1)
                .ok_or_else(|| StoreError::SchemaInvariant {
                    database_kind: "project_state",
                    detail: "project_state.state_version overflow".to_owned(),
                })?;
        let current_state_i64 = u64_to_i64("basis_state_version", current.state_version)?;
        let committed_state_i64 = u64_to_i64("committed_state_version", committed_state_version)?;

        let changed = tx.execute(
            "UPDATE project_state
                SET state_version = ?3,
                    updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
              WHERE project_id = ?1
                AND state_version = ?2",
            params![
                self.project.project_id,
                current_state_i64,
                committed_state_i64
            ],
        )?;
        if changed != 1 {
            return Err(StoreError::SchemaInvariant {
                database_kind: "project_state",
                detail: "project_state state_version update changed no rows".to_owned(),
            });
        }

        let first_event_seq = next_event_seq(&tx, &self.project.project_id)?;
        let mut committed_events = Vec::with_capacity(input.events.len());
        for (index, event) in input.events.iter().enumerate() {
            let event_seq = first_event_seq
                + i64::try_from(index).map_err(|_| StoreError::InvalidInput {
                    detail: "event index does not fit in SQLite integer".to_owned(),
                })?;
            tx.execute(
                "INSERT INTO task_events (
                    project_id,
                    event_seq,
                    event_id,
                    task_id,
                    change_unit_id,
                    state_version,
                    event_kind,
                    event_payload_json,
                    created_at
                )
                VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4,
                    ?5,
                    ?6,
                    ?7,
                    ?8,
                    strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                )",
                params![
                    self.project.project_id,
                    event_seq,
                    event.event_id,
                    event.task_id,
                    event.change_unit_id,
                    committed_state_i64,
                    event.event_kind,
                    event.event_payload_json
                ],
            )?;
            committed_events.push(CommittedEventRef {
                event_id: event.event_id.clone(),
                event_kind: event.event_kind.clone(),
            });
        }

        let facts = CommittedMutationFacts {
            basis_state_version: current.state_version,
            committed_state_version,
            events: committed_events.clone(),
        };
        let response_json = build_response_json(facts)?;
        validate_json_text("tool_invocations.response_json", &response_json)?;

        if let Some(idempotency_key) = &input.idempotency_key {
            tx.execute(
                "INSERT INTO tool_invocations (
                    project_id,
                    tool_name,
                    idempotency_key,
                    request_hash,
                    basis_state_version,
                    committed_state_version,
                    response_json,
                    created_at
                )
                VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4,
                    ?5,
                    ?6,
                    ?7,
                    strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                )",
                params![
                    self.project.project_id,
                    input.tool_name,
                    idempotency_key,
                    input.request_hash,
                    current_state_i64,
                    committed_state_i64,
                    response_json
                ],
            )?;
        }

        tx.commit()?;
        Ok(MutationCommitOutcome::Committed {
            response_json,
            basis_state_version: current.state_version,
            committed_state_version,
            events: committed_events,
        })
    }
}

/// Builds a commit input from typed public identifiers.
pub fn commit_input(
    project_id: &ProjectId,
    method_name: MethodName,
    idempotency_key: Option<&IdempotencyKey>,
    request_hash: &RequestHash,
    expected_state_version: Option<u64>,
    events: Vec<PendingTaskEvent>,
) -> CommitMutationInput {
    CommitMutationInput {
        project_id: project_id.as_str().to_owned(),
        tool_name: method_name.as_str().to_owned(),
        idempotency_key: idempotency_key.map(|key| key.as_str().to_owned()),
        request_hash: request_hash.as_str().to_owned(),
        expected_state_version,
        events,
    }
}

fn read_project_state(conn: &Connection, project_id: &str) -> StoreResult<ProjectStateHeader> {
    conn.query_row(
        "SELECT
            project_id,
            state_version,
            active_task_id,
            default_surface_id,
            default_surface_instance_id
         FROM project_state
         WHERE project_id = ?1",
        params![project_id],
        project_state_from_row,
    )
    .optional()?
    .ok_or_else(|| StoreError::NotFound {
        entity: "project_state",
        id: project_id.to_owned(),
    })
}

fn read_project_state_tx(
    tx: &Transaction<'_>,
    project_id: &str,
) -> StoreResult<ProjectStateHeader> {
    tx.query_row(
        "SELECT
            project_id,
            state_version,
            active_task_id,
            default_surface_id,
            default_surface_instance_id
         FROM project_state
         WHERE project_id = ?1",
        params![project_id],
        project_state_from_row,
    )
    .optional()?
    .ok_or_else(|| StoreError::NotFound {
        entity: "project_state",
        id: project_id.to_owned(),
    })
}

fn project_state_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProjectStateHeader> {
    let state_version = row.get::<_, i64>(1)?;
    Ok(ProjectStateHeader {
        project_id: row.get(0)?,
        state_version: nonnegative_i64_to_u64("project_state.state_version", state_version)?,
        active_task_id: row.get(2)?,
        default_surface_id: row.get(3)?,
        default_surface_instance_id: row.get(4)?,
    })
}

fn surface_by_instance(
    conn: &Connection,
    project_id: &str,
    surface_id: &str,
    surface_instance_id: &str,
) -> StoreResult<Option<SurfaceRecord>> {
    conn.query_row(
        "SELECT
            project_id,
            surface_id,
            surface_instance_id,
            surface_kind,
            display_name,
            capability_profile_json,
            local_access_json,
            metadata_json
         FROM surfaces
         WHERE project_id = ?1
           AND surface_id = ?2
           AND surface_instance_id = ?3",
        params![project_id, surface_id, surface_instance_id],
        surface_record_from_row,
    )
    .optional()
    .map_err(StoreError::from)
}

fn surface_record_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SurfaceRecord> {
    Ok(SurfaceRecord {
        project_id: row.get(0)?,
        surface_id: row.get(1)?,
        surface_instance_id: row.get(2)?,
        surface_kind: row.get(3)?,
        display_name: row.get(4)?,
        capability_profile_json: row.get(5)?,
        local_access_json: row.get(6)?,
        metadata_json: row.get(7)?,
    })
}

fn tool_invocation_tx(
    tx: &Transaction<'_>,
    project_id: &str,
    tool_name: &str,
    idempotency_key: &str,
) -> StoreResult<Option<ToolInvocationRecord>> {
    tx.query_row(
        "SELECT
            project_id,
            tool_name,
            idempotency_key,
            request_hash,
            basis_state_version,
            committed_state_version,
            response_json
         FROM tool_invocations
         WHERE project_id = ?1
           AND tool_name = ?2
           AND idempotency_key = ?3",
        params![project_id, tool_name, idempotency_key],
        |row| {
            let basis_state_version = row.get::<_, i64>(4)?;
            let committed_state_version = row.get::<_, i64>(5)?;
            Ok(ToolInvocationRecord {
                project_id: row.get(0)?,
                tool_name: row.get(1)?,
                idempotency_key: row.get(2)?,
                request_hash: row.get(3)?,
                basis_state_version: nonnegative_i64_to_u64(
                    "tool_invocations.basis_state_version",
                    basis_state_version,
                )?,
                committed_state_version: nonnegative_i64_to_u64(
                    "tool_invocations.committed_state_version",
                    committed_state_version,
                )?,
                response_json: row.get(6)?,
            })
        },
    )
    .optional()
    .map_err(StoreError::from)
}

fn tool_invocation(
    conn: &Connection,
    project_id: &str,
    tool_name: &str,
    idempotency_key: &str,
) -> StoreResult<Option<ToolInvocationRecord>> {
    conn.query_row(
        "SELECT
            project_id,
            tool_name,
            idempotency_key,
            request_hash,
            basis_state_version,
            committed_state_version,
            response_json
         FROM tool_invocations
         WHERE project_id = ?1
           AND tool_name = ?2
           AND idempotency_key = ?3",
        params![project_id, tool_name, idempotency_key],
        |row| {
            let basis_state_version = row.get::<_, i64>(4)?;
            let committed_state_version = row.get::<_, i64>(5)?;
            Ok(ToolInvocationRecord {
                project_id: row.get(0)?,
                tool_name: row.get(1)?,
                idempotency_key: row.get(2)?,
                request_hash: row.get(3)?,
                basis_state_version: nonnegative_i64_to_u64(
                    "tool_invocations.basis_state_version",
                    basis_state_version,
                )?,
                committed_state_version: nonnegative_i64_to_u64(
                    "tool_invocations.committed_state_version",
                    committed_state_version,
                )?,
                response_json: row.get(6)?,
            })
        },
    )
    .optional()
    .map_err(StoreError::from)
}

fn next_event_seq(tx: &Transaction<'_>, project_id: &str) -> StoreResult<i64> {
    let last_seq: i64 = tx.query_row(
        "SELECT COALESCE(MAX(event_seq), 0)
           FROM task_events
          WHERE project_id = ?1",
        params![project_id],
        |row| row.get(0),
    )?;
    last_seq
        .checked_add(1)
        .ok_or_else(|| StoreError::SchemaInvariant {
            database_kind: "project_state",
            detail: "task_events.event_seq overflow".to_owned(),
        })
}

fn table_count(conn: &Connection, table: &str, project_id: &str) -> StoreResult<u64> {
    let escaped_table = table.replace('"', "\"\"");
    let sql = format!("SELECT COUNT(*) FROM \"{escaped_table}\" WHERE project_id = ?1");
    let count: i64 = conn.query_row(&sql, params![project_id], |row| row.get(0))?;
    nonnegative_i64_to_u64("table count", count).map_err(StoreError::from)
}

fn validate_pending_event(event: &PendingTaskEvent) -> StoreResult<()> {
    validate_identifier("event_id", &event.event_id)?;
    validate_identifier("task_id", &event.task_id)?;
    validate_identifier("event_kind", &event.event_kind)?;
    validate_json_text("task_events.event_payload_json", &event.event_payload_json)
}

fn validate_identifier(field: &'static str, value: &str) -> StoreResult<()> {
    if value.trim().is_empty() {
        Err(StoreError::InvalidInput {
            detail: format!("{field} must not be empty"),
        })
    } else {
        Ok(())
    }
}

fn validate_json_text(field: &'static str, text: &str) -> StoreResult<()> {
    serde_json::from_str::<Value>(text).map_err(|error| StoreError::InvalidInput {
        detail: format!("{field} must be JSON text: {error}"),
    })?;
    Ok(())
}

fn nonnegative_i64_to_u64(field: &'static str, value: i64) -> Result<u64, rusqlite::Error> {
    u64::try_from(value).map_err(|_| {
        rusqlite::Error::FromSqlConversionFailure(
            0,
            rusqlite::types::Type::Integer,
            format!("{field} must be nonnegative").into(),
        )
    })
}

fn u64_to_i64(field: &'static str, value: u64) -> StoreResult<i64> {
    i64::try_from(value).map_err(|_| StoreError::InvalidInput {
        detail: format!("{field} does not fit in SQLite integer"),
    })
}
