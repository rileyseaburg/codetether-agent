//! Persistence operations required by manual context reset.
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::Message;
/// Session storage used to append and persist reset markers.
///
/// # Implementors
/// Implement on a durable session that already implements the shared context
/// loader. Append messages without changing their role or content.
#[async_trait]
pub trait ResetSession: codetether_context_loader::Source + Sync {
    /// Append the supplied message to this session's in-memory history.
    fn add_message(&mut self, message: Message);
    /// Persist the updated session before reporting reset success.
    /// # Errors
    /// Returns the underlying session storage error.
    async fn save(&self) -> Result<()>;
}
