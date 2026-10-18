//! Native directory binding for the shared workspace-to-session index.
use super::types::Session;
use anyhow::Result;
pub(super) use codetether_workspace_index::INDEX_FILENAME;
use std::path::PathBuf;
pub(super) struct ApplicationSource;
impl codetether_workspace_index::Source for ApplicationSource {
    fn sessions_dir() -> Result<PathBuf> {
        Session::sessions_dir()
    }
}
pub(super) fn load_sync() -> codetether_workspace_index::WorkspaceIndex {
    codetether_workspace_index::load_sync::<ApplicationSource>()
}
