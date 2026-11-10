//! A gated HTTP body must deliver text before the remaining response or EOF.
use super::{frame::event, server};
use codetether_provider_core::StreamChunk;
use futures::StreamExt;
use serde_json::json;

#[tokio::test]
async fn fable_converse_text_arrives_before_response_finishes() {
    let first = event(
        "contentBlockDelta",
        json!({"contentBlockIndex":0,"delta":{"text":"first"}}),
    );
    let tail = [
        event(
            "contentBlockDelta",
            json!({"contentBlockIndex":0,"delta":{"text":" second"}}),
        ),
        event("messageStop", json!({"stopReason":"end_turn"})),
        event(
            "metadata",
            json!({"usage":{"inputTokens":10,"outputTokens":2,"totalTokens":12}}),
        ),
    ]
    .concat();
    let mut fixture = server::open(first, tail).await;
    assert!(
        matches!(server::next(&mut fixture.stream).await, StreamChunk::Text(t) if t == "first")
    );
    assert!(!fixture.task.is_finished(), "HTTP EOF must remain gated");
    fixture.release.send(()).unwrap();
    assert!(
        matches!(server::next(&mut fixture.stream).await, StreamChunk::Text(t) if t == " second")
    );
    let StreamChunk::Done { usage: Some(usage) } = server::next(&mut fixture.stream).await else {
        panic!("expected terminal usage");
    };
    assert_eq!(
        (
            usage.prompt_tokens,
            usage.completion_tokens,
            usage.total_tokens
        ),
        (10, 2, 12)
    );
    assert!(fixture.stream.next().await.is_none());
    fixture.task.join().unwrap();
}
