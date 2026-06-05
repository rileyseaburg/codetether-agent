use std::path::Path;

use tokio::sync::mpsc;

use crate::bus::BusHandle;
use crate::session::SessionEvent;
use crate::tui::app::session_runtime::{SessionNotice, SessionSlot};
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::TuiWorkerBridge;

#[path = "background_notice.rs"]
mod notice;
#[path = "background_retry.rs"]
mod runtime_retry;

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
    ingest_bus_messages(app, bus_handle);
    queue_worker_tasks(app, worker_bridge);
    display_next_worker_task(app);
    crate::tui::app::session_event_drain::drain_batch(app, slot, worker_bridge, event_rx).await;
    apply_completed_sessions(app, cwd, slot, worker_bridge, event_rx, notice_rx).await;
}

fn ingest_bus_messages(app: &mut App, bus_handle: &mut BusHandle) {
    crate::tui::app::bus::ingest::drain(app, bus_handle);
}

fn queue_worker_tasks(app: &mut App, worker_bridge: &mut Option<TuiWorkerBridge>) {
    if let Some(bridge) = worker_bridge.as_mut() {
        while let Ok(task) = bridge.task_rx.try_recv() {
            let preview = format!(
                "{} from {}",
                task.task_id,
                task.from_agent
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string())
            );
            app.state.push_recent_task(preview);
            app.state.enqueue_worker_task(task);
            app.state.status = format!(
                "Queued remote task(s): {}",
                app.state.worker_task_queue.len()
            );
        }
    }
}

fn display_next_worker_task(app: &mut App) {
    if !app.state.processing
        && let Some(task) = app.state.dequeue_worker_task()
    {
        let from_agent = task.from_agent.unwrap_or_else(|| "unknown".to_string());
        app.state.messages.push(ChatMessage::new(
            MessageType::System,
            format!(
                "Incoming A2A task {} from {}\n{}",
                task.task_id, from_agent, task.message
            ),
        ));
        app.state.status = format!("Received remote task {}", task.task_id);
        app.state.scroll_to_bottom();
    }
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
///
/// Extracted so both the `tokio::select!` branch in the event loop and the
/// batch drain path can share the same logic.
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
