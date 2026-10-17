//! Native persistence operations required by durable registration ordering.
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_registry::{AgentEntry, Session};
/// Durable and live stores used to register a newly created child.
/// # Implementors
/// Applications delegate writes and insertion to their existing store owners.
#[async_trait]
pub trait Registration: Send + Sync {
    /// Native session with the canonical durable identity.
    type Session: Session + Send + Sync;
    /// Save the session before constructing its live entry; propagate write errors.
    async fn save_session(session: &Self::Session) -> Result<()>;
    /// Derive parent display name and child depth from the live registry.
    fn lineage(parent_id: Option<&str>) -> (Option<String>, u8);
    /// Save the durable child manifest; propagate its write error.
    async fn save_entry(name: &str, entry: &AgentEntry<Self::Session>) -> Result<()>;
    /// Insert the entry already covered by a pending residency slot.
    fn insert_reserved(entry: AgentEntry<Self::Session>);
}
