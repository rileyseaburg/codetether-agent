//! Session ownership boundary for model selection.
use super::Session;
/// Active session access and retained-view refresh after selection.
/// # Implementors
/// Frontends wrap their session slot, returning no session during checkout.
pub trait Slot {
    /// Session whose model preference can be updated.
    type Session: Session;
    /// Borrow the active session when owned by this frontend.
    fn borrow_mut(&mut self) -> Option<&mut Self::Session>;
    /// Refresh the retained session view after model application.
    fn refresh_view(&mut self);
}
