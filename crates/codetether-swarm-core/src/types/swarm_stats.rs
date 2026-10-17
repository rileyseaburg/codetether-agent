//! SwarmStats shared contract.
use super::*;

/// Swarm execution statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let stats = codetether_swarm_core::SwarmStats::default();
/// assert_eq!(stats.cache_hit_rate(), 0.0);
/// ```
pub struct SwarmStats {
    /// Total number of sub-agents spawned
    pub subagents_spawned: usize,

    /// Number of sub-agents that completed successfully
    pub subagents_completed: usize,

    /// Number of sub-agents that failed
    pub subagents_failed: usize,

    /// Total tool calls across all sub-agents
    pub total_tool_calls: usize,

    /// Critical path length (longest chain of dependent steps)
    pub critical_path_length: usize,

    /// Wall-clock execution time (milliseconds)
    pub execution_time_ms: u64,

    /// Estimated sequential time (milliseconds)
    pub sequential_time_estimate_ms: u64,

    /// Parallelization speedup factor
    pub speedup_factor: f64,

    /// Per-stage statistics
    pub stages: Vec<StageStats>,

    /// Rate limiting statistics
    pub rate_limit_stats: RateLimitStats,

    /// Number of cache hits (subtasks served from cache)
    pub cache_hits: u64,

    /// Number of cache misses (subtasks that required execution)
    pub cache_misses: u64,
}
