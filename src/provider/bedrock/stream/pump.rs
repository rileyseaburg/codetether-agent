//! Byte-to-chunk pump for the Bedrock converse-stream adapter.
//!
//! Feeds the HTTP byte stream through a [`FrameBuffer`], dispatches decoded
//! eventstream frames to [`super::handle_event`], and terminates with
//! [`StreamChunk::Done`] carrying any usage captured from `metadata`.
//!
//! A stream that stops with `max_tokens` while emitting **no visible
//! content** (encrypted-reasoning models can burn the whole budget on
//! signature-only thinking) ends in [`StreamChunk::Error`] so callers retry
//! or surface the failure instead of recording an empty assistant message.

use super::StreamState;
use crate::provider::StreamChunk;
use crate::provider::bedrock::eventstream::FrameBuffer;
use futures::StreamExt;

/// Convert a successful converse-stream HTTP response into a boxed
/// [`StreamChunk`] stream. Transport and frame-decode failures surface as
/// [`StreamChunk::Error`] and end the stream.
pub(super) fn event_chunk_stream(
    response: reqwest::Response,
) -> futures::stream::BoxStream<'static, StreamChunk> {
    let mut byte_stream = response.bytes_stream();
    let mut framer = FrameBuffer::new();
    let mut state = StreamState::default();
    let mut saw_visible_content = false;

    let stream = async_stream::stream! {
        while let Some(chunk) = byte_stream.next().await {
            match chunk {
                Ok(bytes) => framer.extend(&bytes),
                Err(e) => {
                    yield StreamChunk::Error(format!("transport error: {e}"));
                    return;
                }
            }

            loop {
                match framer.next_frame() {
                    Ok(Some(msg)) => {
                        for out in super::handle_event(&mut state, msg) {
                            saw_visible_content |= matches!(
                                out,
                                StreamChunk::Text(_) | StreamChunk::ToolCallStart { .. }
                            );
                            yield out;
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        yield StreamChunk::Error(format!("frame decode: {e}"));
                        return;
                    }
                }
            }
        }

        if !saw_visible_content && state.stop_reason.as_deref() == Some("max_tokens") {
            yield StreamChunk::Error(
                crate::provider::bedrock::empty_guard::EMPTY_MAX_TOKENS_MSG.to_string(),
            );
            return;
        }
        yield StreamChunk::Done { usage: state.usage };
    };

    Box::pin(stream)
}
