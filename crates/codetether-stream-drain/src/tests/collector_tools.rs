//! Tool-call reconstruction from streamed start, delta and end records.
use super::collect_stream_completion_with_events;
use codetether_provider_core::{ContentPart, FinishReason, StreamChunk};
use futures::stream;

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
