//! Tool execution heartbeat — keeps the watchdog alive during long tools.
//!
//! Spawns a background task that emits `SessionEvent::ToolHeartbeat` every 10 s
//! while a tool is running. This prevents the watchdog from killing an agent
//! that is actively executing a slow tool (bash, kubectl, build, etc.).

use std::time::Duration;

use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::session::SessionEvent;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);

/// Spawn a heartbeat sender. Drop the returned guard to stop it.
pub(crate) fn spawn(
    event_tx: &mpsc::Sender<SessionEvent>,
    tool_call_id: &str,
    tool_name: &str,
    started_at: std::time::Instant,
) -> JoinHandle<()> {
    let tx = event_tx.clone();
    let tool_call_id = tool_call_id.to_string();
    let tool_name = tool_name.to_string();
    tokio::spawn(async move {
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
    })
}
