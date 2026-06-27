use std::{
    collections::BTreeMap,
    fmt,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::{json, Value};
use volicord_core::{CorePipelineError, CoreService, InvocationContext, PipelineResponse};
use volicord_store::{
    core_pipeline::{CoreProjectStore, UserJudgmentRecord},
    runtime_home::{resolve_runtime_home, RuntimeHomeResolutionError},
    StoreError,
};
use volicord_types::{
    ActorSource, IdempotencyKey, JudgmentKind, JudgmentRationale, JudgmentResolutionOutcome,
    OperationCategory, PersistedUserJudgmentOptions, ProjectId, RecordUserJudgmentPayload,
    RecordUserJudgmentRequest, RequestId, SensitiveActionScope, StatusInclude, StatusRequest,
    TaskId, ToolEnvelope, UserJudgmentContext, UserJudgmentId, UserJudgmentOption,
    UserJudgmentOptionId, VERIFICATION_BASIS_CLI_DIRECT_USER_CHANNEL,
};

type UserOptions = BTreeMap<String, Vec<String>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserCommandError {
    Usage(String),
    Runtime(String),
}

impl fmt::Display for UserCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) | Self::Runtime(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for UserCommandError {}

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

impl From<CorePipelineError> for UserCommandError {
    fn from(error: CorePipelineError) -> Self {
        Self::Runtime(error.to_string())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Debug, Clone, Default)]
struct ParsedUserOptions {
    project_id: Option<String>,
    task_id: Option<String>,
    judgment_id: Option<String>,
    option_id: Option<String>,
    request_id: Option<String>,
    idempotency_key: Option<String>,
    expected_state_version: Option<u64>,
    note: Option<String>,
    runtime_home: Option<PathBuf>,
    output: OutputFormat,
}

pub fn user_usage() -> String {
    concat!(
        "volicord user status --project-id ID [--task-id ID] [--runtime-home PATH] [--output text|json]\n",
        "volicord user judgment list --project-id ID [--task-id ID] [--runtime-home PATH] [--output text|json]\n",
        "volicord user judgment show --project-id ID --judgment-id ID [--runtime-home PATH] [--output text|json]\n",
        "volicord user judgment record --project-id ID --judgment-id ID --option-id ID [--request-id ID] [--idempotency-key KEY] [--expected-state-version VERSION] [--note TEXT] [--runtime-home PATH] [--output text|json]\n"
    )
    .to_owned()
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
        return Ok(user_usage());
    };

    match subcommand {
        "-h" | "--help" | "help" => {
            if args.len() == 1 {
                Ok(user_usage())
            } else {
                Err(UserCommandError::Usage(format!(
                    "unexpected argument: {}\n\n{}",
                    args[1],
                    user_usage()
                )))
            }
        }
        "status" => command_status(&args[1..], env_var, current_dir),
        "judgment" => command_judgment(&args[1..], env_var, current_dir),
        other => Err(UserCommandError::Usage(format!(
            "unknown user command: {other}\n\n{}",
            user_usage()
        ))),
    }
}

