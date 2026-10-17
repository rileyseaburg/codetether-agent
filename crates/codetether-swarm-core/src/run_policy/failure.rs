//! Synthetic results for local task and join failures.

use crate::SubTaskResult;

/// Construct the existing synthetic result for a failed local task or join.
///
/// # Arguments
/// * `id` - Failed task identifier.
/// * `error` - Failure description.
///
/// # Returns
/// A failed result with zero execution counters.
///
/// # Examples
/// ```
/// let result = codetether_swarm_core::run_policy::failure_result("task".into(), "failed".into());
/// assert!(!result.success);
/// ```
pub fn result(id: String, error: String) -> SubTaskResult {
    SubTaskResult {
        subtask_id: id.clone(),
        subagent_id: format!("agent-{id}"),
        success: false,
        result: String::new(),
        steps: 0,
        tool_calls: 0,
        execution_time_ms: 0,
        error: Some(error),
        artifacts: Vec::new(),
        retry_count: 0,
    }
}
