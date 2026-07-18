//! Durable detached submission into a child mailbox.

use super::super::helpers::Params;
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::json;

pub(super) async fn detached(
    agent_id: String,
    message: String,
    params: &Params,
) -> Result<ToolResult> {
    super::super::residency::touch(&agent_id);
    let submitted =
        super::super::collaboration_runtime::message_queue::enqueue(&agent_id, message, params)
            .await?;
    super::super::collaboration_runtime::message_queue::dispatch_next(agent_id);
    Ok(ToolResult::success(
        json!({"submission_id":submitted.id}).to_string(),
    ))
}
