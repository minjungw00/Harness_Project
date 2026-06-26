use std::{
    collections::BTreeMap,
    error::Error,
    fmt,
    path::Path,
    process,
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::{json, Map, Value};
use volicord_core::{AdapterSessionBinding, CorePipelineError, CoreService, InvocationContext};
use volicord_store::{
    bootstrap::{list_surfaces, register_surface, SurfaceRecord, SurfaceRegistration},
    core_pipeline::{CoreProjectStore, UserJudgmentRecord},
    runtime_home::{resolve_runtime_home, RuntimeHomeResolutionError},
    StoreError,
};
use volicord_types::{
    AccessClass, ActorKind, IdempotencyKey, JudgmentKind, JudgmentRationale,
    PersistedJudgmentBasis, PersistedUserJudgmentOptions, PersistedUserJudgmentRequest, ProjectId,
    RecordUserJudgmentPayload, RecordUserJudgmentRequest, RequestId, RequiredNullable, RiskId,
    StatusInclude, StatusRequest, SurfaceId, SurfaceInstanceId, SurfaceInteractionRole, TaskId,
    ToolEnvelope, UserJudgmentOption, UserJudgmentOptionId,
    VERIFICATION_BASIS_CLI_DIRECT_SURFACE_BINDING,
};

use crate::registration::{
    access_class_from_local_access, capability_profile_json, local_access_json,
    normalized_access_classes_from_local_access, user_interaction_access_classes,
    validate_role_access_classes, ADMIN_METADATA_JSON,
};

type CliOptions = BTreeMap<String, Vec<String>>;

const DEFAULT_USER_SURFACE_ID: &str = "surface_user_cli";
const DEFAULT_USER_SURFACE_INSTANCE_ID: &str = "surface_instance_user_cli";
const DEFAULT_USER_SURFACE_KIND: &str = "cli";
const DEFAULT_USER_SURFACE_NAME: &str = "Local user CLI";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserCommandError {
    Usage(String),
    Runtime(String),
}

impl UserCommandError {
    fn usage(message: impl Into<String>) -> Self {
        Self::Usage(message.into())
    }

    fn runtime(message: impl Into<String>) -> Self {
        Self::Runtime(message.into())
    }
}

impl fmt::Display for UserCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) | Self::Runtime(message) => formatter.write_str(message),
        }
    }
}

impl Error for UserCommandError {}

impl From<StoreError> for UserCommandError {
    fn from(error: StoreError) -> Self {
        Self::Runtime(error.to_string())
    }
}

impl From<RuntimeHomeResolutionError> for UserCommandError {
    fn from(error: RuntimeHomeResolutionError) -> Self {
        Self::Runtime(error.to_string())
    }
}

impl From<crate::registration::RegistrationMetadataError> for UserCommandError {
    fn from(error: crate::registration::RegistrationMetadataError) -> Self {
        match error {
            crate::registration::RegistrationMetadataError::Usage(message) => Self::Usage(message),
            crate::registration::RegistrationMetadataError::Runtime(message) => {
                Self::Runtime(message)
            }
        }
    }
}

impl From<CorePipelineError> for UserCommandError {
    fn from(error: CorePipelineError) -> Self {
        Self::Runtime(error.to_string())
    }
}

pub fn run_user_command<F>(
    args: &[String],
    env_var: F,
    current_dir: &Path,
) -> Result<String, UserCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let Some(subcommand) = args.first().map(String::as_str) else {
        return Err(UserCommandError::usage(user_usage()));
    };

    match subcommand {
        "setup" => command_setup(&args[1..], env_var, current_dir),
        "status" => command_status(&args[1..], env_var, current_dir),
        "judgment" => command_judgment(&args[1..], env_var, current_dir),
        "-h" | "--help" | "help" => Ok(user_usage()),
        other => Err(UserCommandError::usage(format!(
            "unknown user command: {other}\n\n{}",
            user_usage()
        ))),
    }
}

pub fn user_usage() -> String {
    "volicord user setup --project-id ID [--surface-id ID] [--surface-instance-id ID] [--name NAME]\nvolicord user status --project-id ID [--task-id ID] [--surface-id ID] [--surface-instance-id ID]\nvolicord user judgment list --project-id ID [--task-id ID] [--surface-id ID] [--surface-instance-id ID]\nvolicord user judgment show --project-id ID --judgment-id ID [--surface-id ID] [--surface-instance-id ID]\nvolicord user judgment record --project-id ID --judgment-id ID --option-id ID [--surface-id ID] [--surface-instance-id ID] [--note TEXT] [--request-id ID] [--idempotency-key KEY] [--expected-state-version VERSION]\n".to_owned()
}

fn command_setup<F>(
    args: &[String],
    env_var: F,
    current_dir: &Path,
) -> Result<String, UserCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let options = parse_options(
        args,
        &["project-id", "surface-id", "surface-instance-id", "name"],
    )?;
    let runtime_home = resolve_runtime_home(env_var, current_dir)?;
    let project_id = required_option(&options, "project-id")?;
    let surface_id = options
        .value("surface-id")
        .unwrap_or_else(|| DEFAULT_USER_SURFACE_ID.to_owned());
    let surface_instance_id = options
        .value("surface-instance-id")
        .unwrap_or_else(|| DEFAULT_USER_SURFACE_INSTANCE_ID.to_owned());
    let display_name = options
        .value("name")
        .or_else(|| Some(DEFAULT_USER_SURFACE_NAME.to_owned()));
    let access_classes = user_interaction_access_classes();
    validate_role_access_classes(SurfaceInteractionRole::UserInteraction, &access_classes)?;
    let record = register_surface(
        &runtime_home,
        SurfaceRegistration {
            project_id,
            surface_id,
            surface_instance_id,
            surface_kind: DEFAULT_USER_SURFACE_KIND.to_owned(),
            interaction_role: SurfaceInteractionRole::UserInteraction,
            display_name,
            capability_profile_json: capability_profile_json(&access_classes, None)?,
            local_access_json: local_access_json(&access_classes)?,
            metadata_json: ADMIN_METADATA_JSON.to_owned(),
        },
    )?;
    let access_class =
        access_class_from_local_access(&record.local_access_json).unwrap_or_default();

    Ok(format!(
        "user surface ready\nproject_id: {}\nsurface_id: {}\nsurface_instance_id: {}\nsurface_kind: {}\ninteraction_role: {}\naccess_class: {}\n",
        record.project_id,
        record.surface_id,
        record.surface_instance_id,
        record.surface_kind,
        record.interaction_role,
        access_class
    ))
}

