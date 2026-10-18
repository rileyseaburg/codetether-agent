//! Owned terminal backend and cleanup guards in their original drop order.
use crate::{PanicHookGuard, TerminalGuard};
use ratatui::{Terminal, backend::CrosstermBackend};
/// Terminal session whose guards restore process and terminal state on drop.
///
/// Construct with [`crate::enter`] and keep alive throughout the UI loop.

/// # Examples
/// ```no_run
/// let runtime = codetether_terminal_runtime::enter().expect("terminal setup");
/// assert!(runtime.terminal.size().is_ok());
/// ```
pub struct Runtime {
    /// Terminal backend used for drawing during this runtime.
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub(crate) _terminal_guard: TerminalGuard,
    pub(crate) _panic_guard: PanicHookGuard,
}
