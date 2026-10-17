//! Heartbeat HTTP POST helper.

use reqwest::Client;

/// Submit a signed worker heartbeat and report whether the server accepted it.
///
/// # Arguments
/// * `client` - HTTP client.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `worker_id` - Worker sending the heartbeat.
/// * `payload` - Completed heartbeat JSON body.
/// # Returns
/// True for a successful HTTP response; false for proof or transport errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() {
/// use codetether_a2a_http::worker::post_heartbeat;
/// let accepted = post_heartbeat(&reqwest::Client::new(), "http://localhost:8001", &None, "worker", &serde_json::json!({})).await;
/// # }
/// ```
pub async fn post_heartbeat(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    payload: &serde_json::Value,
) -> bool {
    let req = client.post(format!(
        "{}/v1/agent/workers/{}/heartbeat",
        server, worker_id
    ));
    let mut req = match super::proof::apply_configured(req, "heartbeat", worker_id, "") {
        Ok(request) => request,
        Err(error) => {
            tracing::warn!(target: "codetether_agent::a2a::worker::heartbeat_loop::heartbeat_post", worker_id, %error, "Unable to sign worker heartbeat");
            return false;
        }
    };
    if let Some(token) = token {
        req = req.bearer_auth(token);
    }
    req.json(payload)
        .send()
        .await
        .is_ok_and(|r| r.status().is_success())
}
