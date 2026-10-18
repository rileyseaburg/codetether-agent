use std::future::Future;
/// Native session ownership and runtime steering operations.
/// # Implementors
/// UI adapters retain the real session slot and runtime request channel.
pub trait Transfer {
    /// Session payload moved to and restored from execution.
    type Session;
    /// Runtime request owning that session.
    type Request;
    /// Borrow the session identifier from a request.
    fn request_id(request: &Self::Request) -> &str;
    /// Recover the owned session from a rejected request.
    fn into_session(request: Self::Request) -> Self::Session;
    /// Reserve active-turn steering state.
    fn activate(&self, session_id: &str) -> bool;
    /// Transfer ownership to the runtime, returning rejected ownership on error.
    fn submit(&self, request: Self::Request) -> impl Future<Output = Result<(), Self::Request>>;
    /// Clear the steering reservation after a failed transfer.
    fn clear_steering(&self);
    /// Restore a rejected session to its slot.
    fn restore(&mut self, session: Self::Session);
    /// Set the user-visible handoff status.
    fn status(&mut self, message: &str);
}
