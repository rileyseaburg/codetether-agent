//! Best-effort restoration of the terminal after the interactive UI.
use crossterm::terminal::disable_raw_mode;
use std::io;

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
///
/// # Returns
/// Ignores teardown errors after attempting all restoration operations.
///
/// # Examples
/// ```no_run
/// codetether_terminal_runtime::restore_terminal_state();
/// ```
pub fn restore_terminal_state() {
    codetether_worktree::set_tui_active(false);
    let _ = disable_raw_mode();
    let mut stdout = io::stdout();
    let _ = crate::display_mode::leave(&mut stdout);
}
