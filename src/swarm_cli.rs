//! Native live bus and executor for the shared swarm command.
use crate::{bus, cli, swarm};
struct NativeHost;
#[async_trait::async_trait(?Send)]
impl codetether_swarm_cli::Host for NativeHost {
    type Bus = std::sync::Arc<bus::AgentBus>;
    type Executor = swarm::SwarmExecutor;
    fn create_bus() -> Self::Bus {
        let bus = bus::AgentBus::new().into_arc();
        bus::set_global(bus.clone());
        bus
    }
    fn create_executor(config: swarm::SwarmConfig, bus: Self::Bus) -> Self::Executor {
        swarm::SwarmExecutor::new(config).with_bus(bus)
    }
    async fn run(
        executor: &Self::Executor,
        task: &str,
        strategy: swarm::DecompositionStrategy,
    ) -> anyhow::Result<swarm::SwarmResult> {
        executor.execute(task, strategy).await
    }
}
pub(super) async fn execute(args: cli::SwarmArgs) -> anyhow::Result<()> {
    codetether_swarm_cli::execute::<NativeHost>(args).await
}
