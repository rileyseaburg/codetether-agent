//! Session operations required by the shared inheritance policy.
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::Message;
/// Application-owned transcript loading and append behavior.
///
/// # Implementors
/// Delegate to the existing session load, history, and append methods so page
/// classification and persistence semantics stay with the session owner.
#[async_trait]
pub trait HistorySource: Send + Sized {
    /// Load a parent session by its identity.
    /// # Errors
    /// Returns the application's session-load error.
    async fn load(parent_id: &str) -> Result<Self>;
    /// Borrow the parent's current message history.
    fn history(&self) -> &[Message];
    /// Append a selected message using the application's session lifecycle.
    fn add_message(&mut self, message: Message);
}
