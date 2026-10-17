//! Session metadata and bounded transcript reads required for browsing.
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::Message;
use std::path::Path;
/// Read-only view of the existing durable session and indexed message store.
///
/// # Implementors
/// Delegate message reads and metadata access to the existing session type.
#[async_trait]
pub trait BrowseSource: codetether_context_loader::Source + Sync {
    /// Durable session ID.
    fn id(&self) -> &str;
    /// Workspace used for history directory resolution.
    fn workspace(&self) -> Option<&Path>;
    /// Total durable message count, including the unloaded prefix.
    fn message_count(&self) -> usize;
    /// Read a bounded range of messages by absolute index.
    /// # Errors
    /// Returns storage or decoding errors.
    async fn read_messages(id: &str, start: usize, limit: usize) -> Result<Vec<Message>>;
}
