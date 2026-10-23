//! Owned cumulative reply text and throughput counters.
use std::time::Instant;
/// Live assistant text and timing values, independent of the UI container.
///
/// # Examples
/// ```
/// let state = codetether_chat_model::StreamingState::default();
/// assert!(state.streaming_text.is_empty());
/// ```
#[derive(Default)]
pub struct StreamingState {
    /// Cumulative assistant reply.
    pub streaming_text: String,
    /// Readable provider reasoning received so far, separate from the reply.
    pub streaming_reasoning: String,
    /// Start of throughput measurement.
    pub streaming_start: Option<Instant>,
    /// Cumulative streamed character count.
    pub streaming_chars: usize,
}
