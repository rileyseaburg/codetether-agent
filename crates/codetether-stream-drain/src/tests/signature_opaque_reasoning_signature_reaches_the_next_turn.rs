//! Original single-drain regression case.
use super::collect_stream_completion_with_events;

use codetether_provider_core::{ContentPart, StreamChunk};

use futures::stream;

#[tokio::test]
async fn opaque_reasoning_signature_reaches_the_next_turn() {
    let signature =
        "codetether:openai-reasoning:{\"type\":\"reasoning\",\"encrypted_content\":\"opaque\"}";
    let chunks = vec![
        StreamChunk::Thinking(signature.to_string()),
        StreamChunk::Done { usage: None },
    ];
    let (tx, mut rx) = tokio::sync::mpsc::channel(4);
    let response = collect_stream_completion_with_events(Box::pin(stream::iter(chunks)), Some(&tx))
        .await
        .expect("completion");
    assert!(matches!(rx.try_recv(), Ok(super::SessionEvent::Thinking)));
    assert!(rx.try_recv().is_err());

    assert!(matches!(
        response.message.content.as_slice(),
        [ContentPart::Thinking { text, signature: Some(value) }]
            if text.is_empty() && value.contains("opaque")
    ));
}
