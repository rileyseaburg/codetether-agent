//! Native executor construction and invocation for the inline worker loop.
use crate::{
    bus::AgentBus,
    swarm::{DecompositionStrategy, SwarmConfig, SwarmControl, SwarmExecutor, SwarmResult},
};
use codetether_swarm_core::SwarmEvent;
use std::sync::Arc;
use tokio::sync::mpsc;
impl codetether_swarm_runtime::inline::Executor for SwarmExecutor {
    type Bus = AgentBus;
    fn new(config: SwarmConfig) -> Self {
        SwarmExecutor::new(config)
    }
    fn with_control(self, control: SwarmControl) -> Self {
        SwarmExecutor::with_control(self, control)
    }
    fn with_bus(self, bus: Arc<AgentBus>) -> Self {
        SwarmExecutor::with_bus(self, bus)
    }
    fn with_event_tx(self, tx: mpsc::Sender<SwarmEvent>) -> Self {
        SwarmExecutor::with_event_tx(self, tx)
    }
    async fn execute(
        &self,
        prompt: &str,
        strategy: DecompositionStrategy,
    ) -> anyhow::Result<SwarmResult> {
        SwarmExecutor::execute(self, prompt, strategy).await
    }
}
