//! Ordered worker-bridge deregistration and bounded task shutdown.
use crate::{Bridge, WorkerBridgeCmd};
/// Deregister the TUI agent, request shutdown and await the bridge task.
/// # Arguments
/// * `bridge` — Optional owned bridge whose task should stop.
/// # Returns
/// Completes after task exit or abort-and-join following a two-second timeout.
/// Command-channel and task-join failures remain best-effort.
/// # Examples
/// ```no_run
/// # async fn example(bridge: Option<codetether_tui_worker::Bridge>) {
/// codetether_tui_worker::shutdown_bridge(bridge).await;
/// # }
/// ```
pub async fn shutdown_bridge(bridge: Option<Bridge>) {
    let Some(bridge) = bridge else {
        return;
    };
    let _ = bridge
        .cmd_tx
        .send(WorkerBridgeCmd::DeregisterAgent {
            name: "tui".to_string(),
        })
        .await;
    let _ = bridge.cmd_tx.send(WorkerBridgeCmd::Shutdown).await;
    let mut handle = bridge.handle;
    if tokio::time::timeout(std::time::Duration::from_secs(2), &mut handle)
        .await
        .is_err()
    {
        handle.abort();
        let _ = handle.await;
    }
}
