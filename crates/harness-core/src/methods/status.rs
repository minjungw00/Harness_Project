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
    let guarantee_display = include.guarantees.then(status_guarantee_display);

    if let Some(task) = task {
        let task_id = TaskId::new(task.task_id.clone());
        let current_change_unit = store
            .current_change_unit(&task_id)
            .map_err(CorePipelineError::from)?;
        if include.pending_user_judgments {
            pending_user_judgments = stored_refs_to_state_refs(
                store
                    .pending_user_judgment_refs(&task_id, state_version)
                    .map_err(CorePipelineError::from)?,
            );
        }
        blocker_refs = stored_refs_to_state_refs(
            store
                .active_blocker_refs(&task_id, state_version)
                .map_err(CorePipelineError::from)?,
        );
        let selected_write_authorization = if include.write_authority {
            selected_status_write_authorization(store, &task_id, state_version, now)?
        } else {
            None
        };
        if let Some(record) = selected_write_authorization.as_ref() {
            write_authority_summary = Some(write_authority_summary_for_record(
                record,
                state_version,
                Some(now),
            )?);
        }
        if include.evidence {
            evidence_summary = status_evidence_summary(store, project_id, state_version, task)?;
        }
        let close_plan = if include.close {
            Some(close_task::plan_close_task(
                store,
                project_state,
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
            )?)
        } else {
            None
        };
        if let Some(plan) = close_plan.as_ref() {
            close_state = status_close_state(plan.close_state);
            current_close_basis = plan.current_close_basis.clone();
            risk_acceptance_coverage = plan.risk_acceptance_coverage.clone();
            close_blockers = plan.blockers.clone();
            next_actions.extend(close_next_actions(&plan.blockers));
        }
        if include.task {
            let mut state = build_state_summary(SummaryBuild {
                project_id,
                state_version,
                task,
                current_change_unit: current_change_unit.as_ref(),
                pending_user_judgment_refs: pending_user_judgments.clone(),
                blocker_refs: blocker_refs.clone(),
                active_write_authorization: selected_write_authorization.as_ref(),
                effective_authorization_now: Some(now),
                options: SummaryOptions::status(include),
            })?;
            if include.evidence {
                state.evidence_summary = evidence_summary.clone();
            }
            if include.close {
                state.close_state = close_plan.as_ref().map(|plan| plan.close_state);
                state.close_blockers = close_blockers.clone();
            }
            if include.guarantees {
                state.guarantee_display = guarantee_display.clone();
            }
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

fn selected_status_write_authorization(
    store: &CoreProjectStore,
    task_id: &TaskId,
    state_version: u64,
    now: DateTime<Utc>,
) -> Result<Option<WriteAuthorizationRecord>, PlanError> {
    let records = store
        .write_authorizations_for_task(task_id)
        .map_err(CorePipelineError::from)?;
    let mut selected = None;
    let mut selected_priority = u8::MAX;
    for record in records {
        let status = effective_write_authorization_status(&record, state_version, Some(now))?;
        let priority = match status {
            WriteAuthorizationStatus::Active => 0,
            WriteAuthorizationStatus::Expired => 1,
            WriteAuthorizationStatus::Stale => 2,
            WriteAuthorizationStatus::Consumed => 3,
            WriteAuthorizationStatus::Revoked => 4,
        };
        if priority < selected_priority {
            selected_priority = priority;
            selected = Some(record);
        }
    }
    Ok(selected)
}

fn status_evidence_summary(
    store: &CoreProjectStore,
    project_id: &ProjectId,
    state_version: u64,
    task: &TaskRecord,
) -> Result<Option<EvidenceSummary>, PlanError> {
    let task_id = TaskId::new(task.task_id.clone());
    let record = store
        .latest_evidence_summary(&task_id)
        .map_err(CorePipelineError::from)?;
    Ok(close_task::close_evidence_summary(
        store,
        record.as_ref(),
        task,
        project_id,
        &task_id,
        state_version,
    )?)
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
