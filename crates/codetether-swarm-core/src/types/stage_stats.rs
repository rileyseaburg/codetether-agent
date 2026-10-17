//! StageStats shared contract.
use super::*;

/// Statistics for a single execution stage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let stats = codetether_swarm_core::StageStats::default();
/// assert_eq!(stats.stage, 0);
/// ```
pub struct StageStats {
    /// Stage index
    pub stage: usize,

    /// Number of sub-agents in this stage
    pub subagent_count: usize,

    /// Maximum steps in this stage (critical path contribution)
    pub max_steps: usize,

    /// Total steps across all sub-agents in this stage
    pub total_steps: usize,

    /// Execution time for this stage (milliseconds)
    pub execution_time_ms: u64,
}
