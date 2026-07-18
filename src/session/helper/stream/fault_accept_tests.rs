//! Regression coverage for terminal faults after partial content.

use futures::stream;

use super::super::collect_stream_completion_with_events;
use crate::provider::StreamChunk;

#[tokio::test]
async fn partial_text_is_not_accepted_after_terminal_fault() {
    let chunks = stream::iter([
        StreamChunk::Text("must not commit".into()),
        StreamChunk::Error("processing your request; you can retry".into()),
    ]);
    let error = collect_stream_completion_with_events(Box::pin(chunks), None)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("stream faulted"));
}
