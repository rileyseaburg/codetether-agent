use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub(crate) fn resolve_workspace_root(
    explicit_workspace: Option<&PathBuf>,
    global_project: Option<&PathBuf>,
) -> Result<PathBuf> {
    let base = explicit_workspace
        .cloned()
        .or_else(|| global_project.cloned())
        .unwrap_or(std::env::current_dir().context("Failed to resolve current directory")?);

    normalize_local_path(&base)
}

pub(crate) fn normalize_local_path(path: &Path) -> Result<PathBuf> {
    let expanded = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .context("Failed to resolve current directory")?
            .join(path)
    };

    Ok(expanded)
}
