//! Signed worker registration transport over host-built metadata.
use anyhow::Result;
use reqwest::Client;
/// Submit a worker registration payload with its canonical identity proof.
///
/// # Arguments
/// * `client` - HTTP client.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `worker_id` - Registering worker.
/// * `name` - Worker principal for signing.
/// * `payload` - Completed registration metadata.
/// # Returns
/// Unit after registration is accepted.
/// # Errors
/// Returns payload serialization, proof, transport, or rejection errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_a2a_http::worker::register_worker_payload;
/// register_worker_payload(&reqwest::Client::new(), "http://localhost:8001", &None, "worker", "agent", &serde_json::json!({})).await?;
/// # Ok(()) }
/// ```
pub async fn register_worker_payload(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    name: &str,
    payload: &serde_json::Value,
) -> Result<()> {
    let resource = super::proof::mutation_resource(worker_id, payload)?;
    let request = client.post(format!("{server}/v1/agent/workers/register"));
    let mut request = super::proof::apply(request, "register", worker_id, name, &resource)?;
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    let response = request.json(payload).send().await?;
    super::require_success(response).await
}
