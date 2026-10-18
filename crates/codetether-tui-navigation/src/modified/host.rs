use super::Action;
use crossterm::event::KeyEvent;
/// Current shortcut-relevant UI state.
/// # Implementors
/// Native TUI state adapters.
pub trait State {
    /// Whether the current view is chat.
    fn in_chat(&self) -> bool;
    /// Whether a watchdog notice is active.
    fn watchdog_active(&self) -> bool;
    /// Whether the layout is currently Webview.
    fn webview(&self) -> bool;
    /// Borrow the UI status line.
    fn status(&mut self) -> &mut String;
}
/// Native handlers that may consume shortcuts before or after normal selection.
/// # Implementors
/// Adapters retaining cwd and runtime bindings.
pub trait Routing {
    /// Error propagated by scroll or fallback handlers.
    type Error;
    /// Give scrolling first chance to consume a key.
    fn alt_scroll(&mut self, key: KeyEvent) -> Option<Result<bool, Self::Error>>;
    /// Give approval handling first chance to consume a control character.
    fn approval(&mut self, character: char) -> bool;
    /// Process Ctrl+C and report whether to quit.
    fn ctrl_c(&mut self) -> bool;
    /// Try the remaining shared-file shortcuts.
    fn fallback(&mut self, key: KeyEvent, ctrl: bool) -> Option<Result<bool, Self::Error>>;
}
/// Apply the selected native UI operation.
/// # Implementors
/// Adapters binding each action to its existing handler or state operation.
pub trait Effects {
    /// Apply one effect in dispatch order.
    fn effect(&mut self, action: Action);
}
/// Complete host for modifier dispatch.
/// # Implementors
/// Automatically supplied for state, routing and effect adapters.
pub trait Host: State + Routing + Effects {}
impl<T: State + Routing + Effects> Host for T {}