fn command_status<F>(
    args: &[String],
    env_var: F,
    current_dir: &Path,
) -> Result<String, UserCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let options = parse_surface_options(args, &["project-id", "task-id"])?;
    let runtime_home = resolve_runtime_home(env_var, current_dir)?;
    let project_id = required_option(&options, "project-id")?;
    let surface = selected_user_surface(&runtime_home, &project_id, &options)?;
    let service = CoreService::new(&runtime_home);
    let response = service.status(
        status_request(
            &project_id,
            options.value_ref("task-id").map(String::as_str),
            &surface,
        ),
        invocation(&project_id, &surface, AccessClass::ReadStatus),
    )?;
    if is_rejected(&response.response_value) {
        return Err(UserCommandError::runtime(rejected_summary(
            &response.response_value,
        )));
    }

    Ok(format_status(&project_id, &response.response_value))
}

fn command_judgment<F>(
    args: &[String],
    env_var: F,
    current_dir: &Path,
) -> Result<String, UserCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let Some(subcommand) = args.first().map(String::as_str) else {
        return Err(UserCommandError::usage(judgment_usage()));
    };

    match subcommand {
        "list" => command_judgment_list(&args[1..], env_var, current_dir),
        "show" => command_judgment_show(&args[1..], env_var, current_dir),
        "record" => command_judgment_record(&args[1..], env_var, current_dir),
        "-h" | "--help" | "help" => Ok(judgment_usage()),
        other => Err(UserCommandError::usage(format!(
            "unknown user judgment command: {other}\n\n{}",
            judgment_usage()
        ))),
    }
}

fn command_judgment_list<F>(
    args: &[String],
    env_var: F,
    current_dir: &Path,
) -> Result<String, UserCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let options = parse_surface_options(args, &["project-id", "task-id"])?;
    let runtime_home = resolve_runtime_home(env_var, current_dir)?;
    let project_id = required_option(&options, "project-id")?;
    let surface = selected_user_surface(&runtime_home, &project_id, &options)?;
    let status = CoreService::new(&runtime_home).status(
        status_request(
            &project_id,
            options.value_ref("task-id").map(String::as_str),
            &surface,
        ),
        invocation(&project_id, &surface, AccessClass::ReadStatus),
    )?;
    if is_rejected(&status.response_value) {
        return Err(UserCommandError::runtime(rejected_summary(
            &status.response_value,
        )));
    }

    let Some(task_id) = status_task_id(&status.response_value) else {
        return Ok("judgment_id\ttask_id\tjudgment_kind\tstatus\tquestion\n".to_owned());
    };
    let store = CoreProjectStore::open(&runtime_home, &ProjectId::new(&project_id))?;
    let records = store.pending_user_judgment_records(&TaskId::new(&task_id))?;
    let mut output = String::from("judgment_id\ttask_id\tjudgment_kind\tstatus\tquestion\n");
    for record in records {
        let display = judgment_display(&record)?;
        output.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\n",
            display.judgment_id,
            display.task_id,
            display.judgment_kind,
            display.status,
            tab_safe(&display.question)
        ));
    }
    Ok(output)
}

fn command_judgment_show<F>(
    args: &[String],
    env_var: F,
    current_dir: &Path,
) -> Result<String, UserCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let options = parse_surface_options(args, &["project-id", "judgment-id"])?;
    let runtime_home = resolve_runtime_home(env_var, current_dir)?;
    let project_id = required_option(&options, "project-id")?;
    let judgment_id = required_option(&options, "judgment-id")?;
    let record = user_judgment_record(&runtime_home, &project_id, &judgment_id)?;
    let surface = selected_user_surface(&runtime_home, &project_id, &options)?;
    verify_status_read(&runtime_home, &project_id, &record.task_id, &surface)?;

    Ok(format_judgment_show(&judgment_display(&record)?))
}

