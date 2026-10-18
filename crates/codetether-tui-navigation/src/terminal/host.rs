use crossterm::event::{KeyEvent, MouseEvent};

/// Native input handlers used by terminal routing.
///
/// # Implementors
/// Frontends bind keyboard, paste and mouse actions to application state.
pub trait Host {
    /// Keyboard-handler error propagated to the event loop.
    type Error;
    /// Dispatch a key and report whether the UI should quit.
    fn key(&mut self, key: KeyEvent) -> impl Future<Output = Result<bool, Self::Error>>;
    /// Give the goal-answer editor first chance to consume pasted text.
    fn editor_paste(&mut self, text: &str) -> bool;
    /// Paste text into the normal input when the editor did not consume it.
    fn paste(&mut self, text: &str) -> impl Future<Output = ()>;
    /// Dispatch a mouse event.
    fn mouse(&mut self, mouse: MouseEvent) -> impl Future<Output = ()>;
}
