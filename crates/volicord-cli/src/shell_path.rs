use std::{
    env,
    ffi::{OsStr, OsString},
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

pub(crate) const PATH_ENV: &str = "PATH";
static WRITE_PROBE_COUNTER: AtomicU64 = AtomicU64::new(0);

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

pub(crate) fn path_directory_is_verified_writable(path: &Path) -> bool {
    verify_directory_writable(path).is_ok()
}

pub(crate) fn verify_directory_writable(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    if !metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is not a directory", path.display()),
        ));
    }

    let mut last_collision = None;
    for _ in 0..16 {
        let probe_path = path.join(unique_write_probe_name());
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&probe_path)
        {
            Ok(mut file) => {
                let write_result = file
                    .write_all(b"volicord setup write probe\n")
                    .and_then(|()| file.flush());
                drop(file);
                let cleanup_result = fs::remove_file(&probe_path);
                return match (write_result, cleanup_result) {
                    (Ok(()), Ok(())) => Ok(()),
                    (Err(error), _) => {
                        let _ = fs::remove_file(&probe_path);
                        Err(error)
                    }
                    (Ok(()), Err(error)) => Err(io::Error::new(
                        error.kind(),
                        format!(
                            "created probe file but could not remove {}: {error}",
                            probe_path.display()
                        ),
                    )),
                };
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                last_collision = Some(error);
            }
            Err(error) => return Err(error),
        }
    }

    Err(last_collision.unwrap_or_else(|| {
        io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!(
                "could not create a unique Volicord setup probe file in {}",
                path.display()
            ),
        )
    }))
}

