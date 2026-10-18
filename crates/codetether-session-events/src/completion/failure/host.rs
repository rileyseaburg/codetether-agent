/// Native recovery and presentation operations for a failed prompt.
///
/// # Implementors
/// Frontends provide session storage, retry schedulers and visible error state.
pub trait Host {
    /// Owned session returned by the failed runtime request.
    type Session;
    /// Restore the failed request's session before other updates.
    fn restore(&mut self, session: Self::Session);
    /// Stop processing and notify the worker bridge.
    fn stopped(&mut self) -> impl Future<Output = ()>;
    /// Finish request-duration accounting.
    fn complete_timing(&mut self);
    /// Schedule same-model recovery for a transient disconnect.
    fn reconnect(&mut self, error: &str);
    /// Schedule provider/model switching after same-model recovery is considered.
    fn smart_switch(&mut self, error: &str);
    /// Append the owned error to the visible transcript.
    fn error(&mut self, error: String);
    /// Borrow the UI status line.
    fn status(&mut self) -> &mut String;
    /// Scroll to the latest transcript entry.
    fn scroll(&mut self);
}
