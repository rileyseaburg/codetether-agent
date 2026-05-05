use std::path::Path;

use crate::session::Session;
use crate::tui::app::session_sync::refresh_sessions;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::TuiWorkerBridge;

pub async fn apply(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    result: anyhow::Result<Session>,
) {
    match result {
        Ok(updated) if updated.id != session.id => {
            tracing::warn!(stale_id = %updated.id, "Discarding stale session result");
            let _ = updated.save().await;
            refresh_sessions(app, cwd).await;
        }
        Ok(updated) => save_current(app, cwd, session, worker_bridge, updated).await,
        Err(err) => {
            crate::tui::app::worker_bridge::handle_processing_stopped(app, worker_bridge).await;
            super::bus_reply::emit(app, "failed", Some(err.to_string()));
            app.state.messages.push(ChatMessage::new(
                MessageType::Error,
                err.to_string(),
            ));
            app.state.status = "Request failed".to_string();
            app.state.scroll_to_bottom();
        }
    }
}

async fn save_current(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    worker_bridge: &mut Option<TuiWorkerBridge>,
    updated: Session,
) {
    if app.state.processing {
        crate::tui::app::worker_bridge::handle_processing_stopped(app, worker_bridge).await;
        app.state.clear_request_timing();
    }
    super::bus_reply::emit(app, "completed", None);
    *session = updated;
    session.attach_global_bus_if_missing();
    app.state.session_id = Some(session.id.clone());
    let _ = session.save().await;
    refresh_sessions(app, cwd).await;
}
