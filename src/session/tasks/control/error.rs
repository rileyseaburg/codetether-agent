//! Typed failures kept independent of HTTP status or browser presentation.

/// Native goal changes fail closed on stale identity or unavailable storage.
#[derive(Debug, thiserror::Error)]
pub(crate) enum GoalControlError {
    #[error("invalid goal edit: {0}")]
    Invalid(&'static str),
    #[error("goal edit conflict: {0}")]
    Conflict(&'static str),
    #[error(transparent)]
    Storage(#[from] anyhow::Error),
}
