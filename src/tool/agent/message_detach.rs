//! Non-blocking dispatch for the `message` action (issue #296).
//!
//! When a caller sets `detach: true`, the sub-agent's turn must not freeze the
//! parent (and the TUI) for the full duration. This spawns a background task
//! that drains streaming events, persists the updated session, and announces
//! completion on the bus — then returns immediately. Progress is observable
//! via the `status` action.

use super::event_loop;
use super::execution_state::AgentRunGuard;
use super::message_finalize;
use crate::session::{Session, SessionEvent};
use crate::tool::ToolResult;
use anyhow::Result;
use tokio::sync::mpsc;

/// Spawn a background task to run the sub-agent turn and return immediately.
///
/// The `guard` is moved into the task so the agent stays marked busy until the
/// turn settles; `status` reflects its liveness in the meantime.
pub(super) fn dispatch(
    agent_id: String,
    guard: AgentRunGuard,
    mut rx: mpsc::Receiver<SessionEvent>,
    handle: tokio::task::JoinHandle<Result<Session>>,
    receipt: Option<String>,
) -> ToolResult {
    let display = super::store::get(&agent_id)
        .map(|entry| entry.name)
        .unwrap_or_else(|| agent_id.clone());
    let agent = agent_id.clone();
    tokio::spawn(async move {
        let next = agent.clone();
        let (response, thinking, tools, error, updated) =
            event_loop::run(&agent, &mut rx, handle).await;
        message_finalize::finalize(agent, response, thinking, tools, error, updated).await;
        super::collaboration_runtime::message_queue::finished(next, receipt, guard).await;
    });
    ToolResult::success(format!(
        "Dispatched message to @{display} ({agent_id}) in the background. \
         Use action \"status\" to watch progress; the reply lands on the bus when done."
    ))
}
