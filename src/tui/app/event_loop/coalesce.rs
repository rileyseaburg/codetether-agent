use tokio::sync::mpsc;

use crate::session::SessionEvent;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub async fn drain(
    app: &mut App,
    slot: &mut SessionSlot,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    event_rx: &mut mpsc::Receiver<SessionEvent>,
) {
    crate::tui::app::session_event_drain::drain_batch(app, slot, worker_bridge, event_rx).await;
}
