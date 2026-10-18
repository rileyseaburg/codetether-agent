//! Concurrent execution for policy-approved read-only batches.
use crate::*;
/// Execute independently read-only tool calls concurrently.
pub(crate) async fn execute<H: Host>(
    agent: &H,
    session: &H::Session,
    calls: &[ContentPart],
) -> Vec<ToolResult> {
    let futures = calls.iter().filter_map(|part| {
        let call = part.as_tool_call()?;
        Some(agent.execute_tool_for_session(session, call.id, call.name, call.arguments))
    });
    futures::future::join_all(futures).await
}
