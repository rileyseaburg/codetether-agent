//! RalphState serializable contract.
use super::*;

/// Ralph execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// # fn example(prd: codetether_ralph_core::Prd) {
/// let state = codetether_ralph_core::RalphState { prd, current_iteration: 0, max_iterations: 10, status: codetether_ralph_core::RalphStatus::Pending, progress_log: vec![], prd_path: "prd.json".into(), working_dir: ".".into() };
/// assert_eq!(state.current_iteration, 0);
/// # }
/// ```
pub struct RalphState {
    /// The PRD being worked on
    pub prd: Prd,

    /// Current iteration number
    pub current_iteration: usize,

    /// Maximum allowed iterations
    pub max_iterations: usize,

    /// Current status
    pub status: RalphStatus,

    /// Progress log entries
    #[serde(default)]
    pub progress_log: Vec<ProgressEntry>,

    /// Path to the PRD file
    pub prd_path: PathBuf,

    /// Working directory
    pub working_dir: PathBuf,
}
