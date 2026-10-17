//! Application persistence and live-session operations used by lifecycle flows.
use crate::ResumeConfig;
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_registry::{AgentEntry, Session};
use codetether_collaboration_tools::EnsureOpen;

/// Application operations needed for durable close and resident activation.
/// # Implementors
/// Implement this for the application's existing session and persistence APIs.
/// The lifecycle crate owns sequencing; methods perform the named operation.
#[async_trait]
pub trait Host: Send + Sync {
    /// Application session stored in each live registry entry.
    type Session: Session + Send + Sync;
    /// Resolve an owner-scoped durable ID. Errors propagate from persistence.
    async fn durable_id(owner: Option<&str>, target: &str) -> Result<Option<String>>;
    /// Load the name and entry needed for close, preserving lookup errors.
    async fn prepare_resume(
        owner: Option<&str>,
        target: &str,
    ) -> Result<(String, AgentEntry<Self::Session>)>;
    /// Persist the closed state; returns the underlying write error on failure.
    async fn close(name: &str, entry: &AgentEntry<Self::Session>) -> Result<()>;
    /// Discover immediate durable child IDs; returns discovery errors unchanged.
    async fn child_ids(parent: &str) -> Result<Vec<String>>;
    /// Activate one root using the supplied live configuration.
    /// Returns activation errors or an explicit ready/missing/rejected outcome.
    async fn open(target: &str, owner: Option<&str>, config: ResumeConfig) -> Result<EnsureOpen>;
    /// Clone a resident entry by its durable ID.
    fn get(id: &str) -> Option<AgentEntry<Self::Session>>;
    /// Remove the resident entry, then clear its live trace.
    fn remove_runtime(id: &str);
}
