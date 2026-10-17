//! Native message-turn specialization for mailbox and direct callers.
use super::super::collaboration_runtime::message_input::MessageImage;
use super::super::{execution_state, helpers};
use crate::tool::ToolResult;
pub(in crate::tool::agent) async fn execute(
    agent_id: String,
    message: String,
    images: Vec<MessageImage>,
    params: &helpers::Params,
    guard: execution_state::AgentRunGuard,
    receipt: Option<String>,
) -> anyhow::Result<ToolResult> {
    codetether_child_turn::run_message::<super::super::turn_host::ApplicationHost>(
        agent_id, message, images, params, guard, receipt,
    )
    .await
}
