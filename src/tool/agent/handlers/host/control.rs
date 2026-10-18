//! Bind owner lookup and durable/live removal to existing application stores.
use super::ApplicationHost;
use crate::tool::agent::collaboration_runtime::message_queue;
use crate::tool::agent::{bus_publish, event_loop, persistence, store};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_lifecycle::ControlHost;
#[async_trait]
impl ControlHost for ApplicationHost {
    type Session = crate::session::Session;
    fn resolve(target: &str, parent: Option<&str>) -> Option<(String, store::AgentEntry)> {
        store::scope::resolve_for_parent(target, parent)
    }
    fn get(agent_id: &str) -> Option<store::AgentEntry> {
        store::get(agent_id)
    }
    async fn clear_mailbox(agent_id: &str) -> Result<()> {
        message_queue::clear(agent_id).await
    }
    async fn remove(entry: &store::AgentEntry) -> Result<()> {
        persistence::remove(entry).await
    }
    fn announce_done(agent_id: &str, success: bool, summary: &str) {
        bus_publish::announce_done(agent_id, success, summary);
    }
    fn remove_runtime(agent_id: &str) {
        store::remove(agent_id);
        event_loop::live_trace::clear(agent_id);
    }
}
