//! Terminal and session-event operations.
use std::future::Future;
/// Handlers for terminal and session events.
/// # Implementors
/// Native adapters delegate to existing input/session handlers.
pub trait Events {
    /// Terminal stream item, including its native error type.
    type Terminal;
    /// Owned session-event payload.
    type Session;
    /// Handle terminal input and report a quit request.
    /// # Errors
    /// Returns the native terminal handler error.
    fn terminal(
        &mut self,
        event: Option<Self::Terminal>,
    ) -> impl Future<Output = anyhow::Result<bool>>;
    /// Apply one session event.
    fn session(&mut self, event: Self::Session) -> impl Future<Output = ()>;
    /// Coalesce remaining immediately available session events.
    fn coalesce(&mut self) -> impl Future<Output = ()>;
}
