//! Permanent removal of one canonical child thread.

use super::super::store;
use crate::tool::ToolResult;

pub(in crate::tool::agent) async fn handle(
    target: &str,
    parent: Option<&str>,
) -> anyhow::Result<ToolResult> {
    let Some((agent_id, entry)) = store::scope::resolve_for_parent(target, parent) else {
        return Ok(ToolResult::error(format!("Agent {target} not found")));
    };
    let aborted = super::super::execution_state::abort(&agent_id);
    super::super::collaboration_runtime::thread_status::remove(&agent_id);
    super::super::collaboration_runtime::message_queue::clear(&agent_id).await?;
    super::super::persistence::remove(&entry).await?;
    tracing::info!(agent_id, agent = %entry.name, aborted, "Sub-agent killed");
    super::super::bus_publish::announce_done(&agent_id, false, "killed by user");
    store::remove(&agent_id);
    super::super::event_loop::live_trace::clear(&agent_id);
    Ok(ToolResult::success(format!(
        "Terminated and removed @{} ({agent_id})",
        entry.name
    )))
}
