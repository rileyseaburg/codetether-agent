//! Native session storage and continuation operations for startup.
use anyhow::Result;
use codetether_bus_core::AgentBus;
use codetether_session_store::TailLoad;
use std::sync::Arc;
/// Session operations required by startup selection.
/// # Implementors
/// Applications retain their session type, storage configuration and fork policy.
#[allow(async_fn_in_trait)]
pub trait Host {
    /// Native session owned after resolution.
    type Session;
    /// Load a bounded session by its explicit identifier.
    async fn load_tail(id: &str, window: usize) -> Result<TailLoad<Self::Session>>;
    /// Create a fresh session.
    async fn new_session() -> Result<Self::Session>;
    /// Current human-readable session label.
    fn label(session: &Self::Session) -> String;
    /// Attach the startup bus.
    fn with_bus(session: Self::Session, bus: Arc<AgentBus>) -> Self::Session;
    /// Read session identity for diagnostics.
    fn id(session: &Self::Session) -> &str;
    /// Count messages in the session's effective history.
    fn history_len(session: &Self::Session) -> usize;
    /// Fork a truncated history when required and return its original source ID.
    async fn fork_if_truncated(
        session: &mut Self::Session,
        dropped: usize,
    ) -> Result<Option<String>>;
}
