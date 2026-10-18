//! Incremental text snapshots and usage from the shared stream collector.
use super::{SessionEvent, collect_stream_completion_with_events};
use codetether_context_policy::transcript::extract_text_content;
use codetether_provider_core::{FinishReason, StreamChunk, Usage};
use futures::stream;
use tokio::sync::mpsc;

#[tokio::test]
async fn collect_stream_completion_emits_incremental_text_chunks() {
    let (tx, mut rx) = mpsc::channel(8);
    let stream = Box::pin(stream::iter(vec![
        StreamChunk::Text("hel".to_string()),
        StreamChunk::Text("lo".to_string()),
        StreamChunk::Done {
            usage: Some(Usage {
                prompt_tokens: 3,
                completion_tokens: 5,
                total_tokens: 8,
                ..Default::default()
            }),
        },
    ]));

    let response = collect_stream_completion_with_events(stream, Some(&tx))
        .await
        .expect("stream should collect");

    let first = rx.recv().await.expect("first chunk");
    let second = rx.recv().await.expect("second chunk");

    assert!(matches!(first, SessionEvent::TextChunk(text) if text == "hel"));
    assert!(matches!(second, SessionEvent::TextChunk(text) if text == "hello"));
    assert_eq!(extract_text_content(&response.message.content), "hello");
    assert_eq!(response.usage.total_tokens, 8);
    assert_eq!(response.finish_reason, FinishReason::Stop);
}
