//! Native session specialization for shared child-turn finalization.
use crate::{session::Session, tool::ToolResult};
use serde_json::Value;
pub(super) async fn finalize(
    agent_id: String,
    response: String,
    thinking: String,
    tools: Vec<Value>,
    error: Option<String>,
    updated_session: Option<Session>,
) -> ToolResult {
    codetether_child_turn::finalize::<super::turn_host::ApplicationHost>(
        agent_id,
        response,
        thinking,
        tools,
        error,
        updated_session,
    )
    .await
}
