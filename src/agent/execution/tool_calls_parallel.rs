//! Parallel dispatch for independently read-only agent tool calls.

use crate::agent::Agent;
use crate::provider::ContentPart;
use crate::session::Session;
use crate::tool::ToolResult;

/// Execute independently read-only tool calls concurrently.
pub(super) async fn execute(
    agent: &Agent,
    session: &Session,
    calls: &[ContentPart],
) -> Vec<ToolResult> {
    let futures = calls.iter().filter_map(|part| {
        let call = part.as_tool_call()?;
        Some(agent.execute_tool_for_session(session, call.name, call.arguments))
    });
    futures::future::join_all(futures).await
}
