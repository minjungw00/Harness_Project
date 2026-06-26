use volicord_types::{
    ArtifactRef, JudgmentKind, JudgmentRationale, JudgmentResolutionOutcome, StateRecordRef,
};

pub(crate) fn validate_judgment_rationale(
    rationale: &JudgmentRationale,
    judgment_kind: JudgmentKind,
    resolution_outcome: JudgmentResolutionOutcome,
) -> Result<(), JudgmentRationaleValidationError> {
    require_non_empty("rationale.summary", &rationale.summary)?;
    if let Some(selected_reason) = rationale.selected_reason.as_ref() {
        require_non_empty("rationale.selected_reason", selected_reason)?;
    }
    if requires_structured_rationale(judgment_kind, resolution_outcome) {
        rationale
            .selected_reason
            .as_ref()
            .ok_or(JudgmentRationaleValidationError::MissingSelectedReason)?;
        require_non_empty_list(
            "rationale.tradeoffs",
            &rationale.tradeoffs,
            JudgmentRationaleValidationError::MissingTradeoffs,
        )?;
        require_non_empty_list(
            "rationale.review_triggers",
            &rationale.review_triggers,
            JudgmentRationaleValidationError::MissingReviewTriggers,
        )?;
    }
    validate_string_list(
        "rationale.considered_alternatives",
        &rationale.considered_alternatives,
    )?;
    validate_string_list(
        "rationale.rejected_alternatives",
        &rationale.rejected_alternatives,
    )?;
    validate_string_list("rationale.assumptions", &rationale.assumptions)?;
    validate_string_list("rationale.tradeoffs", &rationale.tradeoffs)?;
    validate_string_list("rationale.uncertainties", &rationale.uncertainties)?;
    validate_string_list("rationale.review_triggers", &rationale.review_triggers)?;
    validate_refs(&rationale.related_refs)?;
    validate_artifact_refs(&rationale.artifact_refs)?;
    Ok(())
}

fn requires_structured_rationale(
    judgment_kind: JudgmentKind,
    resolution_outcome: JudgmentResolutionOutcome,
) -> bool {
    resolution_outcome == JudgmentResolutionOutcome::Accepted
        && matches!(
            judgment_kind,
            JudgmentKind::ProductDecision
                | JudgmentKind::TechnicalDecision
                | JudgmentKind::ScopeDecision
                | JudgmentKind::SensitiveApproval
                | JudgmentKind::FinalAcceptance
                | JudgmentKind::ResidualRiskAcceptance
                | JudgmentKind::Cancellation
        )
}

fn require_non_empty(
    field: &'static str,
    value: &str,
) -> Result<(), JudgmentRationaleValidationError> {
    if value.trim().is_empty() {
        Err(JudgmentRationaleValidationError::EmptyField(field))
    } else {
        Ok(())
    }
}

fn require_non_empty_list(
    field: &'static str,
    values: &[String],
    missing: JudgmentRationaleValidationError,
) -> Result<(), JudgmentRationaleValidationError> {
    if values.is_empty() {
        return Err(missing);
    }
    validate_string_list(field, values)
}

fn validate_string_list(
    field: &'static str,
    values: &[String],
) -> Result<(), JudgmentRationaleValidationError> {
    for value in values {
        require_non_empty(field, value)?;
    }
    Ok(())
}

fn validate_refs(refs: &[StateRecordRef]) -> Result<(), JudgmentRationaleValidationError> {
    for record_ref in refs {
        if record_ref.record_id.as_str().trim().is_empty() {
            return Err(JudgmentRationaleValidationError::EmptyField(
                "rationale.related_refs.record_id",
            ));
        }
    }
    Ok(())
}

fn validate_artifact_refs(refs: &[ArtifactRef]) -> Result<(), JudgmentRationaleValidationError> {
    for artifact_ref in refs {
        if artifact_ref.artifact_id.as_str().trim().is_empty() {
            return Err(JudgmentRationaleValidationError::EmptyField(
                "rationale.artifact_refs.artifact_id",
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum JudgmentRationaleValidationError {
    EmptyField(&'static str),
    MissingSelectedReason,
    MissingTradeoffs,
    MissingReviewTriggers,
}

impl JudgmentRationaleValidationError {
    pub(crate) const fn field(&self) -> &'static str {
        match self {
            Self::EmptyField(field) => field,
            Self::MissingSelectedReason => "rationale.selected_reason",
            Self::MissingTradeoffs => "rationale.tradeoffs",
            Self::MissingReviewTriggers => "rationale.review_triggers",
        }
    }

    pub(crate) fn message(&self) -> &'static str {
        match self {
            Self::EmptyField("rationale.summary") => "rationale.summary must not be empty",
            Self::EmptyField(field) if field.starts_with("rationale.") => {
                "rationale text entries must not be empty"
            }
            Self::EmptyField(_) => "rationale field must not be empty",
            Self::MissingSelectedReason => {
                "accepted high-impact judgments require rationale.selected_reason"
            }
            Self::MissingTradeoffs => "accepted high-impact judgments require rationale.tradeoffs",
            Self::MissingReviewTriggers => {
                "accepted high-impact judgments require rationale.review_triggers"
            }
        }
    }
}
