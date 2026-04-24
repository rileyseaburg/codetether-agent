//! `Ctrl+C` handling: interrupt current turn or quit when idle.
//!
//! Mirrors Claude Code's "Cancel current input or generation" semantics:
//! while the agent is streaming a response `Ctrl+C` cancels the turn
//! (the cancel branch in `run_spawned_task` preserves the partial
//! transcript), and when the app is idle `Ctrl+C` quits.

use crate::tui::app::state::App;

/// Process a `Ctrl+C` keypress.
///
/// Returns `true` when the caller should exit the event loop (i.e. the
/// user wants to quit). Returns `false` when the keypress was consumed
/// to interrupt an in-flight turn and the loop should keep running.
pub(super) fn handle_ctrl_c(app: &mut App) -> bool {
    if app.state.processing {
        if let Some(cancel) = app.state.current_turn_cancel.as_ref() {
            cancel.notify_one();
        }
        app.state.status =
            "Interrupted — partial turn saved. Press Ctrl+C again to quit.".to_string();
        return false;
    }
    true
}
