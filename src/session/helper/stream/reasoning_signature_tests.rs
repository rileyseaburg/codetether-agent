use super::collect_stream_completion_with_events;
use crate::provider::{ContentPart, StreamChunk};
use futures::stream;

#[tokio::test]
async fn opaque_reasoning_signature_reaches_the_next_turn() {
    let signature =
        "codetether:openai-reasoning:{\"type\":\"reasoning\",\"encrypted_content\":\"opaque\"}";
    let chunks = vec![
        StreamChunk::Thinking(signature.to_string()),
        StreamChunk::Done { usage: None },
    ];
    let response = collect_stream_completion_with_events(Box::pin(stream::iter(chunks)), None)
        .await
        .expect("completion");

    assert!(matches!(
        response.message.content.as_slice(),
        [ContentPart::Thinking { text, signature: Some(value) }]
            if text.is_empty() && value.contains("opaque")
    ));
}
