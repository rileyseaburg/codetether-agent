//! Shutdown helpers for the TUI event loop.
//!
//! Deregisters the TUI agent from the worker bridge when
//! the event loop exits.
//!
//! # Examples
//!
//! ```ignore
//! deregister_bridge(&bridge);
//! ```

use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) async fn finish(
    bridge: Option<TuiWorkerBridge>,
    runtime: &crate::tui::app::session_runtime::TuiSessionHandle,
    mux_status: super::setup::mux_status::Reporter,
) {
    mux_status.clear().await;
    stop_bridge(bridge).await;
    runtime.shutdown().await;
}

/// Deregister the TUI agent from the worker bridge.
///
/// Sends a `DeregisterAgent` command to the bridge if
/// one is active.  Errors are silently ignored.
///
/// # Examples
///
/// ```ignore
/// deregister_bridge(&bridge);
/// ```
pub(super) async fn stop_bridge(bridge: Option<TuiWorkerBridge>) {
    let Some(bridge) = bridge else {
        return;
    };
    let _ = bridge
        .cmd_tx
        .send(
            crate::tui::worker_bridge::WorkerBridgeCmd::DeregisterAgent {
                name: "tui".to_string(),
            },
        )
        .await;
    let _ = bridge
        .cmd_tx
        .send(crate::tui::worker_bridge::WorkerBridgeCmd::Shutdown)
        .await;
    let mut handle = bridge.handle;
    if tokio::time::timeout(std::time::Duration::from_secs(2), &mut handle)
        .await
        .is_err()
    {
        handle.abort();
        let _ = handle.await;
    }
}
