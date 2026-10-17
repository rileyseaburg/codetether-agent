//! Native session persistence, steering and publication effects.
use super::ApplicationHost;
use crate::{
    session::Session,
    tool::agent::{bus_publish, store},
};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_status::PublishedAgent;
use codetether_child_turn::FinalizationHost;
#[async_trait]
impl FinalizationHost for ApplicationHost {
    type Session = Session;
    fn clear_steering(agent_id: &str) {
        crate::session::helper::steering::clear(agent_id);
    }
    fn update_session(agent_id: &str, session: Session) {
        store::update_session(agent_id, session);
    }
    async fn save(session: &Session) -> Result<()> {
        session.save().await
    }
    fn lookup(agent_id: &str) -> Option<PublishedAgent> {
        store::get(agent_id).map(|entry| PublishedAgent {
            id: entry.session.id,
            name: entry.name,
            owner_session_id: entry.owner_session_id,
        })
    }
    fn announce_result(agent_id: &str, response: &str, error: Option<&str>) {
        bus_publish::announce_result(agent_id, response, error);
    }
    fn announce_done(agent_id: &str, success: bool, summary: String) {
        bus_publish::announce_done(agent_id, success, summary);
    }
}
