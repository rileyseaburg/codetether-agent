//! Typed failures kept independent of HTTP status or browser presentation.

/// Native goal changes fail closed on stale identity or unavailable storage.
#[derive(Debug, thiserror::Error)]
pub enum GoalControlError {
    /// Invalid input or a control requiring human authority.
    #[error("invalid goal edit: {0}")]
    Invalid(&'static str),
    /// Stale goal identity or revision, or a conflicting lifecycle hold.
    #[error("goal edit conflict: {0}")]
    Conflict(&'static str),
    /// The journal could not be read or appended.
    #[error(transparent)]
    Storage(#[from] anyhow::Error),
}
