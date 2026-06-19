use super::*;

impl CoreService {
    /// Executes `harness.status` as a read-only Core result.
    pub fn status(
        &self,
        request: StatusRequest,
        invocation: InvocationContext,
    ) -> CoreResult<PipelineResponse> {
        let request_json = serde_json::to_value(&request)?;
        let prepared = match prepare_or_response(
            self,
            MethodName::Status,
            request.envelope.clone(),
            request_json,
            invocation,
            MethodPolicy::exact(
                request.requested_access_class(),
                TaskRequirement::Optional,
                ReplayPolicy::None,
                FreshnessPolicy::None,
                MethodEffectPolicy::ReadOnly,
            ),
        )? {
            Ok(prepared) => prepared,
            Err(response) => return Ok(response),
        };
        let state_version = prepared.context.project_state.state_version;

        let task = match status_task(
            &prepared.store,
            &prepared.context.project_state,
            request.envelope.task_id.as_ref(),
        ) {
            Ok(task) => task,
            Err(error) => {
                return core_error_response(&request.envelope, Some(state_version), error)
            }
        };
        let result_fields = match status_result_fields(
            &prepared.store,
            &request.envelope,
            &prepared.context.project_state,
            &prepared.context.verified_surface,
            task.as_ref(),
            &request.include,
            self.now(),
        ) {
            Ok(result_fields) => result_fields,
            Err(error) => {
                return plan_error_response(
                    &request.envelope,
                    &prepared.context.project_state,
                    error,
                )
            }
        };

        match self
            .execute_prepared_request(prepared, OwnerPipelineBranch::ReadOnly { result_fields })
        {
            Ok(response) => Ok(response),
            Err(error) => core_error_response(&request.envelope, Some(state_version), error),
        }
    }
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

fn status_result_fields(
    store: &CoreProjectStore,
    envelope: &ToolEnvelope,
    project_state: &ProjectStateHeader,
    verified_surface: &VerifiedSurfaceContext,
    task: Option<&TaskRecord>,
    include: &StatusInclude,
    now: DateTime<Utc>,
) -> Result<JsonObject, PlanError> {
    let state_version = project_state.state_version;
    let project_id = &envelope.project_id;
    let mut active_task = None;
    let mut pending_user_judgments = Vec::new();
    let mut blocker_refs = Vec::new();
    let mut write_authority_summary = None;
    let mut evidence_summary = None;
    let mut close_state = StatusCloseState::None;
    let mut current_close_basis = None;
    let mut risk_acceptance_coverage = Vec::new();
    let mut close_blockers = Vec::new();
    let mut next_actions = Vec::new();
    let guarantee_projection = guarantee_display_for_surface(verified_surface, state_version);
    let guarantee_display = include.guarantees.then(|| guarantee_projection.clone());

    if let Some(task) = task {
        let task_id = TaskId::new(task.task_id.clone());
        let current_change_unit = store
            .current_change_unit(&task_id)
            .map_err(CorePipelineError::from)?;
        let all_pending_user_judgments =
            projected_pending_user_judgment_refs(store, &task_id, state_version)?;
        if include.pending_user_judgments {
            pending_user_judgments = all_pending_user_judgments.clone();
        }
        blocker_refs = projected_blocker_refs(store, &task_id, state_version)?;
        let projected_write_authority = projected_write_authority_summary(
            store,
            &task_id,
            state_version,
            now,
            Some(guarantee_projection.clone()),
        )?;
        if include.write_authority {
            write_authority_summary = projected_write_authority.clone();
        }
        let projected_evidence =
            projected_evidence_summary(store, project_id, state_version, task)?;
        if include.evidence {
            evidence_summary = projected_evidence.clone();
        }
        let close_plan = close_task::plan_close_task(
            store,
            project_state,
            Some(verified_surface),
            CloseTaskRequest {
                envelope: ToolEnvelope {
                    task_id: Some(task_id.clone()).into(),
                    ..envelope.clone()
                },
                task_id: task_id.clone(),
                intent: CloseIntent::Check,
                close_reason: RequiredNullable::null(),
                superseding_task_id: RequiredNullable::null(),
                user_note: RequiredNullable::null(),
            },
            &utc_timestamp(now),
        )?;
        close_state = status_close_state(close_plan.close_state);
        if include.close {
            current_close_basis = close_plan.current_close_basis.clone();
            risk_acceptance_coverage = close_plan.risk_acceptance_coverage.clone();
        }
        close_blockers = close_plan.blockers.clone();
        next_actions.extend(close_next_actions(&close_plan.blockers));
        if include.task {
            let state = build_state_summary(SummaryBuild {
                project_id,
                state_version,
                task,
                current_change_unit: current_change_unit.as_ref(),
                pending_user_judgment_refs: all_pending_user_judgments,
                blocker_refs: blocker_refs.clone(),
                write_authority_summary: projected_write_authority,
                evidence_summary: projected_evidence,
                close_state: Some(close_plan.close_state),
                close_blockers: close_blockers.clone(),
                guarantee_display: Some(guarantee_projection.clone()),
            })?;
            if let Some(task_ref) = &state.task_ref {
                next_actions.extend(next_actions_for_state(
                    task_ref,
                    state.active_change_unit_ref.as_ref(),
                ));
            }
            active_task = Some(state);
        }
    }
    next_actions = unique_next_actions(next_actions);

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
        write_authority_summary,
        evidence_summary,
        close_state,
        current_close_basis,
        risk_acceptance_coverage,
        close_blockers,
        guarantee_display,
    };
    Ok(strip_base(serde_json::to_value(result)?)?)
}

fn status_close_state(close_state: CloseState) -> StatusCloseState {
    match close_state {
        CloseState::Ready => StatusCloseState::Ready,
        CloseState::Blocked => StatusCloseState::Blocked,
        CloseState::Closed => StatusCloseState::Closed,
        CloseState::Cancelled => StatusCloseState::Cancelled,
        CloseState::Superseded => StatusCloseState::Superseded,
    }
}

fn close_next_actions(blockers: &[CloseReadinessBlocker]) -> Vec<NextActionSummary> {
    blockers
        .iter()
        .flat_map(|blocker| blocker.next_actions.clone())
        .collect()
}

fn unique_next_actions(actions: Vec<NextActionSummary>) -> Vec<NextActionSummary> {
    let mut seen = BTreeSet::new();
    actions
        .into_iter()
        .filter(|action| {
            seen.insert(serde_json::to_string(action).unwrap_or_else(|_| String::new()))
        })
        .collect()
}