fn command_status<F>(
    args: &[String],
    env_var: F,
    current_dir: &Path,
) -> Result<String, UserCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let parsed = parse_user_options(args, status_allowed_options(), current_dir)?;
    let project_id = required_text(parsed.project_id.as_deref(), "project-id")?;
    let runtime_home = resolve_user_runtime_home(&parsed, env_var, current_dir)?;
    let response = CoreService::new(&runtime_home).status(
        StatusRequest {
            envelope: envelope(
                project_id,
                parsed.task_id.as_deref(),
                generated_id("req_user_status"),
                None,
                None,
            ),
            include: StatusInclude {
                task: true,
                pending_user_judgments: true,
                write_check: true,
                evidence: true,
                close: true,
                guarantees: true,
                continuity: true,
            },
        },
        invocation(project_id, OperationCategory::Read),
    )?;
    render_status_response(&response, parsed.output)
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
        return Err(UserCommandError::Usage(user_usage()));
    };
    match subcommand {
        "list" => command_judgment_list(&args[1..], env_var, current_dir),
        "show" => command_judgment_show(&args[1..], env_var, current_dir),
        "record" => command_judgment_record(&args[1..], env_var, current_dir),
        "-h" | "--help" | "help" => Ok(user_usage()),
        other => Err(UserCommandError::Usage(format!(
            "unknown user judgment command: {other}\n\n{}",
            user_usage()
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
    let parsed = parse_user_options(args, list_allowed_options(), current_dir)?;
    let project_id = required_text(parsed.project_id.as_deref(), "project-id")?;
    let runtime_home = resolve_user_runtime_home(&parsed, env_var, current_dir)?;
    let store = CoreProjectStore::open(&runtime_home, &ProjectId::new(project_id))?;
    let Some(task_id) = selected_or_active_task_id(&store, parsed.task_id.as_deref())? else {
        return render_judgment_records(Vec::new(), parsed.output);
    };
    let records = store.pending_user_judgment_records(&TaskId::new(task_id))?;
    render_judgment_records(records, parsed.output)
}

fn command_judgment_show<F>(
    args: &[String],
    env_var: F,
    current_dir: &Path,
) -> Result<String, UserCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let parsed = parse_user_options(args, show_allowed_options(), current_dir)?;
    let project_id = required_text(parsed.project_id.as_deref(), "project-id")?;
    let judgment_id = required_text(parsed.judgment_id.as_deref(), "judgment-id")?;
    let runtime_home = resolve_user_runtime_home(&parsed, env_var, current_dir)?;
    let record = read_judgment_record(&runtime_home, project_id, judgment_id)?;
    render_judgment_record(&record, parsed.output)
}

fn command_judgment_record<F>(
    args: &[String],
    env_var: F,
    current_dir: &Path,
) -> Result<String, UserCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    let parsed = parse_user_options(args, record_allowed_options(), current_dir)?;
    let project_id = required_text(parsed.project_id.as_deref(), "project-id")?;
    let judgment_id = required_text(parsed.judgment_id.as_deref(), "judgment-id")?;
    let option_id = required_text(parsed.option_id.as_deref(), "option-id")?;
    let runtime_home = resolve_user_runtime_home(&parsed, env_var, current_dir)?;
    let store = CoreProjectStore::open(&runtime_home, &ProjectId::new(project_id))?;
    let state_version = store.project_state()?.state_version;
    let record = store.user_judgment_record(judgment_id)?.ok_or_else(|| {
        UserCommandError::Runtime(format!("UserJudgment not found: {judgment_id}"))
    })?;
    if record.status != "pending" {
        return Err(UserCommandError::Runtime(format!(
            "UserJudgment is not pending: {judgment_id}"
        )));
    }
    let judgment_kind = parse_judgment_kind(&record.judgment_kind)?;
    let context = decode_json::<UserJudgmentContext>("context_json", &record.context_json)?;
    let options = decode_options(&record)?;
    let selected_option = options
        .iter()
        .find(|option| option.option_id.as_str() == option_id)
        .ok_or_else(|| {
            UserCommandError::Usage(format!(
                "--option-id must name one of the pending judgment options: {option_id}"
            ))
        })?;
    let request_id = parsed
        .request_id
        .clone()
        .unwrap_or_else(|| generated_id("req_user_judgment_record"));
    let idempotency_key = parsed
        .idempotency_key
        .clone()
        .unwrap_or_else(|| generated_id("idem_user_judgment_record"));
    let expected_state_version = parsed.expected_state_version.unwrap_or(state_version);
    let response = CoreService::new(&runtime_home).record_user_judgment(
        RecordUserJudgmentRequest {
            envelope: envelope(
                project_id,
                Some(&record.task_id),
                request_id,
                Some(idempotency_key),
                Some(expected_state_version),
            ),
            user_judgment_id: UserJudgmentId::new(judgment_id),
            judgment_kind,
            selected_option_id: UserJudgmentOptionId::new(option_id),
            answer: answer_payload_for_record(judgment_kind, selected_option, &record, &context)?,
            rationale: rationale_for_selected_option(judgment_kind, selected_option),
            note: parsed.note.into(),
            accepted_risks: accepted_risks_for_record(judgment_kind, selected_option, &context),
        },
        invocation(project_id, OperationCategory::UserOnly),
    )?;
    render_record_response(&response, parsed.output)
}

fn parse_user_options(
    args: &[String],
    allowed: &[&str],
    current_dir: &Path,
) -> Result<ParsedUserOptions, UserCommandError> {
    let options = parse_options(args, allowed)?;
    let expected_state_version = options
        .value("expected-state-version")
        .map(|value| {
            value.parse::<u64>().map_err(|_| {
                UserCommandError::Usage("--expected-state-version must be an integer".to_owned())
            })
        })
        .transpose()?;
    let runtime_home = options
        .value("runtime-home")
        .map(PathBuf::from)
        .map(|path| absolute_path(current_dir, path));
    let output = match options.value("output").as_deref().unwrap_or("text") {
        "text" => OutputFormat::Text,
        "json" => OutputFormat::Json,
        other => {
            return Err(UserCommandError::Usage(format!(
                "unknown output format: {other}"
            )))
        }
    };
    Ok(ParsedUserOptions {
        project_id: options.value("project-id"),
        task_id: options.value("task-id"),
        judgment_id: options.value("judgment-id"),
        option_id: options.value("option-id"),
        request_id: options.value("request-id"),
        idempotency_key: options.value("idempotency-key"),
        expected_state_version,
        note: options.value("note"),
        runtime_home,
        output,
    })
}

fn parse_options(args: &[String], allowed: &[&str]) -> Result<UserOptions, UserCommandError> {
    let mut options = BTreeMap::new();
    let mut index = 0;
    while index < args.len() {
        let token = &args[index];
        if token == "-h" || token == "--help" || token == "help" {
            return Err(UserCommandError::Usage(user_usage()));
        }
        if !token.starts_with("--") {
            return Err(UserCommandError::Usage(format!(
                "unexpected argument: {token}"
            )));
        }
        let without_prefix = &token[2..];
        let (name, value) = if let Some((name, value)) = without_prefix.split_once('=') {
            (name.to_owned(), value.to_owned())
        } else {
            index += 1;
            let Some(value) = args.get(index) else {
                return Err(UserCommandError::Usage(format!(
                    "missing value for --{without_prefix}"
                )));
            };
            (without_prefix.to_owned(), value.clone())
        };
        if !allowed.iter().any(|allowed_name| *allowed_name == name) {
            return Err(UserCommandError::Usage(format!("unknown option: --{name}")));
        }
        if options.insert(name.clone(), vec![value]).is_some() {
            return Err(UserCommandError::Usage(format!(
                "duplicate option: --{name}"
            )));
        }
        index += 1;
    }
    Ok(options)
}

trait UserOptionsExt {
    fn value(&self, name: &str) -> Option<String>;
}

impl UserOptionsExt for UserOptions {
    fn value(&self, name: &str) -> Option<String> {
        self.get(name).and_then(|values| values.first()).cloned()
    }
}

fn status_allowed_options() -> &'static [&'static str] {
    &["project-id", "task-id", "runtime-home", "output"]
}

