//! Completion and client disconnect release queue ownership.
use super::*;
#[tokio::test]
async fn lmstudio_dropping_stream_releases_active_slot() {
    let gate = gate();
    let stream = leased(
        futures::stream::pending().boxed(),
        gate.acquire().await.unwrap(),
    );
    assert!(gate.acquire().await.is_err());
    drop(stream);
    available(&gate).await;
}
#[tokio::test]
async fn lmstudio_terminal_stream_event_releases_slot_before_consumer_drop() {
    let gate = gate();
    let mut stream = leased(
        futures::stream::iter(vec![
            StreamChunk::Text("ok".into()),
            StreamChunk::Done { usage: None },
        ])
        .boxed(),
        gate.acquire().await.unwrap(),
    );
    assert!(matches!(stream.next().await, Some(StreamChunk::Text(_))));
    assert!(matches!(
        stream.next().await,
        Some(StreamChunk::Done { .. })
    ));
    assert!(gate.acquire().await.is_ok());
    assert!(stream.next().await.is_none());
}
