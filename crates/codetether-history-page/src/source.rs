//! Storage operations required for paginating a saved conversation.
//! Implement HistorySource using the host session store.
use async_trait::async_trait;
use codetether_provider_core::Message;
/// Backend for bounded tail loads and indexed message reads.
///
/// # Implementors
/// Implement this trait for the application's session-store adapter. Errors
/// are displayed by the caller; each method returns chronological messages.
#[async_trait]
pub trait HistorySource: Send + Sync {
    /// Load at most window messages from the end of the source session.
    ///
    /// # Arguments
    /// * `id` — Source identifier.
    /// * `window` — Maximum retained messages.
    ///
    /// # Returns
    /// Chronological tail messages.
    ///
    /// # Errors
    /// Return a displayable storage error on failure.
    async fn load_tail(id: &str, window: usize) -> Result<Vec<Message>, String>;
    /// Read at most limit messages beginning at the absolute start index.
    ///
    /// # Arguments
    /// * `id` — Source identifier.
    /// * `start` — First absolute message index.
    /// * `limit` — Maximum messages to return.
    ///
    /// # Returns
    /// Chronological messages in the requested range.
    ///
    /// # Errors
    /// Return a displayable storage error on failure.
    async fn read_messages(id: &str, start: usize, limit: usize) -> Result<Vec<Message>, String>;
}
