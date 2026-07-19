//! Output streaming callback for active tasks.
#[path = "task_output_request.rs"]
mod request;

use std::sync::Arc;

use crate::bus::AgentBus;

pub(super) fn build_output_callback(
    client: reqwest::Client,
    server: String,
    token: Option<String>,
    worker_id: String,
    task_id: String,
    bus: Arc<AgentBus>,
) -> Arc<dyn Fn(String) + Send + Sync + 'static> {
    Arc::new(move |output: String| {
        let client = client.clone();
        let server = server.clone();
        let token = token.clone();
        let worker_id = worker_id.clone();
        let task_id = task_id.clone();
        bus.handle("task-output").send(
            format!("task.{task_id}"),
            crate::bus::BusMessage::TaskUpdate {
                task_id: task_id.clone(),
                state: crate::a2a::types::TaskState::Working,
                message: Some(output.clone()),
            },
        );
        tokio::spawn(request::send(
            client, server, token, worker_id, task_id, output,
        ));
    })
}
