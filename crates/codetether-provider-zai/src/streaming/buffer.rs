//! Line buffering and capture for streaming SSE responses.
use crate::{StreamChunk, streaming::state::State};

pub(crate) fn append(state: &mut State, text: &str) -> Vec<StreamChunk> {
    state.buffer.push_str(crate::capture::record(text));
    let mut chunks = Vec::new();
    let mut text_buf = String::new();
    while let Some(end) = state.buffer.find('\n') {
        let line = state.buffer[..end].trim().to_string();
        state.buffer.drain(..=end);
        crate::streaming::frame::process(state, &line, &mut chunks, &mut text_buf);
    }
    if !text_buf.is_empty() {
        state.seen.text = true;
        chunks.push(StreamChunk::Text(text_buf));
    }
    chunks
}
