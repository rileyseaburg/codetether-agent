//! Regression tests for first-output latency capture.

use super::MetricsStream;
use crate::provider::StreamChunk;
use futures::{StreamExt, stream};

#[tokio::test]
async fn ttft_is_captured_when_first_output_is_polled() {
    let inner = Box::pin(stream::iter(vec![
        StreamChunk::Text("ready".into()),
        StreamChunk::Done { usage: None },
    ]));
    let start = std::time::Instant::now() - std::time::Duration::from_millis(50);
    let mut measured = MetricsStream::new(inner, "test".into(), "model".into(), start);
    assert!(measured.ttft_ms.is_none());
    assert!(matches!(measured.next().await, Some(StreamChunk::Text(_))));
    assert!(measured.ttft_ms.is_some_and(|millis| millis >= 40));
}
