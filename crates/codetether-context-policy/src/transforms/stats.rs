//! Aggregate effects of context transforms.

/// Aggregate outcome of every strategy in a transform pipeline.
///
/// # Examples
/// ```
/// use codetether_context_policy::transforms::ExperimentalStats;
/// let stats = ExperimentalStats::default();
/// assert_eq!(stats.total_bytes_saved, 0);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExperimentalStats {
    /// Number of tool-result content blocks replaced by a dedup marker.
    pub dedup_hits: usize,
    /// Number of tool-result content blocks head/tail-snipped.
    pub snippet_hits: usize,
    /// Total bytes removed from the `Vec<Message>` across all strategies.
    pub total_bytes_saved: usize,
}

impl ExperimentalStats {
    /// Add another transform's counters to this aggregate.
    ///
    /// # Examples
    /// ```
    /// use codetether_context_policy::transforms::ExperimentalStats;
    /// let mut total = ExperimentalStats::default();
    /// total.merge(ExperimentalStats { snippet_hits: 1, ..Default::default() });
    /// assert_eq!(total.snippet_hits, 1);
    /// ```
    pub fn merge(&mut self, other: ExperimentalStats) {
        self.dedup_hits += other.dedup_hits;
        self.snippet_hits += other.snippet_hits;
        self.total_bytes_saved += other.total_bytes_saved;
    }
}
