//! Separation of GLM reasoning from answer text in streaming deltas.
//!
//! GLM models stream private thinking in `reasoning_content` alongside the real
//! answer in `content`. Concatenating both into one buffer published the
//! model's reasoning as its answer, observed live as a reply beginning
//! "The user wants me to run a specific glob tool ... Let me do that."
//!
//! Reasoning is preserved as [`StreamChunk::Thinking`] rather than dropped: it
//! is genuine model output and it refreshes the session watchdog during long
//! silent phases.

use crate::provider::StreamChunk;

/// Routes a reasoning delta, flushing any pending answer text first.
///
/// Ordering matters: text buffered before the reasoning arrived belongs to the
/// answer, so it is emitted before the thinking chunk to keep the transcript in
/// wire order.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::zai::delta::push_reasoning;
/// use codetether_agent::provider::StreamChunk;
///
/// let mut chunks = Vec::new();
/// let mut buffered = String::from("partial answer");
/// push_reasoning(&mut chunks, &mut buffered, "deciding next step");
///
/// assert!(buffered.is_empty());
/// assert!(matches!(&chunks[0], StreamChunk::Text(t) if t == "partial answer"));
/// assert!(matches!(&chunks[1], StreamChunk::Thinking(t) if t == "deciding next step"));
/// ```
pub fn push_reasoning(chunks: &mut Vec<StreamChunk>, text_buf: &mut String, reasoning: &str) {
    if !text_buf.is_empty() {
        chunks.push(StreamChunk::Text(std::mem::take(text_buf)));
    }
    chunks.push(StreamChunk::Thinking(reasoning.to_string()));
}

#[cfg(test)]
#[path = "zai_stream_delta_tests.rs"]
mod tests;
