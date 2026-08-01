//! File-path context for non-interactive CLI runs.

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

#[cfg(test)]
#[path = "files_tests.rs"]
mod tests;

pub(super) fn with_attachments(message: &str, files: &[PathBuf]) -> Result<String> {
    if files.is_empty() {
        return Ok(message.to_string());
    }
    let paths = files
        .iter()
        .map(|path| resolve(path))
        .collect::<Result<Vec<_>>>()?;
    let paths = serde_json::to_string(&paths)?;
    Ok(format!(
        "{message}\n\nAttached files (use these exact paths before searching for alternatives): {paths}"
    ))
}

fn resolve(path: &Path) -> Result<String> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("Attached file does not exist: {}", path.display()))?;
    if !canonical.is_file() {
        bail!("Attachment is not a regular file: {}", canonical.display());
    }
    Ok(canonical.to_string_lossy().into_owned())
}
