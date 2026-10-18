//! Application-owned session operations used by proactive workers.
use codetether_provider_core::Message;
/// Durable session snapshot, proactive settings and bounded transcript access.
/// # Implementors
/// Preserve normal session loading and consume the existing message window.
pub trait Session:
    codetether_session_store::legacy::Legacy
    + codetether_proactive_context::Source
    + Send
    + Sync
    + 'static
{
    /// Consume the loaded transcript without reordering messages.
    fn into_messages(self) -> Vec<Message>;
    /// Load the normal bounded transcript tail.
    /// # Errors
    /// Propagates application loading, decoding and validation failures.
    fn load_messages(
        id: &str,
        limit: usize,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Message>>> + Send;
}
