//! Native session snapshots used by read-only agent views.
use codetether_agent_registry::{AgentEntry, Session};
use codetether_provider_core::Message;
/// Existing registry and transcript access, with no execution side effects.
/// # Implementors
/// Applications delegate to their current owner-scoped live registry.
pub trait Host {
    /// Native session type with canonical identity.
    type Session: Session;
    /// Clone live entries for the optional owner scope.
    fn entries(parent: Option<&str>) -> Vec<AgentEntry<Self::Session>>;
    /// Resolve a target only in its requested parent scope.
    fn get_for_parent(name: &str, parent: &str) -> Option<AgentEntry<Self::Session>>;
    /// Return the message count of an owned session snapshot.
    fn message_count(session: &Self::Session) -> usize;
    /// Extract messages from the cloned native session.
    fn into_messages(session: Self::Session) -> Vec<Message>;
}
