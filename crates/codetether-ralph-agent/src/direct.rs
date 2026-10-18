//! Instance story execution retaining its UI bridge guard.
use crate::{Host, Invocation};
use codetether_bus_core::AgentBus;
use codetether_provider_core::Provider;
use codetether_ralph_core::events::RalphEvent;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc;
use tracing::info;
/// Execute an instance story with its original fixed budget and exit reporting.
/// # Arguments
/// Receives provider/model, workspace, story identity, prompt and optional UI/bus events.
/// # Returns
/// Final agent text after reporting the loop exit.
/// # Errors
/// Returns native agent-loop failures.
/// # Examples
/// See [crate::Host] for the application binding contract.
pub async fn direct<H: Host>(
    provider: &Arc<dyn Provider>,
    model: &str,
    working_dir: &PathBuf,
    story_id: &str,
    prompt: &str,
    event_tx: Option<&mpsc::Sender<RalphEvent>>,
    bus: &Option<Arc<AgentBus>>,
) -> anyhow::Result<String> {
    let (system_prompt, tool_registry, tool_definitions) =
        crate::prepare::build::<H>(provider, model, working_dir);
    info!(target: "codetether_agent::ralph::ralph_loop::model_direct",
        "Ralph agent starting with {} tools in {:?}", tool_definitions.len(), working_dir);
    let (bridge_tx, _bridge_handle) = if let Some(ralph_tx) = event_tx {
        let (tx, handle) = crate::event_bridge(ralph_tx, story_id.to_string());
        (Some(tx), Some(handle))
    } else {
        (None, None)
    };
    let (output, steps, tool_calls, exit_reason) = H::execute(Invocation {
        provider: Arc::clone(provider),
        model,
        system: &system_prompt,
        prompt,
        definitions: tool_definitions,
        registry: tool_registry,
        max_steps: 30,
        timeout_secs: 180,
        events: bridge_tx,
        id: story_id.to_string(),
        bus: bus.clone(),
        workspace: working_dir.clone(),
    })
    .await?;
    crate::exit::log(story_id, steps, tool_calls, exit_reason);
    Ok(output)
}
