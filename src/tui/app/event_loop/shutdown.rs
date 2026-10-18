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
    codetether_tui_worker::shutdown_bridge(bridge.map(Into::into)).await;
}
