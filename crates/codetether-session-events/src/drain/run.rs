use super::Handler;
use crate::SessionEvent;
use tokio::sync::mpsc;
/// Drain queued events with bounded work and cumulative-text coalescing.
/// # Arguments
/// `handler` applies events, `event_rx` supplies already queued events,
/// `limit` bounds consumption, and `stop_on_done` stops after a Done event.
/// # Returns
/// Completes after flushing the latest text, without waiting for new events.
/// # Examples
/// ```no_run
/// # async fn consume<S: Send>(handler: &mut impl codetether_session_events::drain::Handler<S>, rx: &mut tokio::sync::mpsc::Receiver<codetether_session_events::SessionEvent<S>>) {
/// codetether_session_events::drain::run(handler, rx, 64, false).await;
/// # }
/// ```
pub async fn run<S: Send>(
    handler: &mut impl Handler<S>,
    event_rx: &mut mpsc::Receiver<SessionEvent<S>>,
    limit: usize,
    stop_on_done: bool,
) {
    let mut latest_text = None;
    for _ in 0..limit {
        let Ok(evt) = event_rx.try_recv() else { break };
        let done = matches!(evt, SessionEvent::Done);
        match evt {
            SessionEvent::TextChunk(text) => latest_text = Some(text),
            other => {
                flush_text(handler, &mut latest_text).await;
                handler.handle(other).await;
            }
        }
        if done && stop_on_done {
            break;
        }
    }
    flush_text(handler, &mut latest_text).await;
}
async fn flush_text<S: Send>(handler: &mut impl Handler<S>, latest_text: &mut Option<String>) {
    if let Some(text) = latest_text.take() {
        handler.handle(SessionEvent::TextChunk(text)).await;
    }
}
