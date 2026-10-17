//! Settlement of one accepted child-agent message.

use crate::{MessageHost, detached as message_detach, finalize as message_finalize};
use anyhow::Result;
use codetether_agent_input::Params;
use codetether_child_events as event_loop;
use codetether_collaboration_input::MessageImage;
use codetether_collaboration_state::execution as execution_state;
use codetether_tool_core::ToolResult;

/// Start and settle one accepted message while retaining its execution guard.
/// # Arguments
/// * `agent_id` — Canonical child identity.
/// * `message` — Accepted message text.
/// * `images` — Input images passed to native startup.
/// * `params` — Trusted runtime context and detach setting.
/// * `guard` — Active execution claim held until settlement.
/// * `receipt` — Optional mailbox receipt to complete.
/// # Returns
/// A detached dispatch or completed turn result.
/// # Errors
/// Propagates native message startup failures.
/// # Examples
/// Use an application [MessageHost] implementation.
pub async fn execute<H: MessageHost>(
    agent_id: String,
    message: String,
    images: Vec<MessageImage>,
    params: &Params,
    guard: execution_state::AgentRunGuard,
    receipt: Option<String>,
) -> Result<ToolResult>
where
    H::SourceEvent: 'static,
{
    let (mut rx, handle) = H::begin_message(&agent_id, message, images, params).await?;
    execution_state::register(&agent_id, &handle);
    let handle = event_loop::ChildTask::new(handle);
    if params.detach_or_default() {
        return Ok(message_detach::dispatch::<H>(
            agent_id, guard, rx, handle, receipt,
        ));
    }
    let result = event_loop::run::<H, H::Session>(&agent_id, &mut rx, handle).await;
    let output = message_finalize::finalize::<H>(
        agent_id.clone(),
        result.0,
        result.1,
        result.2,
        result.3,
        result.4,
    )
    .await;
    H::finished(agent_id, receipt, guard).await;
    Ok(output)
}
