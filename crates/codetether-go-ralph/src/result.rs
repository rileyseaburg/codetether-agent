//! Completed pipeline summary with shared Ralph lifecycle types.

use crate::StoryResult;
use codetether_ralph_core::RalphStatus;
use std::path::PathBuf;

/// Result of a go execution via Ralph.
///
/// # Examples
/// ```
/// use codetether_go_ralph::GoRalphResult;
/// use codetether_ralph_core::RalphStatus;
/// let result = GoRalphResult {
///     prd_path: "prd.json".into(), feature_branch: "feature/demo".into(),
///     passed: 0, total: 0, all_passed: true, iterations: 0, max_iterations: 10,
///     status: RalphStatus::MaxIterations, stories: vec![],
/// };
/// assert!(result.all_passed);
/// ```
#[derive(Debug, Clone)]
pub struct GoRalphResult {
    /// Path of the generated PRD.
    pub prd_path: PathBuf,
    /// Branch produced by Ralph.
    pub feature_branch: String,
    /// Number of passing stories.
    pub passed: usize,
    /// Total number of stories.
    pub total: usize,
    /// Whether the objective or all stories completed.
    pub all_passed: bool,
    /// Number of executed iterations.
    pub iterations: usize,
    /// Configured iteration limit.
    pub max_iterations: usize,
    /// Terminal Ralph lifecycle state.
    pub status: RalphStatus,
    /// Per-story results.
    pub stories: Vec<StoryResult>,
}
