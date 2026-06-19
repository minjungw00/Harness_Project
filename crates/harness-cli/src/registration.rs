use std::{error::Error, fmt};

use harness_types::{
    AccessClass, SurfaceInteractionRole, BASELINE_WORKFLOW_ACCESS_CLASSES,
    VERIFICATION_BASIS_LOCAL_ADMIN_REGISTRATION,
};
use serde_json::{json, Map, Value};

/// Default local surface registration kind for the low-level CLI command.
pub const DEFAULT_SURFACE_KIND: &str = "cli";

/// Default local surface access class for the low-level CLI command.
pub const DEFAULT_ACCESS_CLASS: AccessClass = AccessClass::ReadStatus;

/// Supported administrative registration profile.
pub const BASELINE_WORKFLOW_PROFILE: &str = "baseline-workflow";

/// Metadata used by existing low-level administrative registration commands.
pub const ADMIN_METADATA_JSON: &str = r#"{"created_by":"harness_cli_admin"}"#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationMetadataError {
    Usage(String),
    Runtime(String),
}

impl RegistrationMetadataError {
    pub fn usage(message: impl Into<String>) -> Self {
        Self::Usage(message.into())
    }

    fn runtime(message: impl Into<String>) -> Self {
        Self::Runtime(message.into())
    }
}

impl fmt::Display for RegistrationMetadataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) | Self::Runtime(message) => formatter.write_str(message),
        }
    }
}

impl Error for RegistrationMetadataError {}

/// Builds deterministic capability-profile JSON for a surface registration.
pub fn capability_profile_json(
    access_classes: &[AccessClass],
    provided: Option<&str>,
) -> Result<String, RegistrationMetadataError> {
    let mut value = match provided {
        Some(text) => serde_json::from_str::<Value>(text).map_err(|error| {
            RegistrationMetadataError::usage(format!("invalid --capability-profile JSON: {error}"))
        })?,
        None => Value::Object(Map::new()),
    };

    let Some(object) = value.as_object_mut() else {
        return Err(RegistrationMetadataError::usage(
            "--capability-profile must be a JSON object",
        ));
    };
    let primary = primary_access_class(access_classes)?;
    object.insert("access_class".to_owned(), json!(primary.as_str()));
    object
        .entry("supported_access_classes".to_owned())
        .or_insert_with(|| json!(access_class_strings(access_classes)));

    serde_json::to_string(&value).map_err(|error| {
        RegistrationMetadataError::runtime(format!("failed to encode capability profile: {error}"))
    })
}

/// Builds deterministic local-access metadata JSON for a surface registration.
pub fn local_access_json(
    access_classes: &[AccessClass],
) -> Result<String, RegistrationMetadataError> {
    let primary = primary_access_class(access_classes)?;
    serde_json::to_string(&json!({
        "access_class": primary.as_str(),
        "authorized_access_classes": access_class_strings(access_classes),
        "verification_basis": VERIFICATION_BASIS_LOCAL_ADMIN_REGISTRATION
    }))
    .map_err(|error| {
        RegistrationMetadataError::runtime(format!(
            "failed to encode local access metadata: {error}"
        ))
    })
}

/// Parses registered local-access metadata into a de-duplicated access set.
///
/// The preferred field is `authorized_access_classes`; the single-value
/// `access_class` field is retained as a backward-compatible fallback.
pub fn normalized_access_classes_from_local_access(
    text: &str,
) -> Result<Vec<AccessClass>, RegistrationMetadataError> {
    let value = serde_json::from_str::<Value>(text).map_err(|error| {
        RegistrationMetadataError::usage(format!("invalid local access JSON: {error}"))
    })?;
    let Some(object) = value.as_object() else {
        return Err(RegistrationMetadataError::usage(
            "local access metadata must be a JSON object",
        ));
    };

    let mut access_classes = Vec::new();
    if let Some(values) = object.get("authorized_access_classes") {
        let Some(values) = values.as_array() else {
            return Err(RegistrationMetadataError::usage(
                "authorized_access_classes must be an array",
            ));
        };
        for value in values {
            let Some(raw) = value.as_str() else {
                return Err(RegistrationMetadataError::usage(
                    "authorized_access_classes entries must be strings",
                ));
            };
            push_access_class(&mut access_classes, parse_access_class(raw)?);
        }
    }

    if access_classes.is_empty() {
        let Some(raw) = object.get("access_class").and_then(Value::as_str) else {
            return Err(RegistrationMetadataError::usage(
                "local access metadata must include an access class",
            ));
        };
        push_access_class(&mut access_classes, parse_access_class(raw)?);
    }

    Ok(access_classes)
}

