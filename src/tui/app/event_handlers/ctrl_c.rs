//! `Ctrl+C` handling: interrupt current turn or quit when idle.
//!
//! Mirrors Claude Code's "Cancel current input or generation" semantics:
//! while the agent is streaming a response `Ctrl+C` cancels the turn
//! (the cancel branch in `run_spawned_task` preserves the partial
//! transcript), and when the app is idle `Ctrl+C` quits.

use crate::tui::app::session_runtime::TuiSessionHandle;
use crate::tui::app::state::App;

/// Process a `Ctrl+C` keypress.
///
/// Returns `true` when the caller should exit the event loop (i.e. the
/// user wants to quit). Returns `false` when the keypress was consumed
/// to interrupt an in-flight turn and the loop should keep running.
pub(super) fn handle_ctrl_c(app: &mut App, runtime: &TuiSessionHandle) -> bool {
    if app.state.processing {
        runtime.request_cancel_current();
        app.state.status = "Cancellation requested — partial turn will be saved.".to_string();
        return false;
    }
    true
}
