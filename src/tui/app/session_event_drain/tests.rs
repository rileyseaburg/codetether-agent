use super::*;
use crate::tui::app::session_runtime::SessionSlot;

async fn slot() -> SessionSlot {
    let session = crate::session::Session::new().await.expect("session");
    SessionSlot::new(session)
}

#[tokio::test]
async fn batch_keeps_latest_cumulative_text_chunk() {
    let (tx, mut rx) = mpsc::channel(8);
    tx.send(SessionEvent::TextChunk("hel".to_string()))
        .await
        .unwrap();
    tx.send(SessionEvent::TextChunk("hello".to_string()))
        .await
        .unwrap();
    let mut app = App::default();
    let mut slot = slot().await;

    drain_batch(&mut app, &mut slot, &None, &mut rx).await;

    assert_eq!(app.state.streaming_text, "hello");
}

#[tokio::test]
async fn batch_leaves_later_events_for_redraw() {
    let (tx, mut rx) = mpsc::channel(DRAIN_BATCH_LIMIT + 2);
    for i in 0..=DRAIN_BATCH_LIMIT {
        tx.send(SessionEvent::TextChunk(i.to_string()))
            .await
            .unwrap();
    }
    let mut app = App::default();
    let mut slot = slot().await;

    drain_batch(&mut app, &mut slot, &None, &mut rx).await;

    let expected = (DRAIN_BATCH_LIMIT - 1).to_string();
    assert_eq!(app.state.streaming_text, expected);
    assert!(matches!(rx.try_recv(), Ok(SessionEvent::TextChunk(_))));
}

#[tokio::test]
async fn notice_drain_flushes_done_before_notice() {
    let (tx, mut rx) = mpsc::channel(8);
    tx.send(SessionEvent::TextChunk("done".to_string()))
        .await
        .unwrap();
    tx.send(SessionEvent::Done).await.unwrap();
    let mut app = App::default();
    app.state.processing_started_at = Some(std::time::Instant::now());
    let mut slot = slot().await;

    drain_before_notice(&mut app, &mut slot, &None, &mut rx).await;

    assert_eq!(app.state.status, "Ready");
    assert!(app.state.processing_started_at.is_none());
}
