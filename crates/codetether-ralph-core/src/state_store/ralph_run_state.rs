//! RalphRunState persistence contract.
use super::*;

/// Persistent state for a single Ralph run
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// # fn example(prd: codetether_ralph_core::Prd) {
/// let state = codetether_ralph_core::state_store::RalphRunState { run_id: "run-1".into(), okr_id: None, prd, config: Default::default(), status: codetether_ralph_core::RalphStatus::Pending, current_iteration: 0, max_iterations: 10, progress_log: vec![], story_results: vec![], error: None, created_at: String::new(), started_at: None, completed_at: None };
/// assert_eq!(state.run_id, "run-1");
/// # }
/// ```
pub struct RalphRunState {
    /// Unique run identifier.
    pub run_id: String,
    /// Optional linked OKR identifier.
    pub okr_id: Option<String>,
    /// Current PRD content.
    pub prd: Prd,
    /// Execution settings recorded for this run.
    pub config: RalphConfig,
    /// Current execution status.
    pub status: RalphStatus,
    /// Current iteration count.
    pub current_iteration: usize,
    /// Configured iteration limit.
    pub max_iterations: usize,
    #[serde(default)]
    /// Append-only progress entries.
    pub progress_log: Vec<ProgressEntry>,
    #[serde(default)]
    /// Results indexed by story identifier.
    pub story_results: Vec<StoryResultEntry>,
    /// Most recent error, when present.
    pub error: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// First execution timestamp, when started.
    pub started_at: Option<String>,
    /// Completion timestamp, when completed.
    pub completed_at: Option<String>,
}
