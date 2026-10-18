//! Native storage and execution contract for persisted-session HTTP handlers.
use codetether_session_events::SessionResult;
use codetether_session_metadata::SessionSummary;
/// Supply session storage and prompt execution to the HTTP layer.
/// # Implementors
/// Application session types implement this without copying HTTP policy.
#[async_trait::async_trait]
pub trait SessionHost: serde::Serialize + Sized + Send + Sync + 'static {
    /// Create an empty session.
    /// # Errors
    /// Returns initialization or storage failures.
    async fn new() -> anyhow::Result<Self>;
    /// Load the requested durable session.
    /// # Errors
    /// Returns missing or invalid session errors.
    async fn load(id: &str) -> anyhow::Result<Self>;
    /// List persisted session summaries before pagination.
    /// # Errors
    /// Returns listing/storage failures.
    async fn list() -> anyhow::Result<Vec<SessionSummary>>;
    /// Replace the optional session title.
    fn set_title(&mut self, title: Option<String>);
    /// Set the requested session agent.
    fn set_agent_name(&mut self, agent: String);
    /// Persist the current session.
    /// # Errors
    /// Returns durable storage failures.
    async fn save(&self) -> anyhow::Result<()>;
    /// Execute a user prompt.
    /// # Errors
    /// Returns prompt execution failures.
    async fn prompt(&mut self, message: &str) -> anyhow::Result<SessionResult>;
}
