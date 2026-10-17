//! Native validation, identity and first-turn execution services.
use super::ApplicationHost;
use crate::tool::agent::spawn::{ephemeral, identity};
use crate::tool::agent::{spawn_run, spawn_validation};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_input::SpawnRequest;
use codetether_agent_spawn::Host;
use codetether_spawn_policy::Reservation;
use codetether_tool_core::ToolResult;
#[async_trait]
impl Host for ApplicationHost {
    async fn validate(request: &SpawnRequest<'_>) -> Result<Option<String>, ToolResult> {
        spawn_validation::validate_spawn_request(request).await
    }
    async fn reserve_identity(name: &str, owner: Option<&str>) -> Result<Reservation, ToolResult> {
        identity::reserve(name, owner).await
    }
    async fn kick_off(agent_id: &str, detach: bool) -> Result<ToolResult> {
        spawn_run::kick_off(agent_id, detach).await
    }
    async fn ephemeral(request: &SpawnRequest<'_>, warning: Option<&str>) -> Result<ToolResult> {
        ephemeral::run(request, warning).await
    }
}
