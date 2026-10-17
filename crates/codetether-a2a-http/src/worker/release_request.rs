//! Signed worker release request construction.
use super::Payload;
use anyhow::Result;
use reqwest::Client;
/// Build a release request bound to its JSON payload.
///
/// # Arguments
/// * `client` - HTTP client.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `worker_id` - Worker releasing the task.
/// * `payload` - Task release metadata.
/// # Returns
/// A signed request with its body already attached.
/// # Errors
/// Returns serialization or proof configuration errors.
/// # Examples
/// ```rust,no_run
/// # fn example(payload: codetether_a2a_http::worker::Payload<'_>) -> anyhow::Result<()> {
/// use codetether_a2a_http::worker::build_release_request;
/// let request = build_release_request(&reqwest::Client::new(), "http://localhost:8001", &None, "worker", payload)?;
/// # Ok(()) }
/// ```

pub fn build_release_request(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    payload: Payload<'_>,
) -> Result<reqwest::RequestBuilder> {
    let req = client.post(format!("{server}/v1/worker/tasks/release"));
    let req = req.header("X-Worker-ID", worker_id).json(&payload);
    let resource = super::proof::mutation_resource(payload.task_id, &payload)?;
    let req = match token {
        Some(token) => req.bearer_auth(token),
        None => req,
    };
    super::proof::apply_configured(req, "release", worker_id, &resource)
}
