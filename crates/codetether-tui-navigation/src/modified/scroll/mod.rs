//! Chat-only Alt/Control scroll shortcuts.
//!
//! [dispatch] selects an [Action] through [Host] without handling other keys.
mod run;
pub use run::dispatch;
/// Scrolling operation selected by a shortcut.
/// # Examples
/// ```
/// use codetether_tui_navigation::modified::scroll::Action;
/// assert!(matches!(Action::Down(1), Action::Down(1)));
/// ```
pub enum Action {
    /// Scroll down by the given number of rows.
    Down(usize),
    /// Scroll up by the given number of rows.
    Up(usize),
    /// Scroll down one viewport.
    PageDown,
    /// Scroll up one viewport.
    PageUp,
    /// Scroll to the start.
    Top,
    /// Scroll to the latest output.
    Bottom,
}
/// Native scrolling state and operations.
/// # Implementors
/// TUI adapters translate actions into existing scroll methods.
pub trait Host {
    /// Whether the chat view is active.
    fn in_chat(&self) -> bool;
    /// Apply one scrolling action.
    fn apply(&mut self, action: Action);
}
