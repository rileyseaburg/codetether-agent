//! Settlement of one accepted child-agent message.

use super::super::collaboration_runtime::message_input::MessageImage;
use super::super::{event_loop, execution_state, helpers, message_detach, message_finalize};
use crate::tool::ToolResult;
use anyhow::Result;

pub(in crate::tool::agent) async fn execute(
    agent_id: String,
    message: String,
    images: Vec<MessageImage>,
    params: &helpers::Params,
    guard: execution_state::AgentRunGuard,
    receipt: Option<String>,
) -> Result<ToolResult> {
    let (mut rx, handle) = super::start::begin(&agent_id, message, images, params).await?;
    execution_state::register(&agent_id, &handle);
    let handle = event_loop::ChildTask::new(handle);
    if params.detach_or_default() {
        return Ok(message_detach::dispatch(
            agent_id, guard, rx, handle, receipt,
        ));
    }
    let result = super::super::event_loop::run(&agent_id, &mut rx, handle).await;
    let output = message_finalize::finalize(
        agent_id.clone(),
        result.0,
        result.1,
        result.2,
        result.3,
        result.4,
    )
    .await;
    super::super::collaboration_runtime::message_queue::finished(agent_id, receipt, guard).await;
    Ok(output)
}
