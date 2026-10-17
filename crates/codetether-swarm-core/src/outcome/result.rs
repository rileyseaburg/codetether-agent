use super::*;

/// Result of swarm execution
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let result = codetether_swarm_core::SwarmResult { success: true, result: "done".into(), subtask_results: vec![], stats: Default::default(), artifacts: vec![], error: None };
/// assert!(result.success);
/// ```
pub struct SwarmResult {
    /// Overall success status
    pub success: bool,

    /// Aggregated result from all sub-agents
    pub result: String,

    /// Individual sub-task results
    pub subtask_results: Vec<SubTaskResult>,

    /// Execution statistics
    pub stats: SwarmStats,

    /// Any artifacts produced
    pub artifacts: Vec<SwarmArtifact>,

    /// Error message if failed
    pub error: Option<String>,
}
