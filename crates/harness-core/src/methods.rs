use harness_store::{
    core_pipeline::{
        ChangeUnitInsert, ChangeUnitRecord, CoreProjectStore, CoreStorageMutation,
        ProjectStateHeader, StoredRecordRef, TaskInsert, TaskRecord, TaskScopeUpdate,
        WriteAuthorizationRecord,
    },
    StoreError,
};
use harness_types::{
    AccessClass, BaselineRef, ChangeUnitId, ChangeUnitOperation, CloseReason, DryRunSummary,
    EffectKind, ErrorCode, JsonObject, MethodName, NextActionKind, NextActionSummary,
    PlannedEffect, ProjectId, RecordId, RequestedMode, ResumePolicy, StateRecordKind,
    StateRecordRef, StatusCloseState, StatusInclude, StatusRequest, SurfaceInstanceId, TaskId,
    TaskLifecyclePhase, TaskLifecycleState, TaskMode, TaskResult, ToolEnvelope, ToolResultBase,
    UpdateScopeRequest, WriteAuthoritySummary, WriteAuthorizationStatus,
};
use serde_json::{json, Map, Value};

use crate::pipeline::{
    method_result_base, rejected_response, tool_error, CorePipelineError, CoreResult, CoreService,
    InvocationContext, OwnerPipelineBranch, PipelineRequest, PipelineResponse, TaskRequirement,
};

impl CoreService {
    /// Executes `harness.status` as a read-only Core result.
    pub fn status(
        &self,
        request: StatusRequest,
        invocation: InvocationContext,
    ) -> CoreResult<PipelineResponse> {
        let request_json = serde_json::to_value(&request)?;
        let (store, project_state) = match open_store_with_state(self, &request.envelope) {
            Ok(opened) => opened,
            Err(response) => return Ok(*response),
        };

        let task = status_task(&store, &project_state, request.envelope.task_id.as_ref())?;
        let result_fields = status_result_fields(
            &store,
            &request.envelope.project_id,
            project_state.state_version,
            task.as_ref(),
            &request.include,
        )?;

        self.execute_pipeline(PipelineRequest {
            method_name: MethodName::Status,
            envelope: request.envelope,
            request_json,
            invocation,
            required_access_class: AccessClass::ReadStatus,
            task_requirement: TaskRequirement::Optional,
            branch: OwnerPipelineBranch::ReadOnly { result_fields },
        })
    }

    /// Executes `harness.intake` through the shared Core mutation pipeline.
    pub fn intake(
        &self,
        request: harness_types::IntakeRequest,
        invocation: InvocationContext,
    ) -> CoreResult<PipelineResponse> {
        let request_json = serde_json::to_value(&request)?;
        if !request.envelope.dry_run {
            if let Some(response) = validate_committed_envelope(&request.envelope)? {
                return Ok(response);
            }
        }

        let (store, project_state) = match open_store_with_state(self, &request.envelope) {
            Ok(opened) => opened,
            Err(response) => return Ok(*response),
        };
        let selected_surface_instance =
            match selected_surface_instance(&store, &project_state, &request.envelope, &invocation)
            {
                Ok(surface_instance_id) => surface_instance_id,
                Err(response) => return Ok(*response),
            };
        if request.resume_policy == ResumePolicy::RejectIfActive
            && project_state.active_task_id.is_some()
        {
            return validation_rejected(
                request.envelope.dry_run,
                Some(project_state.state_version),
                "resume_policy",
                "resume_policy=reject_if_active cannot proceed while a Task is active",
            );
        }

        let plan = plan_intake(
            &store,
            &project_state,
            request.clone(),
            &selected_surface_instance,
        )?;

        if request.envelope.dry_run {
            return self.execute_pipeline(PipelineRequest {
                method_name: MethodName::Intake,
                envelope: request.envelope,
                request_json,
                invocation,
                required_access_class: AccessClass::CoreMutation,
                task_requirement: TaskRequirement::None,
                branch: OwnerPipelineBranch::DryRunPreview {
                    dry_run_summary: dry_run_summary(
                        "task",
                        "commit",
                        "Intake would select or create a Task.",
                        plan.next_actions,
                    ),
                },
            });
        }

        self.execute_pipeline(PipelineRequest {
            method_name: MethodName::Intake,
            envelope: request.envelope,
            request_json,
            invocation,
            required_access_class: AccessClass::CoreMutation,
            task_requirement: TaskRequirement::None,
            branch: OwnerPipelineBranch::CommitMutation {
                result_fields: plan.result_fields,
                event_kind: "task_intake".to_owned(),
                event_payload: plan.event_payload,
                task_id: Some(plan.task_id),
                change_unit_id: None,
                storage_mutations: plan.storage_mutations,
            },
        })
    }

    /// Executes `harness.update_scope` through the shared Core mutation pipeline.
    pub fn update_scope(
        &self,
        request: UpdateScopeRequest,
        invocation: InvocationContext,
    ) -> CoreResult<PipelineResponse> {
        let request_json = serde_json::to_value(&request)?;
        if let Some(envelope_task_id) = &request.envelope.task_id {
            if envelope_task_id != &request.task_id {
                return validation_rejected(
                    request.envelope.dry_run,
                    None,
                    "task_id",
                    "envelope.task_id must match UpdateScopeRequest.task_id",
                );
            }
        }
        if !request.envelope.dry_run {
            if let Some(response) = validate_committed_envelope(&request.envelope)? {
                return Ok(response);
            }
        }

        let (store, project_state) = match open_store_with_state(self, &request.envelope) {
            Ok(opened) => opened,
            Err(response) => return Ok(*response),
        };
        let plan = match plan_update_scope(&store, &project_state, request.clone()) {
            Ok(plan) => plan,
            Err(PlanError::Response(response)) => return Ok(*response),
            Err(PlanError::Core(error)) => return Err(error),
        };

        if request.envelope.dry_run {
            return self.execute_pipeline(PipelineRequest {
                method_name: MethodName::UpdateScope,
                envelope: request.envelope,
                request_json,
                invocation,
                required_access_class: AccessClass::CoreMutation,
                task_requirement: TaskRequirement::None,
                branch: OwnerPipelineBranch::DryRunPreview {
                    dry_run_summary: dry_run_summary(
                        "scope",
                        "commit",
                        "Scope update would update current Task scope and Change Unit state.",
                        plan.next_actions,
                    ),
                },
            });
        }

        self.execute_pipeline(PipelineRequest {
            method_name: MethodName::UpdateScope,
            envelope: request.envelope,
            request_json,
            invocation,
            required_access_class: AccessClass::CoreMutation,
            task_requirement: TaskRequirement::None,
            branch: OwnerPipelineBranch::CommitMutation {
                result_fields: plan.result_fields,
                event_kind: "scope_updated".to_owned(),
                event_payload: plan.event_payload,
                task_id: Some(plan.task_id),
                change_unit_id: plan.change_unit_id,
                storage_mutations: plan.storage_mutations,
            },
        })
    }
}

struct MethodPlan {
    task_id: TaskId,
    change_unit_id: Option<ChangeUnitId>,
    storage_mutations: Vec<CoreStorageMutation>,
    event_payload: JsonObject,
    result_fields: JsonObject,
    next_actions: Vec<NextActionSummary>,
}

enum PlanError {
    Core(CorePipelineError),
    Response(Box<PipelineResponse>),
}

impl From<CorePipelineError> for PlanError {
    fn from(error: CorePipelineError) -> Self {
        Self::Core(error)
    }
}

impl From<serde_json::Error> for PlanError {
    fn from(error: serde_json::Error) -> Self {
        Self::Core(CorePipelineError::from(error))
    }
}

