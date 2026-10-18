/// Direct active-turn control supplied by the embedding runtime.
/// # Implementors
/// Implementations keep their existing reservation, cancellation and inbox policy.
pub trait Control {
    /// Input accepted by the active turn's steering inbox.
    type Input;
    /// Reserve a session before transferring its prompt to the command channel.
    fn prepare(&self, session_id: &str) -> bool;
    /// Cancel directly, returning whether an active executor was notified.
    fn notify(&self) -> bool;
    /// Clear an active reservation after failed handoff.
    fn clear(&self);
    /// Submit input to the currently active prompt.
    fn steer(&self, input: Self::Input) -> bool;
}
