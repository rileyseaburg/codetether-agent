//! Application persistence, activation and live-session bindings.
use super::super::{event_loop, persistence, residency, store};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_lifecycle::{Host, ResumeConfig};
use codetether_collaboration_tools::EnsureOpen;

/// Binds shared lifecycle orchestration to application session services.
pub struct ApplicationHost;
#[async_trait]
impl Host for ApplicationHost {
    type Session = crate::session::Session;
    async fn durable_id(owner: Option<&str>, target: &str) -> Result<Option<String>> {
        persistence::durable_id(owner, target).await
    }
    async fn prepare_resume(
        owner: Option<&str>,
        target: &str,
    ) -> Result<(String, store::AgentEntry)> {
        let closed = persistence::prepare_resume(owner, target).await?;
        Ok((closed.name, closed.entry))
    }
    async fn close(name: &str, entry: &store::AgentEntry) -> Result<()> {
        persistence::close(name, entry).await
    }
    async fn child_ids(parent: &str) -> Result<Vec<String>> {
        persistence::child_ids(parent).await
    }
    async fn open(target: &str, owner: Option<&str>, config: ResumeConfig) -> Result<EnsureOpen> {
        residency::open(target, owner, config).await
    }
    fn get(id: &str) -> Option<store::AgentEntry> {
        store::get(id)
    }
    fn remove_runtime(id: &str) {
        store::remove(id);
        event_loop::live_trace::clear(id);
    }
}
