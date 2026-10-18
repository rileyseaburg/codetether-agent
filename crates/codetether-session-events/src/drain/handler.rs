use crate::SessionEvent;
use std::future::Future;
/// Native application of coalesced session events.
/// # Implementors
/// Frontends retain ownership of session slots and view-specific effects.
pub trait Handler<S: Send>: Send {
    /// Apply one event before the next queued event is processed.
    fn handle(&mut self, event: SessionEvent<S>) -> impl Future<Output = ()> + Send;
}
