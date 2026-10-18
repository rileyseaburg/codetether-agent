use super::Ui;
use std::future::Future;
/// Native owned-session transfer used by automatic retry.
/// # Implementors
/// UI adapters retain the session slot, provider registry and runtime channel.
pub trait Host {
    /// Native UI implementing recovery effects.
    type Ui: Ui;
    /// Owned session payload.
    type Session;
    /// Owned runtime request containing that session.
    type Request;
    /// Shared provider registry handle.
    type Provider;
    /// Borrow native recovery UI state.
    fn ui(&mut self) -> &mut Self::Ui;
    /// Check out the returned session for retry.
    fn take_session(&mut self) -> Option<Self::Session>;
    /// Build a retry request with no images or new worktree root.
    fn request(session: Self::Session, prompt: String, provider: &Self::Provider) -> Self::Request;
    /// Transfer the request, returning its ownership on channel failure.
    fn submit(&self, request: Self::Request) -> impl Future<Output = Result<(), Self::Request>>;
    /// Restore the session contained in a rejected request.
    fn restore(&mut self, request: Self::Request);
}
