//! Dispatch queued remote tasks through the active chat session.
use crate::{Dispatch, Presentation};
use codetether_worker_events::IncomingTask;
/// # Arguments
/// Receives the frontend bindings or active inbound slot and the operation's input.
/// # Returns
/// Applies the corresponding A2A transition while retaining the existing queue order.
pub async fn trigger_next(view: &mut impl Dispatch) {
    if view.processing() {
        return;
    }
    let Some(task) = view.dequeue_task() else {
        return;
    };
    view.set_active_task(task.clone());
    announce(view, &task);
    view.dispatch_prompt(&task.message).await;
}

fn announce(view: &mut impl Presentation, task: &IncomingTask) {
    let from = task.from_agent.as_deref().unwrap_or("unknown");
    view.push_system(format!(
        "Incoming A2A task {} from {}\n{}",
        task.task_id, from, task.message
    ));
    view.set_status(format!("Executing remote task {}", task.task_id));
    view.scroll_to_bottom();
}
