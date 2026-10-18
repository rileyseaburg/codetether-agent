//! Restored-session ownership and identity bindings.
/// Session operations required immediately after restoring a snapshot.
/// # Implementors
/// Application sessions supply bus attachment and their owned identifier.
pub trait Session {
    /// Attach the process-wide bus only if it is missing.
    fn attach_bus(&mut self);
    /// Copy the restored session identifier.
    fn session_id(&self) -> String;
}
/// Frontend session ownership for snapshot synchronization.
/// # Implementors
/// Frontends restore into their existing slot and expose its current session.
pub trait Sync {
    /// Concrete restored session.
    type Session: Session;
    /// Restore the runtime snapshot into the frontend slot.
    fn restore(&mut self, session: Self::Session);
    /// Borrow the restored session when available.
    fn current_session(&mut self) -> Option<&mut Self::Session>;
    /// Store the restored session identifier in frontend state.
    fn set_session_id(&mut self, id: String);
}
