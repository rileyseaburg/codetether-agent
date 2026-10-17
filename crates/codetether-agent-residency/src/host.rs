//! Application-specific prepared activation and live-session overlays.
use crate::{ResumeConfig, Storage};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_registry::AgentEntry;

/// Bind prepared durable activation and session edits to residency policy.
/// # Implementors
/// Implement [Storage] first, then forward these session-specific operations.
#[async_trait]
pub trait Host: Storage {
    /// Prepared durable entry, including application persistence metadata.
    type Prepared: Send;
    /// Load a durable entry for activation; propagate lookup/load errors.
    async fn prepare_resume(owner: Option<&str>, target: &str) -> Result<Self::Prepared>;
    /// Borrow the live entry inside a prepared activation.
    fn entry_mut(prepared: &mut Self::Prepared) -> &mut AgentEntry<Self::Session>;
    /// Commit activation and registration; preserve persistence rollback errors.
    async fn activate_resume(prepared: Self::Prepared) -> Result<AgentEntry<Self::Session>>;
    /// Apply live-parent settings and save changed sessions; propagate save errors.
    async fn overlay(entry: &mut AgentEntry<Self::Session>, config: &ResumeConfig) -> Result<()>;
}
