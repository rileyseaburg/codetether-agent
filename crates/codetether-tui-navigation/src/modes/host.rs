use super::{Action, Setting};
use codetether_tui_models::ViewMode;
use crossterm::event::KeyModifiers;
/// Mode state and synchronous input effects.
/// # Implementors
/// Native TUI state bindings.
pub trait State {
    /// Whether the requested view is active.
    fn in_view(&self, view: ViewMode) -> bool;
    /// Apply the selected synchronous operation.
    fn effect(&mut self, action: Action);
}
/// Session-backed settings and character input.
/// # Implementors
/// Native adapters retain the session borrow through asynchronous toggles.
pub trait Input {
    /// Toggle the setting if a session is available.
    fn setting(&mut self, setting: Setting) -> impl Future<Output = ()>;
    /// Handle general backspace input.
    fn backspace(&mut self) -> impl Future<Output = ()>;
    /// Edit the session filter with its existing modifier handling.
    fn session_char(&mut self, modifiers: KeyModifiers, character: char);
    /// Handle a general input character.
    fn character(&mut self, modifiers: KeyModifiers, character: char) -> impl Future<Output = ()>;
}
/// Complete host for mode-specific dispatch.
/// # Implementors
/// Supplied automatically by state and input adapters.
pub trait Host: State + Input {}
impl<T: State + Input> Host for T {}
