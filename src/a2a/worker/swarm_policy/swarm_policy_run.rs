use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::{
    bus::AgentBus,
    swarm::{DecompositionStrategy, SwarmConfig, SwarmExecutor, SwarmResult},
};

use super::super::swarm_event_output::format_swarm_event_for_output;

pub(super) async fn run(
    prompt: &str,
    strategy: DecompositionStrategy,
    config: SwarmConfig,
    bus: Option<&Arc<AgentBus>>,
    output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> Result<SwarmResult> {
    if output_callback.is_some() {
        return run_streaming(prompt, strategy, config, bus, output_callback).await;
    }
    let mut executor = SwarmExecutor::new(config);
    if let Some(bus) = bus {
        executor = executor.with_bus(Arc::clone(bus));
    }
    executor.execute(prompt, strategy).await
}

async fn run_streaming(
    prompt: &str,
    strategy: DecompositionStrategy,
    config: SwarmConfig,
    bus: Option<&Arc<AgentBus>>,
    output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> Result<SwarmResult> {
    let (event_tx, mut event_rx) = mpsc::channel(256);
    let mut executor = SwarmExecutor::new(config).with_event_tx(event_tx);
    if let Some(bus) = bus {
        executor = executor.with_bus(Arc::clone(bus));
    }
    let prompt_owned = prompt.to_string();
    let mut handle = tokio::spawn(async move { executor.execute(&prompt_owned, strategy).await });
    let mut final_result = None;
    while final_result.is_none() {
        tokio::select! {
            event = event_rx.recv() => if let Some(event) = event && let Some(cb) = &output_callback && let Some(line) = format_swarm_event_for_output(&event) { cb(line); },
            joined = &mut handle => final_result = Some(joined.map_err(|error| anyhow::anyhow!("Swarm join failure: {}", error))??),
        }
    }
    Ok(final_result.expect("swarm result"))
}
