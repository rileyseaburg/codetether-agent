//! Tests for [`super::next_with_keepalive`].

use super::*;
use futures::stream;

impl std::fmt::Debug for Next {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Next::Chunk(Some(_)) => write!(f, "Chunk(Some)"),
            Next::Chunk(None) => write!(f, "Chunk(None)"),
            Next::IdleTimeout => write!(f, "IdleTimeout"),
        }
    }
}

#[tokio::test(start_paused = true)]
async fn emits_keepalive_while_waiting_then_yields_chunk() {
    // Stream that produces its first chunk only after 25s of virtual time —
    // longer than the 10s keepalive tick, so keepalives must fire meanwhile.
    let mut s: BoxStream<'static, StreamChunk> = Box::pin(stream::once(async {
        tokio::time::sleep(Duration::from_secs(25)).await;
        StreamChunk::Text("hi".into())
    }));
    let (tx, mut rx) = tokio::sync::mpsc::channel(8);
    let next = next_with_keepalive(&mut s, Some(&tx), Duration::from_secs(180)).await;
    match next {
        Next::Chunk(Some(StreamChunk::Text(t))) => assert_eq!(t, "hi"),
        other => panic!("expected text chunk, got {other:?}"),
    }
    // Two 10s ticks elapsed before the 25s chunk → two keepalives.
    assert!(matches!(rx.try_recv(), Ok(SessionEvent::Thinking)));
    assert!(matches!(rx.try_recv(), Ok(SessionEvent::Thinking)));
    assert!(rx.try_recv().is_err());
}

#[tokio::test(start_paused = true)]
async fn reports_idle_timeout_when_budget_elapses() {
    // A stream that never produces anything.
    let mut s: BoxStream<'static, StreamChunk> = Box::pin(stream::pending());
    let (tx, _rx) = tokio::sync::mpsc::channel(8);
    let next = next_with_keepalive(&mut s, Some(&tx), Duration::from_secs(15)).await;
    assert!(matches!(next, Next::IdleTimeout));
}
