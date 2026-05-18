//! Emit a bus reply to `from_agent` after a remote task completes.

use crate::tui::app::state::App;

pub fn emit(app: &mut App, status: &str, error: Option<String>) {
    let Some(task) = app.state.active_remote_task.take() else {
        return;
    };
    let Some(bus) = crate::bus::global() else {
        return;
    };
    let from_agent = task.from_agent.unwrap_or_default();
    if from_agent.is_empty() {
        return;
    }
    let handle = bus.handle("tui");
    let message = error.as_deref().unwrap_or(&task.message);
    handle.send_to_agent(
        &from_agent,
        vec![crate::a2a::types::Part::Text {
            text: format!("Task {} {}: {}", task.task_id, status, message),
        }],
    );
    tracing::info!(
        task_id = %task.task_id,
        to = %from_agent,
        status,
        "Emitted bus reply for remote task"
    );
}
