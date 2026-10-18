//! Typed edit-branch failures, separate from HTTP formatting and storage operations.
/// A stale target must not silently edit a different message.
/// # Examples
/// ```
/// use codetether_session_fork::ForkError;
/// assert!(matches!(ForkError::Conflict, ForkError::Conflict));
/// ```
#[derive(Debug, thiserror::Error)]
pub enum ForkError {
    #[error("The original message changed. Reload the conversation and retry.")]
    Conflict,
    #[error("Could not create edited conversation")]
    Storage(#[from] anyhow::Error),
}
