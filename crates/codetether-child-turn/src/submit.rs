//! Durable detached submission into a child mailbox.

use crate::MessageHost;
use anyhow::Result;
use codetether_agent_input::Params;
use codetether_tool_core::ToolResult;
use serde_json::json;

pub(super) async fn detached<H: MessageHost>(
    agent_id: String,
    message: String,
    params: &Params,
) -> Result<ToolResult> {
    codetether_collaboration_state::residency::touch(&agent_id);
    let submitted = H::enqueue(&agent_id, message, params).await?;
    H::dispatch_next(agent_id);
    Ok(ToolResult::success(
        json!({"submission_id":submitted.id}).to_string(),
    ))
}
