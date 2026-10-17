//! Serializable execution settings.
use super::*;

/// Swarm execution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let config = codetether_swarm_core::SwarmConfig::default();
/// assert_eq!(config.max_subagents, codetether_swarm_core::MAX_SUBAGENTS);
/// ```
pub struct SwarmConfig {
    /// Maximum number of concurrent sub-agents
    pub max_subagents: usize,

    /// Maximum tool calls per sub-agent
    pub max_steps_per_subagent: usize,

    /// Maximum total tool calls across all sub-agents
    pub max_total_steps: usize,

    /// Timeout for individual sub-agent execution (seconds)
    pub subagent_timeout_secs: u64,

    /// Whether to enable parallel execution
    pub parallel_enabled: bool,

    /// Critical path optimization threshold
    pub critical_path_threshold: usize,

    /// Model to use for sub-agents (provider/model format)
    pub model: Option<String>,

    /// Max concurrent API requests (rate limiting)
    pub max_concurrent_requests: usize,

    /// Delay between API calls in ms (rate limiting)
    pub request_delay_ms: u64,

    /// Enable worktree isolation for sub-agents
    pub worktree_enabled: bool,

    /// Automatically merge worktree changes on success
    pub worktree_auto_merge: bool,

    /// Enable speculative branch pruning for explicitly equivalent alternatives.
    #[serde(default)]
    pub collapse_enabled: bool,

    /// Working directory for worktree creation
    pub working_dir: Option<String>,

    /// Execution mode for sub-agent runtime
    #[serde(default)]
    pub execution_mode: ExecutionMode,

    /// Maximum number of Kubernetes sub-agent pods active at once.
    #[serde(default = "default_k8s_pod_budget")]
    pub k8s_pod_budget: usize,

    /// Optional container image override for Kubernetes sub-agent pods.
    #[serde(default)]
    pub k8s_subagent_image: Option<String>,

    /// Maximum number of retry attempts for transient failures (0 = no retries)
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,

    /// Base delay in milliseconds for the first retry (exponential backoff)
    #[serde(default = "default_base_delay_ms")]
    pub base_delay_ms: u64,

    /// Maximum delay in milliseconds between retries (caps exponential growth)
    #[serde(default = "default_max_delay_ms")]
    pub max_delay_ms: u64,
}
