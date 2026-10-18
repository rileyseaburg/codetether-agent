//! Terminal teardown on scope exit and unwinding.
use crate::restore_terminal_state;

/// RAII guard that restores terminal state when it is dropped.
///
/// Create this guard after terminal setup succeeds and keep it alive for the
/// duration of the TUI event loop. Dropping the guard runs the same cleanup path
/// as an explicit shutdown, which protects the user's terminal when the loop
/// exits early or unwinds.
///
/// # Examples
/// ```no_run
/// let _guard = codetether_terminal_runtime::TerminalGuard;
/// ```
pub struct TerminalGuard;

impl Drop for TerminalGuard {
    /// Restores terminal state as the guard leaves scope.
    ///
    /// The cleanup is best-effort and does not panic on terminal I/O failures.
    fn drop(&mut self) {
        restore_terminal_state();
    }
}
