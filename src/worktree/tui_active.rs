//! Global flag signaling that an interactive TUI owns the terminal.
//!
//! When the TUI is active it controls the alternate screen, so any raw
//! `print!`/`read_line` prompt (such as the VS Code auto-open confirmation)
//! would corrupt the rendered UI and break accessibility. Code that would
//! otherwise prompt on stdout must check [`is_tui_active`] first.

use std::sync::atomic::{AtomicBool, Ordering};

static TUI_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Mark the interactive TUI as owning the terminal (call on TUI enter).
pub fn set_tui_active(active: bool) {
    TUI_ACTIVE.store(active, Ordering::SeqCst);
}

/// Whether an interactive TUI currently owns the terminal.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::worktree::{is_tui_active, set_tui_active};
///
/// assert!(!is_tui_active());
/// set_tui_active(true);
/// assert!(is_tui_active());
/// set_tui_active(false);
/// ```
pub fn is_tui_active() -> bool {
    TUI_ACTIVE.load(Ordering::SeqCst)
}
