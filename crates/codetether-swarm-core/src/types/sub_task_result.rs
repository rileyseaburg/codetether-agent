//! SubTaskResult execution contract.
use super::*;

/// Result of a subtask execution
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let result = codetether_swarm_core::SubTaskResult { subtask_id: "task-1".into(), subagent_id: "agent-1".into(), success: true, result: "done".into(), steps: 1, tool_calls: 1, execution_time_ms: 1, error: None, artifacts: vec![], retry_count: 0 };
/// assert!(result.success);
/// ```
pub struct SubTaskResult {
    /// Subtask ID
    pub subtask_id: String,

    /// Sub-agent ID that executed it
    pub subagent_id: String,

    /// Whether it succeeded
    pub success: bool,

    /// Result text
    pub result: String,

    /// Number of steps taken
    pub steps: usize,

    /// Tool calls made
    pub tool_calls: usize,

    /// Execution time (milliseconds)
    pub execution_time_ms: u64,

    /// Any error message
    pub error: Option<String>,

    /// Artifacts produced
    pub artifacts: Vec<String>,

    /// Number of retry attempts before this result was produced
    #[serde(default)]
    pub retry_count: u32,
}
