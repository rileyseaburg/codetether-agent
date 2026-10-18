//! Native live/durable state operations for kill and interrupt.
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_registry::{AgentEntry, Session};
/// Native state effects used by explicit child-control operations.
/// # Implementors
/// Applications preserve their existing owner-scoped lookup and removal APIs.
#[async_trait]
pub trait ControlHost: Send + Sync {
    /// Native child session stored in registry entries.
    type Session: Session + Send + Sync;
    /// Resolve a scoped target to its canonical ID and live entry.
    fn resolve(target: &str, parent: Option<&str>) -> Option<(String, AgentEntry<Self::Session>)>;
    /// Clone a live entry by canonical ID.
    fn get(agent_id: &str) -> Option<AgentEntry<Self::Session>>;
    /// Clear the durable mailbox; propagate write errors before further removal.
    async fn clear_mailbox(agent_id: &str) -> Result<()>;
    /// Remove durable child persistence; propagate errors before live removal.
    async fn remove(entry: &AgentEntry<Self::Session>) -> Result<()>;
    /// Announce a terminal result using the existing lifecycle bus.
    fn announce_done(agent_id: &str, success: bool, summary: &str);
    /// Remove the live entry, then clear its in-flight trace.
    fn remove_runtime(agent_id: &str);
}
