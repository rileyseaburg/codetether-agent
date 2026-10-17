//! Native session persistence and lifecycle publication for a settled turn.
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_status::PublishedAgent;
/// Existing application effects used while finalizing child output.
/// # Implementors
/// Applications forward these operations to their current session and bus owners.
#[async_trait]
pub trait FinalizationHost: Send + Sync {
    /// Native session returned by a child task.
    type Session: Clone + Send + Sync + 'static;
    /// Clear pending steering state for the child.
    fn clear_steering(agent_id: &str);
    /// Replace the live registry's session snapshot.
    fn update_session(agent_id: &str, session: Self::Session);
    /// Save the updated native session and preserve its storage error.
    async fn save(session: &Self::Session) -> Result<()>;
    /// Read current child publication fields from the live registry.
    fn lookup(agent_id: &str) -> Option<PublishedAgent>;
    /// Send the child's result to its owner using existing bus delivery.
    fn announce_result(agent_id: &str, response: &str, error: Option<&str>);
    /// Publish the terminal lifecycle update after parent notification.
    fn announce_done(agent_id: &str, success: bool, summary: String);
}
