//! Atomic writer for [`super::workspace_index::WorkspaceIndex`].
//!
//! Uses write-to-tmp + rename to avoid torn writes and to let multiple
//! codetether processes update concurrently without corrupting the file.

use std::path::Path;

use anyhow::{Context, Result};

use super::workspace_index::WorkspaceIndex;

pub(super) fn save_sync(index: &WorkspaceIndex) -> Result<()> {
    let path = WorkspaceIndex::path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context("create sessions dir")?;
    }
    let tmp = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec(index)?;
    std::fs::write(&tmp, bytes).context("write index tmp")?;
    // Atomic on POSIX; on Windows `rename` over an existing file errors,
    // so fall through to a replace when needed.
    match std::fs::rename(&tmp, &path) {
        Ok(()) => Ok(()),
        Err(_) => {
            let _ = std::fs::remove_file(&path);
            std::fs::rename(&tmp, &path).context("rename index into place")
        }
    }
}

/// Merge one workspace->session mapping into the on-disk index.
pub(super) fn upsert_sync(workspace: &Path, session_id: &str) -> Result<()> {
    let mut index = WorkspaceIndex::load_sync();
    index.set(workspace, session_id);
    save_sync(&index)
}