fn list_allowed_options() -> &'static [&'static str] {
    &["project-id", "task-id", "runtime-home", "output"]
}

fn show_allowed_options() -> &'static [&'static str] {
    &["project-id", "judgment-id", "runtime-home", "output"]
}

fn record_allowed_options() -> &'static [&'static str] {
    &[
        "project-id",
        "judgment-id",
        "option-id",
        "request-id",
        "idempotency-key",
        "expected-state-version",
        "note",
        "runtime-home",
        "output",
    ]
}

fn resolve_user_runtime_home<F>(
    parsed: &ParsedUserOptions,
    env_var: F,
    current_dir: &Path,
) -> Result<PathBuf, UserCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    if let Some(path) = &parsed.runtime_home {
        Ok(path.clone())
    } else {
        resolve_runtime_home(env_var, current_dir).map_err(Into::into)
    }
}

fn read_judgment_record(
    runtime_home: &Path,
    project_id: &str,
    judgment_id: &str,
) -> Result<UserJudgmentRecord, UserCommandError> {
    let store = CoreProjectStore::open(runtime_home, &ProjectId::new(project_id))?;
    store
        .user_judgment_record(judgment_id)?
        .ok_or_else(|| UserCommandError::Runtime(format!("UserJudgment not found: {judgment_id}")))
}

fn selected_or_active_task_id(
    store: &CoreProjectStore,
    selected: Option<&str>,
) -> Result<Option<String>, UserCommandError> {
    if let Some(task_id) = selected {
        Ok(Some(task_id.to_owned()))
    } else {
        Ok(store.project_state()?.active_task_id)
    }
}

