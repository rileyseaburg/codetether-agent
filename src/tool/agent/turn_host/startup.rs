//! Native child lookup, session policy preparation and working-state effects.
use super::ApplicationHost;
use crate::{
    session::Session,
    tool::agent::{bus_publish, store},
};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_input::Params;
use codetether_agent_session::ChildSession;
use codetether_child_turn::StartupHost;
#[async_trait]
impl StartupHost for ApplicationHost {
    fn first_session(agent_id: &str) -> Option<(Session, String)> {
        store::get(agent_id).map(|entry| (entry.session, entry.instructions))
    }
    async fn message_session(agent_id: &str, params: &Params) -> Result<Session> {
        let session = store::get_for_parent(agent_id, params.parent_session_id.as_deref())
            .map(|entry| entry.session);
        codetether_agent_session::prepare_message(agent_id, params, session).await
    }
    fn ensure_bus(session: &mut Session) {
        ChildSession::ensure_bus(session);
    }
    fn open_steering(agent_id: &str) {
        crate::session::helper::steering::open(agent_id);
    }
    fn announce_working(agent_id: &str, summary: &'static str) {
        bus_publish::announce_working(agent_id, summary);
    }
}
