//! Native identity, checkpoint metadata and session-save bindings.
use crate::session::{RunCheckpoint, Session};
use anyhow::Result;
use async_trait::async_trait;
use codetether_run_checkpoint::storage::Storage;
use std::path::PathBuf;
#[async_trait]
impl Storage for Session {
    fn id(&self) -> &str {
        &self.id
    }
    fn checkpoint(&self) -> Option<&RunCheckpoint> {
        self.metadata.run_checkpoint.as_ref()
    }
    fn set_checkpoint(&mut self, checkpoint: Option<RunCheckpoint>) {
        self.metadata.run_checkpoint = checkpoint;
    }
    fn sessions_dir() -> Result<PathBuf> {
        Session::sessions_dir()
    }
    async fn save_session(&self) -> Result<()> {
        Session::save(self).await
    }
}
