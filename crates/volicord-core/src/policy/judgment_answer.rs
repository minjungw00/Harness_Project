use serde_json::Value;
use volicord_types::{JudgmentKind, JudgmentResolutionOutcome, RecordUserJudgmentPayload};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AnswerOutcomeAgreement {
    Agrees,
    UnsupportedClaim,
    ConflictingClaim,
}

pub(crate) fn is_authority_bearing_judgment(judgment_kind: JudgmentKind) -> bool {
    matches!(
        judgment_kind,
        JudgmentKind::ScopeDecision
            | JudgmentKind::FinalAcceptance
            | JudgmentKind::ResidualRiskAcceptance
            | JudgmentKind::SensitiveApproval
            | JudgmentKind::Cancellation
    )
}

pub(crate) fn populated_answer_branch_count(answer: &RecordUserJudgmentPayload) -> usize {
    usize::from(answer.product_decision.is_some())
        + usize::from(answer.technical_decision.is_some())
        + usize::from(answer.scope_decision.is_some())
        + usize::from(answer.sensitive_action_scope.is_some())
        + usize::from(answer.final_acceptance.is_some())
        + usize::from(answer.residual_risk_acceptance.is_some())
        + usize::from(answer.cancellation.is_some())
}

pub(crate) fn answer_branch_matches_kind(
    judgment_kind: JudgmentKind,
    answer: &RecordUserJudgmentPayload,
) -> bool {
    match judgment_kind {
        JudgmentKind::ProductDecision => answer.product_decision.is_some(),
        JudgmentKind::TechnicalDecision => answer.technical_decision.is_some(),
        JudgmentKind::ScopeDecision => answer.scope_decision.is_some(),
        JudgmentKind::SensitiveApproval => answer.sensitive_action_scope.is_some(),
        JudgmentKind::FinalAcceptance => answer.final_acceptance.is_some(),
        JudgmentKind::ResidualRiskAcceptance => answer.residual_risk_acceptance.is_some(),
        JudgmentKind::Cancellation => answer.cancellation.is_some(),
    }
}

pub(crate) fn answer_outcome_agreement(
    answer: &RecordUserJudgmentPayload,
    selected_outcome: JudgmentResolutionOutcome,
) -> Result<AnswerOutcomeAgreement, serde_json::Error> {
    let answer_value = serde_json::to_value(answer)?;
    let mut claims = Vec::new();
    collect_answer_outcome_claims(&answer_value, &mut claims);
    if claims
        .iter()
        .any(|claim| matches!(claim, AnswerOutcomeClaim::Unsupported))
    {
        return Ok(AnswerOutcomeAgreement::UnsupportedClaim);
    }
    if claims
        .iter()
        .filter_map(|claim| claim.supported_outcome())
        .any(|claimed_outcome| claimed_outcome != selected_outcome)
    {
        return Ok(AnswerOutcomeAgreement::ConflictingClaim);
    }
    Ok(AnswerOutcomeAgreement::Agrees)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnswerOutcomeClaim {
    Supported(JudgmentResolutionOutcome),
    Unsupported,
}

impl AnswerOutcomeClaim {
    fn supported_outcome(self) -> Option<JudgmentResolutionOutcome> {
        match self {
            Self::Supported(outcome) => Some(outcome),
            Self::Unsupported => None,
        }
    }
}

fn collect_answer_outcome_claims(value: &Value, claims: &mut Vec<AnswerOutcomeClaim>) {
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                if let Some(outcome) = answer_claimed_outcome(key, value) {
                    claims.push(outcome);
                }
                collect_answer_outcome_claims(value, claims);
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_answer_outcome_claims(value, claims);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

fn answer_claimed_outcome(key: &str, value: &Value) -> Option<AnswerOutcomeClaim> {
    match key {
        "resolution_outcome" | "outcome" | "decision" | "acceptance" => {
            outcome_from_json_value(value)
        }
        "accepted" | "approved" => outcome_from_boolean_or_string(value),
        _ => None,
    }
}

fn outcome_from_boolean_or_string(value: &Value) -> Option<AnswerOutcomeClaim> {
    match value {
        Value::Bool(true) => Some(AnswerOutcomeClaim::Supported(
            JudgmentResolutionOutcome::Accepted,
        )),
        Value::Bool(false) => Some(AnswerOutcomeClaim::Supported(
            JudgmentResolutionOutcome::Rejected,
        )),
        Value::String(raw) => outcome_from_str(raw),
        _ => None,
    }
}

fn outcome_from_json_value(value: &Value) -> Option<AnswerOutcomeClaim> {
    match value {
        Value::String(raw) => outcome_from_str(raw),
        Value::Bool(_) => outcome_from_boolean_or_string(value),
        _ => None,
    }
}

fn outcome_from_str(raw: &str) -> Option<AnswerOutcomeClaim> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "accepted" | "accept" | "approve" | "approved" | "yes" | "true" => Some(
            AnswerOutcomeClaim::Supported(JudgmentResolutionOutcome::Accepted),
        ),
        "rejected" | "reject" | "decline" | "declined" | "deny" | "denied" | "no" | "false" => {
            Some(AnswerOutcomeClaim::Supported(
                JudgmentResolutionOutcome::Rejected,
            ))
        }
        "deferred" | "defer" => Some(AnswerOutcomeClaim::Supported(
            JudgmentResolutionOutcome::Deferred,
        )),
        "blocked" | "block" => Some(AnswerOutcomeClaim::Unsupported),
        _ => None,
    }
}
