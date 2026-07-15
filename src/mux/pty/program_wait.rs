//! Event-driven waiting for new PTY output or process exit.

use std::time::Duration;

use super::{PtyChunk, program::PtyProgram};

impl PtyProgram {
    pub(super) async fn read_wait(&self, offset: u64) -> PtyChunk {
        let mut changed = self.changed.subscribe();
        let chunk = self.read(offset);
        if !chunk.data.is_empty() || !chunk.running {
            return chunk;
        }
        let _ = tokio::time::timeout(Duration::from_secs(5), changed.changed()).await;
        self.read(offset)
    }
}
