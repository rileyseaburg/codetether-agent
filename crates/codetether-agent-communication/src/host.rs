//! Native registry, steering and session effects used by message delivery.
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::Message;
/// Session access supplied by the application runtime.
/// # Implementors
/// Implementations must use the same registry and steering inbox as live turns.
#[async_trait]
pub trait Host: Send + Sync {
    /// Owned session snapshot used for an idle message write.
    type Session: Send + Sync;
    /// Resolve a name or ID within the caller's owner scope.
    fn resolve(target: &str, owner: Option<&str>) -> Option<String>;
    /// Read a live session by its canonical child ID.
    fn session(id: &str) -> Option<Self::Session>;
    /// Attempt to deliver plain text to an active steering inbox.
    fn push(id: &str, message: &str) -> bool;
    /// Append a user message using the application's human-message bookkeeping.
    fn add_human_message(session: &mut Self::Session, message: Message);
    /// Save a modified session, propagating the native persistence error.
    async fn save(session: &Self::Session) -> Result<()>;
    /// Replace the live snapshot only after persistence succeeds.
    fn update_session(id: &str, session: Self::Session);
}
