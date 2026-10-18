//! Fluent construction of configured swarm executors.

use crate::SwarmConfig;
use std::marker::PhantomData;

/// Builds an executor from common execution limits.
///
/// # Examples
///
/// ```rust
/// use codetether_swarm_core::{SwarmConfig, builder::SwarmExecutorBuilder};
/// struct Executor(SwarmConfig);
/// impl From<SwarmConfig> for Executor {
///     fn from(config: SwarmConfig) -> Self { Self(config) }
/// }
/// let executor = SwarmExecutorBuilder::<Executor>::new().max_subagents(4).build();
/// assert_eq!(executor.0.max_subagents, 4);
/// ```
pub struct SwarmExecutorBuilder<E> {
    config: SwarmConfig,
    executor: PhantomData<fn() -> E>,
}

impl<E: From<SwarmConfig>> SwarmExecutorBuilder<E> {
    /// Creates a builder backed by [`SwarmConfig::default`].
    pub fn new() -> Self {
        Self {
            config: SwarmConfig::default(),
            executor: PhantomData,
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
    pub fn build(self) -> E {
        self.config.into()
    }
}

impl<E: From<SwarmConfig>> Default for SwarmExecutorBuilder<E> {
    fn default() -> Self {
        Self::new()
    }
}
