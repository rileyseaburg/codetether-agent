//! Terminal cleanup helpers for restoring the user's shell after the TUI exits.
//!
//! The TUI switches the terminal into raw mode, enters the alternate screen,
//! captures mouse input, hides or moves the cursor, disables line wrapping, and
//! enables bracketed paste.
//! This module centralizes the best-effort teardown path so normal terminal
//! behavior is restored during explicit shutdown and when the guard is dropped.

use std::io;

use crossterm::terminal::disable_raw_mode;

#[path = "terminal_state/display_mode.rs"]
mod display_mode;

pub(super) use display_mode::enter as enter_display_mode;

/// Restores terminal settings modified by the interactive TUI.
///
/// This function is intentionally best-effort: cleanup may run while the
/// process is already unwinding, while stdout is unavailable, or after raw mode
/// has already been disabled. Errors from crossterm commands are ignored so
/// teardown cannot mask the original application error or panic.
///
/// # Side Effects
///
/// Disables raw mode, shows the cursor, restores line wrapping, leaves the
/// alternate screen, and disables mouse capture and bracketed paste for stdout.
pub(super) fn restore_terminal_state() {
    crate::worktree::set_tui_active(false);
    let _ = disable_raw_mode();
    let mut stdout = io::stdout();
    let _ = display_mode::leave(&mut stdout);
}

/// RAII guard that restores terminal state when it is dropped.
///
/// Create this guard after terminal setup succeeds and keep it alive for the
/// duration of the TUI event loop. Dropping the guard runs the same cleanup path
/// as an explicit shutdown, which protects the user's terminal when the loop
/// exits early or unwinds.
pub(super) struct TerminalGuard;

impl Drop for TerminalGuard {
    /// Restores terminal state as the guard leaves scope.
    ///
    /// The cleanup is best-effort and does not panic on terminal I/O failures.
    fn drop(&mut self) {
        restore_terminal_state();
    }
}