fn unique_write_probe_name() -> String {
    let counter = WRITE_PROBE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    format!(
        ".volicord-setup-write-probe-{}-{nanos}-{counter}.tmp",
        std::process::id()
    )
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

pub(crate) fn candidate_setup_link_dirs<F>(env_var: &F) -> Vec<PathBuf>
where
    F: Fn(&str) -> Option<OsString>,
{
    let mut dirs = Vec::new();
    if let Some(path_env) = env_var(PATH_ENV) {
        for dir in env::split_paths(&path_env) {
            if path_directory_is_verified_writable(&dir) {
                push_unique_path(&mut dirs, dir);
            }
        }
    }
    for dir in candidate_user_bin_dirs(env_var) {
        if path_directory_is_verified_writable(&dir) {
            push_unique_path(&mut dirs, dir);
        }
    }
    dirs
}

fn push_unique_path(paths: &mut Vec<PathBuf>, path: PathBuf) {
    if !paths
        .iter()
        .any(|existing| paths_equivalent(existing, &path))
    {
        paths.push(path);
    }
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

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, fs, io::Write, path::Path};

    use volicord_test_support::TempRuntimeHome;

    use super::*;

    #[test]
    fn detect_command_on_path_finds_executable_command() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("shell-path-detect-command")?;
        let path_dir = fixture.path().join("path-bin");
        let command = write_executable(&path_dir, &volicord_binary_name())?;
        let path_env = env::join_paths([path_dir.as_path()])?;

        let detected = detect_command_on_path(&volicord_binary_name(), Some(&path_env))
            .expect("command should be found on PATH");

        assert_eq!(detected, fs::canonicalize(command)?);
        Ok(())
    }

    #[test]
    fn candidate_setup_link_dirs_prefers_writable_path_dirs(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("shell-path-candidates")?;
        let path_dir = fixture.path().join("path-bin");
        let home = fixture.path().join("home");
        let local_bin = home.join(".local").join("bin");
        fs::create_dir_all(&path_dir)?;
        fs::create_dir_all(&local_bin)?;
        let env = BTreeMap::from([
            (PATH_ENV.to_owned(), env::join_paths([path_dir.as_path()])?),
            ("HOME".to_owned(), home.clone().into_os_string()),
        ]);

        let dirs = candidate_setup_link_dirs(&|name| env.get(name).cloned());

        assert_eq!(dirs.first(), Some(&path_dir));
        assert!(dirs.contains(&local_bin));
        assert_eq!(fs::read_dir(&path_dir)?.count(), 0);
        assert_eq!(fs::read_dir(&local_bin)?.count(), 0);
        Ok(())
    }

    #[test]
    fn candidate_setup_link_dirs_uses_user_bin_when_path_has_no_writable_directory(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("shell-path-user-bin")?;
        let path_file = fixture.path().join("path-file");
        let home = fixture.path().join("home");
        let local_bin = home.join(".local").join("bin");
        let home_bin = home.join("bin");
        fs::write(&path_file, "not a directory")?;
        fs::create_dir_all(&local_bin)?;
        fs::create_dir_all(&home_bin)?;
        let env = BTreeMap::from([
            (PATH_ENV.to_owned(), env::join_paths([path_file.as_path()])?),
            ("HOME".to_owned(), home.clone().into_os_string()),
        ]);

        let dirs = candidate_setup_link_dirs(&|name| env.get(name).cloned());

        assert_eq!(dirs.first(), Some(&local_bin));
        assert!(dirs.contains(&home_bin));
        assert!(!dirs.contains(&path_file));
        assert_eq!(fs::read_dir(&local_bin)?.count(), 0);
        assert_eq!(fs::read_dir(&home_bin)?.count(), 0);
        Ok(())
    }

    #[test]
    fn candidate_setup_link_dirs_omits_missing_user_bin_dirs(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("shell-path-missing-user-bin")?;
        let home = fixture.path().join("home");
        fs::create_dir_all(&home)?;
        let env = BTreeMap::from([("HOME".to_owned(), home.clone().into_os_string())]);

        let dirs = candidate_setup_link_dirs(&|name| env.get(name).cloned());

        assert!(dirs.is_empty());
        assert!(!home.join(".local").exists());
        assert!(!home.join("bin").exists());
        Ok(())
    }

    #[test]
    fn writable_directory_probe_cleans_up_probe_file() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("shell-path-write-probe")?;
        let path_dir = fixture.path().join("path-bin");
        fs::create_dir_all(&path_dir)?;

        verify_directory_writable(&path_dir)?;

        assert_eq!(fs::read_dir(&path_dir)?.count(), 0);
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn candidate_setup_link_dirs_skips_path_dir_when_write_probe_fails(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::os::unix::fs::PermissionsExt;

        let fixture = TempRuntimeHome::new("shell-path-unwritable-path-dir")?;
        let path_dir = fixture.path().join("path-bin");
        let home = fixture.path().join("home");
        let local_bin = home.join(".local").join("bin");
        fs::create_dir_all(&path_dir)?;
        fs::create_dir_all(&local_bin)?;
        let mut permissions = fs::metadata(&path_dir)?.permissions();
        permissions.set_mode(0o555);
        fs::set_permissions(&path_dir, permissions)?;

        if path_directory_is_verified_writable(&path_dir) {
            restore_writable_dir(&path_dir)?;
            return Ok(());
        }

        let env = BTreeMap::from([
            (PATH_ENV.to_owned(), env::join_paths([path_dir.as_path()])?),
            ("HOME".to_owned(), home.clone().into_os_string()),
        ]);
        let dirs = candidate_setup_link_dirs(&|name| env.get(name).cloned());

        restore_writable_dir(&path_dir)?;
        assert_eq!(dirs.first(), Some(&local_bin));
        assert!(!dirs.contains(&path_dir));
        Ok(())
    }

    fn write_executable(dir: &Path, name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        fs::create_dir_all(dir)?;
        let path = dir.join(name);
        let mut file = fs::File::create(&path)?;
        writeln!(file, "#!/bin/sh")?;
        make_executable(&path)?;
        Ok(path)
    }

    #[cfg(unix)]
    fn make_executable(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
        Ok(())
    }

    #[cfg(not(unix))]
    fn make_executable(_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    #[cfg(unix)]
    fn restore_writable_dir(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
        Ok(())
    }
}
