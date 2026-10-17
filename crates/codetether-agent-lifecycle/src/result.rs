//! Structured tool results for child lifecycle transitions.

use codetether_agent_registry::{AgentEntry, Session};
use codetether_collaboration_state::ThreadStatus;
use codetether_tool_core::ToolResult;
use serde_json::json;

pub(super) fn closed<S: Session>(
    name: &str,
    entry: &AgentEntry<S>,
    previous: &ThreadStatus,
) -> ToolResult {
    ToolResult::success(
        json!({
            "id": entry.id(),
            "name": name,
            "previous_status": previous
        })
        .to_string(),
    )
}

pub(super) fn resumed<S: Session>(
    name: &str,
    entry: &AgentEntry<S>,
    status: &ThreadStatus,
) -> ToolResult {
    ToolResult::success(json!({"id":entry.id(), "name":name, "status":status}).to_string())
}
