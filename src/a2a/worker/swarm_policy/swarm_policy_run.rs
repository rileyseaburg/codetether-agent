//! Run the executor inline so dropping its owner cannot detach the orchestration.

use super::super::swarm_event_output::format_swarm_event_for_output;
use crate::{
    bus::AgentBus,
    swarm::{DecompositionStrategy, SwarmConfig, SwarmControl, SwarmExecutor, SwarmResult},
};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;

pub(super) async fn run(
    prompt: &str,
    strategy: DecompositionStrategy,
    config: SwarmConfig,
    bus: Option<&Arc<AgentBus>>,
    output: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    control: SwarmControl,
) -> Result<SwarmResult> {
    let (events, mut incoming) = mpsc::channel(256);
    let mut executor = SwarmExecutor::new(config).with_control(control);
    if let Some(bus) = bus {
        executor = executor.with_bus(Arc::clone(bus));
    }
    if output.is_some() {
        executor = executor.with_event_tx(events);
    }
    let execution = executor.execute(prompt, strategy);
    tokio::pin!(execution);
    let mut receiving = output.is_some();
    loop {
        tokio::select! {
            result = &mut execution => return result,
            event = incoming.recv(), if receiving => match event {
                Some(event) => if let Some(cb) = &output && let Some(line) = format_swarm_event_for_output(&event) { cb(line); },
                None => receiving = false,
            }
        }
    }
}
