//! Application session storage operations for conversation continuity.
use anyhow::Result;
use async_trait::async_trait;

/// Session storage operations consumed by the conversation resolver.
///
/// # Implementors
/// Applications supply their session type and existing persistence operations.
#[async_trait]
pub trait ConversationStore: Sync {
    /// Application-owned conversation state.
    type Session: Send;
    /// Create a new session with its normal generated identifier.
    /// # Errors
    /// Returns the application's session-creation error.
    async fn create(&self) -> Result<Self::Session>;
    /// Load a session by context identifier.
    /// # Errors
    /// Returns the application's loading error.
    async fn resume(&self, id: &str) -> Result<Self::Session>;
    /// Whether persisted state exists after a failed resume attempt.
    /// # Errors
    /// Returns errors deriving or checking the session location.
    fn exists(&self, id: &str) -> Result<bool>;
    /// Pin a newly created session to the validated conversation identifier.
    fn set_id(&self, session: &mut Self::Session, id: &str);
}
