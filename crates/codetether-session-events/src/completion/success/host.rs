use super::ActiveSession;

/// Native session ownership and UI bindings for successful completion.
///
/// # Implementors
/// Frontends supply session persistence, list refresh and processing cleanup.
pub trait Host {
    /// Owned session returned by a completed prompt.
    type Session;
    /// Adapter retaining the active session's mutable borrow.
    type Active<'a>: ActiveSession
    where
        Self: 'a;
    /// Read the returned session identifier.
    fn session_id(session: &Self::Session) -> &str;
    /// Read the current slot's session identifier.
    fn current_id(&self) -> &str;
    /// Log that an out-of-date session result will be discarded.
    fn warn_stale(&self, session: &Self::Session);
    /// Attempt to persist an out-of-date session, ignoring save errors.
    fn save(session: &Self::Session) -> impl Future<Output = ()>;
    /// Refresh the session list after receiving an out-of-date result.
    fn refresh(&mut self) -> impl Future<Output = ()>;
    /// Whether the frontend is still processing a prompt.
    fn processing(&self) -> bool;
    /// Stop processing and notify the worker bridge.
    fn stopped(&mut self) -> impl Future<Output = ()>;
    /// Clear request timing after stopping processing.
    fn clear_timing(&mut self);
    /// Restore the returned session into its slot.
    fn restore(&mut self, session: Self::Session);
    /// Borrow the restored session together with its frontend bindings.
    fn active(&mut self) -> Option<Self::Active<'_>>;
}
