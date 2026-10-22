//! Rebase valid cached summaries when a bounded transcript drops its prefix.
use super::{SummaryIndex, SummaryRange};

impl SummaryIndex {
    /// Preserve summaries wholly inside the retained window and rebase their ranges.
    ///
    /// Summaries overlapping the removed prefix are discarded: their text cannot
    /// safely represent only the remaining messages. LRU order is preserved.
    ///
    /// # Arguments
    /// * `count` — Number of leading messages removed from the source window.
    /// # Returns
    /// Nothing; cache coordinates and the generation are updated in place.
    /// # Examples
    /// ```
    /// use codetether_context_policy::summary_index::SummaryIndex;
    /// let mut index = SummaryIndex::new();
    /// index.evict_prefix(10);
    /// assert!(index.is_empty());
    /// ```
    pub fn evict_prefix(&mut self, count: usize) {
        if count == 0 {
            return;
        }
        let rebase = |range: SummaryRange| {
            (range.start >= count).then(|| SummaryRange {
                start: range.start - count,
                end: range.end - count,
            })
        };
        self.tree = std::mem::take(&mut self.tree)
            .into_iter()
            .filter_map(|(range, node)| rebase(range).map(|range| (range, node)))
            .collect();
        self.lru_order = self.lru_order.iter().copied().filter_map(rebase).collect();
        self.generation = self.generation.wrapping_add(1);
    }
}
