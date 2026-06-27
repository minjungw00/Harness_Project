use std::{fmt, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserCommandError {
    Usage(String),
}

impl fmt::Display for UserCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for UserCommandError {}

pub fn user_usage() -> String {
    "volicord user commands are not available in the Agent Connection admin CLI\n".to_owned()
}

pub fn run_user_command<F>(
    _args: &[String],
    _env_var: F,
    _current_dir: &Path,
) -> Result<String, UserCommandError>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    Err(UserCommandError::Usage(user_usage()))
}
