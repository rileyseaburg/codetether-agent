//! Workspace confinement for proposed pre-approval file paths.

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

pub(super) fn confined(workspace: &Path, path: PathBuf) -> Result<PathBuf> {
    let root = workspace
        .canonicalize()
        .context("invalid approval workspace")?;
    let resolved = path.canonicalize().or_else(|_| {
        let parent = path
            .parent()
            .context("file path has no parent")?
            .canonicalize()?;
        Ok::<PathBuf, anyhow::Error>(
            parent.join(path.file_name().context("file path has no name")?),
        )
    })?;
    if !resolved.starts_with(&root) {
        bail!("file path is outside approval workspace")
    }
    Ok(resolved)
}
