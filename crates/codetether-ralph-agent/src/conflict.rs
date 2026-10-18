//! Conflict resolution under the original smaller agent budget.
use crate::{Host, Invocation};
use codetether_bus_core::AgentBus;
use codetether_provider_core::Provider;
use std::{path::Path, sync::Arc};
use tracing::info;
/// Execute a conflict resolver with fifteen steps and its original timeout.
/// # Arguments
/// Receives provider/model, workspace, story ID for logging, prompt and bus.
/// # Returns
/// Final resolver output.
/// # Errors
/// Returns native loop failures.
/// # Examples
/// See [crate::Host] for native execution bindings.
pub async fn conflict<H: Host>(
    provider: &Arc<dyn Provider>,
    model: &str,
    working_dir: &Path,
    story_id: &str,
    prompt: &str,
    bus: Option<Arc<AgentBus>>,
) -> anyhow::Result<String> {
    let (system_prompt, tool_registry, tool_definitions) =
        crate::prepare::build::<H>(provider, model, working_dir);
    info!(target: "codetether_agent::ralph::ralph_loop::conflict_execute",
        "Conflict resolver starting with {} tools", tool_definitions.len());
    let (output, steps, tool_calls, _exit_reason) = H::execute(Invocation {
        provider: Arc::clone(provider),
        model,
        system: &system_prompt,
        prompt,
        definitions: tool_definitions,
        registry: tool_registry,
        max_steps: 15,
        timeout_secs: 120,
        events: None,
        id: String::new(),
        bus: bus.clone(),
        workspace: working_dir.to_path_buf(),
    })
    .await?;
    info!(target: "codetether_agent::ralph::ralph_loop::conflict_execute",
        story_id = %story_id, steps = steps, tool_calls = tool_calls, "Conflict resolver completed");
    Ok(output)
}
