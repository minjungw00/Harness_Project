use std::{
    env,
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};

pub(crate) const PATH_ENV: &str = "PATH";

pub(crate) fn detect_command_on_path(
    command_name: &str,
    path_env: Option<&OsStr>,
) -> Option<PathBuf> {
    path_env
        .map(env::split_paths)?
        .map(|dir| dir.join(command_name))
        .find(|candidate| is_executable_file(candidate))
        .map(|candidate| fs::canonicalize(&candidate).unwrap_or(candidate))
}

pub(crate) fn path_directory_is_on_path(path_env: Option<&OsStr>, dir: &Path) -> bool {
    path_env
        .map(env::split_paths)
        .into_iter()
        .flatten()
        .any(|candidate| paths_equivalent(&candidate, dir))
}

pub(crate) fn path_directory_is_writable(path: &Path) -> bool {
    let probe = if path.exists() {
        path
    } else {
        match path.parent() {
            Some(parent) => parent,
            None => return false,
        }
    };
    fs::metadata(probe)
        .map(|metadata| metadata.is_dir() && !metadata.permissions().readonly())
        .unwrap_or(false)
}

pub(crate) fn candidate_user_bin_dirs<F>(env_var: &F) -> Vec<PathBuf>
where
    F: Fn(&str) -> Option<OsString>,
{
    let mut dirs = Vec::new();
    if let Some(home) = env_var("HOME").filter(|value| !value.is_empty()) {
        let home = PathBuf::from(home);
        dirs.push(home.join(".local").join("bin"));
        dirs.push(home.join("bin"));
    }
    #[cfg(windows)]
    if let Some(local_app_data) = env_var("LOCALAPPDATA").filter(|value| !value.is_empty()) {
        dirs.push(PathBuf::from(local_app_data).join("Volicord").join("bin"));
    }
    dirs
}

pub(crate) fn paths_equivalent(left: &Path, right: &Path) -> bool {
    if left == right {
        return true;
    }
    match (fs::canonicalize(left), fs::canonicalize(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

pub(crate) fn is_executable_file(path: &Path) -> bool {
    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }
    is_executable_metadata(&metadata)
}

#[cfg(unix)]
fn is_executable_metadata(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;

    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable_metadata(_metadata: &fs::Metadata) -> bool {
    true
}

pub(crate) fn volicord_binary_name() -> String {
    format!("volicord{}", env::consts::EXE_SUFFIX)
}

pub(crate) fn mcp_binary_name() -> String {
    format!("volicord-mcp{}", env::consts::EXE_SUFFIX)
}