fn open_store_with_state(
    service: &CoreService,
    envelope: &ToolEnvelope,
) -> Result<(CoreProjectStore, ProjectStateHeader), Box<PipelineResponse>> {
    let store = match CoreProjectStore::open(service.runtime_home(), &envelope.project_id) {
        Ok(store) => store,
        Err(error) => {
            return Err(Box::new(infallible_rejected_pipeline_response(
                envelope.dry_run,
                None,
                vec![store_unavailable_error(error)],
            )))
        }
    };
    let project_state = match store.project_state() {
        Ok(project_state) => project_state,
        Err(error) => {
            return Err(Box::new(infallible_rejected_pipeline_response(
                envelope.dry_run,
                None,
                vec![store_unavailable_error(error)],
            )))
        }
    };
    Ok((store, project_state))
}

fn selected_surface_instance(
    store: &CoreProjectStore,
    project_state: &ProjectStateHeader,
    envelope: &ToolEnvelope,
    invocation: &InvocationContext,
) -> Result<SurfaceInstanceId, Box<PipelineResponse>> {
    if let Some(surface_instance_id) = &invocation.surface_instance_id {
        return Ok(surface_instance_id.clone());
    }
    if project_state.default_surface_id.as_deref() == Some(envelope.surface_id.as_str()) {
        if let Some(surface_instance_id) = &project_state.default_surface_instance_id {
            return Ok(SurfaceInstanceId::new(surface_instance_id.clone()));
        }
    }
    let candidates = match store.surface_candidates(&envelope.surface_id) {
        Ok(candidates) => candidates,
        Err(error) => {
            return Err(Box::new(infallible_rejected_pipeline_response(
                envelope.dry_run,
                Some(project_state.state_version),
                vec![store_unavailable_error(error)],
            )))
        }
    };
    match candidates.as_slice() {
        [surface] => Ok(SurfaceInstanceId::new(surface.surface_instance_id.clone())),
        _ => Err(Box::new(infallible_rejected_pipeline_response(
            envelope.dry_run,
            Some(project_state.state_version),
            vec![tool_error(
                ErrorCode::LocalAccessMismatch,
                "local surface context does not match the registered surface",
                false,
                None,
            )],
        ))),
    }
}

fn validate_committed_envelope(envelope: &ToolEnvelope) -> CoreResult<Option<PipelineResponse>> {
    if envelope.idempotency_key.is_none() {
        return validation_rejected(
            false,
            None,
            "idempotency_key",
            "committed mutations require idempotency_key",
        )
        .map(Some);
    }
    if envelope.expected_state_version.is_none() {
        return validation_rejected(
            false,
            None,
            "expected_state_version",
            "committed mutations require expected_state_version",
        )
        .map(Some);
    }
    Ok(None)
}

fn status_task(
    store: &CoreProjectStore,
    _project_state: &ProjectStateHeader,
    envelope_task_id: Option<&TaskId>,
) -> CoreResult<Option<TaskRecord>> {
    match envelope_task_id {
        Some(task_id) => store.task_record(task_id).map_err(CorePipelineError::from),
        None => store.active_task_record().map_err(CorePipelineError::from),
    }
}

fn plan_intake(
    store: &CoreProjectStore,
    project_state: &ProjectStateHeader,
    request: harness_types::IntakeRequest,
    surface_instance_id: &SurfaceInstanceId,
) -> CoreResult<MethodPlan> {
    let planned_state_version = project_state.state_version + 1;
    let mode = resolve_requested_mode(request.requested_mode);
    let active_task = store.active_task_record()?;

    let create_new = match request.resume_policy {
        ResumePolicy::ResumeActive => active_task.is_none(),
        ResumePolicy::CreateNew | ResumePolicy::RejectIfActive => true,
        ResumePolicy::SupersedeActive => true,
    };
    let task_id = if create_new {
        request.envelope.task_id.clone().unwrap_or_else(|| {
            TaskId::new(format!("task_{}", request.envelope.request_id.as_str()))
        })
    } else {
        TaskId::new(
            active_task
                .as_ref()
                .expect("active_task exists when create_new is false")
                .task_id
                .clone(),
        )
    };

    let mut storage_mutations = Vec::new();
    if request.resume_policy == ResumePolicy::SupersedeActive {
        if let Some(active) = &active_task {
            storage_mutations.push(CoreStorageMutation::SupersedeTask {
                task_id: active.task_id.clone(),
            });
        }
    }

    let task_record = if create_new {
        let shaping_summary = task_shaping_json(
            Some(request.plain_language_request.clone()),
            Some(request.initial_scope.boundary.clone()),
            request.initial_scope.non_goals.clone(),
            request.initial_scope.acceptance_criteria.clone(),
            None,
            None,
            Some(serde_json::to_value(&request.initial_context_refs)?),
        );
        let task = TaskRecord {
            project_id: request.envelope.project_id.as_str().to_owned(),
            task_id: task_id.as_str().to_owned(),
            mode: task_mode_storage(mode).to_owned(),
            lifecycle_phase: "shaping".to_owned(),
            result: Some("none".to_owned()),
            title: Some(request.plain_language_request.clone()),
            summary: Some(request.plain_language_request.clone()),
            shaping_summary_json: serde_json::to_string(&shaping_summary)?,
            bounded_context_json: serde_json::to_string(&json!({
                "initial_context_refs": request.initial_context_refs
            }))?,
            autonomy_boundary_json: serde_json::to_string(&json!({
                "autonomy_boundary": Value::Null
            }))?,
            close_summary_json: serde_json::to_string(&json!({
                "close_reason": "none"
            }))?,
            completion_policy_json: "{}".to_owned(),
            current_change_unit_id: None,
            closed_at: None,
        };
        storage_mutations.push(CoreStorageMutation::InsertTask(TaskInsert {
            task_id: task.task_id.clone(),
            created_by_surface_id: request.envelope.surface_id.as_str().to_owned(),
            created_by_surface_instance_id: surface_instance_id.as_str().to_owned(),
            mode: task.mode.clone(),
            lifecycle_phase: task.lifecycle_phase.clone(),
            result: task.result.clone(),
            title: task.title.clone(),
            summary: task.summary.clone(),
            shaping_summary_json: task.shaping_summary_json.clone(),
            bounded_context_json: task.bounded_context_json.clone(),
            autonomy_boundary_json: task.autonomy_boundary_json.clone(),
            close_summary_json: task.close_summary_json.clone(),
            completion_policy_json: task.completion_policy_json.clone(),
            current_change_unit_id: None,
        }));
        storage_mutations.push(CoreStorageMutation::SetActiveTask {
            task_id: task.task_id.clone(),
        });
        task
    } else {
        active_task.expect("active_task exists when create_new is false")
    };

    let task_ref = state_ref(
        StateRecordKind::Task,
        &task_record.task_id,
        &request.envelope.project_id,
        Some(&task_id),
        Some(planned_state_version),
    );
    let pending_refs = Vec::new();
    let blocker_refs = Vec::new();
    let next_actions = next_actions_for_state(&task_ref, None);
    let state = build_state_summary(SummaryBuild {
        project_id: &request.envelope.project_id,
        state_version: planned_state_version,
        task: &task_record,
        current_change_unit: None,
        pending_user_judgment_refs: pending_refs,
        blocker_refs,
        active_write_authorization: None,
        options: SummaryOptions::mutation(),
    })?;
    let result = harness_types::IntakeResult {
        base: placeholder_base(),
        task_ref: task_ref.clone(),
        change_unit_ref: None,
        state,
        next_actions: next_actions.clone(),
    };
    let event_payload = object_from_value(json!({
        "task_id": task_id,
        "resume_policy": request.resume_policy,
        "requested_mode": request.requested_mode,
        "resolved_mode": mode
    }))?;
    Ok(MethodPlan {
        task_id,
        change_unit_id: None,
        storage_mutations,
        event_payload,
        result_fields: strip_base(serde_json::to_value(result)?)?,
        next_actions,
    })
}

