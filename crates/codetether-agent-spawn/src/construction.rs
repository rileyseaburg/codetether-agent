//! Application session construction and durable registration operations.
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_input::{Params, SpawnRequest};
use codetether_child_workspace::Handoff;
use codetether_collaboration_state::residency::Slot;
use codetether_tool_core::ToolResult;
/// Native session creation and persistence used by durable spawning.
/// # Implementors
/// Applications forward these operations to existing session/storage services.
#[async_trait]
pub trait Construction: Send + Sync {
    /// Native session created before durable registration.
    type Session: Send;
    /// Reserve resident capacity, returning the existing tool rejection on failure.
    async fn reserve_capacity() -> Result<Slot, ToolResult>;
    /// Create a child session and its retained checkout; propagate setup errors.
    async fn create_session(
        params: &Params,
        request: &SpawnRequest<'_>,
    ) -> Result<(Self::Session, Handoff)>;
    /// Save the session and register its durable identity; propagate write errors.
    async fn persist(request: &SpawnRequest<'_>, session: Self::Session) -> Result<String>;
}
