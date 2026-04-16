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
pub(super) fn deregister_bridge(bridge: &Option<TuiWorkerBridge>) {
    if let Some(b) = bridge.as_ref() {
        let _ = b.cmd_tx.try_send(
            crate::tui::worker_bridge::WorkerBridgeCmd::DeregisterAgent {
                name: "tui".to_string(),
            },
        );
    }
}
