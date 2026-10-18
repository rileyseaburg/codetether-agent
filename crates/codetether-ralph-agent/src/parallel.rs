//! Parallel story execution with caller-selected step limits.
use crate::{Host, Invocation};
use codetether_bus_core::AgentBus;
use codetether_provider_core::Provider;
use codetether_swarm_core::SwarmEvent;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc;
use tracing::info;
/// Run a parallel story using its existing events and configured step limit.
/// # Arguments
/// Receives the selected provider/model, prompt, workspace, events, story ID, bus and step limit.
/// # Returns
/// Final agent output.
/// # Errors
/// Returns native loop failures.
/// # Examples
/// See [crate::Host] for native execution bindings.
pub async fn parallel<H: Host>(
    provider: &Arc<dyn Provider>,
    model: &str,
    prompt: &str,
    working_dir: &PathBuf,
    event_tx: Option<mpsc::Sender<SwarmEvent>>,
    story_id: String,
    bus: Option<Arc<AgentBus>>,
    max_steps_per_story: usize,
) -> anyhow::Result<String> {
    let (system_prompt, tool_registry, tool_definitions) =
        crate::prepare::build::<H>(provider, model, working_dir);
    info!(target: "codetether_agent::ralph::ralph_loop::model_static",
        "Ralph sub-agent starting with {} tools in {:?}", tool_definitions.len(), working_dir);
    let max_steps = max_steps_per_story;
    let (output, steps, tool_calls, _exit_reason) = H::execute(Invocation {
        provider: Arc::clone(provider),
        model,
        system: &system_prompt,
        prompt,
        definitions: tool_definitions,
        registry: tool_registry,
        max_steps,
        timeout_secs: 180,
        events: event_tx,
        id: story_id,
        bus: bus.clone(),
        workspace: working_dir.clone(),
    })
    .await?;
    info!(target: "codetether_agent::ralph::ralph_loop::model_static",
        "Ralph sub-agent completed: {} steps, {} tool calls", steps, tool_calls);
    Ok(output)
}
