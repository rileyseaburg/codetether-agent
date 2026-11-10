//! Signature-only reasoning remains activity, not visible text or replay state.
use super::{frame::event, server};
use codetether_provider_core::StreamChunk;
use serde_json::json;

#[tokio::test]
async fn fable_converse_signature_retains_existing_heartbeat_behavior() {
    let first = event(
        "contentBlockDelta",
        json!({"contentBlockIndex":0,"delta":{"reasoningContent":{"signature":"fixture-signature"}}}),
    );
    let tail = event(
        "contentBlockDelta",
        json!({"contentBlockIndex":1,"delta":{"text":"answer"}}),
    );
    let mut fixture = server::open(first, tail).await;
    assert!(
        matches!(server::next(&mut fixture.stream).await, StreamChunk::Thinking(t) if t.is_empty())
    );
    assert!(!fixture.task.is_finished());
    fixture.release.send(()).unwrap();
    assert!(
        matches!(server::next(&mut fixture.stream).await, StreamChunk::Text(t) if t == "answer")
    );
    assert!(matches!(
        server::next(&mut fixture.stream).await,
        StreamChunk::Done { .. }
    ));
    fixture.task.join().unwrap();
}