fn plan_update_scope(
    store: &CoreProjectStore,
    project_state: &ProjectStateHeader,
    request: UpdateScopeRequest,
) -> Result<MethodPlan, PlanError> {
    let planned_state_version = project_state.state_version + 1;
    let task = store
        .task_record(&request.task_id)
        .map_err(|error| {
            PlanError::Response(Box::new(store_error_response(
                &request.envelope,
                project_state,
                error,
            )))
        })?
        .ok_or_else(|| {
            PlanError::Response(Box::new(no_active_task_response(
                &request.envelope,
                project_state,
            )))
        })?;
    let current_change_unit = store
        .current_change_unit(&request.task_id)
        .map_err(|error| {
            PlanError::Response(Box::new(store_error_response(
                &request.envelope,
                project_state,
                error,
            )))
        })?;

    let current_scope = StoredScope::from_task(&task);
    let next_scope = current_scope.apply_request(&request);
    let scope_changed = current_scope != next_scope
        || request.change_unit.operation == ChangeUnitOperation::CreateCurrent
        || request.change_unit.operation == ChangeUnitOperation::ReplaceCurrent;

    let active_write_authorizations = store
        .active_write_authorizations(&request.task_id)
        .map_err(|error| {
            PlanError::Response(Box::new(store_error_response(
                &request.envelope,
                project_state,
                error,
            )))
        })?;
    let stale_write_authorization_refs = if scope_changed {
        active_write_authorizations
            .iter()
            .map(|record| write_authorization_ref(record, planned_state_version))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let mut storage_mutations = vec![CoreStorageMutation::UpdateTaskScope(TaskScopeUpdate {
        task_id: task.task_id.clone(),
        lifecycle_phase: None,
        result: None,
        title: next_scope.goal_summary.clone(),
        summary: next_scope.goal_summary.clone(),
        shaping_summary_json: Some(serde_json::to_string(&next_scope.to_json())?),
        bounded_context_json: Some(serde_json::to_string(&json!({
            "scope_update": request.scope_update.clone()
        }))?),
        autonomy_boundary_json: Some(serde_json::to_string(&json!({
            "autonomy_boundary": next_scope.autonomy_boundary
        }))?),
        close_summary_json: None,
        completion_policy_json: None,
    })];

    let mut synthetic_task = task.clone();
    synthetic_task.title = next_scope.goal_summary.clone();
    synthetic_task.summary = next_scope.goal_summary.clone();
    synthetic_task.shaping_summary_json = serde_json::to_string(&next_scope.to_json())?;
    synthetic_task.bounded_context_json = serde_json::to_string(&json!({
        "scope_update": request.scope_update.clone()
    }))?;
    synthetic_task.autonomy_boundary_json = serde_json::to_string(&json!({
        "autonomy_boundary": next_scope.autonomy_boundary
    }))?;

    let (change_unit_ref, synthetic_change_unit, branch_change_unit_id) =
        match request.change_unit.operation {
            ChangeUnitOperation::KeepCurrent => {
                let change_unit_ref = current_change_unit.as_ref().map(|record| {
                    state_ref(
                        StateRecordKind::ChangeUnit,
                        &record.change_unit_id,
                        &request.envelope.project_id,
                        Some(&request.task_id),
                        Some(record.basis_state_version.unwrap_or(planned_state_version)),
                    )
                });
                (
                    change_unit_ref,
                    current_change_unit.clone(),
                    current_change_unit
                        .as_ref()
                        .map(|record| ChangeUnitId::new(record.change_unit_id.clone())),
                )
            }
            ChangeUnitOperation::CreateCurrent => {
                if current_change_unit.is_some() {
                    let response = validation_rejected(
                        request.envelope.dry_run,
                        Some(project_state.state_version),
                        "change_unit.operation",
                        "create_current requires no current Change Unit",
                    )
                    .map_err(PlanError::Core)?;
                    return Err(PlanError::Response(Box::new(response)));
                }
                let change_unit_id = generated_change_unit_id(&request.envelope);
                let insert = change_unit_insert(&request, &change_unit_id)?;
                let record = synthetic_change_unit_record(
                    &request.envelope.project_id,
                    &request.task_id,
                    &insert,
                    planned_state_version,
                );
                storage_mutations.push(CoreStorageMutation::InsertCurrentChangeUnit(insert));
                synthetic_task.current_change_unit_id = Some(change_unit_id.as_str().to_owned());
                synthetic_task.lifecycle_phase = "ready".to_owned();
                let change_unit_ref = state_ref(
                    StateRecordKind::ChangeUnit,
                    change_unit_id.as_str(),
                    &request.envelope.project_id,
                    Some(&request.task_id),
                    Some(planned_state_version),
                );
                (Some(change_unit_ref), Some(record), Some(change_unit_id))
            }
            ChangeUnitOperation::ReplaceCurrent => {
                if current_change_unit.is_none() {
                    let response = rejected_pipeline_response(
                        request.envelope.dry_run,
                        Some(project_state.state_version),
                        vec![tool_error(
                            ErrorCode::NoActiveChangeUnit,
                            "replace_current requires a current Change Unit",
                            false,
                            None,
                        )],
                    )
                    .map_err(PlanError::Core)?;
                    return Err(PlanError::Response(Box::new(response)));
                }
                let change_unit_id = generated_change_unit_id(&request.envelope);
                let insert = change_unit_insert(&request, &change_unit_id)?;
                let record = synthetic_change_unit_record(
                    &request.envelope.project_id,
                    &request.task_id,
                    &insert,
                    planned_state_version,
                );
                storage_mutations.push(CoreStorageMutation::ReplaceCurrentChangeUnit(insert));
                synthetic_task.current_change_unit_id = Some(change_unit_id.as_str().to_owned());
                synthetic_task.lifecycle_phase = "ready".to_owned();
                let change_unit_ref = state_ref(
                    StateRecordKind::ChangeUnit,
                    change_unit_id.as_str(),
                    &request.envelope.project_id,
                    Some(&request.task_id),
                    Some(planned_state_version),
                );
                (Some(change_unit_ref), Some(record), Some(change_unit_id))
            }
        };

    if scope_changed && !active_write_authorizations.is_empty() {
        storage_mutations.push(CoreStorageMutation::MarkActiveWriteAuthorizationsStale {
            task_id: request.task_id.as_str().to_owned(),
        });
    }

    let pending_refs = store
        .pending_user_judgment_refs(&request.task_id, planned_state_version)
        .map_err(|error| {
            PlanError::Response(Box::new(store_error_response(
                &request.envelope,
                project_state,
                error,
            )))
        })?
        .into_iter()
        .map(state_ref_from_stored)
        .collect::<Vec<_>>();
    let blocker_refs = store
        .active_blocker_refs(&request.task_id, planned_state_version)
        .map_err(|error| {
            PlanError::Response(Box::new(store_error_response(
                &request.envelope,
                project_state,
                error,
            )))
        })?
        .into_iter()
        .map(state_ref_from_stored)
        .collect::<Vec<_>>();
    let task_ref = state_ref(
        StateRecordKind::Task,
        request.task_id.as_str(),
        &request.envelope.project_id,
        Some(&request.task_id),
        Some(planned_state_version),
    );
    let next_actions = next_actions_for_state(&task_ref, change_unit_ref.as_ref());
    let state = build_state_summary(SummaryBuild {
        project_id: &request.envelope.project_id,
        state_version: planned_state_version,
        task: &synthetic_task,
        current_change_unit: synthetic_change_unit.as_ref(),
        pending_user_judgment_refs: pending_refs,
        blocker_refs: blocker_refs.clone(),
        active_write_authorization: None,
        options: SummaryOptions::mutation(),
    })?;
    let result = harness_types::UpdateScopeResult {
        base: placeholder_base(),
        task_ref,
        change_unit_ref,
        linked_scope_decision_refs: request.related_scope_decision_refs.clone(),
        stale_write_authorization_refs,
        blocker_refs,
        state,
        next_actions: next_actions.clone(),
    };
    let event_payload = object_from_value(json!({
        "task_id": request.task_id.clone(),
        "change_unit_operation": request.change_unit.operation,
        "scope_changed": scope_changed
    }))?;

    Ok(MethodPlan {
        task_id: request.task_id,
        change_unit_id: branch_change_unit_id,
        storage_mutations,
        event_payload,
        result_fields: strip_base(serde_json::to_value(result)?)?,
        next_actions,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StoredScope {
    goal_summary: Option<String>,
    scope_summary: Option<String>,
    non_goals: Vec<String>,
    acceptance_criteria: Vec<String>,
    autonomy_boundary: Option<String>,
    baseline_ref: Option<String>,
}

impl StoredScope {
    fn from_task(task: &TaskRecord) -> Self {
        let shaping = parse_json_object(&task.shaping_summary_json);
        let autonomy = parse_json_object(&task.autonomy_boundary_json);
        Self {
            goal_summary: string_member(&shaping, "goal_summary").or_else(|| task.summary.clone()),
            scope_summary: string_member(&shaping, "scope_summary"),
            non_goals: string_array_member(&shaping, "non_goals"),
            acceptance_criteria: string_array_member(&shaping, "acceptance_criteria"),
            autonomy_boundary: string_member(&autonomy, "autonomy_boundary"),
            baseline_ref: string_member(&shaping, "baseline_ref"),
        }
    }

    fn apply_request(&self, request: &UpdateScopeRequest) -> Self {
        Self {
            goal_summary: request
                .goal_summary
                .clone()
                .or_else(|| self.goal_summary.clone()),
            scope_summary: request
                .scope_boundary
                .clone()
                .or_else(|| self.scope_summary.clone()),
            non_goals: request
                .non_goals
                .clone()
                .unwrap_or_else(|| self.non_goals.clone()),
            acceptance_criteria: request
                .acceptance_criteria
                .clone()
                .unwrap_or_else(|| self.acceptance_criteria.clone()),
            autonomy_boundary: request
                .autonomy_boundary
                .clone()
                .or_else(|| self.autonomy_boundary.clone()),
            baseline_ref: request
                .baseline_ref
                .as_ref()
                .map(|value| value.as_str().to_owned())
                .or_else(|| self.baseline_ref.clone()),
        }
    }

    fn to_json(&self) -> Value {
        task_shaping_json(
            self.goal_summary.clone(),
            self.scope_summary.clone(),
            self.non_goals.clone(),
            self.acceptance_criteria.clone(),
            self.baseline_ref.clone(),
            self.autonomy_boundary.clone(),
            None,
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct SummaryOptions {
    pending_user_judgments: bool,
    blockers: bool,
    write_authority: bool,
}

impl SummaryOptions {
    fn mutation() -> Self {
        Self {
            pending_user_judgments: true,
            blockers: true,
            write_authority: false,
        }
    }

    fn status(include: &StatusInclude) -> Self {
        Self {
            pending_user_judgments: include.pending_user_judgments,
            blockers: true,
            write_authority: include.write_authority,
        }
    }
}

struct SummaryBuild<'a> {
    project_id: &'a ProjectId,
    state_version: u64,
    task: &'a TaskRecord,
    current_change_unit: Option<&'a ChangeUnitRecord>,
    pending_user_judgment_refs: Vec<StateRecordRef>,
    blocker_refs: Vec<StateRecordRef>,
    active_write_authorization: Option<&'a WriteAuthorizationRecord>,
    options: SummaryOptions,
}

fn status_result_fields(
    store: &CoreProjectStore,
    project_id: &ProjectId,
    state_version: u64,
    task: Option<&TaskRecord>,
    include: &StatusInclude,
) -> CoreResult<JsonObject> {
    let active_task = if include.task {
        match task {
            Some(task) => {
                let task_id = TaskId::new(task.task_id.clone());
                let current_change_unit = store.current_change_unit(&task_id)?;
                let pending_refs = if include.pending_user_judgments {
                    stored_refs_to_state_refs(
                        store.pending_user_judgment_refs(&task_id, state_version)?,
                    )
                } else {
                    Vec::new()
                };
                let blocker_refs =
                    stored_refs_to_state_refs(store.active_blocker_refs(&task_id, state_version)?);
                let active_write_auths = if include.write_authority {
                    store.active_write_authorizations(&task_id)?
                } else {
                    Vec::new()
                };
                Some(build_state_summary(SummaryBuild {
                    project_id,
                    state_version,
                    task,
                    current_change_unit: current_change_unit.as_ref(),
                    pending_user_judgment_refs: pending_refs,
                    blocker_refs,
                    active_write_authorization: active_write_auths.first(),
                    options: SummaryOptions::status(include),
                })?)
            }
            None => None,
        }
    } else {
        None
    };

    let pending_user_judgments = active_task
        .as_ref()
        .map(|state| state.pending_user_judgment_refs.clone())
        .unwrap_or_default();
    let blocker_refs = active_task
        .as_ref()
        .map(|state| state.blocker_refs.clone())
        .unwrap_or_default();
    let next_actions = active_task
        .as_ref()
        .map(|state| {
            if let Some(task_ref) = &state.task_ref {
                next_actions_for_state(task_ref, state.active_change_unit_ref.as_ref())
            } else {
                Vec::new()
            }
        })
        .unwrap_or_default();
    let result = harness_types::StatusResult {
        base: placeholder_base(),
        active_task,
        status_summary: if task.is_some() {
            "Current Task state is available.".to_owned()
        } else {
            "No current Task is selected.".to_owned()
        },
        next_actions,
        pending_user_judgments,
        blocker_refs,
        close_state: StatusCloseState::None,
        close_blockers: Vec::new(),
        guarantee_display: None,
    };
    strip_base(serde_json::to_value(result)?)
}

fn build_state_summary(input: SummaryBuild<'_>) -> CoreResult<harness_types::StateSummary> {
    let SummaryBuild {
        project_id,
        state_version,
        task,
        current_change_unit,
        pending_user_judgment_refs,
        blocker_refs,
        active_write_authorization,
        options,
    } = input;
    let task_id = TaskId::new(task.task_id.clone());
    let task_ref = state_ref(
        StateRecordKind::Task,
        &task.task_id,
        project_id,
        Some(&task_id),
        Some(state_version),
    );
    let active_change_unit_ref = current_change_unit.map(|record| {
        state_ref(
            StateRecordKind::ChangeUnit,
            &record.change_unit_id,
            project_id,
            Some(&task_id),
            Some(record.basis_state_version.unwrap_or(state_version)),
        )
    });
    let scope = StoredScope::from_task(task);
    let change_unit_scope = current_change_unit.and_then(|record| {
        string_member(
            &parse_json_object(&record.scope_summary_json),
            "scope_summary",
        )
    });
    let write_authority_summary = if options.write_authority {
        active_write_authorization.map(|record| WriteAuthoritySummary {
            status: WriteAuthorizationStatus::Active,
            write_authorization_ref: Some(write_authorization_ref(record, state_version)),
            basis_state_version: Some(record.basis_state_version),
            intended_paths: string_array_member(
                &parse_json_object(&record.attempt_scope_json),
                "intended_paths",
            ),
            guarantee_display: None,
        })
    } else {
        None
    };

    Ok(harness_types::StateSummary {
        project_id: project_id.clone(),
        state_version,
        task_ref: Some(task_ref),
        mode: parse_task_mode(&task.mode)?,
        lifecycle: Some(TaskLifecycleState {
            lifecycle_phase: parse_lifecycle_phase(&task.lifecycle_phase)?,
            close_reason: parse_close_reason(&task.close_summary_json),
            result: parse_task_result(task.result.as_deref().unwrap_or("none"))?,
            closed_at: task.closed_at.clone(),
        }),
        goal_summary: scope.goal_summary,
        scope_summary: change_unit_scope.or(scope.scope_summary),
        non_goals: scope.non_goals,
        acceptance_criteria: scope.acceptance_criteria,
        autonomy_boundary: scope.autonomy_boundary,
        active_change_unit_ref,
        baseline_ref: scope.baseline_ref.map(BaselineRef::new),
        shaping_readiness: None,
        pending_user_judgment_refs: if options.pending_user_judgments {
            pending_user_judgment_refs
        } else {
            Vec::new()
        },
        blocker_refs: if options.blockers {
            blocker_refs
        } else {
            Vec::new()
        },
        write_authority_summary,
        evidence_summary: None,
        close_state: None,
        close_blockers: Vec::new(),
        guarantee_display: None,
    })
}

fn change_unit_insert(
    request: &UpdateScopeRequest,
    change_unit_id: &ChangeUnitId,
) -> CoreResult<ChangeUnitInsert> {
    let fields = &request.change_unit.fields;
    let scope_summary = string_member(fields, "scope_summary")
        .or_else(|| request.scope_boundary.clone())
        .unwrap_or_else(|| "Current Change Unit".to_owned());
    let affected_areas = string_array_member(fields, "affected_areas");
    let affected_paths = string_array_member(fields, "affected_paths");
    let constraints = string_array_member(fields, "constraints");
    Ok(ChangeUnitInsert {
        change_unit_id: change_unit_id.as_str().to_owned(),
        task_id: request.task_id.as_str().to_owned(),
        scope_summary_json: serde_json::to_string(&json!({
            "scope_summary": scope_summary,
            "affected_areas": affected_areas,
            "constraints": constraints
        }))?,
        bounded_paths_json: serde_json::to_string(&affected_paths)?,
        write_basis_json: serde_json::to_string(&json!({
            "baseline_ref": request.baseline_ref
        }))?,
        close_basis_json: "{}".to_owned(),
        lifecycle_json: "{}".to_owned(),
    })
}

fn synthetic_change_unit_record(
    project_id: &ProjectId,
    task_id: &TaskId,
    insert: &ChangeUnitInsert,
    planned_state_version: u64,
) -> ChangeUnitRecord {
    ChangeUnitRecord {
        project_id: project_id.as_str().to_owned(),
        change_unit_id: insert.change_unit_id.clone(),
        task_id: task_id.as_str().to_owned(),
        status: "active".to_owned(),
        is_current: true,
        basis_state_version: Some(planned_state_version),
        scope_summary_json: insert.scope_summary_json.clone(),
        bounded_paths_json: insert.bounded_paths_json.clone(),
        write_basis_json: insert.write_basis_json.clone(),
        close_basis_json: insert.close_basis_json.clone(),
        lifecycle_json: insert.lifecycle_json.clone(),
    }
}

fn task_shaping_json(
    goal_summary: Option<String>,
    scope_summary: Option<String>,
    non_goals: Vec<String>,
    acceptance_criteria: Vec<String>,
    baseline_ref: Option<String>,
    autonomy_boundary: Option<String>,
    initial_context_refs: Option<Value>,
) -> Value {
    json!({
        "goal_summary": goal_summary,
        "scope_summary": scope_summary,
        "non_goals": non_goals,
        "acceptance_criteria": acceptance_criteria,
        "baseline_ref": baseline_ref,
        "autonomy_boundary": autonomy_boundary,
        "initial_context_refs": initial_context_refs.unwrap_or(Value::Array(Vec::new()))
    })
}

fn next_actions_for_state(
    task_ref: &StateRecordRef,
    change_unit_ref: Option<&StateRecordRef>,
) -> Vec<NextActionSummary> {
    match change_unit_ref {
        Some(change_unit_ref) => vec![NextActionSummary {
            action_kind: NextActionKind::PrepareWrite,
            owner_method: Some(MethodName::PrepareWrite),
            label: "Check the current change against current scope.".to_owned(),
            blocking_question: None,
            required_refs: vec![task_ref.clone(), change_unit_ref.clone()],
        }],
        None => vec![NextActionSummary {
            action_kind: NextActionKind::UpdateScope,
            owner_method: Some(MethodName::UpdateScope),
            label: "Create the first currently applied Change Unit before write checking."
                .to_owned(),
            blocking_question: None,
            required_refs: vec![task_ref.clone()],
        }],
    }
}

fn dry_run_summary(
    target_kind: &str,
    action: &str,
    description: &str,
    next_actions: Vec<NextActionSummary>,
) -> DryRunSummary {
    DryRunSummary {
        planned_effects: vec![PlannedEffect {
            target_kind: target_kind.to_owned(),
            action: action.to_owned(),
            description: description.to_owned(),
        }],
        would_blockers: Vec::new(),
        would_errors: Vec::new(),
        next_actions,
        diagnostics: Vec::new(),
    }
}

fn state_ref(
    record_kind: StateRecordKind,
    record_id: &str,
    project_id: &ProjectId,
    task_id: Option<&TaskId>,
    state_version: Option<u64>,
) -> StateRecordRef {
    StateRecordRef {
        record_kind,
        record_id: RecordId::new(record_id),
        project_id: project_id.clone(),
        task_id: task_id.cloned(),
        state_version,
    }
}

fn write_authorization_ref(
    record: &WriteAuthorizationRecord,
    state_version: u64,
) -> StateRecordRef {
    state_ref(
        StateRecordKind::WriteAuthorization,
        &record.write_authorization_id,
        &ProjectId::new(record.project_id.clone()),
        Some(&TaskId::new(record.task_id.clone())),
        Some(state_version),
    )
}

fn state_ref_from_stored(record: StoredRecordRef) -> StateRecordRef {
    let kind = match record.record_kind.as_str() {
        "user_judgment" => StateRecordKind::UserJudgment,
        "blocker" => StateRecordKind::Blocker,
        "write_authorization" => StateRecordKind::WriteAuthorization,
        "change_unit" => StateRecordKind::ChangeUnit,
        "task" => StateRecordKind::Task,
        _ => StateRecordKind::ProjectState,
    };
    StateRecordRef {
        record_kind: kind,
        record_id: RecordId::new(record.record_id),
        project_id: ProjectId::new(record.project_id),
        task_id: record.task_id.map(TaskId::new),
        state_version: record.state_version,
    }
}

fn stored_refs_to_state_refs(records: Vec<StoredRecordRef>) -> Vec<StateRecordRef> {
    records.into_iter().map(state_ref_from_stored).collect()
}

fn strip_base(value: Value) -> CoreResult<JsonObject> {
    let mut object = object_from_value(value)?;
    object.remove("base");
    Ok(object)
}

fn object_from_value(value: Value) -> CoreResult<JsonObject> {
    match value {
        Value::Object(object) => Ok(object),
        _ => Err(CorePipelineError::InvalidDispatch {
            detail: "expected JSON object".to_owned(),
        }),
    }
}

fn placeholder_base() -> ToolResultBase {
    method_result_base(EffectKind::NoEffect, false, None, Vec::new())
}

fn validation_rejected(
    dry_run: bool,
    state_version: Option<u64>,
    field: &'static str,
    message: &'static str,
) -> CoreResult<PipelineResponse> {
    let mut details = Map::new();
    details.insert("field".to_owned(), Value::String(field.to_owned()));
    rejected_pipeline_response(
        dry_run,
        state_version,
        vec![tool_error(
            ErrorCode::ValidationFailed,
            message,
            false,
            Some(details),
        )],
    )
}

fn rejected_pipeline_response(
    dry_run: bool,
    state_version: Option<u64>,
    errors: Vec<harness_types::ToolError>,
) -> CoreResult<PipelineResponse> {
    let response = rejected_response(dry_run, state_version, errors);
    let response_value = serde_json::to_value(response)?;
    let response_json = serde_json::to_string(&response_value)?;
    Ok(PipelineResponse {
        response_json,
        response_value,
        verified_surface: None,
        resolved_task_id: None,
        replayed: false,
    })
}

fn infallible_rejected_pipeline_response(
    dry_run: bool,
    state_version: Option<u64>,
    errors: Vec<harness_types::ToolError>,
) -> PipelineResponse {
    rejected_pipeline_response(dry_run, state_version, errors)
        .expect("rejected response serialization should succeed")
}

fn store_error_response(
    envelope: &ToolEnvelope,
    project_state: &ProjectStateHeader,
    error: StoreError,
) -> PipelineResponse {
    rejected_pipeline_response(
        envelope.dry_run,
        Some(project_state.state_version),
        vec![store_unavailable_error(error)],
    )
    .expect("rejected response serialization should succeed")
}

fn no_active_task_response(
    envelope: &ToolEnvelope,
    project_state: &ProjectStateHeader,
) -> PipelineResponse {
    rejected_pipeline_response(
        envelope.dry_run,
        Some(project_state.state_version),
        vec![tool_error(
            ErrorCode::NoActiveTask,
            "a Task is required but no addressed or current Task is available",
            false,
            None,
        )],
    )
    .expect("rejected response serialization should succeed")
}

fn store_unavailable_error(error: StoreError) -> harness_types::ToolError {
    tool_error(
        match error {
            StoreError::NotFound { .. } => ErrorCode::LocalAccessMismatch,
            StoreError::InvalidInput { .. }
            | StoreError::Io(_)
            | StoreError::Sqlite(_)
            | StoreError::MigrationConflict { .. }
            | StoreError::SchemaInvariant { .. } => ErrorCode::McpUnavailable,
        },
        "Core storage or project binding is unavailable",
        true,
        None,
    )
}

fn generated_change_unit_id(envelope: &ToolEnvelope) -> ChangeUnitId {
    ChangeUnitId::new(format!("cu_{}", envelope.request_id.as_str()))
}

fn resolve_requested_mode(requested_mode: RequestedMode) -> TaskMode {
    match requested_mode {
        RequestedMode::Advisor => TaskMode::Advisor,
        RequestedMode::Direct => TaskMode::Direct,
        RequestedMode::Work | RequestedMode::Auto => TaskMode::Work,
    }
}

fn task_mode_storage(mode: TaskMode) -> &'static str {
    match mode {
        TaskMode::Advisor => "advisor",
        TaskMode::Direct => "direct",
        TaskMode::Work => "work",
    }
}

fn parse_task_mode(value: &str) -> CoreResult<Option<TaskMode>> {
    match value {
        "advisor" => Ok(Some(TaskMode::Advisor)),
        "direct" => Ok(Some(TaskMode::Direct)),
        "work" => Ok(Some(TaskMode::Work)),
        _ => invalid_storage(format!("unsupported Task mode {value}")),
    }
}

fn parse_lifecycle_phase(value: &str) -> CoreResult<TaskLifecyclePhase> {
    match value {
        "shaping" => Ok(TaskLifecyclePhase::Shaping),
        "ready" => Ok(TaskLifecyclePhase::Ready),
        "executing" => Ok(TaskLifecyclePhase::Executing),
        "waiting_user" => Ok(TaskLifecyclePhase::WaitingUser),
        "blocked" => Ok(TaskLifecyclePhase::Blocked),
        "completed" => Ok(TaskLifecyclePhase::Completed),
        "cancelled" => Ok(TaskLifecyclePhase::Cancelled),
        "superseded" => Ok(TaskLifecyclePhase::Superseded),
        _ => invalid_storage(format!("unsupported Task lifecycle_phase {value}")),
    }
}

fn parse_task_result(value: &str) -> CoreResult<TaskResult> {
    match value {
        "none" => Ok(TaskResult::None),
        "advice_only" => Ok(TaskResult::AdviceOnly),
        "completed" => Ok(TaskResult::Completed),
        "cancelled" => Ok(TaskResult::Cancelled),
        "superseded" => Ok(TaskResult::Superseded),
        _ => invalid_storage(format!("unsupported Task result {value}")),
    }
}

fn parse_close_reason(close_summary_json: &str) -> CloseReason {
    let value = parse_json_object(close_summary_json);
    match string_member(&value, "close_reason").as_deref() {
        Some("completed_self_checked") => CloseReason::CompletedSelfChecked,
        Some("completed_with_risk_accepted") => CloseReason::CompletedWithRiskAccepted,
        Some("cancelled") => CloseReason::Cancelled,
        Some("superseded") => CloseReason::Superseded,
        _ => CloseReason::None,
    }
}

fn invalid_storage<T>(detail: String) -> CoreResult<T> {
    Err(CorePipelineError::InvalidDispatch { detail })
}

fn parse_json_object(text: &str) -> JsonObject {
    serde_json::from_str::<Value>(text)
        .ok()
        .and_then(|value| match value {
            Value::Object(object) => Some(object),
            _ => None,
        })
        .unwrap_or_default()
}

fn string_member(object: &JsonObject, key: &str) -> Option<String> {
    object.get(key).and_then(Value::as_str).map(str::to_owned)
}

fn string_array_member(object: &JsonObject, key: &str) -> Vec<String> {
    object
        .get(key)
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::{error::Error, path::PathBuf};

    use harness_store::{
        bootstrap::{
            initialize_runtime_home, register_project, register_surface, ProjectRegistration,
            SurfaceRegistration, ACTIVE_PROJECT_STATUS,
        },
        core_pipeline::{CoreProjectStore, StorageEffectCounts},
        sqlite::open_project_state_database,
    };
    use harness_test_support::TempRuntimeHome;
    use harness_types::{
        ActorKind, ChangeUnitUpdate, IdempotencyKey, InitialScope, RequestId, ScopeUpdate,
        SurfaceId,
    };
    use serde_json::{json, Map, Value};

    use super::*;

    const PROJECT_ID: &str = "project_methods";
    const SURFACE_ID: &str = "surface_methods";
    const SURFACE_INSTANCE_ID: &str = "surface_instance_methods";

    struct MethodHarness {
        _runtime_home: TempRuntimeHome,
        runtime_home_path: PathBuf,
        service: CoreService,
    }

    impl MethodHarness {
        fn new() -> Result<Self, Box<dyn Error>> {
            let runtime_home = TempRuntimeHome::new("core-methods")?;
            initialize_runtime_home(runtime_home.path(), "runtime_home_methods", "{}")?;
            register_project(
                runtime_home.path(),
                ProjectRegistration {
                    project_id: PROJECT_ID.to_owned(),
                    repo_root: runtime_home.path().join("repo"),
                    project_home: None,
                    status: ACTIVE_PROJECT_STATUS.to_owned(),
                    metadata_json: "{}".to_owned(),
                },
            )?;
            register_surface(
                runtime_home.path(),
                SurfaceRegistration {
                    project_id: PROJECT_ID.to_owned(),
                    surface_id: SURFACE_ID.to_owned(),
                    surface_instance_id: SURFACE_INSTANCE_ID.to_owned(),
                    surface_kind: "local_test".to_owned(),
                    display_name: Some("Method Test Surface".to_owned()),
                    capability_profile_json: "{}".to_owned(),
                    local_access_json: "{}".to_owned(),
                    metadata_json: "{}".to_owned(),
                },
            )?;

            let runtime_home_path = runtime_home.path().to_path_buf();
            let service = CoreService::new(&runtime_home_path);
            Ok(Self {
                _runtime_home: runtime_home,
                runtime_home_path,
                service,
            })
        }

        fn counts(&self) -> Result<StorageEffectCounts, Box<dyn Error>> {
            let store =
                CoreProjectStore::open(&self.runtime_home_path, &ProjectId::new(PROJECT_ID))?;
            Ok(store.effect_counts()?)
        }

        fn conn(&self) -> Result<rusqlite::Connection, Box<dyn Error>> {
            Ok(open_project_state_database(
                self.runtime_home_path
                    .join("projects")
                    .join(PROJECT_ID)
                    .join("state.sqlite"),
            )?)
        }
    }

    #[test]
    fn status_is_read_only_including_dry_run() -> Result<(), Box<dyn Error>> {
        let harness = MethodHarness::new()?;
        let before = harness.counts()?;

        let response = harness.service.status(
            StatusRequest {
                envelope: envelope("req_status", None, false, None, None),
                include: status_include(),
            },
            invocation(AccessClass::ReadStatus),
        )?;

        assert_eq!(response.response_value["base"]["response_kind"], "result");
        assert_eq!(response.response_value["base"]["effect_kind"], "read_only");
        assert_eq!(response.response_value["base"]["dry_run"], false);
        assert_eq!(response.response_value["base"]["events"], json!([]));
        assert_eq!(harness.counts()?, before);

        let dry_run = harness.service.status(
            StatusRequest {
                envelope: envelope(
                    "req_status_dry",
                    Some("idem_status_dry"),
                    true,
                    Some(0),
                    None,
                ),
                include: status_include(),
            },
            invocation(AccessClass::ReadStatus),
        )?;

        assert_eq!(dry_run.response_value["base"]["response_kind"], "result");
        assert_eq!(dry_run.response_value["base"]["effect_kind"], "read_only");
        assert_eq!(dry_run.response_value["base"]["dry_run"], true);
        assert_eq!(harness.counts()?, before);
        Ok(())
    }

    #[test]
    fn intake_commits_once_and_replays_without_effect() -> Result<(), Box<dyn Error>> {
        let harness = MethodHarness::new()?;
        let before = harness.counts()?;
        let request = intake_request(
            "req_intake",
            "idem_intake",
            false,
            Some(0),
            RequestedMode::Auto,
        );

        let first = harness
            .service
            .intake(request.clone(), invocation(AccessClass::CoreMutation))?;
        let after_first = harness.counts()?;

        assert_eq!(first.response_value["base"]["response_kind"], "result");
        assert_eq!(
            first.response_value["base"]["effect_kind"],
            "core_committed"
        );
        assert_eq!(first.response_value["base"]["state_version"], 1);
        assert_eq!(first.response_value["state"]["mode"], "work");
        assert_eq!(after_first.state_version, before.state_version + 1);
        assert_eq!(after_first.tasks, before.tasks + 1);
        assert_eq!(after_first.task_events, before.task_events + 1);
        assert_eq!(after_first.tool_invocations, before.tool_invocations + 1);

        let second = harness
            .service
            .intake(request, invocation(AccessClass::CoreMutation))?;
        assert!(second.replayed);
        assert_eq!(second.response_json, first.response_json);
        assert_eq!(harness.counts()?, after_first);
        Ok(())
    }

    #[test]
    fn intake_dry_run_has_no_storage_effect() -> Result<(), Box<dyn Error>> {
        let harness = MethodHarness::new()?;
        let before = harness.counts()?;
        let response = harness.service.intake(
            intake_request(
                "req_intake_dry",
                "idem_intake_dry",
                true,
                Some(0),
                RequestedMode::Work,
            ),
            invocation(AccessClass::CoreMutation),
        )?;

        assert_eq!(response.response_value["base"]["response_kind"], "dry_run");
        assert_eq!(response.response_value["base"]["effect_kind"], "no_effect");
        assert_eq!(harness.counts()?, before);
        Ok(())
    }

    #[test]
    fn update_scope_commits_once_and_creates_one_current_change_unit() -> Result<(), Box<dyn Error>>
    {
        let harness = MethodHarness::new()?;
        let intake = harness.service.intake(
            intake_request(
                "req_scope_task",
                "idem_scope_task",
                false,
                Some(0),
                RequestedMode::Work,
            ),
            invocation(AccessClass::CoreMutation),
        )?;
        let task_id = intake.response_value["task_ref"]["record_id"]
            .as_str()
            .expect("task ref should be present")
            .to_owned();
        let before = harness.counts()?;

        let response = harness.service.update_scope(
            update_scope_request(
                "req_scope_create",
                "idem_scope_create",
                false,
                Some(1),
                &task_id,
                ChangeUnitOperation::CreateCurrent,
                "Create current export scope.",
            ),
            invocation(AccessClass::CoreMutation),
        )?;
        let after = harness.counts()?;

        assert_eq!(response.response_value["base"]["response_kind"], "result");
        assert_eq!(response.response_value["base"]["state_version"], 2);
        assert!(response.response_value["change_unit_ref"].is_object());
        assert_eq!(after.state_version, before.state_version + 1);
        assert_eq!(after.change_units, before.change_units + 1);
        assert_eq!(after.task_events, before.task_events + 1);
        assert_eq!(after.tool_invocations, before.tool_invocations + 1);
        assert_eq!(active_current_change_units(&harness, &task_id)?, 1);
        Ok(())
    }

    #[test]
    fn update_scope_replaces_current_and_marks_write_authorization_stale(
    ) -> Result<(), Box<dyn Error>> {
        let harness = MethodHarness::new()?;
        let intake = harness.service.intake(
            intake_request(
                "req_replace_task",
                "idem_replace_task",
                false,
                Some(0),
                RequestedMode::Work,
            ),
            invocation(AccessClass::CoreMutation),
        )?;
        let task_id = intake.response_value["task_ref"]["record_id"]
            .as_str()
            .expect("task ref should be present")
            .to_owned();
        let create = harness.service.update_scope(
            update_scope_request(
                "req_replace_create",
                "idem_replace_create",
                false,
                Some(1),
                &task_id,
                ChangeUnitOperation::CreateCurrent,
                "Initial current scope.",
            ),
            invocation(AccessClass::CoreMutation),
        )?;
        let change_unit_id = create.response_value["change_unit_ref"]["record_id"]
            .as_str()
            .expect("change unit ref should be present")
            .to_owned();
        insert_active_write_authorization(&harness, &task_id, &change_unit_id)?;
        let before = harness.counts()?;

        let response = harness.service.update_scope(
            update_scope_request(
                "req_replace_current",
                "idem_replace_current",
                false,
                Some(2),
                &task_id,
                ChangeUnitOperation::ReplaceCurrent,
                "Replacement current scope.",
            ),
            invocation(AccessClass::CoreMutation),
        )?;
        let after = harness.counts()?;

        assert_eq!(response.response_value["base"]["state_version"], 3);
        assert_eq!(
            response.response_value["stale_write_authorization_refs"]
                .as_array()
                .expect("stale refs should be an array")
                .len(),
            1
        );
        assert_eq!(after.state_version, before.state_version + 1);
        assert_eq!(after.change_units, before.change_units + 1);
        assert_eq!(active_current_change_units(&harness, &task_id)?, 1);
        assert_eq!(write_authorization_status(&harness, "wa_replace")?, "stale");
        Ok(())
    }

    #[test]
    fn update_scope_dry_run_has_no_storage_effect() -> Result<(), Box<dyn Error>> {
        let harness = MethodHarness::new()?;
        let intake = harness.service.intake(
            intake_request(
                "req_dry_task",
                "idem_dry_task",
                false,
                Some(0),
                RequestedMode::Work,
            ),
            invocation(AccessClass::CoreMutation),
        )?;
        let task_id = intake.response_value["task_ref"]["record_id"]
            .as_str()
            .expect("task ref should be present")
            .to_owned();
        let before = harness.counts()?;

        let response = harness.service.update_scope(
            update_scope_request(
                "req_scope_dry",
                "idem_scope_dry",
                true,
                Some(1),
                &task_id,
                ChangeUnitOperation::CreateCurrent,
                "Dry-run scope.",
            ),
            invocation(AccessClass::CoreMutation),
        )?;

        assert_eq!(response.response_value["base"]["response_kind"], "dry_run");
        assert_eq!(response.response_value["base"]["effect_kind"], "no_effect");
        assert_eq!(harness.counts()?, before);
        Ok(())
    }

    #[test]
    fn scope_decision_ref_alone_does_not_change_current_scope() -> Result<(), Box<dyn Error>> {
        let harness = MethodHarness::new()?;
        let intake = harness.service.intake(
            intake_request(
                "req_decision_task",
                "idem_decision_task",
                false,
                Some(0),
                RequestedMode::Work,
            ),
            invocation(AccessClass::CoreMutation),
        )?;
        let task_id = intake.response_value["task_ref"]["record_id"]
            .as_str()
            .expect("task ref should be present")
            .to_owned();
        let decision_ref = StateRecordRef {
            record_kind: StateRecordKind::UserJudgment,
            record_id: RecordId::new("uj_scope_decision"),
            project_id: ProjectId::new(PROJECT_ID),
            task_id: Some(TaskId::new(&task_id)),
            state_version: Some(1),
        };

        let response = harness.service.update_scope(
            UpdateScopeRequest {
                envelope: envelope(
                    "req_decision_only",
                    Some("idem_decision_only"),
                    false,
                    Some(1),
                    Some(&task_id),
                ),
                task_id: TaskId::new(&task_id),
                goal_summary: None,
                scope_update: None,
                scope_boundary: None,
                non_goals: None,
                acceptance_criteria: None,
                autonomy_boundary: None,
                baseline_ref: None,
                change_unit: ChangeUnitUpdate {
                    operation: ChangeUnitOperation::KeepCurrent,
                    fields: Map::new(),
                },
                related_scope_decision_refs: vec![decision_ref],
            },
            invocation(AccessClass::CoreMutation),
        )?;

        assert_eq!(
            response.response_value["state"]["scope_summary"],
            "Initial test scope."
        );
        assert_eq!(
            response.response_value["linked_scope_decision_refs"]
                .as_array()
                .expect("linked refs should be an array")
                .len(),
            1
        );
        Ok(())
    }

    fn envelope(
        request_id: &str,
        idempotency_key: Option<&str>,
        dry_run: bool,
        expected_state_version: Option<u64>,
        task_id: Option<&str>,
    ) -> ToolEnvelope {
        ToolEnvelope {
            project_id: ProjectId::new(PROJECT_ID),
            task_id: task_id.map(TaskId::new),
            actor_kind: ActorKind::Agent,
            surface_id: SurfaceId::new(SURFACE_ID),
            request_id: RequestId::new(request_id),
            idempotency_key: idempotency_key.map(IdempotencyKey::new),
            expected_state_version,
            dry_run,
            locale: None,
        }
    }

    fn invocation(access_class: AccessClass) -> InvocationContext {
        InvocationContext {
            surface_instance_id: Some(SurfaceInstanceId::new(SURFACE_INSTANCE_ID)),
            access_class,
            verification_basis: "method_test_invocation".to_owned(),
        }
    }

    fn status_include() -> StatusInclude {
        StatusInclude {
            task: true,
            pending_user_judgments: true,
            write_authority: true,
            evidence: true,
            close: true,
            guarantees: true,
        }
    }

    fn intake_request(
        request_id: &str,
        idempotency_key: &str,
        dry_run: bool,
        expected_state_version: Option<u64>,
        requested_mode: RequestedMode,
    ) -> harness_types::IntakeRequest {
        harness_types::IntakeRequest {
            envelope: envelope(
                request_id,
                Some(idempotency_key),
                dry_run,
                expected_state_version,
                None,
            ),
            plain_language_request: "Create a test export flow.".to_owned(),
            requested_mode,
            resume_policy: ResumePolicy::CreateNew,
            initial_scope: InitialScope {
                boundary: "Initial test scope.".to_owned(),
                non_goals: vec!["Changing unrelated flows.".to_owned()],
                acceptance_criteria: vec!["The test export flow is represented.".to_owned()],
            },
            initial_context_refs: Vec::new(),
        }
    }

    fn update_scope_request(
        request_id: &str,
        idempotency_key: &str,
        dry_run: bool,
        expected_state_version: Option<u64>,
        task_id: &str,
        operation: ChangeUnitOperation,
        scope_summary: &str,
    ) -> UpdateScopeRequest {
        let mut fields = Map::new();
        fields.insert(
            "scope_summary".to_owned(),
            Value::String(scope_summary.to_owned()),
        );
        fields.insert(
            "affected_paths".to_owned(),
            json!(["src/export.rs", "tests/export.rs"]),
        );
        UpdateScopeRequest {
            envelope: envelope(
                request_id,
                Some(idempotency_key),
                dry_run,
                expected_state_version,
                Some(task_id),
            ),
            task_id: TaskId::new(task_id),
            goal_summary: Some(scope_summary.to_owned()),
            scope_update: Some(ScopeUpdate {
                include: vec![scope_summary.to_owned()],
                exclude: vec!["Unrelated behavior.".to_owned()],
            }),
            scope_boundary: Some(scope_summary.to_owned()),
            non_goals: Some(vec!["Unrelated behavior.".to_owned()]),
            acceptance_criteria: Some(vec!["The scoped behavior is represented.".to_owned()]),
            autonomy_boundary: Some("Stay inside the scoped test behavior.".to_owned()),
            baseline_ref: Some(BaselineRef::new("baseline_test")),
            change_unit: ChangeUnitUpdate { operation, fields },
            related_scope_decision_refs: Vec::new(),
        }
    }

    fn insert_active_write_authorization(
        harness: &MethodHarness,
        task_id: &str,
        change_unit_id: &str,
    ) -> Result<(), Box<dyn Error>> {
        let conn = harness.conn()?;
        conn.execute(
            "INSERT INTO write_authorizations (
                project_id,
                write_authorization_id,
                task_id,
                change_unit_id,
                basis_state_version,
                status,
                attempt_scope_json,
                created_by_surface_id,
                created_by_surface_instance_id,
                expires_at,
                created_at
            )
            VALUES (
                ?1,
                'wa_replace',
                ?2,
                ?3,
                2,
                'active',
                '{\"intended_paths\":[\"src/export.rs\"]}',
                ?4,
                ?5,
                '2999-01-01T00:00:00Z',
                't0'
            )",
            rusqlite::params![
                PROJECT_ID,
                task_id,
                change_unit_id,
                SURFACE_ID,
                SURFACE_INSTANCE_ID
            ],
        )?;
        Ok(())
    }

    fn active_current_change_units(
        harness: &MethodHarness,
        task_id: &str,
    ) -> Result<i64, Box<dyn Error>> {
        let conn = harness.conn()?;
        Ok(conn.query_row(
            "SELECT COUNT(*)
               FROM change_units
              WHERE project_id = ?1
                AND task_id = ?2
                AND status = 'active'
                AND is_current = 1",
            rusqlite::params![PROJECT_ID, task_id],
            |row| row.get(0),
        )?)
    }

    fn write_authorization_status(
        harness: &MethodHarness,
        write_authorization_id: &str,
    ) -> Result<String, Box<dyn Error>> {
        let conn = harness.conn()?;
        Ok(conn.query_row(
            "SELECT status
               FROM write_authorizations
              WHERE project_id = ?1
                AND write_authorization_id = ?2",
            rusqlite::params![PROJECT_ID, write_authorization_id],
            |row| row.get(0),
        )?)
    }
}
