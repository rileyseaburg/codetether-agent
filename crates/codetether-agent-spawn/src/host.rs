//! Application validation, identity and execution operations for spawning.
use crate::Construction;
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_input::SpawnRequest;
use codetether_spawn_policy::Reservation;
use codetether_tool_core::ToolResult;
/// Runtime operations surrounding durable construction and ephemeral execution.
/// # Implementors
/// Applications implement [Construction] and forward existing runtime operations.
#[async_trait]
pub trait Host: Construction {
    /// Validate depth and cost eligibility, returning any advisory warning.
    async fn validate(request: &SpawnRequest<'_>) -> Result<Option<String>, ToolResult>;
    /// Reserve a live/durable owner-scoped identity, returning existing rejections.
    async fn reserve_identity(name: &str, owner: Option<&str>) -> Result<Reservation, ToolResult>;
    /// Dispatch a durable child's first turn with the requested detach behavior.
    async fn kick_off(agent_id: &str, detach: bool) -> Result<ToolResult>;
    /// Execute a one-shot child without durable registration, preserving errors.
    async fn ephemeral(request: &SpawnRequest<'_>, warning: Option<&str>) -> Result<ToolResult>;
}
