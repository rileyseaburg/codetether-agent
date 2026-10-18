use codetether_provider_core::Message;
/// Native session and durable-prompt lookup used for continuation.
/// # Implementors
/// UI adapters retain session slots and durable goal prompt lookup.
pub trait Source {
    /// Native session type exposing the transcript.
    type Session;
    /// Clone the currently in-flight prompt.
    fn inflight(&self) -> Option<String>;
    /// Borrow the available session, if it is not checked out.
    fn session(&self) -> Option<&Self::Session>;
    /// Resolve a durable continuation prompt for that session.
    fn durable_prompt(session: &Self::Session) -> Option<String>;
    /// Borrow the loaded transcript.
    fn messages(session: &Self::Session) -> &[Message];
    /// Whether the UI reports a running request.
    fn is_processing(&self) -> bool;
}
