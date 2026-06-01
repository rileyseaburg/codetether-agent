//! Streaming text size cap helper.

pub(super) fn bounded_chunk(chunk: String) -> String {
    if chunk.len() <= crate::tui::constants::MAX_STREAMING_TEXT_BYTES {
        return chunk;
    }
    let mut text =
        crate::util::truncate_bytes_safe(&chunk, crate::tui::constants::MAX_STREAMING_TEXT_BYTES)
            .to_string();
    text.push_str(" …[truncated]");
    text
}
