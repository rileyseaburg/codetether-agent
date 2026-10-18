use super::Action;
use codetether_tui_models::{InputMode, ViewMode};
/// Native state reads and effects for character/backspace routing.
/// # Implementors
/// TUI adapters bind these operations to their existing state and handlers.
pub trait Host {
    /// Whether symbol search currently owns text input.
    fn symbols_active(&self) -> bool;
    /// Whether a specific view is current.
    fn in_view(&self, view: ViewMode) -> bool;
    /// Whether bus filter editing is active.
    fn bus_filter_active(&self) -> bool;
    /// Current bus filter text.
    fn bus_filter(&self) -> &str;
    /// Current chat input text.
    fn input(&self) -> &str;
    /// Set the frontend status line.
    fn set_status(&mut self, status: String);
    /// Set the chat input mode.
    fn set_mode(&mut self, mode: InputMode);
    /// Apply one native mutation in routing order.
    fn effect(&mut self, action: Action);
}
