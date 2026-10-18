//! Agent views over application registry lookup and shared session snapshots.
//!
//! Implement [`Source`] for the live registry. [`ApplicationHost`] supplies
//! loaded-message counting, transcript extraction and local-name lookup.
use codetether_agent_registry::{AgentEntry, Session};
use codetether_provider_core::Message;
use codetether_session_store::{Restore, Snapshot};
use std::marker::PhantomData;

/// Live registry operations retained by the embedding application.
/// # Implementors
/// Applications preserve their parent resolution and residency-aware lookup.
pub trait Source {
    /// Session snapshot carried by each live registry entry.
    type Session: Session + Restore;
    /// Clone entries in the requested parent scope, or all entries for `None`.
    fn entries(parent: Option<&str>) -> Vec<AgentEntry<Self::Session>>;
    /// Resolve a child inside the specified parent scope.
    fn get_for_parent(name: &str, parent: &str) -> Option<AgentEntry<Self::Session>>;
    /// Test whether a local name exists in any owner scope.
    fn contains(name: &str) -> bool;
}

/// Shared view and local-name contracts for a registry source.
/// # Examples
/// ```
/// # fn example<S: codetether_agent_view::application::Source>() {
/// fn accepts_view<H: codetether_agent_view::Host>() {}
/// accepts_view::<codetether_agent_view::application::ApplicationHost<S>>();
/// # }
/// ```
pub struct ApplicationHost<S>(PhantomData<fn() -> S>);

impl<S: Source> crate::Host for ApplicationHost<S> {
    type Session = S::Session;
    fn entries(parent: Option<&str>) -> Vec<AgentEntry<S::Session>> {
        S::entries(parent)
    }
    fn get_for_parent(name: &str, parent: &str) -> Option<AgentEntry<S::Session>> {
        S::get_for_parent(name, parent)
    }
    fn message_count(session: &S::Session) -> usize {
        session.messages().len()
    }
    fn into_messages(mut session: S::Session) -> Vec<Message> {
        std::mem::take(session.messages_mut()).into_vec()
    }
}

impl<S: Source> codetether_peer_agent::LocalNames for ApplicationHost<S> {
    fn contains(name: &str) -> bool {
        S::contains(name)
    }
}
