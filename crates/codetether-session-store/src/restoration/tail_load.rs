//! Bounded indexed-session load result.

/// Result of a bounded session load.
/// # Examples
/// ```rust
/// let loaded = codetether_session_store::TailLoad { session: (), dropped: 0, file_bytes: 0 };
/// assert_eq!(loaded.dropped, 0);
/// ```
#[derive(Debug)]
pub struct TailLoad<S> {
    /// The session with its message/tool_use vectors capped to the window.
    pub session: S,
    /// Total number of messages + tool_use entries discarded from the tail
    /// cap. Zero means the full transcript fit within the window.
    pub dropped: usize,
    /// Size of the session file on disk, in bytes.
    pub file_bytes: u64,
}
