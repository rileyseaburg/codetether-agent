//! Native event specialization for shared background child collection.
use super::{event_loop::ChildTask, execution_state::AgentRunGuard};
use crate::{session::SessionEvent, tool::ToolResult};
use tokio::sync::mpsc;
pub(super) fn dispatch(
    agent_id: String,
    guard: AgentRunGuard,
    rx: mpsc::Receiver<SessionEvent>,
    handle: ChildTask,
    receipt: Option<String>,
) -> ToolResult {
    codetether_child_turn::dispatch::<super::turn_host::ApplicationHost>(
        agent_id, guard, rx, handle, receipt,
    )
}
