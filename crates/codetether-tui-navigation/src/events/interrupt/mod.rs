//! Active-turn interruption and idle Ctrl+C exit policy.
//!
//! [handle] intercepts interrupt keys before lower-priority UI handlers.
//! [handle_ctrl_c] requests cancellation when busy and exits when idle.
//! Frontends implement [Host] to retain their cancellation transport.
mod ctrl_c;
mod interrupt_key;
pub use ctrl_c::handle_ctrl_c;
pub use interrupt_key::handle;
/// Frontend state and effects needed to interrupt a turn.
/// # Implementors
/// Frontends report their active view and processing state, and forward the
/// cancellation request through their existing runtime handle.
pub trait Host {
    /// Whether a turn is currently processing.
    fn processing(&self) -> bool;
    /// Whether the current view is chat.
    fn in_chat(&self) -> bool;
    /// Request best-effort cancellation of the current turn.
    fn request_cancel(&mut self);
    /// Replace the frontend status text.
    fn set_status(&mut self, status: String);
}
