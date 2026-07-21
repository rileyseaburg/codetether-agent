//! Routing from a missing local child to a discovered A2A peer.

use super::{helpers, remote};
use crate::tool::ToolResult;
use anyhow::Result;

pub(super) async fn execute(
    target: &str,
    message: &str,
    params: &helpers::Params,
) -> Result<ToolResult> {
    if let Some(output) = crate::mux::control::send_agent_message(target, message).await? {
        return Ok(ToolResult::success(output));
    }
    remote::message_or_missing(
        target,
        message,
        params.context_id.as_deref(),
        params.parent_session_id.as_deref(),
        params.detach_or_default(),
    )
    .await
}
