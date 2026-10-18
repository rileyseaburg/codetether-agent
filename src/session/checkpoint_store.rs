//! Native session specialization of shared checkpoint sidecar persistence.
use super::{checkpoint::RunCheckpoint, types::Session};
use anyhow::Result;
use std::path::PathBuf;
#[path = "checkpoint_store_host.rs"]
mod host;
impl Session {
    pub async fn save_run_checkpoint(&mut self, checkpoint: RunCheckpoint) -> Result<PathBuf> {
        codetether_run_checkpoint::storage::save(self, checkpoint).await
    }
    pub async fn load_run_checkpoint(&self) -> Result<Option<RunCheckpoint>> {
        codetether_run_checkpoint::storage::load(self).await
    }
    pub async fn clear_run_checkpoint(&mut self) -> Result<()> {
        codetether_run_checkpoint::storage::clear(self).await
    }
}
