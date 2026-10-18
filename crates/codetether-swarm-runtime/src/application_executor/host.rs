//! Concrete coordinator and task execution supplied by the application.
use super::SwarmExecutor;
use anyhow::Result;
use codetether_swarm_core::{DecompositionStrategy, SwarmResult};
use std::future::Future;
/// Bind a shared executor to the application's stage and agent services.
/// # Implementors
/// Applications retain concrete agent execution and stage policy adapters.
pub trait ExecutorHost: Sized + Send + Sync + 'static {
    /// Coordinator agent retained behind the executor's shared mutex.
    type Agent: Send + 'static;
    /// Execute the original task under the requested decomposition strategy.
    /// # Errors
    /// Propagates planning and stage-execution failures.
    fn execute(
        executor: &SwarmExecutor<Self>,
        task: &str,
        strategy: DecompositionStrategy,
    ) -> impl Future<Output = Result<SwarmResult>> + Send;
}
