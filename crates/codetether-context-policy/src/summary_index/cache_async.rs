//! Async [`SummaryIndex::summary_for`] — cache-miss producer pattern.
//!
//! On cache hit, returns immediately. On miss, calls a caller-supplied
//! async producer (typically RLM summarisation) then stores the result.

use super::cache::SummaryIndex;
use super::types::{SummaryNode, SummaryRange};

impl SummaryIndex {
    /// Main read path for derivation policies.
    ///
    /// Cache hit: returns immediately. Cache miss: calls `producer`
    /// which encapsulates RLM summarisation, then caches the result.
    ///
    /// # Errors
    ///
    /// Returns the producer's error if the RLM call fails.
    pub async fn summary_for<F, Fut>(
        &mut self,
        range: SummaryRange,
        producer: F,
    ) -> anyhow::Result<SummaryNode>
    where
        F: FnOnce(SummaryRange) -> Fut,
        Fut: std::future::Future<Output = anyhow::Result<SummaryNode>>,
    {
        // Clone first to release the immutable borrow before touch_lru.
        if let Some(node) = self.tree.get(&range).cloned() {
            self.touch_lru(range);
            return Ok(node);
        }
        let node = producer(range).await?;
        self.insert(range, node.clone());
        Ok(node)
    }
}
