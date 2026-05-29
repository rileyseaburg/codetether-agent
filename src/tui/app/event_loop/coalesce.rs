use tokio::sync::mpsc;

use crate::session::{Session, SessionEvent};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub async fn drain(
    app: &mut App,
    session: &mut Session,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    event_rx: &mut mpsc::Receiver<SessionEvent>,
) {
    for _ in 0..64 {
        match event_rx.try_recv() {
            Ok(evt) => {
                crate::tui::app::session_events::handle_session_event(
                    app,
                    session,
                    worker_bridge,
                    evt,
                )
                .await;
            }
            Err(_) => break,
        }
    }
}