fn envelope(
    project_id: &str,
    task_id: Option<&str>,
    request_id: String,
    idempotency_key: Option<String>,
    expected_state_version: Option<u64>,
) -> ToolEnvelope {
    ToolEnvelope {
        project_id: ProjectId::new(project_id),
        task_id: task_id.map(TaskId::new).into(),
        request_id: RequestId::new(request_id),
        idempotency_key: idempotency_key.map(IdempotencyKey::new).into(),
        expected_state_version: expected_state_version.into(),
        dry_run: false,
        locale: None.into(),
    }
}

fn invocation(project_id: &str, operation_category: OperationCategory) -> InvocationContext {
    InvocationContext::new(
        ProjectId::new(project_id),
        ActorSource::LocalUser,
        operation_category,
        VERIFICATION_BASIS_CLI_DIRECT_USER_CHANNEL,
    )
}

fn decode_options(
    record: &UserJudgmentRecord,
) -> Result<Vec<UserJudgmentOption>, UserCommandError> {
    decode_json::<PersistedUserJudgmentOptions>("options_json", &record.options_json)?
        .into_current_options()
        .map_err(|error| UserCommandError::Runtime(error.to_string()))
}

fn decode_json<T>(field: &'static str, text: &str) -> Result<T, UserCommandError>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(text).map_err(|error| {
        UserCommandError::Runtime(format!("failed to decode user_judgments.{field}: {error}"))
    })
}

fn parse_judgment_kind(raw: &str) -> Result<JudgmentKind, UserCommandError> {
    serde_json::from_value(Value::String(raw.to_owned())).map_err(|_| {
        UserCommandError::Runtime(format!(
            "stored user_judgments.judgment_kind is not supported: {raw}"
        ))
    })
}

