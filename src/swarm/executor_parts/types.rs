//! Native coordinator and cache specialization of shared executor state.
use crate::agent::Agent;
use crate::swarm::{SwarmCache, SwarmConfig};
/// Coordinates application swarm execution through native service bindings.
/// # Examples
/// ```
/// use codetether_agent::swarm::{SwarmConfig, SwarmExecutor};
/// let executor = SwarmExecutor::new(SwarmConfig::default());
/// assert!(executor.control().is_none());
/// ```
pub struct SwarmExecutor {
    pub(super) inner: codetether_swarm_runtime::ExecutorState<Agent, SwarmCache>,
}
impl From<SwarmConfig> for SwarmExecutor {
    fn from(config: SwarmConfig) -> Self {
        Self::new(config)
    }
}
