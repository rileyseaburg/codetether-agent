//! Native session restoration and live-registry mutation.
use super::ApplicationHost;
use crate::{session::Session, tool::agent::store};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_storage::RuntimeHost;
#[async_trait]
impl RuntimeHost for ApplicationHost {
    type Session = Session;
    async fn resume(id: &str) -> Result<Session> {
        Session::resume(id).await
    }
    fn get(id: &str) -> Option<store::AgentEntry> {
        store::get(id)
    }
    fn insert(entry: store::AgentEntry) {
        store::insert(entry);
    }
    fn insert_reserved(entry: store::AgentEntry) {
        store::insert_reserved(entry);
    }
    fn remove(id: &str) {
        store::remove(id);
    }
    fn reparent_owner(from: &str, to: &str) {
        store::reparent_owner(from, to);
    }
}
