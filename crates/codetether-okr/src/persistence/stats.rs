//! Counts of persisted objectives and execution runs.

use crate::{OkrRunStatus, OkrStatus};

/// Statistics about the OKR repository
///
/// # Examples
/// ```
/// use codetether_okr::{persistence::OkrRepositoryStats};
/// let value = OkrRepositoryStats { total_okrs: 0, total_runs: 0,
///     okr_status_counts: Default::default(), run_status_counts: Default::default() };
/// assert_eq!(value.total_runs, 0);
/// ```
#[derive(Debug, serde::Serialize)]
pub struct OkrRepositoryStats {
    pub total_okrs: usize,
    pub total_runs: usize,
    pub okr_status_counts: std::collections::HashMap<OkrStatus, usize>,
    pub run_status_counts: std::collections::HashMap<OkrRunStatus, usize>,
}
