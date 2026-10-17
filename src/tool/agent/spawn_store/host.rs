//! Native session, manifest and live-store registration operations.
use crate::session::Session;
use crate::tool::agent::{persistence, store};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_spawn::Registration;
/// Application stores supplying shared durable registration.
pub struct ApplicationStore;
#[async_trait]
impl Registration for ApplicationStore {
    type Session = Session;
    async fn save_session(session: &Session) -> Result<()> {
        session.save().await
    }
    fn lineage(parent_id: Option<&str>) -> (Option<String>, u8) {
        store::lineage_for_session(parent_id)
    }
    async fn save_entry(name: &str, entry: &store::AgentEntry) -> Result<()> {
        persistence::save(name, entry).await
    }
    fn insert_reserved(entry: store::AgentEntry) {
        store::insert_reserved(entry);
    }
}
