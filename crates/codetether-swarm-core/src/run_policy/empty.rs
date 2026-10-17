//! Result returned when decomposition yields no work.

use crate::{SwarmResult, SwarmStats};

/// Construct the unsuccessful result used when decomposition produces no tasks.
///
/// # Arguments
/// None.
///
/// # Returns
/// An empty swarm result reporting that no subtasks were generated.
///
/// # Examples
/// ```
/// let result = codetether_swarm_core::run_policy::empty_result();
/// assert!(!result.success);
/// ```
pub fn empty_result() -> SwarmResult {
    SwarmResult {
        success: false,
        result: String::new(),
        subtask_results: Vec::new(),
        stats: SwarmStats::default(),
        artifacts: Vec::new(),
        error: Some("No subtasks generated".to_string()),
    }
}
