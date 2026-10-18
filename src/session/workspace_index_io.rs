//! Native directory specialization of workspace-index persistence.
use anyhow::Result;
use std::path::Path;
pub(super) fn upsert_sync(workspace: &Path, session_id: &str) -> Result<()> {
    codetether_workspace_index::upsert_sync::<super::workspace_index::ApplicationSource>(
        workspace, session_id,
    )
}
