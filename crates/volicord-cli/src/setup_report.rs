use std::path::Path;

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SetupStatus {
    Complete,
    ActionRequired,
    Failed,
}

impl SetupStatus {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::ActionRequired => "action_required",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct SetupSectionStatus {
    pub(crate) status: SetupStatus,
    pub(crate) summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) details: Option<Value>,
}

impl SetupSectionStatus {
    pub(crate) fn complete(summary: impl Into<String>, details: Value) -> Self {
        Self {
            status: SetupStatus::Complete,
            summary: summary.into(),
            details: Some(details),
        }
    }

    pub(crate) fn failed(summary: impl Into<String>, details: Value) -> Self {
        Self {
            status: SetupStatus::Failed,
            summary: summary.into(),
            details: Some(details),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct CommandAvailability {
    pub(crate) id: String,
    pub(crate) command_name: String,
    pub(crate) discovered: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) discovered_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) discovery_source: Option<String>,
    pub(crate) available_on_path: bool,
    pub(crate) path_matches_discovered: bool,
    pub(crate) discovered_directory_on_path: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) path_match: Option<String>,
}

impl CommandAvailability {
    pub(crate) fn selected_path_ready(&self) -> bool {
        self.available_on_path && self.path_matches_discovered
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SetupActionKind {
    RuntimeHomeReady,
    InstallationProfileSaved,
    CommandAvailability,
    CommandLinks,
    PathUpdate,
    SelectMcpCommand,
    ShellStartup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SetupActionRequirement {
    Required,
    Optional,
    Performed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct SetupAction {
    pub(crate) id: String,
    pub(crate) kind: SetupActionKind,
    pub(crate) requirement: SetupActionRequirement,
    pub(crate) instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) path: Option<String>,
}

impl SetupAction {
    pub(crate) fn required(
        id: impl Into<String>,
        kind: SetupActionKind,
        instruction: impl Into<String>,
    ) -> Self {
        Self::new(id, kind, SetupActionRequirement::Required, instruction)
    }

    pub(crate) fn optional(
        id: impl Into<String>,
        kind: SetupActionKind,
        instruction: impl Into<String>,
    ) -> Self {
        Self::new(id, kind, SetupActionRequirement::Optional, instruction)
    }

    pub(crate) fn performed(
        id: impl Into<String>,
        kind: SetupActionKind,
        instruction: impl Into<String>,
    ) -> Self {
        Self::new(id, kind, SetupActionRequirement::Performed, instruction)
    }

    pub(crate) fn with_command(mut self, command: impl Into<String>) -> Self {
        self.command = Some(command.into());
        self
    }

    pub(crate) fn with_path(mut self, path: &Path) -> Self {
        self.path = Some(path.display().to_string());
        self
    }

    fn new(
        id: impl Into<String>,
        kind: SetupActionKind,
        requirement: SetupActionRequirement,
        instruction: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            requirement,
            instruction: instruction.into(),
            command: None,
            path: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct SetupReport {
    pub(crate) status: SetupStatus,
    pub(crate) runtime_home: SetupSectionStatus,
    pub(crate) installation_profile: SetupSectionStatus,
    pub(crate) commands: Vec<CommandAvailability>,
    pub(crate) actions_required: Vec<SetupAction>,
    pub(crate) actions_optional: Vec<SetupAction>,
    pub(crate) actions_performed: Vec<SetupAction>,
}

impl SetupReport {
    pub(crate) fn new(
        runtime_home: SetupSectionStatus,
        installation_profile: SetupSectionStatus,
        commands: Vec<CommandAvailability>,
        actions_required: Vec<SetupAction>,
        actions_optional: Vec<SetupAction>,
        actions_performed: Vec<SetupAction>,
    ) -> Self {
        let status = if runtime_home.status == SetupStatus::Failed
            || installation_profile.status == SetupStatus::Failed
            || commands.iter().any(|command| !command.discovered)
        {
            SetupStatus::Failed
        } else if !actions_required.is_empty()
            || commands
                .iter()
                .any(|command| !command.selected_path_ready())
        {
            SetupStatus::ActionRequired
        } else {
            SetupStatus::Complete
        };
        Self {
            status,
            runtime_home,
            installation_profile,
            commands,
            actions_required,
            actions_optional,
            actions_performed,
        }
    }
}
