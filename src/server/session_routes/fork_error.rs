//! Typed edit-branch failures, separate from HTTP formatting and storage operations.
/// A stale target must not silently edit a different message.
#[derive(Debug, thiserror::Error)]
pub(super) enum ForkError {
    #[error("The original message changed. Reload the conversation and retry.")]
    Conflict,
    #[error("Could not create edited conversation")]
    Storage(#[from] anyhow::Error),
}
