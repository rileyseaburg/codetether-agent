//! Start and settle the first turn of a durable child agent.

use crate::{TurnHost, detached as message_detach, finalize as message_finalize};
use anyhow::Result;
use codetether_child_events as event_loop;
use codetether_collaboration_state::execution as execution_state;
use codetether_tool_core::ToolResult;

/// Start and settle a durable child's first turn, synchronously or in background.
/// # Arguments
/// * `agent_id` — Canonical child identity.
/// * `detach` — Whether collection continues in a background task.
/// # Returns
/// The turn result, detached message, or existing busy-child success message.
/// # Errors
/// Propagates native startup failures.
/// # Examples
/// Use an application [TurnHost] implementation for native effects.
pub async fn kick_off<H: TurnHost>(agent_id: &str, detach: bool) -> Result<ToolResult>
where
    H::SourceEvent: 'static,
{
    let Some(guard) = execution_state::try_start(agent_id) else {
        return Ok(ToolResult::success(format!(
            "Spawned {agent_id}. Agent will start once its current turn finishes."
        )));
    };
    let (mut rx, handle) = H::begin(agent_id).await?;
    execution_state::register(agent_id, &handle);
    let handle = event_loop::ChildTask::new(handle);
    if detach {
        return Ok(message_detach::dispatch::<H>(
            agent_id.to_string(),
            guard,
            rx,
            handle,
            None,
        ));
    }
    let result = event_loop::run::<H, H::Session>(agent_id, &mut rx, handle).await;
    let output = message_finalize::finalize::<H>(
        agent_id.to_string(),
        result.0,
        result.1,
        result.2,
        result.3,
        result.4,
    )
    .await;
    H::finished(agent_id.to_string(), None, guard).await;
    Ok(output)
}
