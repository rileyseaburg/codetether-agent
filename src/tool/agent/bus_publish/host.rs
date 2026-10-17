//! Publication fields from the existing live-child registry.
use crate::tool::agent::store;
use codetether_agent_status::{PublishHost, PublishedAgent};
/// Native live-child source for lifecycle publication.
pub struct ApplicationHost;
impl PublishHost for ApplicationHost {
    fn get(agent_id: &str) -> Option<PublishedAgent> {
        store::get(agent_id).map(|entry| PublishedAgent {
            id: entry.session.id,
            name: entry.name,
            owner_session_id: entry.owner_session_id,
        })
    }
}
