use codetether_provider_core::Message;
use std::future::Future;
/// Session services required to ingest human input.
/// # Implementors
/// The prompt runner retains history, publication and title-generation access.
pub trait Host: Send {
    /// Append a message with human-message provenance.
    fn append_human(&mut self, message: Message);
    /// Read the current turn correlation identifier.
    fn turn_id(&self) -> &str;
    /// Publish the original human text with its turn correlation.
    fn publish(&self, message: &str, turn_id: String);
    /// Whether the session has no title yet.
    fn needs_title(&self) -> bool;
    /// Generate a title using the active model.
    /// # Errors
    /// Propagates title-generation failures after history and publication.
    fn generate_title(&mut self) -> impl Future<Output = anyhow::Result<()>> + Send;
}
