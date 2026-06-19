use std::{
    error::Error,
    ffi::OsString,
    fmt,
    path::{Path, PathBuf},
};

const HARNESS_HOME: &str = "HARNESS_HOME";
const HOME: &str = "HOME";
const USERPROFILE: &str = "USERPROFILE";
const HOMEDRIVE: &str = "HOMEDRIVE";
const HOMEPATH: &str = "HOMEPATH";

/// Errors returned while selecting a Runtime Home path from process inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeHomeResolutionError {
    EmptyHarnessHome,
    MissingUserHome,
}

impl fmt::Display for RuntimeHomeResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyHarnessHome => formatter.write_str("HARNESS_HOME must not be empty"),
            Self::MissingUserHome => formatter
                .write_str("could not determine a default home directory; set HARNESS_HOME"),
        }
    }
}

impl Error for RuntimeHomeResolutionError {}

/// Resolves the Harness Runtime Home path from environment values and a cwd.
///
/// This function performs path selection only. It does not canonicalize the
/// result, create directories, or require the selected path to exist.
pub fn resolve_runtime_home<F>(
    env_var: F,
    current_dir: impl AsRef<Path>,
) -> Result<PathBuf, RuntimeHomeResolutionError>
where
    F: Fn(&str) -> Option<OsString>,
{
    let current_dir = current_dir.as_ref();
    if let Some(value) = env_var(HARNESS_HOME) {
        if value.is_empty() {
            return Err(RuntimeHomeResolutionError::EmptyHarnessHome);
        }
        return Ok(absolute_path(current_dir, PathBuf::from(value)));
    }

    let home = default_user_home(env_var).ok_or(RuntimeHomeResolutionError::MissingUserHome)?;
    Ok(absolute_path(current_dir, home).join(".harness"))
}

fn default_user_home<F>(env_var: F) -> Option<PathBuf>
where
    F: Fn(&str) -> Option<OsString>,
{
    non_empty_env(&env_var, HOME)
        .map(PathBuf::from)
        .or_else(|| non_empty_env(&env_var, USERPROFILE).map(PathBuf::from))
        .or_else(|| {
            let drive = non_empty_env(&env_var, HOMEDRIVE)?;
            let path = non_empty_env(&env_var, HOMEPATH)?;
            let mut home = PathBuf::from(drive);
            home.push(path);
            Some(home)
        })
}

fn non_empty_env<F>(env_var: &F, name: &str) -> Option<OsString>
where
    F: Fn(&str) -> Option<OsString>,
{
    env_var(name).filter(|value| !value.is_empty())
}

fn absolute_path(current_dir: &Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        current_dir.join(path)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        ffi::OsString,
        path::{Path, PathBuf},
    };

    use super::{resolve_runtime_home, RuntimeHomeResolutionError};

    fn cwd() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn resolve(entries: &[(&str, OsString)]) -> Result<PathBuf, RuntimeHomeResolutionError> {
        resolve_runtime_home(
            |name| {
                entries
                    .iter()
                    .find(|(key, _)| *key == name)
                    .map(|(_, value)| value.clone())
            },
            cwd(),
        )
    }

    #[test]
    fn absolute_harness_home_is_used_as_supplied() {
        let path = cwd().join("runtime-home-absolute");

        let resolved = resolve(&[("HARNESS_HOME", path.clone().into_os_string())])
            .expect("absolute HARNESS_HOME should resolve");

        assert_eq!(resolved, path);
    }

    #[test]
    fn relative_harness_home_is_resolved_against_current_dir() {
        let resolved = resolve(&[("HARNESS_HOME", OsString::from("runtime-home-relative"))])
            .expect("relative HARNESS_HOME should resolve");

        assert_eq!(resolved, cwd().join("runtime-home-relative"));
    }

    #[test]
    fn empty_harness_home_is_an_error() {
        let error = resolve(&[("HARNESS_HOME", OsString::new())])
            .expect_err("empty HARNESS_HOME should fail");

        assert_eq!(error, RuntimeHomeResolutionError::EmptyHarnessHome);
        assert!(error.to_string().contains("HARNESS_HOME"));
    }

    #[test]
    fn home_fallback_appends_harness() {
        let home = cwd().join("home-fallback");

        let resolved =
            resolve(&[("HOME", home.clone().into_os_string())]).expect("HOME should resolve");

        assert_eq!(resolved, home.join(".harness"));
    }

    #[test]
    fn userprofile_fallback_is_used_after_missing_home() {
        let home = cwd().join("userprofile-fallback");

        let resolved = resolve(&[("USERPROFILE", home.clone().into_os_string())])
            .expect("USERPROFILE should resolve");

        assert_eq!(resolved, home.join(".harness"));
    }

    #[test]
    fn homedrive_and_homepath_fallback_are_combined() {
        let drive = cwd().join("drive-fallback");

        let resolved = resolve(&[
            ("HOMEDRIVE", drive.clone().into_os_string()),
            ("HOMEPATH", OsString::from("homepath")),
        ])
        .expect("HOMEDRIVE and HOMEPATH should resolve");

        assert_eq!(resolved, drive.join("homepath").join(".harness"));
    }

    #[test]
    fn empty_fallback_values_are_skipped() {
        let userprofile = cwd().join("fallback-after-empty-home");

        let resolved = resolve(&[
            ("HOME", OsString::new()),
            ("USERPROFILE", userprofile.clone().into_os_string()),
            ("HOMEDRIVE", cwd().join("unused-drive").into_os_string()),
            ("HOMEPATH", OsString::from("unused-path")),
        ])
        .expect("non-empty USERPROFILE should resolve after empty HOME");

        assert_eq!(resolved, userprofile.join(".harness"));
    }

    #[test]
    fn relative_fallback_home_is_made_absolute() {
        let resolved = resolve(&[("HOME", OsString::from("relative-home"))])
            .expect("relative HOME should resolve");

        assert_eq!(resolved, cwd().join("relative-home").join(".harness"));
        assert!(resolved.is_absolute());
    }

    #[test]
    fn no_available_home_source_is_an_error() {
        let error = resolve(&[]).expect_err("missing home sources should fail");

        assert_eq!(error, RuntimeHomeResolutionError::MissingUserHome);
        assert!(error.to_string().contains("set HARNESS_HOME"));
    }

    #[test]
    fn selected_runtime_home_is_not_canonicalized_or_required_to_exist() {
        let resolved = resolve(&[(
            "HARNESS_HOME",
            OsString::from("missing-runtime-home/../still-missing"),
        )])
        .expect("nonexistent relative HARNESS_HOME should resolve");

        assert_eq!(
            resolved,
            cwd().join(Path::new("missing-runtime-home/../still-missing"))
        );
    }

    #[cfg(unix)]
    #[test]
    fn non_utf8_path_values_are_supported_on_unix() {
        use std::os::unix::ffi::OsStringExt;

        let path = PathBuf::from(OsString::from_vec(b"/tmp/harness-\xFF-home".to_vec()));

        let resolved = resolve(&[("HARNESS_HOME", path.clone().into_os_string())])
            .expect("non-UTF-8 HARNESS_HOME should resolve");

        assert_eq!(resolved, path);
    }
}
