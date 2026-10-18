//! Persist or remove a prepared sidecar at an already validated path.
use crate::Prepared;
use anyhow::Result;
use std::path::Path;

/// Serialize and atomically rename a temporary prepared-index sidecar.
/// # Errors
/// Propagates directory creation, JSON serialization, write or rename failures.
pub async fn write(path: &Path, value: &Prepared) -> Result<()> {
    let tmp = path.with_extension("rlm.json.tmp");
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(&tmp, serde_json::to_vec(value)?).await?;
    tokio::fs::rename(tmp, path).await?;
    Ok(())
}
/// Best-effort deletion of a prepared-index sidecar.
pub async fn remove(path: &Path) {
    let _ = tokio::fs::remove_file(path).await;
}
