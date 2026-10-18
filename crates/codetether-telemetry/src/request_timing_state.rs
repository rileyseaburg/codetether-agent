//! Request timing and completed-turn statistics.
//! Shared timing state and operations are independent of application events.

use std::time::Instant;
/// Timing history for one request stream and its completed turns.
///
/// # Examples
/// ```
/// let state = codetether_telemetry::RequestTimingState::default();
/// assert!(state.processing_started_at.is_none());
/// ```
#[derive(Debug, Clone, Default)]
pub struct RequestTimingState {
    /// Clock anchor of the current request.
    pub processing_started_at: Option<Instant>,
    /// First token latency for the current request.
    pub current_request_first_token_ms: Option<u64>,
    /// Most recent token latency for the current request.
    pub current_request_last_token_ms: Option<u64>,
    /// First token latency retained from the completed request.
    pub last_request_first_token_ms: Option<u64>,
    /// Last token latency retained from the completed request.
    pub last_request_last_token_ms: Option<u64>,
    /// Rolling end-to-end statistics for completed turns.
    pub chat_latency: crate::chat_latency::ChatLatencyStats,
}
