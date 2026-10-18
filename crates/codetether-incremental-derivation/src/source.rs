//! Native transcript and prepared-summary lookup boundary.
use async_trait::async_trait;
use codetether_context_policy::{pages::PageKind, summary_index::SummaryIndex};
use codetether_provider_core::Message;
/// Immutable session state needed by incremental projection.
/// # Implementors
/// Expose existing messages/pages and prepared-index lookup without changing history.
#[async_trait]
pub trait Source: Send + Sync {
    /// Source transcript in append order.
    fn messages(&self) -> &[Message];
    /// Parallel page-classification sidecar.
    fn pages(&self) -> &[PageKind];
    /// Read a prepared index, preserving native freshness validation.
    async fn prepared_index(&self) -> SummaryIndex;
}
