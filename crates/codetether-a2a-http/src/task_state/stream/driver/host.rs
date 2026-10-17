//! Application operations needed by the shared stream driver.
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

/// Session and provider operations used to execute one streamed task.
///
/// # Implementors
/// Applications resolve their own session types and provider registries, map
/// events to observers, execute prompts and persist successful sessions.
#[async_trait]
pub trait StreamSessionHost: Send {
    /// Application-owned session state.
    type Session: Send;
    /// Provider registry passed into prompt execution.
    type Registry: Send;
    /// Events produced by the session.
    type Event: Send + 'static;
    /// Resolve an existing or new session for the supplied context.
    /// # Errors
    /// Returns a session construction or loading error.
    async fn resolve(&mut self, context_id: Option<&str>) -> Result<Self::Session>;
    /// Apply application model and runtime settings.
    async fn configure(&mut self, session: &mut Self::Session);
    /// Start observation before the provider registry is loaded.
    fn spawn_events(&self, receiver: mpsc::Receiver<Self::Event>);
    /// Load providers, reporting failure through the application's existing path.
    async fn registry(&mut self) -> Option<Self::Registry>;
    /// Execute the prompt and return its final text.
    /// # Errors
    /// Returns the application's prompt execution error.
    async fn execute(
        &mut self,
        session: &mut Self::Session,
        prompt: &str,
        sender: mpsc::Sender<Self::Event>,
        registry: Self::Registry,
    ) -> Result<String>;
    /// Best-effort persistence after successful prompt execution.
    async fn persist(&mut self, session: &mut Self::Session);
}
