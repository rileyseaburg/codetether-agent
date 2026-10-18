//! Native session and provider operations used by realtime prompt execution.
use codetether_session_events::{SessionEvent, SessionResult};
use codetether_session_metadata::SessionMetadata;
use std::{future::Future, sync::Arc};
use tokio::sync::mpsc;
/// Session storage and provider bindings for a realtime prompt task.
/// # Implementors
/// Native applications retain their existing session and provider implementations.
pub trait Host: Send + Sync + 'static {
    /// Session type also carried in snapshot events.
    type Session: Send + Sync + 'static;
    /// Shared provider registry used for prompt execution.
    type Registry: Send + Sync + 'static;
    /// Load the requested session identity.
    fn load(id: &str) -> impl Future<Output = anyhow::Result<Self::Session>> + Send;
    /// Access the session's existing metadata for explicit model selection.
    fn metadata(session: &mut Self::Session) -> &mut SessionMetadata;
    /// Load the application's shared provider registry.
    fn load_registry() -> impl Future<Output = anyhow::Result<Arc<Self::Registry>>> + Send;
    /// Execute a prompt and stream session events to the supplied channel.
    fn prompt(
        session: &mut Self::Session,
        message: &str,
        events: mpsc::Sender<SessionEvent<Self::Session>>,
        registry: Arc<Self::Registry>,
    ) -> impl Future<Output = anyhow::Result<SessionResult>> + Send;
    /// Persist the session after a successful prompt.
    fn save(session: &Self::Session) -> impl Future<Output = anyhow::Result<()>> + Send;
}