fn command_judgment_record<F>(
    args: &[String],
    env_var: F,
    current_dir: &Path,
) -> Result<String, UserCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let options = parse_surface_options(
        args,
        &[
            "project-id",
            "judgment-id",
            "option-id",
            "note",
            "request-id",
            "idempotency-key",
            "expected-state-version",
        ],
    )?;
    let runtime_home = resolve_runtime_home(env_var, current_dir)?;
    let project_id = required_option(&options, "project-id")?;
    let judgment_id = required_option(&options, "judgment-id")?;
    let option_id = required_option(&options, "option-id")?;
    let surface = selected_user_surface(&runtime_home, &project_id, &options)?;
    let store = CoreProjectStore::open(&runtime_home, &ProjectId::new(&project_id))?;
    let state_version = match options.value_ref("expected-state-version") {
        Some(value) => Some(parse_u64_option("expected-state-version", value)?),
        None => Some(store.project_state()?.state_version),
    };
    let record = store.user_judgment_record(&judgment_id)?.ok_or_else(|| {
        UserCommandError::runtime(format!("user judgment not found: {judgment_id}"))
    })?;
    let display = judgment_display(&record)?;
    if display.status != "pending" {
        return Err(UserCommandError::runtime(format!(
            "user judgment is not pending: {}",
            display.status
        )));
    }
    let selected = display
        .options
        .iter()
        .find(|option| option.option_id.as_str() == option_id)
        .cloned()
        .ok_or_else(|| {
            UserCommandError::usage(format!(
                "--option-id must name one of the Core-generated options for {judgment_id}"
            ))
        })?;

    let request_id = options
        .value("request-id")
        .unwrap_or_else(|| generated_id("req_user_judgment"));
    let idempotency_key = options
        .value("idempotency-key")
        .unwrap_or_else(|| generated_id("idem_user_judgment"));
    let request = RecordUserJudgmentRequest {
        envelope: ToolEnvelope {
            project_id: ProjectId::new(&project_id),
            task_id: Some(TaskId::new(&display.task_id)).into(),
            actor_kind: ActorKind::User,
            surface_id: SurfaceId::new(&surface.surface_id),
            request_id: RequestId::new(request_id.clone()),
            idempotency_key: Some(IdempotencyKey::new(idempotency_key.clone())).into(),
            expected_state_version: state_version.into(),
            dry_run: false,
            locale: None.into(),
        },
        user_judgment_id: volicord_types::UserJudgmentId::new(&display.judgment_id),
        judgment_kind: display.judgment_kind_value,
        selected_option_id: UserJudgmentOptionId::new(&option_id),
        answer: answer_for_selected_option(&display, &selected)?,
        rationale: rationale_for_selected_option(&display, &selected),
        note: options.value("note").into(),
        accepted_risks: Vec::new(),
    };
    let response = CoreService::new(&runtime_home).record_user_judgment(
        request,
        invocation(&project_id, &surface, AccessClass::CoreMutation),
    )?;
    if is_rejected(&response.response_value) {
        return Err(UserCommandError::runtime(rejected_summary(
            &response.response_value,
        )));
    }

    Ok(format!(
        "user judgment recorded\nproject_id: {}\ntask_id: {}\nuser_judgment_id: {}\nselected_option_id: {}\nmachine_action: {}\nresolution_outcome: {}\nstate_version: {}\nrequest_id: {}\nidempotency_key: {}\n",
        project_id,
        display.task_id,
        display.judgment_id,
        option_id,
        enum_string(selected.machine_action)?,
        enum_string(selected.resolution_outcome)?,
        response.response_value["base"]["state_version"].as_u64().unwrap_or_default(),
        request_id,
        idempotency_key
    ))
}

fn judgment_usage() -> String {
    "volicord user judgment list --project-id ID [--task-id ID] [--surface-id ID] [--surface-instance-id ID]\nvolicord user judgment show --project-id ID --judgment-id ID [--surface-id ID] [--surface-instance-id ID]\nvolicord user judgment record --project-id ID --judgment-id ID --option-id ID [--surface-id ID] [--surface-instance-id ID] [--note TEXT] [--request-id ID] [--idempotency-key KEY] [--expected-state-version VERSION]\n".to_owned()
}

fn parse_options(args: &[String], allowed: &[&str]) -> Result<CliOptions, UserCommandError> {
    let mut options = BTreeMap::new();
    let mut index = 0;

    while index < args.len() {
        let token = &args[index];
        if token == "-h" || token == "--help" || token == "help" {
            return Err(UserCommandError::usage(user_usage()));
        }
        if !token.starts_with("--") {
            return Err(UserCommandError::usage(format!(
                "unexpected argument: {token}"
            )));
        }
        let without_prefix = &token[2..];
        let (name, value) = if let Some((name, value)) = without_prefix.split_once('=') {
            (name.to_owned(), value.to_owned())
        } else {
            index += 1;
            let Some(value) = args.get(index) else {
                return Err(UserCommandError::usage(format!(
                    "missing value for --{without_prefix}"
                )));
            };
            (without_prefix.to_owned(), value.clone())
        };
        if !allowed.iter().any(|allowed_name| *allowed_name == name) {
            return Err(UserCommandError::usage(format!("unknown option: --{name}")));
        }
        if options.insert(name.clone(), vec![value]).is_some() {
            return Err(UserCommandError::usage(format!(
                "duplicate option: --{name}"
            )));
        }
        index += 1;
    }
    Ok(options)
}

fn parse_surface_options(
    args: &[String],
    allowed: &[&str],
) -> Result<CliOptions, UserCommandError> {
    let mut allowed = allowed.to_vec();
    allowed.push("surface-id");
    allowed.push("surface-instance-id");
    parse_options(args, &allowed)
}

trait CliOptionsExt {
    fn value(&self, name: &str) -> Option<String>;
    fn value_ref(&self, name: &str) -> Option<&String>;
}

impl CliOptionsExt for CliOptions {
    fn value(&self, name: &str) -> Option<String> {
        self.value_ref(name).cloned()
    }

    fn value_ref(&self, name: &str) -> Option<&String> {
        self.get(name).and_then(|values| values.first())
    }
}

fn required_option(options: &CliOptions, name: &str) -> Result<String, UserCommandError> {
    options
        .value_ref(name)
        .filter(|value| !value.trim().is_empty())
        .cloned()
        .ok_or_else(|| UserCommandError::usage(format!("missing required option: --{name}")))
}

fn selected_user_surface(
    runtime_home: &Path,
    project_id: &str,
    options: &CliOptions,
) -> Result<SurfaceRecord, UserCommandError> {
    let surface_id = options.value_ref("surface-id");
    let surface_instance_id = options.value_ref("surface-instance-id");
    let mut candidates = list_surfaces(runtime_home, project_id)?
        .into_iter()
        .filter(|surface| {
            surface.interaction_role == SurfaceInteractionRole::UserInteraction.as_str()
        })
        .filter(|surface| {
            surface_id
                .map(|value| surface.surface_id == *value)
                .unwrap_or(true)
        })
        .filter(|surface| {
            surface_instance_id
                .map(|value| surface.surface_instance_id == *value)
                .unwrap_or(true)
        })
        .collect::<Vec<_>>();

    match candidates.len() {
        0 => Err(UserCommandError::runtime(
            "no matching user_interaction surface is registered; run volicord user setup",
        )),
        1 => {
            let surface = candidates.remove(0);
            let access_classes =
                normalized_access_classes_from_local_access(&surface.local_access_json)?;
            if !access_classes.contains(&AccessClass::ReadStatus)
                || !access_classes.contains(&AccessClass::CoreMutation)
            {
                return Err(UserCommandError::runtime(
                    "selected user_interaction surface must grant read_status and core_mutation",
                ));
            }
            Ok(surface)
        }
        _ => Err(UserCommandError::usage(
            "multiple user_interaction surfaces match; provide --surface-id and --surface-instance-id",
        )),
    }
}

