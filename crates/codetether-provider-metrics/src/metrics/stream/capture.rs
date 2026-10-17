//! Poll-time TTFT capture and terminal stream recording.

use super::super::super::StreamChunk;
use super::MetricsStream;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

impl Stream for MetricsStream {
    type Item = StreamChunk;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let result = Pin::new(&mut self.inner).poll_next(cx);
        if self.ttft_ms.is_none() && is_output(&result) {
            self.ttft_ms = Some(self.start.elapsed().as_millis() as u64);
        }
        match &result {
            Poll::Ready(Some(StreamChunk::Done { usage })) if !self.recorded => {
                self.record(usage.clone(), true)
            }
            Poll::Ready(Some(StreamChunk::Error(_))) if !self.recorded => self.record(None, false),
            _ => {}
        }
        result
    }
}

fn is_output(result: &Poll<Option<StreamChunk>>) -> bool {
    matches!(result, Poll::Ready(Some(chunk)) if !matches!(
        chunk,
        StreamChunk::KeepAlive | StreamChunk::Done { .. } | StreamChunk::Error(_)
    ))
}
