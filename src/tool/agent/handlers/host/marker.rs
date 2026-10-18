//! Append and persist developer markers through normal session methods.
use super::ApplicationHost;
use crate::{provider::Message, session::Session, tool::agent::store};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_lifecycle::MarkerHost;
#[async_trait]
impl MarkerHost for ApplicationHost {
    fn add_message(session: &mut Session, message: Message) {
        session.add_message(message);
    }
    async fn save(session: &Session) -> Result<()> {
        session.save().await
    }
    fn update_session(agent_id: &str, session: Session) {
        store::update_session(agent_id, session);
    }
}
