//! Batch validation statistics.

use super::trace_types::ValidatedTrace;

/// Statistics from batch trace validation.
#[derive(Debug, Clone, Default)]
pub struct BatchValidationStats {
    pub golden: Vec<ValidatedTrace>,
    pub consensus: Vec<ValidatedTrace>,
    pub unverified: Vec<(ValidatedTrace, String)>,
    pub failed: Vec<(ValidatedTrace, String)>,
}

impl BatchValidationStats {
    /// Total number of traces across all buckets.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let stats = BatchValidationStats::default();
    /// assert_eq!(stats.total(), 0);
    /// ```
    pub fn total(&self) -> usize {
        self.golden.len() + self.consensus.len() + self.unverified.len() + self.failed.len()
    }
}

/// Statistics from split-write operations.
///
/// Records paths and counts for each JSONL bucket
/// produced by [`BatchValidationStats::write_jsonl_split`].
///
/// # Examples
///
/// ```ignore
/// let stats: SplitWriteStats = batch.write_jsonl_split("out", "pfx")?;
/// assert_eq!(stats.golden_count, 3);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SplitWriteStats {
    /// Path to the golden bucket file.
    pub golden_path: String,
    /// Path to the consensus bucket file.
    pub consensus_path: String,
    /// Path to the failed bucket file.
    pub failed_path: String,
    /// Path to the unverified bucket file.
    pub unverified_path: String,
    /// Number of golden traces written.
    pub golden_count: usize,
    /// Number of consensus traces written.
    pub consensus_count: usize,
    /// Number of failed traces written.
    pub failed_count: usize,
    /// Number of unverified traces written.
    pub unverified_count: usize,
}
