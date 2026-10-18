//! Validated session identity to checkpoint-sidecar path.
use anyhow::Result;
use std::path::PathBuf;

pub(super) fn checkpoint_path<S: super::Storage>(id: &str) -> Result<PathBuf> {
    if id.is_empty()
        || id.len() > 128
        || id.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_')
    {
        anyhow::bail!("Invalid session ID: rejecting checkpoint path traversal risk");
    }
    Ok(S::sessions_dir()?.join(format!("{id}.checkpoint.json")))
}
