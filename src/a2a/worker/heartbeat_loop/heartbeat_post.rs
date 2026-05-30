//! Heartbeat HTTP POST helper.

use reqwest::Client;

pub(super) async fn post_heartbeat(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    payload: &serde_json::Value,
) -> bool {
    let mut req = client.post(format!(
        "{}/v1/agent/workers/{}/heartbeat",
        server, worker_id
    ));
    if let Some(token) = token {
        req = req.bearer_auth(token);
    }
    req.json(payload)
        .send()
        .await
        .is_ok_and(|r| r.status().is_success())
}
