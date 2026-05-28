//! Output streaming callback for active tasks.

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
        tokio::spawn(async move {
            let mut request = client
                .post(format!("{}/v1/agent/tasks/{}/output", server, task_id))
                .header("X-Worker-ID", &worker_id);
            if let Some(token) = &token {
                request = request.bearer_auth(token);
            }
            let _ = request
                .json(&serde_json::json!({ "worker_id": worker_id, "output": output }))
                .send()
                .await;
        });
    })
}
