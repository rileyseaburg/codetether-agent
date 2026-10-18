//! Native capabilities needed for direct migration and workspace backfill.
use anyhow::Result;
use async_trait::async_trait;
use codetether_recall_index::IndexedSession;
use std::path::Path;
/// Session loading and projection building layered over shared recall storage.
/// # Implementors
/// Implement on the application's existing recall storage host.
#[async_trait]
pub trait Host: codetether_recall_store::Host + Send + 'static {
    /// Native loaded session, moved to a blocking projection-builder task.
    type Session: Send + 'static;
    /// Schedule optional legacy backfill without waiting for it.
    fn schedule_backfill(&self, workspace: &Path);
    /// Load a bounded transcript tail for one-time direct migration.
    /// # Errors
    /// Propagates native loading, validation and decoding failures.
    async fn load_tail(&self, id: &str, limit: usize) -> Result<Self::Session>;
    /// Build a compact index from a loaded native session.
    fn build(session: &Self::Session) -> Option<IndexedSession>;
}
