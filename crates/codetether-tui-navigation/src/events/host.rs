use crossterm::event::KeyEvent;
use std::time::Instant;
/// Early modal handlers ordered before review and shortcut selection.
/// # Implementors
/// Native TUI adapters with modal state.
pub trait Layers {
    /// Handle the question dialog.
    fn question(&mut self, key: KeyEvent) -> bool;
    /// Handle the goal-answer editor.
    fn editor(&mut self, key: KeyEvent) -> impl Future<Output = bool>;
    /// Handle approval scrolling.
    fn approval_scroll(&mut self, key: KeyEvent) -> bool;
    /// Handle OKR input.
    fn okr(&mut self, key: KeyEvent) -> impl Future<Output = bool>;
}
/// Satisfaction review and optional goal resumption.
/// # Implementors
/// Native adapters with session review storage and resume context.
pub trait Review {
    /// Error returned by review or later key dispatch.
    type Error;
    /// Return a consumed review decision and whether to resume.
    fn review(&mut self, key: KeyEvent) -> impl Future<Output = Result<Option<bool>, Self::Error>>;
    /// Resume after a positive review decision.
    fn resume(&mut self) -> impl Future<Output = ()>;
}
/// Remaining modal priority, shortcuts and final key timing.
/// # Implementors
/// Native adapters delegate to existing key handlers.
pub trait Dispatch: Review {
    /// Handle interrupt and remaining modal priority.
    fn priority(&mut self, key: KeyEvent) -> impl Future<Output = Option<bool>>;
    /// Handle modifier shortcuts.
    fn modified(&mut self, key: KeyEvent) -> Option<Result<bool, Self::Error>>;
    /// Handle a key not consumed by earlier layers.
    fn unmodified(&mut self, key: KeyEvent) -> impl Future<Output = Result<bool, Self::Error>>;
    /// Borrow the timestamp updated only after the unmodified handler returns.
    fn last_key(&mut self) -> &mut Option<Instant>;
}
/// All layers required for ordered event dispatch.
/// # Implementors
/// Automatically implemented for native layer, review and dispatch bindings.
pub trait Host: Layers + Dispatch {}
impl<T: Layers + Dispatch> Host for T {}