fn invocation(
    project_id: &str,
    surface: &SurfaceRecord,
    access_class: AccessClass,
) -> InvocationContext {
    InvocationContext {
        binding: AdapterSessionBinding::new(
            ProjectId::new(project_id),
            SurfaceId::new(&surface.surface_id),
            SurfaceInstanceId::new(&surface.surface_instance_id),
            VERIFICATION_BASIS_CLI_DIRECT_SURFACE_BINDING,
        ),
        requested_access_class: access_class,
    }
}

fn status_request(
    project_id: &str,
    task_id: Option<&str>,
    surface: &SurfaceRecord,
) -> StatusRequest {
    StatusRequest {
        envelope: ToolEnvelope {
            project_id: ProjectId::new(project_id),
            task_id: task_id.map(TaskId::new).into(),
            actor_kind: ActorKind::User,
            surface_id: SurfaceId::new(&surface.surface_id),
            request_id: RequestId::new(generated_id("req_user_status")),
            idempotency_key: None.into(),
            expected_state_version: None.into(),
            dry_run: false,
            locale: None.into(),
        },
        include: StatusInclude {
            task: true,
            pending_user_judgments: true,
            write_authority: true,
            evidence: true,
            close: true,
            guarantees: true,
        },
    }
}

fn verify_status_read(
    runtime_home: &Path,
    project_id: &str,
    task_id: &str,
    surface: &SurfaceRecord,
) -> Result<(), UserCommandError> {
    let response = CoreService::new(runtime_home).status(
        status_request(project_id, Some(task_id), surface),
        invocation(project_id, surface, AccessClass::ReadStatus),
    )?;
    if is_rejected(&response.response_value) {
        Err(UserCommandError::runtime(rejected_summary(
            &response.response_value,
        )))
    } else {
        Ok(())
    }
}

fn user_judgment_record(
    runtime_home: &Path,
    project_id: &str,
    judgment_id: &str,
) -> Result<UserJudgmentRecord, UserCommandError> {
    CoreProjectStore::open(runtime_home, &ProjectId::new(project_id))?
        .user_judgment_record(judgment_id)?
        .ok_or_else(|| UserCommandError::runtime(format!("user judgment not found: {judgment_id}")))
}

#[derive(Debug, Clone)]
struct JudgmentDisplay {
    judgment_id: String,
    task_id: String,
    change_unit_id: Option<String>,
    judgment_kind: String,
    judgment_kind_value: JudgmentKind,
    status: String,
    question: String,
    context_summary: String,
    options: Vec<UserJudgmentOption>,
    basis: PersistedJudgmentBasis,
}

fn judgment_display(record: &UserJudgmentRecord) -> Result<JudgmentDisplay, UserCommandError> {
    let request: PersistedUserJudgmentRequest =
        decode_json("user_judgments.request_json", &record.request_json)?;
    let options = decode_json::<PersistedUserJudgmentOptions>(
        "user_judgments.options_json",
        &record.options_json,
    )?
    .into_current_options()
    .map_err(|_| {
        UserCommandError::runtime(format!(
            "corrupt user_judgments.options_json for {}",
            record.judgment_id
        ))
    })?;
    let context = decode_json::<volicord_types::UserJudgmentContext>(
        "user_judgments.context_json",
        &record.context_json,
    )?;
    let basis: PersistedJudgmentBasis =
        decode_json("user_judgments.basis_json", &record.basis_json)?;
    let judgment_kind_value = parse_enum("user_judgments.judgment_kind", &record.judgment_kind)?;

    Ok(JudgmentDisplay {
        judgment_id: record.judgment_id.clone(),
        task_id: record.task_id.clone(),
        change_unit_id: record.change_unit_id.clone(),
        judgment_kind: record.judgment_kind.clone(),
        judgment_kind_value,
        status: record.status.clone(),
        question: request.question,
        context_summary: context.summary,
        options,
        basis,
    })
}

fn answer_for_selected_option(
    display: &JudgmentDisplay,
    option: &UserJudgmentOption,
) -> Result<RecordUserJudgmentPayload, UserCommandError> {
    let outcome = enum_string(option.resolution_outcome)?;
    let mut payload = empty_answer_payload();
    match display.judgment_kind_value {
        JudgmentKind::ProductDecision => {
            payload.product_decision = Some(json_object(json!({
                "judgment": {
                    "decision": outcome,
                    "rationale": "Recorded from the selected Core-generated option."
                }
            })))
            .into();
        }
        JudgmentKind::TechnicalDecision => {
            payload.technical_decision = Some(json_object(json!({
                "judgment": {
                    "decision": outcome,
                    "rationale": "Recorded from the selected Core-generated option."
                }
            })))
            .into();
        }
        JudgmentKind::ScopeDecision => {
            payload.scope_decision = Some(json_object(json!({
                "decision": outcome
            })))
            .into();
        }
        JudgmentKind::SensitiveApproval => {
            payload.sensitive_action_scope = display
                .basis
                .sensitive_action_scope
                .as_ref()
                .cloned()
                .ok_or_else(|| {
                    UserCommandError::runtime(
                        "pending sensitive approval has no SensitiveActionScope in its basis",
                    )
                })
                .map(RequiredNullable::some)?;
        }
        JudgmentKind::FinalAcceptance => {
            payload.final_acceptance = Some(json_object(json!({
                "judgment": {
                    "decision": outcome,
                    "basis": "Recorded from the selected Core-generated option."
                }
            })))
            .into();
        }
        JudgmentKind::ResidualRiskAcceptance => {
            let risk_ids = display
                .basis
                .residual_risk_ids
                .iter()
                .map(RiskId::as_str)
                .collect::<Vec<_>>();
            payload.residual_risk_acceptance = Some(json_object(json!({
                "risk_ids": risk_ids,
                "decision": outcome
            })))
            .into();
        }
        JudgmentKind::Cancellation => {
            payload.cancellation = Some(json_object(json!({
                "decision": outcome,
                "reason": "Recorded from the selected Core-generated option."
            })))
            .into();
        }
    }
    ensure_option_action_matches_outcome(option)?;
    Ok(payload)
}

