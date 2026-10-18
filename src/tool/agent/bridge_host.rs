//! Native live-registry and transcript access for shared agent views.
use crate::{provider::Message, session::Session, tool::agent::store};
use codetether_agent_view::Host;
/// Native session snapshots used by the shared agent view.
pub struct ApplicationHost;
impl Host for ApplicationHost {
    type Session = Session;
    fn entries(parent: Option<&str>) -> Vec<store::AgentEntry> {
        store::entries_for_parent(parent)
    }
    fn get_for_parent(name: &str, parent: &str) -> Option<store::AgentEntry> {
        store::get_for_parent(name, Some(parent))
    }
    fn message_count(session: &Session) -> usize {
        session.messages.len()
    }
    fn into_messages(session: Session) -> Vec<Message> {
        session.messages.into_vec()
    }
}
impl codetether_peer_agent::LocalNames for ApplicationHost {
    fn contains(name: &str) -> bool {
        store::contains_name(name, None)
    }
}
