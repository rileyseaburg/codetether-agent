use std::path::Path;

use tokio::sync::mpsc;

use crate::bus::recorder::BusRecorder;
use crate::session::SessionEvent;
use crate::tui::app::session_runtime::{SessionNotice, SessionSlot};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

#[path = "background/host/mod.rs"]
mod host;

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
    bus_recorder: &BusRecorder,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    event_rx: &mut mpsc::Receiver<SessionEvent>,
    notice_rx: &mut mpsc::Receiver<SessionNotice>,
) {
    codetether_tui_tick::background::drain(&mut host::Native {
        app,
        cwd,
        slot,
        bus_recorder: Some(bus_recorder),
        worker_bridge,
        event_rx,
        notice_rx: Some(notice_rx),
    })
    .await;
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
    codetether_tui_tick::background::apply_notice(
        &mut host::Native {
            app,
            cwd,
            slot,
            bus_recorder: None,
            worker_bridge,
            event_rx,
            notice_rx: None,
        },
        notice,
    )
    .await;
}
