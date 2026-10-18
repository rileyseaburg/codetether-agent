//! Best-effort loading of the workspace index.
use crate::{INDEX_FILENAME, Source, WorkspaceIndex};
use anyhow::Result;
use std::path::PathBuf;
pub(crate) fn path<S: Source>() -> Result<PathBuf> {
    Ok(S::sessions_dir()?.join(INDEX_FILENAME))
}
/// Load the index, returning an empty map on directory, read or parse failure.
/// # Returns
/// The decoded cache, or an empty index when it cannot be read.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_workspace_index::Source>() {
/// let index = codetether_workspace_index::load_sync::<S>();
/// # let _ = index;
/// # }
/// ```
pub fn load_sync<S: Source>() -> WorkspaceIndex {
    let Ok(path) = path::<S>() else {
        return WorkspaceIndex::default();
    };
    std::fs::read(&path)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}
