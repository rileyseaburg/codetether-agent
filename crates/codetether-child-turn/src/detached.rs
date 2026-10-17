//! Background child collection retaining its execution guard.
use crate::TurnHost;
use crate::finalize as message_finalize;
use codetether_child_events as event_loop;
use codetether_collaboration_state::execution::AgentRunGuard;
use codetether_tool_core::ToolResult;
use tokio::sync::mpsc;

/// Spawn background collection and return the existing detached-dispatch message.
/// # Arguments
/// * `agent_id` — Canonical child identity.
/// * `guard` — Execution guard held until mailbox completion.
/// * `rx` — Native event stream.
/// * `handle` — Cancellation-safe child task.
/// * `receipt` — Optional mailbox receipt to complete.
/// # Returns
/// Immediate success text while collection continues in the runtime.
/// # Examples
/// Use an application [TurnHost] implementation for native effects.
///
/// The `guard` is moved into the task so the agent stays marked busy until the
/// turn settles; `status` reflects its liveness in the meantime.
pub fn dispatch<H: TurnHost>(
    agent_id: String,
    guard: AgentRunGuard,
    mut rx: mpsc::Receiver<H::SourceEvent>,
    handle: event_loop::ChildTask<H::Session>,
    receipt: Option<String>,
) -> ToolResult
where
    H::SourceEvent: 'static,
{
    let display = H::lookup(&agent_id)
        .map(|entry| entry.name)
        .unwrap_or_else(|| agent_id.clone());
    let agent = agent_id.clone();
    tokio::spawn(async move {
        let next = agent.clone();
        let (response, thinking, tools, error, updated) =
            event_loop::run::<H, H::Session>(&agent, &mut rx, handle).await;
        message_finalize::finalize::<H>(agent, response, thinking, tools, error, updated).await;
        H::finished(next, receipt, guard).await;
    });
    ToolResult::success(format!(
        "Dispatched message to @{display} ({agent_id}) in the background. \
         Use action \"status\" to watch progress; the reply lands on the bus when done."
    ))
}
