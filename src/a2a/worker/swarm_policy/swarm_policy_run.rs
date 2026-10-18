//! Native executor binding for shared inline worker execution.
use crate::{
    bus::AgentBus,
    swarm::{DecompositionStrategy, SwarmConfig, SwarmControl, SwarmExecutor, SwarmResult},
};
use std::sync::Arc;
pub(super) async fn run(
    prompt: &str,
    strategy: DecompositionStrategy,
    config: SwarmConfig,
    bus: Option<&Arc<AgentBus>>,
    output: Option<codetether_swarm_runtime::inline::Output>,
    control: SwarmControl,
) -> anyhow::Result<SwarmResult> {
    codetether_swarm_runtime::inline::run::<SwarmExecutor>(
        prompt, strategy, config, bus, output, control,
    )
    .await
}