/// Returns the display value used by the existing low-level list/register output.
pub fn access_class_from_local_access(text: &str) -> Option<String> {
    let value = serde_json::from_str::<Value>(text).ok()?;
    let access_classes = value
        .get("authorized_access_classes")
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(Value::as_str).collect::<Vec<_>>())
        .unwrap_or_default();
    if !access_classes.is_empty() {
        return Some(access_classes.join(","));
    }
    value
        .get("access_class")
        .and_then(Value::as_str)
        .map(str::to_owned)
}

/// Validates role-specific local-access constraints.
pub fn validate_role_access_classes(
    role: SurfaceInteractionRole,
    access_classes: &[AccessClass],
) -> Result<(), RegistrationMetadataError> {
    if role != SurfaceInteractionRole::UserInteraction {
        return Ok(());
    }
    if !access_classes.contains(&AccessClass::CoreMutation) {
        return Err(RegistrationMetadataError::usage(
            "user_interaction surfaces require core_mutation access",
        ));
    }
    if access_classes.iter().any(|access_class| {
        !matches!(
            access_class,
            AccessClass::ReadStatus | AccessClass::CoreMutation
        )
    }) {
        return Err(RegistrationMetadataError::usage(
            "user_interaction surfaces may grant only read_status and core_mutation access",
        ));
    }
    Ok(())
}

pub fn parse_access_class(value: &str) -> Result<AccessClass, RegistrationMetadataError> {
    serde_json::from_value(Value::String(value.to_owned()))
        .map_err(|_| RegistrationMetadataError::usage(format!("unknown access class: {value}")))
}

pub fn push_access_classes<const N: usize>(
    target: &mut Vec<AccessClass>,
    values: [AccessClass; N],
) {
    for value in values {
        push_access_class(target, value);
    }
}

pub fn push_access_class(target: &mut Vec<AccessClass>, value: AccessClass) {
    if !target.contains(&value) {
        target.push(value);
    }
}

pub fn primary_access_class(
    access_classes: &[AccessClass],
) -> Result<AccessClass, RegistrationMetadataError> {
    access_classes.first().copied().ok_or_else(|| {
        RegistrationMetadataError::usage("surface registration requires at least one access class")
    })
}

pub fn access_class_strings(access_classes: &[AccessClass]) -> Vec<&'static str> {
    access_classes.iter().map(|value| value.as_str()).collect()
}

pub fn access_classes_match(actual: &[AccessClass], expected: &[AccessClass]) -> bool {
    actual.len() == expected.len() && expected.iter().all(|value| actual.contains(value))
}

pub fn baseline_workflow_access_classes() -> Vec<AccessClass> {
    BASELINE_WORKFLOW_ACCESS_CLASSES.to_vec()
}

pub fn user_interaction_access_classes() -> Vec<AccessClass> {
    vec![AccessClass::ReadStatus, AccessClass::CoreMutation]
}

pub fn parse_json_object(field: &str, text: &str) -> Result<Value, RegistrationMetadataError> {
    let value = serde_json::from_str::<Value>(text).map_err(|error| {
        RegistrationMetadataError::usage(format!("{field} must be JSON object text: {error}"))
    })?;
    if value.is_object() {
        Ok(value)
    } else {
        Err(RegistrationMetadataError::usage(format!(
            "{field} must be a JSON object"
        )))
    }
}
