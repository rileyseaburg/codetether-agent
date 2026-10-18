//! Emit liveness while a tool executes, stopping when its guard is dropped.
use crate::ToolHeartbeat;
use codetether_session_events::SessionEvent;
use std::time::Duration;
use tokio::sync::mpsc;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);

/// Spawn a heartbeat sender. Drop the returned guard to stop it.
/// # Arguments
/// The event sender, tool identity and start instant.
/// # Returns
/// A guard that stops the task on drop.
/// # Examples
/// ```no_run
/// # async fn example(tx: &tokio::sync::mpsc::Sender<codetether_session_events::SessionEvent<()>>) {
/// let guard = codetether_tool_reporting::spawn_heartbeat(tx, "call", "read", std::time::Instant::now());
/// drop(guard);
/// # }
/// ```
pub fn spawn_heartbeat<S: Send + 'static>(
    event_tx: &mpsc::Sender<SessionEvent<S>>,
    tool_call_id: &str,
    tool_name: &str,
    started_at: std::time::Instant,
) -> ToolHeartbeat {
    let tx = event_tx.clone();
    let tool_call_id = tool_call_id.to_string();
    let tool_name = tool_name.to_string();
    ToolHeartbeat(tokio::spawn(async move {
        let mut interval = tokio::time::interval(HEARTBEAT_INTERVAL);
        interval.tick().await; // skip immediate first tick
        loop {
            interval.tick().await;
            let elapsed = started_at.elapsed().as_secs();
            if tx
                .send(SessionEvent::ToolHeartbeat {
                    tool_call_id: tool_call_id.clone(),
                    name: tool_name.clone(),
                    elapsed_secs: elapsed,
                })
                .await
                .is_err()
            {
                break;
            }
        }
    }))
}
