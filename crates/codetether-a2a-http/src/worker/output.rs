//! Signed transport for streamed worker output.

/// Send one line of task output.
/// # Arguments
/// * `client` - HTTP client.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `worker_id` - Worker identity.
/// * `task_id` - Task receiving output.
/// * `output` - Human-readable output text.
/// # Returns
/// Unit after attempting delivery; proof failures are logged.
/// # Examples
/// ```rust,no_run
/// # async fn example() {
/// use codetether_a2a_http::worker::output::send;
/// send(reqwest::Client::new(), "http://localhost:8001".into(), None, "worker".into(), "task".into(), "done".into()).await;
/// # }
/// ```
pub async fn send(
    client: reqwest::Client,
    server: String,
    token: Option<String>,
    worker_id: String,
    task_id: String,
    output: String,
) {
    send_with_event(client, server, token, worker_id, task_id, output, None).await
}

/// Sends streamed output, optionally carrying one structured session event.
///
/// The typed event travels beside the human-readable text on the same signed
/// request, so tool activity keeps its name, arguments, and status instead of
/// being flattened into a string the server cannot classify.
/// # Arguments
/// * `client` - HTTP client.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `worker_id` - Worker identity.
/// * `task_id` - Task receiving output.
/// * `output` - Human-readable output text.
/// * `event` - Optional structured session event, carried alongside text.
/// # Returns
/// Unit after attempting delivery; proof failures are logged.
/// # Examples
/// ```rust,no_run
/// # async fn example() {
/// use codetether_a2a_http::worker::output::send;
/// send(reqwest::Client::new(), "http://localhost:8001".into(), None, "worker".into(), "task".into(), "done".into()).await;
/// # }
/// ```
pub async fn send_with_event(
    client: reqwest::Client,
    server: String,
    token: Option<String>,
    worker_id: String,
    task_id: String,
    output: String,
    event: Option<serde_json::Value>,
) {
    let mut payload = serde_json::json!({ "worker_id": worker_id, "output": output });
    if let Some(event) = event
        && let Some(fields) = payload.as_object_mut()
    {
        fields.insert("event".into(), event);
    }
    let resource = super::proof::mutation_resource(&task_id, &payload);
    let request = client
        .post(format!("{server}/v1/agent/tasks/{task_id}/output"))
        .header("X-Worker-ID", &worker_id);
    let mut request = match resource.and_then(|resource| {
        super::proof::apply_configured(request, "output", &worker_id, &resource)
    }) {
        Ok(request) => request,
        Err(error) => {
            tracing::warn!(target: "codetether_agent::a2a::worker::task_output::request", task_id, %error, "Unable to sign task output");
            return;
        }
    };
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    let _ = request.json(&payload).send().await;
}
