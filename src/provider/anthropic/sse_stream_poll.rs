//! [`Stream`] implementation for [`SseChunkStream`].
//!
//! Separated from the struct definition to keep each file under 50 lines.

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
                std::task::Poll::Ready(None) => {
                    return std::task::Poll::Ready(Some(StreamChunk::Done { usage: None }));
                }
                std::task::Poll::Pending => return std::task::Poll::Pending,
            }
        }
    }
}
