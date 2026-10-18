//! Native transcript access required for orchestration and input broadcasting.
use codetether_provider_core::Message;
/// A steering transcript with ordinary message insertion and history access.
/// # Implementors
/// Implement on the native session using its existing message bookkeeping.
/// Steering insertion itself remains owned by the inherited transcript trait.
pub trait Session: codetether_session_steering::Transcript {
    /// Append a goal continuation through the ordinary native message path.
    fn add_message(&mut self, message: Message);
    /// Borrow the full ordered transcript for delegated input broadcasting.
    fn messages(&self) -> &[Message];
}
