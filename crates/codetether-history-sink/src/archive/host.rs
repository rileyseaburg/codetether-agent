//! Native durable consumer access used by the history archive worker.
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::Message;
use codetether_session_store::Delivery;
/// Fetches native consumer snapshots without changing their cursor semantics.
/// # Implementors
/// Implement on the application adapter that already loads durable sessions.
#[async_trait]
pub trait ArchiveSource: Send + Sync + 'static {
    /// Native decoded consumer snapshot retained for this delivery.
    type Session: Send;
    /// Read a bounded consumer delivery, propagating native load errors.
    async fn next(id: &str, name: &str, limit: usize) -> Result<Option<Delivery<Self::Session>>>;
    /// Borrow the snapshot's ordered messages for JSONL encoding.
    fn messages(session: &Self::Session) -> &[Message];
}
