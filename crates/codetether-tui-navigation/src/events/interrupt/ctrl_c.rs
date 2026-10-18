//! `Ctrl+C` handling: interrupt current turn or quit when idle.
//!
//! Mirrors Claude Code's "Cancel current input or generation" semantics:
//! while the agent is streaming a response `Ctrl+C` cancels the turn
//! (the cancel branch in `run_spawned_task` preserves the partial
//! transcript), and when the app is idle `Ctrl+C` quits.

use super::Host;

/// Process a `Ctrl+C` keypress.
///
/// Returns `true` when the caller should exit the event loop (i.e. the
/// user wants to quit). Returns `false` when the keypress was consumed
/// to interrupt an in-flight turn and the loop should keep running.
/// Handle Ctrl+C against the current processing state.
/// # Arguments
/// * `host` — Current view/processing state and cancellation effects.
/// # Returns
/// True to exit while idle; false after requesting active-turn cancellation.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_tui_navigation::events::interrupt::Host) {
/// let _ = codetether_tui_navigation::events::interrupt::handle_ctrl_c(host);
/// # }
/// ```
pub fn handle_ctrl_c(host: &mut impl Host) -> bool {
    if host.processing() {
        host.request_cancel();
        host.set_status("Cancellation requested — partial turn will be saved.".to_string());
        return false;
    }
    true
}
