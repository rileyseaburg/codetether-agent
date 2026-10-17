//! First-turn dispatch and spawn-result formatting.

use crate::{
    Host,
    messages::{detached_message, success_message, with_warning},
};
use anyhow::Result;
use codetether_agent_input::SpawnRequest;
use codetether_tool_core::ToolResult;

/// Dispatch the first child turn and format the spawn result.
pub(super) async fn run<H: Host>(
    request: &SpawnRequest<'_>,
    agent_id: &str,
    warning: Option<&str>,
) -> Result<ToolResult> {
    tracing::info!(target: "codetether_agent::tool::agent::spawn::finish", agent = %request.name, model = %request.model, "Sub-agent spawned, auto-starting first turn");
    let mut result = H::kick_off(agent_id, request.detach).await?;
    if request.detach {
        result.output = detached_message(request, agent_id, warning);
        return Ok(result);
    }
    let dispatch = " (synchronous)";
    let body = format!(
        "{}\n\n{}\n\nFirst turn dispatched{dispatch}.",
        success_message(request, agent_id),
        result.output,
    );
    result.output = with_warning(body, warning);
    Ok(result)
}
