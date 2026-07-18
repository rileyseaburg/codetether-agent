//! First-turn dispatch and spawn-result formatting.

use super::super::spawn_messages::{detached_message, success_message, with_warning};
use super::super::spawn_request::SpawnRequest;
use crate::tool::ToolResult;
use anyhow::Result;

/// Dispatch the first child turn and format the spawn result.
pub(super) async fn run(
    request: &SpawnRequest<'_>,
    agent_id: &str,
    warning: Option<&str>,
) -> Result<ToolResult> {
    tracing::info!(agent = %request.name, model = %request.model, "Sub-agent spawned, auto-starting first turn");
    let mut result = super::super::spawn_run::kick_off(agent_id, request.detach).await?;
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
