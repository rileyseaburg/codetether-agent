use super::SessionEvent;
use super::helper::stream::collect_stream_completion_with_events;
use super::helper::text::extract_text_content;
use crate::provider::{ContentPart, FinishReason, StreamChunk, Usage};
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

#[tokio::test]
async fn collect_stream_completion_reconstructs_tool_calls() {
    let stream = Box::pin(stream::iter(vec![
        StreamChunk::ToolCallStart {
            id: "call_1".to_string(),
            name: "read".to_string(),
        },
        StreamChunk::ToolCallDelta {
            id: "call_1".to_string(),
            arguments_delta: "{\"path\":\"src/main.rs\"}".to_string(),
        },
        StreamChunk::ToolCallEnd {
            id: "call_1".to_string(),
        },
        StreamChunk::Done { usage: None },
    ]));

    let response = collect_stream_completion_with_events(stream, None)
        .await
        .expect("stream should collect");

    assert_eq!(response.finish_reason, FinishReason::ToolCalls);
    assert!(matches!(
        response.message.content.first(),
        Some(ContentPart::ToolCall { id, name, arguments, .. })
            if id == "call_1" && name == "read" && arguments == "{\"path\":\"src/main.rs\"}"
    ));
}
