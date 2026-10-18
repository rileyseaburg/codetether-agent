//! Poll execution and optional event output without spawning a detached task.
use super::{Executor, Output};
use anyhow::Result;
use codetether_session_control::ControlHandle;
use codetether_swarm_core::{
    DecompositionStrategy, SwarmConfig, SwarmResult, worker_output::format_swarm_event_for_output,
};
use std::sync::Arc;
use tokio::sync::mpsc;
/// Run an executor while forwarding lifecycle messages to an optional sink.
/// # Arguments
/// Receives task text, strategy/configuration, optional bus/output and control.
/// # Returns
/// The executor result; channel closure disables receiving without stopping it.
/// # Errors
/// Propagates the executor's result unchanged.
/// # Examples
/// ```no_run
/// async fn run<E: codetether_swarm_runtime::inline::Executor>() -> anyhow::Result<()> {
///     let _ = codetether_swarm_runtime::inline::run::<E>("inspect", codetether_swarm_core::DecompositionStrategy::None, Default::default(), None, None, codetether_session_control::ControlHandle::new()).await?;
///     Ok(())
/// }
/// ```
pub async fn run<E: Executor>(
    prompt: &str,
    strategy: DecompositionStrategy,
    config: SwarmConfig,
    bus: Option<&Arc<E::Bus>>,
    output: Option<Output>,
    control: ControlHandle,
) -> Result<SwarmResult> {
    let (events, mut incoming) = mpsc::channel(256);
    let mut executor = E::new(config).with_control(control);
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
