//! Swarm CLI bus initialization and application executor integration.
use super::{ExecutorHost, SwarmExecutor};
use codetether_bus_core::{AgentBus, global::set_global};
use codetether_swarm_core::{DecompositionStrategy, SwarmConfig, SwarmResult};
use std::sync::Arc;

#[async_trait::async_trait(?Send)]
impl<H: ExecutorHost> codetether_swarm_cli::Host for SwarmExecutor<H> {
    type Bus = Arc<AgentBus>;
    type Executor = Self;
    fn create_bus() -> Self::Bus {
        let bus = AgentBus::new().into_arc();
        set_global(bus.clone());
        bus
    }
    fn create_executor(config: SwarmConfig, bus: Self::Bus) -> Self::Executor {
        SwarmExecutor::<H>::new(config).with_bus(bus)
    }
    async fn run(
        executor: &Self::Executor,
        task: &str,
        strategy: DecompositionStrategy,
    ) -> anyhow::Result<SwarmResult> {
        executor.execute(task, strategy).await
    }
}
