//! Native session, provider and identity services for the shared server.
use anyhow::Result;
use codetether_session_events::{SessionEvent, SessionResult};
use std::future::Future;
use tokio::sync::mpsc;
/// Services used while executing an admitted A2A request.
/// # Implementors
/// Applications retain their session configuration, prompt policy and persistence.
pub trait Application: Send + Sync + 'static {
    /// Session state carried by prompts and snapshot events.
    type Session: Send + 'static;
    /// Provider registry passed to streamed prompts.
    type Registry: Send + 'static;
    /// Package version used in the default agent card.
    fn version() -> &'static str;
    /// Resolve a context to an existing or new session.
    /// # Errors
    /// Returns session resolution failures.
    fn resolve(context_id: Option<&str>) -> impl Future<Output = Result<Self::Session>> + Send;
    /// Apply native session configuration before execution.
    fn configure(session: &mut Self::Session) -> impl Future<Output = ()> + Send;
    /// Load the application's shared provider registry.
    /// # Errors
    /// Returns provider discovery failures.
    fn registry() -> impl Future<Output = Result<Self::Registry>> + Send;
    /// Execute a prompt with session events and an existing provider registry.
    /// # Errors
    /// Returns prompt execution failures.
    fn execute(
        session: &mut Self::Session,
        prompt: &str,
        sender: mpsc::Sender<SessionEvent<Self::Session>>,
        registry: Self::Registry,
    ) -> impl Future<Output = Result<String>> + Send;
    /// Execute a headless prompt through the native runtime.
    /// # Errors
    /// Returns prompt execution failures.
    fn prompt(
        session: &mut Self::Session,
        prompt: &str,
    ) -> impl Future<Output = Result<SessionResult>> + Send;
    /// Persist the session using the application's existing best-effort policy.
    fn persist(session: &Self::Session) -> impl Future<Output = ()> + Send;
}
