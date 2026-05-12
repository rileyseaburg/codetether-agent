use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

pub async fn plugin_path(root: &Path, raw_path: &str) -> Result<PathBuf> {
    let root = tokio::fs::canonicalize(root).await.with_context(|| {
        format!(
            "Failed to resolve TetherScript plugin root: {}",
            root.display()
        )
    })?;
    let candidate = candidate_path(&root, raw_path);
    let candidate = tokio::fs::canonicalize(&candidate).await.with_context(|| {
        format!(
            "Failed to resolve TetherScript plugin file: {}",
            candidate.display()
        )
    })?;
    super::validate::plugin_path(&candidate, &root)?;
    Ok(candidate)
}

fn candidate_path(root: &Path, raw_path: &str) -> PathBuf {
    let raw = Path::new(raw_path);
    if raw.is_absolute() {
        raw.to_path_buf()
    } else {
        root.join(raw)
    }
}
