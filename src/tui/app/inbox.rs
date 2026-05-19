use std::{path::Path, sync::Arc};

use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::input::chat_submit_dispatch::dispatch_prompt;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::{IncomingTask, TuiWorkerBridge};

pub async fn trigger_next(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
) {
    if app.state.processing {
        return;
    }
    let Some(task) = app.state.dequeue_worker_task() else {
        return;
    };
    app.state.active_remote_task = Some(task.clone());
    announce(app, &task);
    dispatch_prompt(
        app,
        cwd,
        session,
        registry,
        worker_bridge,
        &task.message,
        Vec::new(),
        event_tx,
        result_tx,
    )
    .await;
}

fn announce(app: &mut App, task: &IncomingTask) {
    let from = task.from_agent.as_deref().unwrap_or("unknown");
    app.state.messages.push(ChatMessage::new(
        MessageType::System,
        format!(
            "Incoming A2A task {} from {}\n{}",
            task.task_id, from, task.message
        ),
    ));
    app.state.status = format!("Executing remote task {}", task.task_id);
    app.state.scroll_to_bottom();
}
