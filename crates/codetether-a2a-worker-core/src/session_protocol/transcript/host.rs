use async_trait::async_trait;
use codetether_provider_core::Message;
/// Native transcript mutation and persistence for worker messages.
/// # Implementors
/// Sessions retain their existing append and save semantics.
#[async_trait]
pub trait Host: Send {
    /// Append one message using the session's standard path.
    fn append(&mut self, message: Message);
    /// Save the updated session.
    /// # Errors
    /// Propagates native persistence failures.
    async fn save(&mut self) -> anyhow::Result<()>;
}