fn ensure_option_action_matches_outcome(
    option: &UserJudgmentOption,
) -> Result<(), UserCommandError> {
    if option.machine_action.resolution_outcome() == option.resolution_outcome {
        Ok(())
    } else {
        Err(UserCommandError::runtime(
            "selected option has inconsistent machine_action and resolution_outcome",
        ))
    }
}

fn rationale_for_selected_option(
    display: &JudgmentDisplay,
    option: &UserJudgmentOption,
) -> JudgmentRationale {
    JudgmentRationale {
        summary: format!("Selected '{}' for {}.", option.label, display.question),
        selected_reason: Some("Recorded from the selected Core-generated option.".to_owned())
            .into(),
        considered_alternatives: display
            .options
            .iter()
            .filter(|candidate| candidate.option_id != option.option_id)
            .map(|candidate| candidate.label.clone())
            .collect(),
        rejected_alternatives: Vec::new(),
        assumptions: vec!["The pending judgment basis remains current at record time.".to_owned()],
        tradeoffs: vec![
            "The recorded rationale preserves intent but does not override the selected option."
                .to_owned(),
        ],
        uncertainties: Vec::new(),
        review_triggers: vec![
            "Review if the Task, scope, close basis, or sensitive-action basis changes.".to_owned(),
        ],
        related_refs: Vec::new(),
        artifact_refs: Vec::new(),
    }
}

fn empty_answer_payload() -> RecordUserJudgmentPayload {
    RecordUserJudgmentPayload {
        product_decision: None.into(),
        technical_decision: None.into(),
        scope_decision: None.into(),
        sensitive_action_scope: None.into(),
        final_acceptance: None.into(),
        residual_risk_acceptance: None.into(),
        cancellation: None.into(),
    }
}

fn json_object(value: Value) -> Map<String, Value> {
    value
        .as_object()
        .cloned()
        .expect("literal value should be a JSON object")
}

fn format_status(project_id: &str, value: &Value) -> String {
    let base = &value["base"];
    let pending = value["pending_user_judgments"]
        .as_array()
        .map(Vec::len)
        .unwrap_or_default();
    let task_id = status_task_id(value).unwrap_or_else(|| "none".to_owned());
    let close_state = value["close_state"].as_str().unwrap_or("none");
    let mut output = format!(
        "user task status\nproject_id: {}\ntask_id: {}\nstate_version: {}\nstatus_summary: {}\npending_user_judgments: {}\nclose_state: {}\n",
        project_id,
        task_id,
        base["state_version"].as_u64().unwrap_or_default(),
        value["status_summary"].as_str().unwrap_or(""),
        pending,
        close_state
    );
    if let Some(actions) = value["next_actions"]
        .as_array()
        .filter(|actions| !actions.is_empty())
    {
        output.push_str("next_actions:\n");
        for action in actions {
            output.push_str(&format!(
                "- {}\t{}\t{}\n",
                action["action_kind"].as_str().unwrap_or(""),
                action["owner_method"].as_str().unwrap_or(""),
                tab_safe(action["label"].as_str().unwrap_or(""))
            ));
        }
    }
    output
}

fn format_judgment_show(display: &JudgmentDisplay) -> String {
    let mut output = format!(
        "user judgment\njudgment_id: {}\ntask_id: {}\nchange_unit_id: {}\njudgment_kind: {}\nstatus: {}\nquestion: {}\ncontext_summary: {}\noptions:\n",
        display.judgment_id,
        display.task_id,
        display.change_unit_id.as_deref().unwrap_or(""),
        display.judgment_kind,
        display.status,
        display.question,
        display.context_summary
    );
    for option in &display.options {
        output.push_str(&format!(
            "- option_id: {}\n  label: {}\n  description: {}\n  consequence: {}\n  machine_action: {}\n  resolution_outcome: {}\n  is_default: {}\n",
            option.option_id.as_str(),
            option.label,
            option.description,
            option.consequence,
            enum_string(option.machine_action).unwrap_or_default(),
            enum_string(option.resolution_outcome).unwrap_or_default(),
            option.is_default
        ));
    }
    output
}

fn status_task_id(value: &Value) -> Option<String> {
    value["active_task"]["task_ref"]["record_id"]
        .as_str()
        .map(str::to_owned)
}

fn is_rejected(value: &Value) -> bool {
    value["base"]["response_kind"] == "rejected"
}

