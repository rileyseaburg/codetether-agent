use std::path::Path;

use tokio::sync::mpsc;

use crate::bus::BusHandle;
use crate::session::SessionEvent;
use crate::tui::app::session_runtime::{SessionNotice, SessionSlot};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

#[path = "background_notice.rs"]
mod notice;
#[path = "background_retry.rs"]
mod runtime_retry;
#[path = "stream_disconnect.rs"]
pub(crate) mod stream_disconnect;
#[path = "stream_reconnect.rs"]
pub(crate) mod stream_reconnect;
#[path = "stream_reconnect_execute.rs"]
pub(crate) mod stream_reconnect_execute;
#[path = "background_worker_tasks.rs"]
mod worker_tasks;

pub(crate) async fn drain_background_updates(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    bus_handle: &mut BusHandle,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    event_rx: &mut mpsc::Receiver<SessionEvent>,
    notice_rx: &mut mpsc::Receiver<SessionNotice>,
) {
    app.state.drain_model_refresh();
    crate::tui::app::bus::ingest::drain(app, bus_handle);
    worker_tasks::queue_worker_tasks(app, worker_bridge);
    worker_tasks::display_next_worker_task(app);
    crate::tui::app::session_event_drain::drain_batch(app, slot, worker_bridge, event_rx).await;
    apply_completed_sessions(app, cwd, slot, worker_bridge, event_rx, notice_rx).await;
}

async fn apply_completed_sessions(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    event_rx: &mut mpsc::Receiver<crate::session::SessionEvent>,
    notice_rx: &mut mpsc::Receiver<SessionNotice>,
) {
    while let Ok(notice) = notice_rx.try_recv() {
        crate::tui::app::session_event_drain::drain_before_notice(
            app,
            slot,
            worker_bridge,
            event_rx,
        )
        .await;
        notice::apply(app, cwd, slot, worker_bridge, notice).await;
    }
}

/// Process a single completed session result.
pub(crate) async fn apply_single_notice(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    event_rx: &mut mpsc::Receiver<crate::session::SessionEvent>,
    notice: SessionNotice,
) {
    crate::tui::app::session_event_drain::drain_before_notice(app, slot, worker_bridge, event_rx)
        .await;
    notice::apply(app, cwd, slot, worker_bridge, notice).await;
}
