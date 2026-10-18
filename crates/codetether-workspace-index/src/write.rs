//! Atomic writer for [`WorkspaceIndex`].
//!
//! Uses write-to-tmp + rename to avoid torn writes and to let multiple
//! codetether processes update concurrently without corrupting the file.

use std::path::Path;

use anyhow::{Context, Result};

use crate::{Source, WorkspaceIndex, load_sync};

/// Persist the complete index using the existing temporary-file replacement.
/// # Arguments
/// * `index` — Workspace mappings to serialize.
/// # Returns
/// Success after the temporary file replaces the index.
/// # Errors
/// Propagates directory resolution, serialization and filesystem errors.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_workspace_index::Source>() -> anyhow::Result<()> {
/// codetether_workspace_index::save_sync::<S>(&Default::default())?;
/// # Ok(()) }
/// ```
pub fn save_sync<S: Source>(index: &WorkspaceIndex) -> Result<()> {
    let path = crate::read::path::<S>()?;
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
/// # Arguments
/// * `workspace` — Canonical workspace path used as the index key.
/// * `session_id` — ID to record as the most-recent session.
/// # Returns
/// Success after loading, updating and saving the index.
/// # Errors
/// Propagates save failures; load failures produce an empty starting index.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_workspace_index::Source>() -> anyhow::Result<()> {
/// codetether_workspace_index::upsert_sync::<S>(std::path::Path::new("/project"), "id")?;
/// # Ok(()) }
/// ```
pub fn upsert_sync<S: Source>(workspace: &Path, session_id: &str) -> Result<()> {
    let mut index = load_sync::<S>();
    index.set(workspace, session_id);
    save_sync::<S>(&index)
}
