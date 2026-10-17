//! Session summary index, production inputs and persistence contract.
use anyhow::Result;
use async_trait::async_trait;
use codetether_context_policy::summary_index::{SummaryIndex, SummaryNode, SummaryRange};
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Existing session storage and summary production operations.
///
/// # Implementors
/// Preserve the application's snapshot, producer and summary persistence behavior.
#[async_trait]
pub trait SummarySource: codetether_context_loader::Source {
    /// Snapshot of the producer's session-specific inputs.
    type Prepared: Send;
    /// Borrow the existing cache for an exact range lookup.
    fn summary_index(&self) -> &SummaryIndex;
    /// Borrow the same cache for production and insertion.
    fn summary_index_mut(&mut self) -> &mut SummaryIndex;
    /// Snapshot producer inputs before borrowing the mutable cache.
    fn prepare(&self) -> Self::Prepared;
    /// Produce the requested range with the supplied provider/model.
    /// # Errors
    /// Returns errors reported by the existing producer.
    async fn produce(
        prepared: Self::Prepared,
        range: SummaryRange,
        target: usize,
        provider: Arc<dyn Provider>,
        model: &str,
    ) -> Result<SummaryNode>;
    /// Persist a newly cached range.
    /// # Errors
    /// Returns storage write errors.
    async fn persist_summary(&mut self, range: SummaryRange, node: &SummaryNode) -> Result<()>;
}
