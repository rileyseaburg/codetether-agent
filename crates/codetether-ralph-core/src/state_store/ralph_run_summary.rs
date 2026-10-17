//! RalphRunSummary persistence contract.
use super::*;

/// Lightweight summary for listing runs
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let summary = codetether_ralph_core::state_store::RalphRunSummary { run_id: "run-1".into(), okr_id: None, status: codetether_ralph_core::RalphStatus::Pending, passed: 0, total: 1, current_iteration: 0, created_at: String::new() };
/// assert_eq!(summary.total, 1);
/// ```
pub struct RalphRunSummary {
    /// Unique run identifier.
    pub run_id: String,
    /// Optional linked OKR identifier.
    pub okr_id: Option<String>,
    /// Current execution status.
    pub status: RalphStatus,
    /// Number of stories that passed.
    pub passed: usize,
    /// Total story count.
    pub total: usize,
    /// Current iteration count.
    pub current_iteration: usize,
    /// Creation timestamp.
    pub created_at: String,
}
