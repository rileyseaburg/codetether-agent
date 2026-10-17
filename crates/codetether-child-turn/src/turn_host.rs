//! Native child startup and mailbox completion operations.
use crate::FinalizationHost;
use anyhow::Result;
use async_trait::async_trait;
use codetether_child_events::Host;
use codetether_collaboration_state::execution::AgentRunGuard;
use tokio::{sync::mpsc, task::JoinHandle};
/// Native event receiver and owned child task returned from startup.
pub type Started<E, S> = (mpsc::Receiver<E>, JoinHandle<Result<S>>);
/// Effects surrounding collection of a started child turn.
/// # Implementors
/// Combine native event observation with finalization, startup and mailbox effects.
#[async_trait]
pub trait TurnHost: FinalizationHost + Host + Send + Sync + 'static {
    /// Prepare and launch the first turn; preserve native startup errors.
    async fn begin(agent_id: &str) -> Result<Started<Self::SourceEvent, Self::Session>>;
    /// Complete a mailbox receipt and release/transfer the existing execution guard.
    async fn finished(agent_id: String, receipt: Option<String>, guard: AgentRunGuard);
}
