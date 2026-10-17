//! Native capacity, session creation and durable registration.
use super::ApplicationHost;
use crate::tool::agent::spawn::session;
use crate::tool::agent::{residency, spawn_store};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_input::{Params, SpawnRequest};
use codetether_agent_spawn::Construction;
use codetether_child_workspace::Handoff;
use codetether_collaboration_state::residency::Slot;
use codetether_tool_core::ToolResult;
#[async_trait]
impl Construction for ApplicationHost {
    type Session = crate::session::Session;
    async fn reserve_capacity() -> Result<Slot, ToolResult> {
        residency::reserve(None).await
    }
    async fn create_session(
        params: &Params,
        request: &SpawnRequest<'_>,
    ) -> Result<(Self::Session, Handoff)> {
        session::create(params, request).await
    }
    async fn persist(request: &SpawnRequest<'_>, session: Self::Session) -> Result<String> {
        spawn_store::persist_spawned_agent(request, session).await
    }
}
