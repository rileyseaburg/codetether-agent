//! Stalled clients and truncated upstream responses terminate predictably.
use super::*;
#[tokio::test]
async fn lmstudio_stalled_consumer_does_not_hold_slot_past_deadline() {
    let gate = gate();
    let stream = leased(
        futures::stream::repeat(StreamChunk::Text("x".into())).boxed(),
        gate.acquire().await.unwrap(),
    );
    tokio::time::sleep(Duration::from_millis(100)).await;
    available(&gate).await;
    assert!(gate.acquire().await.is_ok());
    let chunks: Vec<_> = stream.collect().await;
    assert!(
        matches!(chunks.last(), Some(StreamChunk::Error(message)) if message.contains("deadline"))
    );
    assert!(chunks.len() <= 9);
}
#[tokio::test]
async fn lmstudio_eof_without_done_is_an_error() {
    let chunks: Vec<_> = leased(
        futures::stream::empty().boxed(),
        gate().acquire().await.unwrap(),
    )
    .collect()
    .await;
    assert!(
        matches!(chunks.last(), Some(StreamChunk::Error(message)) if message.contains("completion marker"))
    );
}
