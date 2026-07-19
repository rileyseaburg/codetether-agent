//! Signed transport for streamed worker output.

pub(super) async fn send(
    client: reqwest::Client,
    server: String,
    token: Option<String>,
    worker_id: String,
    task_id: String,
    output: String,
) {
    let payload = serde_json::json!({ "worker_id": worker_id, "output": output });
    let resource = super::super::worker_security::mutation_resource(&task_id, &payload);
    let request = client
        .post(format!("{server}/v1/agent/tasks/{task_id}/output"))
        .header("X-Worker-ID", &worker_id);
    let mut request = match resource.and_then(|resource| {
        super::super::worker_security::apply_configured_proof(
            request, "output", &worker_id, &resource,
        )
    }) {
        Ok(request) => request,
        Err(error) => {
            tracing::warn!(task_id, %error, "Unable to sign task output");
            return;
        }
    };
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    let _ = request.json(&payload).send().await;
}
