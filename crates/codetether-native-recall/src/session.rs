//! Application transcript access for native recall services.
use chrono::{DateTime, Utc};
/// Session storage contracts and the fields needed for recall projections.
/// # Implementors
/// Implement on the existing session type, preserving its tail-loader and
/// prior-context policy through the inherited recall source contract.
pub trait Session:
    codetether_session_store::legacy::Legacy
    + codetether_session_listing::indexed::Header
    + codetether_recall_context::Source
    + Send
    + Sync
    + 'static
{
    /// Optional title stored with the recalled evidence.
    fn title(&self) -> Option<&str>;
    /// Last update time of the indexed session.
    fn updated_at(&self) -> DateTime<Utc>;
    /// Absolute position of the first loaded message.
    fn message_offset(&self) -> usize;
    /// Durable message count, including messages outside the loaded window.
    fn message_count(&self) -> usize;
    /// Load the normal bounded transcript window.
    /// # Errors
    /// Propagates the application's loading and validation failures.
    fn load_tail(
        id: &str,
        limit: usize,
    ) -> impl std::future::Future<Output = anyhow::Result<Self>> + Send;
}
