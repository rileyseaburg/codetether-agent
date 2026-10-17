//! Live-registry and durable-store bindings for residency policy.
use super::super::super::{event_loop, persistence, store};
use super::ApplicationHost;
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_residency::Storage;
#[async_trait]
impl Storage for ApplicationHost {
    type Session = crate::session::Session;
    fn get(id: &str) -> Option<store::AgentEntry> {
        store::get(id)
    }
    fn resolve(target: &str, owner: Option<&str>) -> Option<(String, store::AgentEntry)> {
        store::scope::resolve_for_parent(target, owner)
    }
    fn insert(entry: store::AgentEntry) {
        store::insert(entry);
    }
    fn remove_runtime(id: &str) {
        store::remove(id);
        event_loop::live_trace::clear(id);
    }
    async fn durable_id(owner: Option<&str>, target: &str) -> Result<Option<String>> {
        persistence::durable_id(owner, target).await
    }
    async fn close(name: &str, entry: &store::AgentEntry) -> Result<()> {
        persistence::close(name, entry).await
    }
}
