//! Live registry and durable storage operations used by residency policy.
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_registry::{AgentEntry, Session};

/// Bind residency policy to existing live and durable agent stores.
/// # Implementors
/// Applications forward these operations to their canonical stores.
#[async_trait]
pub trait Storage: Send + Sync {
    /// Live application session type carried by registry entries.
    type Session: Session + Send + Sync;
    /// Clone a live entry by canonical ID, returning none when absent.
    fn get(id: &str) -> Option<AgentEntry<Self::Session>>;
    /// Resolve a path, ID, or unambiguous name in the given owner scope.
    fn resolve(target: &str, owner: Option<&str>) -> Option<(String, AgentEntry<Self::Session>)>;
    /// Replace a live entry and update its existing residency bookkeeping.
    fn insert(entry: AgentEntry<Self::Session>);
    /// Remove the live entry and residency bookkeeping, then clear its trace.
    fn remove_runtime(id: &str);
    /// Look up a durable ID; propagate persistence errors unchanged.
    async fn durable_id(owner: Option<&str>, target: &str) -> Result<Option<String>>;
    /// Persist closed lifecycle state; propagate write errors unchanged.
    async fn close(name: &str, entry: &AgentEntry<Self::Session>) -> Result<()>;
}
