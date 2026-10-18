/// Operations performed while the restored session remains mutably borrowed.
///
/// # Implementors
/// Native adapters borrow the session and disjoint frontend state together.
pub trait ActiveSession {
    /// Attach the global session bus when it is absent.
    fn attach_bus(&mut self);
    /// Clear turn-cancellation state.
    fn clear_cancel(&mut self);
    /// Reset the successful stream's reconnect budget.
    fn reset_reconnect(&mut self);
    /// Update the frontend's current-session identifier.
    fn set_session_id(&mut self);
    /// Attempt to persist the restored session, ignoring save errors.
    fn save(&self) -> impl Future<Output = ()>;
    /// Update the cached session list in memory.
    fn upsert(&mut self);
}
