//! Ordered streaming completion events before bus publication.
use codetether_session_events::SessionEvent;
use tokio::sync::mpsc;
pub(super) async fn emit<S>(
    events: Option<&mpsc::Sender<SessionEvent<S>>>,
    thinking: &str,
    text: &str,
) {
    if let Some(tx) = events {
        if !thinking.is_empty() {
            let _ = tx
                .send(codetether_session_events::SessionEvent::ThinkingComplete(
                    thinking.to_string(),
                ))
                .await;
        }
        if !text.is_empty() {
            let _ = tx
                .send(codetether_session_events::SessionEvent::TextChunk(
                    text.to_string(),
                ))
                .await;
            let _ = tx
                .send(codetether_session_events::SessionEvent::TextComplete(
                    text.to_string(),
                ))
                .await;
        }
    }
}
