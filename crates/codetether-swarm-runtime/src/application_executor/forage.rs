//! Execute forage selections through the shared swarm executor.
use crate::{ExecutorHost, SwarmExecutor};
use codetether_swarm_core::{DecompositionStrategy, SwarmResult};
impl<H: ExecutorHost> codetether_forage_execution::Swarm for SwarmExecutor<H> {
    async fn execute(
        &self,
        prompt: &str,
        strategy: DecompositionStrategy,
    ) -> anyhow::Result<SwarmResult> {
        SwarmExecutor::execute(self, prompt, strategy).await
    }
}
