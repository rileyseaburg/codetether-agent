//! Streaming text size cap helper.

/// Bound the cumulative stream preview at a UTF-8 boundary.
/// # Arguments
/// Receives owned text and the configured byte limit.
/// # Returns
/// Original or truncated text with the existing truncation marker.
pub fn bounded_chunk(chunk: String, max_bytes: usize) -> String {
    if chunk.len() <= max_bytes {
        return chunk;
    }
    let mut text = codetether_util::truncate_bytes_safe(&chunk, max_bytes).to_string();
    text.push_str(" …[truncated]");
    text
}
