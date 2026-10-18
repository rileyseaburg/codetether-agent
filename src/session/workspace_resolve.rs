//! Native identity/path validation for shared persisted workspace resolution.
use crate::session::Session;
use anyhow::Result;
use std::path::PathBuf;
impl Session {
    /// Resolve the workspace recorded for a durable session.
    /// # Errors
    /// Returns identity, path, storage, JSON or missing-workspace errors.
    pub async fn recorded_workspace(id: &str) -> Result<PathBuf> {
        let path = Self::session_path(id)?;
        codetether_session_store::metadata::workspace::recorded_workspace(&path, id).await
    }
}
