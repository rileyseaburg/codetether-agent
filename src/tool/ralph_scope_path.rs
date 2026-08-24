//! Canonical PRD path confinement.

use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};

pub(super) fn resolve(root: &Path, requested: &Path) -> Result<PathBuf> {
    let joined = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        root.join(requested)
    };
    let path = if joined.exists() {
        joined.canonicalize()?
    } else {
        let parent = joined.parent().ok_or_else(|| anyhow!("invalid PRD path"))?;
        parent.canonicalize()?.join(
            joined
                .file_name()
                .ok_or_else(|| anyhow!("invalid PRD path"))?,
        )
    };
    if !path.starts_with(root) {
        return Err(anyhow!("Ralph PRD path is outside trusted workspace"));
    }
    Ok(path)
}
