//! Selective validation of persisted summaries without changing transcript history.
use super::{SummaryIndex, SummaryNode, SummaryRange};
use codetether_provider_core::Message;

impl SummaryNode {
    /// Check exact source content and current policy before using this summary.
    /// Missing legacy provenance, malformed ranges and stale policies fail closed.
    /// # Arguments
    /// * `messages` — Transcript in the same coordinates as `range`.
    /// * `range` — Nonempty source range represented by this summary.
    /// # Returns
    /// True only for a complete source and current-policy match.
    /// # Examples
    /// ```
    /// use codetether_context_policy::summary_index::{Granularity, SummaryNode, SummaryRange};
    /// let legacy = SummaryNode { content: "old".into(), target_tokens: 10,
    ///     granularity: Granularity::Turn, generation: 0, provenance: None };
    /// assert!(!legacy.valid_for(&[], SummaryRange::new(0, 1).unwrap()));
    /// ```
    pub fn valid_for(&self, messages: &[Message], range: SummaryRange) -> bool {
        range.start < range.end
            && messages
                .get(range.start..range.end)
                .is_some_and(|slice| self.provenance.as_ref().is_some_and(|p| p.matches(slice)))
    }
}

impl SummaryIndex {
    /// Remove only entries that cannot be validated against the supplied transcript.
    ///
    /// Use on a working copy when loading legacy sidecars so historical records
    /// remain available for audit. The transcript itself is never modified.
    /// # Arguments
    /// * `messages` — Transcript in the same coordinate system as the index.
    /// # Returns
    /// Number of unusable entries removed; surviving LRU order is preserved.
    /// # Examples
    /// ```
    /// use codetether_context_policy::summary_index::SummaryIndex;
    /// let mut index = SummaryIndex::new();
    /// assert_eq!(index.retain_valid(&[]), 0);
    /// ```
    pub fn retain_valid(&mut self, messages: &[Message]) -> usize {
        let before = self.tree.len();
        self.tree
            .retain(|range, node| node.valid_for(messages, *range));
        self.lru_order.retain(|range| self.tree.contains_key(range));
        before - self.tree.len()
    }
}