fn answer_payload_for_record(
    judgment_kind: JudgmentKind,
    selected_option: &UserJudgmentOption,
    record: &UserJudgmentRecord,
    context: &UserJudgmentContext,
) -> Result<RecordUserJudgmentPayload, UserCommandError> {
    let mut payload = empty_answer_payload();
    let branch = json_object(json!({
        "summary": format!("User selected option {}", selected_option.option_id),
        "selected_option": selected_option.option_id.as_str(),
        "selected_option_label": selected_option.label,
        "selected_option_consequence": selected_option.consequence,
    }));
    match judgment_kind {
        JudgmentKind::ProductDecision => payload.product_decision = Some(branch).into(),
        JudgmentKind::TechnicalDecision => payload.technical_decision = Some(branch).into(),
        JudgmentKind::ScopeDecision => payload.scope_decision = Some(branch).into(),
        JudgmentKind::SensitiveApproval => {
            payload.sensitive_action_scope =
                Some(sensitive_action_scope_for_record(record)?).into();
        }
        JudgmentKind::FinalAcceptance => payload.final_acceptance = Some(branch).into(),
        JudgmentKind::ResidualRiskAcceptance => {
            payload.residual_risk_acceptance = Some(json_object(json!({
                "summary": format!("User selected option {}", selected_option.option_id),
                "selected_option": selected_option.option_id.as_str(),
                "risk_ids": accepted_risk_ids(selected_option, context),
            })))
            .into();
        }
        JudgmentKind::Cancellation => payload.cancellation = Some(branch).into(),
    }
    Ok(payload)
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

fn sensitive_action_scope_for_record(
    record: &UserJudgmentRecord,
) -> Result<SensitiveActionScope, UserCommandError> {
    serde_json::from_str(&record.sensitive_action_scope_json).map_err(|error| {
        UserCommandError::Runtime(format!(
            "pending sensitive approval is missing a valid sensitive action scope: {error}"
        ))
    })
}

fn rationale_for_selected_option(
    judgment_kind: JudgmentKind,
    selected_option: &UserJudgmentOption,
) -> JudgmentRationale {
    let accepted = selected_option.resolution_outcome == JudgmentResolutionOutcome::Accepted;
    JudgmentRationale {
        summary: format!(
            "User selected `{}` for `{}` through the User Channel.",
            selected_option.option_id,
            judgment_kind_value(judgment_kind)
        ),
        selected_reason: Some(format!(
            "{} {}",
            selected_option.description, selected_option.consequence
        ))
        .into(),
        considered_alternatives: Vec::new(),
        rejected_alternatives: Vec::new(),
        assumptions: vec!["The answer covers only the addressed Core UserJudgment.".to_owned()],
        tradeoffs: if accepted {
            vec![selected_option.consequence.clone()]
        } else {
            Vec::new()
        },
        uncertainties: Vec::new(),
        review_triggers: if accepted {
            vec!["Revisit if the captured judgment basis becomes stale or superseded.".to_owned()]
        } else {
            Vec::new()
        },
        related_refs: Vec::new(),
        artifact_refs: Vec::new(),
    }
}

fn accepted_risks_for_record(
    judgment_kind: JudgmentKind,
    selected_option: &UserJudgmentOption,
    context: &UserJudgmentContext,
) -> Vec<volicord_types::AcceptedRiskInput> {
    if judgment_kind == JudgmentKind::ResidualRiskAcceptance
        && selected_option.resolution_outcome == JudgmentResolutionOutcome::Accepted
    {
        context.visible_risks.clone()
    } else {
        Vec::new()
    }
}

fn accepted_risk_ids(
    selected_option: &UserJudgmentOption,
    context: &UserJudgmentContext,
) -> Vec<String> {
    if selected_option.resolution_outcome == JudgmentResolutionOutcome::Accepted {
        context
            .visible_risks
            .iter()
            .map(|risk| risk.risk_id.as_str().to_owned())
            .collect()
    } else {
        Vec::new()
    }
}

fn render_status_response(
    response: &PipelineResponse,
    output: OutputFormat,
) -> Result<String, UserCommandError> {
    if output == OutputFormat::Json {
        return pretty_response(response);
    }
    if response_kind(response) != Some("result") {
        return render_rejected_or_json(response);
    }
    let mut output = String::new();
    output.push_str("User Channel status\n");
    if let Some(summary) = response.response_value["status_summary"].as_str() {
        output.push_str(&format!("summary: {summary}\n"));
    }
    if let Some(state_version) = response.response_value["base"]["state_version"].as_u64() {
        output.push_str(&format!("state_version: {state_version}\n"));
    }
    let pending = response.response_value["pending_user_judgments"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    output.push_str(&format!("pending_user_judgments: {}\n", pending.len()));
    for item in pending {
        if let Some(record_id) = item["record_id"].as_str() {
            output.push_str(&format!("- {record_id}\n"));
        }
    }
    Ok(output)
}

fn render_judgment_records(
    records: Vec<UserJudgmentRecord>,
    output: OutputFormat,
) -> Result<String, UserCommandError> {
    if output == OutputFormat::Json {
        let values = records
            .iter()
            .map(judgment_record_json)
            .collect::<Result<Vec<_>, _>>()?;
        return serde_json::to_string_pretty(&json!({ "pending_user_judgments": values }))
            .map(|text| format!("{text}\n"))
            .map_err(|error| UserCommandError::Runtime(error.to_string()));
    }

    let mut text = String::from("judgment_id\ttask_id\tjudgment_kind\tstatus\tquestion\toptions\n");
    for record in records {
        let request: volicord_types::PersistedUserJudgmentRequest =
            decode_json("request_json", &record.request_json)?;
        let options = decode_options(&record)?
            .into_iter()
            .map(|option| option.option_id.into_inner())
            .collect::<Vec<_>>()
            .join(",");
        text.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\n",
            record.judgment_id,
            record.task_id,
            record.judgment_kind,
            record.status,
            request.question,
            options
        ));
    }
    Ok(text)
}

fn render_judgment_record(
    record: &UserJudgmentRecord,
    output: OutputFormat,
) -> Result<String, UserCommandError> {
    if output == OutputFormat::Json {
        return serde_json::to_string_pretty(&judgment_record_json(record)?)
            .map(|text| format!("{text}\n"))
            .map_err(|error| UserCommandError::Runtime(error.to_string()));
    }
    let request: volicord_types::PersistedUserJudgmentRequest =
        decode_json("request_json", &record.request_json)?;
    let context: UserJudgmentContext = decode_json("context_json", &record.context_json)?;
    let options = decode_options(record)?;
    let mut text = format!(
        "UserJudgment {}\nproject_id: {}\ntask_id: {}\nstatus: {}\njudgment_kind: {}\nquestion: {}\ncontext: {}\n",
        record.judgment_id,
        record.project_id,
        record.task_id,
        record.status,
        record.judgment_kind,
        request.question,
        context.summary
    );
    text.push_str("options:\n");
    for option in options {
        text.push_str(&format!(
            "- {}: {} ({})\n  {}\n",
            option.option_id,
            option.label,
            outcome_value(option.resolution_outcome),
            option.consequence
        ));
    }
    Ok(text)
}

