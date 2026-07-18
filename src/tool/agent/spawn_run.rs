//! Start and settle the first turn of a durable child agent.

use super::{event_loop, execution_state, message_detach, message_finalize};
use crate::tool::ToolResult;
use anyhow::Result;

#[path = "spawn_run/start.rs"]
mod start;

pub(super) async fn kick_off(agent_id: &str, detach: bool) -> Result<ToolResult> {
    let Some(guard) = execution_state::try_start(agent_id) else {
        return Ok(ToolResult::success(format!(
            "Spawned {agent_id}. Agent will start once its current turn finishes."
        )));
    };
    let (mut rx, handle) = start::begin(agent_id).await?;
    execution_state::register(agent_id, &handle);
    if detach {
        return Ok(message_detach::dispatch(
            agent_id.to_string(),
            guard,
            rx,
            handle,
            None,
        ));
    }
    let result = event_loop::run(agent_id, &mut rx, handle).await;
    let output = message_finalize::finalize(
        agent_id.to_string(),
        result.0,
        result.1,
        result.2,
        result.3,
        result.4,
    )
    .await;
    super::collaboration_runtime::message_queue::finished(agent_id.to_string(), None, guard).await;
    Ok(output)
}
