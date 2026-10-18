use crate::Call;
use codetether_provider_core::Message;
use codetether_session_events::SessionEvent;
use codetether_tool_outcome::Outcome;
use std::future::Future;
use tokio::sync::mpsc;
/// Native destinations and services used to publish tool results.
/// # Implementors
/// The prompt runner owns bus, archive, RLM routing and session persistence.
pub trait Host: Send {
    /// Native snapshot type used in stream events.
    type Snapshot: Send;
    /// Optional live event sender.
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent<Self::Snapshot>>>;
    /// Publish the completed response on the agent bus.
    fn broadcast(&self, step: usize, call: &Call, outcome: &Outcome);
    /// Schedule archival of the completed response.
    fn archive(&self, call: &Call, outcome: &Outcome);
    /// Route large output through existing RLM services.
    fn route(&self, call: &Call, content: &str) -> String;
    /// Insert the resulting tool message.
    fn append(&mut self, message: Message);
    /// Persist after tool-message insertion.
    /// # Errors
    /// Propagates the existing post-tool persistence failure.
    fn persist(&mut self) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Record whether the completed tool produced an empty code search.
    fn record_codesearch(&mut self, missed: bool);
}
