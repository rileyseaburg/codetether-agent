//! Explicit coordination failures remain downcastable through anyhow.
use std::fmt;

/// Admission and worker failures, distinct from SQLite and revision errors.
/// # Examples
/// ```
/// use codetether_session_store::coordinator::CoordinationError;
/// assert_eq!(CoordinationError::Full.to_string(), "SESSION_WRITE_QUEUE_FULL");
/// ```
#[derive(Debug)]
pub enum CoordinationError {
    /// This database already has 64 queued operations plus its running operation.
    Full,
    /// The worker or response channel disconnected; outcome may be unknown.
    Stopped,
    /// Registry synchronization was poisoned.
    RegistryPoisoned,
    /// Other processes retained the writer lock beyond thirty seconds.
    Busy,
    /// An operation panicked; its transaction was unwound, not acknowledged.
    Panicked,
    /// A callback left a transaction open; it was rolled back.
    Uncommitted,
    /// A writer callback tried to submit another coordinated operation.
    Reentrant,
}

/// Stable diagnostics for logs and callers.
/// # Examples
/// ```
/// use codetether_session_store::coordinator::CoordinationError;
/// assert_eq!(format!("{}", CoordinationError::Busy), "SESSION_WRITER_BUSY");
/// ```
impl fmt::Display for CoordinationError {
    /// Format a machine-identifiable diagnostic.
    /// # Examples
    /// ```
    /// use codetether_session_store::coordinator::CoordinationError;
    /// assert_eq!(CoordinationError::Full.to_string(), "SESSION_WRITE_QUEUE_FULL");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Full => "SESSION_WRITE_QUEUE_FULL",
            Self::Stopped => "SESSION_WRITER_STOPPED: accepted outcome may be unknown",
            Self::RegistryPoisoned => "SESSION_WRITER_REGISTRY_POISONED",
            Self::Busy => "SESSION_WRITER_BUSY",
            Self::Panicked => "SESSION_WRITE_PANICKED",
            Self::Uncommitted => "SESSION_WRITE_UNCOMMITTED",
            Self::Reentrant => "SESSION_WRITE_REENTRANT",
        })
    }
}

/// Expose coordination failures as standard error sources.
/// # Examples
/// ```
/// use codetether_session_store::coordinator::CoordinationError;
/// let error: &dyn std::error::Error = &CoordinationError::Full;
/// assert!(error.source().is_none());
/// ```
impl std::error::Error for CoordinationError {}
