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
    /// Whether this local pin is completely covered by a reserved mandatory
    /// header. Summaries, optional excerpts, and missing attachments do not
    /// qualify. The default retains the original pinned message.
    fn pinned_source_in_header(&self, _index: usize) -> bool {
        false
    }
    /// Absolute position of the first loaded message in durable history.
    /// Sources that hold the complete transcript use the default zero offset.
    fn source_offset(&self) -> usize {
        0
    }
    /// Cached summaries whose ranges use the current transcript coordinates.
    ///
    /// Local summaries supplement background preparation and take precedence
    /// for identical ranges. Sources without a local cache return `None`.
    fn summary_index(&self) -> Option<&SummaryIndex> {
        None
    }
    /// Read a prepared index, preserving native freshness validation.
    async fn prepared_index(&self) -> SummaryIndex;
}
