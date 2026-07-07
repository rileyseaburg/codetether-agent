//! [`Stream`] implementation for [`SseChunkStream`].
//!
//! Separated from the struct definition to keep each file under 50 lines.
//!
//! ## EOF handling
//!
//! Anthropic's SSE wire protocol emits a `message_delta` event carrying
//! `StreamChunk::Done` **before** the HTTP body closes. So a clean stream
//! looks like: … `Done`(from message_delta) … `Ready(None)`.
//!
//! When the HTTP body closes (`Ready(None)`):
//! - If a real `Done` was already seen (`saw_done == true`): yield `None`
//!   to end the stream cleanly with no duplicate chunk.
//! - If no `Done` was seen (`saw_done == false`): the connection dropped
//!   before the model signalled completion. Yield a transient
//!   `StreamChunk::Error` so the SRP drain classifies this as
//!   `StreamStop::Fault { transient: true }` and fires a restart instead
//!   of silently accepting a truncated reply as `StreamStop::Clean`.

use futures::Stream;

use super::sse_stream::SseChunkStream;
use crate::provider::StreamChunk;

impl Stream for SseChunkStream {
    type Item = StreamChunk;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        loop {
            if let Some(pos) = self.buffer.find('\n') {
                let line = self.buffer[..pos].to_string();
                self.buffer = self.buffer[pos + 1..].to_string();
                if let Some(chunk) = self.process_line(&line) {
                    if matches!(chunk, StreamChunk::Done { .. }) {
                        self.saw_done = true;
                    }
                    return std::task::Poll::Ready(Some(chunk));
                }
                continue;
            }
            match self.inner.as_mut().poll_next(cx) {
                std::task::Poll::Ready(Some(Ok(bytes))) => {
                    self.buffer.push_str(&String::from_utf8_lossy(&bytes));
                    continue;
                }
                std::task::Poll::Ready(Some(Err(e))) => {
                    return std::task::Poll::Ready(Some(StreamChunk::Error(e.to_string())));
                }
                std::task::Poll::Ready(None) if self.saw_done => {
                    // Clean close after a real Done — end the stream normally.
                    return std::task::Poll::Ready(None);
                }
                std::task::Poll::Ready(None) => {
                    // Byte stream closed before any Done: premature EOF.
                    return std::task::Poll::Ready(Some(StreamChunk::Error(
                        "Anthropic stream closed before message_stop (premature EOF)".into(),
                    )));
                }
                std::task::Poll::Pending => return std::task::Poll::Pending,
            }
        }
    }
}
