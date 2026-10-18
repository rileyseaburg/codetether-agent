//! Public task-execution entry points.

use super::{ExecutorHost, SwarmExecutor};
use anyhow::Result;
use codetether_swarm_core::{DecompositionStrategy, SwarmResult};

impl<H: ExecutorHost> SwarmExecutor<H> {
    /// Decomposes and executes a task using the configured swarm.
    ///
    /// # Errors
    ///
    /// Returns an error when orchestration or stage execution fails.
    pub async fn execute(
        &self,
        task: &str,
        strategy: DecompositionStrategy,
    ) -> Result<SwarmResult> {
        H::execute(self, task, strategy).await
    }

    /// Executes a task without decomposition.
    ///
    /// # Errors
    ///
    /// Returns an error when orchestration or execution fails.
    pub async fn execute_single(&self, task: &str) -> Result<SwarmResult> {
        self.execute(task, DecompositionStrategy::None).await
    }
}
