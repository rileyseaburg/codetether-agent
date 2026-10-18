//! Native session and live-registry effects for manifest recovery.
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_registry::{AgentEntry, Session};
/// Existing session loading and live registry operations.
/// # Implementors
/// Applications delegate to their current durable-session and registry owners.
#[async_trait]
pub trait RuntimeHost: Send + Sync {
    /// Native session represented by a durable manifest.
    type Session: Session + Send + Sync;
    /// Load a durable session for restoration, preserving native errors.
    async fn resume(id: &str) -> Result<Self::Session>;
    /// Clone an existing live entry by canonical ID.
    fn get(id: &str) -> Option<AgentEntry<Self::Session>>;
    /// Register and account for a restored live child.
    fn insert(entry: AgentEntry<Self::Session>);
    /// Register a child already covered by a pending residency reservation.
    fn insert_reserved(entry: AgentEntry<Self::Session>);
    /// Remove the live entry and its existing residency bookkeeping.
    fn remove(id: &str);
    /// Change matching live owner IDs after all durable migration succeeds.
    fn reparent_owner(from: &str, to: &str);
}
