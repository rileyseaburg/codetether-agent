use codetether_agent::session::Session;
use codetether_run_checkpoint::{RunCheckpoint, storage::Storage};
use std::{path::PathBuf, sync::OnceLock};

// Keep generic sidecar publication inside this evaluation's temporary directory.
pub static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
pub struct Native<'a>(pub &'a mut Session);
#[async_trait::async_trait]
impl Storage for Native<'_> {
    fn id(&self) -> &str {
        &self.0.id
    }
    fn checkpoint(&self) -> Option<&RunCheckpoint> {
        self.0.metadata.run_checkpoint.as_ref()
    }
    fn set_checkpoint(&mut self, value: Option<RunCheckpoint>) {
        self.0.metadata.run_checkpoint = value;
    }
    fn checkpoint_cleared(&self) -> bool {
        self.0.metadata.run_checkpoint_cleared
    }
    fn set_checkpoint_cleared(&mut self, value: bool) {
        self.0.metadata.run_checkpoint_cleared = value;
    }
    fn sessions_dir() -> anyhow::Result<PathBuf> {
        Ok(DIRECTORY.get().unwrap().clone())
    }
    async fn save_session(&self) -> anyhow::Result<()> {
        self.0.save().await
    }
}
