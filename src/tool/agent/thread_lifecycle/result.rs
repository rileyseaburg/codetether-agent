//! Structured tool results for child lifecycle transitions.

use super::super::collaboration_runtime::thread_status::ThreadStatus;
use super::super::store::AgentEntry;
use crate::tool::ToolResult;
use serde_json::json;

pub(super) fn closed(name: &str, entry: &AgentEntry, previous: &ThreadStatus) -> ToolResult {
    ToolResult::success(
        json!({
            "id": entry.session.id,
            "name": name,
            "previous_status": previous
        })
        .to_string(),
    )
}

pub(super) fn resumed(name: &str, entry: &AgentEntry, status: &ThreadStatus) -> ToolResult {
    ToolResult::success(json!({"id":entry.session.id, "name":name, "status":status}).to_string())
}
