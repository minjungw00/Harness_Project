use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub(crate) const START_MARKER: &str = "# >>> volicord setup >>>";
pub(crate) const END_MARKER: &str = "# <<< volicord setup <<<";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ManagedBlockWrite {
    Created(PathBuf),
    Updated(PathBuf),
    Unchanged(PathBuf),
}

pub(crate) fn path_export_block(path_expr: &str) -> String {
    format!("{START_MARKER}\nexport PATH=\"{path_expr}:$PATH\"\n{END_MARKER}\n")
}

pub(crate) fn apply_managed_block(existing: &str, block: &str) -> String {
    let block = ensure_trailing_newline(block);
    if let Some(start) = existing.find(START_MARKER) {
        if let Some(end_from_start) = existing[start..].find(END_MARKER) {
            let mut end = start + end_from_start + END_MARKER.len();
            if existing[end..].starts_with("\r\n") {
                end += 2;
            } else if existing[end..].starts_with('\n') {
                end += 1;
            }
            let mut updated = String::with_capacity(existing.len() - (end - start) + block.len());
            updated.push_str(&existing[..start]);
            updated.push_str(&block);
            updated.push_str(&existing[end..]);
            return updated;
        }
    }

    let mut updated = existing.to_owned();
    if !updated.is_empty() && !updated.ends_with('\n') {
        updated.push('\n');
    }
    if !updated.is_empty() && !updated.ends_with("\n\n") {
        updated.push('\n');
    }
    updated.push_str(&block);
    updated
}

pub(crate) fn write_managed_block(target: &Path, block: &str) -> io::Result<ManagedBlockWrite> {
    let existing = match fs::read_to_string(target) {
        Ok(text) => Some(text),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(error),
    };
    let updated = apply_managed_block(existing.as_deref().unwrap_or(""), block);
    if existing.as_deref() == Some(updated.as_str()) {
        return Ok(ManagedBlockWrite::Unchanged(target.to_path_buf()));
    }
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(target, updated)?;
    if existing.is_some() {
        Ok(ManagedBlockWrite::Updated(target.to_path_buf()))
    } else {
        Ok(ManagedBlockWrite::Created(target.to_path_buf()))
    }
}

fn ensure_trailing_newline(text: &str) -> String {
    if text.ends_with('\n') {
        text.to_owned()
    } else {
        format!("{text}\n")
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use volicord_test_support::TempRuntimeHome;

    use super::*;

    #[test]
    fn managed_block_appends_without_overwriting_unmanaged_path_lines() {
        let existing = "export PATH=\"$HOME/bin:$PATH\"\n";
        let block = path_export_block("$HOME/.local/bin");

        let updated = apply_managed_block(existing, &block);

        assert!(updated.contains(existing));
        assert!(updated.contains(START_MARKER));
        assert!(updated.contains("export PATH=\"$HOME/.local/bin:$PATH\""));
    }

    #[test]
    fn managed_block_replaces_existing_block_without_duplication() {
        let old = path_export_block("$HOME/bin");
        let block = path_export_block("$HOME/.local/bin");
        let existing = format!("before\n\n{old}\nafter\n");

        let updated = apply_managed_block(&existing, &block);

        assert_eq!(updated.matches(START_MARKER).count(), 1);
        assert!(!updated.contains("export PATH=\"$HOME/bin:$PATH\""));
        assert!(updated.contains("export PATH=\"$HOME/.local/bin:$PATH\""));
        assert!(updated.starts_with("before\n\n"));
        assert!(updated.ends_with("after\n"));
    }

    #[test]
    fn managed_block_write_is_idempotent() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = TempRuntimeHome::new("managed-block-idempotent")?;
        let target = fixture.path().join("home/.zshrc");
        let block = path_export_block("$HOME/.local/bin");

        assert_eq!(
            write_managed_block(&target, &block)?,
            ManagedBlockWrite::Created(target.clone())
        );
        assert_eq!(
            write_managed_block(&target, &block)?,
            ManagedBlockWrite::Unchanged(target.clone())
        );
        let text = fs::read_to_string(target)?;
        assert_eq!(text.matches(START_MARKER).count(), 1);
        Ok(())
    }
}
