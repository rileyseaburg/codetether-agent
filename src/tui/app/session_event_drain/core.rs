use tokio::sync::mpsc;

use crate::session::SessionEvent;
use crate::tui::app::session_events::handle_session_event;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) async fn drain(
    app: &mut App,
    slot: &mut SessionSlot,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_rx: &mut mpsc::Receiver<SessionEvent>,
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
                flush_text(app, slot, worker_bridge, &mut latest_text).await;
                handle_session_event(app, slot, worker_bridge, other).await;
            }
        }
        if done && stop_on_done {
            break;
        }
    }
    flush_text(app, slot, worker_bridge, &mut latest_text).await;
}

async fn flush_text(
    app: &mut App,
    slot: &mut SessionSlot,
    worker_bridge: &Option<TuiWorkerBridge>,
    latest_text: &mut Option<String>,
) {
    if let Some(text) = latest_text.take() {
        handle_session_event(app, slot, worker_bridge, SessionEvent::TextChunk(text)).await;
    }
}
