//! Worker-task queue helpers for the background update loop.

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) fn queue_worker_tasks(app: &mut App, worker_bridge: &mut Option<TuiWorkerBridge>) {
    if let Some(bridge) = worker_bridge.as_mut() {
        codetether_tui_worker::tasks::ingest(
            &mut app.state.worker,
            &mut app.state.status,
            &mut bridge.task_rx,
        );
    }
}

pub(super) fn display_next_worker_task(app: &mut App) {
    if let Some(presentation) =
        codetether_tui_worker::tasks::take_next(&mut app.state.worker, app.state.processing)
    {
        app.state
            .messages
            .push(ChatMessage::new(MessageType::System, presentation.message));
        app.state.status = presentation.status;
        app.state.chat_viewport.scroll_to_bottom();
    }
}
