//! Runtime settings for Ralph execution.
use crate::config_defaults::*;
use serde::{Deserialize, Serialize};
/// Runtime configuration, including story limits, parallelism and retry policy.
///
/// # Examples
/// ```
/// let config = codetether_ralph_core::RalphConfig::default();
/// assert_eq!(config.max_iterations, 10);
/// ```

/// Ralph configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RalphConfig {
    /// Path to prd.json
    #[serde(default = "default_prd_path")]
    pub prd_path: String,

    /// Maximum iterations
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,

    /// Path to progress.txt
    #[serde(default = "default_progress_path")]
    pub progress_path: String,

    /// Whether to auto-commit changes
    #[serde(default = "default_auto_commit")]
    pub auto_commit: bool,

    /// Whether to run quality checks
    #[serde(default = "default_quality_checks_enabled")]
    pub quality_checks_enabled: bool,

    /// Model to use for iterations
    #[serde(default)]
    pub model: Option<String>,

    /// Whether to use RLM for progress compression
    #[serde(default)]
    pub use_rlm: bool,

    /// Enable parallel story execution
    #[serde(default = "default_parallel_enabled")]
    pub parallel_enabled: bool,

    /// Maximum concurrent stories to execute
    #[serde(default = "default_max_concurrent_stories")]
    pub max_concurrent_stories: usize,

    /// Use worktree isolation for parallel execution
    #[serde(default = "default_worktree_enabled")]
    pub worktree_enabled: bool,

    /// Timeout in seconds per step for story sub-agents (resets on each step)
    #[serde(default = "default_story_timeout_secs")]
    pub story_timeout_secs: u64,

    /// Maximum tool call steps per story sub-agent
    /// Increase this for complex stories that need more iterations
    #[serde(default = "default_max_steps_per_story")]
    pub max_steps_per_story: usize,

    /// Timeout in seconds per step for conflict resolution sub-agents
    #[serde(default = "default_conflict_timeout_secs")]
    pub conflict_timeout_secs: u64,

    /// Enable per-story relay teams (multi-agent collaboration per story)
    #[serde(default)]
    pub relay_enabled: bool,

    /// Maximum agents per relay team (2-8)
    #[serde(default = "default_relay_max_agents")]
    pub relay_max_agents: usize,

    /// Maximum relay rounds per story
    #[serde(default = "default_relay_max_rounds")]
    pub relay_max_rounds: usize,

    /// Maximum quality-gate-retry attempts per story.
    /// After quality gates fail, errors are fed back to the agent for self-repair.
    #[serde(default = "default_max_quality_retries")]
    pub max_quality_retries: usize,

    /// Maximum rate-limit retries before giving up on a story.
    #[serde(default = "default_max_rate_limit_retries")]
    pub max_rate_limit_retries: usize,

    /// Base delay in milliseconds for rate-limit exponential backoff.
    #[serde(default = "default_rate_limit_base_delay_ms")]
    pub rate_limit_base_delay_ms: u64,
}
