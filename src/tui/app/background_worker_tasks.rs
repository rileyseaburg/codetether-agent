//! Worker-task queue helpers for the background update loop.

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) fn queue_worker_tasks(app: &mut App, worker_bridge: &mut Option<TuiWorkerBridge>) {
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

pub(super) fn display_next_worker_task(app: &mut App) {
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