fn render_record_response(
    response: &PipelineResponse,
    output: OutputFormat,
) -> Result<String, UserCommandError> {
    if output == OutputFormat::Json {
        return pretty_response(response);
    }
    if response_kind(response) != Some("result") {
        return render_rejected_or_json(response);
    }
    let judgment = &response.response_value["user_judgment"];
    let resolution = &judgment["resolution"];
    let mut text = String::from("User Channel judgment recorded\n");
    if let Some(state_version) = response.response_value["base"]["state_version"].as_u64() {
        text.push_str(&format!("state_version: {state_version}\n"));
    }
    text.push_str(&format!(
        "judgment_id: {}\n",
        judgment["judgment_id"].as_str().unwrap_or("unknown")
    ));
    text.push_str(&format!(
        "selected_option_id: {}\n",
        resolution["selected_option_id"]
            .as_str()
            .unwrap_or("unknown")
    ));
    text.push_str(&format!(
        "resolved_by_actor_source: {}\n",
        resolution["resolved_by_actor_source"]
            .as_str()
            .unwrap_or("unknown")
    ));
    text.push_str(&format!(
        "operation_category: {}\n",
        OperationCategory::UserOnly.as_str()
    ));
    Ok(text)
}

fn pretty_response(response: &PipelineResponse) -> Result<String, UserCommandError> {
    serde_json::to_string_pretty(&response.response_value)
        .map(|text| format!("{text}\n"))
        .map_err(|error| UserCommandError::Runtime(error.to_string()))
}

fn render_rejected_or_json(response: &PipelineResponse) -> Result<String, UserCommandError> {
    if response.response_value["errors"].is_array() {
        let mut output = String::from("Core request rejected\n");
        for error in response.response_value["errors"]
            .as_array()
            .unwrap_or(&Vec::new())
        {
            output.push_str(&format!(
                "{}: {}\n",
                error["code"].as_str().unwrap_or("ERROR"),
                error["message"].as_str().unwrap_or("request rejected")
            ));
        }
        Ok(output)
    } else {
        pretty_response(response)
    }
}

fn judgment_record_json(record: &UserJudgmentRecord) -> Result<Value, UserCommandError> {
    let request: volicord_types::PersistedUserJudgmentRequest =
        decode_json("request_json", &record.request_json)?;
    let context: UserJudgmentContext = decode_json("context_json", &record.context_json)?;
    let options = decode_options(record)?;
    Ok(json!({
        "project_id": record.project_id,
        "judgment_id": record.judgment_id,
        "task_id": record.task_id,
        "change_unit_id": record.change_unit_id,
        "judgment_kind": record.judgment_kind,
        "status": record.status,
        "basis_status": record.basis_status,
        "question": request.question,
        "context_summary": context.summary,
        "options": options,
        "requested_by_actor_source": record.requested_by_actor_source,
        "requested_at": record.requested_at,
    }))
}

fn response_kind(response: &PipelineResponse) -> Option<&str> {
    response.response_value["base"]["response_kind"].as_str()
}

fn required_text<'a>(
    value: Option<&'a str>,
    field: &'static str,
) -> Result<&'a str, UserCommandError> {
    value
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| UserCommandError::Usage(format!("missing required option: --{field}")))
}

fn absolute_path(current_dir: &Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        current_dir.join(path)
    }
}

fn generated_id(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    format!("{prefix}_{nanos}")
}

fn judgment_kind_value(value: JudgmentKind) -> &'static str {
    match value {
        JudgmentKind::ProductDecision => "product_decision",
        JudgmentKind::TechnicalDecision => "technical_decision",
        JudgmentKind::ScopeDecision => "scope_decision",
        JudgmentKind::SensitiveApproval => "sensitive_approval",
        JudgmentKind::FinalAcceptance => "final_acceptance",
        JudgmentKind::ResidualRiskAcceptance => "residual_risk_acceptance",
        JudgmentKind::Cancellation => "cancellation",
    }
}

fn outcome_value(value: JudgmentResolutionOutcome) -> &'static str {
    match value {
        JudgmentResolutionOutcome::Accepted => "accepted",
        JudgmentResolutionOutcome::Rejected => "rejected",
        JudgmentResolutionOutcome::Deferred => "deferred",
    }
}

fn json_object(value: Value) -> serde_json::Map<String, Value> {
    match value {
        Value::Object(object) => object,
        _ => serde_json::Map::new(),
    }
}
