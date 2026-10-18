//! Swarm execution through the application's existing executor.
use anyhow::Result;
use codetether_swarm_core::{DecompositionStrategy, SwarmResult};
use std::future::Future;
/// Run a configured swarm for one opportunity.
/// # Implementors
/// Application wrappers delegate to their existing swarm executor.
pub trait Swarm {
    /// Execute the prompt using the selected decomposition strategy.
    fn execute(
        &self,
        prompt: &str,
        strategy: DecompositionStrategy,
    ) -> impl Future<Output = Result<SwarmResult>> + Send;
}
