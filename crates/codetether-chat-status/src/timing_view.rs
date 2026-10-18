//! Request timing values used by the status bar.
/// Snapshot of current and last-request latency values.
///
/// # Examples
/// ```
/// let timings = codetether_chat_status::RequestTimings::default();
/// assert!(!timings.processing);
/// ```
#[derive(Default, Clone, Copy)]
pub struct RequestTimings {
    /// Whether a request is active.
    pub processing: bool,
    /// First-token latency for the active request.
    pub current_first_token_ms: Option<u64>,
    /// First-token latency for the last completed request.
    pub last_first_token_ms: Option<u64>,
    /// Last-token latency for the last completed request.
    pub last_last_token_ms: Option<u64>,
}
