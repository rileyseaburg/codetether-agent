//! Native metadata and session persistence operations used by checkpoint storage.
use crate::RunCheckpoint;
use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;
/// Session fields and persistence used by the checkpoint sidecar lifecycle.
/// # Implementors
/// Return the existing session identity, metadata checkpoint and directory,
/// and delegate session saves to the native persistence path.
#[async_trait]
pub trait Storage: Send + Sync {
    /// Existing session identity used to name the sidecar.
    fn id(&self) -> &str;
    /// Read the checkpoint already embedded in session metadata.
    fn checkpoint(&self) -> Option<&RunCheckpoint>;
    /// Replace the checkpoint embedded in session metadata.
    fn set_checkpoint(&mut self, checkpoint: Option<RunCheckpoint>);
    /// Resolve the existing session directory, propagating native path errors.
    fn sessions_dir() -> Result<PathBuf>;
    /// Save session metadata after the sidecar operation succeeds.
    async fn save_session(&self) -> Result<()>;
}
