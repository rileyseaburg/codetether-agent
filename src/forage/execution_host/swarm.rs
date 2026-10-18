//! Retain the existing native swarm executor through shared timeout handling.
use crate::swarm::{DecompositionStrategy, SwarmConfig, SwarmExecutor, SwarmResult};
pub struct NativeSwarm(SwarmExecutor);
impl NativeSwarm {
    pub(super) fn new(config: SwarmConfig) -> Self {
        Self(SwarmExecutor::new(config))
    }
}
impl codetether_forage_execution::Swarm for NativeSwarm {
    async fn execute(
        &self,
        prompt: &str,
        strategy: DecompositionStrategy,
    ) -> anyhow::Result<SwarmResult> {
        self.0.execute(prompt, strategy).await
    }
}