fn rejected_summary(value: &Value) -> String {
    let errors = value["errors"].as_array().cloned().unwrap_or_default();
    if errors.is_empty() {
        return "Core rejected the request".to_owned();
    }
    errors
        .iter()
        .map(|error| {
            format!(
                "{}: {}",
                error["code"].as_str().unwrap_or("UNKNOWN"),
                error["message"].as_str().unwrap_or("")
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn parse_u64_option(name: &str, value: &str) -> Result<u64, UserCommandError> {
    value
        .parse::<u64>()
        .map_err(|_| UserCommandError::usage(format!("--{name} must be an unsigned integer")))
}

fn decode_json<T>(field: &'static str, raw: &str) -> Result<T, UserCommandError>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(raw)
        .map_err(|error| UserCommandError::runtime(format!("invalid {field}: {error}")))
}

fn parse_enum<T>(field: &'static str, raw: &str) -> Result<T, UserCommandError>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_value(Value::String(raw.to_owned()))
        .map_err(|_| UserCommandError::runtime(format!("invalid {field}: {raw}")))
}

fn enum_string<T>(value: T) -> Result<String, UserCommandError>
where
    T: serde::Serialize,
{
    match serde_json::to_value(value)
        .map_err(|error| UserCommandError::runtime(format!("failed to encode value: {error}")))?
    {
        Value::String(value) => Ok(value),
        _ => Err(UserCommandError::runtime(
            "failed to encode enum value as string",
        )),
    }
}

fn tab_safe(value: &str) -> String {
    value.replace(['\t', '\n', '\r'], " ")
}

fn generated_id(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    format!("{prefix}_{nanos}_{}", process::id())
}

#[cfg(test)]
mod tests {
    use std::{error::Error, ffi::OsString, path::PathBuf};

    use volicord_store::bootstrap::{
        initialize_runtime_home, list_surfaces, register_project, ProjectRegistration,
        ACTIVE_PROJECT_STATUS,
    };
    use volicord_test_support::TempRuntimeHome;
    use volicord_types::{
        BaselineRef, ChangeUnitId, ChangeUnitOperation, ChangeUnitUpdate, InitialScope,
        IntakeRequest, JudgmentBasisCompatibilityStatus, JudgmentKind, JudgmentPresentation,
        JudgmentRequiredFor, JudgmentResolutionOutcome, ProjectId, RecordId, RequestedMode,
        ResumePolicy, ScopeUpdate, StateRecordKind, StateRecordRef, UpdateScopeRequest,
        UserJudgmentContext, UserJudgmentOptionAction, UserJudgmentOptionInput,
    };

    use super::*;

    const PROJECT_ID: &str = "project_user_cli";

    struct UserCliFixture {
        _runtime_home: TempRuntimeHome,
        runtime_home: PathBuf,
        repo_root: PathBuf,
    }

    impl UserCliFixture {
        fn new(prefix: &str) -> Result<Self, Box<dyn Error>> {
            let runtime = TempRuntimeHome::new(prefix)?;
            let repo_root = runtime.create_product_repo("repo")?;
            initialize_runtime_home(runtime.path(), &format!("runtime_home_{prefix}"), "{}")?;
            register_project(
                runtime.path(),
                ProjectRegistration {
                    project_id: PROJECT_ID.to_owned(),
                    repo_root: repo_root.clone(),
                    project_home: None,
                    status: ACTIVE_PROJECT_STATUS.to_owned(),
                    metadata_json: "{}".to_owned(),
                },
            )?;
            let runtime_home = runtime.path().to_path_buf();
            Ok(Self {
                _runtime_home: runtime,
                runtime_home,
                repo_root,
            })
        }

        fn run_user<const N: usize>(&self, args: [&str; N]) -> Result<String, UserCommandError> {
            let args = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
            run_user_command(
                &args,
                |name| {
                    if name == "VOLICORD_HOME" {
                        Some(OsString::from(&self.runtime_home))
                    } else {
                        None
                    }
                },
                &self.repo_root,
            )
        }

        fn setup_user_surface(&self) -> Result<SurfaceRecord, Box<dyn Error>> {
            self.run_user(["setup", "--project-id", PROJECT_ID])?;
            let surface = list_surfaces(&self.runtime_home, PROJECT_ID)?
                .into_iter()
                .find(|surface| surface.surface_id == DEFAULT_USER_SURFACE_ID)
                .expect("setup should register default user surface");
            Ok(surface)
        }
    }

    #[test]
    fn user_setup_grants_only_status_and_core_mutation() -> Result<(), Box<dyn Error>> {
        let fixture = UserCliFixture::new("cli-user-setup")?;
        let output = fixture.run_user(["setup", "--project-id", PROJECT_ID])?;

        assert!(output.contains("user surface ready\n"));
        let surface = list_surfaces(&fixture.runtime_home, PROJECT_ID)?
            .into_iter()
            .find(|surface| surface.surface_id == DEFAULT_USER_SURFACE_ID)
            .expect("setup should register default user surface");
        assert_eq!(surface.interaction_role, "user_interaction");
        assert_eq!(
            normalized_access_classes_from_local_access(&surface.local_access_json)?,
            vec![AccessClass::ReadStatus, AccessClass::CoreMutation]
        );
        assert!(output.contains("access_class: read_status,core_mutation\n"));
        Ok(())
    }

    #[test]
    fn user_judgment_list_show_and_record_use_core_options() -> Result<(), Box<dyn Error>> {
        let fixture = UserCliFixture::new("cli-user-judgment")?;
        let surface = fixture.setup_user_surface()?;
        let (task_id, change_unit_id) = create_task_with_change_unit(&fixture, &surface)?;
        let judgment_id = request_pending_judgment(
            &fixture,
            &surface,
            &task_id,
            &change_unit_id,
            JudgmentKind::ProductDecision,
            "product",
        )?;

        let list = fixture.run_user(["judgment", "list", "--project-id", PROJECT_ID])?;
        assert!(list.starts_with("judgment_id\ttask_id\tjudgment_kind\tstatus\tquestion\n"));
        assert!(list.contains(&format!(
            "{judgment_id}\t{task_id}\tproduct_decision\tpending\t"
        )));

        let show = fixture.run_user([
            "judgment",
            "show",
            "--project-id",
            PROJECT_ID,
            "--judgment-id",
            &judgment_id,
        ])?;
        assert!(show.contains("judgment_kind: product_decision\n"));
        assert!(show.contains("option_id: accept\n"));
        assert!(show.contains("machine_action: accept\n"));
        assert!(show.contains("resolution_outcome: accepted\n"));

        let record = fixture.run_user([
            "judgment",
            "record",
            "--project-id",
            PROJECT_ID,
            "--judgment-id",
            &judgment_id,
            "--option-id",
            "accept",
            "--note",
            "Human selected the Core option.",
        ])?;
        assert!(record.contains("user judgment recorded\n"));
        assert!(record.contains("selected_option_id: accept\n"));
        assert!(record.contains("resolution_outcome: accepted\n"));

        let stored = CoreProjectStore::open(&fixture.runtime_home, &ProjectId::new(PROJECT_ID))?
            .user_judgment_record(&judgment_id)?
            .expect("judgment should still exist");
        assert_eq!(stored.status, "resolved");
        assert_eq!(stored.resolution_outcome.as_deref(), Some("accepted"));
        assert_eq!(stored.resolved_by_actor_kind.as_deref(), Some("user"));
        assert_eq!(
            stored.resolved_actor_role.as_deref(),
            Some("user_interaction")
        );
        assert!(stored
            .resolved_verification_basis
            .as_deref()
            .unwrap_or_default()
            .contains(VERIFICATION_BASIS_CLI_DIRECT_SURFACE_BINDING));
        Ok(())
    }

    #[test]
    fn user_judgment_record_rejects_agent_surface_selection() -> Result<(), Box<dyn Error>> {
        let fixture = UserCliFixture::new("cli-user-agent-reject")?;
        let user_surface = fixture.setup_user_surface()?;
        let (task_id, change_unit_id) = create_task_with_change_unit(&fixture, &user_surface)?;
        let judgment_id = request_pending_judgment(
            &fixture,
            &user_surface,
            &task_id,
            &change_unit_id,
            JudgmentKind::ScopeDecision,
            "scope",
        )?;
        let access_classes = vec![AccessClass::ReadStatus, AccessClass::CoreMutation];
        register_surface(
            &fixture.runtime_home,
            SurfaceRegistration {
                project_id: PROJECT_ID.to_owned(),
                surface_id: "surface_agent_cli".to_owned(),
                surface_instance_id: "surface_instance_agent_cli".to_owned(),
                surface_kind: "mcp".to_owned(),
                interaction_role: SurfaceInteractionRole::Agent,
                display_name: Some("Agent CLI".to_owned()),
                capability_profile_json: capability_profile_json(&access_classes, None)?,
                local_access_json: local_access_json(&access_classes)?,
                metadata_json: ADMIN_METADATA_JSON.to_owned(),
            },
        )?;

        let error = fixture
            .run_user([
                "judgment",
                "record",
                "--project-id",
                PROJECT_ID,
                "--judgment-id",
                &judgment_id,
                "--option-id",
                "accept",
                "--surface-id",
                "surface_agent_cli",
                "--surface-instance-id",
                "surface_instance_agent_cli",
            ])
            .expect_err("agent surface must not be usable as user interaction");
        assert!(matches!(error, UserCommandError::Runtime(_)));
        assert!(error
            .to_string()
            .contains("no matching user_interaction surface is registered"));

        let stored = CoreProjectStore::open(&fixture.runtime_home, &ProjectId::new(PROJECT_ID))?
            .user_judgment_record(&judgment_id)?
            .expect("judgment should still exist");
        assert_eq!(stored.status, "pending");
        assert_eq!(stored.resolution_outcome, None);
        Ok(())
    }

    #[test]
    fn user_judgment_answers_keep_acceptance_branches_separate() -> Result<(), Box<dyn Error>> {
        let option = accept_option();
        let final_display = judgment_display_for_kind(JudgmentKind::FinalAcceptance, Vec::new())?;
        let final_answer = answer_for_selected_option(&final_display, &option)?;

        assert!(final_answer.final_acceptance.is_some());
        assert!(final_answer.residual_risk_acceptance.is_none());

        let risk_display = judgment_display_for_kind(
            JudgmentKind::ResidualRiskAcceptance,
            vec![RiskId::new("risk_cli_001")],
        )?;
        let risk_answer = answer_for_selected_option(&risk_display, &option)?;
        let residual = risk_answer
            .residual_risk_acceptance
            .as_ref()
            .expect("residual-risk branch should be populated");

        assert!(risk_answer.final_acceptance.is_none());
        assert_eq!(residual.get("risk_ids"), Some(&json!(["risk_cli_001"])));
        assert_eq!(residual.get("decision"), Some(&json!("accepted")));
        Ok(())
    }

    fn create_task_with_change_unit(
        fixture: &UserCliFixture,
        surface: &SurfaceRecord,
    ) -> Result<(String, String), Box<dyn Error>> {
        let service = CoreService::new(&fixture.runtime_home);
        let intake = service.intake(
            IntakeRequest {
                envelope: test_envelope(surface, "req_task", Some("idem_task"), Some(0), None),
                plain_language_request: "Create a CLI user judgment fixture.".to_owned(),
                requested_mode: RequestedMode::Work,
                resume_policy: ResumePolicy::CreateNew,
                initial_scope: InitialScope {
                    boundary: "Initial CLI fixture scope.".to_owned(),
                    non_goals: vec!["Changing unrelated behavior.".to_owned()],
                    acceptance_criteria: vec![
                        "The CLI fixture task can be judged by the user.".to_owned()
                    ],
                },
                initial_context_refs: Vec::new(),
            },
            invocation(PROJECT_ID, surface, AccessClass::CoreMutation),
        )?;
        let task_id = intake.response_value["task_ref"]["record_id"]
            .as_str()
            .expect("task ref should be present")
            .to_owned();
        let mut fields = Map::new();
        fields.insert(
            "scope_summary".to_owned(),
            Value::String("Initial current scope.".to_owned()),
        );
        fields.insert("affected_paths".to_owned(), json!(["src/export.rs"]));
        let scope = service.update_scope(
            UpdateScopeRequest {
                envelope: test_envelope(
                    surface,
                    "req_scope",
                    Some("idem_scope"),
                    Some(1),
                    Some(&task_id),
                ),
                task_id: TaskId::new(&task_id),
                goal_summary: Some("Initial current scope.".to_owned()).into(),
                scope_update: Some(ScopeUpdate {
                    include: vec!["Initial current scope.".to_owned()],
                    exclude: vec!["Unrelated behavior.".to_owned()],
                })
                .into(),
                scope_boundary: Some("Initial current scope.".to_owned()).into(),
                non_goals: Some(vec!["Unrelated behavior.".to_owned()]).into(),
                acceptance_criteria: Some(vec![
                    "The scoped CLI fixture behavior is represented.".to_owned()
                ])
                .into(),
                autonomy_boundary: Some("Stay inside the CLI fixture scope.".to_owned()).into(),
                baseline_ref: Some(BaselineRef::new("baseline_cli")).into(),
                change_unit: ChangeUnitUpdate {
                    operation: ChangeUnitOperation::CreateCurrent,
                    effect_contract: None,
                    fields,
                },
                related_scope_decision_refs: Vec::new(),
            },
            invocation(PROJECT_ID, surface, AccessClass::CoreMutation),
        )?;
        let change_unit_id = scope.response_value["change_unit_ref"]["record_id"]
            .as_str()
            .expect("change unit ref should be present")
            .to_owned();
        Ok((task_id, change_unit_id))
    }

    fn request_pending_judgment(
        fixture: &UserCliFixture,
        surface: &SurfaceRecord,
        task_id: &str,
        change_unit_id: &str,
        judgment_kind: JudgmentKind,
        suffix: &str,
    ) -> Result<String, Box<dyn Error>> {
        let service = CoreService::new(&fixture.runtime_home);
        let request_id = format!("req_judgment_{suffix}");
        let idempotency_key = format!("idem_judgment_{suffix}");
        let response = service.request_user_judgment(
            volicord_types::RequestUserJudgmentRequest {
                envelope: test_envelope(
                    surface,
                    &request_id,
                    Some(&idempotency_key),
                    Some(2),
                    Some(task_id),
                ),
                task_id: TaskId::new(task_id),
                change_unit_id: Some(ChangeUnitId::new(change_unit_id)).into(),
                sensitive_action_scope: None.into(),
                judgment_kind,
                presentation: JudgmentPresentation::Short,
                question: "Choose the focused CLI judgment outcome.".to_owned(),
                options: Some(options_for_kind(judgment_kind)).into(),
                context: UserJudgmentContext {
                    summary: "A CLI-visible user judgment is pending.".to_owned(),
                    related_refs: Vec::new(),
                    artifact_refs: Vec::new(),
                    visible_risks: Vec::new(),
                    constraints: vec![
                        "Only the selected Core-generated option is recorded.".to_owned()
                    ],
                },
                affected_refs: vec![StateRecordRef {
                    record_kind: StateRecordKind::Task,
                    record_id: RecordId::new(task_id),
                    project_id: ProjectId::new(PROJECT_ID),
                    task_id: Some(TaskId::new(task_id)).into(),
                    state_version: Some(2).into(),
                }],
                required_for: required_for_kind(judgment_kind),
                expires_at: None.into(),
            },
            invocation(PROJECT_ID, surface, AccessClass::CoreMutation),
        )?;
        Ok(response.response_value["user_judgment_ref"]["record_id"]
            .as_str()
            .expect("judgment ref should be present")
            .to_owned())
    }

    fn options_for_kind(judgment_kind: JudgmentKind) -> Vec<UserJudgmentOptionInput> {
        if matches!(
            judgment_kind,
            JudgmentKind::ProductDecision | JudgmentKind::TechnicalDecision
        ) {
            vec![UserJudgmentOptionInput {
                option_id: UserJudgmentOptionId::new("accept"),
                label: "Accept".to_owned(),
                description: "Record the focused user-owned judgment.".to_owned(),
                consequence: "Only this judgment record is resolved.".to_owned(),
                is_default: true,
            }]
        } else {
            Vec::new()
        }
    }

    fn required_for_kind(judgment_kind: JudgmentKind) -> Vec<JudgmentRequiredFor> {
        match judgment_kind {
            JudgmentKind::ScopeDecision => vec![JudgmentRequiredFor::ScopeUpdate],
            JudgmentKind::Cancellation => vec![JudgmentRequiredFor::CloseCancel],
            JudgmentKind::SensitiveApproval => vec![JudgmentRequiredFor::PrepareWrite],
            JudgmentKind::ProductDecision
            | JudgmentKind::TechnicalDecision
            | JudgmentKind::FinalAcceptance
            | JudgmentKind::ResidualRiskAcceptance => vec![JudgmentRequiredFor::CloseComplete],
        }
    }

    fn test_envelope(
        surface: &SurfaceRecord,
        request_id: &str,
        idempotency_key: Option<&str>,
        expected_state_version: Option<u64>,
        task_id: Option<&str>,
    ) -> ToolEnvelope {
        ToolEnvelope {
            project_id: ProjectId::new(PROJECT_ID),
            task_id: task_id.map(TaskId::new).into(),
            actor_kind: ActorKind::Agent,
            surface_id: SurfaceId::new(&surface.surface_id),
            request_id: RequestId::new(request_id),
            idempotency_key: idempotency_key.map(IdempotencyKey::new).into(),
            expected_state_version: expected_state_version.into(),
            dry_run: false,
            locale: Some("en-US".to_owned()).into(),
        }
    }

    fn accept_option() -> UserJudgmentOption {
        UserJudgmentOption {
            option_id: UserJudgmentOptionId::new("accept"),
            label: "Accept".to_owned(),
            description: "Accept the focused CLI judgment.".to_owned(),
            consequence: "Only the selected judgment branch is populated.".to_owned(),
            machine_action: UserJudgmentOptionAction::Accept,
            resolution_outcome: JudgmentResolutionOutcome::Accepted,
            is_default: true,
        }
    }

    fn judgment_display_for_kind(
        judgment_kind: JudgmentKind,
        residual_risk_ids: Vec<RiskId>,
    ) -> Result<JudgmentDisplay, UserCommandError> {
        Ok(JudgmentDisplay {
            judgment_id: "judgment_cli_branch".to_owned(),
            task_id: "task_cli_branch".to_owned(),
            change_unit_id: None,
            judgment_kind: enum_string(judgment_kind)?,
            judgment_kind_value: judgment_kind,
            status: "pending".to_owned(),
            question: "Choose the focused CLI branch.".to_owned(),
            context_summary: "The answer should populate only its judgment branch.".to_owned(),
            options: vec![accept_option()],
            basis: PersistedJudgmentBasis {
                task_id: TaskId::new("task_cli_branch"),
                change_unit_id: None.into(),
                scope_revision: 1,
                close_basis_revision: Some(1).into(),
                baseline_ref: None.into(),
                result_refs: Vec::new(),
                residual_risk_ids,
                sensitive_action_scope: None.into(),
                created_at_state_version: 1,
                compatibility_status: JudgmentBasisCompatibilityStatus::Current,
            },
        })
    }
}
