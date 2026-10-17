//! Application-owned status subscriptions and parent activity.
use crate::{Activity, ThreadStatus};
use async_trait::async_trait;
use tokio::{sync::watch, time::Instant};
/// Snapshot of a child needed to prepare a status subscription.
///
/// # Examples
/// ```
/// let child=codetether_collaboration_tools::WaitAgent {id:"child".into(),is_processing:true};
/// assert!(child.is_processing);
/// ```
pub struct WaitAgent {
    /// Durable child identity.
    pub id: String,
    /// Whether a child turn is active.
    pub is_processing: bool,
}
/// Existing application state and notification services for collaboration waits.
///
/// # Implementors
/// Bind all methods to the current owner-scoped child and status registries.
#[async_trait]
pub trait WaitHost: Send + Sync {
    /// Resolve a child within the requesting parent's scope.
    fn find_agent(target: &str, owner: &str) -> Option<WaitAgent>;
    /// Subscribe to status for a child whose status has been initialized.
    fn subscribe(agent_id: &str) -> Option<watch::Receiver<ThreadStatus>>;
    /// Set the initial status when no subscription exists yet.
    fn set_status(agent_id: &str, status: ThreadStatus);
    /// Consume queued or future parent activity up to the supplied deadline.
    async fn activity(owner: &str, deadline: Instant) -> Option<Activity>;
}
