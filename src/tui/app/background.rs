use std::path::Path;

use tokio::sync::mpsc;

use crate::bus::BusHandle;
use crate::session::{Session, SessionEvent};
use crate::tui::app::session_events::handle_session_event;
use crate::tui::app::session_sync::refresh_sessions;
use crate::tui::app::state::App;
use crate::tui::app::worker_bridge::handle_processing_stopped;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::TuiWorkerBridge;

pub async fn drain_background_updates(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    bus_handle: &mut BusHandle,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    event_rx: &mut mpsc::Receiver<SessionEvent>,
    result_rx: &mut mpsc::Receiver<anyhow::Result<Session>>,
) {
    ingest_bus_messages(app, bus_handle);
    queue_worker_tasks(app, worker_bridge);
    display_next_worker_task(app);
    apply_completed_sessions(app, cwd, session, worker_bridge, result_rx).await;
    apply_session_events(app, session, worker_bridge, event_rx).await;
}

fn ingest_bus_messages(app: &mut App, bus_handle: &mut BusHandle) {
    while let Some(envelope) = bus_handle.try_recv() {
        app.state.bus_log.ingest(&envelope);
    }
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
    session: &mut Session,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    result_rx: &mut mpsc::Receiver<anyhow::Result<Session>>,
) {
    while let Ok(updated_session) = result_rx.try_recv() {
        apply_single_result(app, cwd, session, worker_bridge, updated_session).await;
    }
}

/// Process a single completed session result.
///
/// Extracted so both the `tokio::select!` branch in the event loop and the
/// batch drain path can share the same logic.
pub async fn apply_single_result(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    result: anyhow::Result<Session>,
) {
    match result {
        Ok(updated_session) => {
            // Always reset processing — the Done event may not have been
            // consumed yet via event_rx (tokio::select! race condition).
            if app.state.processing {
                handle_processing_stopped(app, worker_bridge).await;
                app.state.clear_request_timing();
            }
            *session = updated_session;
            app.state.session_id = Some(session.id.clone());
            let _ = session.save().await;
            refresh_sessions(app, cwd).await;
        }
        Err(err) => {
            handle_processing_stopped(app, worker_bridge).await;
            app.state
                .messages
                .push(ChatMessage::new(MessageType::Error, err.to_string()));
            app.state.status = "Request failed".to_string();
            app.state.scroll_to_bottom();
        }
    }
}

async fn apply_session_events(
    app: &mut App,
    session: &mut Session,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_rx: &mut mpsc::Receiver<SessionEvent>,
) {
    while let Ok(evt) = event_rx.try_recv() {
        handle_session_event(app, session, worker_bridge, evt).await;
    }
}
