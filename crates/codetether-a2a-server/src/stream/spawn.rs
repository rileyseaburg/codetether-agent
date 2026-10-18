//! Clone execution state and schedule one stream task.
use super::*;
pub(crate) fn spawn<H: Application>(
    server: &A2AServer<H>,
    params: &MessageSendParams,
    task_id: &str,
    prompt: String,
) {
    // Spawn async processing with event streaming
    let tasks = server.tasks.clone();
    let context_id = params.message.context_id.clone();
    let spawn_task_id = task_id.to_string();
    let bus = server.bus.clone();

    tokio::spawn(async move {
        run::run::<H>(tasks, context_id, spawn_task_id, bus, prompt).await;
    });
}
