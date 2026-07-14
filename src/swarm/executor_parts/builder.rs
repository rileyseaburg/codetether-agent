//! Fluent construction of configured swarm executors.

use super::SwarmExecutor;
use crate::swarm::SwarmConfig;

/// Builds a [`SwarmExecutor`] from common execution limits.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::swarm::executor::SwarmExecutorBuilder;
/// let executor = SwarmExecutorBuilder::new().max_subagents(4).build();
/// assert!(executor.control().is_none());
/// ```
pub struct SwarmExecutorBuilder {
    config: SwarmConfig,
}

impl SwarmExecutorBuilder {
    /// Creates a builder backed by [`SwarmConfig::default`].
    pub fn new() -> Self {
        Self {
            config: SwarmConfig::default(),
        }
    }

    /// Sets the maximum number of sub-agents.
    pub fn max_subagents(mut self, max: usize) -> Self {
        self.config.max_subagents = max;
        self
    }

    /// Sets the per-agent step limit.
    pub fn max_steps_per_subagent(mut self, max: usize) -> Self {
        self.config.max_steps_per_subagent = max;
        self
    }

    /// Sets the aggregate step limit.
    pub fn max_total_steps(mut self, max: usize) -> Self {
        self.config.max_total_steps = max;
        self
    }

    /// Sets each sub-agent deadline in seconds.
    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.config.subagent_timeout_secs = secs;
        self
    }

    /// Enables or disables parallel subtask execution.
    pub fn parallel_enabled(mut self, enabled: bool) -> Self {
        self.config.parallel_enabled = enabled;
        self
    }

    /// Builds the configured executor.
    pub fn build(self) -> SwarmExecutor {
        SwarmExecutor::new(self.config)
    }
}

impl Default for SwarmExecutorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
