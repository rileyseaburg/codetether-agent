//! First-turn dispatch and spawn-result formatting.

use super::super::spawn_messages::{success_message, with_warning};
use super::super::spawn_request::SpawnRequest;
use crate::tool::ToolResult;
use anyhow::Result;

/// Dispatch the first child turn and format the spawn result.
pub(super) async fn run(request: &SpawnRequest<'_>, warning: Option<&str>) -> Result<ToolResult> {
    tracing::info!(agent = %request.name, model = %request.model, "Sub-agent spawned, auto-starting first turn");
    let mut result = super::super::spawn_run::kick_off(request.name, request.detach).await?;
    let dispatch = if request.detach {
        " (background)"
    } else {
        " (synchronous)"
    };
    let body = format!(
        "{}\n\n{}\n\nFirst turn dispatched{dispatch}.",
        success_message(request),
        result.output,
    );
    result.output = with_warning(body, warning);
    Ok(result)
}
