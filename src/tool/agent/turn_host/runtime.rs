//! Native startup and mailbox completion for child turns.
use super::ApplicationHost;
use crate::session::{Session, SessionEvent};
use anyhow::Result;
use async_trait::async_trait;
use codetether_child_turn::{Started, TurnHost};
use codetether_collaboration_state::execution::AgentRunGuard;
#[async_trait]
impl TurnHost for ApplicationHost {
    async fn begin(agent_id: &str) -> Result<Started<SessionEvent, Session>> {
        crate::tool::agent::spawn_run::start::begin(agent_id).await
    }
    async fn finished(agent_id: String, receipt: Option<String>, guard: AgentRunGuard) {
        crate::tool::agent::collaboration_runtime::message_queue::finished(
            agent_id, receipt, guard,
        )
        .await;
    }
}
