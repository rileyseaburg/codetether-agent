use codetether_run_checkpoint::{RunCheckpoint, storage::Storage};
use std::{
    path::PathBuf,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
};
pub struct Session {
    id: String,
    pub checkpoint: Option<RunCheckpoint>,
    pub saved: Mutex<Option<RunCheckpoint>>,
    pub fail: bool,
    pub cleared: bool,
    pub saved_cleared: AtomicBool,
}
impl Session {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.into(),
            checkpoint: None,
            saved: Mutex::new(None),
            fail: false,
            cleared: false,
            saved_cleared: AtomicBool::new(false),
        }
    }
}
#[async_trait::async_trait]
impl Storage for Session {
    fn checkpoint_cleared(&self) -> bool {
        self.cleared
    }
    fn set_checkpoint_cleared(&mut self, cleared: bool) {
        self.cleared = cleared;
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn checkpoint(&self) -> Option<&RunCheckpoint> {
        self.checkpoint.as_ref()
    }
    fn set_checkpoint(&mut self, checkpoint: Option<RunCheckpoint>) {
        self.checkpoint = checkpoint;
    }
    fn sessions_dir() -> anyhow::Result<PathBuf> {
        static DIRECTORY: OnceLock<tempfile::TempDir> = OnceLock::new();
        Ok(DIRECTORY
            .get_or_init(|| tempfile::tempdir().unwrap())
            .path()
            .into())
    }
    async fn save_session(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.fail, "injected metadata save failure");
        *self.saved.lock().unwrap() = self.checkpoint.clone();
        self.saved_cleared.store(self.cleared, Ordering::SeqCst);
        Ok(())
    }
}
