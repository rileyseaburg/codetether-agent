use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

pub(super) fn canonical_workspace_path(path: &Path) -> Result<PathBuf> {
    let root =
        super::super::path::detect_workspace_root(path).unwrap_or_else(|| path.to_path_buf());
    std::fs::canonicalize(&root)
        .with_context(|| format!("failed to canonicalize workspace {}", root.display()))
}

pub(super) fn workspace_key(path: &Path) -> String {
    let mut hasher = Sha256::new();
    hasher.update(path.to_string_lossy().as_bytes());
    hex::encode(hasher.finalize())
}
