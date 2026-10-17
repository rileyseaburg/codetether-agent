//! Route child messages into remote fallback, durable queues or a live turn.
use crate::MessageHost;
use anyhow::{Context, Result};
use codetether_agent_input::Params;
use codetether_collaboration_state::execution as execution_state;
use codetether_tool_core::ToolResult;
/// Route a message using the existing synchronous and detached rules.
/// # Arguments
/// * `params` — Agent-tool payload and trusted parent context.
/// # Returns
/// A remote, queued, detached or completed child result.
/// # Errors
/// Returns missing-input, routing, enqueue or native startup errors.
/// # Examples
/// Use an application [MessageHost] implementation.
pub async fn handle_message<H: MessageHost>(params: &Params) -> Result<ToolResult>
where
    H::SourceEvent: 'static,
{
    let target = params
        .name
        .as_ref()
        .context("name required for message")?
        .clone();
    let message = params.message.as_ref().context("message required")?.clone();
    let Some((agent_id, name)) = H::resolve(&target, params.parent_session_id.as_deref()) else {
        return H::remote(&target, &message, params).await;
    };
    if params.detach_or_default() {
        return super::submit::detached::<H>(agent_id, message, params).await;
    }
    let Some(guard) = execution_state::try_start(&agent_id) else {
        let submitted = H::enqueue(&agent_id, message, params).await?;
        return Ok(ToolResult::success(format!(
            "Queued follow-up for @{} ({agent_id}) at mailbox position {} (submission {})",
            name, submitted.depth, submitted.id
        )));
    };
    super::message_run::execute::<H>(
        agent_id,
        message,
        params.message_images.clone(),
        params,
        guard,
        None,
    )
    .await
}
