//! Collect output and then settle the owned child task.
use crate::{ChildTask, Host, collect, finalize};
use serde_json::Value;
use tokio::sync::mpsc;
/// Collect native child events and return output plus the updated session.
/// # Arguments
/// * `agent_name` — Identity passed to native trace observation.
/// * `rx` — Native session-event receiver.
/// * `handle` — Child task, cancelled if this operation is dropped or times out.
/// # Returns
/// Response, thinking, tool previews, error and optional updated session.
/// # Examples
/// Use [ChildTask] for cancellation-safe task ownership and [Host] for events.
pub async fn run<H: Host, S>(
    agent_name: &str,
    rx: &mut mpsc::Receiver<H::SourceEvent>,
    handle: ChildTask<S>,
) -> (String, String, Vec<Value>, Option<String>, Option<S>) {
    let mut state = collect::collect_events::<H>(agent_name, rx).await;
    let updated_session = finalize::finish_handle(handle, state.timed_out, &mut state.error).await;
    (
        state.response,
        state.thinking,
        state.tools,
        state.error,
        updated_session,
    )
}
