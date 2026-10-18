//! Notice application and ordered retry operations.
use std::future::Future;
/// Background notices and their follow-up actions.
/// # Implementors
/// Native adapters preserve the existing reconnect and retry routines.
pub trait Notices {
    /// Owned notice payload.
    type Notice;
    /// Apply one background notice.
    fn notice(&mut self, notice: Self::Notice) -> impl Future<Output = ()>;
    /// Retry a disconnected stream on the existing model.
    fn reconnect(&mut self) -> impl Future<Output = ()>;
    /// Apply a scheduled provider switch.
    fn smart_retry(&mut self) -> impl Future<Output = ()>;
    /// Apply a scheduled watchdog retry.
    fn watchdog_retry(&mut self) -> impl Future<Output = ()>;
}
