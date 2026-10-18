//! Bounded SSE buffering errors.

/// Failure to accept a bounded UTF-8 SSE chunk.
/// # Examples
/// ```
/// use codetether_worker_events::Error;
/// match Error::InvalidUtf8 { Error::InvalidUtf8 => {}, Error::Oversized => {} }
/// ```
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Chunk bytes are not a complete valid UTF-8 string.
    #[error("SSE chunk is not valid UTF-8")]
    InvalidUtf8,
    /// The buffered data would exceed one MiB.
    #[error("SSE partial frame exceeded 1 MiB")]
    Oversized,
}
