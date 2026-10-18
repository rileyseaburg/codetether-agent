use std::{future::Future, sync::Arc};
use tokio::sync::Notify;
/// Native policy and execution effects for runtime command dispatch.
/// # Implementors
/// Frontends bind their review gate, reservations, notices and task executor.
pub trait Host<Q>: Send {
    /// Whether an answer-review gate prevents this request from executing.
    fn held(&self, request: &Q) -> bool;
    /// Release only the prepared reservation for the rejected request.
    fn release_prepared(&self, request: &Q);
    /// Attach the executor notifier, returning false when the runtime is busy.
    fn attach(&self, request: &Q, notify: Arc<Notify>) -> bool;
    /// Return the rejected request's session through a failure notice.
    fn reject(&mut self, request: Q, error: &'static str) -> impl Future<Output = ()> + Send;
    /// Announce that an accepted request is starting.
    fn started(&mut self) -> impl Future<Output = ()> + Send;
    /// Spawn the accepted request using the attached notifier.
    fn spawn(&self, request: Q, notify: Arc<Notify>);
    /// Notify the active executor of cancellation.
    fn cancel(&self);
}
