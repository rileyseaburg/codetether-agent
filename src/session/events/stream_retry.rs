//! Ephemeral state for replacing a failed model-stream attempt.

/// Describes one whole-request reconnect after a retryable stream failure.
///
/// # Examples
///
/// ```
/// use codetether_agent::session::StreamRetryEvent;
///
/// let retry = StreamRetryEvent {
///     attempt: 1,
///     max_restarts: 3,
///     reason: "connection reset".into(),
/// };
/// assert_eq!(retry.attempt, 1);
/// ```
#[derive(Debug, Clone)]
pub struct StreamRetryEvent {
    /// One-based reconnect attempt number.
    pub attempt: u32,
    /// Maximum reconnects allowed for this sampling request.
    pub max_restarts: u32,
    /// Diagnostic stop classification from the failed attempt.
    pub reason: String,
}
