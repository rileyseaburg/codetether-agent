use crate::paths::normalize_local_path;
use anyhow::Result;
use std::path::{Path, PathBuf};

pub(crate) fn relativize_files(workspace_root: &Path, files: &[PathBuf]) -> Result<Vec<String>> {
    let mut relative = Vec::with_capacity(files.len());
    for file in files {
        let normalized = normalize_local_path(file)?;
        let path = normalized.strip_prefix(workspace_root).map_err(|_| {
            anyhow::anyhow!(
                "Attached file '{}' is not inside workspace '{}'.",
                normalized.display(),
                workspace_root.display()
            )
        })?;
        relative.push(path.to_string_lossy().to_string());
    }
    Ok(relative)
}
